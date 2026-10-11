---
satisfies: [R1, R2, R3, R3a, R3b, R4]
---
# fn-255-one-growth-law-reads-light-for-every.1 One growth law reads light for every species

## Description
Implement fn-255 R1-R4 as written. Order: R2's allocation held, R1 cycle graph on the CPU, R3 candidate measured against the lattice (stop for owner approval with the numbers), R3a, R3b, R4.

## Acceptance
The spec's R1-R4, R3a and R3b; R3 passes only on the owner's approval.

## Done summary
The one cycle grows every species on the CPU by one law (lattice light, integer deposit, the spike's reduction order and f32 viability), retires shed subtrees exactly, and its foliage-volume candidate was measured and refused; production routing and the old executors' deletion move with fn-256 (R3B.md).

- R1/R2: `grow_cycle` with every phase run for every species, allocation unchanged; working set with frozen upkeep, retirement exact against the every-axis-open reference (oak, spruce, beech; woken buds that stop at once, 4 seeds).
- R3: candidate measured (R3.md) and refused by the council; the lattice stays. R3a dropped by the council.
- R3b: the cycle's f32 deposit option deleted; routing `grow` through the cycle fails the render gate's resident-against-CPU checks, so it and the reads_light consumers are deleted with fn-256 (R3B.md).
- R4: every preset's stills viewed (CYCLE-COST.md), cost table and peak memory, zero crossings held; combined acceptance with fn-256.
- Finding for the host (specced as fn-265): shed wood keeps charging its bearer's upkeep; the oak at 80 keeps 10.5% more if it stops.

Tier: not given by the dispatch.

stage: impl-review - ran (codex, 3 reviewers NEEDS_WORK on 2 findings, both fixed; re-review SHIP)

Gates on the final commit (45750b509, rerun on 190a79ca7): the workspace without the renderer 1,070 passed, 0 failed; the renderer under the GPU lock 242 passed, 0 failed; npm test 155 passed. An earlier rerun stopped by the system for low memory before any test ran is superseded.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 92d3eec68227791390f0b9cc69689182a4b77cd0, 537c6771926dbb38a42d15072eb52cb95a686198, 3b51a316e15e2b186e2dec1f2999159c1e94f788, 8d9d9bcc4d518635ab93e289e8573115506403fb, d859281e9bc6177deba88c22822d4a832c2f7765, ee5989cbdd66eda8b891383982e68cab856114e6, e5eee562c050ff19345e548e0c1dd04dc4de0399, 972da0dcd87b4ac6715bac278529c658b85298e4, ceb54a1a3bff8c960273b9f289f79f939e38cf8b, 0cc9fa96e43bfb8b40f6e8b0c8e368cd9e66a4fe, ba86e1da8cc4e7899c332b453bb57f894831af6a, bc541bd1cb57869116fcb81bf5ac2aceb79c3ce1, d24a430eca167a6182cbdbbe6815f2a7a8352db1, 0fc870be86612e37e92a16c23f7ce3abb221f428, 45304868601a89e07cfbb784efb825d023f4c258, f21a69f2415fc743d165b65dc7fd1ac2ba91bd33, cd947430b34ff334eb9482d614af3f50076e57f6, 5e8481113f0f6e86e90cef4e7aecbfab9e804bfb, ecc04927cf94db9c54c526afb75fa7885ca293ad, ae7ba4fee6a7881c4bbfffe1b4919ec3e4b721d2, 07c35ece3a9f3b9c6f1c749b8200d6f464692e8d, 667859cf299131f3a8ca152b1f908adb1a6163d9, 821bab3042bf64beb4e6d41101161f2b143a95f2, 9f548d09d15ec0a7058f7cd4445f023eb2a8cb70, f16a84d9bf14748c05ce8eb6d716c5ef94604a24, ecb3b5da01338697894c204ef946e873428900e4, fe164ad6b4f83d1708c337ee4badb0e363d20a31, 11813c3313ff396ec12ddc31ed8aa8efee6bee7d, a1b1ed103578fc6bb355662c83324b87fbe0e499, becf0a6e516692aa507c57fef15e75a85eb99ba0, 45750b5092e4e2bf0b1f652c2d8c31ec9f9ace38
- Tests: cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast (green on the final commit: 1,070 passed), cargo test --profile ci -p telperion-render --no-fail-fast (green on the final commit, GPU lock: 242 passed), npm test (green on the final commit: 155 passed)
- PRs: