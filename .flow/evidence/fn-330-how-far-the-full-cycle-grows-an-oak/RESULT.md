# fn-330 result: how far the full cycle grows an oak

Protocol: `PROTOCOL.md`. Runs: ignored `raw/oak-*.log`. Oregon white oak, seed 1, the full cycle on the CPU with its census, one thread pinned to one core, through `probes/peak.py`.

## R2: the reach

| Age | Wall s | Peak memory | Phytomers kept at the end | Phytomers grown in all | Axes laid | Busiest cycle's active apexes |
|---:|---:|---:|---:|---:|---:|---:|
| 80 | 14.1 | 1.83 GiB | 1,862,836 | 10,052,792 | 1,537,394 | 133,782 (cycle 80) |
| 100 | 27.7 | 3.23 | 2,558,342 | 19,360,178 | 2,976,812 | 181,634 (99) |
| 120 | 45.6 | 4.87 | 1,973,856 | 30,361,532 | 4,763,797 | 223,753 (113) |
| 160 | 74.2 | 7.04 | 2,684,485 | 47,308,647 | 7,413,156 | 229,655 (113) |
| 200 | 103.6 | 9.12 | 1,625,073 | 64,995,549 | 9,929,933 | 230,149 (113) |
| 300, first pass | killed at the 10 GB cap after 2 minutes | | | | | |
| 300, after fn-333 | 210.1 | 1.95 | 1,175,372 | 85,918,598 | 12,625,616 | 230,128 (113) |

Before fn-333 (rows 80 to 200) the phytomer budget was raised to the most a request held; fn-331 then removed it. The 300 row grew on fn-333's grower, which drops shed branches' records and grows the same tree to the bit (its RESULT.md: the oak at 80, 120 and 200 digests the same; its peaks there are 1.26, 2.19 and 2.22 GB).

Time rises a little faster than the phytomers grown: 1.4 µs per grown phytomer at 80, 1.6 at 200, 2.4 at 300. The busiest years are near 113, and after them the tree sheds more than it grows: the kept set falls from 2.7 million at 160 to 1.2 million at 300.

## R3: where the memory went

Before fn-333 the peak followed the axes laid, not the live tree: 9.12 GiB at 200 for 1.6 million kept phytomers on 222 thousand kept axes, about 0.96 KB per axis laid. fn-333's `BREAKDOWN.md` counted it: 95 % of the axis records at 200 sat inside subtrees already shed; the kept phytomers and their draws were 0.42 GiB, about 215 bytes each.

The three savings proposed on 3 Nov, against the 9.12 GiB at 200:

- **Dropping shed wood** (done, fn-333): 9.12 to 2.2 GB, three quarters of it.
- **A smaller phytomer**: at most the 0.42 GiB the phytomers and draws hold, a fifth of what remains. Not taken.
- **Merging settled wood**: not measured.

No estimate stands for 300: it was grown.

## R4: the next step

Full oaks at 300 grow on this machine as they stand: 210 s on one thread and under 2 GiB. Memory no longer limits the route of 3 Nov; the old trees' form does (fn-332: the trunk bends and one limb chain carries the crown). Seeds other than 1 can need more: seed 7 grows larger (fn-332's runs at 120 and over passed a 3 GB claim), so references at old ages are sized per seed by measurement, not by this row.

## R5

Measurement only. Its one code change, the `EARLY_BUDGET` variable in the `early` example, was removed by fn-331, whose gate and review covered that file; nothing of this child's code remains, so its review is skipped for that reason.
