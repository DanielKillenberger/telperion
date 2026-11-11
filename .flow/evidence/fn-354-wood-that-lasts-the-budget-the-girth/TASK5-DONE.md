# fn-354 R7: a lateral's kind follows its light, by the builder, 12 Nov 2026

R7-DESIGN.md's design is built in `995134622`, in the vector form the host chose on 12 Nov. The beech's leaf rose from 52 to 109 m² to 141 to 241 m² at 40, its leaf area index from 1.9 to 3.7 to 4.5 to 7.6, and its trunk from 9 to 12 cm to 14 to 18 cm. Every other species is byte-equal. Two tests are red, and each is the host's call (below). The task stays `in_progress`. The birch's sapwood change, which came between the tasks, is reported under its own heading.

Tier: product code, built by Opus to the host's design (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review of the spec's diff at its end)

Baseline: green at `fe0fb9622` (task 4's narrow suites, 765 of 765).

## What was built

- **`PaState::shade_short`:**
  - one exponent per age, a `Vec<f64>` like the zones' `lateral` tables;
  - listed in `settings()` as `shade_short` entries, so blends and walks move it entry by entry on a `Linear` scale, bounded 0 to 10 like the shade hazard;
  - an empty table is neutral, like all zeros. Test fixtures use it, and a blend reads it as the other end's zeros.
- **The rule** (`lineage/turn.rs`, called from `grow/unit.rs`), for a bud drawn as an age whose table sums to e > 0:
  - **keep or turn:** it keeps its kind with probability L^e, L its bearer apex's light (the shade hazard's, 0 to 1 of open sky); otherwise it becomes age j with probability `shade_short[j]` / e;
  - **draws:** key `TURN` (registered in `lineage/keys.rs`) under the bud's place, and its child for the pick. None is taken where e is 0.
  - **presence:** the made bud's presence is multiplied by the keeping draw's lead past L^e, and, for a bud that turned, by the pick's lead past its kind's bounds.
- **Refused by name:**
  - an entry on the age's own index, as `states[k].shade_short[k]`;
  - an entry off 0 to 10;
  - a table that is neither empty nor one entry per age.
- **Value files:** read and written as `/space/ages/k/shadeShort/j` (`presets/space.rs`). The browser's settings table leaves it out, as it does the zone tables.
- **`reads_light` and `unlit`:** both count it. Every hand-built light-neutral fixture that zeroed the shade hazard, shade size and balance hazard now zeroes the table too. These are the closed-form, space-walk, preset, grown-total, light and tolerance tests, and three examples.
- **The beech** (`beech.rs`, and the beech, Laurelin and Telperion value files, each with its source line):
  - SHOOT and BRANCH carry `shade_short[SHORT] = 0.38`;
  - the branch's medial zone is `[(SHOOT, 0.53), (SHORT, 0.17)]`;
  - every reiterate's medial zone is `[(SHORT, 0.085), (BRANCH, 0.485)]`;
  - sources: Lemoine et al. 2002, Table II; the power law is named an estimate in form.
- **R7-DESIGN.md** is corrected in place ("host, 12 Nov: the row is a vector").
- **docs/pipeline.md** has a paragraph on the rule.

## For the host to decide

1. **`pipeline::space::tests::a_node_keeps_its_identity_across_ages` is red** ("a lineage changed its key"). The test grows the beech to 14 and to 15 years and asserts that a lineage present in both holds the same phytomer key. A lateral's lineage is keyed by its age (`key.onto(pa)`), and its age now depends on the light at its bearer when it forms. That light differs between the two runs, so the same place makes a short shoot in one and a long shoot in the other.
   - **Inferred cause** (not traced): presences are sized by windows built from the requested age (`Windows::new(species, request.age)`), and scales, so leaf and light, follow presences.
   - Under the shade hazard alone a light difference changed survival, which never changes a key; a kind does.
   - `the_tree_at_a_year_is_the_horizons_prefix`, which grows to a year on a grower paused at a fixed horizon, passes.
   - **The question:** does the promise (a lineage keeps its key from one requested age to the next) still hold for a species whose kind follows light, or does it now belong to the paused grower only?
2. **`suite::retention::a_share_of_a_year_keeps_that_share` is red** ("0.25: 0.325 of 9558"). It counts the leaves of four beeches at 30, with leaf years 1, 2 and 1 + share, and reads the share kept as (n(1 + share) − n(1)) / (n(2) − n(1)), on the premise that one tree stands under all four values.
   - Since task 4 the grower reads the leaf years, so each value grows its own tree.
   - It passed at task 4 because the trees differed little. The beech's kind now follows its own shade, and they differ by more than the test's four standard errors.
   - This is the sibling of the test the host recast on 12 Nov ("each value's own tree"). Its recast form needs the stations on last year's wood in one tree, which the prepared stations do not count apart. I have not changed it.

## Smaller choices I made

- **The long-share test** weighs each zone's laterals by its mean node count, 2.8 laterals a unit. It allows four standard errors plus 0.01, for the design's rounded 85% (the table gives 0.848).
- **The walk** is `light: states[1].shade_short[2]`, 0 to 1: a limb bud turning into a twig. It held its bound in `light_changes_the_tree_by_degree`.
- **Not touched:** `node_wood`'s expected wood, which sizes a node's presence window, still reads the full-light table.
- **`light_rows`' list of response rows** does not include the table: it passed, and nothing asked for it.

## Tests

- **Narrow space and core suites:** 755 of 766 before the fixes. 9 of the 11 failures were fixtures and pins, fixed and rerun green. The two above stay red.
- **New tests, green:**
  - the long share at light 1 and 255/1,850 (85% and 40%);
  - three refusals (own index, range, width);
  - the walk.
- **Repinned:**

  | Pin | Before | After |
  |---|---|---|
  | `every_preset_is_the_family_it_was` | 5899378939696360618 | 3725984341644380676 |
  | `every_walk_is_the_walk_it_was` | 6249782841009558150 | 5134440887525534452 |
  | `every_override_reads_and_writes_as_it_did` | 3907831028619621265 | 9054662260167386261 |
  | one cycle, beech | 15759162824176722754 | 11333272162776746682 |
  | one cycle, Telperion | 12978362334831825737 | 7017510222834692025 |
  | one cycle, Laurelin | 1161467971842050269 | 230258731259285086 |
  | closed-form boundary rows, beech at 2 to 80 years | 9 rows | listed in the file |

  The ordinary tree, oak, spruce, birch and palm hold their one-cycle digests to the bit. That is the byte-equal check for every species at 0.
- **Other checks:**
  - both guards, `cargo fmt --all --check`, `npx tsc --noEmit` and `npm run catalogue:check` green;
  - `presets.generated.ts` regenerated;
  - vitest: 218 passed.

## Measured

Seeds 1001, 1007 and 1012, label `fn354-beech` (head `995134622`) against `fn354-beech-before` (`efd26ace8`, task 4 with the birch's sapwood). Base is fn-352's `base-2026-11-11`.

- **Sheets** are in `beech/`, with columns base, task4 and beech: `beech-whole.jpg`, `-bare.jpg` and `-mid.jpg`, and the same three for `telperion` and `laurelin`.
- **Measures** are in `beech/VARIANTS.md`; run records are in `beech/runs/`.
- **The long share** was added to the probe copy in `raw/runner/` (`long_share`): of the laterals whose first unit grew in the last cycle, the share that are not short shoots.

| Tree | Drawn leaf, m² | Leaf area index | Trunk, cm | New shoots | Long share, last year | Height, m |
|---|---|---|---|---|---|---|
| Beech 40, task 4 | 52 / 68 / 109 | 1.9 / 2.4 / 3.7 | 8.9 / 10.4 / 11.5 | 5,521 / 5,377 / 7,786 | 0.170 / 0.170 / 0.164 | 9.7 / 10.0 / 9.8 |
| Beech 40, now | 141 / 173 / 241 | 4.5 / 6.5 / 7.6 | 14.3 / 15.9 / 17.6 | 12,315 / 12,452 / 16,126 | 0.177 / 0.179 / 0.176 | 9.7 / 10.0 / 9.8 |
| Telperion 82, task 4 | 6,182 / 4,900 / 3,799 | 2.9 / 2.3 / 1.7 | 93.9 / 86.0 / 85.6 | 90,196 / 70,579 / 51,469 | 0.143 / 0.137 / 0.144 | 42.4 / 42.6 / 39.6 |
| Telperion 82, now | 8,005 / 5,649 / 4,957 | 4.2 / 2.8 / 2.4 | 113.3 / 105.2 / 106.1 | 112,813 / 82,212 / 67,129 | 0.174 / 0.173 / 0.182 | 43.4 / 43.6 / 40.9 |
| Laurelin 80, task 4 | 6,475 / 5,379 / 4,142 | 3.2 / 2.7 / 2.0 | 95.4 / 87.7 / 87.3 | 85,453 / 73,737 / 51,748 | 0.146 / 0.146 / 0.146 | 41.6 / 42.7 / 39.3 |
| Laurelin 80, now | 8,439 / 6,166 / 5,641 | 4.7 / 3.3 / 2.8 | 115.2 / 107.4 / 108.9 | 109,380 / 82,212 / 71,351 | 0.172 / 0.181 / 0.180 | 42.9 / 43.7 / 40.4 |

- **The birch at 40** is equal on every measure to `fn354-beech-before`'s birch: 22.9 / 20.6 / 23.7 cm, 30,114 / 22,137 / 30,283 new shoots, 333,258 / 257,081 / 350,304 nodes.
- **Against the expectation on paper** (60 to 100 m² and 15 to 19 cm at the median): the trunk lands in it at 15.9 cm. The leaf, 173 m² at the median, is above it, and its leaf area index stays under the refuting 9.
- **The long share is the whole tree's:** short shoots from SHOOT ages, which bear only short shoots, dominate it. Lemoine's 85% and 40% are a branch's laterals, which the unit test checks on the beech's branch table.
- **Cost, one thread, seed 1001,** at load 47 to 71 on both runs:

  | Tree | Before, s | Now, s | Before, GiB | Now, GiB |
  |---|---|---|---|---|
  | Beech | 0.70 | 1.49 | 0.09 | 0.20 |
  | Telperion | 25.3 | 44.9 | 1.90 | 2.33 |
  | Laurelin | 24.0 | 36.5 | 1.84 | 2.33 |

  The beech grows 2.2 times the shoots. The seconds are under heavy and unequal load, so they are not a fair comparison.

**What the sheets show** (descriptions, not verdicts):
- **Beech:** the crown is fuller and denser than both task 4 and base, with more leaf at its surface, on a trunk thicker than task 4's and thinner than base's. The close (crown) view reads as a dense round crown.
- **Telperion:** the crown is a little fuller than task 4, still open and finer-branched than base, on a trunk 105 to 113 cm against base's 265 to 278 cm.

## The birch's sapwood (host, 12 Nov; commit `efd26ace8`)

**What changed:**
- `sapwood` for the birch is 4.6 cm² per m² of leaf on every age, in `birch.rs` and `silver-birch.values`. The comment says "derived, not measured" and names Repola 2008 (eq. 12), Hölttä et al. 2013 (Table 2) and Lintunen et al. 2011 (secondary), as BIRCH-SAPWOOD.md's "Decision" says.
- `sapwood_rings` stays 40, estimated, with the Gebauer et al. 2008 note.
- `BIRCH-SAPWOOD.md` is committed with it.

**Repinned:**

| Pin | Before | After |
|---|---|---|
| `every_preset_is_the_family_it_was` | 14009386315788109358 | 5899378939696360618 |
| `every_walk_is_the_walk_it_was` | 6472862250418074352 | 6249782841009558150 |
| `every_override_reads_and_writes_as_it_did` | 5250702511323947789 | 3907831028619621265 |
| one cycle at both ends of the dial, birch | (3753336579002065071, 11296798367218290338) | (3184370250942454689, 16241647282371321572) |

**Tests:**
- The narrow space and core suites: 765 of 765.
- fmt is green.
- `presets.generated.ts` was regenerated.

**Measured:** the birch at 40, seeds 1001, 1007 and 1012, label `fn354-birch`, against task 4's tree (`fn354-leaf`):

| Measure | before | birch | base |
|---|---|---|---|
| trunk at 1.3 m, cm | 18.6 / 16.7 / 19.2 | 22.9 / 20.6 / 23.7 | 30 / 26 / 30 |
| height, m | 17.1 / 17.4 / 15.9 | 17.1 / 17.5 / 15.9 | 16.8 / 17.1 / 16.0 |
| spread x, m | 7.5 / 6.7 / 9.8 | 7.2 / 6.6 / 9.6 | |
| new shoots | 30,114 / 22,137 / 30,283 | equal | |
| drawn leaf area, m² | 108 / 89 / 121 | 113 / 89 / 122 | |
| drawn nodes | 329,364 / 256,726 / 349,468 | 333,258 / 257,081 / 350,304 | |

- **Against the expectation of 21 to 23.5 cm:** seed 1001 is inside it, seed 1007 is 0.4 cm under and seed 1012 is 0.2 cm over. The ratio to before is 1.23 on every seed, the expected 1.23.
- **What else moved:** the grower makes the same shoots. The drawn tree moved a little where its limbs bend less: the spread falls 0.1 to 0.3 m, and drawn leaf and nodes change by under 5%.

**Sheets** are in `birch/`: `birch-whole.jpg` and `birch-bare.jpg`, with columns base, before and birch. Base has no bare shot. Measures are in `birch/VARIANTS.md`; the run record is in `birch/runs/fn354-birch/`.

**What the sheets show** (descriptions, not verdicts): the same birch in both columns. The trunk is visibly a little thicker at its foot, and the drooping crown is unchanged.
