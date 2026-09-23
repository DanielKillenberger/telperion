---
satisfies: [R1, R2, R3, R4]
---
# fn-113-a-dials-range-is-its-validated-bound.1 Implement A dial's range is its validated bound; the preset span is a stride, not a wall

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The 75 dial rows whose range came from the preset span now take it from the generator's validated bound, with a source line on each: 59 rows. The other 16 stay capped, and each has a `cap` reason. Every one of the 75 keeps its old span as `preset_span`, which never limits a move. No generator code changed and no bound was added.

- **Capped on both sides (7).** leader_internode, lateral_spacing, step, shoulder and writhe_wavelength are validated only as positive, twig_divergence only as finite, and clump_system_order is an unchecked whole number. envelope_height is validated only as >= 0, but a clump refuses a crown that low.
- **Ceiling capped, floor widened to the validated floor (9).** spread, gravitropism, lean, writhe_amplitude and spiral_rate (floor 0); length_taper (floor 0); trunk_radius (floor 4e-6); connector_length, whose ceiling is the leaf's own length.
- **Pairs split to stay independent.** Hue low and high take [-0.5, 0] and [0, 0.5]. Brightness low and high take [-1, 0] and [0, 1]. The generator requires low <= high (material.rs:489).
- **attractors** is [1, 1e6]. Zero is refused whenever the attractor weight is above zero.

Tests are in `crates/telperion-jev/tests/dial_bounds.rs`.
- R1: `no_row_takes_a_wall_from_the_preset_span`. `every_widened_wall_is_a_value_the_generator_accepts` runs each widened wall through the generator's own validators on every family. `a_value_past_a_validated_wall_is_refused` shows the check catches a wall set past the bound.
- R2: `a_move_past_the_preset_span_is_accepted`. leaf_length moves past its span on the date palm.
- R3: `the_date_palm_can_reach_its_measured_proportions`. Leaflets of 20 to 40 cm are reachable through leaf_length and leaf_size. A trunk 0.5 m across on a palm 24.4 to 35 m tall (sources F1 and A1) fits trunk_radius, through the widened floor. No source measures leaflet width, so that value is unknown. The row's ceiling is now the generator's 1e3.
- R4: the gate is green.

Follow-up for the host: the 16 capped rows need a ceiling authored in the generator before they can widen. That is a design decision, left to the host. If the palm's trunk ever needs to be thicker than 0.023 of the height, trunk_radius is one of them.

Tier: session (jev intelligent 0.42)

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 188df3aebd3860dc5237391647044813ab893254, e75d69ff6ba176792f03720b9267bb8a4c313e9f
- Tests: cargo test --profile ci -p telperion-jev --test dial_bounds --test dial_table, cargo test --profile ci --workspace --no-fail-fast, cargo fmt --all -- --check
- PRs: