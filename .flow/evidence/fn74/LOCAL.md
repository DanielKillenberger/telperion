# fn-74 local evidence, commit 805659b3, 2026-09-18

Taken on the desk (32 cores, Rust 1.98.1); the runner numbers for R5 and R6
are the conductor's, from the Actions runs after this branch is pushed, and
go in RUNS.md beside this file.

## R4: the ci profile moves no result

`cargo test --release --workspace` and `cargo test --profile ci --workspace`
on the same commit, each `cargo test` log reduced to one row per test binary
and doc-test run (name, status, passed, failed, ignored).

| Profile | Rows | Passed | Failed | Ignored | Wall | Compile from scratch | species.rs |
|---|---|---|---|---|---|---|---|
| release | 80 | 499 | 0 | 11 | 545 s | 2 m 10 s | 133.4 s |
| ci | 80 | 499 | 0 | 11 | 479 s | 28.7 s | 134.7 s |

The two tables are identical row for row; no suite stays on release.

## R6 proxy: warm compile

Dependencies built, `crates/telperion-core/src/lib.rs` touched,
`cargo test --profile <p> --workspace --no-run`: release 114 s, ci 13 s.
The runner has four cores, so its number is the one that decides whether the
integration tests are grouped into fewer binaries; the profile alone cut the
desk's warm compile by about nine times.

## R1 and R2, the key script

`node scripts/ci-key.mjs <suite>` gives the same key twice for each of the
five suites and a different key per suite. A line appended to
`crates/telperion-core/src/lib.rs` moves the rust-core, rust-jev and node
keys; a line appended to `README.md` moves none. A suite input missing from
HEAD exits 1 with the path named.

## Node suite on this commit

`npm test`: 6 files, 85 tests passed. `npm run typecheck`: clean.
