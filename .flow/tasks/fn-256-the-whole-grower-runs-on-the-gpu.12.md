# fn-256-the-whole-grower-runs-on-the-gpu.12 Three small cuts the re-measure named

## Description
Host rulings 2026-10-12 on task 11's calls: (1) retirement's inside marks taken once before each compaction and before the settle, not every cycle (same marks, measured); (2) a dormant lattice skips its deposit too: the deposit's refusal guards the lattice's input, which a dormant lattice never reads; (3) units rounds sized from the rounds a build actually used plus a stated slack, refused by name past it (task 6 option 2).

## Acceptance
Node-table hashes unchanged at caps 64/128/1,024; per-preset GPU time and dispatches a cycle before and after; gate and review.

## Done summary
All three host rulings are built, and none changes a word: the node tables and the stills are as before. Stamped GPU time over three alternating runs fell 10.5% on Telperion (733.6 → 656.5 ms), 4.0% on the oak (160.9 → 154.4) and 8.3% on the beech (114.5 → 105.0). PROOF.md "fn-256 task 12" has the tables, and DESIGN.md section 18 the design.

- **Inside marks:** retirement's inside sweep is recorded apart from the cycle. It runs before each compaction (ahead of any move) and before the settle, over the last submitted cycle's hierarchy, limits and given workgroups. On Telperion it falls from 974 dispatches and 14.6 ms to 90 and 1.4 ms.
- **Dormant deposit:** a dormant lattice now records neither the deposit, the quanta's clear nor the deposit-ranges flag. Telperion's light fell 62.2 → 14.9 ms. The dormant test holds every refusal but the deposit's to the swept lattice's.
- **Units rounds:** each round marks itself used where a job begins it with work left. A cycle records the rounds the census read used, plus a slack of 2 for the living apexes and 3 for the wake (the most either rose over two cycles on every preset at seeds 1, 7 and 42), up to a bound of 8. Past that the build is refused by name (`UnitRounds`). Telperion's units fell from 13,009 to 10,037 dispatches.
- **Dispatches a cycle, seed 42:** Telperion 414 → 365, Laurelin 408 → 360, beech 330 → 295, birch 356 → 320, palm 269 → 230, spruce 402 → 378, oak 370 → 339, ordinary 408 → 408. The ordinary's living use 5 rounds, so its late cycles record up to 7.
- **Exact:** all 24 node-table hashes (8 presets at caps 64, 128 and 1,024) match the post-task-9 hashes, and all 24 stills are pixel-identical to task 11's. The arena gate passes on every preset at seeds 1, 7 and 42 with no move and no refusal.
- **Gate:** the workspace without the renderer passed 1,066 tests; the renderer under the GPU lock passed 273; npm test passed 162.

stage: impl-review - ran (codex, three reviewers, base d8285cb3c: SHIP, no findings)

Tier: worker on the session model (Opus 5.5).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 55f6d4fe38fe7d72136fc788ec844980a30b049e, 9d12b725e07da26c4c10dcae6632eedd7087e86f, 2656c0ddbba8f65aa58a512968a08f7d842440c4, cd7af62be5854e27760affa0c0739dc8b3bb6569
- Tests: cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), npm test, node_tables_are_hashed NODES_CAPS=64,128,1024, every_preset_grows_in_its_hinted_arena, the_roofline_is_measured (measure.sh)
- PRs: