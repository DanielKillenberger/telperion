# fn-123 friction

## 2026-09-23 - worker, before any edit

- Doing: locating fn-113's `crates/telperion-jev/tests/dial_bounds.rs`, which R2 edits.
- Hindered by: the fn-123 branch is cut from fn-119, and fn-113 was merged only into the fn-80 branch, so the file does not exist here. The dispatch forbids merging another branch in, so R2 cannot be done on this branch.
- Cost: about 5 minutes and one requirement left open.
- Would remove it: cut a spec's branch from the branch that holds the file its acceptance names, or record the dependency on fn-113 in the spec so the conductor sequences it.
