---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-153-releases-publish-cis-tested-artifact-in.1 Releases publish CI's tested artifact in minutes

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
tests.yml is now the only publish path. Every run builds the package once, runs test:dist on the tarball's own dist and uploads the tarball with its sha256. A master push whose package.json version is not on npm publishes that tarball once every suite is green for the commit, by receipt or by run. It refuses on a version below npm's latest, a tag that names another commit, a suite neither receipted nor green, or a missing or altered tarball, then tags v<version>. wasm-bindgen is the prebuilt release binary at the Cargo.lock pin. release.yml is deleted, and master runs queue instead of cancelling. R6 was added: publishing is OIDC trusted publishing with no npm token, which 0.1.0 to 0.1.4 already used. The owner must add tests.yml as a trusted publisher on npmjs.com before the next release. R1 and R5 are measured on the first version bump after merge.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 5f21f6119c5bca427f54c040c236c640718263ff, b641a23e216c8fac48339ec0af0c31aefd374988, 9fb9223304ed9113ef7045d7dca713b26e0667c6, 41321c3f11b12a475093eb0356ff9ad4e806c2e8
- Tests: uvx --from actionlint-py actionlint -shellcheck <shellcheck-py> (clean), python3 yaml.safe_load on tests.yml and wasm-bindgen action.yml, scratchpad sim.sh: publish decide + suite-gate steps on 8 stubbed version/tag cases and 4 suite cases, PR #118 CI run 36124529652 on b641a23e: all jobs green, package artifact uploaded, test:dist on the tarball green, baseline: none (CI workflows only; no Rust or package.json change), restructure (owner decision): actionlint+shellcheck clean on release.yml/tests.yml, scratchpad sim2.sh: release.yml decide 7 cases + suite 6 cases + tag against a bare remote then rerun decides tagged=true, live probes of the jobs and caches APIs, PR #118 CI run 36125486832 on 41321c3f: all jobs green, package artifact holds telperion-0.1.4.tgz and its .sha256 (sha256sum -c OK)
- PRs: https://github.com/DanielKillenberger/telperion/pull/118