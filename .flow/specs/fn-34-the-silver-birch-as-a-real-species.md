# The silver birch as a real species

## Conversation Evidence

> user (2026-09-14, on the natural base of each legendary tree): "Onboard the base species"
> user (2026-09-14, on where that work lives): "Separate species spec"
> user (2026-09-15): "yes move ash to its own spec"
> user (2026-09-16, on moving the beech to its own spec): "ok let's do that."
> user (2026-09-18, the silver birch in the harness, seed 1): "alright i will accept this. It looks great now"

## Goal & Context
<!-- scope: business -->

The silver birch (Betula pendula) is a catalogue species: a frozen botanical profile with cited ranges, catalogued references, a preset that is a value table, the fixed and fresh seed protocol, numeric gates, and the owner's accepting verdict taken in the harness on the mature path. A game developer selecting `silver-birch` gets a tree that reads as a weeping birch at every seed: two stems from a forked bole, a crown of long hanging curtains that clear the stems for their first two metres, and a white bark with dark lenticel plates. [paraphrase]

The owner chose the birch, with the beech and the ash, as the natural bases of fn-10's legendary trees: the White Tree of Gondor grows from the birch. This spec began as all three species; the ash moved to fn-56 on 2026-09-15 and the beech to fn-62 on 2026-09-16, and this spec ships the birch alone. The lesson is the one-species-per-spec rule in CLAUDE.md. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **The packet.** `.flow/evidence/fn34/silver-birch/` holds `profile.json`, `references.json` and `species.json` in the fn-19 schema, filled by `docs/species-onboarding.md` stage by stage. The beech and ash packets gathered here stay beside it for fn-62 and fn-56. [paraphrase]
- **The preset is a value table.** `silver_birch` in `crates/telperion-core/src/presets/species.rs`, one `Preset` variant with no generator or renderer branch. The rows that make it a birch: two stems leaning 28° from a fork half a metre up; two limbs a station at 45° from vertical, stations 1.2 m apart, the shoots under them drooping to a full hang (`rise_secondary` −0.85); seven twig laterals a station at 0.55 of the parent's length; a curtain of 3 m strands with a ragged hem that drops six tenths of the way to the ground and clears the stems by 1.8 m; an 18 m envelope, 0.36 spread; a small rhombic leaf with a fine serrated margin; a white bark row from fn-40. [paraphrase]
- **Every identity site is updated.** The catalogue, the enum and its id parser, the profile-id map, the identity and sweep lists, the measurement example, the browser bindings and the generated browser catalogue all carry `silver-birch`. [inferred]
- **Judging.** Matched shots against the reference photographs (fn-36's rig: S-WHOLE, S-BARE, S-BARK, S-LEAF, S-SHOOT) plus two views the matched cameras never frame, S-FORK from 2 m at the base with leaves hidden and S-UNDER from inside the lower crown at eye height. The record of every round's numbers is `.flow/evidence/fn34/rounds.tsv`; the accepted round's candidates are `round26-trials.tsv`. The per-round narrative was removed on 2026-09-18 and is in git history at `e5eaa4fd`. [paraphrase]
- **Growth traits are copied, not calibrated.** The birch carries the oak's growth traits and a mature-height-by-age entry. The growth path is a hidden feature since fn-65; the birch is judged and shipped on the direct build. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The birch has a frozen profile, catalogued references and a species record in the fn-19 schema, with gating and contextual ranges cited and estimates labelled. [user]
- **R2:** The birch ships as one named preset that is a value table in `presets/species.rs`, selectable natively, through wasm and from the browser, with every identity site updated and no generator or renderer change. [paraphrase]
- **R3:** The fixed and fresh seed protocol passes the birch's gating ranges, 48 of 48 numeric cases with none capped, and the owner has accepted the birch in the harness on the mature path. [paraphrase]
- **R4:** Moved. The ash's template, protocol and judgement are fn-56's; the beech's are fn-62's. Their profiles, references and species records gathered here carry over. [user]
- **R5:** The birch has identity pins, sweep leaf-count bands and a growth-reference entry, and the same seed and parameters yield a byte-identical tree; a moved pin is re-pinned once with the reason stated. [inferred]

## Boundaries
<!-- scope: business -->

- No new generator or renderer capability inside this spec; every gap became its own spec, fifteen of them, listed under Decision Context. [user]
- No legendary trees; fn-10 builds on this base. [paraphrase]
- No growth-trait calibration through time. [inferred]
- Three defects the owner named stay outside: the ring at the fork is fn-66, the flat-ended limbs are fn-4, the veins' tone and relief are fn-60. [paraphrase]

## Decision Context
<!-- scope: both -->

- **One species per spec.** Three species, fifteen capability dependencies (fn-36 to fn-40, fn-44 to fn-52, fn-54) and twenty-six rounds. The ash moved to fn-56 on 2026-09-15 and the beech to fn-62 on 2026-09-16 after a path review recorded in memory as the finish-line decision. [user]
- **Judged where it is seen.** The round-25 acceptance on the judging page was withdrawn on 2026-09-17 when the harness showed a different tree. The harness drew the growth path (590,410 nodes at seed 1) while every still built the mature tree directly (73,337). fn-65 made the direct build the product and hid growth; the round-26 verdict was retaken in the harness with the base and in-crown views added. Recorded in memory. [user]
- **Round 26's rows.** Two limbs a station at 45° instead of four at 62°, seven twig laterals at 0.55 instead of eight at 0.6: the lower crown from 27 primaries to 13, the tree from 54,512 branches to 33,526, the whole-tree centre on the photograph's 83. Nine candidates were measured and four rendered; the two lighter ones overshot the photograph. Recorded in memory as the measure-before-render practice. [paraphrase]
- **The growth traits stay copied** from the oak with a height-by-age target, because fn-30 owns calibration and growth is hidden. [inferred]

## Owner verdict

- **Round 26 (2026-09-18), the silver birch in the harness on the mature path, `fn-34-integration` with fn-65 merged, seed 1.** Accepting. "alright i will accept this. It looks great now". Twenty-five earlier rounds from 2026-09-14 to 2026-09-16, their numbers in `rounds.tsv` and their words in history, ended in the withdrawn page acceptance of round 25.

## Strategy Alignment

- Follows "Growth and botanical fidelity": a measured real-species profile makes the branching and foliage rules answerable to references, and the verdict is taken on the tree that ships.
- Follows "The core and integration": one value table reaches native and browser consumers with no renderer change.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1, R2, R3, R5 | fn-34-the-silver-birch-as-a-real-species.1 |
| R4 | moved to fn-56 and fn-62 |
