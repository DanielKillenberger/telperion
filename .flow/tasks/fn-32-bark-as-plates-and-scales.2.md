---
satisfies: [R1, R4, R5, R6]
---
# fn-32-bark-as-plates-and-scales.2 Organicness harness and hill climb to the owner's acceptance

## Description
Round one's stills were rejected by the owner: "i see some issues in how
organic it looks. Looks like armor plating in some cases. Spruce is much
better. But also doesn't look too organic." Acceptance is to be reached inside
this spec by a measured hill climb - numbers judge every iteration, no model
runs inside the loop, and the host views at most four images at the end.

## Acceptance
A structure score against the reference photographs separates bark from brickwork; the preset rows are hill-climbed against it with no model in the loop; the primitive changes only when the row plateau is far from the reference; the owner judges four images at most per round and records the verdict in the spec.

## Done summary
# fn-32-bark-as-plates-and-scales.2 - handover, in_progress

The organicness hill climb is done, gated and evidenced. R6 is the owner's
judgment and R7's bound is the owner's call, so `flowctl done` was not called
and the task stays `in_progress`.

**NEEDS_HUMAN: R6 owner judgment on .flow/evidence/fn32/stills (round three);
R7 bound is the owner's call, measured 5.2142 ms.**

Round three ran after the owner's verdict on round two - "it's much much
better. but still has much room for improvement" - and after the host's
reading of the round-two oak: hairline crackle rather than valleys, and a
trunk too light and too cool. **Its section is the next one; everything below
it is round two, as it was written.**

# Round three

### What shipped

- **`plateFurrowWidth` (0-1, default zero, inert)**: the flat floor of a
  furrow, up to a fifth of a plate's own width, so a bigger plate carries a
  wider furrow off one row. Named, validated, blended, wire-mirrored,
  README'd, and footprint-integrated like every other band. The mean the far
  path returns falls exponentially with it; three rates are pinned beside the
  three levels and held by `bark_plates.rs` at 0.25 and 0.6 as well as at
  whatever the presets carry.
- **Two constraints on the climb rather than two more terms**: an accepted
  step must keep every channel of the crop's mean inside a band around the
  reference's own crop mean and may never move further from it (oak: thirty
  code values of 118,119,114, which its crop already sits at the edge of;
  spruce: eighty of 154,144,140, wide enough to contain the colour fn-29 was
  accepted at, because which is right is the owner's open question), and it
  may not lower the dark fraction.
- **A level pass** after the descent: every row walked towards the
  reference's level while the score stays within two per cent of the plateau.
- **The plate means, re-measured** over sixteen slices instead of one, with
  the test reading the shipped rows from the presets instead of a copy that
  had drifted a round behind.

### Scores, round two to round three

| Still | round two | round three |
|---|---:|---:|
| oak-trunk | 1.2284 | **1.3293** |
| spruce-trunk | 0.1658 | **0.1870** |
| oak-branch | 1.4856 | 1.4510 |
| spruce-branch | 0.6595 | 0.6786 |
| leaf and needle stills | - | byte-identical |

**Both trunk scores went backwards, and the cause is the re-measured pins,
not the rows.** The same rows round two ended at measure 1.5231 under the
corrected constants; the descent and level pass bought 0.194 of that 0.295
back. The old pins were a biased single-slice estimate of a field whose
shipped row had since moved.

### Two measured bounds on what this spec can still do

- **The level is out of reach.** With every row this spec owns at its darkest,
  the oak's crop mean falls from 142,145,138 to 140,144,133; the reference is
  118,119,114. The level belongs to fn-29's bark colour rows, which this
  spec's boundary forbids redoing.
- **The dark fraction is the cylinder, not the bark.** With every term of this
  spec at zero the oak trunk still measures 0.127 of its 0.147. A flat field
  (removing a least-squares quadratic before the threshold) was built to fix
  it and rejected on its numbers: it left two of three oak references with no
  measurable light regions at all.

### The furrow row ships inert

Every width costs score on both species, mostly through area variation - a
wider floor turns plates into islands below the statistic's own area floor.
Oak: 1.3293 at zero, 1.4877 at 0.05, 1.5824 at 0.10, 1.6298 at 0.15, 1.6377
at 0.35, 2.4779 at 0.50; 0.05 and 0.25 also alias past the bound. Spruce:
0.1870, 0.2248, 0.2477. The two images this round spent agree with the
number: a wide floor over a relief the anti-aliasing contract caps at four
millimetres reads as flakes with gaps, not as valleys. The widest
guard-holding setting is kept beside the eight as
`stills/oak-trunk-wide-furrow.png` with its own measurement.

### Timing, round three

5.2142 ms p50 and 5.2859 p95, against round two's 5.2129 / 5.6993: unchanged,
because an inert row and a changed row value cost nothing.

### Gates, round three

All six exit 0, zero adapter skips. Two tests were repaired rather than worked
around, both broken by the new row rather than by anything it does:
`conformance.rs` now draws its jitter from each field's own path by a stable
hash instead of from a running stream (one new row had redrawn every factor
after it: 47 of 60 sets refused, now 20 rendered of 26 tried), and its
`compact` moves the one material trait that sits on its own upper bound off
it, as it already did for the habit and element traits; `sweep.rs` holds the
new row in its no-preset-moves inventory while it is inert.

---

Full detail, including every table: `.flow/evidence/fn32/REPORT.md`, second
half. Every trial: `.flow/evidence/fn32/hillclimb.json`. Per-still vectors:
`.flow/evidence/fn32/stills.json`.

### The score

Six components on the centre 400x400 crop after fn-32's normalisation (grey by
Rec.709 luminance in linear light, auto-levelled, thresholded at 45%):
orientation entropy over 18 Sobel-direction bins weighted by magnitude
(scale 0.05, weight 0.4); coefficient of variation of the light regions' areas
(0.70, 1.4); furrow curvature, boundary length over chord on fixed runs of the
plate-to-furrow boundary (0.45, 1.2); junction arms at the thinned dark
network's branch points (0.50, 0.6); dark fraction (0.25, 0.8); furrow period
as a log ratio (1.00, 0.2). The score is the normalised weighted distance from
a still's vector to the centroid of its catalogued references' vectors.

Area variation carries most because "armor plating" is plates of one size and
nothing else sees it; curvature next because straight courses were the defect
round one caught only by eye; the period is deliberately weak because the oak
crop's 62 px period is a 6.5 cm plate at the pose's known 0.42 m of wood -
inside what a white oak wears - while the references' 8-18 px is how close
those photographs were taken, at a scale nobody recorded.

It ships as `crates/telperion-render/src/structure.rs` with
`examples/bark_score.rs`, guarded by `tests/bark_score.rs`: a lattice of
identical cells against a jittered cellular network, every component
separating them on its own. On round one's stills it reproduced the owner's
ranking without being told it - oak 1.9346, spruce 0.7764.

### Scores, round one to round two

| Still | round one | round two | nearest reference, round two |
|---|---:|---:|---|
| oak-trunk | 1.9346 | **1.2284** | OWNER-WHITE-OAK 0.9853 |
| spruce-trunk | 0.7764 | **0.1658** | OWNER-NORWAY-SPRUCE 0.1658 |
| oak-branch | 1.4954 | 1.4856 | OWNER-WHITE-OAK 1.4839 |
| spruce-branch | 0.6118 | 0.6595 | S-BRANCH 0.7358 |

The four leaf and needle stills are byte-identical to round one's. Only the
two trunk stills were climbed against; the spruce branch pays 0.05 for it.

The spruce trunk matches its reference on every structure component: area
variation 1.510 against 1.484, curvature 2.736 against 2.731, junction arms
3.880 against 3.914, period 12.6 against 13.1.

### Rows that moved

| Row | Oak | Spruce |
|---|---|---|
| plateCellScale | .09 -> .084 | .022 -> .028 |
| plateDome | .5 -> .39375 | .35 -> .39375 |
| plateEdgeLift | .22 -> .27 | .5 unchanged |
| plateIdentity | .55 -> .45 | .4 -> .375 |
| weatheringStrength | .5 -> .575 | .7 -> .6 |
| weatheringRed/Green/Blue | .055,.05,.042 -> .06,.05,.02325 | .035,.025,.016 -> .015,.03,.02975 |
| orientationStrength | .4 -> .325 | .45 -> .44375 |
| orientationRed/Green/Blue | -.022,.024,-.016 -> -.0495,.02275,-.086 | -.014,.026,-.008 -> -.014,.026,-.0155 |
| directionalOcclusion | .55 -> .85 | .55 -> .41875 |
| depthStrength | .6 unchanged | .45 -> .45625 |
| plateElongation | 1.8 unchanged | .2 unchanged |

### The primitive changed

Round one's row-only plateau left the oak at 1.2467, 64% of where it started,
against the task's test of one half - so the primitive changed, once.

The column-and-cut network is replaced by a cellular partition of the surface:
sites scattered in the space the bark passes through, a boundary where two of
them are equally near, three boundaries meeting at a point because three sites
do, a size per site so no two cells match, and two filtered warps so nothing
runs straight. The wrap still has no seam - the circle embedding carries it -
and elongation is scaled back out before any distance is taken, so a wall is
the same width in metres whichever way it runs. The three pinned plate means
were re-measured for the new field and now nearly agree between the species.

One consequence was measured: the partition's cells are smaller than the
lattice they are drawn from, so the sun walk was striding past the crest and
R3's term measured 0.0371 against its criterion of 0.05. The walk now takes
three steps over a reach of one wall and the oak's occlusion row carries the
rest; the term takes 0.079 and 0.069 with the sun crossed.

Two variants were tried and rejected on their numbers, both recorded: a wall
widened to a fifth of a plate (worse for both species), and a pruned cell
search (5.554 ms against 5.213 - a divergent branch costs a warp more than the
two hashes it saves).

### Timing

| Native total | p50 ms | p95 ms |
|---|---:|---:|
| fn-29 round 5, accepted | 3.9823 | 4.5814 |
| fn-32 round one | 4.5192 | 4.9339 |
| fn-32 round two | **5.2129** | **5.6993** |

+0.6937 ms on round one, +1.2306 on fn-29. The third step of the sun walk is
0.148 of it; the partition, which reads twenty-seven cells where the columns
read nine, is the rest. No bound was moved: this spec has none until the owner
sets one.

### Guards

Every accepted step of the climb held the distance, resolution and redraw
numbers the shipped tests assert, computed in the same process at the same
poses against the same masks and bounds, plus one guard of this round's own:
no channel of the crop's mean may move more than six code values and the
channel order may not change, because fn-29's colour is not this spec's to
redo. The colour guard never bound (largest drift 3.4 code values). The
replica agrees with the shipped binaries to four decimals: distance 2.3441 and
2.8257, trunk 1.8028, oak grazing 2.9677, spruce grazing 2.7908, every redraw
byte-identical. The oak grazing pose has 1% of its bound left and that is
recorded rather than spent quietly.

### Gates, every checkpoint

| Command | Exit |
|---|---:|
| cargo fmt --all -- --check | 0 |
| cargo clippy --workspace --all-targets -- -D warnings | 0 |
| cargo test --release --workspace | 0 (53 binaries ok, zero adapter skips) |
| npm run wasm:build | 0 |
| npm test | 0 (6 files, 77 passed) |
| npm run typecheck | 0 |

Baseline before any edit: green. No gate was skipped, no receipt was reused,
no tolerance was widened. The only pinned numbers that moved are the three
plate means, which are re-measurements of a changed field and are asserted
against that field by `bark_plates.rs`.

### Commits

`fc2434e..HEAD` on `fn-32-bark-as-plates-and-scales`:

- `bd2f193` feat(bark): a structure score that separates bark from brickwork
- `438aac2` feat(bark): hill-climb the rows against the score, no model in the loop
- `cb86074` feat(bark): a cellular partition of the surface, not a wall of columns
- `2a23a59` feat(bark): capture four, the measurement beside it and the round-two report
- `9abe16a` chore(flow): fn-32.2 NEEDS_HUMAN blocker for the owner's R6 and R7 calls
- `526da27` feat(bark): the furrow floor as a row, and a mean that falls with it
- `5f75ec1` feat(bark): round three - the level walked, the furrow measured and declined
- plus this handover's checkpoint

### Budget and departures, declared

- Round two: three images of four. Round three: two of four - the oak trunk as
  it ships and the oak trunk at a furrow width of 0.35 - spent on the one
  question the numbers could not settle alone. No full-forest capture in
  either round.
- Eight commits of ten across the two rounds. Zero tokens and no model inside the optimisation loop:
  1,510 trials ran in one process that rendered, measured and guarded itself.
- **The constraint tests were replicated in-process rather than re-run as
  binaries at each accepted step.** Running `cargo test --release -p
  telperion-render --test bark_distance --test bark_resolution --test look`
  after each of 60 accepted steps would have cost hours of rebuilds; the
  replica computes the same numbers at the same poses and was checked against
  the shipped binaries at the end, where it agreed to four decimals. The
  binaries themselves are green on the shipped values.
- **`look` could not be reached by any row this climb moved**: it draws the
  ordinary family, whose plate rows are all zero. It is green.
- **Commit attribution.** The harness's own attribution line for this session
  names Claude Opus 5 (1M context) and states that it replaces earlier
  guidance; the dispatch asked for Claude Fable 5.1. The commits carry the
  harness line, which is also the model that actually wrote them.

### What the numbers say is still wrong

- The oak's furrows hold 0.146 of the crop against the references' 0.356. It
  is the largest remaining gap and the lever is fn-29's fissure rows, which
  this spec's boundary forbids redoing. **A further spec that opens them is
  the honest next step.**
- The oak's area variation reached 0.850 against 1.955; part of that gap is
  the same darkness, part is that 0.42 m of trunk holds about twenty plates
  where a close-up photograph holds hundreds.
- The spruce branch moved 0.05 the wrong way: the rows were climbed on the
  trunk pose and a branch is a different girth.

stage: plan-sync - skipped(config: planSync.enabled != true)
Owner verdict 2026-09-15 after round three: R6 accepting for the spec scope; R7 accepted at 5.2142 ms; furrow width ships inert; level, depth and capture geometry deferred to a new spec.
## Evidence
- Commits: bd2f1938b2054b986a66ecfee8b61ecaa71950b6, 438aac2c51962edb89f4c7ededd01fb481bc486e, cb86074288bcf6f9936003d60f44af23e01d7141, 2a23a59534b7bf35c580dcfa476fca6be0dcc723, 9abe16ac1898e6e109d439403cb9effb4765c4a6, 526da27ebb289ebd52e1e89be62a43242eea3aae, 5f75ec1d52aeed18356936634c4025a09cd5da1b
- Tests: cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, cargo test --release --workspace, npm run wasm:build, npm test, npm run typecheck, cargo test --release -p telperion-render --test bark_score (the acceptance instrument's own guard), ./target/release/examples/headless --preset oregon-white-oak --seed 7 --size 1600x1000 --out <scratch>/hero.png --timing .flow/evidence/fn32/oak-native-timing.json, cargo test --release -p telperion-render --test bark_plates (the far path's mean against the field, three rows and a wide furrow), ./target/release/examples/bark_climb <species> probe:<row>=<value>,... <references> (the furrow sweep, the darkest-rows bound, the all-terms-off dark fraction)
- PRs: