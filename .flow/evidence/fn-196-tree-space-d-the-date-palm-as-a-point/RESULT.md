# fn-196 round 1: stopped at two design questions

A dispatched worker wrote this. Nothing here decides a design; D1 and D2 go to the host. No sheet was rendered: the palm cannot reach today's dressing until D1 is answered, and its trunk cannot be today's column until D2 is answered, so a sheet would show only the two known gaps.

## What was built

`crates/telperion-space/src/palm.rs`: Corner's model as values on the engine (SOURCES.md). One PA that never moves on, branching readiness 0, no laterals, no abortion or relay, erect, 12 phytomers a year at 0.031 m. The test `palm::tests::one_unbranched_stem` grows it at 50 years, seeds 1 and 7:

```
seed 1: 600 phytomers, 18.6 m, radius base 0.099 m, at 90% 0.056 m, apex 0.020 m
seed 7: 600 phytomers, 18.6 m, radius base 0.099 m, at 90% 0.056 m, apex 0.020 m
```

One axis, as C1 requires. The engine expresses the stem's topology.

## D1. The retained leaf bases cannot reach the pipeline from an engine tree

Checked in the code:

- The bases are hung in the pipeline's skeleton stage, after the radius solve: `pipeline::skeleton` (`crates/telperion-core/src/pipeline.rs:166-174`) calls `branching::clothe_leaf_bases(&mut tree, input.canopy)`.
- The space examples hand the converted tree to `executor::expand` (`examples/space/still.rs`), which goes straight to the expansion (`executor.rs:43-46`) and never runs the skeleton stage.
- `branching` is a private module of `pipeline` (`pipeline.rs:201`), and `clothe_leaf_bases` is not re-exported, so an example cannot call it. The design rule (AGENTS.md, `docs/pipeline.md`) keeps stages private so a second chain does not compile.

The fronds and the skirt are not affected: they are placed in the expansion by `foliage::rosette` from `Tree::stem_apices`, and the conversion sets `stem` on the seed axis and leaves one childless apex (MODEL-PALM S1, S8). This is read from the code, not rendered. No inflorescence is drawn today (capability `infructescence` Absent, fn-111), so none is owed.

Options, for the host:

1. **The expansion clothes the bases.** Move `clothe_leaf_bases` from `pipeline::skeleton` into the expansion, so every tree, grown by either engine, is clothed once on the way out. Today's palm's bytes may move if the order of appended nodes changes.
2. **A public entry for a grown tree.** `executor` gains a call that runs the skeleton stage's post-solve steps (today only `clothe_leaf_bases`) on a tree it is handed, then expands. Two entries into one pipeline.
3. **The engine grows the bases as organs.** A short, kept, unbranched lateral PA at every stem node. The pipeline's lattice packing (`leaf_bases.rs`, `lattice.rs`, the diamond cells laid on the stem's mean girth) would not apply to them, so this is likely a visible regression against today's lattice.
4. **Re-export `clothe_leaf_bases`** and call it in `tree::convert`. A stage called outside the pipeline, which the principles forbid.

## D2. The pipe model cannot give a palm's columnar stem

`girth.rs` sets every phytomer's radius by the pipe model: on a single axis of N phytomers, the radius k phytomers below the apex is `pipe * k^(1/exponent)`. With the exponent at its bound of 4, base over apex is N^(1/4): 4.95 for the 600 phytomers above, measured (0.099 against 0.020 m). Every value of `pipe` scales the whole stem; none changes its shape. `ripening` only thins the young top further. A palm's stem is a near-constant column (MODEL-PALM C8, C10; today's rows `lengthTaper 0.25`, `trunkRadius 0.013` of height; the catalogue's "near-constant trunk diameter" in `manifest.json`), and the leaf-base lattice is laid on the stem's mean girth (MODEL-PALM S5), so a stem five times thicker at the foot than under the crown would read as a spike, not a palm. MODEL-PALM's Q2 raised this before the build; C10 itself is unsourced.

Options, for the host:

1. **A "girth follows load" share on `Form`,** 1 the pipe model as today, 0 a girth set at establishment that the wood above does not add to; the palm at 0, every other species at 1. A new engine setting, changing the tree by degree.
2. **A wider exponent bound.** At 12, 600^(1/12) is 1.7; closer, never columnar, and the bound is there because a fork below 1.5 outgrows its bearer.
3. **The conversion keeps today's radius rows for the stem** (`pipeline::radius::solve` is private too, so this is D1's question again), and the engine's girth is not used for the palm.

## Smaller findings

- **No date-palm references folder.** `.flow/references/` has none; the local copies are in `.worktrees/fn-80-the-gap-loops-first-live-run/.refs/fn80/date-palm/` (whole.jpg, trunk.jpg, base.jpg). FRICTION.md.
- **Framing.** `still.rs` fits the camera to the node bounds. For a palm those are the stem's, so the frond crown above the apex would be cut off, as MODEL-PALM §7 found for today's bar. The sheet needs the camera fitted to the mesh bounds or to the stem plus `rachisLength`. A tooling change; not made.
- **Mature age.** 50 years gives 18.6 m at A1's 0.37 m a year, against today's 19.5 to 20 m.
