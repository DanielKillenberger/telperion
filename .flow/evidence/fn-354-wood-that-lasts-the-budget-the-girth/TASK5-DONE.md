# fn-354 R7: a lateral's kind follows its light, by the builder, 12 Nov 2026

Task 5 has stopped before any code, on the brief's own condition: the lineage tables cannot hold "the PA a bud becomes" as `shade_short`'s second value without a row of a kind they do not have. The host's birch sapwood change, which came between the tasks, is built, committed on its own and measured below.

Tier: product code, built by Opus to the host's design (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review of the spec's diff at its end)

## The stop: what the tables allow

**Checked first, and it passes:** the light a bud's kind would read is the shade hazard's, `Shoot::light`, the pencil's light at its bearer's apex. That is `Field::at`, held to 0 to 1 with the open sky at 1, and 1 wherever nothing shades.

**Where the kind is drawn today** (`grow/unit.rs`, `lineage::bud`): a bud's PA is drawn from its zone's `lateral` table, a vector over the target PAs, by one draw against the table's running sums. Its presence then comes from the same draw (`lineage::bud_presence`). Every PA in the tables is named by its position in such a vector (`lateral[j]`, `dormant[j]`), with the shared chain's indices (`chain::SHOOT` 12, `chain::BRANCH` 9, `chain::SHORT` 14), so no species branch is needed to name the beech's PAs.

**What the tables do not have** is a row whose value is a PA. Every `PaState` and `Form` row is one real number on a `Linear`, `Log`, `Kept` or `Outcome` scale, and walks and blends move it by degree. A scalar `shade_short` naming SHORT by its index:
- would be a whole-number row on a linear scale. Between two species with different targets, a blend would pass through PAs neither names, a switch and not a degree. This is memory entry `a-whole-number-engine-row-on-a-linear-2026-10-08`.
- would also need its own refusal for a non-integer or out-of-range PA.

**What the tables do allow:**
1. **A per-PA vector on the bud's own PA,** `shade_short[j]`, the exponent with which a bud of this PA becomes PA j as shade denies it. It is shaped and blended like `lateral[j]`. The beech's SHOOT and BRANCH carry `shade_short[SHORT] = 0.38`, and every other entry is 0 (dormant). With one nonzero entry it is the design's rule exactly. With several it needs a combining rule, which the design does not give.
2. **A scalar exponent, with the target fixed by the chain:** "the shortest PA the bud's zone table names", or "the next PA down". That puts a rule in the grower in place of a table value, and the rule is the host's to choose.
3. **A scalar `shade_short` integer row,** as written, with blends taking one end's target. This is the switch above.

**What I would build if told:** option 1. It is the only one that blends by degree and names the PA as the tables already do.

**Two smaller design questions** the build would meet either way:
- **The kind switch's presence.** Today a made bud's presence is its draw's lead over its PA's bounds (host decision 8). For the shade's keep-or-turn decision to change the tree by degree, it needs its own draw (a child key of the bud's) and its own presence from the lead of that draw over `L^e`. I would add exactly that, keyed so that it takes no draw where e is 0.
- **The light on the first unit of a lateral.** The bud forms in the cycle its bearer grows, and its bearer's light is the cycle's. I would read the bearer apex's light (`Shoot::light(apex.axis)`), the shade hazard's, as the design says.

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
