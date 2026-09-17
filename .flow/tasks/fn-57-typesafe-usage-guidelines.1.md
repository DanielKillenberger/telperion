---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-57-typesafe-usage-guidelines.1 Implement TypeSafe usage guidelines

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
An agent onboarding to TypeSafe now reads `docs/typesafe.md` for where Jev may run and where it never runs, and reaches the model only through `crates/telperion-jev`: one shared caller that owns the endpoint, the interactive-shell key and a ledger entry per call, four question sets kept as data in `crates/telperion-jev/data`, and the screen, select, cite, triage and cases tools built on them. The labelled pilot sets score 12/12 on sentence kind, 5/5 on span selection, 9/9 on the citation compose with the O1 site-criterion misuse listed for the owner, and 8/8 on routing with 4/4 severity, recorded in `.flow/evidence/fn57/cases-live.txt` beside the sixteen-site sweep in `INVENTORY.md`.

This session resumed the task after the prior one stopped at a usage limit. It verified all eight round-1 review findings against the code at HEAD rather than taking the fix commit on trust: the cite report now carries the screen kind, kind confidence, anchor probability and both ledger references; ledger ids are nanosecond plus pid plus counter and a write refuses to overwrite; `cite --research` reads only `## Resolved via Research`; `--standard` is required before the key is loaded; a malformed response body writes a failure entry; the severity Score sits behind an `assessable` Noul that leaves an unclear note `unassessed`; every slice clamps to a char boundary; and the state fields are bounded with no `whole` field. Each has a named test.

Reading for those findings surfaced one more gap in R6, fixed here: the isolation walk skipped every directory named `render`, so `src/browser/render` was never scanned and a hand-written module there could name the endpoint unseen. The guard now skips only `target` and `node_modules`, and a planted marker under a `render` subdirectory is a regression case that fails red without the fix.

Gates: 39 jev tests pass, including with `TYPESAFE_API_KEY` unset, which is R6's condition; clippy with `-D warnings` and `cargo fmt --check` are clean; `npm test` passes 77 tests and `npm run typecheck` is clean. `cargo test --release --workspace` is green except `telperion-render` `bark_distance` `resolved_scales_survive_until_the_two_pixel_boundary`, an inherited aliasing failure this task did not cause and did not fix: the range touches no file under `crates/telperion-core`, `crates/telperion-render`, `crates/telperion-wasm` or `src/`. No gate receipt was written, because a green receipt for the workspace suite would be false while that test is red.

Two items for the owner. First, rotate the TypeSafe API key: a shell fallback I wrote while checking whether the key was set expanded to its value and printed it into this session's transcript, so treat it as exposed. Second, the range `65d3f531..HEAD` carries three flow chore commits for fn-31 and fn-64 that landed on the branch between this task's commits and belong to other work.

stage: impl-review - ran round 1 [cursor gpt-5.6-sol-high, NEEDS_WORK, 8 findings] then skipped(config: review.backend none; host checks the diff against the round-1 findings)
## Evidence
- Commits: 4447b4eca89c2061042d2687e15ed50794cb0b2e, ff442d749ec5c39cbcd9142eb0cff9f81a364916, 0927af42dbccf240a921dcae8728da9509081466, 46667a41f2bbee177fe232a8722e8a7af495a58e, bd92b6108a1b39a35e6c661af344d42909422812, e39488a340790cd79e1891c75e89da482f6134ab, 76169493aaddf59ed59eedd2f7ddf8508675cb20, ce6b64949e7ec89364f70e95bebe44ae2e72bbb0
- Tests: cargo test -p telperion-jev (39 passed, 0 failed), env -u TYPESAFE_API_KEY cargo test --release -p telperion-jev (39 passed, 0 failed - R6 key-unset condition), cargo clippy -p telperion-jev --all-targets -- -D warnings (clean), cargo fmt --all -- --check (clean), cargo test --release --workspace (all suites green except the inherited telperion-render bark_distance aliasing failure; the task range touches no core/render/wasm/browser path), env -u TYPESAFE_API_KEY npm test (6 files, 77 tests passed), npm run typecheck (clean)
- PRs: