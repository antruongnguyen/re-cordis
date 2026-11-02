//! Typed plugin catalogs and source-aware, owner-scoped entry trees.
//!
//! Entries have stable loader identity independent of core Fiber allocations.
//! Mutations start immediately; tasks only wait. Missing required services settle
//! to Pending. Keep the original Runtime::run driven through joined disposal.
//!
//! ```
//! use re_cordis_core::{BoxError, Plugin, Runtime};
//! use re_cordis_loader::{Catalog, Config, EntrySpec, Loader};
//! use std::convert::Infallible;
//!
//! fn main() -> Result<(), BoxError> {
//!     let executor = tokio::runtime::Builder::new_current_thread().build()?;
//!     executor.block_on(async {
//!         let runtime = Runtime::new();
//!         runtime.run(async {
//!             let root = runtime.context();
//!             let plugin = Plugin::new("worker", |ctx, value: u32| async move {
//!                 assert!(value == 1 || value == 2);
//!                 ctx.on_dispose_sync(|| Ok::<_, Infallible>(()))?;
//!                 Ok::<_, BoxError>(())
//!             });
//!             let mut catalog = Catalog::new();
//!             catalog.register("worker", plugin)?;
//!             let loader = Loader::new(&root, catalog)?;
//!             let id = loader.create(EntrySpec::new("worker", Config::new(1_u32)))?;
//!             assert!(loader.settle()?.await?.failures.is_empty());
//!             loader.update(&id, EntrySpec::new("worker", Config::new(2_u32)))?.await?;
//!             loader.dispose()?.await?;
//!             assert!(root.registry().is_empty()?);
//!             Ok::<_, BoxError>(())
//!         }).await??;
//!         Ok::<_, BoxError>(())
//!     })
//! }
//! ```
//!
//! ```compile_fail
//! use re_cordis_loader::Loader;
//! fn move_to_thread(loader: Loader) {
//!     std::thread::spawn(move || drop(loader));
//! }
//! ```
//!
//! See the repository's paired loader/group guides for bounded JSON, typed scope
//! evaluation, source reconciliation and namespace/move adaptations.
mod scope;
mod source;
use futures_util::future::{FutureExt, LocalBoxFuture, Shared};
pub use re_cordis_core::hooks::Value as Config;
use re_cordis_core::{
    BoxError, Context, Dependency, FiberHandle, FiberId, FiberState, LifecycleError, LifecycleTask,
    Listener, Plugin, UpdateOptions, hooks,
};
pub use scope::Scope;
pub use source::{
    EntryPatch, FieldPatch, MAX_ENTRIES, MAX_JSON_BYTES, MAX_JSON_DEPTH, MAX_JSON_NODES,
    MAX_STRING_BYTES, SourceDocument, SourceId, SourceLocation, SourceSpan,
};
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    convert::Infallible,
    error::Error,
    fmt,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context as TaskContext, Poll},
};

/// Stable identity of a loader entry.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntryId(String);
impl EntryId {
    /// The local, loader-wide identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for EntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
/// Catalog allocation identity, independent of core Fiber identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactoryId(u64);
/// Refused mutation, decode failure or lifecycle failure with original source.
#[derive(Clone, Debug)]
pub enum LoaderError {
    /// A factory with this name already exists.
    DuplicateFactory(String),
    /// No registered factory has this name.
    UnknownFactory(String),
    /// An explicit entry ID is already used.
    DuplicateEntry(String),
    /// No entry has this identity.
    UnknownEntry(EntryId),
    /// Invalid bounded input.
    Invalid(String),
    /// Checked typed configuration decoding failed.
    Decode {
        /// Factory being decoded.
        factory: String,
        /// Original decoder or contract error.
        source: Rc<dyn Error>,
    },
    /// Core lifecycle refused or failed.
    Lifecycle(LifecycleError),
    /// Contained loader-origin callback panic or refused callback admission.
    Callback(re_cordis_core::CallbackError),
    /// Transport or source decoding failed at a structured path.
    Located {
        /// Source/path/span, when provided.
        location: SourceLocation,
        /// Original error.
        source: Rc<dyn Error>,
    },
}
impl fmt::Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateFactory(n) => write!(f, "duplicate factory: {n}"),
            Self::UnknownFactory(n) => write!(f, "unknown factory: {n}"),
            Self::DuplicateEntry(n) => write!(f, "duplicate entry: {n}"),
            Self::UnknownEntry(n) => write!(f, "unknown entry: {n}"),
            Self::Invalid(n) => n.fmt(f),
            Self::Decode { factory, source } => write!(f, "configuration for {factory}: {source}"),
            Self::Lifecycle(e) => e.fmt(f),
            Self::Callback(e) => e.fmt(f),
            Self::Located { location, source } => write!(f, "{source} at {:?}", location.path),
        }
    }
}
impl Error for LoaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Decode { source, .. } | Self::Located { source, .. } => Some(source.as_ref()),
            Self::Lifecycle(e) => Some(e),
            Self::Callback(e) => Some(e),
            _ => None,
        }
    }
}
impl From<re_cordis_core::CallbackError> for LoaderError {
    fn from(error: re_cordis_core::CallbackError) -> Self {
        Self::Callback(error)
    }
}
impl From<re_cordis_core::EventError> for LoaderError {
    fn from(e: re_cordis_core::EventError) -> Self {
        Self::Lifecycle(LifecycleError::Hook(e))
    }
}
impl From<LifecycleError> for LoaderError {
    fn from(e: LifecycleError) -> Self {
        Self::Lifecycle(e)
    }
}
impl From<re_cordis_core::AdmissionError> for LoaderError {
    fn from(e: re_cordis_core::AdmissionError) -> Self {
        Self::Lifecycle(e.into())
    }
}

trait Instance {
    fn id(&self) -> FiberId;
    fn state(&self) -> FiberState;
    fn context(&self) -> Context;
    fn ready(&self) -> Result<LifecycleTask, LoaderError>;
    fn dispose(&self) -> Result<LifecycleTask, LoaderError>;
    fn update(&self, source: &Config) -> Result<LifecycleTask, LoaderError>;
}
struct TypedInstance<C: 'static> {
    name: String,
    fiber: FiberHandle<C>,
}
impl<C: Clone + 'static> Instance for TypedInstance<C> {
    fn id(&self) -> FiberId {
        self.fiber.id().clone()
    }
    fn state(&self) -> FiberState {
        self.fiber.state()
    }
    fn context(&self) -> Context {
        self.fiber.context()
    }
    fn ready(&self) -> Result<LifecycleTask, LoaderError> {
        Ok(self.fiber.ready()?)
    }
    fn dispose(&self) -> Result<LifecycleTask, LoaderError> {
        Ok(self.fiber.dispose()?)
    }
    fn update(&self, source: &Config) -> Result<LifecycleTask, LoaderError> {
        let value = source.get::<C>().map_err(|e| LoaderError::Decode {
            factory: self.name.clone(),
            source: Rc::new(e),
        })?;
        Ok(self
            .fiber
            .update_with((*value).clone(), UpdateOptions { no_save: true })?)
    }
}
trait Factory {
    fn contract(&self) -> std::any::TypeId;
    fn check(&self, config: &Config) -> Result<(), LoaderError>;
    fn create(&self, context: &Context, config: &Config) -> Result<Rc<dyn Instance>, LoaderError>;
}
struct TypedFactory<C, R> {
    name: String,
    plugin: Plugin<C, R>,
}
impl<C: Clone + 'static, R: 'static> Factory for TypedFactory<C, R> {
    fn contract(&self) -> std::any::TypeId {
        std::any::TypeId::of::<C>()
    }
    fn check(&self, config: &Config) -> Result<(), LoaderError> {
        config
            .get::<C>()
            .map(|_| ())
            .map_err(|e| LoaderError::Decode {
                factory: self.name.clone(),
                source: Rc::new(e),
            })
    }
    fn create(&self, context: &Context, config: &Config) -> Result<Rc<dyn Instance>, LoaderError> {
        let value = config.get::<C>().map_err(|e| LoaderError::Decode {
            factory: self.name.clone(),
            source: Rc::new(e),
        })?;
        Ok(Rc::new(TypedInstance {
            name: self.name.clone(),
            fiber: context.plugin(&self.plugin, (*value).clone())?,
        }))
    }
}
type Decoder = dyn Fn(Option<&serde_json::Value>) -> Result<Config, BoxError>;
#[derive(Clone)]
struct FactoryRecord {
    id: FactoryId,
    adapter: Rc<dyn Factory>,
    decoder: Option<Rc<Decoder>>,
}
/// Explicit names resolving to reusable typed core Plugin declarations.
#[derive(Default)]
pub struct Catalog {
    factories: BTreeMap<String, FactoryRecord>,
    next: u64,
    services: BTreeMap<String, ServiceContract>,
}
type IsolateService = dyn Fn(&Scope, &re_cordis_core::IsolationId) -> Scope;
type ConfigureService = dyn Fn(&serde_json::Value) -> Result<(Dependency, Scope), BoxError>;
#[derive(Clone)]
struct ServiceContract {
    dependency: Dependency,
    isolate: Rc<IsolateService>,
    configure: Option<Rc<ConfigureService>>,
}
impl Catalog {
    /// Construct an empty catalog.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a typed declaration. Native configuration needs no serialization.
    pub fn register<C: Clone + 'static, R: 'static>(
        &mut self,
        name: impl Into<String>,
        plugin: Plugin<C, R>,
    ) -> Result<(), LoaderError> {
        let name = name.into();
        if name.is_empty() || name.len() > 256 {
            return Err(LoaderError::Invalid(
                "factory name must contain 1..=256 UTF-8 bytes".into(),
            ));
        }
        if self.factories.contains_key(&name) {
            return Err(LoaderError::DuplicateFactory(name));
        }
        self.next = self
            .next
            .checked_add(1)
            .ok_or_else(|| LoaderError::Invalid("factory identity exhausted".into()))?;
        self.factories.insert(
            name.clone(),
            FactoryRecord {
                id: FactoryId(self.next),
                adapter: Rc::new(TypedFactory { name, plugin }),
                decoder: None,
            },
        );
        Ok(())
    }
}
impl Catalog {
    /// Register a typed declaration with an explicit JSON decoder.
    /// None means absent config; Some(Null) means JSON null.
    pub fn register_json<C: Clone + 'static, R: 'static, F, E>(
        &mut self,
        name: impl Into<String>,
        plugin: Plugin<C, R>,
        decode: F,
    ) -> Result<(), LoaderError>
    where
        F: Fn(Option<&serde_json::Value>) -> Result<C, E> + 'static,
        E: Into<BoxError> + 'static,
    {
        let name = name.into();
        self.register(name.clone(), plugin)?;
        self.factories.get_mut(&name).expect("registered").decoder = Some(Rc::new(move |value| {
            Ok(Config::new(decode(value).map_err(Into::into)?))
        }));
        Ok(())
    }
}
impl Catalog {
    /// Register a typed service name for serialized injection/isolation.
    pub fn register_service<T: 'static>(
        &mut self,
        key: &re_cordis_core::ServiceKey<T>,
    ) -> Result<(), LoaderError> {
        if self.services.contains_key(key.name()) {
            return Err(LoaderError::Invalid(format!(
                "duplicate service contract: {}",
                key.name()
            )));
        }
        let isolated = key.clone();
        self.services.insert(
            key.name().into(),
            ServiceContract {
                dependency: key.required(),
                isolate: Rc::new(move |scope, slot| scope.clone().isolate_with(&isolated, slot)),
                configure: None,
            },
        );
        Ok(())
    }
    /// Register typed injection/intercept JSON config for a service name.
    pub fn register_service_config<T: 'static, C: Clone + 'static, F, E>(
        &mut self,
        key: &re_cordis_core::ServiceKey<T>,
        decode: F,
    ) -> Result<(), LoaderError>
    where
        F: Fn(&serde_json::Value) -> Result<C, E> + 'static,
        E: Into<BoxError> + 'static,
    {
        self.register_service(key)?;
        let key = key.clone();
        let name = key.name().to_owned();
        self.services.get_mut(&name).expect("registered").configure = Some(Rc::new(move |value| {
            let config = decode(value).map_err(Into::into)?;
            Ok((
                key.required().with_config(config.clone()),
                Scope::new().intercept(&key, config),
            ))
        }));
        Ok(())
    }
}
/// Origin of an accepted source mutation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Origin {
    /// Programmatic edit that can be persisted by a subscriber.
    Runtime,
    /// Read-only reconciliation from an external source owner.
    Source(SourceId),
}
/// One accepted edit. The event is emitted once; no history is stored.
#[derive(Clone, Debug)]
pub struct CommitRecord {
    /// Stable edited identity.
    pub id: EntryId,
    /// Previous value; absent for creation.
    pub old: Option<EntrySnapshot>,
    /// New value; absent for removal.
    pub new: Option<EntrySnapshot>,
    /// Previous group for a move.
    pub from: Option<EntryId>,
    /// Current target group.
    pub group: Option<EntryId>,
    /// Explicit origin.
    pub origin: Origin,
}
type Mapper = dyn Fn(&Context, &Config) -> Result<Config, BoxError>;
type Condition = dyn Fn(&Context) -> Result<bool, BoxError>;
/// Structural child configuration; behavior lives in the loader package.
#[derive(Clone, Debug)]
pub struct Group {
    /// Ordered child sources.
    pub entries: Vec<EntrySpec>,
}
impl Group {
    /// Specify an ordered structural group.
    pub fn new(entries: Vec<EntrySpec>) -> Self {
        Self { entries }
    }
}
/// Explicit distinction between plugin entries and structural groups.
#[derive(Clone, Debug)]
pub enum EntryKind {
    /// Apply a catalog plugin.
    Plugin,
    /// Retain and reconcile ordered children.
    Group(Group),
}
/// Requested plugin and native source configuration.
#[derive(Clone)]
pub struct EntrySpec {
    /// Optional explicit loader-wide identity.
    pub id: Option<String>,
    /// Registered factory name.
    pub factory: String,
    /// Retained source, before evaluation and plugin validation.
    pub config: Config,
    /// Disabled source flag.
    pub disabled: bool,
    /// Plugin or structural group.
    pub kind: EntryKind,
    /// Entry-level required dependencies, distinct from plugin injection.
    inject: Vec<Dependency>,
    /// Isolation selected before entry dependency resolution.
    pub scope: Scope,
    /// Intercepts evaluated after entry dependency resolution.
    pub intercept: Scope,
    condition: Option<Rc<Condition>>,
    mapper: Option<Rc<Mapper>>,
    inject_token: Rc<()>,
    location: Option<SourceLocation>,
    transport: Option<serde_json::Value>,
}
impl EntrySpec {
    /// Specify a registered factory and a checked heterogeneous source.
    pub fn new(factory: impl Into<String>, config: Config) -> Self {
        Self {
            id: None,
            factory: factory.into(),
            config,
            disabled: false,
            kind: EntryKind::Plugin,
            inject: Vec::new(),
            scope: Scope::new(),
            intercept: Scope::new(),
            condition: None,
            mapper: None,
            inject_token: Rc::new(()),
            location: None,
            transport: None,
        }
    }
    /// Select a nonempty ID without ':' and at most 128 UTF-8 bytes.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    /// Retain this entry without applying its plugin.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
impl fmt::Debug for EntrySpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EntrySpec")
            .field("id", &self.id)
            .field("factory", &self.factory)
            .field("config", &self.config)
            .field("disabled", &self.disabled)
            .field("kind", &self.kind)
            .finish()
    }
}
impl EntrySpec {
    /// A structural group. Its own owner remains active when descendants are disabled.
    pub fn group(group: Group) -> Self {
        let mut spec = Self::new("group", Config::new(()));
        spec.kind = EntryKind::Group(group);
        spec
    }
    /// Declare entry-level service requirements.
    pub fn injecting(mut self, dependencies: impl IntoIterator<Item = Dependency>) -> Self {
        self.inject = dependencies.into_iter().collect();
        self.inject_token = Rc::new(());
        self
    }
    /// Read the declared entry-level requirements.
    pub fn dependencies(&self) -> &[Dependency] {
        &self.inject
    }
    /// Select typed isolation before resolving entry requirements.
    pub fn scope(mut self, scope: Scope) -> Self {
        self.scope = scope;
        self
    }
    /// Select typed intercepts evaluated after entry requirements are committed.
    pub fn intercept(mut self, scope: Scope) -> Self {
        self.intercept = scope;
        self
    }
    /// Evaluate an enabled verdict in the entry injection Context.
    pub fn condition<F, E>(mut self, condition: F) -> Self
    where
        F: Fn(&Context) -> Result<bool, E> + 'static,
        E: Into<BoxError> + 'static,
    {
        self.condition = Some(Rc::new(move |ctx| condition(ctx).map_err(Into::into)));
        self
    }
    /// Transform source in the exact inner plugin Context before its validator.
    pub fn map_config<C: 'static, F, E>(mut self, mapper: F) -> Self
    where
        F: Fn(&Context, &C) -> Result<C, E> + 'static,
        E: Into<BoxError> + 'static,
    {
        self.mapper = Some(Rc::new(move |ctx, value| {
            let value = value.get::<C>()?;
            Ok(Config::new(mapper(ctx, &value).map_err(Into::into)?))
        }));
        self
    }
}
/// An entry's current runtime condition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryState {
    /// Waiting for required publications.
    Pending,
    /// Applying the plugin.
    Loading,
    /// Plugin successfully applied.
    Active,
    /// Latest load failed.
    Failed,
    /// Owned resources are draining.
    Unloading,
    /// Present in source but disabled.
    Disabled,
    /// Removed runtime entry.
    Disposed,
}
impl From<FiberState> for EntryState {
    fn from(state: FiberState) -> Self {
        match state {
            FiberState::Pending => Self::Pending,
            FiberState::Loading => Self::Loading,
            FiberState::Active => Self::Active,
            FiberState::Failed => Self::Failed,
            FiberState::Unloading => Self::Unloading,
            FiberState::Disposed => Self::Disposed,
        }
    }
}
/// Immutable source and lifecycle view; contains no Fiber control.
#[derive(Clone, Debug)]
pub struct EntrySnapshot {
    /// Stable loader identity.
    pub id: EntryId,
    /// Current factory allocation.
    pub factory_id: FactoryId,
    /// Factory name.
    pub factory: String,
    /// Retained source configuration.
    pub source: Config,
    /// Complete immutable source options.
    pub options: EntrySpec,
    /// Current core allocation, absent while disabled.
    pub fiber_id: Option<FiberId>,
    /// Observable state.
    pub state: EntryState,
    /// Current logical group, independent of core ownership generations.
    pub parent: Option<EntryId>,
    /// Mutable composite structural address.
    pub address: String,
    /// Ordered children for structural groups.
    pub children: Vec<EntryId>,
    /// Last transformed source before plugin validation; absent before evaluation.
    pub evaluated: Option<Config>,
    /// Entry-level core owner allocation.
    pub outer_id: Option<FiberId>,
    /// Source owner or runtime origin.
    pub origin: Origin,
    /// Original JSON transport node, distinct from decoded/evaluated config.
    pub transport: Option<serde_json::Value>,
}
type Work = Shared<LocalBoxFuture<'static, Result<(), LoaderError>>>;
struct Entry {
    id: EntryId,
    spec: RefCell<EntrySpec>,
    factory: RefCell<FactoryRecord>,
    instance: RefCell<Option<Rc<dyn Instance>>>,
    revision: Cell<u64>,
    live: Cell<bool>,
    running: Cell<bool>,
    work: RefCell<Option<Work>>,
    outer: RefCell<Option<FiberHandle<Rc<Entry>>>>,
    parent: RefCell<Option<EntryId>>,
    children: RefCell<Vec<EntryId>>,
    view: RefCell<Option<Context>>,
    verdict: Cell<Option<bool>>,
    evaluated: RefCell<Option<Config>>,
    origin: RefCell<Origin>,
    transport: RefCell<Option<serde_json::Value>>,
    rejected: RefCell<Option<LoaderError>>,
}
impl Entry {
    fn snapshot(&self) -> EntrySnapshot {
        let spec = self.spec.borrow();
        let instance = self.instance.borrow();
        let outer = self.outer.borrow();
        let state = if spec.disabled && !matches!(spec.kind, EntryKind::Group(_)) {
            EntryState::Disabled
        } else if let Some(instance) = instance.as_ref() {
            instance.state().into()
        } else if let Some(outer) = outer.as_ref() {
            if matches!(spec.kind, EntryKind::Group(_)) {
                outer.state().into()
            } else if outer.state() == FiberState::Active {
                EntryState::Disabled
            } else {
                outer.state().into()
            }
        } else if self.running.get() {
            EntryState::Unloading
        } else if self.live.get() {
            EntryState::Disabled
        } else {
            EntryState::Disposed
        };
        EntrySnapshot {
            id: self.id.clone(),
            factory_id: self.factory.borrow().id,
            factory: spec.factory.clone(),
            source: spec.config.clone(),
            options: spec.clone(),
            fiber_id: instance.as_ref().map(|i| i.id()),
            state,
            parent: self.parent.borrow().clone(),
            address: self.id.0.clone(),
            children: self.children.borrow().clone(),
            evaluated: self.evaluated.borrow().clone(),
            outer_id: outer.as_ref().map(|o| o.id().clone()),
            origin: self.origin.borrow().clone(),
            transport: self.transport.borrow().clone(),
        }
    }
}
/// Per-entry failure in a completed settlement pass.
#[derive(Clone, Debug)]
pub struct EntryFailure {
    /// The failed entry.
    pub id: EntryId,
    /// Original failure.
    pub error: LoaderError,
}
/// A settlement pass. Pending is a normal successful result.
#[derive(Clone, Debug)]
pub struct Settlement {
    /// Current immutable views.
    pub entries: Vec<EntrySnapshot>,
    /// Independent entry failures.
    pub failures: Vec<EntryFailure>,
}
/// Wait-only barrier for already-started loader work.
#[must_use = "loader work has started; await the barrier or explicitly drop it"]
pub struct LoaderTask<T>(LocalBoxFuture<'static, Result<T, LoaderError>>);
impl<T> Future for LoaderTask<T> {
    type Output = Result<T, LoaderError>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}
type Retirement = Shared<LocalBoxFuture<'static, Result<FiberState, LifecycleError>>>;
pub(crate) struct NamedRealm {
    id: re_cordis_core::IsolationId,
}
struct State {
    owner: FiberHandle<()>,
    catalog: RefCell<Catalog>,
    entries: RefCell<BTreeMap<EntryId, Rc<Entry>>>,
    next: Cell<u64>,
    retired: RefCell<Vec<Retirement>>,
    retirement_error: RefCell<Option<LifecycleError>>,
    retiring: RefCell<Vec<(Rc<Entry>, Option<Retirement>)>>,
    disposed: Cell<bool>,
    outer: Plugin<Rc<Entry>>,
    group: FactoryRecord,
    order: RefCell<Vec<EntryId>>,
    commits: re_cordis_core::Event<CommitRecord>,
    sources: RefCell<BTreeMap<SourceId, Vec<EntryId>>>,
    realms: RefCell<BTreeMap<String, std::rc::Weak<NamedRealm>>>,
}
/// Cloneable view of a dedicated parent-owned loader Fiber. Drop is inert.
#[derive(Clone)]
pub struct Loader(Rc<State>);
impl Loader {
    /// Admit a dedicated owner. Eager create is allowed before its first poll.
    pub fn new(owner: &Context, catalog: Catalog) -> Result<Self, LoaderError> {
        let plugin = Plugin::new("loader", |_, ()| async { Ok::<_, Infallible>(()) });
        let fiber = owner.plugin(&plugin, ())?;
        let state = Rc::new_cyclic(|weak: &std::rc::Weak<State>| {
            let loader = weak.clone();
            let outer = Plugin::new("loader:outer", move |ctx, entry: Rc<Entry>| {
                let loader = loader.clone();
                async move {
                    let state = loader
                        .upgrade()
                        .ok_or_else(|| LoaderError::Invalid("loader released".into()))?;
                    Loader(state).start(&ctx, &entry)?;
                    Ok::<_, LoaderError>(())
                }
            });
            State {
                owner: fiber,
                catalog: RefCell::new(catalog),
                entries: RefCell::new(BTreeMap::new()),
                next: Cell::new(0),
                retired: RefCell::new(Vec::new()),
                retirement_error: RefCell::new(None),
                retiring: RefCell::new(Vec::new()),
                disposed: Cell::new(false),
                outer,
                group: FactoryRecord {
                    id: FactoryId(0),
                    adapter: Rc::new(TypedFactory {
                        name: "group".into(),
                        plugin: Plugin::new("loader:group", |_, ()| async {
                            Ok::<_, Infallible>(())
                        }),
                    }),
                    decoder: None,
                },
                order: RefCell::new(Vec::new()),
                commits: re_cordis_core::Event::symbol("loader/commit"),
                sources: RefCell::new(BTreeMap::new()),
                realms: RefCell::new(BTreeMap::new()),
            }
        });
        let retained = state.clone();
        state.owner.context().on_dispose(move || {
            retained.disposed.set(true);
            let entries: Vec<_> = retained.entries.borrow().values().cloned().collect();
            for entry in entries {
                entry.live.set(false);
                let outer = entry.outer.borrow_mut().take();
                let inner = entry.instance.borrow_mut().take();
                let view = entry.view.borrow_mut().take();
                drop((outer, inner, view));
            }
            async move { join_retired(&retained).await }
        })?;
        let weak = Rc::downgrade(&state);
        state.owner.context().on(
            &hooks::config(),
            Listener::around(
                move |_, request: hooks::Config, next: re_cordis_core::Next<Config>| {
                    let value = next.call()?;
                    let Some(state) = weak.upgrade() else {
                        return Ok::<_, BoxError>(value);
                    };
                    let entries: Vec<_> = state.entries.borrow().values().cloned().collect();
                    for entry in entries {
                        let instance = entry.instance.borrow().clone();
                        if let Some(instance) =
                            instance.filter(|instance| instance.id() == request.fiber.id)
                        {
                            let mapper = entry.spec.borrow().mapper.clone();
                            let mapped = if let Some(mapper) = mapper {
                                mapper(&instance.context(), &value)?
                            } else {
                                value
                            };
                            let previous = entry.evaluated.replace(Some(mapped.clone()));
                            drop(previous);
                            return Ok(mapped);
                        }
                    }
                    Ok(value)
                },
            ),
        )?;
        let weak = Rc::downgrade(&state);
        state.owner.context().on(
            &hooks::plugin(),
            Listener::sync(move |_, info: hooks::FiberInfo| {
                if !info.live {
                    if let Some(state) = weak.upgrade().filter(|state| !state.disposed.get()) {
                        let entries: Vec<_> = state.entries.borrow().values().cloned().collect();
                        for entry in entries {
                            let instance = entry.instance.borrow().clone();
                            if entry.live.get()
                                && Loader(state.clone()).owners_active(&entry)
                                && instance.is_some_and(|instance| instance.id() == info.id)
                            {
                                let loader = Loader(state.clone());
                                let old = loader.snapshot(&entry.id);
                                entry.spec.borrow_mut().disabled = true;
                                entry.verdict.set(Some(false));
                                let group = entry.parent.borrow().clone();
                                loader.commit(CommitRecord {
                                    id: entry.id.clone(),
                                    old,
                                    new: loader.snapshot(&entry.id),
                                    from: None,
                                    group,
                                    origin: Origin::Runtime,
                                })?;
                            }
                        }
                    }
                }
                Ok::<_, LoaderError>(std::ops::ControlFlow::Continue(()))
            }),
        )?;
        let weak = Rc::downgrade(&state);
        state.owner.context().on(
            &hooks::commit(),
            Listener::sync(move |_, request: hooks::Commit| {
                if let Some(state) = weak.upgrade().filter(|state| !state.disposed.get()) {
                    let entries: Vec<_> = state.entries.borrow().values().cloned().collect();
                    for entry in entries {
                        let instance = entry.instance.borrow().clone();
                        if entry.live.get()
                            && instance.is_some_and(|instance| instance.id() == request.fiber.id)
                        {
                            let loader = Loader(state.clone());
                            let old = loader.snapshot(&entry.id);
                            let mut spec = entry.spec.borrow().clone();
                            spec.config = request.source.clone();
                            let previous = entry.spec.replace(spec);
                            drop(previous);
                            let group = entry.parent.borrow().clone();
                            loader.commit(CommitRecord {
                                id: entry.id.clone(),
                                old,
                                new: loader.snapshot(&entry.id),
                                from: None,
                                group,
                                origin: Origin::Runtime,
                            })?;
                        }
                    }
                }
                Ok::<_, LoaderError>(std::ops::ControlFlow::Continue(()))
            }),
        )?;
        Ok(Self(state))
    }
    fn admit(&self) -> Result<(), LoaderError> {
        self.0.owner.context().invoke_callback(|| ())?;
        self.prune();
        if self.0.disposed.get() {
            return Err(LoaderError::Invalid("loader disposed".into()));
        }
        Ok(())
    }
    fn prune(&self) {
        let retired = self.0.retired.replace(Vec::new());
        let retained: Vec<_> = retired
            .into_iter()
            .filter(|task| match task.clone().now_or_never() {
                None => true,
                Some(Err(error)) => {
                    if self.0.retirement_error.borrow().is_none() {
                        *self.0.retirement_error.borrow_mut() = Some(error);
                    }
                    false
                }
                Some(Ok(_)) => false,
            })
            .collect();
        *self.0.retired.borrow_mut() = retained;
        let retiring = self.0.retiring.replace(Vec::new());
        let retained: Vec<_> = retiring
            .into_iter()
            .filter(|(entry, retirement)| {
                let cleanup_pending = retirement
                    .as_ref()
                    .is_some_and(|task| task.clone().now_or_never().is_none());
                let work = entry.work.borrow().clone();
                cleanup_pending || work.is_some_and(|task| task.now_or_never().is_none())
            })
            .collect();
        *self.0.retiring.borrow_mut() = retained;
        self.0.sources.borrow_mut().retain(|source, ids| {
            ids.retain(|id| {
                self.entry(id).ok().is_some_and(|entry| {
                    *entry.origin.borrow() == Origin::Source(source.clone())
                        && entry.parent.borrow().is_none()
                })
            });
            !ids.is_empty()
        });
        self.0
            .realms
            .borrow_mut()
            .retain(|_, realm| realm.strong_count() > 0);
    }
    fn capacity(&self, spec: &EntrySpec, existing: bool) -> Result<(), LoaderError> {
        fn added(loader: &Loader, spec: &EntrySpec, existing: bool) -> usize {
            let present = existing
                || spec
                    .id
                    .as_ref()
                    .is_some_and(|id| loader.0.entries.borrow().contains_key(&EntryId(id.clone())));
            let mut count = usize::from(!present);
            if let EntryKind::Group(group) = &spec.kind {
                for child in &group.entries {
                    count = count.saturating_add(added(loader, child, false));
                }
            }
            count
        }
        if self.0.entries.borrow().len()
            + self.0.retiring.borrow().len()
            + added(self, spec, existing)
            > MAX_ENTRIES
        {
            return Err(LoaderError::Invalid(
                "resident and retiring entry limit 1024 exceeded".into(),
            ));
        }
        Ok(())
    }
    fn preflight(
        &self,
        spec: &EntrySpec,
        depth: usize,
        ids: &mut Vec<String>,
    ) -> Result<(), LoaderError> {
        if depth > 32 {
            return Err(LoaderError::Invalid("group depth limit 32 exceeded".into()));
        }
        if let Some(id) = &spec.id {
            if id.is_empty() || id.len() > 128 || id.contains(':') {
                return Err(LoaderError::Invalid(
                    "entry ID must contain 1..=128 UTF-8 bytes and no ':'".into(),
                ));
            }
            if ids.contains(id) {
                return Err(LoaderError::DuplicateEntry(id.clone()));
            }
            ids.push(id.clone());
        }
        match &spec.kind {
            EntryKind::Plugin => {
                let factory = self
                    .0
                    .catalog
                    .borrow()
                    .factories
                    .get(&spec.factory)
                    .cloned()
                    .ok_or_else(|| LoaderError::UnknownFactory(spec.factory.clone()))?;
                factory.adapter.check(&spec.config)?;
            }
            EntryKind::Group(group) => {
                for child in &group.entries {
                    self.preflight(child, depth + 1, ids)?;
                }
            }
        }
        Ok(())
    }
    fn entry(&self, id: &EntryId) -> Result<Rc<Entry>, LoaderError> {
        self.0
            .entries
            .borrow()
            .get(id)
            .cloned()
            .ok_or_else(|| LoaderError::UnknownEntry(id.clone()))
    }
    fn parent_view(&self, parent: Option<&EntryId>) -> Result<Context, LoaderError> {
        if let Some(parent) = parent {
            let parent = self.entry(parent)?;
            if !matches!(parent.spec.borrow().kind, EntryKind::Group(_)) {
                return Err(LoaderError::Invalid("parent must be a group".into()));
            }
            let view = parent.view.borrow().clone();
            Ok(view
                .or_else(|| parent.outer.borrow().as_ref().map(|outer| outer.context()))
                .ok_or_else(|| LoaderError::Invalid("parent group is not admitted".into()))?)
        } else {
            Ok(self.0.owner.context())
        }
    }
    fn owners_active(&self, entry: &Entry) -> bool {
        if self.0.owner.state() != FiberState::Active {
            return false;
        }
        let mut current = Some(entry.id.clone());
        while let Some(id) = current {
            let Ok(entry) = self.entry(&id) else {
                return false;
            };
            if !entry
                .outer
                .borrow()
                .as_ref()
                .is_some_and(|outer| outer.state() == FiberState::Active)
            {
                return false;
            }
            current = entry.parent.borrow().clone();
        }
        true
    }
    fn blocked(&self, entry: &Entry) -> bool {
        let mut parent = entry.parent.borrow().clone();
        while let Some(id) = parent {
            let Ok(ancestor) = self.entry(&id) else {
                return true;
            };
            if ancestor.verdict.get() != Some(true) {
                return true;
            }
            parent = ancestor.parent.borrow().clone();
        }
        false
    }
    fn start(&self, context: &Context, entry: &Rc<Entry>) -> Result<(), LoaderError> {
        if !entry.live.get() || self.0.disposed.get() {
            return Ok(());
        }
        let revision = entry.revision.get();
        let spec = entry.spec.borrow().clone();
        let structural = matches!(spec.kind, EntryKind::Group(_));
        if !structural && (spec.disabled || self.blocked(entry)) {
            entry.verdict.set(Some(false));
            return Ok(());
        }
        let enabled = if spec.disabled || self.blocked(entry) {
            false
        } else if let Some(condition) = &spec.condition {
            condition(context).map_err(|source| LoaderError::Decode {
                factory: spec.factory.clone(),
                source: Rc::from(source),
            })?
        } else {
            true
        };
        if !entry.live.get() || entry.revision.get() != revision {
            return Ok(());
        }
        entry.verdict.set(Some(enabled));
        if !structural && !enabled {
            return Ok(());
        }
        let context = if enabled {
            spec.intercept
                .apply(context)
                .map_err(|source| LoaderError::Decode {
                    factory: spec.factory.clone(),
                    source: Rc::from(source),
                })?
        } else {
            context.clone()
        };
        if !entry.live.get() || entry.revision.get() != revision {
            return Ok(());
        }
        *entry.view.borrow_mut() = Some(context.clone());
        if let EntryKind::Group(group) = spec.kind {
            self.reconcile_group(entry, &group)?;
        } else {
            let factory = entry.factory.borrow().clone();
            let instance = factory.adapter.create(&context, &spec.config)?;
            let previous = entry.instance.replace(Some(instance));
            drop(previous);
        }
        Ok(())
    }
    fn spawn_outer(&self, entry: &Rc<Entry>) -> Result<(), LoaderError> {
        let spec = entry.spec.borrow().clone();
        if !matches!(spec.kind, EntryKind::Group(_)) && (spec.disabled || self.blocked(entry)) {
            entry.verdict.set(Some(false));
            return Ok(());
        }
        let parent = self.parent_view(entry.parent.borrow().as_ref())?;
        let context = parent
            .invoke_callback(|| spec.scope.apply(&parent))?
            .map_err(|source| LoaderError::Decode {
                factory: spec.factory.clone(),
                source: Rc::from(source),
            })?;
        if !entry.live.get() {
            return Ok(());
        }
        let outer = context.plugin(&self.0.outer.clone().injecting(spec.inject), entry.clone())?;
        *entry.outer.borrow_mut() = Some(outer);
        Ok(())
    }
    fn attach(
        &self,
        id: &EntryId,
        parent: Option<&EntryId>,
        position: usize,
    ) -> Result<(), LoaderError> {
        let entry = self.entry(id)?;
        let previous = entry.parent.replace(parent.cloned());
        if let Some(previous) = previous {
            if let Ok(group) = self.entry(&previous) {
                group.children.borrow_mut().retain(|child| child != id);
            }
        } else {
            self.0.order.borrow_mut().retain(|child| child != id);
        }
        if let Some(parent) = parent {
            let group = self.entry(parent)?;
            let mut children = group.children.borrow_mut();
            let position = position.min(children.len());
            children.insert(position, id.clone());
        } else {
            let mut order = self.0.order.borrow_mut();
            let position = position.min(order.len());
            order.insert(position, id.clone());
        }
        Ok(())
    }
    /// Admit one checked source entry without eagerly running plugin validation.
    pub fn create(&self, spec: EntrySpec) -> Result<EntryId, LoaderError> {
        self.create_in(None, usize::MAX, spec)
    }
    /// Admit an entry into a logical group at a bounded insertion position.
    pub fn create_in(
        &self,
        parent: Option<&EntryId>,
        position: usize,
        spec: EntrySpec,
    ) -> Result<EntryId, LoaderError> {
        self.create_in_origin(parent, position, spec, Origin::Runtime)
    }
    fn create_in_origin(
        &self,
        parent: Option<&EntryId>,
        position: usize,
        spec: EntrySpec,
        origin: Origin,
    ) -> Result<EntryId, LoaderError> {
        self.admit()?;
        let mut requested_ids = Vec::new();
        self.preflight(&spec, 0, &mut requested_ids)?;
        self.capacity(&spec, false)?;
        self.parent_view(parent)?;
        if self.0.entries.borrow().len() + self.0.retiring.borrow().len() >= MAX_ENTRIES {
            return Err(LoaderError::Invalid("entry limit 1024 exceeded".into()));
        }
        let id =
            if let Some(id) = &spec.id {
                EntryId(id.clone())
            } else {
                loop {
                    let next =
                        self.0.next.get().checked_add(1).ok_or_else(|| {
                            LoaderError::Invalid("entry identity exhausted".into())
                        })?;
                    self.0.next.set(next);
                    let id = EntryId(format!("entry-{next}"));
                    if !requested_ids.contains(&id.0)
                        && !self.0.entries.borrow().contains_key(&id)
                        && !self
                            .0
                            .retiring
                            .borrow()
                            .iter()
                            .any(|(entry, _)| entry.id == id)
                    {
                        break id;
                    }
                }
            };
        if self.0.entries.borrow().contains_key(&id)
            || self
                .0
                .retiring
                .borrow()
                .iter()
                .any(|(entry, _)| entry.id == id)
        {
            return Err(LoaderError::DuplicateEntry(id.0));
        }
        let factory = if matches!(spec.kind, EntryKind::Group(_)) {
            self.0.group.clone()
        } else {
            self.0
                .catalog
                .borrow()
                .factories
                .get(&spec.factory)
                .cloned()
                .expect("preflight factory")
        };
        let transport = spec.transport.clone();
        let entry = Rc::new(Entry {
            id: id.clone(),
            spec: RefCell::new(spec),
            factory: RefCell::new(factory),
            instance: RefCell::new(None),
            revision: Cell::new(0),
            live: Cell::new(true),
            running: Cell::new(false),
            work: RefCell::new(None),
            outer: RefCell::new(None),
            parent: RefCell::new(None),
            children: RefCell::new(Vec::new()),
            view: RefCell::new(None),
            verdict: Cell::new(None),
            evaluated: RefCell::new(None),
            origin: RefCell::new(origin.clone()),
            transport: RefCell::new(transport),
            rejected: RefCell::new(None),
        });
        self.0
            .entries
            .borrow_mut()
            .insert(id.clone(), entry.clone());
        self.attach(&id, parent, position)?;
        if let Err(error) = self.spawn_outer(&entry) {
            let removed = self.0.entries.borrow_mut().remove(&id);
            drop(removed);
            return Err(error);
        }
        self.commit(CommitRecord {
            id: id.clone(),
            old: None,
            new: self.snapshot(&id),
            from: None,
            group: parent.cloned(),
            origin,
        })?;
        Ok(id)
    }
    fn reconcile_group(&self, entry: &Rc<Entry>, group: &Group) -> Result<(), LoaderError> {
        let revision = entry.revision.get();
        let current = || entry.live.get() && entry.revision.get() == revision;
        let old = entry.children.borrow().clone();
        let origin = entry.origin.borrow().clone();
        let mut desired = Vec::new();
        for (position, spec) in group.entries.iter().cloned().enumerate() {
            if !current() {
                return Ok(());
            }
            let existing = spec
                .id
                .as_ref()
                .map(|id| EntryId(id.clone()))
                .filter(|id| self.0.entries.borrow().contains_key(id));
            let id = if let Some(id) = existing {
                if self.entry(&id)?.parent.borrow().as_ref() != Some(&entry.id) {
                    drop(self.move_origin(&id, Some(&entry.id), position, origin.clone())?);
                    if !current() {
                        return Ok(());
                    }
                }
                let outcome = self
                    .update_origin(&id, spec.clone(), origin.clone())
                    .map(drop);
                if !current() {
                    return Ok(());
                }
                if let Err(error) = outcome {
                    let child = self.entry(&id)?;
                    let previous = child
                        .rejected
                        .replace(Some(locate_error(error, spec.location)));
                    drop(previous);
                }
                id
            } else {
                let id = self.create_in_origin(Some(&entry.id), position, spec, origin.clone())?;
                if !current() {
                    return Ok(());
                }
                id
            };
            desired.push(id);
        }
        for id in old {
            if !current() {
                return Ok(());
            }
            if !desired.contains(&id)
                && self
                    .entry(&id)
                    .ok()
                    .is_some_and(|child| child.parent.borrow().as_ref() == Some(&entry.id))
            {
                drop(self.remove_origin(&id, origin.clone())?);
            }
        }
        if current() {
            *entry.children.borrow_mut() = desired;
        }
        Ok(())
    }
    /// Read a value-only snapshot with the current structural address.
    pub fn snapshot(&self, id: &EntryId) -> Option<EntrySnapshot> {
        let entry = self.0.entries.borrow().get(id).cloned()?;
        let mut snapshot = entry.snapshot();
        let mut parent = snapshot.parent.clone();
        while let Some(id) = parent {
            let entry = self.0.entries.borrow().get(&id).cloned()?;
            snapshot.address = format!("{}:{}", id, snapshot.address);
            parent = entry.parent.borrow().clone();
        }
        Some(snapshot)
    }
    /// Read all entries in source tree order.
    pub fn entries(&self) -> Vec<EntrySnapshot> {
        fn walk(loader: &Loader, ids: Vec<EntryId>, out: &mut Vec<EntrySnapshot>) {
            for id in ids {
                if let Some(snapshot) = loader.snapshot(&id) {
                    let children = snapshot.children.clone();
                    out.push(snapshot);
                    walk(loader, children, out);
                }
            }
        }
        let mut out = Vec::new();
        walk(self, self.0.order.borrow().clone(), &mut out);
        out
    }
    /// Resolve a current structural address; moving an entry changes this address.
    pub fn resolve(&self, address: &str) -> Result<EntryId, LoaderError> {
        self.entries()
            .into_iter()
            .find(|entry| entry.address == address)
            .map(|entry| entry.id)
            .ok_or_else(|| LoaderError::Invalid(format!("unknown entry address: {address}")))
    }
    fn retire(&self, instance: &Rc<dyn Instance>) -> Result<Retirement, LoaderError> {
        let task = instance.dispose()?.boxed_local().shared();
        let mut retired = self.0.retired.borrow_mut();
        retired.push(task.clone());
        Ok(task)
    }
    fn work(&self, entry: Rc<Entry>, retirement: Option<Retirement>) -> Result<Work, LoaderError> {
        if entry.running.get() {
            return Ok(entry.work.borrow().as_ref().expect("running work").clone());
        }
        let (sender, mut receiver) = tokio::sync::watch::channel(None::<Result<(), LoaderError>>);
        let work = async move {
            loop {
                if let Some(outcome) = receiver.borrow_and_update().clone() {
                    return outcome;
                }
                receiver
                    .changed()
                    .await
                    .map_err(|_| LoaderError::Invalid("loader worker stopped".into()))?;
            }
        }
        .boxed_local()
        .shared();
        entry.running.set(true);
        *entry.work.borrow_mut() = Some(work.clone());
        let loader = self.clone();
        let worker_entry = entry.clone();
        let admission = self.0.owner.context().task(async move {
            let result = loader.drive(worker_entry.clone(), retirement).await;
            worker_entry.running.set(false);
            move || {
                sender.send_replace(Some(result));
            }
        });
        if let Err(error) = admission {
            entry.running.set(false);
            entry.work.borrow_mut().take();
            return Err(error.into());
        }
        Ok(work)
    }
    fn retire_entry(&self, entry: &Rc<Entry>) -> Result<Option<Retirement>, LoaderError> {
        let outer = entry.outer.borrow().clone();
        let instance = entry.instance.borrow().clone();
        let retired = if let Some(outer) = outer.as_ref() {
            let task = outer.dispose()?.boxed_local().shared();
            self.0.retired.borrow_mut().push(task.clone());
            Some(task)
        } else {
            instance
                .as_ref()
                .map(|instance| self.retire(instance))
                .transpose()?
        };
        let detached_outer = entry.outer.borrow_mut().take();
        let detached_inner = entry.instance.borrow_mut().take();
        let view = entry.view.borrow_mut().take();
        let evaluated = entry.evaluated.borrow_mut().take();
        entry.verdict.set(None);
        drop((detached_outer, detached_inner, instance, view, evaluated));
        Ok(retired)
    }
    async fn drive(
        &self,
        entry: Rc<Entry>,
        mut retirement: Option<Retirement>,
    ) -> Result<(), LoaderError> {
        loop {
            if let Some(task) = retirement.take() {
                task.await?;
            }
            if !entry.live.get() || self.0.disposed.get() {
                return Ok(());
            }
            let revision = entry.revision.get();
            self.spawn_outer(&entry)?;
            let outer = entry.outer.borrow().clone();
            let mut result = Ok(());
            if let Some(outer) = outer {
                result = outer.ready()?.await.map(|_| ()).map_err(LoaderError::from);
            }
            let instance = entry.instance.borrow().clone();
            if result.is_ok() {
                if let Some(instance) = instance {
                    result = instance
                        .ready()?
                        .await
                        .map(|_| ())
                        .map_err(LoaderError::from);
                }
            }
            if revision == entry.revision.get() {
                return result;
            }
            retirement = self.retire_entry(&entry)?;
        }
    }
    /// Replace source options. A loaded plugin rejects invalid config before commit.
    /// Factory rebuilds coalesce to the latest accepted source during joined cleanup.
    pub fn update(&self, id: &EntryId, spec: EntrySpec) -> Result<LoaderTask<()>, LoaderError> {
        self.update_origin(id, spec, Origin::Runtime)
    }
    fn update_origin(
        &self,
        id: &EntryId,
        spec: EntrySpec,
        origin: Origin,
    ) -> Result<LoaderTask<()>, LoaderError> {
        self.admit()?;
        if let Some(explicit) = &spec.id {
            if explicit != id.as_str() {
                return Err(LoaderError::Invalid(
                    "update cannot change entry identity".into(),
                ));
            }
        }
        let mut ids = if spec.id.is_none() {
            vec![id.0.clone()]
        } else {
            Vec::new()
        };
        let mut ancestor = self.entry(id)?.parent.borrow().clone();
        while let Some(parent) = ancestor {
            ids.push(parent.0.clone());
            ancestor = self.entry(&parent)?.parent.borrow().clone();
        }
        self.preflight(&spec, 0, &mut ids)?;
        self.capacity(&spec, true)?;
        let factory = if matches!(spec.kind, EntryKind::Group(_)) {
            self.0.group.clone()
        } else {
            self.0
                .catalog
                .borrow()
                .factories
                .get(&spec.factory)
                .cloned()
                .expect("checked factory")
        };
        let entry = self
            .0
            .entries
            .borrow()
            .get(id)
            .cloned()
            .ok_or_else(|| LoaderError::UnknownEntry(id.clone()))?;
        let old = self.snapshot(id);
        let next = entry
            .revision
            .get()
            .checked_add(1)
            .ok_or_else(|| LoaderError::Invalid("entry revision exhausted".into()))?;
        let previous_spec = entry.spec.borrow().clone();
        let obsolete_children = if matches!(previous_spec.kind, EntryKind::Group(_))
            && !matches!(spec.kind, EntryKind::Group(_))
        {
            entry.children.borrow().clone()
        } else {
            Vec::new()
        };
        let same = entry.factory.borrow().id == factory.id
            && (previous_spec.inject.is_empty() && spec.inject.is_empty()
                || Rc::ptr_eq(&previous_spec.inject_token, &spec.inject_token))
            && previous_spec.scope.same(&spec.scope)
            && previous_spec.intercept.same(&spec.intercept)
            && same_callback(&previous_spec.condition, &spec.condition)
            && same_callback(&previous_spec.mapper, &spec.mapper);
        let instance = entry.instance.borrow().clone();
        let direct = if same
            && !spec.disabled
            && !self.blocked(&entry)
            && !entry.running.get()
            && !matches!(spec.kind, EntryKind::Group(_))
            && instance.as_ref().is_some_and(|instance| {
                !matches!(
                    instance.state(),
                    FiberState::Disposed | FiberState::Unloading
                )
            }) {
            instance
                .as_ref()
                .map(|instance| instance.update(&spec.config))
                .transpose()?
        } else {
            None
        };
        let structural = same
            && matches!(spec.kind, EntryKind::Group(_))
            && spec.condition.is_none()
            && spec.intercept.is_empty()
            && !entry.running.get()
            && entry
                .outer
                .borrow()
                .as_ref()
                .is_some_and(|outer| outer.state() == FiberState::Active);
        let retirement = if direct.is_none() && !structural && !entry.running.get() {
            self.retire_entry(&entry)?
        } else {
            None
        };
        let rejected = entry.rejected.borrow_mut().take();
        drop(rejected);
        let previous = entry.spec.replace(spec);
        let old_factory = entry.factory.replace(factory);
        entry.revision.set(next);
        if matches!(origin, Origin::Source(_)) {
            let old_origin = entry.origin.replace(origin.clone());
            drop(old_origin);
        }
        drop((previous, old_factory));
        let group = entry.parent.borrow().clone();
        self.commit(CommitRecord {
            id: id.clone(),
            old,
            new: self.snapshot(id),
            from: None,
            group,
            origin: origin.clone(),
        })?;
        for child in obsolete_children {
            if !entry.live.get() || entry.revision.get() != next {
                break;
            }
            if self
                .entry(&child)
                .ok()
                .is_some_and(|child| child.parent.borrow().as_ref() == Some(id))
            {
                drop(self.remove_origin(&child, origin.clone())?);
            }
        }
        if structural {
            let outer = entry.outer.borrow().clone();
            if let Some(outer) = outer {
                let context = outer.context();
                context.invoke_callback(|| self.start(&context, &entry))??;
            }
            let settled = self.settle()?;
            return Ok(LoaderTask(
                async move {
                    settled.await?;
                    Ok(())
                }
                .boxed_local(),
            ));
        }
        if let Some(task) = direct {
            let work = async move {
                task.await?;
                Ok(())
            }
            .boxed_local()
            .shared();
            let previous = entry.work.replace(Some(work.clone()));
            drop(previous);
            return Ok(LoaderTask(work.boxed_local()));
        }
        let work = self.work(entry, retirement)?;
        Ok(LoaderTask(work.boxed_local()))
    }
    /// Move stable identity to a group. Reordering within one group does not restart.
    /// Crossing groups rebuilds core Fibers after the old generation's cleanup.
    pub fn move_entry(
        &self,
        id: &EntryId,
        parent: Option<&EntryId>,
        position: usize,
    ) -> Result<LoaderTask<()>, LoaderError> {
        self.move_origin(id, parent, position, Origin::Runtime)
    }
    fn move_origin(
        &self,
        id: &EntryId,
        parent: Option<&EntryId>,
        position: usize,
        origin: Origin,
    ) -> Result<LoaderTask<()>, LoaderError> {
        self.admit()?;
        self.parent_view(parent)?;
        let entry = self.entry(id)?;
        let mut ancestor = parent.cloned();
        while let Some(id) = ancestor {
            if id == entry.id {
                return Err(LoaderError::Invalid("group move creates a cycle".into()));
            }
            ancestor = self.entry(&id)?.parent.borrow().clone();
        }
        let old = self.snapshot(id);
        let from = entry.parent.borrow().clone();
        let same = entry.parent.borrow().as_ref() == parent;
        let retirement = if same || entry.running.get() {
            None
        } else {
            self.retire_entry(&entry)?
        };
        self.attach(id, parent, position)?;
        self.commit(CommitRecord {
            id: id.clone(),
            old,
            new: self.snapshot(id),
            from: if same { None } else { from },
            group: parent.cloned(),
            origin,
        })?;
        if same {
            return Ok(LoaderTask(async { Ok(()) }.boxed_local()));
        }
        entry.revision.set(
            entry
                .revision
                .get()
                .checked_add(1)
                .ok_or_else(|| LoaderError::Invalid("entry revision exhausted".into()))?,
        );
        let work = self.work(entry, retirement)?;
        Ok(LoaderTask(work.boxed_local()))
    }
    /// Unlink source and start joined teardown now; dropping the waiter is inert.
    pub fn remove(&self, id: &EntryId) -> Result<LoaderTask<()>, LoaderError> {
        self.remove_origin(id, Origin::Runtime)
    }
    fn remove_origin(&self, id: &EntryId, origin: Origin) -> Result<LoaderTask<()>, LoaderError> {
        self.admit()?;
        let entry = self
            .0
            .entries
            .borrow()
            .get(id)
            .cloned()
            .ok_or_else(|| LoaderError::UnknownEntry(id.clone()))?;
        let old = self.snapshot(id);
        let retirement = self.retire_entry(&entry)?;
        entry.live.set(false);
        let children = entry.children.borrow().clone();
        for child in children {
            if self
                .entry(&child)
                .ok()
                .is_some_and(|child| child.parent.borrow().as_ref() == Some(id))
            {
                drop(self.remove_origin(&child, origin.clone())?);
            }
        }
        let work = entry.work.borrow().clone();
        let parent = entry.parent.borrow().clone();
        if let Some(parent) = parent {
            if let Ok(parent) = self.entry(&parent) {
                parent.children.borrow_mut().retain(|child| child != id);
            }
        } else {
            self.0.order.borrow_mut().retain(|child| child != id);
        }
        self.0
            .retiring
            .borrow_mut()
            .push((entry.clone(), retirement.clone()));
        let removed = self.0.entries.borrow_mut().remove(id);
        drop(removed);
        let group = entry.parent.borrow().clone();
        self.commit(CommitRecord {
            id: id.clone(),
            old,
            new: None,
            from: None,
            group,
            origin,
        })?;
        Ok(LoaderTask(
            async move {
                if let Some(work) = work {
                    work.await?;
                }
                if let Some(task) = retirement {
                    task.await?;
                }
                Ok(())
            }
            .boxed_local(),
        ))
    }
    /// Wait for admitted work, newly included group children and detached cleanup.
    /// Pending succeeds; each latest entry failure is reported independently.
    pub fn settle(&self) -> Result<LoaderTask<Settlement>, LoaderError> {
        self.admit()?;
        let loader = self.clone();
        Ok(LoaderTask(
            async move {
                let mut failures = BTreeMap::new();
                loop {
                    let entries: Vec<_> = loader.0.entries.borrow().values().cloned().collect();
                    let before: Vec<_> = entries
                        .iter()
                        .map(|entry| {
                            (
                                entry.id.clone(),
                                entry.revision.get(),
                                entry
                                    .outer
                                    .borrow()
                                    .as_ref()
                                    .map(|outer| outer.id().clone()),
                                entry
                                    .instance
                                    .borrow()
                                    .as_ref()
                                    .map(|instance| instance.id()),
                            )
                        })
                        .collect();
                    join_retired(&loader.0).await?;
                    for entry in entries {
                        let work = entry.work.borrow().clone();
                        if let Some(work) = work {
                            if let Err(error) = work.await {
                                failures.insert(
                                    entry.id.clone(),
                                    locate_error(error, entry.spec.borrow().location.clone()),
                                );
                                continue;
                            }
                        }
                        let outer = entry.outer.borrow().clone();
                        if let Some(outer) = outer {
                            if let Err(error) = outer.ready()?.await {
                                failures.insert(
                                    entry.id.clone(),
                                    locate_error(
                                        error.into(),
                                        entry.spec.borrow().location.clone(),
                                    ),
                                );
                                continue;
                            }
                        }
                        let instance = entry.instance.borrow().clone();
                        if let Some(instance) = instance {
                            if let Err(error) = instance.ready()?.await {
                                failures.insert(
                                    entry.id.clone(),
                                    locate_error(
                                        error.into(),
                                        entry.spec.borrow().location.clone(),
                                    ),
                                );
                                continue;
                            }
                        }
                        if let Some(error) = entry.rejected.borrow().clone() {
                            failures.insert(entry.id.clone(), error);
                        } else {
                            failures.remove(&entry.id);
                        }
                    }
                    let entries: Vec<_> = loader.0.entries.borrow().values().cloned().collect();
                    let after: Vec<_> = entries
                        .iter()
                        .map(|entry| {
                            (
                                entry.id.clone(),
                                entry.revision.get(),
                                entry
                                    .outer
                                    .borrow()
                                    .as_ref()
                                    .map(|outer| outer.id().clone()),
                                entry
                                    .instance
                                    .borrow()
                                    .as_ref()
                                    .map(|instance| instance.id()),
                            )
                        })
                        .collect();
                    if before == after {
                        break;
                    }
                }
                Ok(Settlement {
                    entries: loader.entries(),
                    failures: failures
                        .into_iter()
                        .map(|(id, error)| EntryFailure { id, error })
                        .collect(),
                })
            }
            .boxed_local(),
        ))
    }
    /// The per-loader accepted runtime source edit event. Reads/reconciliation emit none.
    pub fn commits(&self) -> re_cordis_core::Event<CommitRecord> {
        self.0.commits.clone()
    }
    fn commit(&self, record: CommitRecord) -> Result<(), LoaderError> {
        if record.origin == Origin::Runtime {
            drop(self.0.owner.context().emit(
                &self.0.commits,
                record,
                re_cordis_core::DispatchOptions::default(),
            )?);
        }
        Ok(())
    }
    /// Apply explicit Keep/Set/Clear source patch semantics.
    pub fn patch(&self, id: &EntryId, patch: EntryPatch) -> Result<LoaderTask<()>, LoaderError> {
        let entry = self.entry(id)?;
        let mut spec = entry.spec.borrow().clone();
        match patch.factory {
            FieldPatch::Keep => {}
            FieldPatch::Set(factory) => spec.factory = factory,
            FieldPatch::Clear => {
                return Err(LoaderError::Invalid("factory cannot be cleared".into()));
            }
        }
        match patch.config {
            FieldPatch::Keep => {}
            FieldPatch::Set(config) => spec.config = config,
            FieldPatch::Clear => spec.config = Config::new(()),
        }
        match patch.disabled {
            FieldPatch::Keep => {}
            FieldPatch::Set(disabled) => spec.disabled = disabled,
            FieldPatch::Clear => spec.disabled = false,
        }
        match patch.inject {
            FieldPatch::Keep => {}
            FieldPatch::Set(inject) => spec = spec.injecting(inject),
            FieldPatch::Clear => spec = spec.injecting([]),
        }
        match patch.group {
            FieldPatch::Keep => {}
            FieldPatch::Set(group) => spec.kind = EntryKind::Group(group),
            FieldPatch::Clear => spec.kind = EntryKind::Group(Group::new(Vec::new())),
        }
        self.update(id, spec)
    }
    fn realm(&self, label: &str) -> Result<Rc<NamedRealm>, LoaderError> {
        if label.is_empty() || label.len() > 128 {
            return Err(LoaderError::Invalid(
                "isolation label must contain 1..=128 bytes".into(),
            ));
        }
        let mut realms = self.0.realms.borrow_mut();
        realms.retain(|_, realm| realm.strong_count() > 0);
        if let Some(realm) = realms.get(label).and_then(std::rc::Weak::upgrade) {
            return Ok(realm);
        }
        if realms.len() >= MAX_ENTRIES {
            return Err(LoaderError::Invalid(
                "live named realm limit 1024 exceeded".into(),
            ));
        }
        let realm = Rc::new(NamedRealm {
            id: re_cordis_core::IsolationId::new(),
        });
        realms.insert(label.into(), Rc::downgrade(&realm));
        Ok(realm)
    }
    fn decode_source_node(
        &self,
        node: &serde_json::Value,
        path: Vec<re_cordis_core::PathSegment>,
        failures: &mut Vec<EntryFailure>,
    ) -> Result<EntrySpec, LoaderError> {
        if node.get("group") == Some(&serde_json::Value::Bool(true)) {
            let children = node
                .get("config")
                .and_then(|value| value.as_array())
                .ok_or_else(|| {
                    let mut config_path = path.clone();
                    config_path.push(re_cordis_core::PathSegment::Key("config".into()));
                    locate_error(
                        LoaderError::Invalid("group config must be array".into()),
                        Some(SourceLocation {
                            source: None,
                            path: config_path,
                            span: None,
                        }),
                    )
                })?;
            let mut valid = Vec::new();
            for (index, child) in children.iter().enumerate() {
                let mut child_path = path.clone();
                child_path.push(re_cordis_core::PathSegment::Key("config".into()));
                child_path.push(re_cordis_core::PathSegment::Index(index));
                match self.decode_source_node(child, child_path, failures) {
                    Ok(spec) => valid.push(spec),
                    Err(error) => failures.push(EntryFailure {
                        id: EntryId(
                            child
                                .get("id")
                                .and_then(|id| id.as_str())
                                .unwrap_or("invalid-child")
                                .into(),
                        ),
                        error,
                    }),
                }
            }
            let mut shallow = node.clone();
            shallow["config"] = serde_json::Value::Array(Vec::new());
            let mut spec = self.decode_node(&shallow, path)?;
            spec.kind = EntryKind::Group(Group::new(valid));
            spec.transport = Some(node.clone());
            Ok(spec)
        } else {
            self.decode_node(node, path)
        }
    }
    fn decode_node(
        &self,
        node: &serde_json::Value,
        path: Vec<re_cordis_core::PathSegment>,
    ) -> Result<EntrySpec, LoaderError> {
        self.0
            .owner
            .context()
            .invoke_callback(|| self.decode_node_impl(node, path))?
    }
    fn decode_node_impl(
        &self,
        node: &serde_json::Value,
        path: Vec<re_cordis_core::PathSegment>,
    ) -> Result<EntrySpec, LoaderError> {
        use re_cordis_core::PathSegment;
        let fail = |name: &str, message: &str| LoaderError::Located {
            location: SourceLocation {
                source: None,
                path: {
                    let mut p = path.clone();
                    p.push(PathSegment::Key(name.into()));
                    p
                },
                span: None,
            },
            source: Rc::new(LoaderError::Invalid(message.into())),
        };
        let object = node
            .as_object()
            .ok_or_else(|| fail("", "entry must be an object"))?;
        for key in object.keys() {
            if ![
                "id",
                "name",
                "config",
                "group",
                "disabled",
                "inject",
                "isolate",
                "intercept",
            ]
            .contains(&key.as_str())
            {
                return Err(fail(key, "unknown entry field"));
            }
        }
        let structural = match object.get("group") {
            None | Some(serde_json::Value::Null) | Some(serde_json::Value::Bool(false)) => false,
            Some(serde_json::Value::Bool(true)) => true,
            _ => return Err(fail("group", "group must be boolean or null")),
        };
        let mut spec = if structural {
            let children = object
                .get("config")
                .and_then(|value| value.as_array())
                .ok_or_else(|| fail("config", "group config must be an array"))?;
            let mut specs = Vec::new();
            for (index, child) in children.iter().enumerate() {
                let mut child_path = path.clone();
                child_path.push(PathSegment::Key("config".into()));
                child_path.push(PathSegment::Index(index));
                specs.push(self.decode_node(child, child_path)?);
            }
            EntrySpec::group(Group::new(specs))
        } else {
            let name = object
                .get("name")
                .and_then(|value| value.as_str())
                .ok_or_else(|| fail("name", "plugin name must be a string"))?;
            let factory = self
                .0
                .catalog
                .borrow()
                .factories
                .get(name)
                .cloned()
                .ok_or_else(|| fail("name", &format!("unknown factory: {name}")))?;
            let decoder = factory
                .decoder
                .ok_or_else(|| fail("config", "factory has no JSON decoder"))?;
            let config = self
                .0
                .owner
                .context()
                .invoke_callback(|| decoder(object.get("config")))?
                .map_err(|source| LoaderError::Located {
                    location: SourceLocation {
                        source: None,
                        path: {
                            let mut p = path.clone();
                            p.push(PathSegment::Key("config".into()));
                            p
                        },
                        span: None,
                    },
                    source: Rc::from(source),
                })?;
            EntrySpec::new(name, config)
        };
        if let Some(id) = object.get("id") {
            if !id.is_null() {
                let id = id
                    .as_str()
                    .ok_or_else(|| fail("id", "id must be string or null"))?;
                if id.is_empty() || id.len() > 128 || id.contains(':') {
                    return Err(fail(
                        "id",
                        "entry ID must contain 1..=128 UTF-8 bytes and no ':'",
                    ));
                }
                spec.id = Some(id.into());
            }
        }
        if let Some(value) = object.get("disabled") {
            if !value.is_null() {
                spec.disabled = value.as_bool().ok_or_else(|| {
                    fail(
                        "disabled",
                        "disabled must be boolean; JavaScript expressions are unsupported",
                    )
                })?;
            }
        }
        if let Some(value) = object.get("inject").filter(|value| !value.is_null()) {
            let mut dependencies = Vec::new();
            if let Some(names) = value.as_array() {
                for name in names {
                    let name = name
                        .as_str()
                        .ok_or_else(|| fail("inject", "injection names must be strings"))?;
                    let service = self
                        .0
                        .catalog
                        .borrow()
                        .services
                        .get(name)
                        .cloned()
                        .ok_or_else(|| {
                            fail("inject", &format!("unknown service contract: {name}"))
                        })?;
                    dependencies.push(service.dependency);
                }
            } else if let Some(names) = value.as_object() {
                for (name, value) in names {
                    let service = self
                        .0
                        .catalog
                        .borrow()
                        .services
                        .get(name)
                        .cloned()
                        .ok_or_else(|| {
                            fail("inject", &format!("unknown service contract: {name}"))
                        })?;
                    if value.is_null() {
                        dependencies.push(service.dependency)
                    } else {
                        let configure = service.configure.ok_or_else(|| {
                            fail("inject", "service has no typed configuration decoder")
                        })?;
                        dependencies.push(
                            configure(value)
                                .map_err(|source| LoaderError::Located {
                                    location: SourceLocation {
                                        source: None,
                                        path: path.clone(),
                                        span: None,
                                    },
                                    source: Rc::from(source),
                                })?
                                .0,
                        );
                    }
                }
            } else {
                return Err(fail("inject", "inject must be an array or object"));
            }
            spec = spec.injecting(dependencies);
        }
        if let Some(value) = object.get("isolate").filter(|value| !value.is_null()) {
            let names = value
                .as_object()
                .ok_or_else(|| fail("isolate", "isolate must be an object"))?;
            for (name, value) in names {
                let service = self
                    .0
                    .catalog
                    .borrow()
                    .services
                    .get(name)
                    .cloned()
                    .ok_or_else(|| fail("isolate", &format!("unknown service contract: {name}")))?;
                let slot = match value {
                    serde_json::Value::Bool(true) => Some(re_cordis_core::IsolationId::new()),
                    serde_json::Value::Bool(false) | serde_json::Value::Null => None,
                    serde_json::Value::String(label) => {
                        let realm = self.realm(label)?;
                        let slot = realm.id.clone();
                        spec.scope.retain_realm(realm);
                        Some(slot)
                    }
                    _ => {
                        return Err(fail(
                            "isolate",
                            "isolation is true, false, null or shared label",
                        ));
                    }
                };
                if let Some(slot) = slot {
                    spec.scope = (service.isolate)(&spec.scope, &slot);
                }
            }
        }
        if let Some(value) = object.get("intercept").filter(|value| !value.is_null()) {
            let names = value
                .as_object()
                .ok_or_else(|| fail("intercept", "intercept must be an object"))?;
            for (name, value) in names {
                let service = self
                    .0
                    .catalog
                    .borrow()
                    .services
                    .get(name)
                    .cloned()
                    .ok_or_else(|| {
                        fail("intercept", &format!("unknown service contract: {name}"))
                    })?;
                let configure = service
                    .configure
                    .ok_or_else(|| fail("intercept", "service has no typed intercept decoder"))?;
                let scope = configure(value)
                    .map_err(|source| LoaderError::Located {
                        location: SourceLocation {
                            source: None,
                            path: path.clone(),
                            span: None,
                        },
                        source: Rc::from(source),
                    })?
                    .1;
                spec.intercept.extend(&scope);
            }
        }
        self.preflight(&spec, 0, &mut Vec::new())?;
        spec.transport = Some(node.clone());
        spec.location = Some(SourceLocation {
            source: None,
            path,
            span: None,
        });
        Ok(spec)
    }
    /// Decode a complete runtime JSON array. All entries are preflighted together.
    pub fn decode_json(&self, text: &str) -> Result<Vec<EntrySpec>, LoaderError> {
        self.admit()?;
        let value = source::parse(text)?;
        let items = value
            .as_array()
            .ok_or_else(|| LoaderError::Invalid("JSON source must be an array".into()))?;
        let mut specs = Vec::new();
        let mut ids = Vec::new();
        for (index, item) in items.iter().enumerate() {
            let spec = self.decode_node(item, vec![re_cordis_core::PathSegment::Index(index)])?;
            self.preflight(&spec, 0, &mut ids)?;
            specs.push(spec);
        }
        Ok(specs)
    }
    /// Decode and admit one complete runtime JSON entry, preflighting its whole group.
    pub fn create_json(&self, text: &str) -> Result<EntryId, LoaderError> {
        self.admit()?;
        let value = source::parse(text)?;
        let spec = self.decode_node(&value, Vec::new())?;
        self.create(spec)
    }
    fn prepare_source_spec(
        &self,
        mut spec: EntrySpec,
        document: &SourceDocument,
        path: Vec<re_cordis_core::PathSegment>,
        failures: &mut Vec<EntryFailure>,
        ids: &mut Vec<String>,
    ) -> Result<EntrySpec, LoaderError> {
        let path = spec
            .location
            .as_ref()
            .filter(|location| location.source.is_none())
            .map(|location| location.path.clone())
            .unwrap_or(path);
        spec.location = Some(document.location(path.clone()));
        let children = if let EntryKind::Group(group) = &mut spec.kind {
            Some(std::mem::take(&mut group.entries))
        } else {
            None
        };
        let mut candidate_ids = ids.clone();
        self.preflight(&spec, path.len() / 2, &mut candidate_ids)
            .map_err(|error| locate_error(error, spec.location.clone()))?;
        *ids = candidate_ids;
        if let Some(children) = children {
            let mut valid = Vec::new();
            for (index, child) in children.into_iter().enumerate() {
                let id = child
                    .id
                    .clone()
                    .map(EntryId)
                    .unwrap_or_else(|| EntryId(format!("source-child-{index}")));
                let mut child_path = path.clone();
                child_path.push(re_cordis_core::PathSegment::Key("config".into()));
                child_path.push(re_cordis_core::PathSegment::Index(index));
                match self.prepare_source_spec(child, document, child_path, failures, ids) {
                    Ok(child) => valid.push(child),
                    Err(error) => failures.push(EntryFailure { id, error }),
                }
            }
            if let EntryKind::Group(group) = &mut spec.kind {
                group.entries = valid;
            }
        }
        Ok(spec)
    }
    /// Reconcile a read-only in-memory source; independent valid siblings survive errors.
    pub fn reconcile(
        &self,
        document: SourceDocument,
    ) -> Result<LoaderTask<Settlement>, LoaderError> {
        self.admit()?;
        let origin = Origin::Source(document.id.clone());
        let old = self
            .0
            .sources
            .borrow()
            .get(&document.id)
            .cloned()
            .unwrap_or_default();
        let mut nested_failures = Vec::new();
        let nodes: Vec<(Result<EntrySpec, LoaderError>, Option<serde_json::Value>)> =
            match &document.input {
                source::SourceInput::Typed(specs) => {
                    specs.iter().cloned().map(|spec| (Ok(spec), None)).collect()
                }
                source::SourceInput::Json(nodes) => nodes
                    .iter()
                    .enumerate()
                    .map(|(index, node)| {
                        (
                            self.decode_source_node(
                                node,
                                vec![re_cordis_core::PathSegment::Index(index)],
                                &mut nested_failures,
                            ),
                            Some(node.clone()),
                        )
                    })
                    .collect(),
            };
        let mut desired = Vec::new();
        let mut descendants = Vec::new();
        let mut source_ids = Vec::new();
        let mut failures = nested_failures
            .into_iter()
            .map(|failure| EntryFailure {
                id: failure.id,
                error: match failure.error {
                    LoaderError::Located { location, source } => LoaderError::Located {
                        location: document.location(location.path),
                        source,
                    },
                    error => error,
                },
            })
            .collect::<Vec<_>>();
        for (index, (result, transport)) in nodes.into_iter().enumerate() {
            let explicit = result
                .as_ref()
                .ok()
                .and_then(|spec| spec.id.as_ref())
                .map(|id| EntryId(id.clone()))
                .or_else(|| {
                    transport
                        .as_ref()
                        .and_then(|node| node.get("id"))
                        .and_then(|id| id.as_str())
                        .map(|id| EntryId(id.into()))
                });
            let result = result
                .and_then(|spec| {
                    self.prepare_source_spec(
                        spec,
                        &document,
                        vec![re_cordis_core::PathSegment::Index(index)],
                        &mut failures,
                        &mut source_ids,
                    )
                })
                .map_err(|error| match error {
                    LoaderError::Located { location, source } => LoaderError::Located {
                        location: document.location(location.path),
                        source,
                    },
                    error => LoaderError::Located {
                        location: document
                            .location(vec![re_cordis_core::PathSegment::Index(index)]),
                        source: Rc::new(error),
                    },
                });
            let spec = match result {
                Ok(spec) => spec,
                Err(error) => {
                    if let Some(id) = explicit
                        .as_ref()
                        .filter(|id| old.contains(id) && !desired.contains(*id))
                    {
                        if self.entry(id).ok().is_some_and(|entry| {
                            *entry.origin.borrow() == origin && entry.parent.borrow().is_none()
                        }) {
                            drop(self.remove_origin(id, origin.clone())?);
                        }
                    }
                    failures.push(EntryFailure {
                        id: explicit.unwrap_or_else(|| EntryId(format!("source-{index}"))),
                        error,
                    });
                    continue;
                }
            };
            let existing = spec
                .id
                .as_ref()
                .map(|id| EntryId(id.clone()))
                .or_else(|| {
                    old.get(index)
                        .filter(|id| {
                            self.entry(id).ok().is_some_and(|entry| {
                                *entry.origin.borrow() == origin && entry.parent.borrow().is_none()
                            })
                        })
                        .cloned()
                })
                .filter(|id| self.entry(id).is_ok());
            let nested = nested_ids(&spec);
            let retained = existing.clone();
            let admitted = if let Some(id) = existing {
                drop(self.move_origin(&id, None, index, origin.clone())?);
                self.update_origin(&id, spec, origin.clone()).map(|task| {
                    drop(task);
                    id
                })
            } else {
                self.create_in_origin(None, index, spec, origin.clone())
            };
            match admitted {
                Ok(id) => {
                    descendants.extend(nested);
                    if !source_ids.contains(&id.0) {
                        source_ids.push(id.0.clone());
                    }
                    let entry = self.entry(&id)?;
                    let previous = entry.transport.replace(transport);
                    drop(previous);
                    desired.push(id);
                }
                Err(error) => {
                    if let Some(id) = retained {
                        desired.push(id);
                    }
                    failures.push(EntryFailure {
                        id: explicit.unwrap_or_else(|| EntryId(format!("source-{index}"))),
                        error: locate_error(
                            error,
                            Some(
                                document.location(vec![re_cordis_core::PathSegment::Index(index)]),
                            ),
                        ),
                    });
                }
            }
        }
        for id in old {
            if !desired.contains(&id)
                && !descendants.contains(&id)
                && self.entry(&id).ok().is_some_and(|entry| {
                    *entry.origin.borrow() == origin && entry.parent.borrow().is_none()
                })
            {
                drop(self.remove_origin(&id, origin.clone())?);
            }
        }
        let previous = if desired.is_empty() {
            self.0.sources.borrow_mut().remove(&document.id)
        } else {
            self.0.sources.borrow_mut().insert(document.id, desired)
        };
        drop(previous);
        let settled = self.settle()?;
        Ok(LoaderTask(
            async move {
                let mut report = settled.await?;
                report.failures.extend(failures);
                Ok(report)
            }
            .boxed_local(),
        ))
    }
    /// Replace a catalog declaration and rebuild linked entries after joined cleanup.
    /// This in-memory seam performs no watching or dynamic library/module loading.
    pub fn replace_factory<C: Clone + 'static, R: 'static>(
        &self,
        name: &str,
        plugin: Plugin<C, R>,
    ) -> Result<LoaderTask<Settlement>, LoaderError> {
        self.admit()?;
        let previous = self
            .0
            .catalog
            .borrow()
            .factories
            .get(name)
            .cloned()
            .ok_or_else(|| LoaderError::UnknownFactory(name.into()))?;
        let adapter: Rc<dyn Factory> = Rc::new(TypedFactory {
            name: name.into(),
            plugin,
        });
        if adapter.contract() != previous.adapter.contract() {
            return Err(LoaderError::Invalid(
                "factory replacement must preserve the source config contract".into(),
            ));
        }
        let entries: Vec<_> = self
            .0
            .entries
            .borrow()
            .values()
            .filter(|entry| {
                entry.spec.borrow().factory == name
                    && matches!(entry.spec.borrow().kind, EntryKind::Plugin)
            })
            .cloned()
            .collect();
        for entry in &entries {
            adapter.check(&entry.spec.borrow().config)?;
        }
        let next = self
            .0
            .catalog
            .borrow()
            .next
            .checked_add(1)
            .ok_or_else(|| LoaderError::Invalid("factory identity exhausted".into()))?;
        let record = FactoryRecord {
            id: FactoryId(next),
            adapter,
            decoder: previous.decoder,
        };
        self.0.catalog.borrow_mut().next = next;
        let old = self
            .0
            .catalog
            .borrow_mut()
            .factories
            .insert(name.into(), record);
        drop(old);
        for entry in entries {
            let spec = entry.spec.borrow().clone();
            let origin = entry.origin.borrow().clone();
            drop(self.update_origin(&entry.id, spec, origin)?);
        }
        self.settle()
    }
    /// Start disposal of the whole dedicated owner and join all detached cleanup.
    pub fn dispose(&self) -> Result<LoaderTask<()>, LoaderError> {
        let task = self.0.owner.dispose()?;
        self.0.disposed.set(true);
        let state = self.0.clone();
        Ok(LoaderTask(
            async move {
                let outcome = task.await;
                let retirement = join_retired(&state).await;
                outcome?;
                retirement?;
                Ok(())
            }
            .boxed_local(),
        ))
    }
}

fn same_callback<T: ?Sized>(a: &Option<Rc<T>>, b: &Option<Rc<T>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        _ => false,
    }
}

fn locate_error(error: LoaderError, location: Option<SourceLocation>) -> LoaderError {
    match location {
        Some(location) => LoaderError::Located {
            location,
            source: Rc::new(error),
        },
        None => error,
    }
}

async fn join_retired(state: &State) -> Result<(), LifecycleError> {
    let tasks = state.retired.borrow().clone();
    for task in tasks {
        if let Err(error) = task.await {
            if state.retirement_error.borrow().is_none() {
                *state.retirement_error.borrow_mut() = Some(error);
            }
        }
    }
    match state.retirement_error.borrow().clone() {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

fn nested_ids(spec: &EntrySpec) -> Vec<EntryId> {
    let mut ids = Vec::new();
    if let EntryKind::Group(group) = &spec.kind {
        for child in &group.entries {
            if let Some(id) = &child.id {
                ids.push(EntryId(id.clone()));
            }
            ids.extend(nested_ids(child));
        }
    }
    ids
}

#[cfg(test)]
mod retirement_tests {
    use super::*;
    #[tokio::test(flavor = "current_thread")]
    async fn pruned_lifecycle_error_remains_visible_and_parent_joins_later_retirement() {
        let runtime = re_cordis_core::Runtime::new();
        runtime
            .run(async {
                let root = runtime.context();
                let loader = Loader::new(&root, Catalog::new()).unwrap();
                loader.0.retired.borrow_mut().push(
                    async { Err(LifecycleError::HostStopped) }
                        .boxed_local()
                        .shared(),
                );
                let (release, gate) = tokio::sync::oneshot::channel();
                let joined = Rc::new(Cell::new(false));
                loader.0.retired.borrow_mut().push({
                    let joined = joined.clone();
                    async move {
                        gate.await.unwrap();
                        joined.set(true);
                        Ok(FiberState::Disposed)
                    }
                    .boxed_local()
                    .shared()
                });
                loader.admit().unwrap();
                let mut shutdown = Box::pin(root.drain().unwrap());
                assert!(futures_util::poll!(&mut shutdown).is_pending());
                assert!(!joined.get());
                release.send(()).unwrap();
                shutdown.await.unwrap();
                assert!(joined.get());
                assert!(runtime.take_failures().iter().any(|failure| {
                    let mut error: &(dyn Error + 'static) = failure;
                    loop {
                        if error
                            .downcast_ref::<LifecycleError>()
                            .is_some_and(|error| matches!(error, LifecycleError::HostStopped))
                        {
                            return true;
                        }
                        match error.source() {
                            Some(source) => error = source,
                            None => return false,
                        }
                    }
                }));
            })
            .await
            .unwrap();
    }

    #[tokio::test(flavor = "current_thread")]
    async fn empty_source_cycles_release_source_metadata() {
        let runtime = re_cordis_core::Runtime::new();
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            runtime.run(async {
                let loader = Loader::new(&runtime.context(), Catalog::new()).unwrap();
                for index in 0..2048 {
                    loader
                        .reconcile(SourceDocument::typed(
                            SourceId::new(format!("empty-{index}")),
                            vec![],
                        ))
                        .unwrap()
                        .await
                        .unwrap();
                }
                assert!(loader.0.sources.borrow().is_empty());
                loader.dispose().unwrap().await.unwrap();
            }),
        )
        .await
        .unwrap()
        .unwrap();
    }
    #[tokio::test(flavor = "current_thread")]
    async fn rejected_json_cycles_release_named_realm_metadata() {
        let runtime = re_cordis_core::Runtime::new();
        tokio::time::timeout(std::time::Duration::from_secs(5),runtime.run(async{
            let root=runtime.context();let key=re_cordis_core::ServiceKey::<u32>::new("value");let mut catalog=Catalog::new();catalog.register_service(&key).unwrap();
            catalog.register_json("worker",Plugin::new("worker",|_,():()|async{Ok::<_,Infallible>(())}),|_|Ok::<_,Infallible>(())).unwrap();
            let loader=Loader::new(&root,catalog).unwrap();
            for index in 0..2048{assert!(loader.create_json(&format!(r#"{{"name":"worker","isolate":{{"value":"failed-{index}"}},"intercept":{{"unknown":1}}}}"#)).is_err());}
            loader.admit().unwrap();assert!(loader.0.realms.borrow().is_empty());
            loader.dispose().unwrap().await.unwrap();
        })).await.unwrap().unwrap();
    }
}
