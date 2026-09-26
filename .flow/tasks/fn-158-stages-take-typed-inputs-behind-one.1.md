# fn-158-stages-take-typed-inputs-behind-one.1 Stages take typed inputs behind one pipeline boundary

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Blocked:
NEEDS_HUMAN (2026-09-26): the typed stage inputs and the executor interface are in and byte-identical (commits 071169be, e392371c, 69549f15; `.flow/evidence/fn-158-stages-take-typed-inputs-behind-one/RESULTS.md`), and the GPU executor's schedule and position/station overlap and the growth path both fit the interface without a behaviour change. The visibility step (stages private, tests moved, the compile-fail test) needs three host decisions, because making the stages private breaks callers the spec does not name:

1. **GPU executor tests on fixtures.** `telperion-render`'s generation tests (`generation/tests.rs`, `wood_tests.rs`, `position_tests.rs`, `position_probe.rs`, `readback_tests.rs`) build hand-made trees and custom leaf boxes (`Reference::spanning`) and call `prepare_stations`, `surface::prepared`, `surface::compact`, `place_on_surface` and `cull` on them. Options: (a) the interface admits a solved tree the executor already holds, `executor::expand(tree, &Family) -> Expansion`, and the GPU compute keeps taking the box as an argument (recommended); (b) rewrite those tests onto presets, which loses their targeted kernel coverage (huge phases, signed zero, seated contacts).
2. **Elements built from rows.** Seven render tests (`lobe_shade`, `canopy_light`, `leaf_detail`, `blade_colour`, `submit`, `shadow`, and unit tests in `foliage.rs`, `select/frame.rs`) call `foliage::build_element(ElementParams)`. Options: (a) expose it through the interface as `executor::element` (recommended); (b) keep `build_element` public as a type constructor; (c) build elements through `pipeline::build`, which grows a whole tree per element.
3. **Evidence examples with their own chains.** `species_measure` (npm `species:measure`, `species:qa`), `measure`, `geometry_benchmark`, `occupancy_audit`, `node_buffer`, `generation_limits`, `field_still` and `curtain_audit` call stages directly; `species_measure` grows with `branching::generate`, skipping the apical clearing and leaf bases the pipeline applies. Options: (a) reroute each through `pipeline::build` and compare its output with the base before and after (recommended; `species_measure`'s listed presets should not move, the unlisted date palm would gain its clearing and bases); (b) delete the ones no script runs.

Mechanism I would use once these are settled, unless the host objects: the 47 core integration tests that call stages move into `crates/telperion-core/src/suite/` as `#[cfg(test)]` modules, compiled with `extern crate self as telperion_core` and facade modules that re-export the whole stage only under `cfg(test)`; `Reference::of`, `TwigPlacement::of` and `footprint::predict` move to typed inputs; the growth path's own entry (`Specimen::build(&Family)`) stays as the sanctioned exception.
## Evidence
- Commits:
- Tests:
- PRs:
