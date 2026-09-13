# FN26: bark relief, leaf veins and transmission

2026-09-13, session 5. All five code gates pass, and the new resolution
regression fails before this change and passes after it. **R5 is not cleared:**
the valid native total p50 is **3.9276 ms**, above the 3.8 ms bound by
**0.1276 ms**. The valid browser orbit also contains a **50 ms** worst frame.
No filter was weakened or implementation changed after those clocks.

The final oak distance series is delivered. The final spruce row is implemented,
but its replacement still remains outstanding: the one requested spruce capture
was an internally rejected filtering diagnostic. Permission to replace it within
the eight-capture limit was requested and has not arrived. Seven captures were
used; one remains. R4 stays open. Every owner verdict slot below is blank.

Changes remain for the host to commit. No git writes, .flow writes, agents,
priority changes, owner-server changes or other-worktree edits were made.

## Field and row

The two physical footprints are the axial distance fwidth and the sum of the
screen derivatives' lengths of the radius-scaled circle embedding. This measures
the coordinates read by the field in metres without an angular-wrap derivative.
The arc footprint grows at grazing incidence. Each noise projection propagates
those footprints through its own frequencies. Noise, ridges, plates and flakes
fade toward constant mean levels; band amplitude falls smoothly between
one-quarter and one-half wavelength per pixel. The fine flake uses the complete
footprint of its warped axial coordinate and fades first.

Every shoulder, outline and lifted edge uses smoothstep support at least as
wide as the propagated footprint. The final filter also smooths the intrinsic
profiles: full-strength shoulders span 0.28–0.32 ridge units, and axial edges
have at least 0.15 interval units of half-width. This is a deliberate visible
antialiasing tradeoff: merely widening sharp steps with the camera either
failed resolution agreement or erased the plate faces. Adjacent axial plates
contribute when their support overlaps a cut. Column boundaries retain zero
height and slope; the rejected two-column seed blend is absent.

The surface-gradient normal takes symmetric physical-coordinate differences of
the **filtered height**, holding the pixel footprint fixed. It does not
differentiate an unfiltered height or turn camera-dependent filter changes into
surface relief. Four explicit subpixel evaluations integrate the normal's
nonlinear lighting. These are shading samples; geometry multisampling remains
four, and the existing shadow lookup remains one per fragment using the
original surface normal. No new light, texture, shadow model, displacement,
vertex attribute, radius upload or mesh byte is introduced. Constant, unresolved
bands and smooth young wood skip invisible calculations.

| Material control | Oak | Spruce |
|---|---:|---:|
| Ridge scale, m | 0.032, unchanged | 0.020, previously 0.025 |
| Plate scale, m | 0.055, unchanged | 0.030, previously 0.090 |
| Furrow strength | 1.0 | 0.025 |
| Bark RGB, linear | 0.225, 0.218, 0.198, unchanged | 0.147, 0.078, 0.045, unchanged |
| Roughness detail | 0.12, unchanged | 0.16, unchanged |

**furrowStrength is the one added numeric term.** It controls furrow width and
depth together, independently of plate and flake spacing. It is range-validated
in 0–1, serialized, exposed through the generic panel and linearly blended like
the other row values. Its default of one preserves older enabled rows. At zero
the low scales retain narrow outlines. Presets remain value tables; no species
branch is added. Spruce uses the upper part of the photograph's 1–3 cm range
with very little broad furrow relief. Its colour remains fn-29's responsibility.

The existing young-wood fade and girth strengthening remain. Leaf veins,
transmission, selection ordering, leaf geometry and leaf stills are unchanged.
The retained leaf observations remain: frontlit oak veins and margin tone,
yellow-green backlit transmission with reduced vein contrast, and plain,
nearly opaque backlit spruce needles.

## Visual comparison and captures

All stills are seed 7, 1600 × 1000, bare view, unchanged scene and geometry.
The near oak uses the exact session-four camera:
eye (1.7307636095778745, 2.2967023330704928, -1.7307636095778745),
target (0, 2, 0), 38-degree field of view, near/far 0.01/1000.
The other distances multiply the eye-to-target offset by two and four.
No image is postprocessed. No capture inspected more than three images.

| Capture | Result | Path / log |
|---|---|---|
| 1 | Rejected spruce diagnostic: over-filtered, scales almost erased | discarded/session5-capture1-spruce-trunk.png; logs/session5-capture-1-spruce.log |
| 2 | Rejected oak diagnostic: too smooth, isolated pinpricks | discarded/session5-capture2-oak-trunk.png; logs/session5-capture-2-oak-near.log |
| 3 | Final oak, session-four framing | [oak-trunk.png](stills/oak-trunk.png); logs/session5-capture-3-oak-near.log |
| 4 | Final oak, twice as far | [oak-distance-2x-trunk.png](stills/oak-distance-2x-trunk.png); logs/session5-capture-4-oak-2x.log |
| 5 | Final oak, four times as far | [oak-distance-4x-trunk.png](stills/oak-distance-4x-trunk.png); logs/session5-capture-5-oak-4x.log |
| 6 | Valid native clock; above budget | logs/session5-capture-6-native.log; incidental session5-native-hero.png not inspected |
| 7 | Valid browser clock; 50 ms worst frame | logs/session5-capture-7-browser.log; no screenshot |

At the original distance, raised scales and wandering furrows remain clearly
visible. Compared with [session four](discarded/session4-final-oak-trunk.png),
the edges are broader and rounder, the fine grain is quieter, and the sharp
black/white steps have softened. It is not a claim of unchanged pixels or
identical lifted lips. The field still looks cleaner and more regular than
the owner's white-oak photograph. At 2x the scale pattern continues with less
contrast toward the sides. At 4x the trunk is smooth with faint axial structure;
no isolated sparkling grain is visible. Existing geometry and cast shadows
remain exposed. Owner acceptance of this tradeoff is still required.

The initial spruce diagnostic was compared with the Norway-spruce photograph
before the oak series. It failed the visual goal despite an intermediate numeric
pass, so it was excluded from the verdict set. The prior session-four
[spruce-trunk.png](stills/spruce-trunk.png) is restored unchanged and is explicitly
**historical evidence, not a capture of the final row or shader**. A replacement
was requested because the prompt permits one spruce capture; elapsed time is
not permission. No second spruce capture has been taken.

The other seven original detail stills are unchanged: oak branch (session 4);
spruce trunk (session 4); spruce branch and all four front/back leaf stills
(session 2). Both branch stills predate this shader. The spruce foreground-twig
obstruction and unresolved clear-socket framing request remain exposed.
stills.json records all ten current/historical stills, their sessions and hashes.

## Resolution and redraw regression

The retained test renders the actual oak at the pinned near camera at
1600 × 1000 and 800 × 500. It compares the native smaller frame with an exact
2 × 2 box average of the larger frame's delivered sRGB channels, with no
registration, sharpening or image filtering beyond that box reduction.

The fixed, material-independent mask is x=280..520, y=8..492 at 800 × 500:
116160 interior trunk pixels, including shade and the foreshortened right side,
excluding geometric silhouettes, sky, ground and frame edges. Limits were
chosen **before** the first measurement: mean absolute RGB error at most
3/255 and channel p95 at most 12/255. They were never loosened.

| Shader | Mean error /255 | p95 error /255 | Outcome |
|---|---:|---:|---|
| Session four, before edits | 12.731650 | 48.25 | fails |
| Final session five | 2.655901 | 11.75 | passes |
| Detail-disabled diagnostic | 0.155721 | 0.50 | control only |

Red log: logs/session5-resolution-red.log.
Final green log: logs/session5-profile-green.log.
The identical 800 × 500 trunk redraw moves **zero channels, worst 0/255**.
The existing three look tests also pass with their exact-equality assertion
unchanged: logs/session5-final-redraw.log reports zero moved channels, worst zero.

The additional production-field GPU contract failed on the saved session-four
shader in logs/session5-axis-red.log: unresolved arc and axial variation both
remained 0.000651567 m. The final shader gives zero variation for either
unresolved axis. Closing furrows retains scale relief: height deviation
0.000244849 m versus 0.000645047 m with full furrows. Existing wrap, physical
spacing, young-wood and axial/girth assertions pass unchanged. The new row and
panel assertions failed on the missing trait before implementation:
logs/session5-row-red.log and logs/session5-panel-red.log; their final passes
are in the complete gates below. Every intermediate rejection and correction
is documented chronologically in child-notes.md.

## R5 numbers

Both final clocks returned valid on their first measurement attempt. No
contended verdict occurred or was accepted. All of this session's builds and
tests had finished; another session's CPU-only species tests remained active.
GPU utilization at preflight was 8–9% with existing desktop processes. Nothing
was stopped or reprioritized. The values are shared-machine measurements;
no causal or zero-cost claim is made from single sessions.

| Native whole view | Total p50, ms | Total p95, ms | Measurement verdict |
|---|---:|---:|---|
| fn14 | 4.949 | 5.412 | valid |
| fn27 | 3.4243 | 3.7484 | valid |
| fn26 session 3 | 3.5323 | 3.8410 | valid |
| fn26 session 4 | 3.5656 | 3.8853 | valid |
| fn26 session 5 | **3.9276** | **4.4613** | valid; **over budget** |

Native p50 rose 0.3620 ms and exceeds the 3.8 ms bound by 0.1276 ms.
Vegetation p50/p95 is 3.5451/4.0550 ms; selection 0.1034/0.1050 ms.
The unchanged shadow counter reports 0.2775/0.3041 ms. NVIDIA GeForce RTX 3080,
NVIDIA 610.57.04, Vulkan. Record: oak-native-timing.json.
The normal native protocol is unchanged: eight conditioning, eight warmup,
120 measured frames, whole-tree hero pose.

| Browser oak orbit | Wall p50, ms | Wall p95, ms | Wall max, ms | Frames | Measurement verdict |
|---|---:|---:|---:|---:|---|
| fn14 | 10.00 | 10.10 | 10.20 | 999 | valid |
| fn27 | 10.00 | 10.10 | 10.20 | 999 | valid |
| fn26 session 4 | 10.00 | 10.10 | 10.10 | 999 | valid |
| fn26 session 5 | **10.00** | **10.10** | **50.00** | **992** | valid |

Browser GPU total p50/p95 is **3.9360/4.5773 ms**, versus 3.6457/5.0596 ms
previously. Selection is 0.1085/0.1116 ms. Wall p95 is below 16.7 ms, but
the 50 ms worst frame exceeds the prior 33 ms hitch guard. Its cause is not
established. Record: oak-browser-orbit.json, including Chromium version,
flags, WebGPU details and 0.1 ms wall-clock quantization. The existing isolated
canvas ten-second orbit used the owner's localhost:5175 server after the native
process exited. The server remains running.

Previous final records are preserved as session4-final-native-timing.json and
session4-final-browser-orbit.json. **No implementation change or weaker filter
follows these failed R5 limits.**

## Gates, references and deviations

All five required gates exit zero for the final code:

- logs/session5-final-gate-fmt.log
- logs/session5-final-gate-clippy.log
- logs/session5-final-gate-rust.log
- logs/session5-verified-gate-npm.log: explicit wasm build and npm test; 66 tests pass
- logs/session5-final-gate-typecheck.log

npm run render:build was run after shader changes; the final explicit rebuild
is logs/session5-profile-render-build.log, also repeated by the final npm gates.
The temporary Rust capture example was removed before final gates. No code
in production changed after final capture 3. No test, assertion, identity pin or lint was
weakened. Authored changed Rust/shader/test files remain under 400 lines.
The pre-existing generated preset catalogue remains a value-data exception:
878 lines, formerly 872; splitting its generator would broaden this correction.

The extra intrinsic edge smoothing and four subpixel lighting evaluations are
departures from a minimal amplitude-only filter, required here to meet the
unchanged resolution tolerance while retaining visible relief. Their visual
rounding and measured R5 failure are reported, not hidden. Native performance
and the final spruce still remain unresolved; this report does not close the spec.

References are owner-supplied local files, never redistributed:
OWNER-WHITE-OAK, OWNER-BLACK-OAK and OWNER-NORWAY-SPRUCE under .refs/fn26/.
Their filenames, provenance and SHA-256 values are in
session5-reference-metadata.json. The Norway-spruce SHA-256 is
e5b82a73eb7094d68f8ab407358b11de6780f7eb95165c528245960dc77d9691.
The earlier O-BARE/O-LEAF/S-BRANCH/S-NEEDLE sources and checksums remain in
references.json and the archived session4-REPORT.md. S-BRANCH is not a bark
close-up; the owner photograph is the appropriate spruce-scale reference.

## Owner verdict

| Species | Scale | Reference | Owner verdict |
|---|---|---|---|
| Oregon white oak | trunk | OWNER-WHITE-OAK; O-BARE | |
| Oregon white oak | branch/socket | OWNER-WHITE-OAK; O-BARE | |
| Oregon white oak | leaf | O-LEAF | |
| Norway spruce | trunk | OWNER-NORWAY-SPRUCE; S-BRANCH | |
| Norway spruce | branch/socket | S-BRANCH | |
| Norway spruce | needle | S-NEEDLE | |
