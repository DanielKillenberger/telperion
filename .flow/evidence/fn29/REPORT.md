# FN29: colour, cavity and crown occlusion

2026-09-14, session 3. **NEEDS_HUMAN: R6 remains over budget.** The valid
native retry is **4.0005 ms total p50**, **0.2005 ms above 3.8 ms**. The first
valid measurement was 3.9549 ms. Sharing the foliage seed varying and skipping
zero highlights did not recover the budget. The requested R6 stop rule applies.
No bound, protocol or oak relief row was changed. This report records a blocked
implementation, not completion or an owner verdict. In host round 3 the owner
authorized stills and a browser orbit before deciding R6; native evidence holds.

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

Bark fissure and crest weights read the filtered relief height. At maturity m,
crest t is clamp(height / max(0.35 * ridgeScale, 0.0001), 0, m), and fissure f is
m - t. With cavity strength k, base B and strength-weighted tint offsets F and C,
the albedo before mottle and clamping is

```
A = B * (1 - k*f) + F*f + C*t
  = B * (1 - k*m) + F*m + (B*k - F + C)*t
```

This is one affine height-to-albedo map. Multiplying a separately tinted colour
by cavity introduced a quadratic term. Oak's red quadratic coefficient was
0.069. Before round 4, the mature red albedo was 0.025 + 0.25*t, with an
11:1 endpoint ratio; removing the quadratic term alone did not solve the
image comparison. The owner-approved round 4 offset gives 0.064 + 0.211*t. Both ambient and sun multiply this albedo. Sheen takes the
same 1 - k*f cavity weight. Independent geometric contact multiplies the sum.

The crest clamp remains bounded to [0, maturity]. The 0.35-ridgeScale bound
covers the authored oak relief. The final colour clamp remains [0,1]; oak's
albedo endpoints including its 0.8-1.2 mottle multiplier stay inside that range.
The mottle product is not globally affine across pixels, but its 0.8 m scale
is much larger than the roughly 6.5 mm facing footprint at distance 4x.
Lighting normals, roughness, sheen and tone mapping remain nonlinear.

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
| fissureRed/Green/Blue | each -1..1 | 0,0,0 | -.04,-.03,.002 | -.035,-.03,-.02 | 0,0,0 | 0,0,0 |
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

Near and both grazing redraws are byte-identical, worst 0/255. Look's repeat
check moves zero channels. The GPU bark-detail fixture's wood hash is
16822752187927960657 with relief off and on. Physical near/far height deviations
remain approximately .000913929/.000836467 m. Logs retain each red and green
measurement; device-tests.json enumerates executions and the zero skip count.

## Native and browser clocks

Both native runs use the exact headless command in checks.json. Each has one
initial hero render, eight conditioning frames, eight warmup and 120 measured
frames at 1600x1000, seed 7, Whole view and the oak row fully enabled. The hero
PNG is incidental and was never inspected. No test, browser or GPU work owned
by this session overlapped either run. The pre-run GPU query reported 0%
utilization. Owner processes, display settings and priority were untouched.

| Native total | p50 ms | p95 ms | Status |
|---|---:|---:|---|
| fn-14 | 4.9487 | 5.4116 | historical valid |
| fn-27 | 3.4243 | 3.7484 | historical valid |
| fn-26 final | 3.6879 | 4.0284 | historical valid |
| fn-29 before varying packing | 3.9549 | 4.4964 | valid; over 3.8 |
| fn-29 measured final shader | 4.0005 | 4.3791 | valid; over 3.8 by .2005 |

| Final native pass | p50 ms | p95 ms |
|---|---:|---:|
| Vegetation | 3.6045 | 3.9629 |
| Selection | .1032 | .1050 |
| Shadow | .2724 | .3092 |
| Total, ranked per-frame sums | 4.0005 | 4.3791 |

The final total is .3126 ms above fn-26 and .0456 ms above the first fn-29 run.
There is no measured performance improvement to claim. Hardware is NVIDIA
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

**Two of two still-capture rounds used.** The first round used 6d9b325. The
second, final round uses baca4a6 plus the owner-approved oak fissure offset
(-0.04,-0.03,0.002), replacing (-0.1,-0.075,0.005). Strength remains 0.65.
The host traced its slate-colour finding to cavity 0.6 cutting the base
(0.225,0.218,0.198) to 40% before adding the offset. The old endpoint was
(0.025,0.03845,0.08245), with blue over three times red. The host reported
round-3 trunk centre-crop means of (79,89,115), against fn-26's (169,167,160).
The approved row gives (0.064,0.0677,0.0805); no other material value changed.
This is the owner's reason for the change, not an image verdict from this run.

All eight PNGs were overwritten using the unchanged archived driver. Build and
capture exited 0. Every camera and scene row matches the first capture exactly.
Only the oak trunk and branch hashes changed. stills.json records session 3,
capture 2, SHA-256, repo-relative paths, exact cameras, complete scene rows and
fn-26's species/scale/reference fields. The temporary example was removed again;
stills-driver.rs remains the reproduction source. Neither clock was rerun.

In each round only oak-trunk, spruce-trunk, oak-branch and oak-leaf-frontlit were
inspected for blank frames, black frames or wrong cameras. No reference was
viewed again. Owner verdicts remain blank. capture-status.json records both
inspection rounds; the two-round capture budget is now exhausted.

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
| cargo fmt --all -- --check | 0 | local round 4 checkpoint |
| cargo clippy --workspace --all-targets -- -D warnings | 0 | local round 4 checkpoint |
| cargo test --release --workspace | 0 | local round 4 checkpoint; 45 binaries, zero adapter skips |
| cargo test --release --workspace | 101 | host at 08ece33; anatomy word in shared-noise comment |
| cargo test --release --workspace --no-fail-fast | 101 | host at 08ece33; 45 binaries, one failure, zero adapter skips, no SIGSEGV; final shader focused device tests green |
| npm run wasm:build && npm test | 0 | local round 4 checkpoint; 77 passed; regenerated preset mirror |
| npm run typecheck | 0 | local round 4 checkpoint |

Round 4 ran all five gates serially. RUST_TEST_NOCAPTURE=1 exposed adapter
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
The round 3 browser orbit and eight stills are now captured by owner direction;
the separate Chromium compilation check was not rerun. R6 remains over the
native bound and awaits the owner. No image was inspected from either native clock.

Nine checkpoints include the implementation, evidence and host lifecycle
records through round 4. This round uses one checkpoint; the host's final
lifecycle commit reaches the ten-commit cap.
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
