//! Explicit compiled-unit dependency selection over loader-owned replacement.
use re_cordis_loader::{FactoryReplacement, Loader, LoaderError, LoaderTask, Settlement};
use std::collections::BTreeSet;

/// An application-declared code unit; dependency-only units have no factory.
#[derive(Clone, Debug)]
pub struct ReloadNode {
    /// Stable explicit unit name, unrelated to inferred Rust module paths.
    pub name: String,
    /// Catalog factory rebuilt when this unit is affected, if any.
    pub factory: Option<String>,
    /// Other explicitly declared units used by this unit.
    pub dependencies: Vec<String>,
}
/// An immutable bounded graph selecting linked catalog replacements.
/// No watcher, module discovery, code compilation or new background task is owned.
pub struct ReloadPlan {
    nodes: Vec<ReloadNode>,
}
impl ReloadPlan {
    /// Check at most 256 nodes and 1024 edges. Cycles are supported.
    /// Names contain 1..=256 UTF-8 bytes; node and factory mappings are unique.
    pub fn new(nodes: Vec<ReloadNode>) -> Result<Self, LoaderError> {
        if nodes.len() > 256
            || nodes
                .iter()
                .map(|node| node.dependencies.len())
                .sum::<usize>()
                > 1024
        {
            return Err(LoaderError::Invalid(
                "reload graph exceeds 256 nodes or 1024 edges".into(),
            ));
        }
        let mut names = BTreeSet::new();
        let mut factories = BTreeSet::new();
        for node in &nodes {
            if node.name.is_empty() || node.name.len() > 256 || !names.insert(node.name.as_str()) {
                return Err(LoaderError::Invalid(
                    "reload node names must be unique and contain 1..=256 UTF-8 bytes".into(),
                ));
            }
            if let Some(factory) = &node.factory {
                if factory.is_empty() || factory.len() > 256 || !factories.insert(factory.as_str())
                {
                    return Err(LoaderError::Invalid(
                        "reload factory mappings must be unique and contain 1..=256 UTF-8 bytes"
                            .into(),
                    ));
                }
            }
        }
        if nodes
            .iter()
            .flat_map(|node| &node.dependencies)
            .any(|name| !names.contains(name.as_str()))
        {
            return Err(LoaderError::Invalid(
                "reload dependency must name an explicit node".into(),
            ));
        }
        Ok(Self { nodes })
    }
    /// Select factories in declaration order through the reverse dependency closure.
    /// Unchanged shared dependencies do not affect their other consumers.
    pub fn affected(&self, changed: &[String]) -> Result<Vec<String>, LoaderError> {
        if changed.len() > 256 {
            return Err(LoaderError::Invalid(
                "changed unit limit 256 exceeded".into(),
            ));
        }
        let mut affected = BTreeSet::new();
        for name in changed {
            if !self.nodes.iter().any(|node| node.name == *name) {
                return Err(LoaderError::Invalid(format!("unknown reload unit: {name}")));
            }
            affected.insert(name.as_str());
        }
        loop {
            let before = affected.len();
            for node in &self.nodes {
                if node
                    .dependencies
                    .iter()
                    .any(|name| affected.contains(name.as_str()))
                {
                    affected.insert(node.name.as_str());
                }
            }
            if affected.len() == before {
                break;
            }
        }
        Ok(self
            .nodes
            .iter()
            .filter(|node| affected.contains(node.name.as_str()))
            .filter_map(|node| node.factory.clone())
            .collect())
    }
    /// Require one compiled candidate for every affected factory, then admit one batch.
    /// Loader owns independent cleanup and latest-intent convergence. Settlement
    /// reports post-unload failures; it does not promise rollback of external effects.
    pub fn replace(
        &self,
        loader: &Loader,
        changed: &[String],
        replacements: Vec<FactoryReplacement>,
    ) -> Result<LoaderTask<Settlement>, LoaderError> {
        loader.replace_factories(self.candidates(changed, replacements)?)
    }
    /// Replace the same Include-managed instances through the optional Include seam.
    /// Source bytes, journal and provenance remain unchanged; no flush is requested.
    #[cfg(feature = "include")]
    pub fn replace_include(
        &self,
        include: &re_cordis_include::Include,
        changed: &[String],
        replacements: Vec<FactoryReplacement>,
    ) -> Result<
        re_cordis_include::IncludeTask<re_cordis_include::Report>,
        re_cordis_include::IncludeError,
    > {
        include.replace_factories(self.candidates(changed, replacements)?)
    }
    fn candidates(
        &self,
        changed: &[String],
        replacements: Vec<FactoryReplacement>,
    ) -> Result<Vec<FactoryReplacement>, LoaderError> {
        let affected = self.affected(changed)?;
        let mut supplied = BTreeSet::new();
        for replacement in &replacements {
            if !affected.iter().any(|name| name == replacement.name())
                || !supplied.insert(replacement.name())
            {
                return Err(LoaderError::Invalid(
                    "replacement candidates must exactly cover affected factories".into(),
                ));
            }
        }
        if supplied.len() != affected.len() {
            return Err(LoaderError::Invalid(
                "replacement candidates must exactly cover affected factories".into(),
            ));
        }
        Ok(replacements)
    }
}
