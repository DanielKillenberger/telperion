# fn-217 task 2: the group's rows, scale and leaves

DESIGN.md sections 6 and 7, release half (a). Commits `1192044d` (group and scale), `76136924` (leaf mirrors), `7155dac6` (stills), then the conformance fix, the pins (`92ecdedf`) and the browser presets file.

## What was built

- **X2 extended (7.1).** `/supernatural` (`crates/telperion-core/src/supernatural.rs`):
  - its own rows: `enabled`, `scale`, `backReflectance`, `backRed/Green/Blue`, `marginReflectance` and `roughness`;
  - overrides that stand in for the natural row while `enabled` is on and leave it as it is while unset: `/supernatural/material/{leafFront,leafBack,margin}{Red,Green,Blue}`, every `/supernatural/bloom/<row>`, and `/supernatural/flowering/age<k>` for `/space/ages/<k>/flowering`.

  `Family::applied()` lays the overrides over the natural rows. The pipeline's two stage inputs (`GrowInput::of`, `Inputs::of`), the device's species (`resident.rs`) and the renderer (`Renderer::set_family`) read the family through it. The blend walks what each side applies. Every row is checked whether the group is on or off.
- **S1 (7.2).** The conversion runs in natural units. Then `space::scaled` multiplies every position, `radius`, `start_radius`, `base_radius` and grown length by the scale once. Frames, keys and topology stay natural. This happens before the folds, in both `grow` and `part`. The device dresses its roots at `Part::scale()`. `dress.wgsl` scales positions, radii and the grown record, so the previous wave's records and the stubs' distances arrive scaled. Its thinness and merge decisions read the laid, natural values.
- **E2 (7.6).** Two terms in `foliage.wgsl`:
  - the underside's mirror: a sun glint and a sky sheen, tinted by the back tint;
  - the margin band's glint on either face, tinted by the colour the band shows.

  Energy rule: each term takes its share from the face's diffuse of the same light. The front and back glints are never on one face. Within the band, the rim's share adds to the face's, capped at all of it. One `roughness` row serves both terms. The organ block zeroes both reflectances. `set_material` resets them to none.

## Proof (measured)

| Claim | Run | Result |
|---|---|---|
| Off is the base species | `supernatural::tests::a_family_with_the_group_off_draws_its_base_species_to_the_byte` | Every row set with the group off: mesh hash equal to the plain beech's. On: differs. |
| Passed species unchanged | `space_hash` at `33bc84cf` and HEAD (all four engine presets at ages 40 and 80, seeds 1 and 7) | 16 of 16 hashes identical. |
| Neutral pixels (geometry apart) | `space_beech` stills (whole, spray, crown; beech 20, seed 1), pre-E2 `1192044d` against HEAD | 3 of 3 PNGs byte-identical. |
| Neutral and per-face | `tests/leaf_mirrors.rs` | Off, and on at zero: 0 pixels apart, front and back. Silver changes the back only. The rim changes both faces. |
| Organ takes no leaf term | `scene::frame::tests::the_flowers_take_none_of_the_leaf_s_mirrors` | Both reflectances are zero in the organ block. |
| S1 on the CPU | `supernatural::scale_tests` (0.1, 1, 100) | Nodes, radii and grown wood are exactly natural times s, with topology, frames and keys unchanged. Envelope height is times s with its shares unchanged. The field's wood is times s. Leaves keep their size; their count rises with s. |
| S1 on the device | `resident_tests::the_real_path_matches_the_cpu_path_at_every_scale` (beech 20, seed 1) | At 0.1 and 100: all 3,546 nodes and 700 runs matched, and the leaves matched (766 at 0.1, 159,008 at 100) with none over bound. The screen is taken at the scale. |
| Contact precision | `scale_tests::contact_precision_at_every_scale` | See below. |

Contact precision on the beech at age 12, seed 1. The worst position error after packing, against the smallest leaf's 0.085 m:

| Scale | Code step | Worst error | Share of the leaf |
|---|---|---|---|
| 0.1 | 8.6 µm | 3.4 µm | 0.00 % |
| 1 | 86 µm | 34 µm | 0.04 % |
| 5 | 0.43 mm | 0.17 mm | 0.20 % |
| 100 | 8.6 mm | 3.4 mm | 4.0 % |

The step is the box's extent over 65,535, so it grows with the tree's size times the scale. On a 30 m beech at scale 5, the Two Trees' size, it is about 2.3 mm (inferred). At the ceiling of 100 on a mature tree, the error approaches a quarter of a leaf (inferred).

## Gate (measured)

- Render half, `cargo test --profile ci -p telperion-render --no-fail-fast` under the GPU lock: one failure, `conformance::the_renderer_library_never_names_a_family` (`set_family` named `telperion_core::presets::Family`), fixed and re-run green.
- The rest, `cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast`: three targets failed on the catalogue's pins.
  - `catalogue_identity` was re-pinned. With the group stripped from the HEAD texts, each text equals the base `33bc84cf`'s to the byte, and so does every other row's single refusal.
  - The sweep's `HELD` list takes the group's 48 rows.
  - The frozen fn-19 protocol omits the group.
  - The age walk is declared in `docs/generation-limits-inventory.json`.

  Each was re-run green, focused.
- `npm test`: the browser presets file was regenerated (`TELPERION_WRITE_REFERENCE=1`), then 13 files and 143 tests passed.

## Stills (viewed)

`raw/stills/` holds four images, each with the natural tree on the left and the group on the right: `silver-under`, `silver-whole`, `gold-limb` and `gold-whole`. The beech is 20 years old at seed 1.

- **Silver.** The undersides read grey-silver against the sky from below. In the whole view the crown greys a little.
- **Gold.** On the limb close-up, the rim reads as a gold edge on every leaf. In the whole view, leaves under a pixel turn gold over their whole face. The band's width is pixel-based (`foliage.wgsl`, the margin smoothstep), so a sub-pixel leaf is all band. That is today's margin rule, now made visible by a bright colour.

## Decisions taken inside the design

- The override path is `/supernatural` plus the natural path. The flowering share is per age, `/supernatural/flowering/age<k>`, because the natural share is per age.
- `marginWidth` is not overridden: section 7.1 lists the margin colours only.
- One roughness row serves both mirrors, as "roughness is chosen by a row" (7.6).

## Open for the host

1. **The gold rim at distance.** A sub-pixel leaf is drawn all band, so Laurelin reads as a gold crown at whole-tree distance, not as green leaves with gold rims (see `gold-whole`). This is a far-draw question for the host.
2. **Material lengths do not scale.** `shootRadius`, `matureRadius` and `matureSpan` are metres of wood radius that the renderer reads. At scale 5 nearly all wood is past the mature radius. They are draw rows, outside S1's geometry, and are recorded here unchanged.
3. **The GPU ring-position domain.** `compact::qualified_ring` admits rings within 64 m. A family that seats leaves on its wood and stands taller than 64 m is refused "ring positions" on the real path. The beech seats none, so the Two Trees are unaffected (inferred from `resident.rs`).
4. **A debug overflow in the engine.** The dev-profile test of the beech at 80 panicked in `telperion-space/src/shed.rs:228` with "attempt to subtract with overflow". The ci profile does not check overflow, and the code predates this task.
