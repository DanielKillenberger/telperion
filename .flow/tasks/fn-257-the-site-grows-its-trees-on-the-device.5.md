---
satisfies: [R7]
---
# fn-257-the-site-grows-its-trees-on-the-device.5 The field's resident data written compact

## Description
The owner (spec, "Compact from the start"): every table is laid out compact where it is created, never converted at the end; lossy layouts only for data growth never reads back, the field's resident primitives among them, their precision bounded by tests.

**Touches:** `crates/telperion-render/src/generation/resident/field.rs`, `field/make.rs`, `field/index.rs`, `field/ask.rs`, `field.wgsl`, `field_build.wgsl`, `field_query.wgsl`, `field_tests.rs`; evidence `FIELD-LAYOUT.md`.

How (the host's design):
- **The box first, then the primitives written compact.** A `box` kernel grows the field's box from the drawn nodes and the station segments (and frond leaflets) as `wood`, `leaves` and `leaflets` do today; the box is read back; then those three kernels write each primitive directly in its resident layout, 16 B: each endpoint's position as three 16-bit fractions of the box (12 B), and 4 B of attributes: the wood's start and end radii as two halves; a segment's reach as a half and its leaf count as a 16-bit integer (a tile holds at most 256); a frond leaflet's reach as a half, its flank in two 5-bit octahedral fractions and its thickness share in 6 bits. No f32 copy of the primitives is made.
- **Systems only where leaves are:** `owner` covers the leaf primitives alone, indexed from the first.
- **One decoder:** `field.wgsl`'s `endpoints` and `attributes` read the layout for `span`, `count`, `fill` and `query`.

## Acceptance
- `FIELD-LAYOUT.md`: the layout, each field's precision; the field's resident bytes per shipped preset before and after (`DeviceField::bytes`), the spruce at or under about 60 MB.
- R2's tolerances hold on every preset at 24, 36 and 48 cells (the default test extended to the three grids); answers bit-equal run to run; crowded-cell and lost-device tests green.
- The harness build untouched (the field is the field entry's alone).

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
