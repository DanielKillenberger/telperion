---
satisfies: [R2, R4, R5]
---
# fn-19-comparative-botanical-geometry-benchmark.2 Add reproducible structural distribution measurements

## Description
Add a benchmark-only native measurement runner with analytically tested structural distributions, driven by the declared species inventory.

## Acceptance
- Analytic fixtures validate the frozen distributions and unavailable/ambiguous states.
- Malformed protocols, nonfinite input, capped generation and absent cases fail explicitly.

## Done summary
Native runner for frozen species/case manifests: operational axis distributions, taper, angles and biological-unit centroid bins, with immutable receipts that verify source/tool/binary identity before comparison. Analytic Rust tests plus six Python controls; six small 2 m fixtures reproduce across two runs and six capped cases stay failed and retained.

## Evidence
- Commits: 2554948ee3650d9924efe5ce9a0e33a55e5c6948, 18cf081ae6ebc608c9952aa3d874d3db9448b5ab, 880452cf7d3aa7d3054ffef510648ee3b254181c
- Tests: cargo test --release -p telperion-core --test geometry_benchmark --test species_metrics; npm run typecheck
