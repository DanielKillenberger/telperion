# fn-294: the two plausible findings, reproduced before any fix

Both reproduced, so both were fixed. Each repro was committed as a red test before its fix.

## R5 (Codex 5.1): metadata panics under another light

- Test: `params::tests::an_overlay_of_a_family_under_another_light_is_refused_by_name` (`crates/telperion-core/src/params/tests.rs`). It takes the oak (`preset(3)`), sets `space.light = Light::NEUTRAL` (a valid `Light`, not `SITE`) and calls `params::overlay(&family, &{})`.
- Red at `53545a1f4` (base `7f6d6e8ea`): it panicked at `crates/telperion-core/src/params.rs:31:38`, the `expect` on `space::rows`.
- Fix: `overlay` writes the wire through a fallible `wire(f)`. That returns `space::rows`'s refusal as `Error::InvalidValue { field: "space", value: "a preset grows under the site's light, which no row states" }`. There is no fallback. `metadata` stays infallible for what it serves (every preset and every parsed wire is written as rows), and its doc says a hand-built family that is not written as rows panics there.

## R6 (Codex 6.1): an oversized light lattice asserts

- Probe (not kept): through the public `telperion_space::grow`, with extinction 0.5, seed 1 and a 10,000 budget, the oak, palm, spruce and beech species were each tried with every internode set to 5, 20, 50 and 100 m, at ages 1, 2, 3, 5 and 8.
  - The oak at 5 m and 8 years panicked: "a lattice inside the contract's domain".
  - Every case at 20 m or more was refused by name first: the ground bound, or "a position beyond the contract's 4,096 m".
  - Codex's candidate (the palm at 100 m, year 3) is refused by the ground bound.
  - The bounds therefore do not rule the assert out: 5 m is well inside `MAX_INTERNODE` (100 m), and the tree stays under 4096 m.
- Test: `light::tests::a_lattice_past_its_nodes_is_refused_by_name` (`crates/telperion-space/src/light/tests.rs`). It is the oak, every internode 5 m, at 8 years with a 10,000-phytomer budget.
- Red at `6b2ba9725`: it panicked at `crates/telperion-space/src/light/field.rs:66:9`.
- Fix: `Grid::spanning` and `Field::spanning` return `Result`. A box past 2^28 nodes is refused as `Error::Refused { input: "light.lattice", reason: "a lattice spans at most 2^28 nodes" }` before the density, light or quanta vectors are allocated. `Field::quantised_with` (the one cycle and the capture) propagates the refusal. The test-only constructors expect it.
- The `REACH` assert beside it stays: the geometry layer refuses any position past 4096 m by name before a lattice is spanned. That is the probe's "a position beyond the contract's 4,096 m".
