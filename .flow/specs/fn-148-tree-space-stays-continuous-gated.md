# Tree space stays continuous: gated settings are declared, and blends are measured smooth

## Conversation Evidence

> owner (2026-09-24): "do we still uphold the continuous tree space constraint as true?"
> owner (2026-09-24): "but we need also a spec to make tree space continuous again. I mean some params being zero just basically disable other params. Maybe that's just a ui problem"
> host renders (2026-09-24): the tuned date palm at side-branch order 1 and 2 is pixel-identical to order 0; with envelope spread 0.5 its wood grows from about 6,700 to 38,000 vertices, but the new branches carry no fronds, because a rosette stands only at a stem's apex.

## Goal & Context
<!-- scope: business -->

STRATEGY.md holds that every parameter acts on every tree and no field is a switch. Two things break that in practice. Some settings gate others: at zero side-branch order, or a near-zero crown envelope, every twig and branch setting does nothing, and the harness still offers them as live sliders, so the space reads as broken. And some paths between presets are continuous only on paper: blending an oak toward the palm moves the numbers smoothly while the crown's form jumps, because fronds exist only at stem tips. Gating is partly a presentation problem (declare it and show it); the jumps are generator problems (measure them, then fix what the measure finds). [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-24 on the fn-80 branch.** `blend::families` interpolates every field linearly at one seed, rounds counts last and ramps supernatural terms from nothing; its doc claims "no frame where the tree changes kind". No test measures that claim across shipped presets. The harness (`harness/dials.tsx`) lists rows without any notion of which rows are inert for the tree in view; side-branch order and stem count are not sliders there. [checked]
- **Gates, declared.** Each row that only acts when another row is non-zero (or above a floor) declares that gate beside its doc comment, as data; code verifies each declared gate by a sweep (moving the gated row changes nothing while the gate is closed) and fails on an undeclared one it finds. [inferred]
- **Harness.** A gated row shows as dormant with its reason ("acts once side-branch order is above 0") and stays reachable; opening the gate makes it live. Every row a species can move, stems and branch orders included, is reachable. [inferred]
- **Tuning.** A round never proposes a row whose gate is closed on the current tree. [inferred]
- **Blend measure.** For every pair of shipped presets, the blend at t in twenty steps is grown and a few whole-tree measures read (height, crown width, wood volume, leaf area, foliage count); a jump between adjacent steps larger than a stated share of the pair's total change is a discontinuity, reported with the rows that step there. [inferred]
- **Unknown.** What the measure finds and how each jump is fixed; the known candidate is crowns at branch tips (a rosette at every apex a lateral ends in, weighted so today's trees are unchanged), which the host decides once the measure has run. [unknown]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A gate table covers every gated row; the sweep finds no undeclared gate and no declared gate that does not hold. [inferred]
- **R2:** The harness shows dormant rows with their reason for the preset in view, and exposes stems and branch orders. [inferred]
- **R3:** The blend measure runs over every pair of shipped presets and its report lists each discontinuity found; every jump it finds is fixed or captured as its own spec before this spec closes. [inferred]
- **R4:** Shipped presets are byte-identical unless a fix under R3 changes them with visual evidence; the workspace gate is green.

## Boundaries
<!-- scope: business -->

- Not per-species variation ranges or locked traits (fn-146). Waits for the runner cleanup (fn-80, SIMPLIFY.md).

## Strategy Alignment

- Serves "Our approach": one continuous tree space where no family field is a switch. [strategy:Our approach]
