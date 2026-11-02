//! Immutable typed scope layers.
use re_cordis_core::{BoxError, Context, IsolationId, ServiceKey};
use std::{fmt, rc::Rc};
type Layer = dyn Fn(&Context) -> Result<Context, BoxError>;
/// Immutable service isolation and intercept layers, reused across rebuilds.
#[derive(Clone, Default)]
pub struct Scope {
    layers: Vec<Rc<Layer>>,
    realms: Vec<Rc<crate::NamedRealm>>,
}
impl fmt::Debug for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scope")
            .field("layers", &self.layers.len())
            .finish()
    }
}
impl Scope {
    /// An inherited scope with no extra layers.
    pub fn new() -> Self {
        Self::default()
    }
    /// Allocate a fresh typed service realm for this scope definition.
    pub fn isolate<T: 'static>(self, key: &ServiceKey<T>) -> Self {
        self.isolate_with(key, &IsolationId::new())
    }
    /// Select an explicit shared typed service realm.
    pub fn isolate_with<T: 'static>(mut self, key: &ServiceKey<T>, slot: &IsolationId) -> Self {
        let key = key.clone();
        let slot = slot.clone();
        self.layers
            .push(Rc::new(move |ctx| Ok(ctx.isolate_with(&key, &slot))));
        self
    }
    /// Add typed use-site intercept configuration.
    pub fn intercept<T: 'static, C: Clone + 'static>(
        mut self,
        key: &ServiceKey<T>,
        config: C,
    ) -> Self {
        let key = key.clone();
        self.layers
            .push(Rc::new(move |ctx| Ok(ctx.intercept(&key, config.clone()))));
        self
    }
    /// Evaluate a typed intercept after entry injection is committed.
    pub fn intercept_with<T: 'static, C: 'static, F, E>(
        mut self,
        key: &ServiceKey<T>,
        evaluate: F,
    ) -> Self
    where
        F: Fn(&Context) -> Result<C, E> + 'static,
        E: Into<BoxError> + 'static,
    {
        let key = key.clone();
        self.layers.push(Rc::new(move |ctx| {
            Ok(ctx.intercept(&key, evaluate(ctx).map_err(Into::into)?))
        }));
        self
    }
    pub(crate) fn extend(&mut self, other: &Self) {
        self.layers.extend(other.layers.iter().cloned());
        self.realms.extend(other.realms.iter().cloned());
    }
    pub(crate) fn apply(&self, context: &Context) -> Result<Context, BoxError> {
        let mut context = context.clone();
        for layer in &self.layers {
            context = layer(&context)?;
        }
        Ok(context)
    }
    pub(crate) fn retain_realm(&mut self, realm: Rc<crate::NamedRealm>) {
        self.realms.push(realm);
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }
    pub(crate) fn same(&self, other: &Self) -> bool {
        self.layers.len() == other.layers.len()
            && self
                .layers
                .iter()
                .zip(&other.layers)
                .all(|(a, b)| Rc::ptr_eq(a, b))
    }
}
