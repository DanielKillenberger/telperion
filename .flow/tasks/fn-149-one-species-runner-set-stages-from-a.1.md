---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-149-one-species-runner-set-stages-from-a.1 Implement One species runner: set stages from a name to an accepted tree

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The species runner is built. `species <id>` runs eight stages, one module each behind one `Stage` trait. A stage reruns only when the content of one of its inputs changes, and a run stops only for an identity gap or the owner's look. The runner takes `--until`, `--stage`, `--status` (with a preflight of the paid services), `--record`, `--replay` and `--tools`.

The proof is the beech: recorded live from a bare seed on 2026-09-25, and replayed offline through Start inside the workspace gate by `tests/replay.rs`, with no network and no key. A second replay reruns nothing. The fixture is `crates/telperion-jev/tests/fixtures/replay/european-beech`: 7.6 MB in 247 files. The repository is public, so every recorded page that is not openly licensed keeps only the passages the run quoted to Jev (`tape_trim`, checked by `the_recording_keeps_only_the_passages_the_run_quoted`); the unpushed history was rewritten so the full pages never entered it. fn-157 replaces the literature stages, so tuning's first live revision and the beech's values move to fn-157's proof.

stage: impl-review - ran (codex; SHIP at 03a1584c, receipt /tmp/impl-review-receipt-c2ca1be780e9-fn-149-one-species-runner-set-stages-from-a.1.json; review rounds were reset after each owner re-plan)

Lines, crate `telperion-jev`: src went from 36,881 to 28,584 (runner 3,007, pipeline 12,989, tuning 8,532, binaries 443, tape 723). Tests went from 22,936 to 17,743. Before this task the crate also held the conductor (4,964 lines) and the separate conductor and pipeline binaries (764 lines of binaries in all).

Gate: `cargo test --profile ci --workspace --no-fail-fast` passed 1,019 of 1,019 on the rerun at 03a1584c, the replay and trim tests included; the first run failed four telperion-core memory-ceiling tests under a loaded host, which pass alone (friction entry). `npm test` passed 129 of 129, and `python3 scripts/test-reviewers.py` passed 12 of 12.

R8, the rewrite carried these of the 2026-09-24 stack specs:
- Carried: fn-113, fn-114, fn-116, fn-118, fn-119, fn-122, fn-127 to fn-133, fn-135, fn-136, fn-137, fn-139, fn-140, fn-141 and fn-142.
- Partly carried: fn-117. The no-progress stop is kept; the spending cap was removed by design.
- Removed by design: fn-121, with the conductor.

Friction: 15 entries are in `.flow/evidence/fn-149-one-species-runner-set-stages-from-a/FRICTION.md`.

The CC BY-SA Wikipedia lead, the DOAJ record and the Commons photographs stay whole, as their licences allow.
## Evidence
- Commits: fa0f1c98728ceac672950f12728f0bdd14dd8920, 50f7b873ad93d59d3903ea9e5b88fadf04ea058b, 1a5237ba206e659a3e757f47b7f60f42bd8593a3, 184b79a111eac0561ad6435dbba2bd223ea5ff93, 86dcf5495adeb5a2229003b85feff76386126610, 62c32303247a95c98b5f716d5f2229789434d0b5, 66792f97c8b8c0d799253929c52bed42e5325d00, ae931332a5bea665ea635f4661d7ed836e0b1022, cffb42e23687025bf61be663992ca71407d42fd4, c38b338e83dd7765d76bc686611979e072473400, caea58ddd43e8ce2557e2eb70026053e19478315, 55d988057ed868390f26962f9c99268d4d6e0b0d, f9e40644b3465a02b4e9ca9317792c03602e028b, 205dd71fbb0bd9d5d5e08cfb07ffa90988920972, 94d4cf26a430cd0286911956b69824cb7837077e, d8ba16e15663e84c4f916d36f9cce445ed869b12, d48c56cd947f99b265db9d20f23a5945442ce14d, 7f5f21d773e0a6be995ff2887bc9dc56feda7367, be78faeeb1f30e4fba3930b672d16b77c610df67, 74327e8ec460dd1f45428f1136cf6a283a7e96f3, 3f16dccd2ff6d44eccf0ccdc1895c5af99663b78, b4cc8628b39c3c426d5388fc3a2865a6089de9b5, e6a2df66c165dff4b89a9f7c7ce528d62220e1b4, d002a09e7471abfc508282f7aed3ac49309f2281, 5ed6944fe9b79657d9786ebeb748a26bf2203751, 8f277795a6b9335e02a2f5ce9912ca539f41c433, 86cdfb9e43073225f995e30c3466d70fe7ed6a4c, f01f04a577f5838b1f5b2df1b52f4139101df1f7, b43c2763f09cce13d2a001b5f335b4aef1c37a3b, adf7f78b731929ebbdd6b69ba03125e992eec08e, 1e1303094c23d910f72f38b6ddf069899675d0c6, 5faa20e229e1f8f8dd0cea712753ab3d2e7f2f2a, 6c1879fe333675245fe3b75e668e5fe87ddc5297, 9ce20c84820262780c3ea8c1b6a3ac0db13f5c13, 3f9d6a86ed1d700857c36b454af4c52575ca2b11, 8550bc92f7948c0d5b0d6f22fa2c2c173d048865, 1f9610faf334bff0d635c20271716485c0cfcdd0, d4599b9508d07094517e35195247031c14b648c3, ad4b31431d3d0e4f92217978ef4001687402e883, 91e54d385eda34d2b1e5523c949004053a038f52, 055a414e64541699862bfc250075e15ffda68742, f383e45ff11f1a3def1e628a0e5e277a4aa582cb, a0a8a92eb8f59626a440da7863d7acf14b0d5e98, 645ce3c44fdacb9ccb88afd81df6af04478940ec, 11ddd561ed07b2e5feda49996e95fd2601d8c85b, 03a1584c756e18e92d6bdcc6ba48152ebccd5d43
- Tests: cargo test --profile ci --workspace --no-fail-fast (1019 passed, 0 failed, at 03a1584c; first run INCONCLUSIVE: four telperion-core memory-ceiling tests failed under host load, pass alone), npm test (129 passed), python3 scripts/test-reviewers.py (12 passed), tape_trim <fixture>/tape --check (no page beyond its quotes), node scripts/catalogue-check.mjs (6 species pass)
- PRs: