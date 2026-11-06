//! Public-API R0 shutdown regression: canceled group convergence must stop reporting Unloading.
use futures_util::FutureExt;
use re_cordis_core::{Plugin, Runtime, ServiceKey};
use re_cordis_loader::{Catalog, Config, EntrySpec, EntryState, Group, Loader, Scope};
use std::{
    cell::{Cell, RefCell},
    convert::Infallible,
    rc::Rc,
};
use tokio::sync::oneshot;

#[tokio::test(flavor = "current_thread")]
async fn joined_ancestor_shutdown_has_terminal_group_snapshot() {
    let runtime = Runtime::new();
    runtime
        .run(async {
            let root = runtime.context();
            let parent = root
                .plugin(
                    &Plugin::new("parent", |_, ()| async { Ok::<_, Infallible>(()) }),
                    (),
                )
                .unwrap();
            let (entered_tx, entered) = oneshot::channel();
            let (release, gate) = oneshot::channel();
            let pause = Rc::new(RefCell::new(Some((entered_tx, gate))));
            let released = Rc::new(Cell::new(false));
            let captures = Rc::new(RefCell::new(Vec::new()));
            let observe = captures.clone();
            let released_worker = released.clone();
            let mut catalog = Catalog::new();
            catalog
                .register(
                    "worker",
                    Plugin::new("worker", move |ctx, _: u32| {
                        let pause = pause.clone();
                        let released = released_worker.clone();
                        let capture = Rc::new(());
                        observe.borrow_mut().push(Rc::downgrade(&capture));
                        async move {
                            ctx.on_dispose(move || async move {
                                let gate = pause.borrow_mut().take();
                                if let Some((entered, gate)) = gate {
                                    entered.send(()).unwrap();
                                    gate.await.unwrap();
                                }
                                drop(capture);
                                released.set(true);
                                Ok::<_, Infallible>(())
                            })
                            .unwrap();
                            Ok::<_, Infallible>(())
                        }
                    }),
                )
                .unwrap();
            let loader = Loader::new(&parent.context(), catalog).unwrap();
            let group = loader
                .create(
                    EntrySpec::group(Group::new(vec![
                        EntrySpec::new("worker", Config::new(1u32)).id("worker"),
                    ]))
                    .id("group"),
                )
                .unwrap();
            loader.settle().unwrap().await.unwrap();
            let key = ServiceKey::<u32>::new("unused");
            drop(
                loader
                    .update(
                        &group,
                        EntrySpec::group(Group::new(vec![
                            EntrySpec::new("worker", Config::new(2u32)).id("worker"),
                        ]))
                        .scope(Scope::new().isolate(&key)),
                    )
                    .unwrap(),
            );
            entered.await.unwrap();
            let mut task = parent.dispose().unwrap();
            for _ in 0..8 {
                tokio::task::yield_now().await;
            }
            assert!((&mut task).now_or_never().is_none());
            assert!(!released.get());
            assert!(
                captures
                    .borrow()
                    .iter()
                    .any(|capture| capture.upgrade().is_some())
            );
            release.send(()).unwrap();
            task.await.unwrap();
            assert!(released.get());
            assert!(
                captures
                    .borrow()
                    .iter()
                    .all(|capture| capture.upgrade().is_none())
            );
            assert!(runtime.take_failures().is_empty());
            assert!(root.registry().is_empty().unwrap());
            assert!(root.get_effects().unwrap().is_empty());
            let snapshot = loader.snapshot(&group).unwrap();
            assert!(snapshot.fiber_id.is_none() && snapshot.outer_id.is_none());
            assert_eq!(
                snapshot.state,
                EntryState::Disposed,
                "joined owner must have terminal snapshot"
            );
        })
        .await
        .unwrap();
}
