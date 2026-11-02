//! Bounded JSON transport and in-memory source identity.
use crate::{EntrySpec, LoaderError};
use re_cordis_core::PathSegment;
use serde_json::Value;
use std::{fmt, rc::Rc};
/// Maximum UTF-8 JSON input bytes.
pub const MAX_JSON_BYTES: usize = 1_048_576;
/// Maximum nested JSON value depth.
pub const MAX_JSON_DEPTH: usize = 32;
/// Maximum JSON nodes.
pub const MAX_JSON_NODES: usize = 65_536;
/// Maximum JSON string bytes.
pub const MAX_STRING_BYTES: usize = 65_536;
/// Maximum resident plus retiring entries.
pub const MAX_ENTRIES: usize = 1024;
/// Explicit identity of an in-memory source. No file I/O is implied.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceId(String);
impl SourceId {
    /// Identify a caller-managed source.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
    /// Caller-provided source label.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for SourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
/// Optional actual source position, supplied by the transport owner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceSpan {
    /// One-based line.
    pub line: usize,
    /// One-based column.
    pub column: usize,
}
/// Structured source location. Runtime input has no source identity or span.
#[derive(Clone, Debug)]
pub struct SourceLocation {
    /// Caller-managed source identity.
    pub source: Option<SourceId>,
    /// Structured field path.
    pub path: Vec<PathSegment>,
    /// Actual source span when known.
    pub span: Option<SourceSpan>,
}
type Locator = dyn Fn(&[PathSegment]) -> Option<SourceSpan>;
#[derive(Clone)]
pub(crate) enum SourceInput {
    Typed(Vec<EntrySpec>),
    Json(Vec<Value>),
}
/// Read-only typed or JSON source snapshot. Persistence belongs to its caller.
#[derive(Clone)]
pub struct SourceDocument {
    /// Source identity.
    pub id: SourceId,
    pub(crate) input: SourceInput,
    pub(crate) locator: Option<Rc<Locator>>,
}
impl SourceDocument {
    /// A native source snapshot with no serialization requirement.
    pub fn typed(id: SourceId, entries: Vec<EntrySpec>) -> Self {
        Self {
            id,
            input: SourceInput::Typed(entries),
            locator: None,
        }
    }
    /// Parse a bounded JSON array, retaining null versus absent config.
    pub fn json(id: SourceId, text: &str) -> Result<Self, LoaderError> {
        let value = parse(text)?;
        let entries = value
            .as_array()
            .ok_or_else(|| LoaderError::Invalid("source must be a JSON array".into()))?
            .clone();
        Ok(Self {
            id,
            input: SourceInput::Json(entries),
            locator: None,
        })
    }
    /// Supply actual source spans; the loader never invents filenames or offsets.
    pub fn with_locations<F: Fn(&[PathSegment]) -> Option<SourceSpan> + 'static>(
        mut self,
        locate: F,
    ) -> Self {
        self.locator = Some(Rc::new(locate));
        self
    }
    pub(crate) fn location(&self, path: Vec<PathSegment>) -> SourceLocation {
        let span = self.locator.as_ref().and_then(|locate| locate(&path));
        SourceLocation {
            source: Some(self.id.clone()),
            path,
            span,
        }
    }
}
pub(crate) fn parse(text: &str) -> Result<Value, LoaderError> {
    if text.len() > MAX_JSON_BYTES {
        return Err(LoaderError::Invalid(
            "JSON byte limit 1048576 exceeded".into(),
        ));
    }
    let value: Value = serde_json::from_str(text).map_err(|error| LoaderError::Located {
        location: SourceLocation {
            source: None,
            path: Vec::new(),
            span: Some(SourceSpan {
                line: error.line(),
                column: error.column(),
            }),
        },
        source: Rc::new(error),
    })?;
    fn walk(value: &Value, depth: usize, nodes: &mut usize) -> Result<(), LoaderError> {
        *nodes += 1;
        if depth > MAX_JSON_DEPTH || *nodes > MAX_JSON_NODES {
            return Err(LoaderError::Invalid(
                "JSON depth or node limit exceeded".into(),
            ));
        }
        match value {
            Value::String(s) if s.len() > MAX_STRING_BYTES => {
                return Err(LoaderError::Invalid(
                    "JSON string limit 65536 exceeded".into(),
                ));
            }
            Value::Array(items) => {
                for item in items {
                    walk(item, depth + 1, nodes)?
                }
            }
            Value::Object(items) => {
                for (key, item) in items {
                    if key.len() > MAX_STRING_BYTES {
                        return Err(LoaderError::Invalid(
                            "JSON key string limit exceeded".into(),
                        ));
                    }
                    walk(item, depth + 1, nodes)?
                }
            }
            _ => {}
        }
        Ok(())
    }
    walk(&value, 0, &mut 0)?;
    Ok(value)
}
/// Whether a field is retained, explicitly set, or removed to its default.
#[derive(Clone, Debug, Default)]
pub enum FieldPatch<T> {
    /// Retain current source.
    #[default]
    Keep,
    /// Set a new value, including a native nullable value.
    Set(T),
    /// Clear the field to its documented default.
    Clear,
}
/// Partial source mutation. Runtime validation precedes commit.
#[derive(Clone, Debug, Default)]
pub struct EntryPatch {
    /// Replace factory name; clearing is invalid.
    pub factory: FieldPatch<String>,
    /// Replace source; clearing stores the native unit value.
    pub config: FieldPatch<crate::Config>,
    /// Replace disabled flag; clearing enables.
    pub disabled: FieldPatch<bool>,
    /// Replace entry injection; clearing removes entry requirements.
    pub inject: FieldPatch<Vec<re_cordis_core::Dependency>>,
    /// Replace structural children; clearing makes the group empty.
    pub group: FieldPatch<crate::Group>,
}
