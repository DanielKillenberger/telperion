---
satisfies: [R7]
---
# fn-257-the-site-grows-its-trees-on-the-device.5 The field's resident data compact; the growth tables measured

## Description
The owner's re-scope (spec, "The owner's decisions on tasks 4 and 5 and the spruce"): compress the data a consumer holds, the field's resident primitives (`resident/field.rs`, `DeviceField`: today two `vec4<f32>` a primitive, 32 B, a `u32` system for every primitive, and the bucket index), the spruce's 100-109 MB to about 60 MB.

How (the host's design):
- **The primitives built as today, then packed.** `field/make.rs` writes today's f32 primitives into a scratch buffer and reads the field's box back; a new `pack` kernel (`field_build.wgsl`) writes the resident layout, 16 B a primitive: each endpoint's position as three 16-bit fractions of the field's box (12 B), and 4 B of attributes: the wood's start and end radii as two halves; a segment's reach as a half and its leaf count as a 16-bit integer; a frond leaflet's reach as a half and its box's flank (octahedral, 5 bits each) and thickness share (6 bits). The scratch is let go once packed, so only the compact data stays resident.
- **Systems only where leaves are.** The `owner` words cover the leaf primitives alone (wood has none), indexed from the first leaf primitive.
- **Readers.** `field.wgsl`'s `span`, `field_build.wgsl`'s `count`/`fill` and `field_query.wgsl`'s `query` decode through one pair of functions (`position`, `attributes`), so the layout lives in one place.
- **The growth tables measured for the same treatment** (`build_grow.rs`'s `Build`, `state.rs`): each table's allocated bytes against its most used rows on the spruce at 40 and Telperion at 82, and, per table, whether a compact layout would change any kernel's arithmetic (a packed or narrower field read by a kernel does; slack rows, padding or a buffer held past its last reader do not). Changes that touch no kernel arithmetic are taken here; the rest are reported for fn-285 in `TABLES.md`.

## Acceptance
- The field's resident bytes per shipped preset before and after (`DeviceField::bytes`), the spruce at or under about 60 MB.
- R2's tolerances hold on every preset at 24, 36 and 48 cells (`every_preset_s_device_field_answers_within_its_tolerance` extended to the three grids), the answers still bit-equal run to run, the crowded-cell and lost-device tests green.
- `TABLES.md`: the growth tables' allocated and used bytes, which compaction needs no kernel arithmetic change (taken, with the peak before and after) and which does (reported for fn-285).
- The trees' look unchanged: the harness build is untouched by the field's layout; any growth-table change shows its presets' stills unchanged to the eye.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
