---
satisfies: [R1, R3]
---
# fn-256-the-whole-grower-runs-on-the-gpu.3 Production builds through the one cycle; the old growers go

## Description
Route the harness build and the CPU build through fn-255's cycle on its chosen executor (device in the harness, CPU as the reference), then carry out every deletion fn-255 R3B.md lists as 'deleted with fn-256' (reads_light and its branches, apart, alone, Certain/lasting, the stepped executor, the fine-root cut and marks, chain/contract/wide.wgsl, the f64 survival path, the f32 Field::on deposit); the render gate's resident-against-CPU checks move to the cycle's agreement checks. A GPU capacity failure is a named error, never a CPU rebuild (R1).

## Acceptance
No species-dependent executor selection compiles; each deletion named with the consumer it checked; gate and npm test green.

## Done summary
The resident build grows and draws the whole tree on the device cycle: the cycle's settled node table is dressed into drawn records by chain waves (`cycle/chains.wgsl`, `cycle/dress.wgsl`) and read directly by the folds, sweep, stations, rings and handoff, with no Part, stubs or attachment seam. The CPU route keeps the reference role (the CPU cycle plus `prepare_whole`). The device build is async end to end, and a device or capacity failure is a named error, never a CPU rebuild. Every deletion R3B.md lists is done or kept for a named site-path reader (DESIGN.md 12.5). The device's stations stage also places a rosette's fronds (DESIGN 12.7.1).

- **Agreement** (`resident/agree_tests.rs`, every preset, age 12):
  - nodes equal in lineage, positions within 3e-5 m;
  - stations equal;
  - every wood run matched, the trunk included;
  - the curve's section rows equal the CPU's;
  - the palm's 6,554 leaflets within 3.2e-5 of the height of the CPU's, withering identical, mean size 0.2% apart.
- **Stills:** every preset whole, mid and close beside F4's, viewed (DESIGN 12.5a, 12.7.1). The beech and Telperion are identical to the eye; the others keep their forms. The palm's crown is F4's, with a bare trunk until the leaf-base spec lands.
- **Fixed red to green:** two device defects, the trunk's buried foot (found by the stills) and the curve's rows (found by the codex review).
- **Build time,** warm ms (DESIGN 12.5b), F4 then now:
  - ordinary 523 then 518
  - oak 1159 then 515
  - spruce 403 then 657
  - beech 210 then 287
  - birch 716 then 739
  - Telperion 671 then 5929
  - Laurelin 627 then 5834
  The Two Trees' cost is the device grow's restarts, which task 4 owns.
- **Host decision (ii), 2026-10-12:** the retained leaf bases and their cells are their own spec, which fn-256 depends on. Until it lands, the resident palm is refused by that capability (retained leaf bases), never by species.
- **Follow-up:** `prepare_whole`'s resident branches have no production caller and are kept as the route tests' reference.

Tier: as dispatched by the conductor.

stage: impl-review - ran (codex; round 1 NEEDS_WORK, 3 fixed; round 2 NEEDS_WORK on the palm; after the host's decision and the rosette, SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 50b96d82056f94c7808feea86eaac0ebca7aa634, 29218095e87cdc62b4d4ed1681f973d0fa728d79, be6c8d39e59a5e40fbc4237df687cfafe81a6ce4, 187a89777bc1205f6dd486bc2e7b5a87753dc19a, d63377accaea3a576ba18a6924fec9883ba21177, 3d2cad00856cf254a1b12d77912f5c2e833e8c63, 4f261f6f1e96a29cd7869e0e3b2558fdc3d623b1, 7c8ddb9020d84da4adfde2ae9d238eba41461cf4, 392f618bce1e21904125a421b2e7a76d8a45ba57, 7310d35b8959c77968926b8d96bb06ffdf228dcc, e8d4984fd79a9ae4c5edce73f4655aa045ee138b, 2ee4839685909621d808f2a0499941646349804c, ca385d8334ae7fd6c831f360fd6f35fef0c50c65, 13b5101967591344fa12296f29bf40a0fdced53d, 86e5b11036730c212ab043dd55f82bd0b6463765, 1a0da25d1a4db7394962476120b6ee4c63d637c6, 6547b99432379ceec8992d2d193baf2ffaf59038, ff221ad737e2b21c71c54cc773674cabeec388f2
- Tests: cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast (1061 passed, 0 failed, 48 ignored), cargo test --profile ci -p telperion-render --no-fail-fast under the GPU lock (250 passed, 0 failed, 30 ignored), npm test (15 files, 162 tests passed)
- PRs: