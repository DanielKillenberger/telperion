# Comparative botanical geometry baseline — fn19-v1

The twelve frozen mature oak/spruce cases complete numerical collection and all
84 required visual captures. All 84 views have implementation-side visual
inspection records; identical oak base images reuse an inspected image only after
exact PNG hash verification. 25 original PNG previews and 1,236 verified
artifact records are retained. The
engineering baseline does **not** establish biological superiority: references
are qualitative, matched competitor assets are unavailable, and independent
botanical assessment remains unassessed. R3's independent-assessment portion is
explicitly unresolved. No reviewer was contacted and no assets were purchased.

## Frozen evidence and scope

Generation uses source revision `b5249e16c7e2c10f88f8409294fce38c0ddfb435` with
complete source/tool/native/Wasm content identities. The mature generator and capture tool
were frozen before generation. After mature collection, a key-order validation
defect was corrected for the separate small replay. Its distinct capture-tool
identity is retained; no mature artifact or frozen protocol was rewritten. [Source receipts](final/sources.json) pin
inputs and artifacts; [the protocol](PROTOCOL.md) pins parameters and cameras.
Original fn9 evidence remains unchanged and is historical context, not a
substitute for fn19's six newly frozen seeds.

| Species, six specimens each | Height m | Crown width m | Individual foliage units |
|---|---:|---:|---:|
| Oregon white oak | 17.588–19.633 | 18.454–23.876 | 495,781–585,533 leaves |
| Norway spruce | 15.001–15.005 | 8.349–8.571 | 7,540,566–7,969,757 needles |

[Numeric evidence](final/numeric.json) retains every case, distribution, support
count and native artifact hash. Raw axis samples remain in the hashed bulk
artifacts. The operational axis/order definition is estimated, not a botanical
branch-order ground truth. Foliage bins count biological-unit centroids, not
leaf area. No source-backed distributions exist for these quantities, so their
collection success is not a biological pass threshold. Holdouts generated here
are now used evaluation cases, never fresh seeds for later tuning.

## Visual collection and projected gaps

[Visual receipts](final/visual.json) retain all 84 terminal records: 84 capture
passes, zero failures, zero unavailable and zero pending views. Every pass
retains native, 64-sample and 128-sample beauty/coverage artifacts and measured
convergence. This is technical collection success, not botanical approval.
The actual backend was ANGLE/Vulkan on NVIDIA GeForce RTX 3080, Chromium
151.0.7922.173, served from the isolated source checkout on port 5199.

The following ranges cover 12 whole views per species (six seeds, two azimuths).
They use the frozen projected crown ROI and primary coverage threshold 0.5.
The full receipts also retain threshold sensitivity, height bands, distributions
and convergence values. No source-backed biological target or favorable direction
is assigned to these measurements.

| Species | Occupied ROI % | Exterior-connected opening % | Enclosed gap % | Enclosed holes | Largest enclosed gap m² |
|---|---:|---:|---:|---:|---:|
| Oak |61.30–72.29|24.70–34.57|1.77–5.47|792–1,426|0.143–3.702|
| Spruce |53.31–56.78|40.33–44.50|2.19–3.42|440–640|0.112–0.459|

All 24 whole-view primary measurements report zero occupied pixels outside the
frozen ROI. These are 2D projected coverage diagnostics, not 3D empty volume or
leaf area. Direct native/128 inspection shows that fine spruce coverage is
sampling-sensitive; its repeated crown tiers also persist in the bare views.

Bulk evidence remains at `/tmp/fn19-mature-numeric-20260906`,
`/tmp/fn19-mature-conditions-20260906` and
`/tmp/fn19-mature-visual-20260906`; complete hashes and original paths are in the
compact receipts. All three roots are symlinks to durable storage under
`/home/daniel/Projects/telperion/.git/flow-artifacts/fn19-baseline/`.
[Preservation verification](final/preservation.json) rechecks the raw bytes and
hashes, including binaries; the evidence survives worker worktree cleanup. Original
fn9 and task3 smoke images were not substituted for mature evidence.

## Reference comparison limits

All six original reference JPEGs were hash-verified against the frozen inventory
and directly inspected. New retrieval returned HTTP403; task1's retained original
bytes supply the inspected evidence. The source photographs are not redistributed.
Their URLs, attribution and hashes remain in [references.json](references.json).

| Reference IDs | What the inspected imagery supports | Limit |
|---|---|---|
| O-WHOLE, O-BARE | Rounded spreading crown, irregular gaps, crooked branching hierarchy | Different individuals and leaf states; bare crown cropped; unknown dimensions/age |
| O-LEAF | Lobed blade and visible petiole connection | One local view; not complete shoot development or calibrated organ dimensions |
| S-WHOLE | Pyramidal habit and variable hanging crown tiers | Montage of two separate individuals; neither is a calibrated generated match |
| S-BRANCH, S-NEEDLE | Drooping secondary branchlets; attached individual needles and pegs | Unknown age, dimensions and viewing conditions |

Dedicated trunk bases, close oak collars/forks and bare spruce structure remain
reference gaps. The Grove, SpeedTree and Interactive Invigoration records are
qualified discovery/methodology references only: no admitted comparable mesh or
matched capture exists. Different species, a commercial spruce seedling and
uncalibrated research imagery cannot support a competitor ranking. No missing
comparison is scored as a Telperion win.

## Ranked engineering findings for fn20/fn21

These priorities rank the evidence available to implementation work; they are
not botanical grades. [Per-view inspection](final/inspection.json) keeps the
worker and conductor observations separate from expert feedback.

| ID / priority | Anatomical location and finding | Evidence and confidence | Handoff |
|---|---|---|---|
| FN19-G01 / high | Trunk base forms a smooth near-rotational trumpet flare, with angular silhouette transitions and a rounded lower rim; distinct buttress/root ridges are not visible | All twelve specimens `base/0`, also whole/bare silhouettes. High confidence in visible geometry; dedicated botanical base reference unavailable, so no calibrated species threshold | fn20: replace generic flare assumptions with reference-backed root/buttress anatomy; retain these frames as before evidence |
| FN19-V01 / high | Foliage overlap hides the selected fork/collar and prevents confident isolation of the attached shoot | Oak `fork/0` and `attached-shoot/0` across seeds; some selectors say resolved despite inadequate visible anatomy. High confidence in visibility limitation, not proof of a hidden geometry defect | fn20/fn21: keep hidden traits unassessed; use the separate visibility-v2.2 diagnostic below for exposed anatomy; preserve these original occluded views and their status |
| FN19-G02 / medium | Exposed front-facing fork has a rounded plug-like junction shoulder; trunk bends have angular silhouette changes, with no clearly resolved directional collar/ridge | Oak 1462003817 `fork/0`. Medium confidence in scoped surface-anatomy finding; no close calibrated botanical fork reference. Watertightness, intersection and side/rear continuity cannot be inferred | fn20: validate reference-backed local collars/ridges and directional transition geometry at this retained view |
| FN19-D02 / medium | Spruce shows repeated horizontal tiers in both whole and bare views, with hanging needle-bearing subdivisions | All six spruce specimens, both azimuths. High confidence that tiering is geometric; medium confidence as an organization question. S-WHOLE/S-BRANCH are qualitative and uncalibrated, and whole-scale needle coverage is resolution-dependent | fn21: assess tier regularity and connected-shoot organization against attributed age/context evidence; do not equate subpixel visibility with missing needles |
| FN19-D01 / medium | Mature-only shoot/crown evidence cannot establish developmental organization across age or crowded growth | Protocol population, O-LEAF, S-BRANCH/S-NEEDLE and explicit age/context gaps in the reference inventory. High confidence that evidence is absent; no inferred developmental failure | fn21: acquire attributed age/context and connected-shoot evidence; add a later cohort while retaining this one |

The frozen selector marks 14 of 84 targets occluded; their views remain in the
baseline. A resolved selector is not an anatomical visibility guarantee. Local
traits hidden by foliage remain unassessed even when capture converges.

The oak bare views show thick spreading scaffolds and retained fine subdivisions;
whole views show unequal crown masses, projecting tips and view-dependent
windows. Those are visible descriptive observations. The real O-WHOLE/O-BARE
images support qualitative spreading habit and irregular branching, but do not
supply a calibrated acceptable gap size, branch-order distribution or root-flare
shape. No extra leaves or camera changes were introduced to improve these views.

Representative full-resolution original PNGs are retained with their hashes:
[oak base](final/previews/oregon-white-oak-1-base-0.png),
[exposed oak fork](final/previews/oregon-white-oak-1462003817-fork-0.png),
[spruce whole](final/previews/norway-spruce-1-whole-0.png),
[spruce bare](final/previews/norway-spruce-1-bare-0.png), and
[spruce attached shoot](final/previews/norway-spruce-1-attached-shoot-0.png).
These are previews of the original captures, without resizing or retouching.

## Separate local-anatomy diagnostic supplement — visibility v2.2

The original v1 local views retain their occluded/unassessed statuses. A separate
[versioned visibility protocol](visibility-v2/protocol.json) addresses the capture
limitation with two diagnostic views per original specimen. It keeps the same
semantic targets, exact mature parameters and generated geometry. Every original
wood array remains unchanged. Exact admitted depth planes may crop remote
context; depth-cut ends are crop boundaries, never anatomical cap evidence.
Fork diagnostics suppress foliage;
shoot diagnostics retain the original complete elements and connectors mapped
to the selected terminal segment, suppressing unrelated foliage. The mapping is
geometric inference under the existing tapered-segment rule, because generator
output has no foliage-owner ID. Original instance indices are retained.

Automatic proposals rank fixed directions using branch-specific probes,
projected retained organ area and local framing. Probes do not decide anatomical
visibility. [Twenty-four visual admissions](visibility-v2/admissions.json) retain
actual inspected previews, hashes, observations and exact per-case conditions.
Several shoot proposals required wider depth to retain incoming connectivity;
oak-1 fork reuses a source-verified uncropped pilot camera. Final capture reuses
the admitted cameras, targets and instance indices without search or refitting.
Hidden rear surfaces remain unassessed. This is a declared anatomy diagnostic, not a
natural full-foliage view, density measurement or replacement for any of the
84 v1 views. The supplement uses neutral 1600×1000 renders at 64 and 128 samples,
with the same 0.005 beauty-convergence ceiling; no gap statistics are computed.

The [capture receipt](visibility-v2/capture.json) retains exact cameras, semantic
node IDs, retained original instance IDs, original geometry hashes, actual
backend and complete executable identities. The capture source was frozen at
`5234c624bc5664cdc2bba53d95338e0df7a96127`, clean for all consumed files, with
content identity `47689a8f524aba74aeeb9de94fe0a64574de920b7be005f7d02d81614d574b35`.
[Direct image inspections](visibility-v2/inspection.json) distinguish anatomy
visibility from collection and from independent botanical assessment. Original
128-sample PNGs are retained without resizing, annotations or retouching.

[Verification](visibility-v2/verification.json) records 24/24 captured and directly
inspected diagnostics, with all 48 capture PNG hashes verified. Beauty RMSE
between 64 and 128 samples spans 0.000181–0.001711, below 0.005. Every final
camera, target and retained instance list equals its frozen admission; generated
array and Wasm hashes equal the original mature receipts. Collection success
does not assess hidden anatomy or supply independent botanical feedback.

The [superseded v2 attempt](visibility-v2/failed-attempt-inspection.json) is
preserved: a broad parent surface falsely satisfied a small lateral's probe,
leaving the collar hidden. Direct inspection caught this. Version v2.2 measures
probe depth against the local interpolated branch radius and uses a
radial-facing lateral proposal check; it does not promote probe success to expert or
anatomical approval. [Proposal history](visibility-v2/attempts.json) also preserves
crowded and depth-sliced proposals that were withheld, with original images and
tool identities. Per-case visual admission is implementation-side judgment;
it does not establish independent botanical approval.

The supplement runner only reproduces the pinned original geometry and rejects
changed generated arrays. It is not a changed-revision comparator. A future
candidate adapter must reuse the retained baseline cameras and declared filters
and validate semantic mappings; independently searching or refitting candidate
cameras would not be a matched comparison. V1 and visibility-v2.2 conditions must
remain separate. Independent botanical assessment is still unassessed.

The [preservation receipt](visibility-v2/preservation.json) verifies all 26 raw
capture/proposal/source-archive directories and 202 files before and after
relocation to `.git/flow-artifacts/fn19-visibility/`. Original `/tmp` paths remain
symlinks; no evidence bytes changed. Committed previews and final PNGs provide
reviewable images alongside those retained raw receipts.

## Revision replay and costs

The [comparison controls](final/comparison-controls.json) verify two independent
small native runs with unchanged source, then deliberately changed and failed
receipt fixtures. Unchanged measured values compare successfully. Metric drift,
missing/duplicate cases, interrupted output, source/binary corruption and a
changed cohort remain inconclusive. A synthetic changed-source/metric fixture
surfaces its change; it is not a production geometry improvement. Node controls
also surface image changes and reject changed cameras or failed views.

The [real replay receipt](final/replay.json) covers two independent native runs
and two independent 42-view captures of the six-case artificial 2m oak cohort.
Both captures completed 42/42 views. Comparison finds zero changes in metrics,
geometry hashes, projected gaps or PNG/F32 image hashes. Representative baseline
and candidate images were also directly inspected. A separate fixture substitutes
one existing second-azimuth PNG with a corrected artifact receipt; comparison
reports exactly one changed image. This tests reporting, not a generated geometry
improvement. [Replay preservation](final/replay-preservation.json) verifies the
complete raw trees under `.git/flow-artifacts/fn19-replay/`, with original `/tmp`
paths retained as symlinks. The [full mature self-integrity check](final/mature-integrity.json)
validates 12 cases and 84 views but is explicitly not independent replay.

The first small visual preparation was rejected because the capture validator
compared serialized object key order. A subsequent old-tool attempt was stopped
before the fix; its incomplete records remain retained and compare as
inconclusive, even against themselves. The validator now compares object values
semantically, preserving array order and rejecting changed render values; the
same correction applies to frozen-camera equality. A reordered nested-key test
failed before the fix and passes afterward. No render, camera or parameter values
were changed. Mature tool hash `bda8f89c…` and corrected replay tool hash `ee62dad3…`
remain separate; their complete identities are in [sources.json](final/sources.json). Relevant dirty replay-tool bytes are fully enumerated in its
receipt; the source revision and unchanged generator binary are retained.

The combiner verifies source/tool digests, actual binary/artifact bytes,
parameters and full case/view sets before comparison. Complete validated metric objects receive canonical content digests; raw axis samples remain in their hashed artifacts while the report emits compact distributions. Native and browser source
identities have different domains: common core files and the native support
binary must agree within each combined run; their aggregate hashes need not.
Across revisions geometry/output hashes may change. Frozen protocol/reference,
metric/capture tools, camera/ROI, browser and backend conditions must agree.
Numerical-only comparison is explicitly labeled. Costs have a separate
inconclusive disposition even when measured geometry is comparable.

Native `generation` measures **skeleton branching only**, excluding mesh,
foliage and metric collection. Browser generation and capture preparation are
separate observations. CPU RSS, Wasm capacity, GPU allocation and GPU time retain
their original domain/status rather than becoming one runtime score. Other host
work, including fn18 native measurements, overlapped this run; no exclusive
window, end-to-end speed comparison or performance gain is claimed.

[Cost observations](final/costs.json) retain each case and unavailable domain.
Ranges below span six cases each; these columns measure different stages and
must not be compared as interchangeable total runtimes.

| Species | Native skeleton only ms | Browser generation ms | Browser preparation ms | Wasm capacity MiB |
|---|---:|---:|---:|---:|
| Oak |25.35–28.61|2,017.9–2,387.5|4,823.9–6,143.2|229.94–248.13|
| Spruce |36.27–39.77|8,303.9–10,090.5|12,032.3–15,390.6|1,298.00–1,317.06|

CPU RSS, GPU allocation and GPU time were not measured and remain unavailable.
Wasm capacity is reserved linear-memory capacity, not CPU RSS or GPU allocation.

## Independent review and reuse

The [fixed independent-review packet](final/independent-review.json) lists every
case and six trait categories, requests evidence IDs and supported/contradicted/
unassessed judgments, and preserves reviewer role, relevant expertise, relation
to implementation and original feedback. All feedback fields are empty because
no qualified feedback was received. Implementer image inspection is engineering
judgment and does not fill those fields.

[Replay instructions](../../../tests/migration/README.md#comparative-botanical-benchmark-fn19)
cover fresh output directories, source freeze, baseline conditions, revision
comparison and failure semantics. The [onboarding workflow](../../../docs/species-onboarding.md),
[dispatch template](../../../templates/species-profile.md) and
[separate example packets](onboarding-examples/README.md) support parallel species
research and template work. Shared capabilities, registry/binding integration
and exclusive measurement windows require coordinated ownership and ordering.

The later manifest's Scots pine fixture remains `unsupported-anatomy`,
`implemented=false`; it tests admission mechanics, not a new generated species.
Its comparison control rejects mixing the later cohort with old runs. Existing
protocols stay immutable as the catalogue grows. Fn20/fn21 should cite stable
finding and case/view IDs, preserve failures and seek missing reference/expert
evidence before making stronger biological claims.

## Verification

[Gate receipts and logs](final/gates.json) retain the green pre-edit baseline,
red-to-green regression checks, 16 focused Node tests, 9 Rust geometry/species
tests and TypeScript check. The conductor separately ran its full Rust/Vitest
suites at the pre-task4 revision; that is labeled separately and is not claimed
as a worker post-integration result. Historical fn9 evidence is unchanged.

The visibility completion fix has separate [gate receipts](visibility-v2/gates.json):
14 focused Node checks and TypeScript passed, including red-to-green controls for
parent-radius false positives, radial-facing direction and admission cohort
validation. No production generator code changed; full Rust/Vitest suites were
not rerun for this diagnostic-only fix.
