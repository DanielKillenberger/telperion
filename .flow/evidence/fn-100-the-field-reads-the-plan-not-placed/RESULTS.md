# fn-100 results (2026-09-22)

Base: master `3fdfb440`, its Wasm binding built in a separate worktree
(`raw/base-3fdfb440.wasm`, ignored). Candidate: this branch's binding. Every
measurement is a single tree through the raw Wasm exports in Node, no
browser; the source is `measure.mjs` beside this note, and the rows it wrote
are under `raw/` (ignored). The 32-core desk, release profile of the wasm
target.

## R2: field-only builds, median of five, with a 64^3 batch query

Each sample is one `build({field: true})` plus one batch query of 262,144
cells over the field's bounds. The candidate's field-only build is faster
than the base's on every tree, and faster than the same commit's
surface-plus-foliage build; its peak linear memory is lower than the base's.

| tree | base field ms (build + query) | candidate field ms (build + query) | candidate surface+foliage ms | peak MB base → candidate | field MB base → candidate | ratio | wood share |
|---|---|---|---|---|---|---|---|
| oak, seed 1 | 940 (863 + 76) | 288 (138 + 151) | 657 | 145 → 82 | 81.3 → 27.6 | 0.34 | 0.67 |
| oak, seed 7 | 1,071 (995 + 74) | 330 (165 + 173) | 787 | 178 → 107 | 97.0 → 31.7 | 0.33 | 0.65 |
| birch, seed 1 | 2,362 (2,304 + 57) | 345 (210 + 136) | 2,296 | 73 → 54 | 30.0 → 15.5 | 0.52 | 0.66 |
| birch, seed 7 | 2,598 (2,523 + 75) | 579 (403 + 176) | 2,503 | 83 → 74 | 35.6 → 18.7 | 0.53 | 0.68 |
| spruce, seed 1 | 9,002 (8,968 + 34) | 233 (125 + 109) | 5,548 | 853 → 75 | 659.4 → 29.5 | 0.045 | 0.48 |
| spruce, seed 7 | 8,646 (8,612 + 33) | 223 (117 + 105) | 5,285 | 815 → 77 | 628.7 → 27.9 | 0.044 | 0.48 |

Wood share is the field's bytes with the canopy at size zero (wood segments
and their index only) over the whole field's bytes (`raw/wood-share.jsonl`).
The wood index is the same in both builds, so the plan's sweeps cost a third
to a half of what the oak's and birch's leaf boxes cost, and a twentieth on
the spruce, whose seven million needles were the base's 660 MB.

The batch query itself is about twice as slow per cell on the candidate
(151 ms against 76 ms on the oak): the plan's answer visits every sweep that
reaches a cell to tally its count and its limb, where the placed field
stopped at the first leaf box. A flags-only query could keep the early exit;
not built here.

## R3: wood answers are byte-identical

`crates/telperion-core/tests/field.rs` passes without edits. On eight trees
(oak, birch and spruce at seeds 1 and 7, ordinary and Telperion at their own
seeds; the 8-tree forest harness no longer exists in the checkout, see
FRICTION.md) the wood bit of a 64^3 batch query over the base's field bounds
is identical byte for byte between base and candidate
(`raw/wood-compare.json`, one SHA-256 per tree).

## R4, R5, R6: `crates/telperion-core/tests/field_plan.rs`

- `every_retained_leaf_vertex_lies_in_a_foliage_cell`: for the oak, birch
  and spruce at 0.1 m, 0.25 m and 1 m, every vertex of every leaf the CPU
  placement retains after the cull lies in a cell the plan reports as
  foliage. `spruce_needles_are_found_by_cells_far_larger_than_their_reach`
  and `a_cube_touching_only_the_sweep_boundary_is_covered` are the named
  tests.
- `over_coverage_is_at_most_twice_the_placed_field_at_a_quarter_metre`: the
  plan's foliage cells over the placed field's, at 0.25 m: oak 1.39 (212,228
  against 152,351), birch 1.63 (62,929 against 38,536), spruce 1.16 (15,073
  against 13,003). The readiness probe's circumsphere numbers reproduced.
- `count_estimates_sum_to_the_plan_total_over_a_grid`: at a 1 m grid the
  estimates sum to the plan's total within one percent on all three.
  `two_limbs_report_their_own_systems` pins the limb ids, the larger-estimate
  rule, the lower-id tie and the empty cell.

## R7: the plan builds without the surface and the placement

`cargo build --profile ci -p telperion-core --no-default-features` and
`cargo test --profile ci -p telperion-core --no-default-features --lib`
(185 passed) run locally and as the `core-plan` job in
`.github/workflows/tests.yml`, gated by the core receipt. What the
`geometry` feature carries, and what stayed in, is in FRICTION.md.

## R8: the sheets, for the owner

`raw/sheet-64-base-structure.png` is the prototype as it was, on the base
binding's structure export. `raw/sheet-64-candidate-field.png` is the same
script with `FIELD=1`, reading the new field: one batch query over the 64
grid, foliage where the field reports it and the owning limb keeps its
clump, wood where the field reports wood that no leaf reaches. Same rows
(keep 0.5, 0.3, 0.15), same columns (oak, birch, spruce), same renderer.

What the two sheets show, from four views of them:

- The oak and the birch read as themselves on both; the field's crowns are
  slightly fuller at keep 0.5 (the reach is conservative).
- The spruce reads as a conifer on the field sheet: a cone with a leader.
  On the base sheet it is a low mass. The prototype clothed twig nodes only;
  the plan clothes every run the placement clothes, and the spruce's slender
  leader is under its `shoot_radius`, so the field shows the foliage the
  renderer actually draws there.
- Dark specks inside the crowns and around dropped clumps are limb wood no
  leaf reaches; the prototype drew no wood thinner than a fifth of a cell,
  which the field's answer cannot tell apart. A consumer that wants the
  prototype's look can drop wood cells not connected to the trunk; that is
  consumer styling, not built here.
- The clumps are coarser: the field's limb id is the family's
  `clump_system_order` system (the trunk and its first two limb orders), so
  at keep 0.3 the oak parts into a few large masses where the prototype's
  per-bearer coin opened many small gaps. A consumer wanting finer clumps
  has no dial today; a follow-up could let the field take an order.

The verdict is the owner's; R8 is recorded as awaiting the owner.
