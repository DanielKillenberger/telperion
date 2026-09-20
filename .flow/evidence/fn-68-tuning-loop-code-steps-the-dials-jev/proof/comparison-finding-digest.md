# Comparison finding digest — host inspection, not qualification

R7 is stopped. Code guards passed. Semantic qualification is false until the host confirms from these findings. The clipping substring is not proof.

Cumulative after three charged calls: 622967 tokens, visual 23. Runtime `d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e` unchanged.

## Accounting

| stage | reserve | used | in+out | model / effort | status |
| --- | ---: | ---: | --- | --- | --- |
| stage-a-birch | 25000 | 19456 | 18544+912 | gpt-6-astra / medium | ok |
| stage-b-birch-positive | 40000 | 25153 | 23789+1364 | gpt-6-astra / medium | ok |
| stage-b-beech-negative | 40000 | 25927 | 24312+1615 | gpt-6-astra / medium | ok |

Packet spend 70536. Visual this packet 3 / 5. Caps 902431 / 25. No unknown usage. No retry.

## Stage A inventory (bound, not a placeholder)

Request `cc34097651c758f31253e35bf11f5457510e2054c0b2b086d2aa10bb5b901613`. Prompt `72933135a419384322f8be504614ea613da923a98007c4bdc0cbecfa1e4d5ad5`. Schema `4c13c160206ee1491a194676a627beccdc5e53d97adad294646e2d74ab3f74f5`.

Core traits: `pendulous_branchlets`, `pale_dark_marked_trunks`, `fine_foliage_texture`, `irregular_cascading_crown`. File: `stage-a-birch-inventory.json`.

## Stage B+ birch-positive — raw finding text

Request `8eb19549202d1c9cc3fb75a9aa189359bb2346d0dcfa03ed07d6bf3f3cbe6bd3`. Prompt `e46e0a6869a674552a877b707ff1a7e027dbfd95f35653f8893f79d48eeee04c`. Schema `40b44f74e2832e0e52874cb964ff3f11b714cc6333ecca85f93378397c38d7cc`.

Code guard: `positive_calibration_ok` (required cells all `pass`; one supported finding cites `render-0` and `reference-0`). Not host semantic qualification.

Raster: historical-geometry reframe `5c0112df0a647fdfa5eb6ad8816a45c92061206b919a772535646861c69e382e`. Disclosed label only; not new owner aesthetic approval.

passes: `["pass"]`. defects: none.

Observations:

- The candidate supports all four core traits at whole-view scale and meets the supplied catalogue finish floor.
- The reference's cropped top and overlapping background vegetation limit exact silhouette comparison; specimen correspondence is unknown.
- The candidate exposes more upper branching and has darker, less white-looking bark than the photograph. These differences do not erase the supported defining traits.

Findings:

1. impact `supported`, evidence `render-0` `reference-0`: Long, finely leafed hanging strands, an irregular cascading crown, and visibly marked silvery-gray stems jointly support the reference character.
2. impact `optional`, evidence `render-0` `reference-0`: Upper branch fans are more exposed and visually wiry than the photograph's leafy shoulders. Softening their prominence would improve resemblance, but the whole crown retains its fine pendulous character.
3. impact `supported`, evidence `render-0` `anchor-0`: The candidate's fine geometry, coherent silhouette, foliage detail, and trunk treatment meet the stylized whole-view finish demonstrated by the spruce anchor. No clear construction defect warrants blocking at this floor.

Coverage: all ten inventory traits `pass` with `render-0` and `reference-0`.

Raw: `proof/stage-b-birch-positive-stdout.json`.

## Stage B− beech-negative — raw finding text

Request `6bdf3e2c9d7ae5d3467c2b1279797d46fed12637a6bce2ac5cf13a9e9c40f7e2`. Prompt `1ab2752b307e8cee19d5f204180abc6f7aadef047fe77c1c10d313c3e0b76f8f`. Schema `40b44f74e2832e0e52874cb964ff3f11b714cc6333ecca85f93378397c38d7cc`.

Code guard: `negative_code_guard_ok` (passes contains `fail`; one blocker cites render and reference). `semantic_qualification` is false. Clipping substring is not proof.

Raster: historical beech `7e69fda3…`, laterally cropped, width unknown. Clipping is UNKNOWN, not qualification.

passes: `["fail"]`.

Defect: `foliage_layering: Repeated elongated, upright foliage masses dominate the crown instead of the references’ irregular overlapping lateral sprays and interlocking leafy masses.`

Observations:

- Image mapping: Image #1 is render-0; Image #2 is reference-0; Image #3 is reference-1; Image #4 is anchor-0.
- The candidate has a substantial green crown and smooth gray trunk, but these supported similarities do not resolve the foliage-character mismatch.
- The catalogue anchor establishes finish/style only. It does not establish beech morphology or excuse missing evidence.
- The supplied known-negative designation is not visual evidence; the verdict rests on the attached images.

Findings:

1. impact `blocker`, evidence `render-0` `reference-0`, uncertain false: The candidate’s crown is dominated by repeated steeply oriented, elongated foliage masses, with conspicuous upright stems emerging through them. Both leafy references show more irregular lateral spray development, including shelves on the left and interlocking, spreading masses on the right. The difference concerns branch-bearing foliage organization, not photographic realism.
2. impact `required_unknown`, evidence `render-0` `reference-0` `reference-1`, uncertain true: The apex and much of the candidate’s outline are visible, but outer lateral foliage reaches and is clipped by the image edges. Complete crown proportions and perimeter irregularity cannot be established.
3. impact `required_unknown`, evidence `render-0` `reference-1`, uncertain true: Visible ascending limbs support part of the reference architecture. Dense foliage obscures the candidate’s repeated subdivision and outer twig taper, which are especially explicit in the bare reference.
4. impact `required_unknown`, evidence `render-0` `reference-1`, uncertain true: The exposed candidate trunk supports a gray, smooth, continuous bark reading without deep furrows or thick plates. Fine shallow surface markings remain unresolved at this viewing scale.
5. impact `optional`, evidence `render-0` `reference-0` `reference-1`, uncertain false: The candidate’s base widens but has a comparatively simple, even termination. The references show more irregular spreading root contact. This is a secondary refinement rather than the defining crown blocker.

Coverage: `foliage_layering` fail; `trunk_base` fail; `crown_outline` `branch_architecture` `bark_surface` `foliage_visual_grain` `lower_branch_reach` `bark_mottling` unknown; `foliage_color_range` `framework_variation` `foliage_presence` pass.

Raw: `proof/stage-b-beech-negative-stdout.json`.

## Host question

Do these findings semantically qualify the bounded calibration? If yes, same chat proceeds to the verified scoped R7 resume. If no, stop; no retry or redesign from this worker.
