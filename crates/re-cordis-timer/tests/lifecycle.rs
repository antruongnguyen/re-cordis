#![allow(missing_docs)]

use futures_util::{FutureExt, StreamExt};
use re_cordis_core::{BoxError, Context, Plugin, ServiceKey};
use re_cordis_timer::{IntervalStream, Scheduled, Sleep, TimerError, TimerExt};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};

include!("common/mod.rs");

struct Waiters {
    sleep: Sleep,
    stream: IntervalStream,
    schedules: [Scheduled; 2],
}

fn install_all(ctx: &Context, fired: Rc<Cell<usize>>) -> Result<Waiters, BoxError> {
    let timeout = fired.clone();
    ctx.timeout(Duration::from_secs(60), move || {
        timeout.set(timeout.get() + 1)
    })?;
    let interval = fired.clone();
    ctx.interval(Duration::from_secs(60), move || {
        interval.set(interval.get() + 1)
    })?;
    let sleep = ctx.sleep(Duration::from_secs(60));
    let stream = ctx.interval_stream(Duration::from_secs(60))?;
    let throttle_count = fired.clone();
    let throttle = ctx.throttle(Duration::from_secs(60), false, move || {
        throttle_count.set(throttle_count.get() + 1)
    })?;
    throttle.call()?;
    throttle.call()?;
    let debounce = ctx.debounce(Duration::from_secs(60), move || fired.set(fired.get() + 1))?;
    debounce.call()?;
    Ok(Waiters {
        sleep,
        stream,
        schedules: [throttle, debounce],
    })
}

fn assert_cancelled(mut waiters: Waiters) -> Result<(), BoxError> {
    assert!(matches!(
        waiters.sleep.now_or_never(),
        Some(Err(TimerError::Disposed))
    ));
    assert!(matches!(
        waiters.stream.next().now_or_never(),
        Some(Some(Err(TimerError::Disposed)))
    ));
    assert!(matches!(waiters.stream.next().now_or_never(), Some(None)));
    for schedule in waiters.schedules {
        schedule.call()?;
    }
    Ok(())
}

#[test]
fn all_six_operations_follow_update_restart_and_owner_disposal() -> Result<(), BoxError> {
    block(|root| async move {
        let holders = Rc::new(RefCell::new(Vec::new()));
        let sink = holders.clone();
        let fired = Rc::new(Cell::new(0));
        let count = fired.clone();
        let configs = Rc::new(RefCell::new(Vec::new()));
        let loaded = configs.clone();
        let plugin = Plugin::new("timed", move |ctx: Context, config: u32| {
            let sink = sink.clone();
            let count = count.clone();
            let loaded = loaded.clone();
            async move {
                loaded.borrow_mut().push(config);
                sink.borrow_mut().push(install_all(&ctx, count)?);
                Ok::<_, BoxError>(())
            }
        });
        let child = root.plugin(&plugin, 1)?;
        child.ready()?.await?;
        child.update(2)?.await?;
        assert_cancelled(holders.borrow_mut().remove(0))?;
        child.restart()?.await?;
        assert_cancelled(holders.borrow_mut().remove(0))?;
        child.dispose()?.await?;
        assert_cancelled(holders.borrow_mut().remove(0))?;
        assert_eq!(&*configs.borrow(), &[1, 2, 2]);
        assert_eq!(
            fired.get(),
            3,
            "only each generation's leading throttle fires"
        );
        tokio::time::advance(Duration::from_secs(120)).await;
        tokio::task::yield_now().await;
        assert_eq!(fired.get(), 3);
        assert!(root.registry().is_empty()?);
        assert!(root.get_effects()?.is_empty());
        Ok(())
    })
}

#[test]
fn one_hundred_owner_create_use_dispose_cycles_restore_baselines() -> Result<(), BoxError> {
    block(|root| async move {
        let holders = Rc::new(RefCell::new(Vec::new()));
        let sink = holders.clone();
        let fired = Rc::new(Cell::new(0));
        let count = fired.clone();
        let plugin = Plugin::new("timed-cycles", move |ctx: Context, ()| {
            let sink = sink.clone();
            let count = count.clone();
            async move {
                sink.borrow_mut().push(install_all(&ctx, count)?);
                Ok::<_, BoxError>(())
            }
        });
        for _ in 0..100 {
            let child = root.plugin(&plugin, ())?;
            child.ready()?.await?;
            child.dispose()?.await?;
            assert_cancelled(holders.borrow_mut().pop().unwrap())?;
            assert!(root.registry().is_empty()?);
            assert!(root.get_effects()?.is_empty());
        }
        assert_eq!(fired.get(), 100);
        Ok(())
    })
}

#[test]
fn service_bound_callback_owns_nested_timer_at_consumer_use_site() -> Result<(), BoxError> {
    block(|root| async move {
        let key = ServiceKey::<Cell<usize>>::new("timer-counter");
        let provider = root.provide(&key, Cell::new(0))?;
        let observed = Rc::new(RefCell::new(None::<Context>));
        let output = observed.clone();
        let use_key = key.clone();
        let child = root.plugin(
            &Plugin::new("timer-consumer", move |ctx: Context, ()| {
                let key = use_key.clone();
                let output = output.clone();
                async move {
                    *output.borrow_mut() = Some(ctx.clone());
                    let bound = ctx.bind(&key)?;
                    ctx.timeout(Duration::ZERO, move || {
                        bound.call(|value, context| {
                            value.set(value.get() + 1);
                            context.timeout(Duration::from_secs(60), || {}).unwrap();
                        });
                    })?;
                    Ok::<_, BoxError>(())
                }
            })
            .injecting([key.required()]),
            (),
        )?;
        child.ready()?.await?;
        tokio::task::yield_now().await;
        assert_eq!(root.get(&key)?.unwrap().get(), 1);
        assert_eq!(
            observed.borrow().as_ref().unwrap().get_effects()?.len(),
            1,
            "nested work belongs to consumer"
        );
        assert_eq!(
            root.get_effects()?.len(),
            2,
            "provider and child ownership only"
        );
        child.dispose()?.await?;
        provider.into_effect().dispose()?.await?;
        assert!(root.get_effects()?.is_empty());
        Ok(())
    })
}
