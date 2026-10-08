//! A phase's work on an executor as the driver takes it (fn-212 task 10,
//! host decision 1): a future, ready at once on the CPU reference, a
//! device's readback on the GPU's. Natively the driver blocks on it where
//! its own cores work beside the executor; in a browser it awaits it,
//! since WebGPU completes a readback only on the page's event loop.
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// What an executor's phase returns: its result, once the work is done.
pub type Step<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

/// `value` as a step that is ready at once.
pub fn ready<'a, T: 'a>(value: T) -> Step<'a, T> {
    Box::pin(std::future::ready(value))
}

/// The result of `future`, whose every step is ready at once: the CPU
/// reference executor's settle, driven on any target.
pub(crate) fn now<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    match future
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(out) => out,
        Poll::Pending => unreachable!("the CPU reference executor's steps are ready at once"),
    }
}

/// The result of `future`, this thread parked until it is ready: the
/// driver's wait where its cores work beside the executor (native only).
#[cfg(not(target_family = "wasm"))]
pub(crate) fn wait<T>(future: impl Future<Output = T>) -> T {
    struct Unpark(std::thread::Thread);
    impl std::task::Wake for Unpark {
        fn wake(self: std::sync::Arc<Self>) {
            self.0.unpark();
        }
    }
    let waker = Waker::from(std::sync::Arc::new(Unpark(std::thread::current())));
    let mut context = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(out) => return out,
            Poll::Pending => std::thread::park(),
        }
    }
}
