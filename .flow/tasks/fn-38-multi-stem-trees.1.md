---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-38-multi-stem-trees.1 Implement Multi-stem trees

## Description
The birch in S-WHOLE stands on two stems that part at the ground, one
leaning out; the generator grew one. This task adds `skeleton.habit.stems`
(1 to 6, neutral 1, blended as a count), `stem_divergence` (0 to 120
degrees) and `stem_lean` (0 to 45 degrees): stems are order-zero axes born
at the root with the leader's length rule, the bole gates read them as
trunks, the radius solve shares the base through the pipe model, every run
leaving the root is a trunk run (buried, flared, no fork socket), and the
canopy and twig layer key off the largest stem's radius. The birch declares
two stems at 22 degrees of lean and its pairs are round 6.

## Acceptance
- [x] **R1** the three rows railed and refused by name, on the wire,
      blended, on the harness sliders, in the regenerated metadata, walked by
      the sweep; every preset at one stem byte-identical, the birch re-pinned
      once with the reason.
- [x] **R2** stems from the root, spread and leaned, inside the tree's
      vertical extent, refused only when degenerate (a shared heading), each
      a trunk run with a buried root and a flare; the run seeding moved to
      `surface/samples.rs`.
- [x] **R3** the base shared by the pipe model; DBH reports the largest stem
      with a count; twig eligibility keys off `Tree::stem_radius`.
- [x] **R4** the birch's table declares two stems; round 6 rendered at 22
      degrees (12 read as one trunk dividing), recorded beside round 5 in
      REPORT.md and `round6-fn38/stills.json`; 48/48 protocol, heaviest seed
      94,838 nodes.
- [x] **R5** `tests/stems.rs`, nine cases; stem roots are never shed. The
      year-zero birth stamp was implemented and reverted (see below).

## NEEDS_HUMAN — three owner items on round 6

The host read the worker's digest and the range bd39034..bb67d11. Visual
QA by the worker (four images): S-BARE plainly reads as two stems, one
flared foot dividing at once into two separately flared stems; S-WHOLE does
not read as two stems because the curtain covers the base (fn-44's reach,
fn-40's colour). Gates green: core, render, clippy, typecheck, wasm,
compare self-test, 48/48 protocol.

1. **The symmetric V.** Both stems lean by the same angle about one bearing,
   where the photograph has one near-vertical stem and one leaning far out
   before it rises. A per-stem lean row is beyond this spec's three rows; a
   new spec if the owner wants the photograph's pair.
2. **Stem birth year.** The spec asked for year zero with the root. A read
   of the tree at an age must equal a fresh build of that age, and a fresh
   build at year zero has grown nothing, so a stem stamped year zero appears
   in the read and not the build (five specimen tests). Reverted; "never
   shed" holds. Whether stems are created with the root before the first
   slice is a design call, recorded on `stem_root` in
   `branching/specimen.rs`.
3. **"Never through each other" is a degeneracy test.** Any positive
   clearance makes some point on a blend walk invalid, and every point
   between two valid families must be a family; two stems on one heading are
   refused, two that touch are a narrow fork.

## Done summary
TBD, after the owner's verdict on the round-6 pairs and the three items.

## Evidence
- Commits:
- Tests:
- PRs:
