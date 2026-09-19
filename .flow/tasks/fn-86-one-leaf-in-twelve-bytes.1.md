---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8]
---
# fn-86-one-leaf-in-twelve-bytes.1 Implement One leaf in twelve bytes

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
# fn-86 One leaf in twelve bytes - every criterion met but one count

A leaf is three words. The encoding landed in `5b49227c` and the prior run
carried R1, R3, R5 and R6's shader half; this run closed R6's growth half, R4,
R7 and R8, confirmed R2 against the owner's amendment, and found and fixed a
red gate the prior run had not reached.

### What this run added

**R6's growth half.** `a_leaf_cached_at_one_age_decodes_at_a_later_one` grows
the birch to sixteen and then to eighteen. Of the 744 leaves written at the
earlier age, 407 come back byte-identical, so they were served from the cache
rather than re-derived; each decodes to a point standing off its own wood at
exactly the distance it had when it was written, and inside the box. The same
test carries the counterfactual: the crown's own bounds are not the same at the
two ages, and the cached words read against them drift up to 2.85 m. Decoding
against an age-dependent box was run and makes the standoff assertion fail at
0.16 m, so the test is red for the reason it exists.

**R4.** Ninety-six `species:qa` measurement cases - four species, twenty-four
seeds each, both profiles files - pass at `3345b07f` and pass now. No gate
moved and no frozen profile was touched. Retained leaf counts, read from each
case's `output_bytes.retained_matrices` at sixty-four bytes a leaf before and
twelve now:

| Species | Seeds moved | Worst single seed | Total move |
|---|---|---|---|
| `oregon-white-oak` | 0 of 24 | +0.00000% | +0.00000% |
| `norway-spruce` | 0 of 24 | +0.00000% | +0.00000% |
| `european-beech` | 24 of 24 | +0.00170% (85 leaves in 5,004,170) | -0.00024% |
| `silver-birch` | 20 of 24 | -0.00177% (4 leaves in 225,635) | -0.00006% |

R4 allows 0.1 percent; the worst seed is fifty-eight times inside it. The shape
is the shell test rather than drift: a beech leaf is small against a 32 m box
and its vertices lie densest at the shell, while a spruce needle sits deep
enough inside that a quarter millimetre never reaches the decision.

**R7.** Peak RSS, `/proc/<pid>/status` VmHWM polled to exit, one test per
process on the `ci` profile:

```
fixed_spruces   3,244 MB -> 1,763 MB   -46%   (R7 asked for under 1,800)
fixed_beeches   3,945 MB -> 2,589 MB   -34%
fixed_oaks      1,469 MB -> 1,271 MB   -13%
fixed_birches     778 MB ->   704 MB   -10%
```

Fifty-two bytes a leaf saved over the four specimens a spruce test holds at
once predicts about 1,530 MB; 1,481 MB came off.

**R8.** One still per species, 960x720 on an RTX 3080, captured only after the
numeric gates passed, in
`.flow/evidence/fn-86-one-leaf-in-twelve-bytes/stills/` with a manifest. Every
still's foliage instance count is the retained count `species_measure` reported
for the same species and seed, so the renderer drew the leaves the generator
counted and drew them out of three words; each tree casts a foliage shadow,
which is `shadow.wgsl` reading the same words. The renderer change landed in
`5b49227c` and no commit since touches `crates/telperion-render` or
`crates/telperion-core/src`, so these are the landing commit's renderer and
generator.

**R2.** Nothing implemented. The test holds the measured 0.0026 rad and the
spec now says why; its comment still read as a question put to the host, and
now reads as the ruling it is.

**A red gate the prior run had not reached.** `npm test` was failing:
`catalogue:check` found four species' `README.md` differing from regeneration.
The pages are generated from `pins.json`, the encoding commit moved every
species' placement hash and bounds, and nothing regenerated them. Mirror drift,
the class CLAUDE.md already names. Regenerated in `50f7e9d5`; the base commit
was green on this check, so the failure was this task's.

### Gates

`cargo test --profile ci --workspace` green at `e61c9ae8`, 94 binaries,
`suite_rc=0`; clippy clean; `cargo fmt --all -- --check` clean; `npm test`
green - catalogue check across five species and 98 vitest cases; `npx tsc
--noEmit` clean. The baseline before this run's first edit was green.

### The one thing that needs the host

**R4 and R8 name five catalogue species; four of them can be measured and
drawn.** The fifth, the European ash, has no preset. Its `pins.json` is
`{"empty": true}`, its README records `Preset | none yet` and readiness
`unready`, it appears in neither profiles file, `species_measure`'s own help
says it "is not a catalogue species", and `crates/telperion-core/tests/species.rs:166`
asserts `Preset::from_id("european-ash").is_none()`. There is no generator
output for it, so there is no numeric gate to run and nothing to draw. No
implementation can satisfy a criterion that requires building it.

The choice is the host's: read R4 and R8 as met by the four species the
generator can express, or amend their count. The evidence files name the gap in
place rather than passing over it. Everything else in the spec is met.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 5b49227c21c06f9ef656f2cf6d16f2a6540557a0, 4dbe6124d0dbebb2b9b1db30c6d7a379ce7c2e40, a5c1624345e92702c5f752b60e6117c32ea04e54, ca78691f29071c15a3335358fe52f467cefb4ee7, 138427a8dd2e18040db15345bbecd0ddce97ab59, 50f7e9d5034c614b76ab8029c4172d6be214ce5d, 1817b29d48fa787f8030f75aa544b57c24feaedb, e61c9ae845c0201fccdc7fd08945f575b1ff1151, 96c91c6dd11a55dd5170086329d92f77b95f7af2
- Tests: cargo test --profile ci --workspace (94 binaries, suite_rc=0), cargo clippy --profile ci --workspace --all-targets (clean), cargo fmt --all -- --check (clean), npm test (catalogue:check 5 species pass; vitest 7 files, 98 cases), npx tsc --noEmit (clean), node tests/species.mjs --measure-only --profiles .flow/evidence/fn9/profiles.json (48/48 pass, rc=0), node tests/species.mjs --measure-only --profiles .flow/evidence/fn34/profiles.json --seeds .flow/evidence/fn34/seeds.json (48/48 pass, rc=0), same two measure commands at base 3345b07f (96/96 pass, rc=0) for the retained-count comparison, VmHWM poll, one fixed_* test per process on the ci profile: spruces 1763 MB, beeches 2589 MB, oaks 1271 MB, birches 704 MB, target/release/examples/headless --preset <id> --seed 1 --view whole (4 stills, RTX 3080)
- PRs: