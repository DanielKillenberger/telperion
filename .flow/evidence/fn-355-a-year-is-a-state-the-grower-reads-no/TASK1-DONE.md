# fn-355 task 1: stopped at the table's cost, by the builder, 12 Nov 2026

Stopped before building the windows, at the spec's first stop: "any PA whose lifespan ... or whose look makes the table's cost material". The oak has an age with a 500-year lifespan and the spruce two with 300, so the look takes the expected-length table 500 and 300 rows past the year grown. On the oak at 40 the table then costs about 1.4 s against a tree of 2.6 to 3.9 s. Nothing in the grower is changed. Two tests are committed: the table check (green) and R1's structural test (red, as it should be on this base).

Tier: product code, built by Opus to the host's design (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review at the landing with fn-354)

Baseline: green via handoff (narrow space and core 765 of 766 at fn-354's head `d0728b827`, the one red being `a_node_keeps_its_identity_across_ages`; vitest 218). No path outside `.flow/` changed since.

## What was done

- **The table check** (`closed_form::tests::an_expected_length_does_not_depend_on_how_far_the_table_runs`, green):
  - **What it holds:** rows 0 to 40 of `expected_log_lengths` filled to 40 are the same rows filled to 120, to the bit.
  - **Species:** the oak, beech, spruce, birch and palm.
  - **On paper:** a row is memoised by its age, its cycles, what it carries and its first unit's law (`closed_form::sub`). The table's `Living` has no windows, so its `left` is always 0 and no key reads the table's extent. Nothing needs fixing first.
- **R1's structural test** (`grow::cycle::paused::tests::a_year_is_a_state_not_a_horizon`, red on this base, as written):
  - **What it holds:** the tree at year y of a paused growth begun for a later age is `grow_cycle` at y, by `tree_digest` (the digest the paused-prefix test uses).
  - **Trees:** the oak, spruce and beech at (14 of 15) and (20 of 28).
  - **On this base:** it fails on its first case, the oak at 14 of 15: 1236893093839513191 against 16524772271080607844.

## The numbers behind the stop

**Lifespans.** Each age's `life()`, the longest per species, and the extent the look needs (the year grown plus the longest lifespan):

| Species | Longest lifespan | Ages with it | Extent at 40 | Extent at 120 |
|---|---|---|---|---|
| Oak | 500 | PA 6 | 540 | 620 |
| Spruce | 300 | PA 4, PA 9 | 340 | 420 |
| Palm | 150 | PA 2 | 190 | 270 |
| Birch | 100 | PA 5, PA 7 | 140 | 220 |
| Beech, Telperion, Laurelin | 70 | PA 6 | 110 (Telperion at 82: 152) | 190 |

The ordinary tree's lifespans are fractional (0.44 to 61.7 years), so a look between two whole rows has to be read between them.

**The table's cost.** `expected_log_lengths` to each extent, one thread, in the `ci` profile, at load 40 to 49 (another session's scoreboard was running):

| Species | Table to the year (today) | Extent at 40 | Extent at 120 |
|---|---|---|---|
| Oak | 0.04 s at 120 | 1.38 s (540) | 3.01 s (620) |
| Spruce | 0.05 s at 120 | 0.88 s (340) | 2.24 s (420) |
| Birch | 0.08 s at 120 | 0.14 s (140) | 0.41 s (220) |
| Beech | 0.06 s at 120 | 0.08 s (110) | 0.25 s (190) |
| Palm | under 0.01 s | 0.01 s (190) | 0.02 s (270) |

**Against the tree.** fn-354's one-thread costs (TASK3-DONE.md and TASK4-DONE.md, both under load):

| Tree | Tree, s | Table, s | Share |
|---|---|---|---|
| Oak at 40 | 2.6 to 3.9 | 1.4 | 35 to 53% |
| Oak at 120 | 43 to 49 | 3.0 | 6 to 7% |
| Spruce at 40 | 19 to 27 | 0.9 | 3 to 5% |

The cost grows faster than the rows: the oak's table at 440 rows took 0.56 s, and at 620 rows 3.0 s.

## Where else the cost lands (read in code, not measured)

- **The closed form.** `expected_kept`, `expected_grown` and `expected_counts` build `Windows::new(species, age)` for `Living::borderline`. That form is the grower's move-on window, so it would take the same look and the same extent.
  - `pipeline::space::foreseen` calls `expected_kept` before every growth. Its comment reads "in milliseconds where growth takes seconds".
  - `blend::size` calls `expected_kept` and `expected_grown` inside a bisection, once per step.
  - Neither is measured under the look.
- **Lifespans up to 1,000,000.** Validation admits them (`MAX_LIFESPAN`), and such a lifespan would need a table with that many rows. Whether such an input is refused by name or its look is bounded is a design question.

## What the sleepers would cost (read, for when the build resumes)

- **The bound:** a sleeper that sleeps past its last stage never wakes (`dormant::aged`). So the years a sleeper may wake in are bounded by its stages' summed lifespans.
- **The sleeping ages** (a zone with a nonzero rate bears them):
  - oak: PA 9, 16 years;
  - spruce: PA 10, 50 years;
  - birch: PA 9, 12 years;
  - the beech and the palm have none.
- **Cost:** the sleeping-wood sum and `asleep` stay short under that bound.
- **Unsettled:** the wait drawn by `decide::wake` is unbounded, so the design's "every sleeper drawn is recorded" needs that bound, or something like it, before `asleep` can grow as needed.

## For the host

1. **The look's extent.** The oak's 500-year age and the spruce's 300-year ages make the table cost about 1.4 s on the oak at 40, a third to a half of the tree. The same extent reaches the closed form's callers. The spec's stop applies. Options I can see, none chosen:
   - Keep the look and make the table cheaper. This is not in the design.
   - Bound the look.
   - Read the whole tree's wood differently for long-lived ages.
2. **Unbounded lifespans.** A lifespan validation admits up to 1,000,000 needs a table that long.
3. **Sleepers that never wake.** I propose to record only sleepers whose wake time falls within their stages (`aged` is some), since `asleep` cannot be indexed by an unbounded cycle. Such a sleeper has no effect on growth today except keeping its bearer open in the working set, which touches closure, another of the spec's stops.

## Tests

- `telperion-heavy 6 --threads 4 -- cargo test --profile ci -p telperion-space --lib -- closed_form::tests::an_expected_length a_year_is_a_state_not_a_horizon`: the table check passed. R1's test failed, as written, on the oak at 14 of 15.
- `cargo fmt --all --check`: green.
- Not run: the narrow suites, since no product code changed.
