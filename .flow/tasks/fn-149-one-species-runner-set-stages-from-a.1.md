---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-149-one-species-runner-set-stages-from-a.1 Implement One species runner: set stages from a name to an accepted tree

## Description
Replace the conductor, gap loop and pause machinery with one eight-stage species runner.

## Acceptance
Spec R1-R8.

## Done summary
`species <id>` runs eight stages behind one `Stage` trait, rerunning only on input-content change and stopping only for an identity gap or the owner's look. Proof: the beech recorded live on 2026-09-25 and replayed offline through Start in `tests/replay.rs` (no network, no key; a second replay reruns nothing; non-open pages trimmed to quoted passages). Crate src 36,881 -> 28,584 lines. R8: carried fn-113, 114, 116, 118, 119, 122, 127-133, 135-137, 139-142; fn-117 partly; fn-121 removed. Codex SHIP at `03a1584c`. Gate 1,019 passed; npm test 129.

## Evidence
- Commits: fa0f1c98728ceac672950f12728f0bdd14dd8920, 50f7b873ad93d59d3903ea9e5b88fadf04ea058b, 1a5237ba206e659a3e757f47b7f60f42bd8593a3, 184b79a111eac0561ad6435dbba2bd223ea5ff93, 86dcf5495adeb5a2229003b85feff76386126610, 62c32303247a95c98b5f716d5f2229789434d0b5, 66792f97c8b8c0d799253929c52bed42e5325d00, ae931332a5bea665ea635f4661d7ed836e0b1022, cffb42e23687025bf61be663992ca71407d42fd4, c38b338e83dd7765d76bc686611979e072473400, caea58ddd43e8ce2557e2eb70026053e19478315, 55d988057ed868390f26962f9c99268d4d6e0b0d, f9e40644b3465a02b4e9ca9317792c03602e028b, 205dd71fbb0bd9d5d5e08cfb07ffa90988920972, 94d4cf26a430cd0286911956b69824cb7837077e, d8ba16e15663e84c4f916d36f9cce445ed869b12, d48c56cd947f99b265db9d20f23a5945442ce14d, 7f5f21d773e0a6be995ff2887bc9dc56feda7367, be78faeeb1f30e4fba3930b672d16b77c610df67, 74327e8ec460dd1f45428f1136cf6a283a7e96f3, 3f16dccd2ff6d44eccf0ccdc1895c5af99663b78, b4cc8628b39c3c426d5388fc3a2865a6089de9b5, e6a2df66c165dff4b89a9f7c7ce528d62220e1b4, d002a09e7471abfc508282f7aed3ac49309f2281, 5ed6944fe9b79657d9786ebeb748a26bf2203751, 8f277795a6b9335e02a2f5ce9912ca539f41c433, 86cdfb9e43073225f995e30c3466d70fe7ed6a4c, f01f04a577f5838b1f5b2df1b52f4139101df1f7, b43c2763f09cce13d2a001b5f335b4aef1c37a3b, adf7f78b731929ebbdd6b69ba03125e992eec08e, 1e1303094c23d910f72f38b6ddf069899675d0c6, 5faa20e229e1f8f8dd0cea712753ab3d2e7f2f2a, 6c1879fe333675245fe3b75e668e5fe87ddc5297, 9ce20c84820262780c3ea8c1b6a3ac0db13f5c13, 3f9d6a86ed1d700857c36b454af4c52575ca2b11, 8550bc92f7948c0d5b0d6f22fa2c2c173d048865, 1f9610faf334bff0d635c20271716485c0cfcdd0, d4599b9508d07094517e35195247031c14b648c3, ad4b31431d3d0e4f92217978ef4001687402e883, 91e54d385eda34d2b1e5523c949004053a038f52, 055a414e64541699862bfc250075e15ffda68742, f383e45ff11f1a3def1e628a0e5e277a4aa582cb, a0a8a92eb8f59626a440da7863d7acf14b0d5e98, 645ce3c44fdacb9ccb88afd81df6af04478940ec, 11ddd561ed07b2e5feda49996e95fd2601d8c85b, 03a1584c756e18e92d6bdcc6ba48152ebccd5d43
- Tests: cargo test --profile ci --workspace --no-fail-fast (1019 passed at 03a1584c); npm test (129); python3 scripts/test-reviewers.py (12); tape_trim --check; node scripts/catalogue-check.mjs
- PRs:
