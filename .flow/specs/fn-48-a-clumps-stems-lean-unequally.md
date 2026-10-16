## Goal

fn-38's clump leaned every stem by one angle about one bearing, so a two-stemmed birch parted in a symmetric V; the photograph's pair is one near-vertical stem and one leaning far out. This spec spreads a clump's lean across its stems by a row (neutral at fn-38's even lean), and, by the owner's later ask, lets a later stem part from the first at a stated height with a clean fork.

## Decisions

- user (2026-09-15, on the beech round-5 pair): "Second trunk is missing but that's coming later that's fine."
- worker read, fn-38 round 6 (2026-09-15): "the parting is a symmetric V, because both stems are spread about one bearing and lean by the same angle, so neither is the near-vertical stem the photograph's pair has."
- Owner, round 13 (2026-09-15): "Can we also have the second trunk be moved up/down?" then "if we can add it with a short task to the existing spec we can add it. If it's nothing major." (task 2, `stem_fork_height`).
- Host decision (2026-09-15): the ledge fix at the fork was major, so task 2 reverted the birch's fork height to 0; the owner then asked "ok capture the follow up task then pls" (task 3, the clean fork).
- Leans spread in stem order (first at `stem_lean * (1 - spread)`, last at the full lean); the alternative left an odd clump's middle stem upright. No coppice, stem death or roots.

## Acceptance Criteria

- **R1:** `skeleton.habit.stem_lean_spread` rail 0 to 1, neutral 0, refused by name, wire, blend, metadata, harness; presets byte-identical at neutral — met (seven identity pins held).
- **R2:** with positive spread leans run from `stem_lean*(1-spread)` to `stem_lean` in stem order; two stems on one heading refused — met.
- **R3:** birch states spread and lean; pairs re-rendered; visual QA; owner judges in fn-34 — met (two stems, divergence 0, lean 28, spread 1; round 9b, `round9b-fn48` rows in `.flow/evidence/fn34/rounds.tsv`; 48/48, heaviest birch seed 146,028 nodes).
- **R4:** tests for neutral identity, rail, leans in order, blend walk — met (`tests/stem_lean_spread.rs`).

## Result

- Task 1: one near-vertical and one leaning stem on S-BARE, S-WHOLE and S-BARK.
- Task 2: `skeleton.habit.stem_fork_height` (0 to 0.5 of crown base height, neutral 0): later stems born from the first at that height; DBH counts stems at breast height. The fork showed a ledge, so the birch was reverted to parting at the ground.
- Task 3: the trunk run continues into the straighter stem and eases its girth over the fork's diameter; the stem-or-limb flag survives every read (full, shoot-less, sparse interval, browser view; specimen snapshot schema 2) and the contact query picks the same run. Birch states fork height 0.5 (fork node at 0.96 m), the rail's top; S-WHOLE parts at the ground, S-BARE forks a third of its height up, so the row cannot satisfy both. Host read: no ledge, no seam. Round 20 (`round20-fn48c` rows), 48/48, presets byte-identical at fork height 0. Consequence: the S-BARK camera at 0.06 of the 16.43 m birch now frames the crotch.
- The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the European beech's verdict moved to fn-62.

## Open

None.
