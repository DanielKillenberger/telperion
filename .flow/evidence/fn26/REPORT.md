# FN26: bark relief, leaf veins and transmission

2026-09-13, session 4. The owner rejected the previous bark as crackle. This
revision replaces it with a field of vertical ridges, shouldered furrows and
flat scales with lifted lower edges, guided by the supplied white-oak photo.
The final still remains cleaner and more regular than the photograph; no
visual acceptance is inferred. R4 stays open and all owner verdict slots are blank.
No git writes, .flow writes, agents or other worktree edits were made.

## Protocol

Seed 7, 1600 × 1000, four samples. The existing bare view, scene and cameras
are unchanged across all four oak trunk iterations. The fourth trunk is the
final image, followed by one oak branch capture and one spruce trunk capture.
Every capture inspects one new image and at most two references, never more
than three images. No other detail still is rendered. Exact cameras and scene
rows are in logs/session4-capture-*.log. The temporary Rust driver is preserved
as session4-evidence.rs and removed from the worktree before final gates.

The native clock uses the unchanged headless whole-tree pose, eight conditioning,
eight warmup and 120 measured frames. Its incidental hero is outside the detail
set and is not inspected. The browser uses the existing isolated-canvas ten-second
orbit protocol on the owner's server at port 5175. That server is neither stopped
nor replaced. Timing runs are sequential, at the inherited lowered CPU priority,
after this session’s builds and tests. A contended record is not accepted.

## References

The owner-supplied references are local, ignored, never committed and never
redistributed. OWNER-WHITE-OAK is the primary bark target; OWNER-BLACK-OAK
shows the deeper, longer-furrow end that the same numeric row should reach.
The rejected harness still records the starting defect. Source: owner supplied;
no external attribution or redistribution rights are inferred.

| ID | Local path | SHA-256 |
|---|---|---|
| OWNER-WHITE-OAK | .refs/fn26/owner-white-oak-bark.png | 7dc7bab59113d8db7922dd83630efd070e850af5f42c45b52790f0611584153b |
| OWNER-BLACK-OAK | .refs/fn26/owner-black-oak-bark.png | 211845d998fda31359fec5032ddc7206a23c992e4e4512a86d260d1b0001d069 |
| REJECTED | .refs/fn26/harness-trunk-as-judged.png | feab46aff84c51da810713e7cdfad73364813658bb183eb05bf3f8633168509b |

The earlier catalogued references remain local. Copyright Oregon State
University Landscape Plants; Patrick Breen page contact, individual photographer
unspecified. Their hashes match the frozen fn19 manifest; no reference image is
included in the handover.

| ID | Local path | Source | SHA-256 |
|---|---|---|---|
| O-BARE | .refs/fn26/quga999A.jpg | https://landscapeplants.oregonstate.edu/sites/plantid7/files/plantimage/quga999A.jpg | 18c79a6dac6d848ec707397d2a87109d60a3200f11424fbd663fd9f81665805c |
| O-LEAF | .refs/fn26/quga28.jpg | https://landscapeplants.oregonstate.edu/sites/plantid7/files/plantimage/quga28.jpg | 9354b9a366ca129d069331f887ba844460fdebe9e5931c3f576862fc11be83fe |
| S-BRANCH | .refs/fn26/piab428B.jpg | https://landscapeplants.oregonstate.edu/sites/plantid7/files/plantimage/piab428B.jpg | d3792f4dade2389e47a6f6be326bfbedd3e33a518493e4c719893cc3d1cdf800 |
| S-NEEDLE | .refs/fn26/piab347A_0.jpg | https://landscapeplants.oregonstate.edu/sites/plantid7/files/plantimage/piab347A_0.jpg | 47e6c9dd3175e6a5deb0fad36da9b034a627f1ab1cb50f411fa1feee7ebd37e6 |


O-BARE is a whole-tree reference, not a bark close-up. S-BRANCH shows an
attached needled branch, not trunk bark or a bare fork; it cannot establish
spruce bark calibration. The new owner photographs supply oak bark detail.

## Stills

| Species | Scale | Still | Reference | Evidence session |
|---|---|---|---|---|
| Oregon white oak | trunk | [oak-trunk.png](stills/oak-trunk.png) | OWNER-WHITE-OAK; O-BARE for form | 4, capture 4 |
| Oregon white oak | branch/socket | [oak-branch.png](stills/oak-branch.png) | OWNER-WHITE-OAK; O-BARE for form | 4, capture 5 |
| Oregon white oak | leaf, frontlit | [oak-leaf-frontlit.png](stills/oak-leaf-frontlit.png) | O-LEAF | 2, unchanged |
| Oregon white oak | leaf, backlit | [oak-leaf-backlit.png](stills/oak-leaf-backlit.png) | O-LEAF | 2, unchanged |
| Norway spruce | trunk | [spruce-trunk.png](stills/spruce-trunk.png) | S-BRANCH, with the limitation above | 4, capture 6 |
| Norway spruce | branch/socket | [spruce-branch.png](stills/spruce-branch.png) | S-BRANCH | 2, unchanged |
| Norway spruce | needle, frontlit | [spruce-needle-frontlit.png](stills/spruce-needle-frontlit.png) | S-NEEDLE | 2, unchanged |
| Norway spruce | needle, backlit | [spruce-needle-backlit.png](stills/spruce-needle-backlit.png) | S-NEEDLE | 2, unchanged |

The three superseded trunk diagnostics are under discarded/session4-capture{1,2,3}-oak-trunk.png.
The final oak has pale flat faces, varied dark lower edges and irregular vertical
columns. Its surface still looks more orderly and its faces smoother than the
white-oak photo. The branch exposes the phase/direction mismatch at independent
fork axes, and weaker relief on smaller runs. No geometry is hidden.

The spruce trunk has vertical ridges broken into scales, but foreground twigs
heavily obstruct it. Its row is unchanged. The earlier spruce socket framing
request remains unresolved. The retained spruce branch is prior-shader evidence;
this session authorized only its trunk recapture, so that branch does not verify
the revised bark. All five retained still hashes are checked unchanged in stills.json.
Leaf lighting observations remain: oak frontlit veins and margin tone, yellow-green
backlit transmission that washes out some vein contrast; plain spruce needles,
green frontlit and nearly opaque backlit. Existing leaf geometry is unchanged.

## Material and field

| Field | Oak | Spruce | Other three presets |
|---|---:|---:|---:|
| Ridge scale, m | 0.032 (was 0.04) | 0.025 | 0 |
| Plate scale, m | 0.055 (was 0.18) | 0.09 | 0 |
| Bark RGB, linear | 0.225, 0.218, 0.198 | 0.147, 0.078, 0.045 | unchanged |
| Roughness detail | 0.12 | 0.16 | 0 |
| Vein pairs | 7 | 8 | 8 |
| Vein contrast | 0.45 | 0 | 0 |
| Transmission strength | 0.55 | 0.01 | 0 |
| Transmission tint | 0.24, 0.52, 0.07 | 0.12, 0.24, 0.08 | 0.3, 0.6, 0.1 |
| Optical thickness | 0.65 | 3.5 | 1 |

The circle embedding keeps a radius × angle metric in metres without integer
ridge counts or an angular seam. Jittered sites partition the circumference into
columns; independent two-coordinate noise drifts their outlines along the run.
A distance-based shoulder gives each furrow finite width and each ridge a flat
face. Uneven, independently staggered axial intervals split the columns into
scales. Their asymmetric height profile rises at the lower edge and settles
onto a face; amplitude varies per scale. Fine flakes occur only on faces and
fade separately when the pixel footprint cannot resolve them.

Scale spacing is derived from the row, bounded to 1.5–2 ridge widths. The
plate/ridge ratio also controls furrow depth, shoulder width and axial persistence.
At the shallow oak setting, furrow strength varies along the run, leaving mostly
short grooves and occasional longer fissures. At larger ratios, columns persist
longer, their shoulders broaden, and high-frequency wandering decreases. For
example, 0.035/0.21 m gives nominal 7 cm scales, 56 cm column persistence and
about 1 cm furrow floors before noise variation. This end is structurally derived,
not a separately photographed or visually accepted black-oak preset.

The existing diameter fade from two to five ridge widths remains. An additional
smooth girth factor grows from 0.3 to 1 as radius spans 2.5–20 ridge widths, so
mature stems have stronger relief than mature branches. The original radius
storage buffer, interpolant and upload limits remain unchanged. The new field
only perturbs shading normals; shadow receivers still use the original normal.
All darkening comes from those normals under the existing sun and sky, without
new micro-shadowing or geometric overhangs.

Veins and transmission are unchanged: roundness 1 suppresses veins and margin
tone; transmission is tint × strength × exp(-thickness) × shadow visibility ×
backface sunlight × squared alignment toward the eye. Every level shares the
same coordinates and filtering. No subsurface-scattering model is introduced.

## R5 numbers

| Native whole view | Total p50, ms | Total p95, ms | Verdict |
|---|---:|---:|---|
| fn14 | 4.949 | 5.412 | valid |
| fn27 | 3.4243 | 3.7484 | valid |
| fn26 session 2 | 3.5113 | 3.8223 | valid |
| fn26 session 3 | 3.5323 | 3.8410 | valid |
| fn26 session 4 | **3.5656** | **3.8853** | valid |

Native total p50 is 0.0333 ms (0.94%) above session 3, leaving **0.2344 ms**
inside the 3.8 ms bound. Vegetation p50/p95 3.1928/3.4847 ms;
selection 0.1029/0.1044 ms. The unchanged shadow counter reports
0.2703/0.2970 ms. Record: oak-native-timing.json;
log: logs/session4-native-timing.log. Valid first attempt. NVIDIA GeForce RTX 3080,
NVIDIA 610.57.04, Vulkan. The incidental session4-native-hero.png is not inspected.

| Browser oak orbit | Wall p50, ms | Wall p95, ms | Wall max, ms | Frames | Verdict |
|---|---:|---:|---:|---:|---|
| fn14 | 10.00 | 10.10 | 10.20 | 999 | valid |
| fn27 | 10.00 | 10.10 | 10.20 | 999 | valid |
| fn26 session 2 | 10.00 | 10.10 | 10.20 | 999 | valid |
| fn26 session 3 | 10.00 | 10.10 | 10.40 | 999 | valid |
| fn26 session 4 | **10.00** | **10.10** | **10.10** | 999 | valid |

Browser GPU total p50/p95 **3.6457/5.0596 ms**, versus
3.5955/3.9598 ms previously. The GPU p95 increased by 1.0998 ms in this run;
no zero-cost or causal claim is made from these single sessions. Wall p95 stays
10.10 ms, below 16.7 ms, and max 10.10 ms is below 33 ms. Selection p50/p95
0.1085/0.1966 ms. Record: oak-browser-orbit.json;
log: logs/session4-browser-orbit-retry.log. Chromium 153.0.8010.12; exact flags
and WebGPU information are in the record. Wall clock resolution is 0.1 ms.

The first browser invocation failed before measurement because it targeted
127.0.0.1 while the owner's server listened on IPv6 localhost. The unchanged
protocol then ran at http://localhost:5175 and returned valid. No contended
verdict occurred or was accepted. Native and browser ran sequentially after
this session's tests and builds; GPU utilization was 0% at preflight. Another
session's CPU-only species tests were active. No process or priority was changed.
Previous records: session3-final-native-timing.json and session3-final-browser-orbit.json.
No visual tuning or implementation change follows the final clocks.

## Gates and pins

All five required gates pass: cargo fmt --all -- --check; cargo clippy
--workspace --all-targets -- -D warnings; cargo test --release --workspace;
npm run wasm:build && npm test (66 tests); npm run typecheck. Final logs are
logs/session4-gate-{fmt,clippy,rust,npm,typecheck}.log. The generated browser
preset table contains only the matching oak value changes.

Twenty release look iterations with --test-threads=3 --nocapture pass: 60 test
executions, **zero changed channels in every repeat draw, worst 0/255**, equal
to session 3. Evidence: logs/session4-look-loop.log. The new physical
field test failed against the rejected implementation in logs/session4-structure-red.log
(trunk/branch deviation ratio 0.9884). It passes the final shader in
logs/session4-field-refinement.log (3.1962, across/along slope ratio 1.6332).
The unchanged wrap test passes with a height difference of 8.8316e-8 m.
Intermediate failed wrap checks and their correction are recorded in child-notes.md.
The existing wood-byte test retains equality: both hashes 9238220531640137937.
The identity literals, vertex layout and exact-equality redraw assertion are untouched.

## Deviations

The rejected rounded cellular relief is replaced by anisotropic columns with
finite-width shouldered furrows, flat scales, lifted lower edges and finer flakes
on faces. Three scales derive from the two row lengths; girth continues to
strengthen relief beyond the existing young-wood fade. Circumferential character,
outline, phase and individual scale amplitude vary. The oak row is shorter-scaled
and less brown to follow the owner's white-oak photograph; spruce and the other
three preset rows are unchanged.

There is no departure from the spec's boundaries: no image textures, new light,
shadow change, vertex layout change or mesh displacement. Normal perturbation
cannot reproduce actual lifted silhouettes or micro-occlusion. The final white-oak
field remains cleaner and more regular than the photo. The deep-furrow end has
not received a separate visual capture or owner acceptance. Independent fork axes
still change pattern phase/direction; the spruce trunk is obstructed, and its
older branch still is retained under this session's capture limits. These limits
remain exposed for R4; no acceptance or spec closure is claimed.

The generated preset catalogue retains its pre-existing 872-line format; only
five oak values change there. Authored Rust/shader files changed here remain
below 400 lines. Reformatting or splitting the catalogue generator would broaden
this bark correction, so this existing generated-data size exception is retained.

## Owner verdict

| Species | Scale | Reference | Owner verdict |
|---|---|---|---|
| Oregon white oak | trunk | OWNER-WHITE-OAK; O-BARE | |
| Oregon white oak | branch/socket | OWNER-WHITE-OAK; O-BARE | |
| Oregon white oak | leaf | O-LEAF | |
| Norway spruce | trunk | S-BRANCH | |
| Norway spruce | branch/socket | S-BRANCH | |
| Norway spruce | needle | S-NEEDLE | |
