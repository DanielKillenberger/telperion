# Tree space: the silver birch as a point

## Goal

The silver birch (Betula pendula) as a point in the tree space, built through the engine so the `silver-birch` preset builds again on the engine path (fn-198 refused it by name) and killenberger.com's grove, where nearly every grove holds a birch, can draw on a released package. Owner, 2026-10-07: "build the 4 missing tree species. They don't need to look the same as today's harness implementations. Ideally they look better". The bar is a mature open-grown birch as reference photographs show it: slender, often curving white stem, narrow ovate crown, long fine pendulous outer shoots on ascending limbs, small diamond leaves on short shoots. One species per spec, modelled on fn-193 to fn-196; depends on fn-198 and fn-224.

## Decisions

- Stand on botany: mechanisms reproduced against sources, no external code or data copied; errors, not fallbacks; presets are values. A generator gap is its own spec, decided by the host.
- Gate: the host views every still first; the species passes when the host is confident it reads as a silver birch beside its references and Astra agrees independently. Wrong-path stop: two rejections naming the same trait after a change aimed at it.
- Owner, 2026-10-07, on round 7 against the old generator, verbatim: "birch crown of the old generator looked more convincing tbh. Not enough curtainesque branches/leaves/foliage. Branches bare look very unconvincing." The host then measured old against new (old: about 33,000 straight unbranched strands up to 3 m carrying two thirds of the leaves, 30 km of straight fine twigs; round 7: 3,000 to 14,000 short branching strands, 3 to 10 km of kinked twigs) and aimed rounds 8 to 10 at those gaps by values only; no row was taken from the old generator.
- Owner, 2026-10-08, on round 10: "structurally the birch looks almost right though the leaves look strange? weird color and are they small and numerous to appear like they just fill the twigs length." Round 11 set a 50 mm triangular-ovate blade (Flora of China) and leaf colour read by code from the S2-Oslo reference; round 12 seated three leaves at each short shoot's tip (fn-223 clusters); round 13 raised medial short-shoot shares to 0.85 (SU03); round 14 (host) cut only the short shoot's `pipe` 0.0007 to 0.0005 so old black bark no longer climbed the stem.
- After the second wrong-path stop on traits j and h (feather-like sprays), leaf clustering at the engine's short shoots was captured as fn-223.

## Acceptance Criteria

- **R1:** MODEL-BIRCH.md: architecture, long and short shoots, pendulous shoots, growth and form at 20 to 80 years, every value cited and screened — met (`MODEL-BIRCH.md`, rounds 1 to 14).
- **R2:** The birch's values as engine rows in the `silver-birch` preset, building natively and in the harness, refusal lifted — met.
- **R3:** Host confident and Astra agrees on a sheet of seeds 1, 7, 42 at 40 and 80, bare and in leaf — superseded by the owner's verdict (below); Astra not run from round 12 on, the owner judged.
- **R4:** A walk from the beech to the birch with no visible pop; other species' sheets unchanged — partial: the four other species' `space_hash` rows unchanged at the F4 merge; no walk strip recorded.
- **R5:** killenberger.com's field path for the birch measured — not recorded in this spec's evidence.

## Result

Round 14 (final): DBH 30.7 / 33.0 / 30.3 cm at 40 and 59.7 / 60.3 / 62.1 cm at 80 (seeds 1 / 7 / 42), within 3% of round 12's target stem and above LO96's stand-grown top diameters; short-shoot base radius p50 0.66 to 0.95 mm; largest tree 2.93 M nodes at 80 (under the 4 M cap), 1.07 to 1.20 M leaves at 80. White stem above the flare at 40; at 80 black bark only on the lowest metre. The crown still reads see-through and grey-olive beside S2-Oslo. F4 merged onto the birch (`MERGE-F4.md`) with the other four species' hashes unchanged; gate render 214/1 (a load-bound cold-build timing, inconclusive), workspace 1,065/0, `npm test` 143/143.

Owner verdict, 2026-10-15 (fn-237 R3 table): needs work but accepted for the release: "We still need to fix the ground branches and here also we need to be able to make larger species with big leading branches. But it's the best species." fn-286's take-over 0.5 approved: "definitely better".

## Open

- Ground branches (fn-282) and larger trees with big leading branches (fn-285), after the release.
- Crown colour and see-through at distance; per-shoot leaf count 2 or 3 is a host design question.

## Friction

- `cargo run -p telperion-jev -- screen` fails (three binaries, no `default-run`) -> `default-run = "jev"` or `--bin jev` in docs/typesafe.md.
- Shell guard refuses dynamic redirects and `git show > path` (several times) -> local setting, reported not specced.
- GPU lock held by another session's whole workspace gate (~15 min) -> take the lock only around render-crate binaries.
- A gate piped through `head` ended cargo early (~25 min) -> write gate output to a file.
- Gate's one failure was on the base (`generation_limits_are_classified`, shoulder.rs) -> reported, not fixed here.
- A value row at its default failed `preset_values` only in the full gate -> refuse default-stating rows at compile/still time.
- Compare-sheet scripts missing -> `compare_sheet.sh` kept beside the measures.
- No leaf close-up shot in the still runner -> add a `leaf` shot.
- Round 11b stills not reusable after a merge -> a still-runner option naming a value file.
- The brief's ancestry claim was wrong and timings scattered 2x on a shared machine -> state tips with `merge-base --is-ancestor`; time on an idle machine or report CPU time.
- Gate passed the 10-minute foreground limit twice under load -> longer ceiling or split the gate in halves.

## Evidence

- `.flow/evidence/fn-213-tree-space-the-silver-birch-as-a-point/MODEL-BIRCH.md` (model, sources, every round's values)
- `.flow/evidence/fn-213-tree-space-the-silver-birch-as-a-point/MERGE-F4.md` (F4 merge checks)
- `.flow/evidence/fn-213-tree-space-the-silver-birch-as-a-point/birch_measure.rs`, `birch_dbh.rs`, `compare_sheet.sh`, `leaf_colour.py` (measures, sheet command, leaf colour)
- `.flow/evidence/fn-213-tree-space-the-silver-birch-as-a-point/FRICTION.md` (kept: cited by fn-237, which is not edited here)
