# Comparative botanical geometry baseline — fn19-v1

The twelve frozen mature oak/spruce cases complete numerical collection and all
84 required visual captures, each with an implementation-side inspection record.
The engineering baseline does **not** establish biological superiority:
references are qualitative, matched competitor assets are unavailable, and
independent botanical assessment remains unassessed (R3's independent-assessment
portion is unresolved). No reviewer was contacted and no assets were purchased.

The intermediate receipts, logs, preview PNGs, replay trees and visibility-v2
captures were archived in the 2026-10-16 evidence trim; see
`.flow/evidence/ARCHIVE-MANIFEST.md` for their source revision and recovery.

## Frozen evidence and scope

Generation used source revision `b5249e16c7e2c10f88f8409294fce38c0ddfb435` with
complete source/tool/native/Wasm content identities; generator and capture tool
were frozen before generation. [The protocol](PROTOCOL.md) pins parameters and
cameras; [references.json](references.json) holds reference URLs, attribution and
hashes (source photographs are not redistributed).

| Species, six specimens each | Height m | Crown width m | Individual foliage units |
|---|---:|---:|---:|
| Oregon white oak | 17.588–19.633 | 18.454–23.876 | 495,781–585,533 leaves |
| Norway spruce | 15.001–15.005 | 8.349–8.571 | 7,540,566–7,969,757 needles |

[Numeric evidence](final/numeric.json) retains every case, distribution, support
count and native artifact hash. The operational axis/order definition is
estimated, not botanical ground truth; foliage bins count unit centroids, not
leaf area; no source-backed distributions exist, so collection success is not a
biological pass. The holdouts are now used evaluation cases.

## Projected gaps (12 whole views per species, ROI threshold 0.5)

| Species | Occupied ROI % | Exterior-connected opening % | Enclosed gap % | Enclosed holes | Largest enclosed gap m² |
|---|---:|---:|---:|---:|---:|
| Oak |61.30–72.29|24.70–34.57|1.77–5.47|792–1,426|0.143–3.702|
| Spruce |53.31–56.78|40.33–44.50|2.19–3.42|440–640|0.112–0.459|

2D projected coverage diagnostics only, not 3D voids or leaf area. Fine spruce
coverage is sampling-sensitive. Capture ran on ANGLE/Vulkan, RTX 3080.

## Reference comparison limits

| Reference IDs | What the inspected imagery supports | Limit |
|---|---|---|
| O-WHOLE, O-BARE | Rounded spreading crown, irregular gaps, crooked branching hierarchy | Different individuals and leaf states; unknown dimensions/age |
| O-LEAF | Lobed blade and visible petiole connection | One local view |
| S-WHOLE | Pyramidal habit and variable hanging crown tiers | Montage of two individuals |
| S-BRANCH, S-NEEDLE | Drooping branchlets; attached needles and pegs | Unknown age, dimensions, conditions |

Trunk bases, close oak collars/forks and bare spruce structure remain reference
gaps. The Grove, SpeedTree and Interactive Invigoration are discovery references
only; no missing comparison is scored as a Telperion win.

## Ranked engineering findings for fn20/fn21

[Per-view inspection](final/inspection.json) keeps worker and conductor
observations separate from expert feedback.

| ID / priority | Finding | Handoff |
|---|---|---|
| FN19-G01 / high | Trunk base is a smooth near-rotational trumpet flare; no buttress/root ridges | fn20: reference-backed root/buttress anatomy |
| FN19-V01 / high | Foliage hides the selected fork/collar and attached shoot | fn20/fn21: keep hidden traits unassessed |
| FN19-G02 / medium | Exposed oak fork has a rounded plug-like shoulder; no directional collar/ridge | fn20: local collars and ridges |
| FN19-D02 / medium | Spruce shows repeated horizontal tiers in whole and bare views | fn21: tier regularity against age/context evidence |
| FN19-D01 / medium | Mature-only evidence cannot establish developmental organization | fn21: age/context and connected-shoot evidence |

The frozen selector marks 14 of 84 targets occluded; a resolved selector is not
a visibility guarantee.

## Visibility v2.2 supplement

A separate [versioned visibility protocol](visibility-v2/protocol.json) adds two
diagnostic views per specimen (fork without foliage; shoot with only its own
foliage) on the same geometry. 24/24 diagnostics were captured, admitted and
inspected; beauty RMSE 64 vs 128 samples 0.000181–0.001711. A superseded v2
attempt let a parent surface satisfy a lateral's probe; v2.2 measures probe depth
against the local branch radius. This is a declared anatomy diagnostic, not a
natural view, and hidden rear surfaces stay unassessed.

## Revision replay and costs

Comparison controls verified that unchanged runs compare clean while metric
drift, missing cases, interruption, corruption and a changed cohort stay
inconclusive. A real replay of a six-case 2 m oak cohort (two native runs, two
42-view captures) found zero changes in metrics, geometry hashes, gaps or images.
A capture-validator defect (JSON key order) was fixed semantically; mature tool
`bda8f89c…` and replay tool `ee62dad3…` stay distinct.

[Cost observations](final/costs.json); columns are different stages, not totals:

| Species | Native skeleton only ms | Browser generation ms | Browser preparation ms | Wasm capacity MiB |
|---|---:|---:|---:|---:|
| Oak |25.35–28.61|2,017.9–2,387.5|4,823.9–6,143.2|229.94–248.13|
| Spruce |36.27–39.77|8,303.9–10,090.5|12,032.3–15,390.6|1,298.00–1,317.06|

CPU RSS, GPU allocation and GPU time were not measured. Other host work
overlapped; no speed claim.

## Independent review and reuse

The [fixed independent-review packet](final/independent-review.json) holds no
feedback because none was received. [Replay instructions](../../../tests/migration/README.md#comparative-botanical-benchmark-fn19),
the [onboarding workflow](../../../docs/species-onboarding.md), the
[dispatch template](../../../templates/species-profile.md) and the
[example packets](onboarding-examples/README.md) support parallel species work.
The Scots pine fixture stays `unsupported-anatomy`, `implemented=false`.

## Verification

Integrated gates: Rust 72 passed/6 ignored, Vitest 106, benchmark Node 21,
onboarding 22 checks, TypeScript, rustfmt. Completion review SHIP on round two.
