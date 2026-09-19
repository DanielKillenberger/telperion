---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-84-the-generator-declares-what-it-can.1 Implement The generator declares what it can express

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The capability gate now asks the generator what it can express, not a preset what its own value table produces.

`crates/telperion-core/src/capability.rs` declares the vocabulary: `EXPRESSED`, the eight names the generator draws, and `UNEXPRESSED`, the seven an assessment may state before the generator can meet them, each name carrying the one line that says what it means. `version()` is an FNV-1a digest over the declared names and the list each sits in, so it changes when a name is added and when a name moves between the lists, and not when a meaning is reworded. `geometry_benchmark --vocabulary` prints the whole thing with its version, which is how an assessment round reads it and records the version it assessed against.

`gate.rs` computes `missing` from the species' required list against that vocabulary. A required name the vocabulary does not carry is reported as unrecognised, which is its own report and not missing. The preset-derived list keeps its own question — whether a registered preset's table produces what the species requires of it — and is filed as `preset-capability`, never as `missing`; an unregistered preset is not asked at all. The gate fails closed: no recorded requirement is an unresolved gate, and a probe that errors files the error.

One defect the fail-closed rule exposed: the generate stage writes `packet/species.json` with `required_capabilities` empty for every species, and the gate preferred that file over the manifest's engineering row. A rerun after generate would have read the placeholder as the answer and blocked a species that had already passed. The gate now reads the manifest's row when the packet names nothing.

On the date palm's recorded required list the gate reports `woody-axes` met and the five organs missing — the false positive that would have sent the gap loop to write candidate fixes for a capability the generator already has.

No generator, renderer, preset or identity-pin behaviour changed.
## Evidence
- Commits: d42449478e604524f28501bdc3461d48be09c864, 6483e80b3e0ec6f9e13edd61f05e1d71d67b6ff9
- Tests: cargo fmt --all --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release --workspace --no-fail-fast -- --test-threads=1, node scripts/catalogue-check.mjs, cargo run --release -p telperion-core --example geometry_benchmark -- --vocabulary
- PRs: https://github.com/DanielKillenberger/telperion/pull/43