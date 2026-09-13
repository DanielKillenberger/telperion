# FN26: bark relief, leaf veins and transmission

2026-09-13. Implementation and evidence for host review. R4 awaits the owner;
no accepting visual verdict is inferred from tests. No git writes or .flow
writes were made by this implementer. The corrective shader work, lit leaf
pairs, deterministic selection, final clocks and all five gates are delivered.
Session 3 also passed twenty parallel look iterations and the full Rust gate
twice. Clear spruce socket framing remains incomplete from session 2. Species-level visual
calibration is not established; the task/spec must remain open for R4.

## Protocol

Seed 7, 1600 × 1000, four samples where the adapter offers them. Timing and
wood stills use the default scene; leaf pairs use the sun directions below.
Native timing uses the unchanged whole-tree hero pose, eight conditioning,
eight warmup and 120 measured frames. Browser timing uses one isolated canvas
and the same ten-second orbit as fn14/fn27, on port 5175. Invalid sessions
carry no accepted number. GPU totals rank each frame's sum, not sums of
independently ranked percentiles.

The corrected branch/trunk stills use an evidence-only Rust driver with the
existing Camera and bare view. The trunk is framed from the sun-facing side;
the fork pose selects a side view of the thickest fork above 1.5 m, closer
than the original stills, using skeleton obstruction checks. All geometry
stays present. Exact cameras and scene rows are printed in the capture logs.
Leaf pairs use the same front-facing camera with the existing sun at azimuth
0° (frontlit) or 180° (backlit), elevation 10°. Its intensity, colour, shadow
rules and every material value stay unchanged between those two views.
The driver is preserved as session2-evidence.rs in this handover and removed
from the worktree before final gates. No public view or camera command is added.

## The references

All four assets were absent from the permitted fn9 reference directory and
were fetched from the manifest URLs. Each SHA-256 matches the frozen fn19
manifest. Local reference only, copyright Oregon State University Landscape
Plants; Patrick Breen page contact, individual photographer unspecified.
No reference image is included in this handover or committed.

| ID | Local path | Source | SHA-256 |
|---|---|---|---|
| O-BARE | .refs/fn26/quga999A.jpg | https://landscapeplants.oregonstate.edu/sites/plantid7/files/plantimage/quga999A.jpg | 18c79a6dac6d848ec707397d2a87109d60a3200f11424fbd663fd9f81665805c |
| O-LEAF | .refs/fn26/quga28.jpg | https://landscapeplants.oregonstate.edu/sites/plantid7/files/plantimage/quga28.jpg | 9354b9a366ca129d069331f887ba844460fdebe9e5931c3f576862fc11be83fe |
| S-BRANCH | .refs/fn26/piab428B.jpg | https://landscapeplants.oregonstate.edu/sites/plantid7/files/plantimage/piab428B.jpg | d3792f4dade2389e47a6f6be326bfbedd3e33a518493e4c719893cc3d1cdf800 |
| S-NEEDLE | .refs/fn26/piab347A_0.jpg | https://landscapeplants.oregonstate.edu/sites/plantid7/files/plantimage/piab347A_0.jpg | 47e6c9dd3175e6a5deb0fad36da9b034a627f1ab1cb50f411fa1feee7ebd37e6 |

O-BARE is a winter whole-tree photograph, not calibrated bark microscopy.
S-BRANCH is an attached needled branch, not a bare fork close-up. These limits
remain exposed when judging trunk/branch detail. O-LEAF shows midrib and
secondary veins; S-NEEDLE shows plain needles and woody pegs.

## The stills

| Species | Scale | Still | Reference |
|---|---|---|---|
| Oregon white oak | trunk | [stills/oak-trunk.png](stills/oak-trunk.png) | O-BARE |
| Oregon white oak | branch/socket | [stills/oak-branch.png](stills/oak-branch.png) | O-BARE |
| Oregon white oak | leaf, frontlit | [stills/oak-leaf-frontlit.png](stills/oak-leaf-frontlit.png) | O-LEAF |
| Oregon white oak | leaf, backlit | [stills/oak-leaf-backlit.png](stills/oak-leaf-backlit.png) | O-LEAF |
| Norway spruce | trunk | [stills/spruce-trunk.png](stills/spruce-trunk.png) | S-BRANCH |
| Norway spruce | branch/socket | [stills/spruce-branch.png](stills/spruce-branch.png) | S-BRANCH |
| Norway spruce | needle, frontlit | [stills/spruce-needle-frontlit.png](stills/spruce-needle-frontlit.png) | S-NEEDLE |
| Norway spruce | needle, backlit | [stills/spruce-needle-backlit.png](stills/spruce-needle-backlit.png) | S-NEEDLE |

Second-session capture ledger and observations are recorded in child-notes.md.
The original dark oak-leaf.png and spruce-needle.png are superseded by these
pairs. The timing hero is a timing artifact, not an owner verdict slot.

Final observations: oak ridges now wander and break into curved plates at
comparable widths on the trunk and mature branches. The field still has broad,
smooth plate interiors and is visibly stylized at this magnification. Mature
collars expose changes in plate phase and direction. Frontlit oak shows veins
and margin tone; its shaded-side backlit view is yellow-green, with vein
contrast largely washed out by the simple additive transmission term. The
existing faceted outline and vein bends remain unchanged.

Spruce has plain needles, green when frontlit and near-black when backlit.
Its young wood is smooth. **The clear spruce socket framing request remains
incomplete:** foreground twigs still cross the closer collar view and the trunk.
The obstruction search found no clear candidate in its sampled poses. No
geometry was removed. A clearer socket still needs a later authorized capture
session; the eight-capture limit is exhausted here.

Eight captures were used in this session: two initial clocks, two diagnostic
oak wood pairs, two final clocks, then four final views per species. Captures
5 and 6 are the final timing records; 7 and 8 supply the eight judged stills.
Four images were inspected per final species capture, two per diagnostic
capture, and none from the clock sessions. Discarded images and superseded
clock records are retained outside the final still set. No session was
contended. The [native hero](stills/oak-timing-hero.png) is uninspected timing
evidence, without an R4 verdict slot.

## The materials

Eight concepts add ten scalar fields because tint has three channels.
Defaults disable relief, roughness variation, vein contrast and transmission.
Ridge/plate size: 0–1 m; roughness detail, vein contrast, strength and each tint
channel: 0–1; vein scale: 0–32 pairs; thickness: 0–8 optical depth.
Each field has named endpoint refusal tests and wire/blend coverage. The panel
renders the generated numeric row generically. The frozen geometry protocol
already excludes the material object and was left unchanged; an added-row
compatibility test checks that stripping the ten additions recovers the older
material document exactly. The sweep's exact inventory includes the additions
as moved preset fields, so its HELD set stays unchanged in the final diff.

| Field | Oak | Spruce | Other three defaults |
|---|---:|---:|---:|
| Ridge scale, m | 0.04 | 0.025 | 0 |
| Plate scale, m | 0.18 | 0.09 | 0 |
| Roughness detail | 0.12 | 0.16 | 0 |
| Vein pairs | 7 | 8 | 8 |
| Vein contrast | 0.45 | 0 | 0 |
| Transmission strength | 0.55 | 0.01 | 0 |
| Transmission tint | 0.24, 0.52, 0.07 | 0.12, 0.24, 0.08 | 0.3, 0.6, 0.1 |
| Optical thickness | 0.65 | 3.5 | 1 |

Bark uses ring radii reconstructed from the existing vertex positions and
fn14's coordinate ring boundaries. A read-only float storage buffer and one
radius interpolant carry those values; the existing vertex layout and all
mesh arrays stay identical. Cap centres remain smooth. Storage size is
checked before upload, including the device's binding limit.

The procedural field embeds the circle with the arc metric radius × angle,
so metre-sized cells continue across the angular wrap without integer ridge
counts. Smooth longitudinal noise drifts phase and amplitude; jittered
nearest-site cross-fissures cut the ridges into plates. Relief vanishes below
a diameter of two ridge widths and reaches full strength at five. Pixel
footprints filter unresolved detail. The original normal still serves the
shadow receiver; no light, shadow map or caster rule changed.

Veins restore the across-coordinate's sign before interpolation, preserving
the midrib on coarse triangles. Both vein and margin tone vanish at roundness
1. The same coordinates, phase and footprint rule apply at every level.
Transmission is tint × strength × exp(-thickness) × shadow visibility ×
backface sunlight × squared alignment toward the eye. It shares the existing
shadow lookup with reflection. This is a thin-blade model, not multi-bounce
subsurface scattering. Spruce's strength and thickness keep it near zero.

## The oak on the clock

| Native whole view | Total p50, ms | Total p95, ms | Verdict |
|---|---:|---:|---|
| fn14 | 4.949 | 5.412 | valid |
| fn27 | 3.4243 | 3.7484 | valid |
| fn26 session 2 | 3.5113 | 3.8223 | valid |
| fn26 session 3 | **3.5323** | **3.8410** | valid |

The fn26 session-3 native clock is valid and passes the 3.8 ms total-p50
bound with 0.2677 ms remaining. It is 0.0210 ms (0.60%) above session 2 and
0.1080 ms above fn27. Vegetation p50/p95 is 3.1508/3.4314 ms; selection
0.1032/0.1052 ms; the unchanged shadow pass reports 0.2724/0.3011 ms.
Stable compaction adds a measured 0.0164 ms to selection p50. This is a small
cost within R5, not evidence of literally zero overhead. Record:
oak-native-timing.json; previous record: session2-final-native-timing.json.
NVIDIA GeForce RTX 3080, NVIDIA 610.57.04, Vulkan. No look reduction or shader
tuning follows this measurement. The native command's incidental hero is
session3-native-hero.png outside the detail-still set; existing stills stay intact.

## The orbit, on the clock

| Browser oak orbit | Wall p50, ms | Wall p95, ms | Wall max, ms | Frames | Verdict |
|---|---:|---:|---:|---:|---|
| fn14 | 10.00 | 10.10 | 10.20 | 999 | valid |
| fn27 | 10.00 | 10.10 | 10.20 | 999 | valid |
| fn26 session 2 | 10.00 | 10.10 | 10.20 | 999 | valid |
| fn26 session 3 | **10.00** | **10.10** | **10.40** | 999 | valid |

The browser session is valid: 999 frames over ten seconds on the 100 Hz
display, comfortably above 60 fps; p95 10.10 ms is below 16.7 ms, and worst
10.40 ms below 33 ms. Browser GPU total p50/p95: 3.5955/3.9598 ms, versus
3.6198/3.9841 ms in session 2. Selection p50/p95: 0.1085/0.1116 ms,
versus 0.0922/0.0932 ms. Record:
oak-browser-orbit.json; the exact Chromium version and hardware flags are in
that record. Wall times use the page's 0.1 ms clock resolution. Native and
browser measurements ran one at a time, after tests and builds stopped.
The first browser attempt closed before returning a report and is not counted;
the unchanged protocol succeeded on retry. No contended verdict was accepted.
Previous browser record: session2-final-browser-orbit.json.

## Gates and pins

All five required gates pass: formatting, clippy with warnings denied, the
release workspace tests, Wasm build plus npm tests (66), and TypeScript.
The original anatomy-neutrality scan caught the initial `blade_detail` name;
it was renamed to `leaf_detail` without changing the gate. Session 3 ran the complete Rust
gate twice consecutively, both successfully, after the twenty-iteration parallel
look loop (60 passing test executions, worst repeat-draw move 0/255).
The five final gate logs carry the session3-gate prefix; the Rust logs are
session3-gate-rust-1.log and session3-gate-rust-2.log. Red/green commands and logs are listed in
child-notes.md. Original fn24 identity literals and frozen .flow evidence are
unchanged. The wood test hashes positions, normals, coordinates and indices
before/after material detail, then requires a visible shading change.

## Deviations

The host found that the previously reported green Rust gate was intermittent:
atomic reservations assigned both within-workgroup slots and between-workgroup
ranges in arrival order. Equal-depth leaf edge samples could select different
leaves on a repeated draw, and the new shading amplified the existing difference
to 18/255. Session 3 reproduced that failure immediately, then took route 1:
GPU selection now compacts each level in ascending placement-index order.
Membership bitsets determine local ranks, a prefix dispatch determines group
offsets, and a scatter dispatch writes the ordered lists. A checked scratch
buffer holds ranks and offsets. The leaf terms, depth values, original placement
array and shadow pass are unchanged. There is no route-2 shading compromise.
The requested zero-cost check found about 16 microseconds of extra selection
time: native total p50 rose by 21 microseconds, while browser wall p50/p95
were unchanged and browser GPU total p50 fell by 24.3 microseconds. Both R5
bounds pass, but literal zero overhead is not claimed.

All twenty parallel runs of the release look binary passed, with exactly zero
changed channels and worst move **0/255** over the loop. The original tolerance
and assertion remain unchanged; an additional exact-equality assertion now
protects determinism. A direct GPU regression checks all sixteen levels,
culling, forced levels, workgroup/prefix boundaries, repeated dispatches and
smaller submissions reusing the same buffers. Logs: session3-look-before.log,
session3-selection-green.log and session3-look-loop.log under logs/.

The eight detail stills are unaffected: bare views do not select foliage, and
single-leaf views bind their fixed identity list. Their shader and geometry
are unchanged, so no detail still is recaptured in session 3.

The fixed 24-ridge angular pattern was replaced by a metre-scale cellular
field. Ridge widths now stay comparable across girths, phase and amplitude
wander along each run, curved cross-fissures split the ridges into plates,
and the scale-derived maturity fade removes relief from young wood. The
initial corrected still showed overly rectangular plates; local phase wander
and rounded ridge tops address that remaining regularity in the final field.

The original unlit single-leaf evidence is superseded by frontlit and backlit
pairs, using the existing sun and camera. The branch driver now frames a
closer side view of the collar, but spruce foreground twigs still obscure it.
One diagnostic capture omitted the
material upload in that temporary driver and was discarded; the omission
never affected the production renderer or its timing sessions.

The field is continuous along each run and around its angular wrap, but it
still does not stitch cells between independent fork axes. Plate phase and
direction can change at mature collars of comparable girth; the child's lower
radius reduces relief where it falls inside the maturity ramp. Existing
socket geometry remains exposed. No mesh, placement or caster is removed to
hide a join. The spruce collar is still crossed by foreground twigs, so that
specific framing request remains incomplete. The session-2 capture limit prevented a
further attempt then; session 3 authorizes recaptures only for a changed look,
so this selection-only correction does not authorize a new framing attempt. Final observations are recorded above and in
the capture ledger.

There is no departure from the spec's boundaries: no new light, shadow
change, vertex layout change, image texture or subsurface-scattering model.
The additional radius storage buffer and single interpolant leave the original
mesh bytes and fn24 pins intact. The temporary evidence driver is removed
from the worktree, with its source and exact poses preserved in the handover.

## Verdicts

R1–R3: implementation and tests described above; visual acceptance remains R4.
R4: awaiting the owner's six scale judgments below; leaf/needle judgments
have frontlit and backlit views. No owner's verdict is supplied or inferred.
R5: final native and browser records are listed above.

The reference limitations, stylized plate interiors and independent-axis
socket mismatch remain for the owner to judge. The spruce needs a clearer
socket still before its framing requirement can be considered delivered. The
spec must remain open until the owner fills accepting verdicts in the six
slots below.

## Owner verdict

| Species | Scale | Reference | Owner verdict |
|---|---|---|---|
| Oregon white oak | trunk | O-BARE | |
| Oregon white oak | branch/socket | O-BARE | |
| Oregon white oak | leaf | O-LEAF | |
| Norway spruce | trunk | S-BRANCH | |
| Norway spruce | branch/socket | S-BRANCH | |
| Norway spruce | needle | S-NEEDLE | |
