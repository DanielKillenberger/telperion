# Bark relief that reads the same at a distance

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 50% [paraphrase], 20% [inferred] -->

The bark's relief does not read the same at a distance as it does up close: a trunk drawn at 4x the hero distance and reduced to the near frame's size differs from the near draw by more than the relief tests allow, and the owner had noticed the change of look before fn-55 opened. [user]

fn-55 uncovered the number. Its 4x distance series reads a mean of 3.134/255 against the bound of 3.0 once the old sheen is gone, and 3.167 with the highlight removed entirely; the bound held before only because 15% of the sun on the lit trunk sat the frame on the tone curve's shoulder, where the relief's cross-resolution error compressed into fewer code values. The bound sat at 2.904 before a line of fn-55 changed. fn-55 recalibrated the byte-domain bound to 3.25 with that reason on record and ships the bark grain rows held off, because any grain visible in S-BARK or B-BASE puts a cell on the half-size still's Nyquist edge and reads 3.4 to 4.5 against the smooth-bark bound. This spec owns the cause. [paraphrase]

The owner's word on 2026-09-18: the drift at distance is a real problem, seen before this spec, and it is not fn-55's to fix. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **The relief's cross-resolution error is measured first, in linear light.** The distance series and the smooth-bark series compare the near draw with a reduced far draw in bytes after the tone map; a linear-light measurement of the same pair separates the relief's own filtering error from the tone curve's. The resolution tests record their measured margins on every green run, not only on failure, which is fn-55's second friction entry. [inferred]
- **Footprint filtering is the lever.** `bark.wgsl` already fades each relief term by its footprint (`bark_edge`, `bark_box`, `bark_scale`); the drift is the residue those fades leave at the 2x2 box reduction the tests use. The fix is in the filtering of the relief and the grain, never a species row: a term that converges to its mean at the footprint the far draw samples. [inferred]
- **The grain gets its distance answer here.** The grain rows fn-55 added fade an octave before the mottle and still cross the Nyquist edge at close range; a grain that converges under the half-size footprint, or a close-range bound with its own reason, lets the bark tables set a value. [paraphrase]
- **Rows, not species.** Whatever is added is a renderer term every table shares; neutral values reproduce fn-55's frame. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The 4x distance series reads at or under 3.0/255 mean on the oak and the spruce with the physical highlight fn-55 shipped, and the bound is restored to 3.0 with the reason for the 3.25 interval recorded beside it. [user] Errors: a series over the bound fails naming the preset and the distance.
- **R2:** Every resolution test writes its measured mean and p95 to a receipt under the spec's evidence on a green run, so the margin is known before the next shading change. [inferred] Errors: a missing receipt fails the test.
- **R3:** The bark grain rows carry a value on the beech and the birch tables that reads as grain in B-BASE and S-BARK at 1440 tall, with the smooth-bark bound held at 3.0. [paraphrase] Errors: a value that breaks the bound is refused, never clamped.
- **R4:** The beech's and birch's close-ups and hero frames are rendered again at the hero distance and at 4x, the implementer answers after looking whether the bark reads the same material at both, and the owner judges in the harness on the mature path. [user]
- **R5:** Every redraw stays byte-identical, single-stem and untouched presets keep their pins, and the frame cost is recorded beside fn-55's. [inferred] Errors: a moved pin on an untouched preset fails naming it.

## Decisions

- **Owner, 2026-09-18, R4 rejected on the first round.** Judged in the harness on the mature path with the two new wood terms and the beech's grain on: "it still looks much less detailed from a distance. There's a noticeable change in fidelity which shouldn't be the case. Fidelity should go down to a degree where it's not noticeable at that distance." [user] The 2x and 4x bounds passing is not the finish line; the far draw must read as the near draw minus what the eye cannot resolve, judged across a walk, and the terms that fade by amplitude an octave before a pixel are the first suspects. The task is reopened on this verdict. [paraphrase]

- **Owner, 2026-09-18, R4 rejected on the second round.** Judged on the round-two build in the harness: "there's still a jump where the relief visibly disappears. It's a bit further away now but still there." [user] A fade window, however placed, is a jump when the walk crosses it; the relief may only leave by the box integral of its own height over the footprint, with no separate attenuation. The task is reopened for round three. [paraphrase]

- **Owner, 2026-09-18, R4 accepted on the third round.** Walking the round-three build in the harness with every amplitude fade gone: "yea much better". [user] The seam is gone; the footprint sweep is flat within 0.03 between adjacent factors and the birch keeps its dashes and marks through the wheel notch. The cost went to 10.0, 15.0 and 19.6 ms on the three hero frames against round two's 5.9, 4.0 and 11.4, so the three reductions the report names run before the PR with round two's cost as the target and the flat sweep as the invariant. [paraphrase]

- **Not fully met, tracked rather than claimed.** R1 names the oak and the spruce; the shipped 4x series asserts the oak alone, because the spruce's bare crown throws twig shadows over its trunk at every strip the fixture can frame (plain wood with no relief reads 6.2/255 there), and its bark is held meanwhile by the grazing fixture at 2.70. R3's lichen keeps its amplitude fade while every other term lost one. Both are the follow-up spec's, named in this spec's report. [paraphrase]

## Boundaries
<!-- scope: business -->

- No new bark pattern; fn-32's plates and fn-40's layers are kept. [inferred]
- No tone-map change; the filmic curve stays as fn-29 set it. [inferred]
- No species branch and no per-preset filtering path. [paraphrase]

## Strategy Alignment

- Serves "Surface and rendering at scale": simplification and detail selection operate on generated geometry and measured error, without visible stepping or shimmer from a hero tree to a forest. [strategy:Surface and rendering at scale]
