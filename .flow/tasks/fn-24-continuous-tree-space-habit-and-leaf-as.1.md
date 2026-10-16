---
satisfies: [R1, R2]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.1 One scaffold builder from the habit trait table

## Description
Replace the three habit builders and the separate colonization pass with one scaffold builder driven by numeric habit traits, apply the bias field at every growth unit, seed each axis from its own stream, and prove R2 with a cross-trait hash test. Early proof point: oak row must read as an oak, spruce row as a spruce.

## Acceptance
- One builder under 400 lines; `habit.rs` and the benchmark wire copy deleted; no habit enum identifiers remain
- Traits validated by range, errors name the trait; a `kind` tag is rejected naming the field
- Every growth unit of every axis passes through the bias field; R2 hash test green

## Done summary
One process grows every axis from fifteen numeric habit traits; the four habit branches below the crossover read traits. Headings are one sum of rule heading, attractor pull and bias field; per-axis streams hash family seed, parent axis and child index. Rows deviated from the spec's starting table: oak lateral spacing 1.6 m and 3 orders (0.35 m hit the 250,000-node ceiling), spruce 0.15 m and 4 orders (91,331 nodes vs 113,132), Telperion/Laurelin their own row. Rust only; TypeScript restored in task 4.

## Evidence
- Commits: f3c30e2
- Tests: worker baseline: green - cargo test --release --workspace (125 passed, 7 ignored) before any edit, worker: cargo test --release --workspace - 129 passed, 0 failed, 7 ignored, worker: cargo fmt --all -- --check, ...
- PRs:
