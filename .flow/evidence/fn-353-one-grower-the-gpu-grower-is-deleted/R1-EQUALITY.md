# R1's equality does not hold on the CPU path today (11 Nov 2026)

Task 1 stopped here, as its dispatch says to: the equality R1 asks the test to check is criterion P4 of the goal and a design matter.

## What was run (measured)

The test R1 names, written against `telperion_core::pipeline::space::growth` as it stands at `8a3a20b74`: the European beech, seed 7, begun for a horizon of 12 and grown on a year at a time; at every whole year y, `settle(state, y)` is compared with the pipeline's `grow` asked for age y at the same seed. The probe is `r1-equality-probe.patch` (the test, with its diagnostic prints); it is not in the tree.

Command: `cargo test --profile ci -p telperion-core --lib every_year_grown_on -- --nocapture` (through `telperion-heavy 8 --threads 8`).

| Year | Nodes grown on | Nodes asked | Root radius grown on (m) | Root radius asked (m) | Leaves grown on | Leaves asked |
|--:|--:|--:|--:|--:|--:|--:|
| 1 | 8 | 8 | 0.016549 | 0.015964 | 0 | 0 |
| 2 | 30 | 30 | 0.021946 | 0.021191 | 4 | 3 |
| 3 | 76 | 76 | 0.025441 | 0.025114 | 20 | 19 |
| 4 | 190 | 190 | 0.029124 | 0.028792 | 66 | 65 |
| 5 | 384 | 383 | 0.032874 | 0.032471 | 124 | 123 |
| 6 | 702 | 692 | 0.036935 | 0.036402 | 243 | 242 |
| 7 | 1,129 | 1,112 | 0.041217 | 0.040594 | 348 | 348 |
| 8 | 1,630 | 1,617 | 0.045306 | 0.044750 | 466 | 466 |
| 9 | 2,222 | 2,206 | 0.049122 | 0.048632 | 639 | 639 |
| 10 | 2,870 | 2,849 | 0.051750 | 0.051417 | 853 | 851 |
| 11 | 3,412 | 3,411 | 0.053911 | 0.053781 | 972 | 971 |
| 12 | 4,080 | 4,080 | 0.055948 | 0.055948 | 1,136 | 1,136 |

Leaves are `foliage.instances` of `growth::mesh` against `mesh::build` at age y. Field by field: years 1 to 4 differ in node radii, frames and `grown` with the same topology; from year 5 the keys, clusters, leafing and crossover differ too, so the topology has diverged. Year 12, the horizon, is equal in every field, which is what the existing `a_tree_grown_on_is_the_pipelines_tree_grown_at_once` holds. The tree's `year` record agrees at every year.

## Where the horizon reaches growth (inferred from reading, not tested)

The paused grower is begun with the horizon as its request's age, and two reads take it from there:

- `telperion-space/src/grow.rs:201`: `Windows::new(species, request.age)`, the presence windows. Each bud's expected wood is taken as a share of the whole tree's expected metres at that age (`presence.rs:48-66`), so a tree grown toward 12 weighs its buds against the 12-year tree.
- `telperion-space/src/grow/cycle/paused.rs:203`: `sketching = cycle < self.request.age`. A tree grown to y as its last year skips year y's layout and light; one grown on toward a later horizon sketches it.

Which of the two moves the root radius at year 1 was not separated.

## The question for the host

A year of a growth begun for a horizon H is not the tree asked for at that age. The device grower took the same horizon, so the harness's grow-in has always shown the tree on its way to H, never the y-year-old tree. Options as I see them, none chosen:

1. **R1's test holds what is true:** each year drawn is the state's own (repeatable, equal at the horizon, as the existing test checks), and the y-year tree is a different tree. The bindings and the harness then go ahead as specified. P4 stays open in the goal.
2. **Each year begun for its own horizon:** year y begun for horizon y and grown whole. The equality holds by construction; a grow-in to 40 costs about the sum of the 40 trees' growth, not one.
3. **The engine made horizon-free:** the presence windows and the last-year sketch stop reading the horizon. This changes how trees grow, which this spec's Boundaries forbid; it would be its own spec.

Nothing in the product changed in task 1. `BEFORE.md` holds the baseline the later tasks compare against.
