//! The whole-tree GPU probes run one at a time in a process (fn-212): each
//! holds a settled tree's buffers, which beside another's outgrow the
//! device, so the harness's threads wait here rather than fail. The lock
//! does not re-enter: a second take on the thread that holds it fails at
//! once, naming the guard, instead of waiting on itself with the GPU lock
//! held (fn-261 FRICTION.md, fn-237 R12).
use std::cell::Cell;
use std::sync::{Mutex, MutexGuard};

static LOCK: Mutex<()> = Mutex::new(());

thread_local! {
    static HELD: Cell<bool> = const { Cell::new(false) };
}

/// The one whole tree this process's GPU probes hold.
pub(crate) struct WholeTree {
    _lock: MutexGuard<'static, ()>,
}

impl Drop for WholeTree {
    fn drop(&mut self) {
        HELD.with(|h| h.set(false));
    }
}

pub(crate) fn whole_tree() -> WholeTree {
    assert!(
        !HELD.with(|h| h.replace(true)),
        "whole_tree() is already held on this thread; a second take would wait on itself"
    );
    WholeTree {
        _lock: LOCK.lock().unwrap_or_else(|e| e.into_inner()),
    }
}

#[test]
fn a_second_take_on_one_thread_fails_at_once() {
    let held = whole_tree();
    let second = std::panic::catch_unwind(|| drop(whole_tree())).unwrap_err();
    let message = (second.downcast_ref::<&str>().map(|m| m.to_string()))
        .or_else(|| second.downcast_ref::<String>().cloned())
        .unwrap_or_default();
    assert!(
        message.contains("whole_tree() is already held"),
        "{message:?}"
    );
    drop(held);
    drop(whole_tree());
}
