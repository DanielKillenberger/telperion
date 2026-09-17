---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-34-the-silver-birch-as-a-real-species.1 Implement the silver birch as a real species

## Description
Onboard the silver birch as an fn-9-style packet and value table, run the fixed and fresh seed protocol, and take the owner's verdict in the harness. The beech and ash packets gathered here carry over to fn-62 and fn-56.

## Acceptance
- [x] R1 packet under `.flow/evidence/fn34/silver-birch/` with gating and contextual ranges and labelled estimates
- [x] R2 `silver_birch` preset as a value table; every identity site updated; no generator or renderer change
- [x] R3 numeric protocol 48/48, none capped; the owner accepted round 26 in the harness on the mature path
- [x] R4 ash and beech moved to fn-56 and fn-62 with their packets
- [x] R5 identity pins, sweep bands, mature height-by-age; pins re-recorded once per round that moved the birch

## Done summary
The silver birch ships as a catalogue species on the mature path, accepted by the owner at round 26 in the live harness on 2026-09-18 after fn-65 made the harness draw the direct build: "alright i will accept this. It looks great now". Round 26 moved four rows of the value table, two limbs a station at 45° instead of four at 62°, and seven twig laterals at 0.55 of the parent's length instead of eight at 0.6, which took the lower crown from 27 primaries to 13 and the tree from 54,512 branches to 33,526 at seed 1, with the whole-tree centre on the photograph's 83. The round-25 acceptance on the judging page was withdrawn on 2026-09-17 when the harness showed the growth path's tree; that path is hidden since fn-65.

Three defects the owner named are outside this spec and stay real on the mature path: the ring at the fork (fn-66), the flat-ended limbs (fn-4), the veins' tone and relief (fn-60). The European beech continues in fn-62, the ash in fn-56.

stage: impl-review - skipped(config: review.backend none; the host checked each round's diff)

## Evidence
- Commits: 04d7ce8afc511525101f7e1a9b807c0823710827, e2309e2b84255ef0ba9171bbad262db163b118b8, 3b635d6f3fa645914c621ad7b0730133bd5921c9, 3ebe0f19e6ce9ba7ec38573b5d010600968b9a8c, cbeb9223507551f1d2ef3772e4bb833876fdbe37, c5a9ac7d1523f9e076b5c5949ff9ad0390bd3f47
- Tests: npm run species:qa (48/48 numeric, none capped; visual unassessed by design), cargo test --release --workspace, clippy -D warnings, cargo fmt --check (round 26 worker), npm test (77 then 83 after fn-65), npm run typecheck
- PRs: https://github.com/DanielKillenberger/telperion/pull/29
