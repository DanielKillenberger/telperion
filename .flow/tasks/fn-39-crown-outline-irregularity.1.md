---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-39-crown-outline-irregularity.1 Implement Crown outline irregularity

## Description
Every seed filled the envelope's smooth superellipse to the same outline, so
a tree's silhouette was an oval however the branches inside it varied, and
the round-3 pairs of the beech and the birch read as ovals against
photographs whose crowns are lumpy: lobes where scaffold limbs end, gaps
where they do not. This task gives the envelope a bearing and a seed: two
rows, `skeleton.envelope.irregularity` (amplitude as a fraction of the
radius, 0 to 0.5, neutral 0) and `skeleton.envelope.lobe_scale` (wavelength
as a fraction of the height, 0.05 to 1), a seeded low-frequency noise of
height and bearing that the containment tests, the scaffold and the twig
planner reject against, while the two-dimensional profile keeps driving
shedding and the crown index. The compare script gains the outline's radial
deviation from its own fitted ellipse, recorded on the photograph and the
still. The beech and the birch set an amplitude against their whole
references, the matched pairs are rendered again as round 5, and the owner
judges them in fn-34.
## Acceptance
- [x] **R1** `skeleton.envelope.irregularity` (0 to 0.5, neutral 0) and
      `skeleton.envelope.lobe_scale` (0.05 to 1) are rows: refused by name off
      their rails, on the wire, blended linearly, in the regenerated browser
      metadata and on the harness's own sliders. Every shipped preset that
      leaves the amplitude at zero is byte-identical: the oak, the spruce and
      the Two Trees' pins hold.
- [x] **R2** With a positive amplitude the shell's radius varies with height
      and bearing by a seeded low-frequency noise at the stated wavelength,
      bounded by one plus the amplitude; growth, the scaffold and the twig
      planner reject against the lumpy shell and every containment test holds
      against it on every fixed seed. The smooth profile still drives shedding
      and the crown index, and the envelope says so.
- [x] **R3** `scripts/compare-references.py` records `outline_deviation`, the
      mask's boundary radius about its best-fit ellipse over 180 bins of
      bearing, on the still and the photograph, null when the boundary is not
      closed; the self-test tells a five-lobed disc from an oval.
- [x] **R4** The beech (0.18 at 0.7) and the birch (0.15 at 0.45) state an
      amplitude against their whole references; the matched pairs are rendered
      again as round 5 with the numbers beside round 3's and round 4b's in
      `.flow/evidence/fn34/REPORT.md`, the pairs recorded by sha256 in
      `round5-fn39/stills.json`. The beech's twig length ratio went from 0.42
      to 0.40 so the lobes' per-seed noise stays under the node ceiling (seed
      89 reached it at any amplitude); the 48-case protocol passes. The owner's
      verdict is the open item, recorded in fn-34.
- [x] **R5** `tests/outline.rs`: neutral byte identity, the rails refused by
      name, the bound on the perturbed radius, containment against the
      perturbed shell on every fixed seed, one seed one outline and two seeds
      two, a blend walk of amplitude and wavelength; the compare self-test
      covers the outline statistic on a synthetic pair. The beech and the
      birch identity pins are re-recorded once with the reason stated.
## NEEDS_HUMAN — the owner's verdict on the round-5 pairs

R4 reserves the verdict to the owner and this task cannot award it. The
round-5 pairs are on disk under `.flow/evidence/fn34/measure/pairs-fn39/`,
recorded by sha256 in `.flow/evidence/fn34/round5-fn39/stills.json` with
`visual_status: unassessed`, and laid out with every earlier round, the
numbers and the references on the local judging page
`.flow/evidence/fn34/measure/judge.html`.

Both silhouettes left the oval: the outline statistic reads 0.10 on the
beech's whole still and 0.11 on the birch's against 0.00-class ovals before,
under the photographs' 0.23 and 0.22 (which the box's background inflates).
The proportion numbers did not move, as a perturbation about the shell's own
mean should not. What the host read on the whole pairs is in the round-5
section of `.flow/evidence/fn34/REPORT.md`: the beech's lobes read, the mass
and the colour do not yet, which the gap table assigns to appearance; the
birch is ragged rather than round and still one stem under a frosted
curtain, which are fn-38's and fn-40's.

Gates on the branch: cargo test --release -p telperion-core (24 binaries, all
pass, identity re-pinned once for both species), cargo clippy -D warnings,
npm run typecheck, uv run scripts/compare-references.py --self-test, the
48-case fixed and fresh protocol (`measure/protocol-fn39/`, all pass).

## Done summary
The crown shell takes an irregular outline by amplitude and wavelength rows; neutral is the smooth shell to the byte. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the European beech's verdict moved to fn-62.
## Evidence
- Commits: 40e42bb5
- Tests: cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release -p telperion-core --no-fail-fast, cargo test --release -p telperion-render --no-fail-fast, npm run typecheck, npm run rust:test:wasm, npm test, uv run scripts/compare-references.py --self-test, node tests/species.mjs --measure-only (48 cases, measure/protocol-round25)
- PRs: