## Conversation Evidence

> owner (2026-09-27), on the host's three fixes from the beech run (fork defaults, splitting a bundle that fails to build, curated references read as stored): "go"

## Goal & Context
<!-- scope: business -->

The beech's first runner Tune after fn-61, fn-170 and fn-177 (fn-62, 2026-09-27) kept nothing: every one of 16 bundles raised `codominance` from 0 while the beech's `forkDivergence` and `forkLean` stayed at their default 0, and the generator refused each build ("fork parts pass through each other"). The loop redrew the same refused bundle for five rounds, 1,453,019 Jev tokens, and its Gaps result (four identity traits) says nothing about the beech. The run also needed two hand patches to use the beech's curated references. `codominance` must build on its own, a bundle that fails to build must be taken apart, and a run must read references as the catalogue stores them. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-27 on master (`563b1534`).** [checked]
- `pipeline/branching/scaffold/fork.rs:177` (`placed`) refuses a family with `codominance > 0` whose part headings coincide; with `forkDivergence` and `forkLean` at their defaults of 0 (`pipeline/branching/traits.rs:324-325`), every part heading coincides, so `codominance` alone is refused (reproduced by `species_measure --family` on the beech at 0.05 and the oak at 0.3; `codominance` 0.3 with `forkLean` 20 still refused; with `forkDivergence` 30 as well it builds).
- `tuning/bundle.rs:233` (`split`) halves a bundle only when a better variant breaks a check; a bundle whose every variant fails to build is redrawn, so the loop ended "bundle already tried; no new direction".
- The runner takes a reference's view only from a `view` field and its image only from `<run-dir>/cache/photos/<sha256>.jpg`; the shipped catalogue's curated records (`catalogue/european-beech/packet/references.json`) carry the view in `shot.scale` (and `scale`) and their images under `.refs/`.

**Shape.** [inferred]
1. **Fork rows default to a fork that builds.** `forkDivergence` and `forkLean` take nonzero defaults from the shipped birch and fn-170's plane candidate, stated and justified; they stay dormant while `codominance` is 0, so every preset that does not set `codominance` is byte-identical. The birch states its own values and is unchanged. A family whose parts still coincide is refused as today, naming the rows.
2. **A bundle that fails to build is split.** When every variant of a bundle fails to build (not a check), the loop splits it with `split` and tries its halves in the next round, recording the refusal and its message; a single move that fails alone is dropped for the revision and logged. No bundle is drawn twice.
3. **Curated references read as stored.** A reference record with a shot takes its view from the shot's scale (`whole` in leaf, `bare`, `bark`/base) when it has no `view`, and its image from the path the record's `asset_sha256` resolves to in `.refs/`, copied into the run's cache by hash, verified, never committed.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `codominance` 0.05 alone builds on every shipped preset at seeds 1 and 7; at `codominance` 0 every preset is byte-identical to master; the birch is unchanged. [inferred]
- **R2:** On a recorded or synthetic loop run where a bundle fails to build, the next round tries its halves, a lone failing move is dropped and logged, and no bundle is drawn twice; the beech's refused bundle from fn-62's run is the regression case. [inferred]
- **R3:** A run seeded with the beech's curated references needs no hand patch: views and images are resolved from the records as stored, and the photograph search does not run. [inferred]
- **R4:** The workspace gate and `npm test` are green. [inferred]

## Boundaries
<!-- scope: business -->

- Not the tuning cost work of fn-168 beyond the split; not branching hierarchy; not the beech's values. fn-62 reruns Tune on top of this spec.

## Strategy Alignment

- Serves "Our approach": every parameter changes the tree by degree, and `codominance` no longer depends on two other rows being set first. [strategy:Our approach]
