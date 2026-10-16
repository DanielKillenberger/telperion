---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-153-releases-publish-cis-tested-artifact-in.1 Releases publish CI's tested artifact in minutes

## Description
Publish CI's tested artifact on a version bump without re-running suites or rebuilding.

## Acceptance
Spec R1-R6.

## Done summary
`tests.yml` builds once, runs `test:dist` on the tarball and uploads it with its `.sha256`; wasm-bindgen is prebuilt at the lock pin. `release.yml` publishes on `workflow_run` after a green Tests push on master: reads `head_sha`, checks suites by green job or cached receipt, verifies the artifact sha256, applies version and tag checks, pushes the tag before `npm publish`. OIDC trusted publishing, no token. R1, R2, R3, R5 to be measured on the first version bump after merge.

## Evidence
- Commits: 5f21f6119c5bca427f54c040c236c640718263ff, b641a23e216c8fac48339ec0af0c31aefd374988, 9fb9223304ed9113ef7045d7dca713b26e0667c6, 41321c3f11b12a475093eb0356ff9ad4e806c2e8
- Tests: actionlint + shellcheck (clean); simulated decide/suite/tag cases; PR #118 CI run 36125486832 (all green, artifact sha256 OK)
- PRs: https://github.com/DanielKillenberger/telperion/pull/118
