---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-32-bark-as-plates-and-scales.1 Implement Bark as plates and scales

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
# fn-32-bark-as-plates-and-scales.1 - handover, in_progress

R1-R5 implemented, gated and evidenced. R6 and R7 wait on the owner, so
`flowctl done` was not called and the task stays `in_progress`.

**NEEDS_HUMAN: R6 owner judgment on .flow/evidence/fn32/stills; R7 bound is
the owner's call, measured 4.5192 ms p50.**

### What changed

The bark field gains a second primitive, a plate network. The circumference is
partitioned by the same circle embedding `bark_column` uses, so it crosses the
angular wrap seamlessly; each column is cut along its run into plates at
heights of its own, ragged at the plate's own scale so the cuts bend rather
than run in courses; the nearer of the two boundary distances is the furrow.
Cell size carries girth. Faces dome, rims lift, and each plate hashes to an
identity for how proud it stands, how it leans and what colour it keeps.
Colour follows that structure - a greyed, lightened weathered face against a
fresh furrow, a colour for the side turned from the sun and the foot of the
trunk, and a per-channel gain per plate - every term affine in the filtered
height per fn-29's rule. A directional walk towards the sun darkens a furrow
floor its own crest stands over. Depth is parallax on the height field.

Full detail: `.flow/evidence/fn32/REPORT.md`.

### Wire names, ranges and preset values

All fifteen default to zero, so an existing document renders exactly as before.
Range refusals name the field; the blend walks them; the browser mirror and
README carry them.

| Wire name | Range | Ordinary | Oak | Spruce |
|---|---|---|---|---|
| plateCellScale | 0-1 m | 0 | .09 | .022 |
| plateElongation | 0-16 | 0 | 1.8 | .2 |
| plateDome | 0-1 | 0 | .5 | .35 |
| plateEdgeLift | 0-1 | 0 | .22 | .5 |
| plateIdentity | 0-1 | 0 | .55 | .4 |
| weatheringStrength | 0-1 | 0 | .5 | .7 |
| weatheringRed/Green/Blue | each -1-1 | 0,0,0 | .055,.05,.042 | .035,.025,.016 |
| orientationStrength | 0-1 | 0 | .4 | .45 |
| orientationRed/Green/Blue | each -1-1 | 0,0,0 | -.022,.024,-.016 | -.014,.026,-.008 |
| directionalOcclusion | 0-1 | 0 | .55 | .55 |
| depthStrength | 0-1 | 0 | .6 | .45 |

Telperion and Laurelin take the defaults: their ridge scale is zero, so a
plate network over them would be dead arithmetic.

### The depth winner and why

Parallax on the height field, kept; vertex displacement on the near wood
level, deleted.

| Candidate | native total p50 | frame change, trunk pose | 4x aliasing |
|---|---:|---|---|
| row off | 4.351, 4.368 ms | - | - |
| parallax | 4.453, 4.440 ms | mean 1.8916/255, worst 129 | 2.971 / 9.00 |
| displacement | 4.406, 4.391 ms | mean 0.5177/255, worst 74 | 2.839 / 8.25 |

Parallax changes the picture 3.7 times as much for 0.049 ms more, inside the
protocol's own run-to-run spread. Displacement's effect is bounded by how
finely the wood is tessellated rather than by the row; it moves the colour
pass alone, so the tree would shadow and select against a surface it is not
drawn at; and its one virtue, a moved silhouette, is what the spec's boundary
and the fn-24 pins ask to keep still. Parallax's own limit is recorded: it
leaves the silhouette the smooth cylinder the mesh is. Neither reaches the
core, so the wood bytes and fn-24 pins are untouched by both, row on or off.

The eight-tree forest the spec names was retired with the TypeScript harness
at fn-22/fn-24 and has no driver here; both candidates were measured through
the fn-26 single-oak native protocol identically. No full-forest capture.

### Measured crops beside the references

Centre 400x400 crop, mean RGB; then the same crop greyed, auto-levelled and
thresholded at 45% for the dark fraction and the band count. Period is in each
image's own crop pixels - sound between two stills, indicative against a
photograph whose physical scale is unrecorded.

| Crop | RGB | Order | Dark | Period px | Bands/row |
|---|---|---|---:|---:|---:|
| oak-trunk | 145,147,143 | GRB | .2281 | 39.23 | 10.20 |
| OWNER-WHITE-OAK | 118,119,114 | GRB | .3575 | 7.80 | 51.29 |
| OWNER-BLACK-OAK | 141,132,118 | RGB | .3906 | 10.55 | 37.90 |
| O-BARE | 108,141,175 | BGR | .3324 | 17.41 | 22.98 |
| oak-branch | 34,42,49 | BGR | .9758 | 324.54 | 1.23 |
| spruce-trunk | 132,99,73 | RGB | .6805 | 13.32 | 30.02 |
| OWNER-NORWAY-SPRUCE | 154,144,140 | RGB | .2112 | 13.21 | 30.29 |
| S-BRANCH | 92,94,82 | GRB | .7160 | 24.56 | 16.29 |
| spruce-branch | 91,69,53 | RGB | .7836 | 45.75 | 8.74 |
| oak-leaf-frontlit | 125,167,90 | GRB | .8601 | 594.80 | 0.67 |
| oak-leaf-backlit | 96,144,49 | GRB | .8614 | 599.25 | 0.67 |
| O-LEAF | 123,142,85 | GRB | .5433 | 37.02 | 10.81 |
| spruce-needle-frontlit | 169,188,199 | BGR | .1316 | 400.00 | 1.00 |
| spruce-needle-backlit | 166,181,196 | BGR | .1316 | 400.00 | 1.00 |
| S-NEEDLE | 90,79,50 | RGB | .6519 | 18.11 | 22.08 |

The spruce trunk's band period now matches its reference to 1%. The oak keeps
G>=R>=B with a crop mean of 145, inside fn-29's [120,158]. Two findings for
the owner: the oak's dark fraction (0.228) is well under the owner
photographs' (0.357, 0.391), which is at least partly those being close-ups at
an unrecorded scale; and both spruce references measure R/B near 1.1 against
fn-29's [2.1, 2.6] band, which fn-29 set with no spruce reference supplied.

### Timing against fn-29

| Native total | p50 ms | p95 ms |
|---|---:|---:|
| fn-29 round 5, accepted | 3.9823 | 4.5814 |
| fn-32, every term on | 4.5192 | 4.9339 |

+0.5369 ms, +13.5%. The depth row is 0.087 of it; the network, the directional
walk and the structure colour are the rest. No bound was moved - this spec has
none until the owner sets one. Vegetation 4.1400/4.5292, selection
.1032/.1047, shadow .2734/.3031. RTX 3080, NVIDIA 610.57.04, Vulkan, 4
samples a pixel, GPU idle beside it, 90,760 wood caster triangles and 217,328
foliage caster instances unchanged. Browser orbit not run: the browser reaches
this work only through the wasm mirror, which was regenerated and checked.

### Distance and resolution, unchanged bounds

| Test | fn-29 final | fn-32 final | Bound |
|---|---|---|---|
| Distance 2x | 2.247 / 6.00 | 2.550 / 7.75 | mean <=3, p95 <=12 |
| Distance 4x | 2.444 / 6.50 | 2.945 / 8.75 | mean <=3, p95 <=12 |
| Oak near | 1.124 / 4.00 | 2.482 / 8.75 | mean <=3, p95 <=12 |
| Oak grazing | 1.788 / 5.75 | 2.479 / 8.75 | mean <=3, p95 <=12 |
| Spruce grazing | 1.279 / 4.25 | 2.547 / 7.00 | mean <=3, p95 <=12 |

Every redraw byte-identical, worst 0/255. No pin, mask or tolerance moved. The
plates' steepness is bounded by these: a 4.1 mm relief on a 9 cm plate is the
most the unchanged bounds integrate, recorded rather than tuned away.

### Commits

| SHA | Subject |
|---|---|
| 7fbc411 | feat(bark): a plate network and colour by structure |
| 49ed108 | feat(bark): the furrow floor its own crest stands over |
| 9b4689d | feat(bark): depth by parallax, chosen against displacement |
| f246b35 | test(bark): one focused device test per criterion |
| bc38f63 | feat(bark): measured stills, evidence and the R7 native timing |

Range: 34c0751..bc38f63 on branch fn-32-bark-as-plates-and-scales. Nothing
pushed, no rebase, no history rewritten.

### Gates, each with its exit code

| Command | Exit |
|---|---:|
| cargo fmt --all -- --check | 0 |
| cargo clippy --workspace --all-targets -- -D warnings | 0 |
| cargo test --release --workspace | 0 (52 binaries ok, zero adapter skips) |
| npm run wasm:build | 0 |
| npm test | 0 (6 files, 77 passed) |
| npm run typecheck | 0 |

baseline: green (all five gates run before any edit at 34c0751)

stage: impl-review - skipped(config: REVIEW_MODE=none)

stage: plan-sync - skipped(config: planSync.enabled != true)
Owner verdict 2026-09-15: R6 accepting for the spec scope; R7 accepted at the measured 5.2142 ms.
## Evidence
- Commits: 7fbc411c0c81bf7de59099ba17689734be6066a3, 49ed1084d1116467e478cf55ea13372b70c67af8, 9b4689de6d5297aa8cbf97f234dc6784f4a333e4, f246b3549dd561ec4b5a8c61b736e81fb199378d, bc38f630eb8ad87e5b753f1fd361af9a07490751
- Tests: cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, cargo test --release --workspace, npm run wasm:build, npm test, npm run typecheck, ./target/release/examples/headless --preset oregon-white-oak --seed 7 --size 1600x1000 --timing .flow/evidence/fn32/oak-native-timing.json
- PRs:
## NEEDS_HUMAN

NEEDS_HUMAN: R6 owner judgment on .flow/evidence/fn32/stills; R7 bound is the
owner's call, measured 4.5192 ms p50 (4.9339 p95) against fn-29's accepted
3.9823 / 4.5814, +0.5369 ms.

R1 through R5 are implemented, gated and evidenced in
`.flow/evidence/fn32/REPORT.md`; R6 and R7 are the owner's and this task is
left in_progress rather than done. The bark field now carries a plate network
beside its ridges, colour follows that structure, a furrow floor is darkened
by its own crest towards the sun, and depth is parallax, chosen against vertex
displacement on measured cost and measured effect. Fifteen rows were added,
every one inert by default, and all five gates exit 0 with zero adapter skips.
A third question is waiting beside the two the spec asked: both catalogued
Norway spruce references measure R/B near 1.1 while fn-29's recorded
acceptance band is [2.1, 2.6] and was set with no spruce reference supplied;
the render measures 1.81. Whether that band stands or the references replace
it decides a change to fn-26's base colour row, which this spec's boundary
forbids and a further spec would own.
