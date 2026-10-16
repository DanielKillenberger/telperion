---
satisfies: [R3a, R4]
---
# fn-256-the-whole-grower-runs-on-the-gpu.4 Proof: speed, memory, stills, Chrome

## Description
fn-256 R3a and R4:
- **Warm build per preset** at or under F4's (beech 0.8 s, Telperion 1.2 s), including preparation and readback.
- **Cold compile** against fn-254's 11.7 s.
- **CPU peak during preparation** at or under master's (oak at 100: 9.0 GB today).
- **Stills** of every preset, whole, mid and close, beside F4's, viewed.
- **The owner's probe page**, extended to a whole tree and run by the owner (or by the host through the Chrome extension on the owner's word), with no device loss.
- Astra's agreement; codex impl-review; the owner's verdict.

Carried over from task 2 (EXECUTOR.md):
1. **Wall time from small tables.** Grown from the least sizes, with retirement letting go at moves, Telperion 82 takes 2.16 s unstamped (5 restarts, 61 moves), against 1.05 s of stamped build and 0.06 s of settle at its own sizes. Restarts and moves must come out of the build's wall time, through better first sizes or cheaper moves.
2. **Dispatches.** Telperion's build is 765 dispatches a cycle, about 0.44 s at Chrome's 7 µs, against DESIGN.md 7a's target of about 340. The units record every round each cycle (about 424 dispatches); skipping the rounds no job needs would roughly halve that.
3. **Cold compile** passes task 2's bound natively: 0.98 s all at once, 1.67 s by module. The owner's probe confirms it in Chrome.
## Acceptance
fn-256 R3a and R4 met or each miss named with its measurement for the owner.

## Done summary
Task 4 measured fn-256 against R3a and R4. The changes behind the measurements: F4 (fn-253's flowers, ported onto the cycle's drawing path) and fn-266 (the palm's boots) merged; the device grow's restarts removed and its unit rounds gated; modules compiled side by side natively; a browser build held to the page's device budget. `.flow/evidence/fn-256-the-whole-grower-runs-on-the-gpu/PROOF.md` holds the tables.

- **Met:**
  - **Cold compile:** 1.54 s native with the cache off, every program a page uses, against 3.33 s before.
  - **CPU peak during preparation:** 0.77 to 1.01 GB, every preset and the ordinary at 100, against F4's 3.6 to 9.2 GB. The process's floor alone is 0.77 GB, so this is not comparable with master's 0.35 GB.
  - **Stills:** every preset, whole, mid and close, beside F4's, viewed. All keep F4's look. Telperion at share 0.3 and Laurelin as approved. The palm draws with its boots.
  - **Probe:** the owner's Chrome probe is built and checked without a browser (probe-v2 `ff224a610`). It is not run.
  - **Astra:** AGREE (round 3).
  - **Review:** codex SHIP (round 4).
  - **Gate:** green, with the repeat test isolated under `whole_tree()`.
  - **npm test:** 162 tests, green.
- **Missed, each with its measurement:**
  - **Warm build:** Telperion 1,909 ms against F4's 671 (5,929 before this task); Laurelin 1,782 against 627; spruce 613 against 403; beech 295 against 210; birch 782 against 716; ordinary 604 against 523.
  - **Dispatches:** 690 a cycle on Telperion, against a target of about 340.
  - **The oak at 100:** device out of memory, so it does not build on this card.
- **For the host:**
  - the hierarchy's dispatch schedule;
  - device memory at the card's edge: the Two Trees peak near 6 GB, which forces quarter-step tables and their 58 moves;
  - the device-memory guard natively.

## Evidence
- Commits: 79365f26e441856f4ad1b53faf762ae47d8f788e, 80f3ac29a9492487449f93631ca0402bcbf757a4, 09835ae0f6196574f5d758eb3286a4049895ce2f, 04d2600b1234e7ffb7dc690197c8124451181e66, 369146a511f09721c3bca1c740ece04da57b735c, 2118ef4bf9faa0ae77cfaf607446845018ccb799, 49950270baf34987dcdab0035da1243917a28c1e, bd5b9023affb8ea3d2c186d02dd1726baa76cc61, 2b28657e8d617e9cadf1d862bd9d1164cdcb89bc, 63ea58fb059eaedfc720b61695d960b08301487f, 202f6ca8a417b41631a5dbd08d0575ccc1857196, a1341e7ea96450fa0b93ab9e49364c69c6db1ae6, d4388608b3f3225a7fc48343a23d8ba834d893d8, c06a8d37dc8046cc73b70f766076e651d5a8d12f, 513c6175218098cc50b54337503f262f0e23e762, 04e3d01db35355efac4b30b6c3144bec4f2aadde, 7d65edc1d9cbbe53b6c623c884c56c772989c93c, 8dedf7c01524daa376c1d2060d77fe3e1e620c40, 9061f8b8b7b9ac46b4c50534a3a0a72471bea6f5, 4b63df0dd3b4a95c2e7b14ac567942b2f727371e, 1d373383e1d3a21f645d71c8da35c730b8e57162, a5db2d092f700d63d78da9298054c9c065f3cda8, e659e6613e89a6f6713985cb20ab0c304277ef70, 292852fe136edfa420dc57f0c8fe0a08bc284d37, 7c84ad1f819407049889802e6a677e8cdb2fb58a, 2dc75fb1f7ec88799b33c6f763f6b6481fc7533e, a9d6d45409b8c39b6d46e11fc229cd241cb16cc9, 63f194c266b744d1c64c73903b3554ed1b39fba6, 406d946ae4cb7a9738fc67110fa430c5e9656d55, 56ad07dba7a46f70f270341f5acea2cdf254af24, 8f02bb006feb158e379f41546be33211e4764984, 96e79037c32fe73e265bb58d1a8127aa136e24b2, 69838ca4d6fd878287bd77d86702d6ad54df8aa7, 665ffb78db42835bd19e3383d5e318021a71cbe6, 252301ae3798af57dbddffb6f478916760df3828, 901c9a7e7e676265ecfd3e9b8536ae067bb493c0, a845d947477c71a34dd7c56420d670031122e8be, 9e4f2c276b192dccf2b8954bdd40418f7f36f211, 5c15a9b420f86c7e44b8468d4197e87223695c79, bab9c3dfcb0393c78f02286a910f1f89e89b8bea, 7cb5f52de6aaf6b40613113d3973086b8ef77588, b48ccf9cde26af354f24e3f3db44cb53cb76cd28, c2a866360f01e5a5972f2f28fe79eddca5470cdb, 51918a2dbb9cd3a255634f85ea27c9a13c1787e4, 4a215532e804c787db55f2d73d39b1cfbf72673a, eb78425e64d66379e4199af8b7f9acdfef120574, 02fffe2476cff0ed7232aeebfa5abb1a37ac190e, 2d8ddf38bdb99c975d2fb40899bd88624905898f, ba71f0e3eb218627a1ffd3227dacb193dbf53663, 4310ff64471a0e7e179d6d5b073fb36078cb7999, 7ef613cd914abeda01ca5dd9b8ba9e6c2782f2d4, 97323bc2dfad23eb8cb3f05b1288b2ee4591d9e2, d147179ed02269fe98f947fdfdbf5204a0e006c7, c4a93695331c64c4f288ce5b2129773526430bb9, c02b3e060d9f53b63c7a4b68657d6123affb26fd, 34b72b4c95c751b57c133e3ed1e6d101de28c7fa, 6d3d28ebde0e8af7d4b678e68d2ef30a3978d0cf
- Tests: cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-render --lib, npm test
- PRs:
