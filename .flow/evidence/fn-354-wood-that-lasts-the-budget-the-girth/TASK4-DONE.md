# fn-354 R5: the leaf account, by the builder, 11 Nov 2026

LEAF-ACCOUNT.md's design is built in one commit, `bfe2fd92f`. `ground_reach` is green. The oak and the beech are unmoved on every measure. The spruce keeps its shape with 7% fewer new shoots and a trunk twice as thick. The Two Trees' limbs are off the ground. Three tests are red, and each needs a call from the host: two one-cycle reference tests on the spruce, and the drawing's retention test. The task stays `in_progress`, and task 5 has not been started.

Tier: product code, built by Opus to the host's design (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review of the spec's diff at its end)

Baseline: green via handoff (narrow space and core 760 of 762 at `0583ece15`, `ground_reach` red on purpose; vitest 218).

## What was built

- **`girth::leaf(state, p, year, fade)`** is `leaf_area × scale × size × clamp(leaf_years − age, 0, 1) × fade`. `girth::bearing` gives the first phytomer of an axis that still bears leaf, since an axis's phytomers are laid in order.
- **The deposit** (`grow/draw.rs`, `sketch_end`) puts down the year's phytomers first, in the order laying gave them, and then every held axis's older phytomers that still bear leaf. The second walk runs only where some age's `leaf_years` is over 1. Where it is not, no older phytomer bears leaf, and the deposit is today's to the bit.
- **The wood pass** (`girth::lay`) reads the same `leaf`. It skips a closed axis only when its subtree bears no leaf that year. A closed spruce subtree whose needles still stand lays wood as any other.
- **The fade** is `Retirement::drawn`: a closed subtree kept by a lateral that sheds is drawn at `shed::faded` of its living at that year, times its bearer's share. Everything else is whole. It is computed only where a leaf outlives its year (`Grower::fades`).
- **`leaf_years`:**
  - spruce 5.5 (Muukkonen & Lehtonen 2004; estimated in form);
  - birch 1 ("deciduous", Dox et al. 2020, already cited in its value file);
  - ordinary 1 (estimated: a deciduous broadleaf);
  - oak, beech and the Two Trees 1, as before;
  - palm never, as before.

  The values are in `spruce.rs`, `birch.rs` and the three value files, each with its source.
- **The walk tree** takes 1 on every age. The new walk `light: leaf_years` runs from 1 to 6.
- **The Two Trees' leaf:** in `Family::applied` (`supernatural.rs`), beside the internode and primary overrides. The natural size is reachable there as `self.canopy.size`, the family's own row before the group's rows are laid over it. Where the group sets `canopy.size`, every age's `leaf_area` is multiplied by `(size / natural)²`: 4.37 for Telperion, 4.76 for Laurelin.
- **Docs:** `docs/pipeline.md`, the `leaf_years` doc comment and its row text, and the regenerated `presets.generated.ts` and `parameters.generated.ts`.

## For the host to decide

1. **The one-cycle reference tests on the spruce are red.** They are `grow::cycle::tests::the_working_set_and_retirement_leave_the_tree_as_it_was` and `grow::cycle::paused::tests::the_tree_at_a_year_is_the_horizons_prefix`. Both answer `girth apart None` on the spruce, and the oak and the beech pass. The spruce failed in the same way before its value moved from never to 5.5. The reference retires nothing and computes no fade, so a subtree the product has shed and let go still deposits its needles in the reference, and lays wood with them. Before this task, closed wood bore no leaf, so the reference and the product agreed. The test's promise, that retirement leaves the tree as it was, cannot hold for an age whose leaf outlives closure unless the reference sheds as it grows. That needs every presence assigned each year, which the grower does not have. I have not changed either test.
2. **The fade inside a closed subtree is an approximation.** `Retirement::drawn` knows the living only of the roots it evaluated: closed subtrees that closed beside an open axis. An axis inside such a subtree that closed in the same year as its root stays whole until the root fades. The settle gives it its own fade, so a spruce twig of shedding 1 under a limb of shedding 3 is drawn by the settle a year or two before its needles leave the grower's account. Getting the settle's value would need each evaluated subtree's inner livings kept, or the whole tree's fades each year. Not built.
3. **`suite::retention::a_leaf_stays_only_where_its_wood_keeps_it` is red.** Its premise is that `leaf_years` moves only the drawing, so the beech that keeps its leaves for ever bears every leaf the others keep, bit for bit. The grower now reads the row, so the tree changes with it and leaves move ("3: a leaf moved").
   - I tried holding the tree still: no sapwood, shade and balance hazards 0.
   - Then the beech at 15 bears 9 leaves in all, and none at 1.5 years, because the bearing bound is a share of the root's radius.
   - The fixture was reverted. The test needs a new form, or its promise is withdrawn; that is your call.
4. **A supernatural canopy size can now refuse a family.** `leaf_area` is bounded at 5 m², so a beech-leaved family with a canopy size above about 56 times its natural size is refused by name. The legendary test fixture's 250 was. Its group-off test now draws "on" at Telperion's 2.3.

## Smaller choices I made

- The walk over `leaf_years` runs from 1 to 6: from a deciduous year to just past the spruce's 5.5.
- The canopy multiple applies only where the natural size is above 0. At 0 no leaf is drawn and the ratio has no value.
- A closed axis is skipped when its own bearing range is empty and every child returned no leaf.
- In the deposit, an older phytomer whose leaf is 0 is not deposited. The year's phytomers are deposited as before, zero or not, so the lattice's extent is unchanged where nothing outlives its year.
- `leaf_years_mix_from_never_by_the_share_they_keep` (chain.rs) now sets the spruce's leaf years to never in the test, since the spruce no longer has that value. The walk it checks is unchanged.
- `grow/draw.rs` is 421 lines, over the 400 the code rules ask.
- `floor.rs`'s measuring `bearing_area` still uses its own filter, unfaded.

## Tests

| Test | Result |
|---|---|
| Narrow space and core (16 GB, `-j 3`, the long walk and the repeat test excluded) | 757 of 766 |
| Reruns after the fixes and repins | the 7 fixed ones green |
| New tests: the leaf formula, a closed subtree in leaf is laid, the deposit is every held phytomer's leaf, the supernatural leaf multiple | green |
| `ground_reach::nothing_drawn_passes_below_the_ground` | green (119 s) |
| Both guards, `cargo fmt --all --check`, `npx tsc --noEmit`, `npm run catalogue:check` | green |
| vitest | 218 passed |
| Red, for the host | the three tests above |

The new `light: leaf_years` walk held the stated bound in `light_changes_the_tree_by_degree` (46 s, in the narrow run). The long walk, `every_setting_changes_the_tree_by_degree_and_branches_come_and_go_at_vanishing_size`, passed alone in 372 s.

## Digests repinned

| Pin | Before | After | Why |
|---|---|---|---|
| `every_preset_is_the_family_it_was` | 9516092986113572468 | 14009386315788109358 | leaf years on three presets |
| `every_walk_is_the_walk_it_was` | 11072528967666382666 | 6472862250418074352 | same |
| `every_override_reads_and_writes_as_it_did` | 5044572365911084041 | 5250702511323947789 | the spruce's and birch's leaf years on the wire |
| one cycle at both ends of the dial: spruce | (8090738721501801322, 14161706634267101928) | (10911873026870395678, 3276643373200734258) | its needles shade and lay wood 5.5 years |
| the same, Telperion | 12770220182948769363 at both ends | 12978362334831825737 | its leaf is its drawn leaf |
| the same, Laurelin | 8783816086298483637 at both ends | 1161467971842050269 | the same |

The ordinary tree, the oak, the beech, the birch and the palm hold their one-cycle digests.

## Measured

Seeds 1001, 1007 and 1012 were run through the scoreboard's runner as label `fn354-leaf` (head `bfe2fd92f`) and `fn354-leaf-before` (`ba0fc19eb`). Base is fn-352's `base-2026-11-11`.

- **Sheets** are in `leaf/`, with columns base, before and leaf: `<tree>-whole.jpg`, `-bare.jpg` and `-mid.jpg` for spruce, birch, ordinary, beech, telperion and laurelin, and `oak-whole.jpg` and `oak-bare.jpg`.
- **Measures** are in `leaf/VARIANTS.md`; run records are in `leaf/runs/`.
- **How it was run:** fn-319's optioned runner was copied into the ignored `raw/runner/` and pointed at this checkout (FRICTION.md).

Before against leaf, per tree; ranges over the three seeds:

| Tree | New shoots | Trunk at 1.3 m, cm | Drawn leaf area, m² | Leaf area index | Height |
|---|---|---|---|---|---|
| Oak 40 | equal | equal (27 to 34) | equal | equal | equal |
| Beech 40 | equal | equal (9 to 12) | equal | equal | equal |
| Birch 40 | equal | equal (17 to 19) | 183/142/199 → 108/89/121 | 4.9/3.4/3.7 → 2.9/2.1/2.3 | equal |
| Ordinary 40 | equal | equal (3 to 4) | 33/43/45 → 23/30/30 | 0.6 to 0.9 → 0.5 to 0.6 | equal |
| Spruce 40 | 104,994/97,399/106,120 → 98,092/90,342/98,506 (−7%) | 19/18/19 → 36/35/38 (base 34/33/37) | 271/257/281 → 201/189/208 | 9.0/8.5/9.4 → 7.1/6.6/7.4 | equal; spread −0.2 m |
| Telperion 82 | 136,257/129,708/95,191 → 90,196/70,579/51,469 | 55/53/53 → 94/86/86 (base 265/273/278) | 9,127/8,498/6,757 → 6,182/4,900/3,799 | 4.7/4.9/3.2 → 2.9/2.3/1.7 | 36.4/36.9/33.0 → 42.4/42.6/39.6 m |
| Laurelin 80 | 126,907/128,964/91,049 → 85,453/73,737/51,748 | 53/53/51 → 95/88/87 (base 285/294/298) | 9,264/8,996/7,071 → 6,475/5,379/4,142 | 5.1/5.4/3.6 → 3.2/2.7/2.0 | 36.5/38.9/33.9 → 41.6/42.7/39.3 m |

**Why the oak and the beech did not move:** their leaf years are 1, so the deposit is today's to the bit and the fade is never computed. The birch and the ordinary tree grow the same tree; only their drawing changed, because it stops leafing wood older than a year.

**Cost**, one thread, seed 1001, pinned:

| Tree | Before, s | Leaf, s | Before, GiB | Leaf, GiB |
|---|---|---|---|---|
| Spruce | 19.0 | 26.6 | 1.81 | 1.71 |
| Birch | 3.3 | 4.6 | 0.46 | 0.48 |
| Oak 40 | 2.9 | 3.9 | 0.44 | 0.44 |
| Beech | 0.55 | 0.72 | 0.09 | 0.09 |
| Telperion | 28.7 | 26.6 | 2.76 | 1.90 |
| Laurelin | 24.6 | 25.6 | 2.62 | 1.86 |

The machine was loaded (load 35 to 56 on both runs). The oak, the beech and the birch grow the identical tree yet ran 33 to 40% slower on the second run, so the spruce's 40% is inside that noise: no spruce-specific cost was measured.

**What the sheets show** (descriptions, not verdicts):

- **Telperion and Laurelin:** in the before column the limbs sweep out and down to the ground around a slender stem. In the leaf column the crown stands up off the ground on a visible trunk, round-topped and open. It is sparser and finer-branched than base, whose trunk is about three times as thick, and its leaf is visibly thinner than base's.
- **Spruce:** the same cone in all three columns, a little narrower in the leaf column. It is not stripped to a pole or a tuft.
- **Birch:** the same tree, its interior a little barer in the leaf column.
- **Oak and beech:** before and leaf are identical.
