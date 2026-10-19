# fn-256-the-whole-grower-runs-on-the-gpu.11 Dominant kernels, then the re-measure

## Description
COUNCIL round 5 step 11: optimise the remaining dominant kernels (occupancy, fusion, redundant scans) within the 2 s cold compile and watchdog margins; then every preset end to end native and through the owner's Chrome probe, against F4 and 0.1.4.

## Acceptance
Per-preset warm build, peak, dispatches and cold compile; the remainder against 0.1.4 named for the owner.

## Done summary
Six exact kernel and scheduling changes cut every build's stamped GPU time by 17-18%. Medians of three runs alternating with task 10's tip: Telperion 902 → 738 ms, oak 196 → 161 and beech 138 → 114. The changes are lane-parallel depth bounds in every block sweep, children's words loaded four at a time, the units' `written_main` split from `sprouted_main`, the normaliser's block in workgroup memory, the settle's dispatches sharing compute passes, and a dormant lattice keeping its field and scratch at their least. Every preset was then re-measured at seeds 1 and 42. PROOF.md "fn-256 task 11" holds the table against F4 and 0.1.4; DESIGN.md section 17 holds the design.

- Warm build, seed 42: Telperion 1,531 → 1,276 ms, Laurelin 1,382 → 1,177, oak 305 → 261, beech 230 → 190, birch 274 → 230, spruce 492 → 451, ordinary 522 → 458, palm 87 → 77.
- Device peak: Telperion 3,503 → 3,309 MB, Laurelin 3,209 → 3,029.
- Settle wall on Telperion: 165 → 101 ms.
- `inside_main` with its fold: 25.5 → 14.5 ms.
- Cold compile: 1.77 s, median of five, against 1.90 at the tip.
- Longest dispatch: 1.24 ms.
- Node-table hashes: 72 on three contexts, all equal to the post-task-9 hashes.
- Stills: all 24 pixel-identical to the post-task-9 stills.

Against 0.1.4, at its older ages, this branch is at or under on the beech and birch. It is over on the oak (1.5-1.7×), spruce (2.8×), Laurelin (2.1-4.2×), Telperion (1.5× on seed 1, 15× on seed 42), ordinary (7.5-7.9×) and palm (13×). Against F4 it is slower on the Two Trees (1.9×), spruce (1.12×) and palm.

Stopped, with no gain past noise: inside stamps (slower), and the sleeping-bud count in an entry of its own (its dispatches cost what it saved).

NEEDS_HUMAN (design calls, PROOF.md "Open"):
1. Take the inside marks only before a compaction and the settle. That would save 14.5 ms and about 25 dispatches a cycle on Telperion.
2. The dormant lattice's deposit, kept for its refusals, costs 48 ms on Telperion.
3. Task 6's dispatch-count calls still stand.

The probe needs no code change: the Wasm is built at this tip and the probe typechecks against it. PROOF.md gives the commands to serve it.

Follow-ups, not built: the split's per-node group loop is still serial dependent loads (58 ms on Telperion), and `gather_main` takes 57 ms.

## Evidence
- Commits: d56174069716936c79c15fb16d94c66be281a77c, 3f3295f39828271beafabf9ae60ff41138094ecf, dce760404e2fe7f806f5ae085e6d4fdce829bfdc, 2724e07f532584da7aa78068c56f114ba8ec6f96, 2692687525f1346d227693962f54fcd4cd3535af, 1b17faa93dcdb4161ceeab640207b663a9dcc4b3, 27db4ae5e00995ce209e0bab29f39ba412be670f, 0ebea5c0604fc209f1f9cc4060c47c02176ff985, 48558c46e4274cc46220aeb3f1b80dfeb8c74b02, 9e5faf3438375cd82a48959cd5e2d303bb081898
- Tests: cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), npm test, node_tables_are_hashed NODES_CAPS=64,128,1024 NODES_CONTEXTS=3
- PRs:
