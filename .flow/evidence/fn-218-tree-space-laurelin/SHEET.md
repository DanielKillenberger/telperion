# fn-218: Laurelin's sheet, scale, contact and cost

The raw stills are in `raw/` (ignored). The final round is `raw/r8/`. Each sheet is `sheet-80-<seed>.png`:

- the top row has the group off, and the bottom row has it on;
- the columns are the whole tree (hero pose), the mid view (the crown's edge on the camera's side, from 0.4 of the height) and the close view (the outermost whole floret half-way up, or on the off tree the outermost leaf, from 0.7 m).

The stills come from this command, run from the worktree root:

```
target/ci/examples/space_two_trees laurelin <out> 80 <seed>   # space_laurelin when this sheet was made
```

I viewed every still on the final sheets: seeds 1, 7 and 42, where 42 is the preset's default seed.

## The values (R1)

`presets/laurelin.values` copies the `european-beech` preset's rows with one change: `/age` is 80. Then the supernatural group follows.

**Why age 80.** At 40 the beech is a 10 m sapling (`raw/r1`). At 80 it is a broad dome 18 m tall and 19 m across, the "old beech" the brief asks for.

**Off is the beech.** `supernatural::tests::the_two_trees_with_the_group_off_are_the_beech` (named `laurelin_with_her_group_off_is_the_beech` when this sheet was made) shows that Laurelin with her group off hashes to the beech's mesh to the byte at the same age. With the group on, the hash differs.

| Group row | Value | Why |
|---|---|---|
| `scale` | 2.5 | Broad and nearly as tall as Telperion: 44.5 m tall and 43 x 48 m across at seed 1. At 3 the crown read twiggy, because the leaves keep their natural size. At 2 it is only a big beech's height (`raw/r3`, `raw/r4`). |
| leaf front / back | 0.03/0.1/0.02, 0.06/0.12/0.035 | Young green. The olive tones of rounds 3 to 7 were replaced after Astra's reading of round 7. |
| margin, `marginReflectance`, `roughness` | 0.75/0.52/0.04, 0.9, 0.25 | The thin gold rim, with its glint. |
| `flowering/age14` | 0.7 | Racemes on seven in ten of the outermost short shoots. At 1.0 on ages 12 and 14, the whole crown read gold (Astra, round 6). |
| `bloom` | 14 florets on a 0.24 m rachis, droop 1, spread 30, open 9, petals 30 x 10 mm widest at 0.9, cup 0.6, curl 0.45, 5 stamens | Hanging horn florets: a narrow throat flaring into a mouth. |
| petal albedo (natural rows) | front 0.75/0.32/0.015, back 0.65/0.27/0.015 | Gold, not straw. fn-234's (0.85, 0.55, 0.05) read lemon on the close view. These rows are dormant with the group off, because no age flowers then. |
| eye | 0.7/0.2/0.01 over half the petal | An amber throat, for the flame. |

## Judgment (R2)

**Host reading.**
- **Off:** an old, open-grown beech with a broad dome and spreading lower limbs. Seed 7 is the most cohesive.
- **On, whole tree:** a fresh green crown with gold through it.
- **On, mid view:** strands of gold racemes hang from the sprays.
- **On, close view:** laburnum-like strands of amber-throated golden horns, with each leaf edged by a fine gold line.
- **The harness Laurelin:** the new tree is a real tree first. Today's harness Laurelin is a 132 m envelope with default brown and green.

**Astra's readings**, from the same prompt on the same sheets for seeds 1 and 7:

| Round | Off | On | Tree first | Overall | What changed after it |
|---|---|---|---|---|---|
| 6 | PASS | FAIL: whole-tree gold overwhelms the green; petals pale lemon, star-like | PASS | DISAGREE | flowering on age 14 only; 14 florets; narrower, longer horns; deeper gold |
| 7 | PASS | FAIL: yellow-green crown at whole-tree distance | PASS | DISAGREE | flowering 0.7; fresher leaf; open 9, curl 0.45, petals 30 mm |
| 8 | PASS | PASS | PASS | **AGREE** | none |

Astra's round-8 polish notes are optional and not applied:
- 10 % less rim intensity;
- 10 to 15 % more variation in raceme length;
- shorter lowest laterals on seed 1. These come from the beech's own architecture.

## Contact error at her scale (R3)

`supernatural::scale_tests::laurelin_contact_at_her_scale` (ignored, run by name) builds the shipped preset (age 80, seed 42, scale 2.5) with every leaf's transform recorded. It holds each packed position to half its box's step.

| Tree | Leaves | Code step | Worst contact error | Share of the smallest leaf |
|---|---|---|---|---|
| Laurelin, 80, scale 2.5 | 11,431,678 | 1.36 mm | 0.42 mm | 0.50 % |
| beech 12, scale 1 (fn-217's row) | 654 | 0.086 mm | 0.034 mm | 0.04 % |

fn-125's screen for leaves is "within two quantisation steps of the reference box plus float slack". The worst error is under half a step, so it is inside that screen.

## Cost (R4)

All numbers are measured once on this machine, 4 rayon threads, seed as shown.

| Route | Seed | Off (the beech at 80) | On (Laurelin) |
|---|---|---|---|
| CPU pipeline, grow to mesh (`space_laurelin`) | 1 | 4.0 s; 2.49M leaves | 8.3 s; 6.13M leaves, 8.58M florets |
| the same | 7 | 5.4 s; 4.41M leaves | 12.8 s; 10.83M leaves, 14.64M florets |
| the same | 42 | 4.5 s; 4.65M leaves | 11.1 s; 11.43M leaves, 14.76M florets |
| device, resident (`prepare_timing`, the harness's `setTreeGpu` route) | 42 | 144.5 s (beech 80) | 139.5 s; peak 5.46 GB |

- **The device route has a fixed cost.** The shipped beech at 40 (240k leaves) also takes 136.9 s on the device route. So Laurelin's own cost there cannot be separated from the route's. On the CPU route the group costs about 2.1 to 2.5 times the beech at 80, and the times move by up to a third from run to run (seed 1 off took 2.7 s and 4.0 s in two runs).
- **Florets.** One raceme stands on each flowering node of age 14, and the scale does not change their number. The leaves rise with the scale (S1).

## Gate (measured)

- **Render half,** `cargo test --profile ci -p telperion-render --no-fail-fast` under the GPU lock. One failure: `conformance::every_shipped_family_renders_through_the_one_path`. Its drawn crown counts florets beside leaves (26,187,743 = 11,431,678 + 14,756,065), and Laurelin is the first shipped preset that flowers. The check now adds the tree's florets (`839c88b0`). Re-run focused: green.
- **Workspace half,** `cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast`. Four failures, each re-run focused and green after `238e3ca9`:
  - `family::tests::every_row_past_its_bound_is_refused_on_every_family`. While the group was on, `validate` judged only the applied family, so a natural `/bloom` or leaf colour row that an override stands in for was never judged. `validate` now also judges those natural rows, after the applied family.
  - `suite::growth::limits::natural_bias_is_independent_of_disabled_effects`. Laurelin no longer states the scaffold's bias rows (fn-217 decision 11), so the assertion is now on Telperion only.
  - `material_detail` (young wood, canopy). Laurelin carries the beech's rows, so both lists name her.
- **Earlier focused runs:** the sweep's held list and the catalogue's family and walk digests were re-pinned in `14bd745f`. With Laurelin as an engine end, `every_walk_is_the_walk_it_was` takes 90 to 99 s. Its time before the change was not measured.
- **`npm test`** (after `npm run wasm:build`): 13 files, 143 tests passed. Laurelin's native and Wasm node buffers are byte-identical, and the field test grows her.

## Open for the host

1. **Harness default seed.** The default seed (42) builds 11.4M leaves and 14.8M florets, which takes about 11 s on the CPU route. Whether the harness should open her at another seed or age for cost is a question for the host.
2. **Device-route cost.** The device route's fixed cost of about 140 s makes her device cost unreadable.
3. **Astra's optional polish.** Astra's round-8 suggestions are not applied: 10 % less rim intensity and more variation in raceme length.
