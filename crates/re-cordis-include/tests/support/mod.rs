#![allow(dead_code)]
use re_cordis_core::{Plugin, Runtime};
use re_cordis_loader::Catalog;
use std::{
    convert::Infallible,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
pub struct Dir(PathBuf);
impl Dir {
    pub fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "cordis-include-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
    pub fn path(&self, n: &str) -> PathBuf {
        self.0.join(n)
    }
    pub fn put(&self, n: &str, t: &str) {
        std::fs::write(self.path(n), t).unwrap();
    }
}
impl Drop for Dir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
pub fn catalog() -> Catalog {
    let mut c = Catalog::new();
    c.register_json(
        "worker",
        Plugin::new("worker", |_, _: Option<serde_json::Value>| async {
            Ok::<_, Infallible>(())
        }),
        |v| Ok::<_, Infallible>(v.cloned()),
    )
    .unwrap();
    c
}
pub fn read(p: &Path) -> serde_json::Value {
    serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap()
}
pub async fn run<F: std::future::Future<Output = ()>>(runtime: &Runtime, f: F) {
    runtime.run(f).await.unwrap();
}
