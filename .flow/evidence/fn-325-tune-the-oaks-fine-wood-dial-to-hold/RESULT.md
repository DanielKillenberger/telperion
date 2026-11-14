# fn-325 result: the dial holds the oak at 6.3 times the speed

**One candidate holds all six R4 medians within 5% at the oak's preset age on seeds that chose nothing, at 274 ms against 1,722 ms.** It does not reach R2's 50 ms or, natively, 0.1.4's oak time of 233-260 ms (in a browser). Going on looks achievable for R1, R4, R7 and "at or under 0.1.4's". 50 ms is not reached, and what limits it is below.

Measured 1 and 2 Nov 2026 on the integration branch at `dbaa69803` and later, plus the probe, native, `--profile ci`, one thread. Raw logs are in `raw/`, each round's freeze in `ROUNDS.md`, the search in `probes/`. No shipped tree changes.

## The candidate, A

On the oak's branch age and later (age 9 up): internode times 2, node bounds over 2, leaf area per node times 16, pipe times 2.5 over the unit-preserving `2^(1/exponent)`, lateral probability times 1.3, the shade and balance hazards times 2, shedding delay times 0.25.

| Held-out seeds, age 40 | Full cycle | A's ratio |
| --- | --- | --- |
| Height | 10.5 m | 1.019 |
| Crown spread | 7.7 m | 1.010 |
| Trunk | 0.68 m | 1.038 |
| Limbs | 471 | 1.017 |
| Shed share | 0.63 | 0.983 |
| Leaf area | 205 m² | 1.028 |
| Cycle | 1,722 ms | 274 ms |

It also held on tuning seeds 1001 to 1008, at worst 0.039 and 256 ms.

## The trade-off

Held-out seeds at age 40, all chosen before those seeds grew:

| Candidate | Worst departure | ms |
| --- | --- | --- |
| A | 0.038 | 274 |
| E | 0.095 | 236 |
| B | 0.095 | 135 |
| F | 0.141 | 106 |
| C | 0.132 | 109 |
| D | 0.161 | 80 |

Candidates tuned on two to five seeds lost several points on the held-out seeds, and A is the only one under 5%. With five seeds, a median itself moves by a few percent, so these rows are rough, and R4's 50 seeds are the real test.

## What limits the cost

- **Values have levelled off.** Three climbs moved the 5% line little, and scaling from age 7 or 8 is worse (best 0.116 and 0.079).
- **Cost follows apex-cycles.** Seed 1, age 40: the full cycle takes 3.4 us per apex-cycle, A 2.2 us, D 2.6 us.
- **Three passes take about 60%.** In A: balance 43 ms, units 35 ms, sketch 26 ms, of 165 ms. Light's field is 15 ms. So cost per bud in the engine is the remaining lever, untested. A sampling profile needs `perf_event_paranoid` at 1 or lower, set by root, and this machine's is 2.
- **Unknown:** Wasm's factor and the field's cost on these trees, so no browser figure exists yet.

## Age

| Age | A: seconds, height, trunk | Full cycle |
| --- | --- | --- |
| 80 | 2.0 s, 16.8 m, 1.08 m | 13.3 to 17.7 s |
| 120 | 4.3 s, 23.8 m, 0.98 m | refused (20M phytomers) |
| 160 | 7.1 s, 18.6 m, 0.82 m | |
| 200 | 9.0 s, 23.8 m, 0.67 m | |
| 300 | 12.6 s, 36.7 m, 0.90 m | |

Seed 1, one grow, one thread. So the coarse oak reaches age 300 with no refusal, and cost is close to linear past age 120, which the full cycle's power law is not. At 300, one seed's height sits over Silvics' 15-27 m, the trunk passes R7's 60 cm, and the crown of 21.3 m stays under the 38.4 m record. Height and trunk are not monotonic with age, since the preset's rows were set for ages 40-80. One seed, so this is reach, not R7.

**Values tuned at age 40 do not carry to age 80:** A at 80 has worst 0.189 (leaf area 0.81, limbs 0.86, trunk 0.86) at 2,079 ms against 17,669 ms. R4 reads the preset age, so this does not fail R4, but a fidelity holding at every age needs tuning across ages.

## Friction

- 1 Nov: one incremental build of the probe took over 5 minutes, against 13-22 s for the same build around it. Cause unknown, and it did not recur. Cost: about 5 minutes.
- 1-2 Nov: the `dcg` hook refused a redirect to a variable path and a heredoc holding backticks. Literal paths and script files replaced them, at about 2 minutes. Local setup, so reported only.
- 2 Nov: sampling profiles need a root setting (above). Pass timers stood in for it.
