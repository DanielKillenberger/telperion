//! The browser's compiles as promises the test holds (fn-243 R4): one
//! resolves late, one rejects, the device goes while they are pending.
use super::settle;
use crate::RenderError;
use std::cell::{Cell, RefCell};
use std::future::Future;
use std::pin::{pin, Pin};
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

type Outcome = std::result::Result<(), String>;

/// A compile the test resolves or rejects when it chooses, counting the
/// polls it was given.
#[derive(Clone, Default)]
struct Promise {
    outcome: Rc<RefCell<Option<Outcome>>>,
    polled: Rc<Cell<usize>>,
}
impl Promise {
    fn settle(&self, outcome: Outcome) {
        *self.outcome.borrow_mut() = Some(outcome);
    }
}
impl Future for Promise {
    type Output = Outcome;
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Outcome> {
        self.polled.set(self.polled.get() + 1);
        self.outcome
            .borrow_mut()
            .take()
            .map_or(Poll::Pending, Poll::Ready)
    }
}

fn poll<F: Future>(future: Pin<&mut F>) -> Poll<F::Output> {
    future.poll(&mut Context::from_waker(Waker::noop()))
}

fn named(promises: &[Promise]) -> Vec<(String, Promise)> {
    ["grow", "asks", "pack"]
        .into_iter()
        .zip(promises)
        .map(|(name, promise)| (name.to_string(), promise.clone()))
        .collect()
}

#[test]
fn preparation_waits_on_a_late_compile_and_then_is_ready() {
    let promises = [Promise::default(), Promise::default(), Promise::default()];
    let mut settled = pin!(settle(named(&promises), || None));
    promises[0].settle(Ok(()));
    assert!(
        poll(settled.as_mut()).is_pending(),
        "ready before `asks` resolved"
    );
    assert!(poll(settled.as_mut()).is_pending());
    promises[1].settle(Ok(()));
    promises[2].settle(Ok(()));
    assert!(matches!(poll(settled.as_mut()), Poll::Ready(Ok(()))));
}

#[test]
fn a_rejected_program_is_named_and_nothing_after_it_is_awaited() {
    let promises = [Promise::default(), Promise::default(), Promise::default()];
    let mut settled = pin!(settle(named(&promises), || None));
    promises[0].settle(Ok(()));
    promises[1].settle(Err("validation: entry point `asks` not found".into()));
    let Poll::Ready(Err(error)) = poll(settled.as_mut()) else {
        panic!("a rejection did not end preparation");
    };
    assert_eq!(
        error.to_string(),
        "the browser refused the GPU program `asks`: validation: entry point `asks` not found"
    );
    assert_eq!(promises[2].polled.get(), 0);
}

#[test]
fn a_device_lost_during_preparation_is_the_error_named() {
    let promises = [Promise::default(), Promise::default(), Promise::default()];
    let lost = Rc::new(Cell::new(false));
    let seen = lost.clone();
    let mut settled = pin!(settle(named(&promises), move || seen.get().then(|| {
        RenderError::DeviceLost {
            reason: "Unknown: A valid external Instance reference no longer exists.".into(),
        }
    })));
    promises[0].settle(Ok(()));
    assert!(poll(settled.as_mut()).is_pending());
    // The browser answers a lost device's compile as it likes; the loss is
    // what is reported.
    lost.set(true);
    promises[1].settle(Err("internal: the device was destroyed".into()));
    let Poll::Ready(Err(error)) = poll(settled.as_mut()) else {
        panic!("a lost device did not end preparation");
    };
    assert!(
        error.to_string().starts_with("the GPU device was lost: "),
        "{error}"
    );
}
