# fn-122 friction

## 2026-09-23: the spec's shape needs a four-line edit in a directory the dispatch forbade

What I was doing: adding `moves`, `adopted`, `stood` and `rolled_back` to `tuning::handoff::Attempt`, the struct every gap attempt in `result.json` serialises.

What hindered it: `crates/telperion-jev/src/conductor/gapcheck.rs` builds an `Attempt` by struct literal in its test module, and the dispatch forbade touching `src/conductor`. Rust has no way to add a field to a struct without editing every literal of it, so the committed branch cannot compile the lib tests without that edit. The spec's architecture section named the keys but not where `Attempt` is constructed, so the conflict with the ownership split was not visible at dispatch.

Cost: about 5 minutes of looking for a shape that avoids the edit (a wrapper type, flatten) before confirming there is none, since `GapEntry` is built by literal there too.

What would have removed it: a dispatch that checked `grep -rn "Attempt {"` against the forbidden paths, or a test-helper constructor (`Attempt::tried(dial)`) so tests outside `tuning` do not spell every field. The patch is at `.flow/tmp/fn-122.1-gapcheck.patch`; the gate is green with it applied.
