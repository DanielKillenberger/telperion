# The tests run in GitHub Actions

## Goal
The repository had no CI; every test ran only on someone's machine, and fn-58's friction (2026-09-18) recorded a ten-minute workspace run blocking a chore commit. The owner asked for the tests to run in GitHub Actions: one workflow on push to master and on pull requests, calling the package scripts the README names, green on master before close.

## Decisions
- > user (2026-09-18): "a new spec for ci/cd"
- > user (2026-09-18): "so run tests in github actions"
- > user (2026-09-18): "you can work and merge autonomously once it works"
- One workflow that calls the package scripts (`npm run rust:test`, `npm test`, `npm run typecheck`) and duplicates none of them; toolchain read from the repository's pins; caches keyed on the lockfiles; no secrets, so the jev tests run on the mock transport.
- GPU suites stay in the Rust run and print `skipped:` on the runner rather than being excluded by name. Browser suites, fmt and clippy are out of scope; no deployment or publish.
- Rejected: an OS matrix, a nightly schedule, browser suites on a software adapter.

## Acceptance Criteria
- **R1:** A workflow on every push to master and every PR runs `npm run rust:test`; GPU suites skip — met.
- **R2:** The same workflow runs `npm test` (building wasm and render modules) and `npm run typecheck` — met.
- **R3:** The toolchain comes from the repository's pins, none restated — met.
- **R4:** Cargo and npm caches keyed on lockfiles; first and cached run lengths recorded — met (lengths below; caches restore but do not shorten the run).
- **R5:** No secret; `TYPESAFE_API_KEY` absent, jev tests pass on the mock — met.
- **R6:** Green on master after the merge — met.

## Result
`.github/workflows/tests.yml` (commit 8bd3e5f9). Runs observed 2026-09-18: PR #32 run 35349954061 green in 24 min 3 s (rust:test 19 min 2 s, npm test 2 min 58 s, wasm-bindgen install 77 s); master run 35354444073 on merge dc835949 green in 23 min 46 s. The caches restore (rust-cache 5 s, npm ci 6 s) but the run length does not move: the LTO release test build relinks every test binary, and the wasm-bindgen install repeats at about 80 s.

## Open
A `timeout-minutes` bound and a faster test profile (owner's call; later fn-74 and fn-76); the fmt and clippy gate once three files are formatted.

## Friction
- No SSH agent in the tool shell; the fence chain committed onto local master -> recovered by hand; fix is an https `insteadOf` rewrite through gh.
- Codex quota exhausted at implement -> session model implemented; fix is a quota probe before bridging.
- gh token lacked the `workflow` scope, so the branch could not be pushed -> NEEDS_HUMAN, owner granted the scope.
- A concurrent fn-58 session committed onto the checked-out branch -> branch rebuilt in its own worktree; fix is one worktree per session and branching from origin/master.
