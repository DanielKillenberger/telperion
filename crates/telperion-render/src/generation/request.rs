use super::Generator;
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub(crate) struct State {
    revision: u64,
    pending: bool,
    disposed: bool,
    pub generator: Option<Rc<Generator>>,
}
impl State {
    pub fn invalidate(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }
    pub fn dispose(&mut self) {
        self.invalidate();
        self.disposed = true;
        self.generator = None;
    }
    /// Whether the page let its renderer go: a tree request invalidates
    /// the builds before it, never the programs a page prepared (fn-243).
    pub fn disposed(&self) -> bool {
        self.disposed
    }
}
pub(crate) struct Request {
    state: Rc<RefCell<State>>,
    revision: u64,
}
impl Request {
    pub fn begin(state: Rc<RefCell<State>>) -> Result<Self, &'static str> {
        let revision = {
            let mut current = state.borrow_mut();
            if current.pending {
                return Err("GPU generation is busy");
            }
            current.pending = true;
            current.revision
        };
        Ok(Self { state, revision })
    }
    pub fn check(&self) -> Result<(), &'static str> {
        if self.state.borrow().revision != self.revision {
            return Err("GPU generation is stale or disposed");
        }
        Ok(())
    }
}
impl Drop for Request {
    fn drop(&mut self) {
        self.state.borrow_mut().pending = false;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalidation_keeps_busy_until_guard_drops_and_errors_release_it() {
        let state = Rc::new(RefCell::new(State::default()));
        let first = Request::begin(state.clone()).unwrap();
        assert!(Request::begin(state.clone()).is_err());
        state.borrow_mut().invalidate();
        assert!(first.check().is_err());
        assert!(Request::begin(state.clone()).is_err());
        drop(first);
        let second = Request::begin(state.clone()).unwrap();
        assert!(second.check().is_ok());
        state.borrow_mut().dispose();
        assert!(second.check().is_err());
        assert!(Request::begin(state.clone()).is_err());
        drop(second);
        assert!(Request::begin(state).is_ok());
    }
    #[test]
    fn a_tree_request_never_marks_the_prepared_programs_disposed() {
        let mut state = State::default();
        state.invalidate();
        assert!(!state.disposed());
        state.dispose();
        assert!(state.disposed());
    }
}
