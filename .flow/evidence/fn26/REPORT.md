# FN26: grazing relief and physical footprint filtering

2026-09-13, session 7. The four requested corrections and six final detail
stills are implemented. Final native p50 is **3.6879 ms**, below 3.8 ms by
**0.1121 ms**; the valid browser orbit delivers **601 frames in ten seconds**,
wall p95 **16.7 ms**. All five final gates pass. All owner verdict
slots remain blank; this report does not close the spec. The session-six
blockers are absent; its report is archived as session6-REPORT.md.

## What changed

Columns and axial plate breaks now share one two-axis band rejection. Neither
can survive alone after the other becomes unresolved. Normal slopes come from
shared corner heights of the filtered field, with the physical footprint held
fixed. A 3×3 grid supplies four shading cells with nine field evaluations,
replacing twenty independent evaluations. Four outer corners supply one coarse
box-averaged normal. The two lighting quadratures blend smoothly over three to
two pixels per wavelength before reducing the sample count. This changes the
integration cost without switching off the resolved height field.

The perturbed normal blends toward the original normal at grazing incidence:
smoothstep(0,0.6,abs(N dot V)), weight 0.5 at cosine 0.3 and zero at tangency.
Bounding the normal blend prevents a large slope from defeating the fade.
The circle embedding's existing metre derivative is retained; a speculative
facet-incidence multiplier was tested and rejected. All geometry derivatives
are evaluated before non-uniform fragment branches, as Chromium requires.

Completely unresolved or young/smooth bark takes one lighting evaluation,
skipping constant-height differences. MSAA and the single original shadow
lookup are unchanged.

No material row, light, shadow implementation, texture, vertex layout, radius
upload, mesh, selection order, identity pin, leaf term or tolerance changes.
The spruce row remains 0.020 m ridge scale, 0.030 m plate scale and 0.025 furrow
strength; oak remains 0.032/0.055 m and strength 1.

## Distance filter and derivation

The actual shader footprint F is measured in metres, separately across the
radius-scaled circle and along the run: derivative-length sum and fwidth.
For a wavelength L, q=F/L. Both axes share rejection
1-smoothstep(0.5,1,max(q_across,q_along)): no whole-band fade above two pixels,
50% retained at 1.333 pixels, and the constant mean at one pixel.

Interpolated lattice noise has a shortest alternating wavelength of **two
cells**, so its q is cell-footprint/2. Treating a cell as a wavelength was
prematurely fading coordinate warps and moving coarse outlines between
resolutions. Noise warps now remain intact while resolved; averaging is applied
to their resulting height profiles and normal slopes, rather than additionally
shrinking resolved warp amplitudes.

A cubic edge of span s has derivative variance s²/20. A pixel box has variance
F²/12. Additional box width is therefore sqrt(max(F²-0.6s²,0)), where 0.6=12/20.
The shader integrates the cubic primitive x³-x⁴/2, with constant tails, about
the original edge centre. The intrinsic edge is unchanged when its existing
width covers the pixel. This retains plate faces and furrow locations instead
of widening a shoulder only into the face until the face disappears.

Removed high-frequency slope variance contributes through the existing
roughness-detail row. Squared profile-slope estimates weight the fine flake,
plate and furrow contributions; removed variance is 1 minus squared retained
response. A polynomial sinc estimates the box response. This is a complement
to retained relief, not a replacement for its low-frequency shapes.

Facing-arc projection check, using the actual oak radius r=0.4109311114 m,
stand-off d=6r and the unchanged 38-degree lens:
F≈2(distance-r)tan(19°)/1000. The shader still uses local derivatives,
including the larger grazing footprint; this table is a facing-arc sanity check.

| Framing | F, mm/pixel | 32 mm columns, pixels | 55 mm intervals, pixels | Whole-band fade |
|---|---:|---:|---:|---:|
| Near | 1.414949 | 22.62 | 38.87 | 0% |
| 2× | 3.112888 | 10.28 | 17.67 | 0% |
| 4× | 6.508767 | 4.92 | 8.45 | 0% |

The GPU distance fixture's 1.414/6.514 mm footprints match these estimates
within 0.1%. Far height deviation is 0.000836467 m versus 0.000913929 m near
(91.52% retained). Delivered stills show outlines at both 2× and 4×. The far
frame no longer becomes a blank shiny trunk. The near frame retains the prior
plate character and pose; grazing and fine filtering change some pixels, so
**unchanged near appearance is not a claim of byte equality**. The intrinsic
zero-footprint field and material values are retained.

## Resolution and redraw evidence

All comparisons use delivered sRGB RGB channels: exact 2×2 box reduction of
1600×1000 versus native 800×500. Limits remain mean ≤3/255 and p95 ≤12/255.
No registration, sharpening, tolerance change or comparison-image processing.

The original near mask is unchanged (x=280..520, y=8..492). The new grazing
mask is selected from a CPU perspective raster of the original wood: main-run
pixels at view cosine 0.12..0.35, with occlusion and two-pixel coverage erosion.
Oak has 6178 pixels; spruce 2962. The off-centre 20-degree camera targets y=2
for oak and y=0.9 for spruce, below its first fork. Foreground twig pixels were
caught by a detail-disabled control and excluded geometrically before fixing
the final baseline. The 2×/4× comparisons use fixed interior trunk strips.

| Test | Before: mean / p95 | Final: mean / p95 |
|---|---:|---:|
| Oak grazing | **5.013732 / 19.00 — fails** | **1.407319 / 5.25 — passes** |
| Spruce grazing | 2.487593 / 7.75 | **0.842843 / 2.75 — passes** |
| Original oak near | 2.655901 / 11.75 (session 5) | **1.293709 / 4.75** |
| Oak 2× | — | **1.375550 / 4.75** |
| Oak 4× | — | **2.992883 / 8.50** |

Red: logs/session7-grazing-final-red.log. Spruce already passed the tolerance
before correction; the combined regression fails on oak. Plain edge controls
are 0.227609/0.50 (oak) and 0.335978/0.50 (spruce).
Distance test red: logs/session7-distance-red.log (premature band rejection),
and session7-distance-render.log (rendered 2× error). Final targeted green:
logs/session7-uniform-derivatives.log. The 4× mean has only 0.007117/255
margin; it is not presented as a loose pass. Intermediate failures remain in
the notes. Browser compilation also has a retained red/green regression:
logs/session7-browser-compile-red.log and session7-browser-compile-green.log.
It compiles the actual renderer with Chromium and asserts no WGSL errors,
without generating a frame or timing sample.

Identical near and grazing redraws move **zero channels, worst 0/255**.
The original exact-equality assertions are untouched; logs/session7-final-redraw.log
confirms all three existing look tests and zero moved channels, worst zero. Enabled/disabled wood
hashes both remain **9238220531640137937**; original identity tests and physical
field contracts are retained. Both unresolved-axis height deviations are zero.

## Stills and reference comparison

All six final detail stills are 1600×1000, seed 7, bare view, with the original
scene and geometry. No image postprocessing. The oak near camera is exactly
the session-four/five camera; the distance series scales its eye-target offset.

| Final capture | Still | Judged beside / observation |
|---|---|---|
| 10 | stills/oak-grazing-trunk.png | OWNER-WHITE-OAK, OWNER-BLACK-OAK; relief diminishes at the edge instead of continuing as rails |
| 10 | stills/spruce-grazing-trunk.png | OWNER-NORWAY-SPRUCE and harness-spruce-grazing-streaks.png; shallow scales fade at tangency |
| 10 | stills/oak-trunk.png | OWNER-WHITE-OAK, session-five near; same plate character and framing |
| 10 | stills/oak-distance-2x-trunk.png | OWNER-WHITE-OAK; plate faces and furrows remain resolved |
| 10 | stills/oak-distance-4x-trunk.png | OWNER-WHITE-OAK; visible outlines on trunk/root flare, without a glossier far appearance |
| 10 | stills/spruce-trunk.png | OWNER-NORWAY-SPRUCE; target (0,0.65,0), stand-off 1.8 m, exposing scales below the first fork |

Capture 10 refreshed every final still after the normal-sharing optimization
and Chromium correction. Its four inspected images are both grazing views,
the spruce trunk and the 4× oak. Final near and 2× images were compared
numerically to the previously inspected same-pose images: mean RGB changes
**0.619376/255** and **0.822292/255** in fixed trunk masks. Final near versus
session five is **1.891480/255**; it is not byte-identical. Values and masks are
in session7-still-comparison.json. No comparison image is saved or delivered
as a still, and the source PNGs are unmodified.

The spruce photograph has shallow, irregular 1–3 cm scales. The final row's
2–3 cm scales and narrow outlines are visible in the replacement. Its brown
colour, clean faces and more uniform vertical organisation still differ from
the grey, weathered photograph. Oak likewise remains cleaner than its reference.
These are exposed limitations, not accepting owner verdicts. Original twigs
remain in every spruce image; no geometry was removed to obtain the view.
The retained branch/leaf stills are historical and unchanged.

stills.json records twelve current/historical detail stills and their hashes.
All five supplied reference filenames/provenance/checksums are recorded in
session7-reference-metadata.json; reference bytes are never redistributed.
Older O-BARE/O-LEAF/S-BRANCH/S-NEEDLE metadata remains in references.json.
Capture 4 inspected exactly four images (three new views and the historical
near); capture 10 inspected four. All others inspected at most two. No extra
image views followed. **Ten of ten captures used**, with this complete ledger:

| Capture | Evidence process |
|---|---|
| 1 | Initial oak grazing, now archived before the shared-normal change |
| 2 | Initial spruce grazing, likewise archived |
| 3 | Rejected obstructed spruce camera; discarded/session7-capture3-spruce-obstructed.png |
| 4 | Initial oak near/2×/4× series, three images in one process |
| 5 | Initial lower spruce replacement, subsequently refreshed |
| 6 | Valid native cost diagnostic, **4.5775 ms p50**, rejected for budget |
| 7 | Valid shared-normal native clock, **3.6695 ms p50**, before derivative hoisting |
| 8 | **Disjoint** browser orbit, rejected; Chromium rejected non-uniform derivative control flow |
| 9 | Valid final browser orbit after the correction |
| 10 | One sequential final evidence process: unchanged native timing protocol, then six detail stills |

Initial detail stills remain in session7-before-shared-stills; the obstructed
spruce diagnostic remains in discarded/. The incidental native hero at
session7-native-hero.png was refreshed but never inspected or used for bark
judgement. Compilation probes create no tree, frame, screenshot or clock and
are tests, not evidence captures.

## R5 clocks

The final native and browser measurements are valid. No contended result was
returned or accepted. The disjoint browser result was diagnosed and replaced,
not reported as R5 evidence. The first native result exceeded the bound and
prompted the shared-height optimization; a final native clock follows the
browser portability fix. All clocks ran sequentially, with no concurrent
build, test or other timing process owned by this session. Prior CPU-only work
on the shared machine was neither stopped nor reprioritized.

| Native total | p50, ms | p95, ms | Verdict |
|---|---:|---:|---|
| fn14 | 4.949 | 5.412 | historical valid |
| fn27 | 3.4243 | 3.7484 | historical valid |
| fn26 session 5 | 3.9276 | 4.4613 | valid, 0.1276 ms over 3.8 |
| fn26 session 7 final | **3.6879** | **4.0284** | **valid, 0.1121 ms below 3.8** |

Final native vegetation p50/p95 is **3.3085/3.6076 ms**, selection
**0.1032/0.1055 ms**, unchanged shadow counter **0.2724/0.3113 ms**. Total p50
improves session five by **0.2397 ms**. NVIDIA GeForce RTX 3080, NVIDIA
610.57.04, Vulkan; 1600×1000, seed 7, whole view. The protocol remains one
initial hero render, eight conditioning, eight warmup and 120 measured frames.
Capture 10 invokes the same public measure function and Frame target used by
the unmodified headless example, before rendering any detail stills.

| Browser orbit | wall p50 / p95 / max, ms | GPU total p50 / p95, ms | Frames |
|---|---|---|---:|
| Session 5 | 10.00 / 10.10 / 50.00 | 3.9360 / 4.5773 | 992 |
| Session 7 final | **16.70 / 16.70 / 16.80** | **3.7379 / 3.9265** | **601** |

The ten-second orbit meets wall p95 ≤16.7 ms and the existing 33 ms hitch guard.
Its cadence is 60 fps rather than the prior session's approximately 100 fps;
GPU cost is lower. The display-cadence difference is reported, not attributed
to a confirmed cause. Chromium **153.0.8010.12**, NVIDIA/ampere WebGPU adapter,
same launch flags, same isolated canvas on the owner's localhost:5175 server.
Selection is **0.1085/0.1106 ms**. Browser wall values have 0.1 ms quantization.

Final records: oak-native-timing.json and oak-browser-orbit.json. Historical
session-five records are preserved with the session5-final prefix. Rejected
session-seven records are session7-first-native-timing.json and
session7-disjoint-browser.json. The recovered pre-hoist native record is
session7-shared-native-timing.json. Logs: session7-capture-9-browser.log and
session7-capture-10-final.log under logs/.

## Gates and boundaries

- Formatting: logs/session7-final-gate-fmt.log, exit 0.
- Clippy: logs/session7-final-gate-clippy.log, exit 0; fixed the new test's chunk API without suppression.
- Full release workspace: logs/session7-final-gate-rust-confirmed.log, exit 0 (also recorded in session7-final-gate-rust.exit).
- Explicit wasm build and npm test: logs/session7-final-gate-npm.log, exit 0, 66 tests; full command output.
- Typecheck: logs/session7-final-gate-typecheck.log, exit 0.

The first final Rust wrapper returned 143 after all suites reported success.
That attempt is retained but does not count as a green gate. The exact command
was rerun without code or flag changes and exited zero, explicitly recorded
in session7-final-gate-rust.exit.

render:build ran after shader changes, most recently during the npm gates.
No production shading change follows the delivered stills. Temporary capture
source was copied into the handover and removed from the worktree. Changes
remain uncommitted for the host. No .flow writes, agents, other-worktree edits,
priority changes, or changes to the owner's port-5175 server.

Additional browser regression: `BROWSER_URL=http://localhost:5175 node
tests/browser/bark-shader.mjs`, red before derivative hoisting, then green.
Chromium compilation info caught an error accepted by native validation.

Validation scheduling differs from the requested item order: final clocks
follow the distance correction so that R5 measures the final shader. Spruce
framing was lowered after rejecting the inherited obstructed camera. The final evidence
process combines the native clock and six stills, with only four images viewed,
within the ten-capture limit. Earlier distance-series processes also produced
multiple images, consistent with the earlier multi-view evidence protocol.

## Owner verdict

| Species | Scale | Reference | Owner verdict |
|---|---|---|---|
| Oregon white oak | trunk, distance, grazing | OWNER-WHITE-OAK; OWNER-BLACK-OAK; O-BARE | |
| Oregon white oak | branch/socket | OWNER-WHITE-OAK; O-BARE | |
| Oregon white oak | leaf | O-LEAF | |
| Norway spruce | trunk, grazing | OWNER-NORWAY-SPRUCE | |
| Norway spruce | branch/socket | S-BRANCH | |
| Norway spruce | needle | S-NEEDLE | |
