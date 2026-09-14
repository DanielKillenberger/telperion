# FN29: colour, cavity and crown occlusion

2026-09-14, session 3. **NEEDS_HUMAN: R6 remains over budget.** Round 5's
valid native run is **3.9823 ms total p50**, **0.1823 ms above 3.8 ms**.
The centred map and measured offset meet both crop targets. Owner judgments
remain blank. The previous 4.0005 ms record is preserved as native-round4.json;
no bound or timing protocol changed. The owner authorised this round and
extended the commit/capture budget before work began.

The distance and socket regressions have recorded GPU red/green results.
Thirteen distinct device tests ran in the focused suites, with 27 executions
including the red diagnostics and **zero adapter skips**. The host subsequently
ran all gates at 08ece33; only the anatomy-name conformance test failed. Round 2
rewords the shared-noise comment and records the complete workspace pass below.
The prior sandbox run's adapter skips do not count as GPU validation.

## What changed

Nineteen numeric material additions now roundtrip through validation, JSON,
blending and generated browser metadata. Ordinary keeps MaterialParams::default()
exactly, with inert additions and no bark relief. Parsing no longer resets the
material independently of that default. The Two Trees start from the same
default and retain their base colours with explicit detail overrides. Oak and
spruce keep the fn-26 base colours and relief parameters.

Round 5 centres colour on the field's constant-height mean h0. Let e be the
field's existing elongated factor, D=mix(.095,.20,e), M its maturity, G its girth,
and k its cavity strength. The shared range is derived from the field constants:

```
ridge_mean = .7 * D * .95 * mix(.575, 1, e) * furrow
face_bands = .05 * .7 * .89 + .012 * .89 * .75 * .5
h0 = ridgeScale * M * G * (ridge_mean + face_bands)
s  = ridgeScale * M * G * (D * 1.35 * furrow + face_bands)
t = clamp((height - h0) / max(s, 1e-10), -1, 1)
crest = max(t, 0) * M; fissure = max(-t, 0) * M
A = base * (1 - k*fissure) + fissureOffset * fissureStrength * fissure
    + crestOffset * crestStrength * crest
```

The face remainder is 1-fissure-crest. At h0 both detail weights vanish and
albedo equals the base. The field's far shortcut returns this same h0 exactly.
The 1.35 scale is the field's peak character-depth multiplier.
This is piecewise affine, not globally affine: splitting at zero introduces a
kink. It removes the old tint-times-cavity quadratic but cannot claim exact
box-average commutation across zero. The unchanged GPU bounds check that limit.
The signed clamp is [-1,1], each weight is [0,M], and final colour is [0,1].
Low-frequency filtered mottle still multiplies albedo; geometric contact and
sky occlusion remain unchanged. Sheen takes the same relief cavity weight.
The 0.8 m oak mottle scale exceeds the roughly 6.5 mm facing footprint at 4x.
Normals, roughness, sheen and tone mapping remain nonlinear.

The GPU exposed double filtering in the previous integration. Each of four
shading cells sampled a height already filtered over the entire pixel, then
integrated those heights over that pixel again. The new nine shared heights
use each half-pixel cell's footprint. All four cells contribute without a fifth
coarse lighting evaluation. Wavelengths shorter than 1.33 pixels widen smoothly
back to the full footprint before the existing one-pixel constant-field shortcut.
The relief field itself, its amplitudes and its physical sampling contracts
remain intact. No texture or species branch was introduced.

Blade mottling uses the stable placement seed and filtered leaf coordinates.
The margin uses the existing blade coordinates and pixel footprint. A front-face
cuticle reflects the shadowed sun; the back retains its own colour. The six
shared noise functions live in common.wgsl, so foliage no longer concatenates
bark.wgsl. A fully rejected noise band returns its exact mean before evaluating
four hashes. The final cost attempt packs the seed into the existing leaf
varying and reconstructs the same hue/brightness mix in the fragment. Zero
reflected-sun factors skip specular work. Neither change recovered R6.

### The socket question

The initial implementation had relief cavity and ground contact only. Root
distance in surface.x cannot identify a socket start. A zero-relief fork test
confirmed the missing term on the actual seed-7 oak joint at y=4 m.

The added contact term measures minimum principal curvature from geometric
normal and position derivatives. It clamps -radius * minimumCurvature to [0,1].
Convex tubes have no concave direction; joined necks can. It shares the existing
cavity strength with the ground factor, taking the larger geometric contact.
No vertex, mesh buffer, vertex layout, light or shadow map changed. The term
measures concavity in the existing joined surface. It cannot identify nearby
contact between disconnected crossing limbs; correcting those intersections
and the fork anatomy remains in fn-4/fn-20. Whether this local socket shade is
sufficient for R5 remains the owner's judgment.

| Socket crop, relief disabled | Plain luminance | Cavity luminance |
|---|---:|---:|
| Before geometric term, sky only | 1404259.702798 | 1404259.702798 |
| After, sky only | 1404259.702798 | 1393880.189798 |
| After, sun and sky | 2534895.467599 | 2523882.676799 |

The regression requires a loss above 0.1% of the plain crop sum, then exact
redraw equality. Logs are socket-red.log and gpu-cell-and-contact.log.

### Crown depth

Sky occlusion removes only the sky hemisphere. Ground bounce remains. For a
leaf at depth d, ambient becomes (sky*(1-occlusion*d) + ground)*(1-interior*d).
Wood uses fragment depth in the same placement-bounds ellipsoid. Leaf view
has no crown interior; Clay returns before material shading.

The failed 400-node Ordinary fixture had zero leaves and therefore no crown
bounds. The revised fixture uses the complete tree with 28,626 placements,
including 16,657 at depth greater than 0.2. A 512x512 camera aims at the crown
centre. Independent material marker frames select visible wood and leaves in
a central crown disk. Each mask must lose over 1% of its plain luminance.

| View / mask | Pixels | Plain sum | Occluded sum |
|---|---:|---:|---:|
| Whole wood | 56405 | 1747566.411801 | 1254977.857400 |
| Whole leaves | 27490 | 944863.106600 | 774029.772600 |
| Bare wood | 64931 | 2167263.246401 | 1595254.629799 |
| Whole full frame | 262144 | 20294859.968772 | 19473157.122773 |
| Bare full frame | 262144 | 24952185.207966 | 24279061.098566 |
| Leaf full frame | 262144 | 48471873.267058 | 48471873.267058 |
| Clay full frame | 262144 | 46549873.296870 | 46549873.296870 |

Leaf and Clay pass byte equality. These values also hold after the varying
packing change in logs/packed-varying-tests.log. The later zero-highlight
shortcuts were exercised by the final native render, but the focused image
suite was not rerun after those shortcuts before the R6 stop.

## Wire rows and calibration

RGB entries below are linear offsets. Metre scales belong to wood; blade scale
is noise cells per blade length. All nineteen additions default to zero. Zero
mottle scale or strength disables mottling; zero margin width disables its tint.
README.md documents these rows. Range refusals name the offending material field.

| Wire names | Range | Ordinary | Oak | Spruce | Telperion | Laurelin |
|---|---|---|---|---|---|---|
| fissureRed/Green/Blue | each -1..1 | 0,0,0 | -.1,-.05,.005 | -.035,-.03,-.02 | 0,0,0 | 0,0,0 |
| fissureStrength | 0..1 | 0 | .65 | .4 | .12 | .12 |
| crestRed/Green/Blue | each -1..1 | 0,0,0 | .1,.085,.055 | .05,.025,.01 | 0,0,0 | 0,0,0 |
| crestStrength | 0..1 | 0 | .5 | .3 | .1 | .1 |
| barkMottleScale | 0..8 m | 0 | .8 | .4 | 0 | 0 |
| barkMottleStrength | 0..1 | 0 | .2 | .2 | .05 | .05 |
| cavityStrength | 0..1 | 0 | .6 | .6 | .2 | .2 |
| bladeMottleScale | 0..32 | 0 | 6 | 8 | 0 | 0 |
| bladeMottleStrength | 0..1 | 0 | .15 | 0 | .04 | .04 |
| marginWidth | 0..0.5 | 0 | .08 | 0 | 0 | 0 |
| marginRed/Green/Blue | each -1..1 | 0,0,0 | .025,.035,.006 | .01,.015,.002 | 0,0,0 | 0,0,0 |
| cuticleGloss | 0..1 | 0 | .35 | .05 | .25 | .3 |
| skyOcclusionStrength | 0..1 | 0 | .5 | .6 | .2 | .2 |

The rows retain the approved oak fissure offsets, pale crest offsets,
low-frequency variation and a blade highlight. Spruce has shallower offsets,
zero blade mottle strength and margin width, and low cuticle gloss. All seven
references were viewed once in this session. No preset was altered during the
GPU filtering or timing work. These values are supplied for the owner's
comparison, without a claim of realistic appearance or acceptance.

## Resolution and redraw evidence

Comparisons retain the original masks and delivered sRGB channels. An exact
2x2 reduction of 1600x1000 is compared with native 800x500. Limits stay mean
<=3/255 and p95<=12/255. No image processing, mask relaxation or tolerance change.

| Test | fn-26 final mean / p95 | Incoming fn-29 mean / p95 | Corrected mean / p95 |
|---|---:|---:|---:|
| Oak near | 1.293709 / 4.75 | 1.685466 / 4.25 | .925530 / 3.00 |
| Oak grazing | 1.407319 / 5.25 | 2.172993 / 5.25 | 1.367392 / 4.25 |
| Spruce grazing | .842843 / 2.75 | .691003 / 2.00 | .670043 / 1.75 |
| Oak 2x | 1.375550 / 4.75 | 1.544733 / 4.75 | 1.958775 / 4.75 |
| Oak 4x | 2.992883 / 8.50 | 5.562083 / 13.75 | 1.908467 / 5.25 |

Incoming here means the affine-only checkpoint tested on this device. Before
that affine change, the host measured 1.320017/4.25 at 2x and 3.826100/9.00 at
4x. Four full-footprint cells alone reduced 4x mean to 4.270750, still failing;
cell-sized footprints reduced it to 1.919700 before the contact/fade cleanup.
The corrected column above is commit 4c5bf29, preceding the final cost attempt.
Round 5 final means/p95 are 2.247425/6.00 at 2x, 2.443950/6.50 at 4x,
1.123815/4.00 near, 1.788092/5.75 oak grazing and 1.279400/4.25 spruce grazing.
All pass mean <=3 and p95 <=12; redraws remain exact. No pin was moved.

Near and both grazing redraws are byte-identical, worst 0/255. Look's repeat
check moves zero channels. The GPU bark-detail fixture's wood hash is
16822752187927960657 with relief off and on. Physical near/far height deviations
remain approximately .000913929/.000836467 m. Logs retain each red and green
measurement; device-tests.json enumerates executions and the zero skip count.

## Native and browser clocks

The earlier and round-5 native runs use the exact command in checks.json. Each has one
initial hero render, eight conditioning frames, eight warmup and 120 measured
frames at 1600x1000, seed 7, Whole view and the oak row fully enabled. The hero
PNG is incidental and was never inspected. No test, browser or GPU work owned
by this session overlapped these runs. Earlier pre-run utilization was 0%;
round 5 queries were 9% then 5% immediately before measurement. No owner
process, display setting or priority was changed.

| Native total | p50 ms | p95 ms | Status |
|---|---:|---:|---|
| fn-14 | 4.9487 | 5.4116 | historical valid |
| fn-27 | 3.4243 | 3.7484 | historical valid |
| fn-26 final | 3.6879 | 4.0284 | historical valid |
| fn-29 before varying packing | 3.9549 | 4.4964 | valid; over 3.8 |
| fn-29 through round 4 | 4.0005 | 4.3791 | valid; preserved in native-round4.json |
| fn-29 round 5 | 3.9823 | 4.5814 | valid; over 3.8 by .1823 |

| Final native pass | p50 ms | p95 ms |
|---|---:|---:|
| Vegetation | 3.6024 | 4.1810 |
| Selection | .1032 | .1044 |
| Shadow | .2724 | .3011 |
| Total, ranked per-frame sums | 3.9823 | 4.5814 |

Round 5 p50 is .2944 ms above fn-26 and .0182 ms below round 4; R6 still fails.
This single comparison does not establish a speed improvement. Hardware is NVIDIA
GeForce RTX 3080, NVIDIA 610.57.04, Vulkan, four samples per pixel. Counts remain
90,760 wood caster triangles and 217,328 foliage caster instances.

| Browser orbit | wall p50 / p95 / max ms | GPU total p50 / p95 ms | Frames | Status |
|---|---|---|---:|---|
| fn-14 | 10 / 10.1 / 10.2 | 5.0109 / 5.4561 | 999 | historical valid |
| fn-27 | 10 / 10.1 / 10.2 | 3.5205 / 4.2045 | 999 | historical valid |
| fn-26 final | 16.7 / 16.7 / 16.8 | 3.7379 / 3.9265 | 601 | historical valid |
| fn-29 round 3 | 10 / 10.1 / 10.1 | 4.1009 / 4.4567 | 999 | valid; wall bounds pass |

The browser bounds remain wall p95 <=16.7 ms and max <=33 ms with a valid
session. Round 3 passes both wall limits with a valid record in
oak-browser-orbit.json. The ten-second run produced 999 wall frames, compared
with fn-26's 601; display settings and owner processes were untouched. The
GPU totals cover 120 measured frames after eight conditioning and eight warmup.
The supplied /tmp/fn29-orbit.mjs ran one isolated 1600x1000 canvas against this
worktree's Vite server on port 5179 after npm run render:build (exit 0). The first
launch failed before Chromium because the hard-coded Playwright path was absent;
only that scratch import was changed to the repository-resolved parent install.
The retry exited 0. No other GPU work owned by this session overlapped it.
Chromium closed and the owned Vite server stopped; port 5175 was not used.
The earlier explicit shader compilation check passed at 4c5bf29.

## Stills

**Capture 3 completed under the owner's round-5 extension.** The host traced
the full-face darkening to height/(.35*ridgeScale), which kept fissure weights
around .6–1 over the trunk. The centred map above addresses that cause. The
oak offset is (-.1,-.05,.005), strength .65: original red/blue, with green
adjusted from -.075 after measurement. No spruce row changed. README quotes no changed number and remains unchanged.

The exact ImageMagick centre-400x400, resize-to-one-pixel integer RGB protocol
gives the following. Reference RGB is host-supplied; other rows were measured
here. Round 4 is read from e9c4b7d, without recapturing or viewing it.

| Crop | Oak RGB | Spruce RGB | Spruce R/B |
|---|---|---|---:|
| Reference | 120,122,117 | not supplied | not supplied |
| fn-26 | 158,156,150 | 138,96,66 | 2.0909 |
| Round 4 | 98,99,104 | 80,44,26 | 3.0769 |
| Centred map, round-4 offset | 147,146,142 | 139,97,65 | 2.1385 |
| Nominal scale, original offset | 142,142,142 | 139,97,65 | 2.1385 |
| Peak scale, original offset | 149,148,146 | 139,97,66 | 2.1061 |
| Round 5 final | 149,149,146 | 139,97,66 | 2.1061 |

Round 5 meets G>=R>=B and mean 148 in [120,158]; spruce is red-top with R/B
in [2.1,2.6]. These are numerical acceptance results, not an owner verdict.
round5-crops.json retains all map/offset measurements. The nominal scale failed
the 4x mean bound at 3.166967/255; the field's peak depth scale fixes that
without changing the field itself or the 3.0/255 bound. Only four images from the first original-offset candidate were inspected:
oak-trunk, spruce-trunk, oak-branch and oak-leaf-frontlit,
for blank/black frames or wrong cameras; no reference was viewed again.

In the same resolved oak crop, averaged blend weights are face 71.2400%,
fissure 22.5895%, crest 6.1705%. Pixel classes differ: 67.4575% have fissure-side
samples only, 27.0331% crest-side only, 5.5094% mixed cells, and 0% exact face.
There is no finite dead band; face means the untinted remainder.
A temporary shader probe emitted the three weights before tone mapping, using
the production four-cell integration. Its sRGB output was decoded before
averaging and normalizing the sum (8-bit quantization). round5-face-weights.json
and the archived probe
patch make the measurement explicit. The probe was removed before GPU tests.

All eight stills were overwritten with the archived driver, then the temporary
example was deleted. Cameras and scene rows match capture 2 exactly. stills.json
records session 3, capture 3, exact cameras/scene rows and refreshed SHA-256s.
The owner-verdict cells remain blank; capture-status.json records the extension.

All captures use seed 7, 1600x1000, Level::Chosen. Wood uses the default scene.
Leaf frontlit/backlit uses sun azimuth 0/180 and elevation 10, with every other
scene value default. Each capture is paired below with its fn-26 counterpart.

| fn-29 still | fn-26 counterpart | Catalogued references |
|---|---|---|
| [oak-trunk.png](stills/oak-trunk.png) | [fn-26](../fn26/stills/oak-trunk.png) | O-BARE, OWNER-WHITE-OAK, OWNER-BLACK-OAK |
| [oak-branch.png](stills/oak-branch.png) | [fn-26](../fn26/stills/oak-branch.png) | O-BARE, OWNER-WHITE-OAK |
| [oak-leaf-frontlit.png](stills/oak-leaf-frontlit.png) | [fn-26](../fn26/stills/oak-leaf-frontlit.png) | O-LEAF |
| [oak-leaf-backlit.png](stills/oak-leaf-backlit.png) | [fn-26](../fn26/stills/oak-leaf-backlit.png) | O-LEAF |
| [spruce-trunk.png](stills/spruce-trunk.png) | [fn-26](../fn26/stills/spruce-trunk.png) | OWNER-NORWAY-SPRUCE |
| [spruce-branch.png](stills/spruce-branch.png) | [fn-26](../fn26/stills/spruce-branch.png) | S-BRANCH, OWNER-NORWAY-SPRUCE |
| [spruce-needle-frontlit.png](stills/spruce-needle-frontlit.png) | [fn-26](../fn26/stills/spruce-needle-frontlit.png) | S-NEEDLE |
| [spruce-needle-backlit.png](stills/spruce-needle-backlit.png) | [fn-26](../fn26/stills/spruce-needle-backlit.png) | S-NEEDLE |

The pinned oak eye is (1.7307636095778745,2.2967023330704928,-1.7307636095778745),
target (0,2,0), FOV 38, near .01, far 1000. Spruce scales that eye-target unit
direction to 1.8 m about (0,.65,0). Leaf cameras use hero_pose of the current
leaf bounds at aspect 1.6 and GROUND_REACH, once the element is submitted.

| Branch | Node | Radius m | Height m | Eye | Target |
|---|---:|---:|---:|---|---|
| Oak | 8 | .39088976459153435 | 4 | (-1.335209229402595,4.351800788132381,-1.9281673692824055) | (0,4,0) |
| Spruce | 6 | .2091256108761402 | 1.8 | (1.2547536652568412,1.9882130497885262,0) | (0,1.8,0) |

Both branch cameras have FOV 38, near .01, far 1000. They are reconstructed,
since fn-26 did not keep its driver. The driver selects the thickest fork above
1.5 m, takes the perpendicular to its children's mean horizontal direction,
and compares clearance on both sides while excluding the fork's own run.
A vanishing horizontal mean uses the X direction. Current-element leaf hero
poses are another declared difference from any historical element framing.

### Reference provenance

The host re-fetched the four Oregon State University Landscape Plants images
and verified their checksums against fn-26. Patrick Breen is the page contact;
individual photographers are unspecified. Owner images were supplied locally
and copied by the host from the fn-26 worktree. references.json retains source
URLs and the 2026-09-14 fetch date; owner-reference-metadata.json retains local
provenance. All image bytes remain under ignored .refs/fn29/. This session
viewed each once and did not modify or redistribute any reference.

| Reference / filename | SHA-256 |
|---|---|
| O-BARE / quga999A.jpg | 18c79a6dac6d848ec707397d2a87109d60a3200f11424fbd663fd9f81665805c |
| O-LEAF / quga28.jpg | 9354b9a366ca129d069331f887ba844460fdebe9e5931c3f576862fc11be83fe |
| S-BRANCH / piab428B.jpg | d3792f4dade2389e47a6f6be326bfbedd3e33a518493e4c719893cc3d1cdf800 |
| S-NEEDLE / piab347A_0.jpg | 47e6c9dd3175e6a5deb0fad36da9b034a627f1ab1cb50f411fa1feee7ebd37e6 |
| OWNER-WHITE-OAK / owner-white-oak-bark.png | 7dc7bab59113d8db7922dd83630efd070e850af5f42c45b52790f0611584153b |
| OWNER-BLACK-OAK / owner-black-oak-bark.png | 211845d998fda31359fec5032ddc7206a23c992e4e4512a86d260d1b0001d069 |
| OWNER-NORWAY-SPRUCE / owner-norway-spruce-bark.png | e5b82a73eb7094d68f8ab407358b11de6780f7eb95165c528245960dc77d9691 |

## Gates and boundaries

| Required command | Exit code | Scope |
|---|---:|---|
| cargo fmt --all -- --check | 0 | local round 5 checkpoint |
| cargo clippy --workspace --all-targets -- -D warnings | 0 | local round 5 checkpoint |
| cargo test --release --workspace | 0 | local round 5 checkpoint; 45 binaries, zero adapter skips |
| cargo test --release --workspace | 101 | host at 08ece33; anatomy word in shared-noise comment |
| cargo test --release --workspace --no-fail-fast | 101 | host at 08ece33; 45 binaries, one failure, zero adapter skips, no SIGSEGV; final shader focused device tests green |
| npm run wasm:build && npm test | 0 | local round 5 checkpoint; 77 passed; regenerated preset mirror |
| npm run typecheck | 0 | local round 5 checkpoint |

Round 5 ran all five gates serially. RUST_TEST_NOCAPTURE=1 exposed adapter
skip messages in the workspace log; there were zero, with no crashes. Core and
sweep passed without changing any pin. No test or tolerance was edited. README
does not quote the offset values and needed no update. checks.json retains the
commands, exit codes and logs for this and the earlier rounds.

Round 4 distance means/p95 were 1.706067/4.50 at 2x and 1.662400/4.50 at 4x,
in /255. Oak near was 0.926440/3.25; oak grazing 1.280511/4.25; spruce grazing
0.670043/1.75. All passed the unchanged bounds and redraw differences were zero.

The focused distance, resolution, socket and material shader command exited 0
at 4c5bf29. The packed-varying blade/crown/look/material-shader command exited 0.
Naga validation and actual device rendering both ran. No GPU test process
crashed in this full-access session. The earlier sandbox SIGSEGV and serial
adapter-skipping run do not establish a device pass. checks.json records exact
commands, exit codes, logs and the shader state each check covered.

The last specular shortcuts were compiled and exercised by the final native
measurement, whose process exited 0 while the numerical R6 bound failed.
The host's full-workspace run at 08ece33 observed the final shader's focused
device tests green with zero adapter skips. Round 2 changes only the rejected-band
comment in common.wgsl; no test, shader expression or timing evidence changed.
The local workspace log contains no skipped messages or crashes. A second run,
RUST_TEST_NOCAPTURE=1 cargo test --release -p telperion-render, exited 0 with
**zero adapter skips** and no crashes; uncaptured output verifies the skip count.
The browser orbit remains the round-3 record and was not rerun. Capture 3
replaces the eight stills. The separate Chromium compilation check was not
rerun. R6 remains over the native bound. No native timing hero was inspected.

The owner extended the commit budget for round 5 on 2026-09-14. This round
records the centred map and captures, then native timing and the final gates.
No spec/task content was edited by this session, no owner verdict was issued, no agent was spawned,
no history was rewritten and nothing was pushed. The temporary build target was
removed; its source remains reproducible evidence rather than a public command.

## Owner verdict

| Species | Scale | Reference | Owner verdict |
|---|---|---|---|
| Oregon white oak | trunk | O-BARE, OWNER-WHITE-OAK, OWNER-BLACK-OAK | |
| Oregon white oak | branch/socket | O-BARE, OWNER-WHITE-OAK | |
| Oregon white oak | leaf frontlit | O-LEAF | |
| Oregon white oak | leaf backlit | O-LEAF | |
| Norway spruce | trunk | OWNER-NORWAY-SPRUCE | |
| Norway spruce | branch/socket | S-BRANCH, OWNER-NORWAY-SPRUCE | |
| Norway spruce | needle frontlit | S-NEEDLE | |
| Norway spruce | needle backlit | S-NEEDLE | |
