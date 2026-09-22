# Investigation — fn-109, the apical rosette

Written for fn-80 dispatch-5 (role investigate, tier cheap), 2026-09-22. Verifies every file and
line the strong-tier design handoff (`.flow/evidence/fn80/design-fn-109.md`) cites, against the code
at the current worktree HEAD. Gathers only; decides nothing.

## Touch surface

| File | Function / struct, line range | What changes | Handoff's cited lines correct today? |
| --- | --- | --- | --- |
| `crates/telperion-core/src/foliage/placement.rs` | `struct CanopyParams`, 15-90 | Ten new fields + doc comments | Yes, exact |
| same | `impl Default for CanopyParams`, 92-121 | Ten new defaults (0, 137.508, 45., 60., 0., 1, 0., 45., 0., 0.) | Handoff does not cite a line for this block; range confirmed by inspection |
| same | `fn runs`, 216-224 | Returns `Vec::new()` when `p.rosette_fronds > 0` | Yes, exact |
| same | `fn leaves_on`, 226-245 | Adds `rosette::count`; skip `short_shoots::count` when rosette on | Yes, exact |
| same | `fn leaf_count`, 250-261 | No direct edit (inherits) | Handoff cites `:250` only; correct as the fn start |
| same | `fn place_impl`, 263-313 | Calls `rosette::clothe` after run loop, before clumping thin (short_shoots call site is line 308) | Yes, exact |
| same | `fn validate`, 314-347 (float table 322-341, clump check 342-344) | Ten new range entries + `short_shoots::validate`-style row | Yes, exact |
| same | `fn bearing_runs`, closure 421-428 | Unchanged; cited only as context for why `runs` must switch off entirely | Yes, exact |
| `crates/telperion-core/src/foliage/rosette.rs` | new file | `Rosette`, `rosettes`, `validate`, `count`, `clothe`, `place_rosette`, `fan` | N/A (file does not exist yet) |
| `crates/telperion-core/src/foliage/reference.rs` | `fn reach`, 61-97; `Reference::from_params`, 44-56 | New arm: `reach` grows by `rosette_depth + rachis_length + element reach` when `rosette_fronds > 0` | Yes, exact (`from_params` starts line 44, `reach` spans 61-97 — file is 97 lines total) |
| `crates/telperion-core/src/foliage/timeline.rs` | `impl Foliage`, `fn new`, 79/88-118 | No edit; new rows rejected by name automatically via `place` on an empty tree | Line **88 is correct**, but the handoff names this `Placement::new` / `timeline::Placement::new` twice (Interfaces section and Difficult cases). The struct at that `impl` block is **`Foliage`**, not `Placement` — `Placement` is a different, unrelated struct at line 29 (`{ identity: PlacementIdentity, leaf: Leaf }`). The constructor is `foliage::timeline::Foliage::new`. Fn closes at line 118, not 117 (off by one) |
| `crates/telperion-core/src/params.rs` | `fields!` macro, `"canopy"` block, 108-129 | Ten new `$op!` lines | Yes, exact |
| same | `IN_WORK`, 259-262 | No edit (already lists date-palm at id 7) | Yes, exact |
| `crates/telperion-core/src/capability.rs` | `UNEXPRESSED`, `apical-rosette` 76-78, `pinnate-frond` 79-82; also `pinnate-compound` 72-74 (spec's host decision, not in the design handoff) | Move to `EXPRESSED` | Yes, exact for both cited entries |
| same | `DERIVABLE`, 101-108 | Append two names (spec's host decision: three, if `pinnate-compound` also moves) | Yes, exact |
| same | `fn derived`, 185-209 | Two new `if` clauses | Yes, exact |
| same | `fn digest`, 143-161 | No edit; moves version by construction | Yes, exact |
| `crates/telperion-core/src/specimen/view.rs` | `fn mesh`, 65-108 (short-shoot branch 90-99) | Add `place_rosette` call beside `place_short_shoots`/`place_short_shoots_clumped` | Yes, exact for both `:79-105` and `:90-99` |
| `crates/telperion-core/src/blend.rs` | `fn families`, `walk!` invocations, lines 34-70+ (canopy bucket at 65-68, 167, 178-182) | **Not mentioned anywhere in the design handoff.** Every existing `CanopyParams` field is bucketed into a `walk!(linear/degrees/counts/density/many: ...)` call for family-to-family interpolation. The ten new rows have no bucket yet and none is proposed | New authoring site the handoff omits — confirmed by direct inspection, not cited at all |
| `crates/telperion-jev/data/dials.json` | flat JSON array, 201 entries today, 20 with `group: "canopy"` | Ten new entries | File exists at the cited path; row shape (`id`, `path`, `meaning`, `min`, `max`, `small`, `substantial`, `group`, `score_visible`, `meaning_basis`, `range_basis`, `source`) matches an existing canopy row inspected (`leaves`, sourced from `placement.rs:64`) |
| `crates/telperion-core/tests/capability.rs` | `PRESETS`, 9-17; `the_six_names_the_derivation_produces_are_all_in_the_vocabulary`, 53-63 (`DERIVABLE.len() == 6` at line 55); `the_derivation_produces_no_name_the_vocabulary_does_not_carry`, 65-83; `the_date_palms_recorded_needs_read_as_one_met_and_five_absent`, 118-132 | Add `"date-palm"` to `PRESETS`; bump `6` to `8`; rewrite the third test to expect `Expressed` for the two names | Yes, exact for `:9-17`, `:55`, `:66-83`, `:118-132` |
| `crates/telperion-jev/tests/dial_table.rs` | `SWITCHES_AT_ZERO`, 35-62; `every_numeric_row_of_every_family_is_a_dial_or_an_excluded_row`, **102**-134; `every_authored_row_is_a_dial_the_loop_can_ask_about`, **135**-185; `every_dial_steps_to_a_value_the_generator_accepts`, **188**-230; `every_row_that_switches_a_feature_on_says_what_zero_does`, **231**-247 | Add `rosette_fronds`, `rachis_length` to `SWITCHES_AT_ZERO`; ten new dials must satisfy all four coverage tests | `:35-62` exact. The four test-function citations (`:101-132`, `:134-185`, `:187-228`, `:230-245`) are each **one line early** — every function actually starts one line later than cited (102/135/188/231, not 101/134/187/230). Content and scope are otherwise correct |
| `crates/telperion-core/tests/rosette.rs` | new file | Build `Preset::from_id("date-palm")` at seed 1, assert placed count, position, and `rosette_fronds = 0` parity | N/A (file does not exist yet) |
| `crates/telperion-core/tests/species.rs` | `fn digest`, 37-60; `fn check`, 63-74 | No structural edit; add one positive-control test asserting the palm's digest changes when `rosette_fronds` is raised | Handoff's `:37-72` covers both functions but the digest/check pair actually spans 37-74, two lines past the cited end |
| `crates/telperion-jev/src/pipeline/stages/gate.rs` | `fn capability_gate`, 172-217; `fn preset_table`, 224-255 (unproduced filter 235-239, empty-check block 240-250) | No edit; re-run only | `:172-222` overshoots the actual function end (217) by 5 lines — `preset_table` starts at 224. `:235-239` for the `DERIVABLE.contains(...)` filter is exact. `:235-250` for "no `unproduced` name" is approximately right (the construction block runs 235-254) |

## Counts

- **Files touched (implementation, excluding new test files and evidence):** 8 — `foliage/placement.rs`, `foliage/rosette.rs` (new), `foliage/reference.rs`, `params.rs`, `capability.rs`, `specimen/view.rs`, `blend.rs` (not named by the handoff), `crates/telperion-jev/data/dials.json`. Test-only files touched: `tests/capability.rs`, `tests/dial_table.rs` (no code edit, just satisfied), `tests/species.rs` (one added test), `tests/rosette.rs` (new).
- **New wire rows:** 10 (`rosetteFronds`, `rosetteDivergence`, `rosettePitch`, `rosettePitchSpread`, `rosetteDepth`, `leafletCount`, `rachisLength`, `leafletPitch`, `rachisArch`, `terminalLeaflet`), all under `/canopy/`.
- **Authoring sites per row:** the handoff names five (struct field + doc comment, `fields!` macro line, validate range-table entry, dial-table row, and the `SWITCHES_AT_ZERO` two-of-ten). Direct inspection finds a **sixth**, unnamed in the handoff: a `blend.rs` bucket entry (`walk!(linear/degrees/counts/density/many: ...)`) — every existing `CanopyParams` field is bucketed there for family-to-family interpolation, and no coverage test was found (searched `crates/telperion-core/tests/` and `blend.rs` itself) that would fail if a new field were left out of every bucket. An omitted row there is a silent gap, not a red test.
- **Call sites whose signature changes, canopy vs. element:** with the rows on `canopy` (the handoff's choice), zero call sites need a new parameter — confirmed by counting actual callers: `foliage::place(...)` has 9 call sites (`tests/species.rs:396`, `tests/limb_clumping.rs:40`, `tests/short_shoots.rs:49,296,367,370`, `src/params/tests.rs:174`, `examples/measure.rs:36`, `tests/field.rs:108`); `place_on_surface(...)` has 9 (`src/mesh.rs:84`, `tests/surface_attachments.rs:63`, `examples/occupancy_audit.rs:80`, `tests/foliage.rs:160`, `examples/species_measure.rs:68`, `tests/twig_generations.rs:267`, `examples/geometry_benchmark.rs:41`, `examples/generation_stages.rs:41`, `telperion-render/src/generation/tests.rs:137`); `leaf_count(...)` has 1 (`src/footprint.rs:60`); `Reference::from_params(...)` has 1 (`src/foliage/reference.rs:33`, inside `Reference::of`); `Foliage::new` (the handoff's "`Placement::new`") has 1 direct construction path (`branching/specimen/timeline.rs:47` calls `foliage::timeline::Foliage::new`). Total: **21 existing call sites** that stay untouched under the `canopy` placement. Putting the rows on `element` instead would touch all of these plus `mesh.rs:82`'s `build_element` call and every site that builds an `ElementParams` value, which the handoff estimates at "roughly a dozen" more — not independently recounted here since the host has already decided `canopy` (fn-109 spec, "Host decisions," 2026-09-22).
- **Existing tests that go red before the implementation:** 4, all confirmed by reading their current bodies —
  1. `crates/telperion-core/tests/capability.rs::the_six_names_the_derivation_produces_are_all_in_the_vocabulary` (line 55: `assert_eq!(DERIVABLE.len(), 6)`) — red once `DERIVABLE` grows past 6.
  2. `crates/telperion-core/tests/capability.rs::the_derivation_produces_no_name_the_vocabulary_does_not_carry` (66-83) — red unless a listed preset's table is made to produce every newly derivable name (motivates adding `date-palm` to `PRESETS` with `rosette_fronds` etc. set).
  3. `crates/telperion-core/tests/capability.rs::the_date_palms_recorded_needs_read_as_one_met_and_five_absent` (118-132) — currently asserts `Support::Absent` for `apical-rosette` and `pinnate-frond`; red once they move to `EXPRESSED`.
  4. `crates/telperion-jev/tests/dial_table.rs::every_numeric_row_of_every_family_is_a_dial_or_an_excluded_row` (102-134) — red the moment the ten wire rows land in `params.rs` before `dials.json` gains them.
  Not red: `crates/telperion-core/tests/capability.rs::the_shipped_presets_derive_what_they_derive_today` (88-115) hardcodes derivation for the five already-shipped presets, none of which sets `rosette_fronds`, so it stays green throughout.
- **Tests that pin byte-identity of shipped presets:** `crates/telperion-core/tests/species.rs` digests four species (`silver-birch`, `oregon-white-oak`, `european-beech`, `norway-spruce`) against `tests/species/digests.json`, confirmed to hold **12-13 seeds each** (`1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233`, plus `4250668600` for `norway-spruce`) — broader than the handoff's shorthand "seeds 1, 2, 3, 5, 8". Separately, `catalogue/*/pins.json` exists today for `european-ash`, `european-beech`, `norway-spruce`, `oregon-white-oak`, `silver-birch` (5 files, read by `tests/catalogue/pins.rs`'s helpers, consumed by per-species test files, not a single aggregate test). **`catalogue/date-palm/` has no `pins.json` yet** (only `packet/profile.json` exists), so the palm itself is not under that particular pin mechanism today — the byte-identity proof for the palm rests on the digests file and its own value table staying at defaults.

## Existing precedent

The handoff names `short_shoots.rs` as the code-shape template and fn-33 as the decision precedent for grouping at the placement. The commit that actually built the short-shoot placement source (the closest prior feature adding foliage placement rows to `CanopyParams` plus a sibling module under `foliage/`) is:

- **Commit:** `57b2401c` "feat(foliage): short shoots clothe the limbs", 2026-09-15, under spec fn-50 (merged into the fn-34 integration branch via `a70dd57a`).
- **Size:** 12 files changed, 844 insertions(+), 14 deletions(-). New files: `crates/telperion-core/src/foliage/short_shoots.rs` (228 lines) and `crates/telperion-core/tests/short_shoots.rs` (472 lines). Touched: `blend.rs` (+/-27), `foliage.rs` (+4), `foliage/placement.rs` (+/-26), `foliage/station.rs` (+/-6), `params.rs` (+5), `params/tests.rs` (+11), `specimen/view.rs` (+/-17), `tests/geometry_benchmark.rs` (+16), `harness/params.test.ts` (+6), `src/browser/presets.generated.ts` (+40, an auto-generated file per its own header, "Generated by scripts/build-wasm.mjs from Rust presets. Do not edit.").

This confirms two things the design handoff does not surface: (1) `blend.rs` was in fact touched (+/-27 lines) the last time `CanopyParams` grew rows this way, consistent with the gap found above; (2) a downstream generated TypeScript file (`src/browser/presets.generated.ts`) and a harness test (`harness/params.test.ts`) were also touched, via the build step, not by hand.

The capability-vocabulary side (`EXPRESSED`/`UNEXPRESSED`/`DERIVABLE`/`derived`) itself was built by a separate, larger commit:

- **Commit:** `c9978710` "The generator declares what it can express (#43)", 2026-09-19, spec fn-84.
- **Size:** 11 files changed, 853 insertions(+), 78 deletions(-). New: `crates/telperion-core/src/capability.rs` (242 lines), `crates/telperion-core/tests/capability.rs` (192 lines). Touched: `examples/geometry_benchmark.rs` (-, refactor), `crates/telperion-jev/src/pipeline/stages/gate.rs` (+165/-), `crates/telperion-jev/tests/stages_downstream.rs` (+203/-), `docs/species-pipeline.md` (+42/-).

fn-109's touch surface (8 implementation files plus 2-4 test files) is smaller than either precedent on its own, since both the placement pattern and the vocabulary machinery already exist; fn-109 only has to add rows and two `if` clauses to structures both precedents built from scratch.

## What the handoff leaves to the implementer

None of the five "Remaining unknowns" are left to the implementer — the fn-109 spec's "Host decisions on the design handoff" section (2026-09-22) already resolves three of the five as system design calls, and marks the other two as open for a different owner (fn-82), not for whoever writes the code:

1. **Whether a rosette-bearing apex also suppresses twig wood** — settled by the host: "An apex that bears a rosette bears no twig." Not open.
2. **Whether the pinnate rows live on `canopy` or `element`** — settled by the host: `canopy`, authored once; fn-33 is to adopt these names for its own leaflet half. Not open.
3. **Whether `pinnate-compound` also moves into `EXPRESSED`** — settled by the host: yes, moves with the other two. Not open.
4. **What `species_measure` reports for a frond** — left open, but explicitly to fn-82 ("fn-82's to say"), not to this implementer.
5. **What the rosette's pitch does on a leaning stem at `rosette_depth = 0`** — left open as a botanical judgment; the design's stated default (axis, not sky) stands unless a later owner verdict on the rendered still asks for the other. This is the one place an implementer proceeds on a stated default without a host ruling overriding it.

Two further places in the design handoff itself use a range rather than a fixed value, which is a design choice already made (not deferred): `rosette_fronds` validated `0..=128` and `leaflet_count` validated `1..=256` are stated as the authored rails, not placeholders — the design handoff does not flag either as undecided.

## Nothing decided

This document gathers and cross-checks what the design handoff and the code say against each other; it makes no design, scoping, or implementation choice.
