# fn-330 protocol

Frozen 3 Nov 2026, before the first grow.

- **Grow:** `cargo run --profile ci -p telperion-core --example early -- three oregon-white-oak <age> 1 cycle`, the full cycle on the CPU with its census, seed 1, one process per age, `RAYON_NUM_THREADS=1` and pinned to one core so seconds compare with fn-323's one-thread reads.
- **Budget:** `EARLY_BUDGET=4294967295`, the most the request holds, so the budget does not refuse first.
- **Ages:** 80, 100, 120, 160, 200, 300, in that order; the run stops at the first age that is refused, killed or past its time.
- **Limits:** each grow runs in `telperion-heavy 10`, so a grow past 10 GB of memory is killed by name and not swapped, and `timeout 4h`. 10 GB is what the desk leaves for one job beside the owner's work; (first frozen at 12, lowered to 10 before any grow when the queue had 11 GB of room); a grow killed at it is reported as needing more, not as the machine's limit.
- **Recorded:** wall seconds and peak resident memory from `probes/peak.py` (the machine has no `/usr/bin/time`); from the census, phytomers grown in all, kept at the end, axes kept, the busiest cycle's active apexes and phytomers, and seconds by phase.

## Added after the first pass, 3 Nov 2026

The first pass grew ages 80 to 200 and the cap killed age 300 at 10 GB after 2 minutes. Age 300 is grown once more alone, the same command and seed, in `telperion-heavy 18`, which starts only when the desk has 18 GB of room beside its reserve. Nothing else changes.

## The age-200 tree, seen (3 Nov 2026)

`oak-age200-seed1.png`: the full cycle at age 200, seed 1, through the native still with the budget lifted in a local build (not committed). The trunk rises a few metres, then bends near horizontal and carries the crown far to one side; the crown is sparse. Owner: "yea that's bad". The full cycle has never been judged past age 80, so its old trees need fixing before they serve as references or training data.

## Age 300 after fn-333, 3 Nov 2026

fn-331 removed the budget (and `EARLY_BUDGET` with it) and fn-333 let the grower drop shed branches' records, the tree unchanged to the bit. Age 300 is grown once more with the same command and seed, now in `telperion-heavy 10`, the first pass's cap, alone on the desk. The ages 80 to 200 are not grown again: their trees are the same, and fn-333's RESULT.md gives their new peaks.

