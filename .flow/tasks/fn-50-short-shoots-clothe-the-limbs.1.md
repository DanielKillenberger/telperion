---
satisfies: [R1, R2, R3, R4]
---
# fn-50-short-shoots-clothe-the-limbs.1 Implement Short shoots clothe the limbs

## Description
Leaves grew only on twig wood at the limb ends, so the beech's inside was
bare and each twig read as a fern's pinna; round 11's leaf-on beech read as
a conifer. This task adds short shoots as placements on existing wood, not
skeleton nodes (`foliage/short_shoots.rs`): five canopy rows for spacing,
the wood radius band, length, leaves per cluster and the cluster's spread.
The beech states a spur every 2.5 cm on wood under 0.35 of the trunk's
radius, 5 cm long, five leaves fanned level; and, under the host's direction
on the conifer read, its twig rows (angle 50, golden-angle divergence, a
leaf every 5 cm) and leaf lean (0.1). Habit, envelope and radius rows are
round 11's.

## Acceptance
- [x] **R1** five rows railed and refused by name, on the wire, blended,
      on the harness, in the regenerated metadata; every existing pin held.
- [x] **R2** clusters on eligible wood above the crown base; the node count
      is identical with short shoots on and off, asserted.
- [x] **R3** round 12 rendered and recorded in REPORT.md and
      `.flow/evidence/fn34/rounds.tsv` (the `round12-fn50` rows);
      48/48 protocol, beech seeds 151,319 to 200,476
      nodes, 4.25 to 5.60 million leaves inside the fidelity band.
- [x] **R4** the short-shoot tests, split under the 400-line rule.

## NEEDS_HUMAN — the owner's verdict on the combined round

Commits 57b2401c, 79f8df8a, 0b4c3846, db38df56. Gates green. The host read
B-WHOLE: the fronds and spiky sprays are gone, and the crown is one
rounded, dense broadleaf mass leafed through its inside. It is very dark on
this branch because canopy lighting (fn-52) is not in it. Still not the
photograph: a uniform mass without billowy lobes, and leaves no lower than
about 3.5 m at the sides against the photograph's 2 m, which is the habit
rows' (the lowest limbs carry their side wood high). Specimen reads and the
wasm `specimen_read` carry no short shoots, by the spec's boundary on
seasonal development; only the growth view's mesh shows them.

## Done summary
Short spur shoots with leaf clusters clothe the limbs by rows. The capability is merged; the beech's own verdict is fn-62's. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the European beech's verdict moved to fn-62.
## Evidence
- Commits: d0bf6f21
- Tests: cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release -p telperion-core --no-fail-fast, cargo test --release -p telperion-render --no-fail-fast, npm run typecheck, npm run rust:test:wasm, npm test, uv run scripts/compare-references.py --self-test, node tests/species.mjs --measure-only (48 cases, measure/protocol-round25)
- PRs: