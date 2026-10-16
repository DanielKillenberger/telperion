# The twig layer grows without the crown as a wall

## Goal

The twig layer treated the crown's outline as a wall: each planned stride asked the outline whether it was inside, a refused stride started a 40-step bisection, and terminal and leaf twigs had their own admission. That wall was most of growth's crown queries (56% on the oak, 60% on the beech, 74% on the birch at seed 1) from the 7 to 11% of axes that reach it. The owner asked why it must be checked at all; twigs already take their length from their own rules, so the crown's edge can come from architecture. The work is removed, not made cheaper (`docs/principles.md`, "Question, delete, then optimise").

## Decisions

- user (2026-10-02): "one question i have is why do we even have to check this? couldn't we embed the growth rule differently without capping an exact crown width?"
- user (2026-10-02, on twigs planned from their room instead of clipped at the outline): "that sounds like it'd fit a tree crown better anyway?"
- user (2026-10-02): "so this is what i'm talking about when i asked about a fundamentally more elegant solution. How could we have landed there quicker?", then "yes add it and then spec the new spec" (now `docs/principles.md`, "Question, delete, then optimise").
- user (2026-10-02, on the beech now waiting behind fn-182, fn-173 and this spec): "that's fine let's do this proper"
- Astra review (2026-10-02) reframed it: "twig extent follows branch architecture; outline queries must earn their place"; added the zero-query and inherited-allowance rungs, the terminal admission, whole-run excursion and density checks; the first draft's light-starved-edge claim was withdrawn (Sterck and Bongers 2001; Buck-Sorlin and Bell 2000, cited by Astra, not read here).
- Decision (owner, 2026-10-02, on the R1 stills; replaces the host's first pick of rung 3b, the directional probe): rung 1, the twig layer asks the crown nothing. The owner: "option 1 looks better? less regular. No tree looks like a perfect sphere on the edge?" The curtain keeps its floor, pendulous length, sag and clearance. No trunk-height floor, no excursion limit (excursion reported, never gated), no new row.
- R4 targets set from R1: queries at least 80% lower on the oak, 85% beech, 95% birch, 30% spruce, 40% Telperion; growth time at least 20% lower on the beech and 80% on the birch; no preset slower (warm median outside the base's warm range, interleaved rounds).
- Host decisions on what removing the wall broke (2026-10-02): packed-leaf box and field cells sized from the grown tree; curtain floor held by an exact height cut per stride; the hang row fades in from zero; containment assertions retire to excursion reports; the drop walk's monotonic check takes a 1 cm tolerance (why the dial is weak is fn-184's R1); the surface-collapse generator fixture is retired (no catalogue tree reproduces a drop in seeds 1 to 64); the oak's stuck crown width traced before deciding.
- owner (2026-10-02): no twig layer where the pipeline clears it; host: an apex bearing a rosette is not a tip and keeps its girth (the palm's crown top had been 0.24 m or 3.5 mm by accident).
- owner (2026-10-02), decision on the containment tests, recorded as: "Deleted, owner (2026-10-02): the containment parts of `species::fixed_*`, `growth`, `habit`, `drop` and the shortened-limb `limb_tests`. The scaffold-in-share check stays." (source: archived R4-RESULT.md)
- fn-173 waits on this spec and is re-scoped from its residual time; fn-182 follows this spec. The 40-to-10 bisection fix is not taken (it keeps the wall).

## Acceptance Criteria

- **R1:** `growth_profile` reports queries by exclusive caller and purpose; each rung measured on five presets at seeds 1 and 7 with stills — met (`query-count` feature; ladder in `R1-LADDER.md`).
- **R2:** The host's decision recorded before production code — met (rung 1, above).
- **R3:** The mature twig layer makes no outline query for any limb system; curtain floor holds; builds repeatable; excursion reported — met (`suite::twig_extent::the_twig_layer_asks_the_crown_nothing_and_builds_one_tree`; the growth path keeps its wall).
- **R4:** Paired runs meet the targets with no preset slower — met: queries −83 to −86% oak, −87 to −88% beech, −98% birch, −35% spruce, −44 to −49% Telperion; growth time −28 to −29% beech, −87% birch (183 to 23 ms), −10 to −26% oak; nodes and leaves rose 7 to 18% on oak, beech and birch (no crown emptied).
- **R5:** Owner's visual verdict on oak, beech, birch, spruce and Telperion — met (below).
- **R6:** Gates green; cost and sizes reported — met: 1,105 passed, 0 failed; `npm test` 141 passed; peak RSS +1.6 to +6 MB; artifacts +0.07 to +0.12%.

## Result

The direct build's twig layer asks the crown nothing. What remains is the scaffold's containment and sampling plus shedding (230k of Telperion's 239k queries at seed 1). Whole-run excursion past the lobed outline: 1 to 14% of axes, p95 0.18 to 0.49 of the widest radius on the main presets. The oak's crown width now varies by seed (27.6 to 31.1 m) because the species gate reads the grown leaf box. `clear_apical_twigs` was removed. Codex review SHIP.

Owner verdict (2026-10-02), on the final stills beside today: "i'd say most look better. beech still not good but that's not part of this spec". The beech's structure is fn-182's.

## Open

The beech's structure (fn-182, then fn-190); the drop dial's weak effect (fn-184); fn-173 re-scoped from the residual queries.

## Friction

- The dcg hook refused a truncating redirect in the measurement driver -> local setup, reported.
- The earlier count hid the curtain's lower-surface search inside "bisection" -> the exclusive purpose scope separates them.
- The `query-count` build is ~40% slower -> counters compile out for timing.
- Rung 1 broke consumers that assumed the wall, found only after R1 (the suite never ran on the exploration build) -> run the lib suite `--no-fail-fast` on exploratory code in R1.
- Pins live in four places with partial print helpers -> proposal: one pin recorder for every pin file, cross-crate pins included.
- The workspace gate ran twice for cross-crate pins -> same recorder plus a gate-doc note.
- `telperion-jev` `objectives.rs` raced on one temp dir -> fixed (one directory per test).
- The beech replay tape cannot be re-recorded offline -> the beech replay through Tune was dropped (host).
- Worktree had no `node_modules` -> local setup.
- Skipping the palm's apex twig layer could not be byte-identical -> host decided the apex keeps its girth.

## Evidence

- `.flow/evidence/fn-183-the-twig-layer-grows-without-the-crown/R1-LADDER.md` (the ladder measurement; cited by `docs/principles.md`).
- `.flow/evidence/fn-183-the-twig-layer-grows-without-the-crown/QUERY-COUNTS.md` (crown queries by caller on master e12a9a28, with Astra's corrections).
