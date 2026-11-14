## Goal

After fn-44 every strand of the birch's curtain ran the same 2.5 m to one level hem, a ball on a stick, where the photograph's strands run from a hand's width to three metres with a ragged hem. This spec varies the pendulous run per shoot by a seeded row, neutral at today's uniform curtain, judged on the birch's pairs.

## Decisions

- user (2026-09-15): "If you can see it's clearly not there and you have a way to fix it you should do that."
- worker read, fn-44 (2026-09-15): "85% of hanging shoots run exactly 2.50 m ... No twig row varies a pendulous run per shoot."
- worker read, round 7 (2026-09-15): "A round ball on a V. The curtain stops at one height all the way round, where the photograph's curtain hangs lower on the leaning side ... it comes from the uniform strand length and the single floor height."
- The draw is keyed on shoot identity and seed, never growth order; the hem follows the wood (no per-side term); sag spends over each shoot's own run. No change to sag, floor or separation.

## Acceptance Criteria

- **R1:** `skeleton.twigs.pendulousVariation` rail 0 to 1, neutral 0, refused by name, wire, blend, metadata, harness; presets byte-identical at neutral — met (birch re-pinned with the reason).
- **R2:** runs spread between the pendulous length and length times (1 - variation), keyed per identity, sag over each run — met (direct, monthly and uneven builds agree).
- **R3:** birch states a variation; pairs re-rendered; implementer visual QA; owner judges in fn-34 — met (0.95 over 3 m, round 8b, `round8b-fn47` rows in `.flow/evidence/fn34/rounds.tsv`, 48/48, heaviest birch seed 107,569 nodes).
- **R4:** tests for neutral identity, rail, spread, determinism across build orders, short-strand end angle, blend walk — met (`tests/strands.rs`).

## Result

Host direction added a lumpier shell (irregularity 0.35, lobe scale 0.25). Worker read: strands of different lengths show and the lower edge is uneven (18% of columns end clear of the round bottom, against 7%), but the tree still read as a round crown with a lumpy edge, because the curtain stays inside the shell; letting it hang past the shell or bunch per limb was left to the owner. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the beech's verdict moved to fn-62.

## Open

None.
