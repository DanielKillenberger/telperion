---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-57-typesafe-usage-guidelines.1 Implement TypeSafe usage guidelines

## Description
Write `docs/typesafe.md` and build `crates/telperion-jev`: one caller and ledger, question sets as data, screen, select, cite, triage and cases tools, and the isolation guard.

## Acceptance
Every R-ID in the parent spec's acceptance is satisfied; judged against the spec's criteria directly.

## Done summary
Agents read `docs/typesafe.md` and reach the model only through `crates/telperion-jev` (shared caller, interactive-shell key, ledger entry per call; question sets in `crates/telperion-jev/data`). Labelled sets score 12/12 kind, 5/5 selection, 9/9 citation compose with O1 listed for the owner, 8/8 routing, 4/4 severity (`.flow/evidence/fn57/cases-live.txt`). All eight round-1 review findings verified fixed with tests; the R6 guard no longer skips `render` directories. 39 jev tests pass with `TYPESAFE_API_KEY` unset; clippy, fmt, `npm test` (77) and typecheck clean; the workspace was green except an inherited `bark_distance` aliasing failure untouched by this range. Owner asked to rotate the API key (printed into a transcript by a shell fallback).

## Evidence
- Commits: 4447b4eca89c2061042d2687e15ed50794cb0b2e, ff442d749ec5c39cbcd9142eb0cff9f81a364916, 0927af42dbccf240a921dcae8728da9509081466, 46667a41f2bbee177fe232a8722e8a7af495a58e, bd92b6108a1b39a35e6c661af344d42909422812, e39488a340790cd79e1891c75e89da482f6134ab, 76169493aaddf59ed59eedd2f7ddf8508675cb20, ce6b64949e7ec89364f70e95bebe44ae2e72bbb0
- Tests: env -u TYPESAFE_API_KEY cargo test --release -p telperion-jev (39), cargo clippy -p telperion-jev --all-targets -- -D warnings, env -u TYPESAFE_API_KEY npm test (77), npm run typecheck
- PRs:
