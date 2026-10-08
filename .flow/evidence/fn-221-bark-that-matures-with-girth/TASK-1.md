# fn-221 task 1: old bark blended in by radius

## Rows

There are six new `/material` rows, read by the wood shader alone:
- `matureRed`, `matureGreen` and `matureBlue` set the old bark's colour.
- `matureFurrowStrength` is the furrow strength old bark takes in place of `furrowStrength`.
- `matureRadius` (m) is where the bark starts to turn old. Zero means no wood is old.
- `matureSpan` (m) is how much further the radius grows before the bark is wholly old.

The blend is a smoothstep from `matureRadius` to `matureRadius + matureSpan`. Young wood's shoot colour still blends over the result.

The first build had a low and a high radius with a refusal for a backwards range. The dial table's first step of `matureRadiusLow` from a zero high was refused (`every_dial_steps_to_a_value_the_generator_accepts`), so the pair became a start and a span. That way every dial steps on its own.

## Neutral rows

- **Inert row:** `a_row_with_no_mature_radius_draws_the_bark_whatever_old_bark_it_states` (`crates/telperion-render/tests/mature_bark.rs`) draws the same frame whatever old bark a row with `matureRadius` zero states. It covers the Whole and Bare views, on bark with relief.
- **Stills against the base:** 12 stills were rendered at the base `c09679b3` and on the branch: beech, oak, spruce and palm at seed 1, in the whole, bare and base close-up views. All 12 are byte-identical (`raw/stills.sh`).
- **Catalogue identity:** the family, walk and override digests are still fn-222's once the six rows are stripped. Every other row's single-row refusal reads as it did; only the index-paired refusals re-paired (`raw/refusals.sh`).

## The blend on a specimen

The specimen is the Oregon white oak at seed 1 with test rows (`raw/oak-mature.json`):
- young furrow 0.1;
- old bark 0.045/0.04/0.036 with furrow 1.0;
- the blend runs from 0.08 m over a span of 0.12 m.

The stills are in `raw/`:
- `oak-bare-mature.png`: the trunk is dark, the limbs stay pale.
- `oak-close.png`: the shipped oak at the base.
- `oak-close-mature.png`: the base is dark and deeply fissured, while the thinner limbs keep the pale bark.

The horizontal seam on the trunk is in the shipped oak too: it is the geometry, not the blend.

Notes for fn-213:
- The fissure, crest and weathering tints are absolute offsets, so they still act on a dark old bark. The oak's fissure tint reads purple on it.
- Where the radius steps at a fork, the colour steps with it.

## Gates

- **Render half:** `cargo test --profile ci -p telperion-render --no-fail-fast` passed.
- **Workspace half:** `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` failed only on the material dial count (95 to 101). After the fix, the `tuning_engine` target passed on a rerun.
- **npm test:** 143 of 143 passed.
