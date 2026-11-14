# fn-329 result: the coarsest tree's time in Wasm

Protocol: `PROTOCOL.md`, frozen before any number was read. Probe: `probes/wasm.mjs`. Raw lines: ignored `raw/wasm.jsonl`.

## R2: the numbers

Median of seeds 1, 1234 and 5678, each the median of five warm builds with a grid-36 field, Node v26.12.1 on one pinned core of the 5950X, the package built at `11bd70e49`:

| Preset | Fidelity 1 (coarsest) | Per seed | Fidelity 0 (full) | Native, fidelity 1 | 0.1.4, browser |
|---|---|---|---|---|---|
| Ordinary | 535 ms | 444, 535, 547 | 1,000 ms | | |
| Oak | 164 ms | 144, 254, 164 | 3,584 ms | 149 ms | 233-260 ms |
| Spruce | 726 ms | 697, 751, 726 | 11,947 ms | 617 ms | 203-212 ms |
| Beech | 293 ms | 321, 167, 293 | 258 ms | | |
| Birch | 219 ms | 219, 301, 198 | 1,230 ms | 193 ms | 239-304 ms |
| Date palm | 17 ms | 20, 17, 17 | 17 ms | | |
| Telperion | 9,144 ms | 12,346, 9,144, 9,063 | 9,836 ms | | |
| Laurelin | 16,820 ms | 26,168, 16,820, 16,089 | 8,845 ms | | |

No tree was refused. Growth is 88 to 95% of each build and the field read 4 to 8%, except the date palm, whose 17 ms is mostly the field (about 10 ms) and 2 ms growth.

## R3: the reading

- **Within 50 ms:** the date palm only.
- **Within 0.1.4's time:** the oak (164 against 233) and the birch (219 against 239). The spruce is 3.4 times 0.1.4's 212 ms.
- **Wasm against native:** 10 to 18% slower (oak 1.10, birch 1.13, spruce 1.18). The website's cost is the native cost plus a little.
- **The goal's question:** the oak and the birch reach 0.1.4's time and not 50 ms, the case the goal's route sends to the owner. It is logged in the goal with these numbers, beside the owner's later ruling against per-preset rows.

## A defect it found

Laurelin's coarsest is slower than its full cycle, 16,820 ms against 8,845 ms, and the beech's is 293 ms against 258 ms; Telperion's barely changes. These presets have no coarse rows, so the dial's default does not save work on them, and the goal's R11 forbids any setting slower than the full cycle. Not yet diagnosed. It belongs to whatever replaces the per-preset rows, which the owner ruled against on 3 Nov.

## R4

Measurement only: the commits hold `.flow/` evidence and no product code, so there is no gate change and no review (fn-322's rule).
