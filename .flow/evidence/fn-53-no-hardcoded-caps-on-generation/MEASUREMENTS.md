# Mature direct-build compatibility and cost

Baseline: commit `38e6c57f1ee00836a6e44e667dd468dfebed51ce`. After: this task's implementation. Command: `cargo build --profile ci -p telperion-core --example generation_limits`, then the resulting executable with each preset id. Baseline was built from an isolated archive of that commit using the same measurement example. Linux 7.2.3-arch1-3, AMD Ryzen 9 5950X, 32 logical CPUs. No renderer or GPU capture was involved.

Each of seven shipped presets has three samples. Ordinary, oak, spruce, beech and birch use their shipped seed 42; Telperion uses 1 and Laurelin 2. These are mature direct `branching::generate` plus `mesh::assemble` builds; authored age is recorded but the direct builder does not consume age. This covers these seven preset/seed pairs, not all possible seeds. The separate high-budget regression uses beech seed 1 with three laterals per station.

The matched run alternated a baseline process and an after process for each preset, three successive builds inside each process. Sample 0 includes first-build allocator/cache effects; samples 1–2 are process-warm. Neither is an OS cold-cache claim. No core suite ran concurrently with the matched run. `/proc/loadavg` is captured for every sample; one-minute load declined from 5.84 to 3.27, briefly rose to 4.05 during after-birch, and ended at 3.62. Medians include all three samples, identically on both sides.

| Preset | Nodes | Leaves | Before median ms (range) | After median ms (range) |
| --- | ---: | ---: | ---: | ---: |
| Ordinary | 9,240 | 28,626 | 145.42 (144.70–148.07) | 147.14 (138.68–149.83) |
| Oregon white oak | 138,510 | 807,079 | 1,147.48 (1,144.84–1,160.15) | 1,131.23 (1,127.67–1,164.87) |
| Norway spruce | 91,331 | 7,102,504 | 5,789.32 (5,700.25–5,829.16) | 5,762.97 (5,703.56–5,808.12) |
| European beech | 179,736 | 4,657,111 | 4,070.66 (4,002.94–4,091.08) | 3,941.88 (3,935.43–3,964.43) |
| Silver birch | 73,337 | 224,905 | 3,661.93 (3,645.35–3,710.63) | 3,818.54 (3,745.06–3,834.83) |
| Telperion | 75,697 | 534,778 | 1,325.45 (1,306.19–1,348.60) | 1,339.01 (1,333.85–1,347.86) |
| Laurelin | 63,971 | 464,123 | 760.46 (744.38–773.68) | 737.55 (732.12–743.78) |

All 21 matched sample pairs have identical serialized-tree, wood-buffer and foliage-instance FNV fingerprints, node counts and leaf counts. Every preset is uncapped in these measurements. Therefore default output compatibility required no image repins or renderer captures. Median cost shifts range from approximately −3.2% to +4.3%; the birch rise coincided with increased external load. This small sample does not establish a repeatable cost regression attributable to cap removal, and no compensating cap or speculative optimization was introduced.

`matched-before.jsonl` and `matched-after.jsonl` retain raw generation/assembly times, counts, hashes and load. `before.jsonl` and `after-confounded.jsonl` retain the first comparison, whose baseline ran alongside the core suite at higher load; those timings are not used to claim improvement. Raw historical JSON predates the example's explicit seed field; seeds are stated above and derive from the unchanged preset constructors.

The explicit high-budget contract test grows beech seed 1 with `lateralsPerStation=3` and `maxNodes=1,000,000`: **254,252 nodes, uncapped**, with valid topology. A diagnostic run measured 177.305 ms direct generation while the full suite was active. This single timing is not a comparative performance claim; it records the formerly truncated candidate's successful execution. The regression first failed against the old effective ceiling, then passed with the caller budget honoured.
