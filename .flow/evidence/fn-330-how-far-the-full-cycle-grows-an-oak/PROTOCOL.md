# fn-330 protocol

Frozen 3 Nov 2026, before the first grow.

- **Grow:** `cargo run --profile ci -p telperion-core --example early -- three oregon-white-oak <age> 1 cycle`, the full cycle on the CPU with its census, seed 1, one process per age, `RAYON_NUM_THREADS=1` and pinned to one core so seconds compare with fn-323's one-thread reads.
- **Budget:** `EARLY_BUDGET=4294967295`, the most the request holds, so the budget does not refuse first.
- **Ages:** 80, 100, 120, 160, 200, 300, in that order; the run stops at the first age that is refused, killed or past its time.
- **Limits:** each grow runs in `telperion-heavy 12`, so a grow past 12 GB of memory is killed by name and not swapped, and `timeout 4h`. 12 GB is what the desk leaves for one job beside the owner's work; a grow killed at it is reported as needing more, not as the machine's limit.
- **Recorded:** wall seconds and peak resident memory from `/usr/bin/time -v`; from the census, phytomers grown in all, kept at the end, axes kept, the busiest cycle's active apexes and phytomers, and seconds by phase.
