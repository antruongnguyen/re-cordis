use crate::{IncludeError, Location, MAX_BYTES, MAX_DEPTH, MAX_NODES, MAX_STRING_BYTES};
use re_cordis_core::PathSegment;
use re_cordis_loader::SourceSpan;
use serde_json::{Map, Number, Value};
use std::{collections::BTreeMap, path::Path, str::Chars};
use yaml_rust2::{
    parser::{Event, Parser},
    scanner::{Marker, TScalarStyle},
};
/// A bounded parsed value with actual Unicode-scalar source positions.
#[derive(Clone, Debug)]
pub struct ParsedDocument {
    /// Parsed JSON-compatible value.
    pub value: Value,
    /// Positions indexed by structured value path.
    pub locations: BTreeMap<String, SourceSpan>,
}
impl ParsedDocument {
    /// Locate a structured path using actual parser positions.
    pub fn location(&self, path: &[PathSegment]) -> Option<&SourceSpan> {
        self.locations.get(&crate::source::path_key(path))
    }
}
struct Reader<'a> {
    parser: Parser<Chars<'a>>,
    file: &'a Path,
    nodes: usize,
    locations: BTreeMap<String, SourceSpan>,
}
impl Reader<'_> {
    fn error(&self, message: impl Into<String>, m: Marker, path: Vec<PathSegment>) -> IncludeError {
        IncludeError::located(
            message,
            Location {
                file: self.file.into(),
                path,
                span: Some(SourceSpan {
                    line: m.line(),
                    column: m.col() + 1,
                }),
            },
        )
    }
    fn next(&mut self) -> Result<(Event, Marker), IncludeError> {
        self.parser.next_token().map_err(|e| {
            IncludeError::located(
                e.to_string(),
                Location {
                    file: self.file.into(),
                    path: vec![],
                    span: Some(SourceSpan {
                        line: e.marker().line(),
                        column: e.marker().col() + 1,
                    }),
                },
            )
        })
    }
    fn node(
        &mut self,
        event: Event,
        m: Marker,
        path: Vec<PathSegment>,
        depth: usize,
    ) -> Result<Value, IncludeError> {
        self.nodes += 1;
        if depth > MAX_DEPTH || self.nodes > MAX_NODES {
            return Err(self.error("value depth or node limit exceeded", m, path));
        }
        self.locations.insert(
            crate::source::path_key(&path),
            SourceSpan {
                line: m.line(),
                column: m.col() + 1,
            },
        );
        match event {
            Event::Alias(_) => Err(self.error("YAML aliases are unsupported", m, path)),
            Event::Scalar(s, style, anchor, tag) => {
                if anchor != 0 || tag.is_some() {
                    return Err(self.error(
                        "YAML anchors and explicit tags are unsupported",
                        m,
                        path,
                    ));
                }
                if s.len() > MAX_STRING_BYTES {
                    return Err(self.error("string byte limit exceeded", m, path));
                }
                if style != TScalarStyle::Plain {
                    return Ok(Value::String(s));
                }
                if matches!(
                    s.to_ascii_lowercase().as_str(),
                    ".inf" | "+.inf" | "-.inf" | ".nan"
                ) {
                    return Err(self.error("non-finite YAML numbers are unsupported", m, path));
                }
                match s.as_str() {
                    "" | "~" | "null" | "Null" | "NULL" => Ok(Value::Null),
                    "true" | "True" | "TRUE" => Ok(Value::Bool(true)),
                    "false" | "False" | "FALSE" => Ok(Value::Bool(false)),
                    ".inf" | ".Inf" | ".INF" | "-.inf" | "+.inf" | ".nan" | ".NaN" | ".NAN" => {
                        Err(self.error("non-finite YAML numbers are unsupported", m, path))
                    }
                    _ => Ok(number(&s).map(Value::Number).unwrap_or(Value::String(s))),
                }
            }
            Event::SequenceStart(anchor, tag) => {
                if anchor != 0 || tag.is_some() {
                    return Err(self.error(
                        "YAML anchors and explicit tags are unsupported",
                        m,
                        path,
                    ));
                }
                let mut a = Vec::new();
                loop {
                    let (e, m) = self.next()?;
                    if e == Event::SequenceEnd {
                        break;
                    }
                    let mut p = path.clone();
                    p.push(PathSegment::Index(a.len()));
                    a.push(self.node(e, m, p, depth + 1)?);
                }
                Ok(Value::Array(a))
            }
            Event::MappingStart(anchor, tag) => {
                if anchor != 0 || tag.is_some() {
                    return Err(self.error(
                        "YAML anchors and explicit tags are unsupported",
                        m,
                        path,
                    ));
                }
                let mut a = Map::new();
                loop {
                    let (e, m) = self.next()?;
                    if e == Event::MappingEnd {
                        break;
                    }
                    let Event::Scalar(key, _, 0, None) = e else {
                        return Err(self.error("mapping keys must be untagged strings", m, path));
                    };
                    if key.len() > MAX_STRING_BYTES {
                        return Err(self.error("key byte limit exceeded", m, path));
                    }
                    let mut p = path.clone();
                    p.push(PathSegment::Key(key.clone()));
                    if a.contains_key(&key) {
                        return Err(self.error("duplicate mapping key", m, p));
                    }
                    self.nodes += 1;
                    if self.nodes > MAX_NODES {
                        return Err(self.error("node limit exceeded", m, p));
                    }
                    let (e, m) = self.next()?;
                    a.insert(key, self.node(e, m, p, depth + 1)?);
                }
                Ok(Value::Object(a))
            }
            _ => Err(self.error("expected one YAML value", m, path)),
        }
    }
}
fn number(s: &str) -> Option<Number> {
    if let Some(v) = s
        .strip_prefix("0x")
        .and_then(|v| u64::from_str_radix(v, 16).ok())
        .or_else(|| {
            s.strip_prefix("0o")
                .and_then(|v| u64::from_str_radix(v, 8).ok())
        })
    {
        return Some(v.into());
    }
    if let Ok(v) = s.parse::<i64>() {
        return Some(v.into());
    }
    if let Ok(v) = s.parse::<u64>() {
        return Some(v.into());
    }
    if (s.contains('.') || s.contains('e') || s.contains('E'))
        && s.chars().any(|c| c.is_ascii_digit())
    {
        return s.parse::<f64>().ok().and_then(Number::from_f64);
    }
    None
}
/// Parse strict JSON or supported YAML values according to the file extension.
/// Canonical writes preserve values, not comments/style. Positions use Unicode scalars.
pub fn parse_document(file: &Path, text: &str) -> Result<ParsedDocument, IncludeError> {
    if text.len() > MAX_BYTES {
        return Err(IncludeError::new("file byte limit exceeded"));
    }
    let json = match file.extension().and_then(|s| s.to_str()) {
        Some("json") => return json_document(file, text),
        Some("yaml" | "yml") => None,
        _ => {
            return Err(IncludeError::new(
                "source extension must be json, yaml or yml",
            ));
        }
    };
    let mut r = Reader {
        parser: Parser::new_from_str(text),
        file,
        nodes: 0,
        locations: BTreeMap::new(),
    };
    let (e, m) = r.next()?;
    if e != Event::StreamStart {
        return Err(r.error("expected stream", m, vec![]));
    }
    let (e, m) = r.next()?;
    if e != Event::DocumentStart {
        return Err(r.error("expected a document", m, vec![]));
    }
    let (e, m) = r.next()?;
    let value = r.node(e, m, vec![], 0)?;
    let (e, m) = r.next()?;
    if e != Event::DocumentEnd {
        return Err(r.error("expected document end", m, vec![]));
    }
    let (e, m) = r.next()?;
    if e != Event::StreamEnd {
        return Err(r.error("multiple YAML documents are unsupported", m, vec![]));
    }
    Ok(ParsedDocument {
        value: json.unwrap_or(value),
        locations: r.locations,
    })
}

fn json_document(file: &Path, text: &str) -> Result<ParsedDocument, IncludeError> {
    let value = serde_json::from_str::<Value>(text).map_err(|e| {
        IncludeError::located(
            e.to_string(),
            Location {
                file: file.into(),
                path: vec![],
                span: Some(SourceSpan {
                    line: e.line(),
                    column: e.column(),
                }),
            },
        )
    })?;
    let mut r = JsonPositions {
        text,
        file,
        pos: 0,
        nodes: 0,
        locations: BTreeMap::new(),
    };
    r.node(vec![], 0)?;
    Ok(ParsedDocument {
        value,
        locations: r.locations,
    })
}
struct JsonPositions<'a> {
    text: &'a str,
    file: &'a Path,
    pos: usize,
    nodes: usize,
    locations: BTreeMap<String, SourceSpan>,
}
impl JsonPositions<'_> {
    fn whitespace(&mut self) {
        while self.pos < self.text.len() && self.text.as_bytes()[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }
    fn span(&self) -> SourceSpan {
        let prefix = &self.text[..self.pos];
        SourceSpan {
            line: prefix.bytes().filter(|b| *b == b'\n').count() + 1,
            column: prefix.rsplit('\n').next().unwrap().chars().count() + 1,
        }
    }
    fn fail(&self, message: &str, path: Vec<PathSegment>) -> IncludeError {
        IncludeError::located(
            message,
            Location {
                file: self.file.into(),
                path,
                span: Some(self.span()),
            },
        )
    }
    fn string(&mut self) -> Result<String, IncludeError> {
        let start = self.pos;
        self.pos += 1;
        while self.pos < self.text.len() {
            match self.text.as_bytes()[self.pos] {
                b'\\' => self.pos += 2,
                b'"' => {
                    self.pos += 1;
                    let s = serde_json::from_str::<String>(&self.text[start..self.pos])
                        .map_err(|e| IncludeError::new(e.to_string()))?;
                    if s.len() > MAX_STRING_BYTES {
                        return Err(self.fail("string byte limit exceeded", vec![]));
                    }
                    return Ok(s);
                }
                _ => self.pos += 1,
            }
        }
        Err(self.fail("unterminated string", vec![]))
    }
    fn node(&mut self, path: Vec<PathSegment>, depth: usize) -> Result<(), IncludeError> {
        self.whitespace();
        self.nodes += 1;
        if depth > MAX_DEPTH || self.nodes > MAX_NODES {
            return Err(self.fail("value depth or node limit exceeded", path));
        }
        self.locations
            .insert(crate::source::path_key(&path), self.span());
        match self.text.as_bytes()[self.pos] {
            b'[' => {
                self.pos += 1;
                self.whitespace();
                let mut i = 0;
                while self.text.as_bytes()[self.pos] != b']' {
                    let mut p = path.clone();
                    p.push(PathSegment::Index(i));
                    self.node(p, depth + 1)?;
                    i += 1;
                    self.whitespace();
                    if self.text.as_bytes()[self.pos] == b',' {
                        self.pos += 1;
                        self.whitespace();
                    } else {
                        break;
                    }
                }
                self.pos += 1;
            }
            b'{' => {
                self.pos += 1;
                self.whitespace();
                let mut seen = std::collections::BTreeSet::new();
                while self.text.as_bytes()[self.pos] != b'}' {
                    let key = self.string()?;
                    let mut p = path.clone();
                    p.push(PathSegment::Key(key.clone()));
                    if !seen.insert(key) {
                        return Err(self.fail("duplicate mapping key", p));
                    }
                    self.nodes += 1;
                    self.whitespace();
                    self.pos += 1;
                    self.node(p, depth + 1)?;
                    self.whitespace();
                    if self.text.as_bytes()[self.pos] == b',' {
                        self.pos += 1;
                        self.whitespace();
                    } else {
                        break;
                    }
                }
                self.pos += 1;
            }
            b'"' => {
                self.string()?;
            }
            _ => {
                while self.pos < self.text.len()
                    && !matches!(
                        self.text.as_bytes()[self.pos],
                        b',' | b']' | b'}' | b' ' | b'\n' | b'\r' | b'\t'
                    )
                {
                    self.pos += 1;
                }
            }
        }
        Ok(())
    }
}
