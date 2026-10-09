# fn-219: Telperion's sheet, scale, contact and cost

The raw stills are in `raw/` (ignored). The final round is `raw/r14/`. Each sheet is `sheet-92-<seed>.png`:

- the top row has the group off, and the bottom row has it on;
- the columns are the whole tree (hero pose), the mid view (the crown's edge on the camera's side, from 0.4 of the height) and the close view (the outermost whole floret half-way up, from 0.7 m).

The stills come from this command, run from the worktree root. It is `space_laurelin` generalised to both trees:

```
target/ci/examples/space_two_trees telperion <out> 92 <seed>
```

I viewed every still of every round: 14 rounds and one diagnostic. The final sheets are seeds 1, 7 and 42, where 42 is the preset's default seed. Astra's readings are in `astra/`.

## The values (R1)

`presets/telperion.values` copies the `european-beech` preset's rows with one change: `/age` is 92. Then the supernatural group follows. The old scaffold Telperion (a 148 m envelope with the writhe bias) is gone.

**Off is the beech.** `supernatural::tests::the_two_trees_with_the_group_off_are_the_beech` holds Laurelin and Telperion, each with the group off, to the beech's mesh to the byte at one age. With the group on, each hash differs.

**His age is the release's ceiling, not the brief's choice.** The brief asks for the beech's architecture made older. Four limits stand above 80, each met by a different failing run:

| Age | Result |
|---|---|
| 92 | the oldest the browser grows: the Wasm route builds him (`harness/parity.test.ts`, native and Wasm byte-identical) |
| 93, 95, 100, 110 | refused in Wasm: `the tree keeps more than 4,000,000 phytomers, more than a browser prepares in its 4 GB Wasm memory` (`WASM_KEPT`) |
| 120 | at seed 42 the device route runs out of memory (`DeviceRefused ... Out of Memory`, RTX 3080, 10 GB); the beech alone at 120 is refused the same way |
| 140 | the on tree is refused at draw: the wood points buffer needs 330 MB, above the device's 256 MB |
| 160 | refused at grow: `engine phytomer budget` |

Rounds 1 to 11 were judged at 120, and rounds 12 and 13 at 110. Round 13 (age 110, scale 2.2, these colours and blossom) passed with the host and with two Astra draws (`raw/r13`, `astra/r13*.md`), and the device route prepares it at seeds 1, 7 and 42. `npm test` then showed the Wasm bound, and round 14 moved him to 92. The code says fn-209's fine path lifts that bound; the age is the first open item below.

| Group row | Value | Why |
|---|---|---|
| `scale` | 2.4 | The elder and the taller: 47.2 to 50.0 m against Laurelin's 44.5. At 120 years, scales of 3 and 2.5 read twiggy (`raw/r1`, `raw/r4`), because the leaves keep their natural size; at 92, 2.4 is a little more open than 2.2 was at 110. |
| leaf front | 0.0055/0.022/0.007 | Dark green. Halving the front albedo darkened the drawn front by only about 12 % (measured on the close stills, rounds 7 to 11): the beech's transmission and sheen, natural rows the group does not override, set the rest. |
| leaf back | 0.018/0.03/0.022, `backReflectance` 0.7, tint 0.95/0.93/0.86, `roughness` 0.12 | Silver beneath. Every view but the close one looks up into the crown and sees mostly undersides. In `raw/diag-backs`, tinting the underside red turns the whole crown red, so the underside sets the crown's colour at distance. A dark albedo under a strong, sharp, warm-tinted mirror reads silver where the leaves turn without washing the crown grey; the warm tint cancels the sky's blue. |
| `flowering/age14` | 1.0 | Every outermost short shoot. Flowering on ages 9, 12 and 14 read as hoarfrost beaded along each shoot and doubled the build (`raw/r5`). |
| `bloom` | 9 florets on a 0.13 m rachis, droop -1, spread 70, open 100, five petals 31 x 17 mm widest at 0.5, cup 0.4, curl 0.25, 12 stamens | Upright candles standing out of the leaves, which hide about half of fn-234's hanging blossom. Clusters hanging inside the leaf mass read as a speckle at the whole-tree view (`raw/r6`). |
| petal albedo (natural rows) | front 0.75/0.77/0.81, back 0.66/0.68/0.73 | fn-234's white (about 0.8) with a cool cast. Dormant with the group off, because no age flowers then. |
| eye | 0.45/0.5/0.58 over 0.3 of the petal | A grey centre, so a floret shows a form against the white. |

## Judgment (R2)

**Host reading (round 14, seeds 1, 7 and 42).**
- **Off:** an old, open-grown beech with a broad dome and spreading limbs. Seed 42 is the most convincing; seed 1's thin lower framework reads old rather than very old. At 110 (round 13) the crown and bole were heavier and read as the very old tree the brief asks for.
- **On, whole tree:** a silver-green crown with white across all of it. It reads white-silver at the whole-tree view and is still a tree.
- **On, mid view:** white candles standing through dark sprays.
- **On, close view:** dark-green fronts, undersides turned silver, white candles. The florets are crowded and read as papery white masses close up.
- **The harness Telperion:** the new tree is a real tree first. Today's harness Telperion is a 148 m envelope with default brown and green.

**Astra's readings**, from one prompt on the seed 1 and 7 sheets. From round 11 on, the prompt states each tree's measured size and Laurelin's height, because each view is framed to its own tree. Round 14's prompt also states both trees' ages:

| Round | Off | On | Tree first | Overall | What changed after it |
|---|---|---|---|---|---|
| 7 (120, scale 2) | PASS | FAIL: fronts medium green, undersides flat blue-grey, petals solid clumps | PASS | DISAGREE | darker fronts, lighter neutral undersides, narrower curled petals |
| 10 | FAIL: not *very* old (on the off stills she passed in round 7) | FAIL: fronts medium, undersides muted blue-grey | PASS | DISAGREE | a warm-tinted, stronger mirror; darker fronts; ages past 120 probed and refused |
| 11 | PASS | PASS | PASS | **AGREE** | second draw: AGREE |
| 13 (110, scale 2.2) | PASS | PASS | PASS | **AGREE** | second draw: AGREE |
| 14 (92, scale 2.4) | PASS: "mature, old beeches" | PASS | PASS | **AGREE** | none |

Astra's round-14 polish notes are optional and not applied:
- blossom at 85 to 90 % of its size, and less overlap, to loosen the crowded clusters;
- slightly brighter, more neutral undersides.

**Seed 42's close view** in rounds 12 and 13 framed a leaf 0.7 m from a floret it hid, because the example's close camera picks the outermost floret without testing that it is visible.

## Contact error at his scale (R3)

`supernatural::scale_tests::telperion_contact_at_his_scale` (ignored, run by name) builds the shipped preset (age 92, seed 42, scale 2.4) with every leaf's transform recorded. It holds each packed position to half its box's step.

| Tree | Leaves | Code step | Worst contact error | Share of the smallest leaf |
|---|---|---|---|---|
| Telperion, 92, scale 2.4 | 16,584,935 | 1.50 mm | 0.47 mm | 0.56 % |
| Laurelin, 80, scale 2.5 (fn-218) | 11,431,678 | 1.36 mm | 0.42 mm | 0.50 % |

fn-125's screen for leaves is "within two quantisation steps of the reference box plus float slack". The worst error is under half a step, so it is inside that screen.

## Cost (R4)

All numbers are measured once on this machine with 4 rayon threads.

| Route | Seed | Off (the beech at 92) | On (Telperion) |
|---|---|---|---|
| CPU pipeline, grow to mesh (`space_two_trees`) | 1 | 3.4 s; 3.85M leaves | 8.0 s; 9.05M leaves, 10.38M florets |
| the same | 7 | 6.2 s; 6.87M leaves | 14.3 s; 16.18M leaves, 17.75M florets |
| the same | 42 | 6.0 s; 7.03M leaves | 13.9 s; 16.58M leaves, 16.96M florets |
| device, resident (`prepare_timing`, the harness's `setTreeGpu` route) | 42 | 4.2 s (`european-beech` at 92), peak RSS 469 MB | 4.5 s, peak RSS 479 MB |
| the same | 7 | | 4.3 s, peak RSS 444 MB |
| the same | 1 | | 4.0 s, peak RSS 391 MB |

- **On the CPU route** the group costs about 2.3 times the beech at the same age; the leaves rise with the scale (S1), and the florets are one candle of nine on every outermost short shoot.
- **The device route** prepares him in about the beech's time. fn-218's fixed cost of about 140 s on this route did not appear here, so the tree's own cost is readable there. That F4's merge removed it is inferred, not checked.
- **Peak RSS** is the host process's. The device's own memory is the limit that refuses 120 (above).

## Gate (measured, on `52ab0912`)

Every run used `RAYON_NUM_THREADS=4 CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4`. The logs are in `raw/gate/`.

- **Render half,** `cargo test --profile ci -p telperion-render --no-fail-fast` under the GPU lock: 207 passed, 0 failed, 36 ignored.
- **Workspace half,** `cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast`: 1,067 passed, 0 failed, 49 ignored.
- **`npm test`:** 13 files, 143 tests passed. Its pre-test Wasm build confirms that `presets.generated.ts` is what the Rust presets render. Telperion's native and Wasm node buffers are byte-identical, and the field test grows him.
- **Before the final gate.** The first gate (at age 110) failed in three places. Two were `material_detail` lists, which now name Telperion beside Laurelin. The third was `suite::outline`, which walked Telperion to the birch as two presets without an engine species. `npm test` then failed on the Wasm bound, which set his age (above).
- **Earlier focused runs:** the sweep's held list (Telperion's scaffold rows held, the silver underside rows walked), the catalogue's family and walk digests (re-pinned for each value change), and `space_presets`.

## Open for the host

1. **His age.** The brief's "older" stands at 92 against Laurelin's 80, the oldest the browser grows (`WASM_KEPT`). Round 13 at 110 is ready, judged and timed on the device, for when fn-209's fine path lifts the Wasm bound: set `/age` to 110 and `/supernatural/scale` to 2.2.
2. **Dark fronts.** Darker upper faces than these need the leaf's transmission and sheen under the group, which it does not override. Astra lists them as optional polish in every passing round.
3. **Astra's optional polish**, not applied: smaller, less overlapping blossom; slightly brighter, more neutral undersides.
4. **The close camera** of the sheet example can frame a hidden floret (seed 42, rounds 12 and 13).
