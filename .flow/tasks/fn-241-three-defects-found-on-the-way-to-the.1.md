---
satisfies: [R1, R2, R3, R4]
---
# fn-241-three-defects-found-on-the-way-to-the.1 Below ground, the field test's time, the cold-build margin

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
All three fn-241 defects are fixed as the host decided.

- **R1:** the fine path's box is built from each ring's own extent, on the CPU curve and in the device handoff. The new test holds it to `mesh::build`'s within fn-125's screen for every shipped species at 40 (by default) and at 80 (a device-limits probe).
- **R2:** the field test is one test a preset, all eight inside 60 s.
- **R3:** the cold-build test asserts each pipeline's bound and reports the first tree's total. The place threshold's log-odds now comes from the CPU and `asks` has one lead call site.
- **R4:** the gate, npm test and `space_hash` are green.

Evidence: `.flow/evidence/fn-241-three-defects-found-on-the-way-to-the/` (R1.md, R2.md, R3.md, GATE.md, FRICTION.md).

Follow-ups (impl-review P2s, not fixed):
- `CurveCluster::bounds` encloses the stored planar rings but not wood drawn between them by Hermite tessellation on a strongly bent stretch, nor the cylinder-wrapped vertices of shaped sections. The old sphere box held both.
- Shipped species measure within the screen. A general containment fix would bound the interpolated stretch envelope and shaped runs.

stage: impl-review - ran (codex, three draws, SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 8fbc9f8e2b172d43ad299cf3089900b3b316ba64, c6e6c94c0ee8f4914b8d2d96bc588ed6bdfe8670, aeece325e6f10444084eef88d6d23ea6d083967c, b864c20ebc54146705a43bca69a934dab49f8064, 5b97b3bbcaddcf8119efe55ea06458c317f3576b, 9ff2a939a628dcb0d8cde83b890764d5a6da59ff, 434521ccf5db2e256ec6c712972139918087129c, 7a9258ae89c7892964e83401f2b4e42520867623, 8ed889dae013905ab3bdd8676700e2400bfc1dc3, 44a8e583245bafb25f708294e958ec6b04e2e5ec, e449774cd5cecc2c4bbe53fea6412fc89c7d1d8b, 661e72e4cddb623f1248d028890b8678cf797f8e, e621b93cdbd178b596f61755a122f9c85bb1294a
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, npm test, cargo run --profile ci -p telperion-render --example space_hash
- PRs: