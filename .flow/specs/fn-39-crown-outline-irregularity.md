# Crown outline irregularity

## Conversation Evidence

> user (2026-09-14, after fn-34 round 3): "so i would like these species to inspire the generator to improve. I guess we can try and get as close as possible with values and then make a gap analysis what the generator needs and then spec that and once implemented do another comparison."
> gap analysis (`.flow/evidence/fn34/GAPS.md`, round 3): the beech's and the birch's crown outlines are smooth ovals where the photographs are lumpy and asymmetric, and no row changes that.

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 45% [paraphrase], 25% [inferred] -->

Every generated crown has the same outline on every seed: a superellipse of revolution, exact, axisymmetric, with no noise and no seed in it. The branches inside vary from seed to seed; the silhouette does not. A photographed beech is lumpy, with lobes where scaffold limbs end and hollows where they do not, and no two beeches share an outline. The round-3 pairs show it on both species: proportion and fill match to within a few percent, and the tree still reads as a shape rather than a plant. [paraphrase]

This spec gives the envelope a per-seed irregularity: a low-frequency perturbation of its radius that depends on height and bearing, with an amplitude and a wavelength as rows, neutral at zero so every shipped tree is untouched. Growth, retention and the surface then follow the perturbed shell the way they follow the smooth one. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **The envelope learns a bearing and a seed.** `radius_at(y)` gains a sibling `radius_at_bearing(y, azimuth)` that multiplies the smooth radius by one plus the amplitude times a seeded low-frequency noise of height and bearing at the stated wavelength; the existing noise module supplies it. `contains` and the shell rejection in the scaffold and the twig planner use the bearing form; `sample` rejects against it. With the amplitude at zero the sibling returns the smooth radius to the byte. [inferred]
- **The profile stays two-dimensional, and says so.** `profile()` and `distance_to_profile` feed shedding and the crown index, which assume one polyline; they keep the smooth profile, and the shedding depth and exposure are measured against it. The irregularity shapes where growth may go, not how retention is judged; a later spec may make retention bearing-aware if the pairs ask for it. [inferred]
- **The envelope carries the seed's phase.** The family's seed already selects the specimen; the envelope's noise is keyed by that seed so the same seed keeps the same outline, and a blend between two families walks amplitude and wavelength linearly while the phase follows the seed. [inferred]
- **Rows.** `skeleton.envelope.irregularity` (amplitude as a fraction of the radius, 0 to 0.5, neutral 0) and `skeleton.envelope.lobe_scale` (the wavelength as a fraction of the height, 0.05 to 1); harness sliders since envelope rows are built explicitly. [inferred]
- **The beech and the birch are the proof.** Their tables set an amplitude against the whole references, the matched pairs are rendered again, and a new silhouette statistic in the compare script, the outline's radial deviation from its fitted ellipse, is recorded on photograph and still. [user]

## API Contracts
<!-- scope: technical -->

- **Family rows** as above, validated by name, on the wire, blended linearly, in the browser metadata, in the sweep's moved or held lists. [inferred]
- **Compare script** gains `outline_deviation`: the standard deviation of the tree mask's boundary radius about the best-fit ellipse, over the boundary angle, for the still and for the photograph's box. [inferred]
- **Views and commands unchanged.** [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Neutral is inert.** Amplitude zero is byte-identical to today on every shipped preset, asserted by the pins. [paraphrase]
- **Containment holds against the perturbed shell.** Every test that asserts nodes above the crossover lie inside the envelope asserts it against the bearing form, and the perturbed radius never exceeds the smooth radius times one plus the amplitude, so the planning envelope and the influence radius stay bounded. [inferred]
- **The crown base is untouched.** The perturbation applies to the crown's radius only; the bole and the base height are as authored. [inferred]
- **Determinism and blend.** Same seed, same rows, same outline; a walk between two families changes the outline continuously since the phase is the seed's and the amplitude walks. [paraphrase]
- **File sizes.** The envelope is at 150 lines and has room; the noise module exists. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The envelope carries `irregularity` and `lobe_scale` rows with rails, neutral at zero, on the wire, blended and in the browser metadata; every shipped preset is byte-identical at neutral. Errors: a value outside its rail is refused naming the field. [inferred]
- **R2:** With a positive amplitude the shell's radius varies with height and bearing by a seeded low-frequency noise at the stated wavelength, growth and the twig planner reject against that shell, and every containment test holds against it; the smooth profile still drives shedding and the crown index and the spec says so. Errors: a node outside the perturbed shell fails the containment test naming the seed. [paraphrase]
- **R3:** The compare script records the outline's radial deviation from its fitted ellipse for the still and the photograph. Errors: a mask without a closed boundary reports the statistic as unavailable. [inferred]
- **R4:** The beech's and the birch's tables set an amplitude against their whole references, the matched pairs are rendered again with the numbers beside round 3's, and the owner judges the pairs and records the verdict in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]
- **R5:** The tests cover: neutral byte identity, the rails, the bound on the perturbed radius, containment against the perturbed shell on every fixed seed, determinism per seed, a blend walk of the amplitude, and the outline statistic on a synthetic pair. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- No growth-driven irregularity, no shading-out or limb death; fn-16 and fn-21 own those. [inferred]
- No bearing-aware retention or shedding in this spec. [inferred]
- No pendulous shoots and no multi-stem; their own specs. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner's loop: species expose gaps, gaps become generator specs, the comparison runs again. The smooth outline is the gap both species share. [user]

### Implementation Tradeoffs

- A perturbed envelope over irregular growth: the envelope is the one place every stage consults for where the crown may go, so one term there reaches growth, twigs and sampling at once; irregular growth is a lifecycle question and belongs to the environment specs. [inferred]
- A bearing-aware sibling over changing `radius_at` itself: the two-dimensional profile has consumers that assume one polyline; a sibling keeps them honest and lets retention stay smooth until the pairs ask otherwise. [inferred]

## Strategy Alignment

- Follows "The catalogue": a species exposed an unsupported form and the generator gains rows, never a branch.
- Follows the approach's continuous-tree-space rule: the seed selects the specimen, so the outline follows the seed.

## Resolved via Codebase

- The envelope is an axisymmetric superellipse with no seed: `crates/telperion-core/src/envelope.rs:6-12`, `radius_at` `:48-69`, `contains` `:71-78`, `sample` `:79-108`, `profile` `:109-117`, `distance_to_profile` `:122-149`.
- Consumers of the radius: scaffold edge rejection `crates/telperion-core/src/branching/scaffold.rs:90-113`, reach `:183-199`, twig planner `branching/local/planner.rs:3-8` and `:91-107`, shedding `branching.rs:203-211`, exposure `branching/specimen/crown.rs:94-116`, influence radius `branching.rs:107-120`, juvenile scaling `scaffold/frontier.rs:83-93`.
- Attractors are unused by every species preset (`attractor_weight: 0.0`); the shell alone bounds growth.
- Per-seed variation today is interior only: axis keys `scaffold.rs:22-29`, station phase `:235`, crookedness phase `:275`, twig jitter `local/advance.rs:122-135`.
- Noise: `crates/telperion-core/src/noise.rs` (`Noise::new(seed)`, `Noise::at`), used by the bias field only (`bias.rs:90`, `:140`).
- Containment tests: `tests/species.rs:202-204`, `tests/growth.rs:11-13`, `tests/growth/habit.rs:308-310`, `tests/crown_reference.rs`, `tests/colonization.rs`.
- Envelope sliders are built explicitly: `harness/params.ts:303-312`, `harness/family.ts:51-57`.
- File sizes: `envelope.rs` 150, `noise.rs` small, `scaffold.rs` 370.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R5 | TBD during planning |
