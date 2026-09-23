# fn-120 friction

## 2026-09-23 - a new row touches ten hand-kept lists

- Doing: adding three canopy rows and three material rows.
- Hindered: every row has to be added by hand to `fields!`, the blend `walk!` groups, the dial table, `SWITCHES_AT_ZERO`, the sweep's `HELD` list with its count, two dial-count pins in `tuning_engine.rs`, the capability `DERIVABLE` count test and each full `MaterialParams` literal (beech, birch) that does not use `..default()`. Nothing names the list that was missed until the gate runs.
- Cost: about 15 minutes of list-finding by reading fn-109 and fn-110's diffs, on a 6-row change.
- Would remove it: one checklist (or one test that prints every list a new wire path must join) next to `params.rs`'s `fields!`; count pins that read the table's length instead of a literal.

## 2026-09-23 - the shell cull ate two thirds of the skirt, seen only in a still

- Doing: R3's one small still of the palm.
- Hindered: the still showed no skirt. `foliage::cull` drops every leaf deeper than `shell_depth` of the crown radius inside the envelope, and a skirt hangs against the trunk, the deepest place there is: 592 of 1,760 dead leaflets survived at the family default 0.45 (a scratch probe over `mesh::build`). The placement tests pass on `foliage::place`, which is before the cull, so nothing short of a render showed it.
- Cost: one capture, one image view and about ten minutes.
- Would remove it: a placement test helper that reads the culled crown (`mesh::build`) alongside the placed one, or a line in the rosette's doc saying the shell cull still applies to what a rosette places.

## 2026-09-23 - a unit-test array length only compiles under the whole gate

- Doing: the one gate run at the end.
- Hindered: `material/tests.rs` pins its refusal table as `[Refusal; 81]`, so three new rows are a compile error in the core's lib tests, which no focused `--test` run builds. The first gate run was spent on it.
- Cost: one wasted full gate run (about ten minutes).
- Would remove it: a slice (`&[...]`) instead of a fixed-length array, or naming `cargo test -p telperion-core --lib` beside the gate for any row added to `MaterialParams`.

## 2026-09-23 - a neutral on its rail's end fails the conformance jitter

- Doing: the gate after the first fix.
- Hindered: `telperion-render/tests/conformance.rs` scales every number by 0.8 to 1.25, so a neutral at the top of its rail (skirt length 1.0 on 0..1, skirt pitch 160 on 0..180) is refused in 390 of 400 sets. Nothing in the row pattern says a neutral must sit at most four fifths of the way up its rail; fn-110's neutrals happen to.
- Cost: a second full gate run, about ten minutes, plus choosing new neutrals (0.8 and 140).
- Would remove it: a unit test beside `ranges.rs` that every default times 1.25 stays on its rail, run by the focused tests rather than only by the GPU conformance suite.
