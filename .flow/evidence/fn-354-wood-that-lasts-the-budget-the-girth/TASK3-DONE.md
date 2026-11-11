# fn-354 R6: the girth and the sag as product, by the builder, 11 Nov 2026

PRODUCT-DESIGN.md is built in five commits on `fn-354-wood-that-lasts`: the rows, the state and the yearly pass, the sag values, the stem's easing, and the docs. The narrow tests are green except one: the ground-contact check draws 14 flowers below the ground on Telperion at 82. The yearly pass takes 13 to 19% of the grow, which is over the design's tenth, so this task is returned with the profile below and not marked done. The oak draws the probe's G2 trees to within 1.4% on every measure. The stills of the stem's easing are the same as G2's to 0.08% of their pixels, because the steps the verdict saw sit where a lateral leaves its bearer, and the design keeps those from easing.

Tier: product code, built by Opus to the host's design (model: claude-opus-5-5). Narrow tests only. The full gate and the codex review come at the spec's end.

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review of the spec's diff at its end)

## Commits

| Step | Commit | What |
|---|---|---|
| 1 | `a3e16468e` | The rows: `pipe` renamed `primary`; `sapwood` and `sapwood_rings` added; `exponent`, `ripening`, `secondary`, `retained`, `leaf_girth`, the coarse pipe row and everything only they reach deleted |
| 2 | `3c9b20ad6` | The state `[T, S]` on each phytomer, the yearly pass (`girth::lay`), `girth::leaf`, the last year's layout, the settle reading T, relay joins, fading shares |
| 4 | `c3e3b861c` | The sweep eases a drop in section along one stem (committed before step 3 by a slip of the shell) |
| 3 | `55fbbadba` | Sag 4ρg/E on every growth stage of every species |
| 5 | `2fd847851` | docs/pipeline.md and docs/tree-space.md |

## For the host to decide

1. **The pass's cost is over the design's tenth.** The pass cannot share the balance's walk. The balance runs at the start of a year over the open axes, before that year's growth. The wood reads that year's layout after the growth, and a settle at any age reads the last year's wood. On the grower's threads the pass still costs:

   | oak, seed 1001 | threads | grow, s | `Pass(Wood)`, s | share | load |
   |---|---|---|---|---|---|
   | 40 | 1 | 3.24 | 0.42 | 13% | 6 |
   | 120 | 1 | 76.0 | 13.6 | 18% | 6 to 47 |
   | 300 | 1 | 281.8 | 53.2 | 19% | 47 |
   | 120 | 4 | 59.5 | 9.6 | 16% | 47 |
   | 300 | 4 | 191.9 | 32.8 | 17% | 32 to 47 |

   - The full phases are in `raw/t3-census.log` and `raw/t3-census4.log` (`early three ... cycle`, census on).
   - The pass walks every held phytomer each year, the closed subtrees' too.
   - A closed subtree never lays wood again, since no leaf stands above it. Skipping it would leave its T exact, but its S would stop retiring, and S is in the digest that the paused and one-call trees are compared by. Whether to do that is not mine to decide.

2. **The ground check: `ground_reach::nothing_drawn_passes_below_the_ground`.**
   - Telperion at 82, seed 1, draws 14 flowers below the ground, the lowest at -0.049 m. Its wood and leaves stay above the ground.
   - Laurelin at 80 passes.
   - Inferred cause: the beech's ages had sag 0 before this task and now have 4.1e-6, so Telperion's limbs, on internodes 2.6 times the beech's, now bend. Not checked against a run without the sag.

3. **The easing does not reach the steps the verdict saw.**
   - In the trunk run of the oak at 300, seed 1001, the curve eases a drop of 0.72 m to 0.34 m over 4.8 to 6.0 m. Read from the mesh's curve points.
   - Every visible rim in the close views is where the run passes from a bearer into a lateral, and the design keeps that from easing:
     - the flat-topped cylinder under the joint;
     - the bands on the limb above it.
   - The product's stills differ from G2's on at most 0.08% of their pixels (ImageMagick AE, fuzz 10%).

4. **Tests whose promise needed a new form.** Each still checks what it promised, and each is a judgment I made:
   - **The relay test:** a stem may now stand over the node below by one primary section, since a phytomer's own primary section is not carried below it.
   - **The lever beam test:**
     - Its pinned fn-200 bends rested on a fixture girth with exponent 2.2 and ripening 10, rows that are now deleted.
     - It now compares against the small-deflection beam computed on its unbent tree. They agree within 1e-4 relative.
     - Its sag went from 1e-6 to 1e-9, because the branch is now primary wood only.
   - **The walk fixture:** it takes the oak's sapwood and a leaf of 0.8 m² a node. That leaf was chosen so its trunk's base stands at its old 0.21 m (now 0.213 m), because the sags of every test on it were set for that girth.
   - **The supernatural scale test** reads the beech at 20, not 12. At 12 a tenfold shrink no longer loses a twig: 1,018 leaves at either scale, against 2,652 and 2,631 at 20.
   - **The group-off test** turns the group on without the legendary 0.2 m internodes. With them, the beech at 12 bends into the ground and is refused.
   - **The shed-certain tests** take species that lay no sapwood and bend none.
   - **The pipe-model sum test** (`form.rs`) is deleted with the model.

## Built as written, with these readings

- **The phytomer's leaf:** `leaf(phytomer, year)` is the grower's deposit, `leaf_area × scale × size`, in the year the phytomer was laid, and nothing in later years. The sketch deposits only this year's phytomers.
- **Relay joins:** a relay's leaf enters its bearer at the tip, as in the probe. Its join stands at the bearer's radius at the join's node, on both sides.
- **The rough layout:** it is grown wherever an age lays sapwood, so a tree under neutral light has a scale for its leaf. Where it is grown, what will be shed is grown too.
- **The sag load's leaf stand-in:** it is now `primary × scale`, with no ripening.

## Digests repinned

| Pin | Before | After | Why |
|---|---|---|---|
| closed-form row hashes (40 rows) | over 9 fields | over kept, count and reach | the section and mass columns were deleted (step 1) |
| `every_preset_is_the_family_it_was` | 5784278485669999534 | 9516092986113572468 | rows renamed, added and deleted (steps 1 and 3) |
| `every_walk_is_the_walk_it_was` | 10352803689928916410 | 11072528967666382666 | same |
| `every_override_reads_and_writes_as_it_did` | 1402438456769190590 | 5044572365911084041 | the supernatural primary rename, the coarse pipe deleted, sag |
| `every_refusal_is_the_refusal_it_was` | 4291284850864067711 | 6914840421197035565 | sapwood refusals added, the pipe model's deleted |
| `cycle_cost_tests` PINNED, 8 pairs | listed in the file's history | listed in the file | every tree moved under the rule and the sag |

No pinned trunk median at fidelity above 0 exists among the tests run, so there are no fidelity numbers to list.

## Tests (commands)

- `telperion-heavy 16 --threads 4 -- cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core --no-fail-fast -j 3 -E 'not test(every_setting_changes...) & not test(the_largest_tree_and_the_palm...)'`
  - 753 of 761 passed.
  - Since then I fixed the failures and reran each failing set alone, all green, except `ground_reach`.
- The long walk alone: passed in 370 s. The new walks' steepest steps:
  - `sapwood` 4.41
  - `sapwood_rings` 0.01
  - `primary` 3.66
- The repeat test alone: passed in 149 s.
- Both guards (`generation_limit_guard`, `compile_size`): green.
- `cargo fmt --all --check`, `npx tsc --noEmit` and `npm run catalogue:check`: green.
- vitest: 218 passed after step 1. Not rerun after steps 2 to 5.
- Not run:
  - the render crate's lib tests, beyond `compile_size`;
  - the full gate, which comes at the spec's end.

## Measured

**Cost.** One thread, seed 1001, pinned to one core, through the scoreboard's cost stage. The product ran under heavier load than the base, so the seconds are not a fair comparison.

| oak | before (933a318c6), s | before, GiB | product (2fd847851), s | product, GiB |
|---|---|---|---|---|
| 40 | 1.61 | 0.41 | 2.61 | 0.44 |
| 120 | 42.9 | 3.29 | 49.2 | 3.47 |
| 300 | 141.7 | 1.98 | 166.1 | 2.32 |

- Before: load 7 to 4.
- Product: load 54 to 10.
- The pass's own share is in the first table above.

**Panel.** The oak at 40, 80, 120, 200 and 300 on seeds 1001, 1007 and 1012, label `fn354-product`.

- **Sheets:** `product/oak-whole.jpg`, `oak-bare.jpg` and `oak-close.jpg`, with columns base, G2, product.
- **Measures:** `product/VARIANTS.md`.
- **Run records:** `product/runs/`.

**Differences between G2 and the product:**

- **Trunk at 1.3 m and height:** equal on all 15 trees.
- **Spread:** three trees differ by 0.1 m.
- **The three largest limbs:** three trees differ by 1 cm at one limb.
- **Drawn leaves:**
  - at 40 and 80, 0.4 to 1.4% fewer;
  - at 120, up to 0.3% fewer;
  - at 200 and 300, within 0.05%.
- **Drawn leaf area:** up to 1.5% less.
- **Drawn nodes and phytomers under 0.2 mm:** within 100 of G2.
- **Inferred cause:** the sag load's leaf stand-in, which no longer ripens. Young shoots weigh a little more, so they are posed a little differently. Not checked by a run.
- **The easing** changes the curve where the run follows one stem. That shows in no tile at these views (above).

**What the sheets show.** These are descriptions, not verdicts.

- The product's columns draw the G2 trees: the same outlines, trunks and crowns at every age and seed.
- At 300 the close view still shows the wide lower cylinder with a flat rim. Above it is a narrower leaning stem in banded sleeves.
- At 120 the joint view still shows the flat-topped cylinder under the fork.

## Follow-ups (not built)

- Phytomers that lay nothing could be skipped by the pass (a cost question, above).
- The vitest suite rerun on the final tree.

## Friction

Two entries were added to FRICTION.md: the interim girth step 1 needed, and an 8 GB narrow run killed at the core's heavy tests.
