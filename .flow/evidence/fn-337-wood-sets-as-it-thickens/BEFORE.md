# fn-337 R4: the old oak before the rule

fn-332's probe (`crates/telperion-core/examples/trunk.rs`), the oak as shipped at `fcbb6834f`, seeds 1, 7 and 42. The steepest lean is the largest per-metre angle from vertical the probe prints; over 90 degrees is a metre that runs downhill.

- **40 and 80:** run for this child, 9 Nov 2026, by `probes/trunk.sh before 40 80` inside one `telperion-heavy 8 --threads 4` claim. Logs: ignored `raw/before/`. Every number matches fn-332's R1 table (`26013bcdc`) to the printed digit.
- **120, 200 and 300:** reused from fn-336 (`../fn-336-sheets-of-020s-trees-as-they-stand/raw/trunk/`), grown at `a755efe26`. `git diff --stat a755efe26 fcbb6834f -- crates src` is empty, so they are the same trees; the steepest lean was read from those logs here.

| Age | Seed | Highest tip m | Trunk path ends: high, out m | Crown's mean off the base m | Steepest lean deg | Wall s | Peak GiB |
|---:|---:|---:|---|---:|---:|---:|---:|
| 40 | 1 | 10.61 | 10.61, 0.65 | 0.60 | 19.8 | 0.7 | 0.22 |
| 40 | 7 | 11.08 | 9.94, 1.65 | 0.35 | 31.3 | 1.0 | 0.32 |
| 40 | 42 | 10.02 | 9.90, 2.42 | 0.41 | 70.6 | 0.7 | 0.21 |
| 80 | 1 | 16.54 | 14.60, 6.48 | 1.58 | 60.3 | 7.6 | 1.26 |
| 80 | 7 | 20.57 | 10.22, 3.92 | 1.81 | 69.8 | 14.8 | 2.41 |
| 80 | 42 | 16.96 | 13.70, 8.61 | 2.29 | 87.3 | 6.1 | 0.92 |
| 120 | 1 | 23.24 | 15.11, 10.34 | 8.97 | 96.4 | 26.0 | 2.11 |
| 120 | 7 | 29.08 | 7.02, 7.84 | 3.87 | 121.0 | 55.5 | 4.60 |
| 120 | 42 | 26.93 | 16.77, 14.57 | 3.23 | 91.3 | 22.2 | 2.06 |
| 200 | 1 | 24.89 | 22.87, 15.65 | 15.65 | 80.1 | 54.9 | 2.00 |
| 200 | 7 | 32.99 | 9.12, 12.43 | 8.52 | 119.1 | 124.2 | 4.97 |
| 200 | 42 | 23.63 | 18.03, 5.70 | 11.91 | 116.3 | 59.3 | 3.11 |
| 300 | 1 | 36.74 | 33.53, 17.40 | 19.73 | 89.9 | 77.2 | 2.01 |
| 300 | 7 | 35.57 | 18.93, 14.44 | 12.47 | 145.7 | 174.2 | 4.36 |
| 300 | 42 | 35.78 | 28.89, 5.66 | 6.16 | 112.5 | 90.3 | 2.74 |

Median seed of the crown's mean: 0.41 m at 40, 1.81 at 80, 3.87 at 120, 11.91 at 200, 12.47 at 300 (the goal's R4 asks within 3 m at 200 and 300).

Wall and peak are the probe process through fn-330's `peak.py` at rayon 4 (the census on), not R6's one-thread cycle time, which is measured separately.
