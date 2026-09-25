---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-153-releases-publish-cis-tested-artifact-in.1 Releases publish CI's tested artifact in minutes

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
After the owner's decision, tests.yml tests and builds only. Its package job builds once, runs test:dist on the tarball's own dist, and uploads the tarball beside its .sha256; wasm-bindgen is the prebuilt release binary at the Cargo.lock pin. release.yml publishes on workflow_run after a successful Tests push run on master. It reads the run's head_sha and checks each suite by green job or by its receipt in master's cache. It downloads the run's package artifact and verifies its sha256, then applies the version and tag checks. The v<version> tag is pushed on head_sha before npm publish, so a rerun converges. R6 is added: publishing is OIDC trusted publishing with no npm token, which 0.1.0 to 0.1.4 already used; the workflow_ref names release.yml, so npm needs no change. R1, R2, R3, R5 and R6 are measured on the first version bump after merge.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 5f21f6119c5bca427f54c040c236c640718263ff, b641a23e216c8fac48339ec0af0c31aefd374988, 9fb9223304ed9113ef7045d7dca713b26e0667c6, 41321c3f11b12a475093eb0356ff9ad4e806c2e8
- Tests: uvx --from actionlint-py actionlint -shellcheck <shellcheck-py> (clean), python3 yaml.safe_load on tests.yml and wasm-bindgen action.yml, scratchpad sim.sh: publish decide + suite-gate steps on 8 stubbed version/tag cases and 4 suite cases, PR #118 CI run 36124529652 on b641a23e: all jobs green, package artifact uploaded, test:dist on the tarball green, baseline: none (CI workflows only; no Rust or package.json change), restructure (owner decision): actionlint+shellcheck clean on release.yml/tests.yml, scratchpad sim2.sh: release.yml decide 7 cases + suite 6 cases + tag against a bare remote then rerun decides tagged=true, live probes of the jobs and caches APIs, PR #118 CI run 36125486832 on 41321c3f: all jobs green, package artifact holds telperion-0.1.4.tgz and its .sha256 (sha256sum -c OK)
- PRs: https://github.com/DanielKillenberger/telperion/pull/118