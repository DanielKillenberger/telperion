# fn-314 R4: every tests.yml job run locally

Run 2026-10-24, branch `fn-314-the-release-branchs-ci-is-green-and-the` at `6d60599d6` (tasks .1 and .2 merged in), in the f4-314 worktree, one heavy command at a time. Each command ran inside `telperion-heavy 5 --threads 2` except where a row says otherwise. Full output for each row is in `raw/` (gitignored), under the log name given. Wall times are `date +%s` differences around the whole command, including the wait for room.

The Rust jobs ran the nextest action's command (`.github/actions/nextest/action.yml`), not the gate. `NEXTEST_TEST_THREADS` was set because nextest ignores the `RUST_TEST_THREADS` that `telperion-heavy` exports. It controls how many tests run at once, never which tests run. The core ran once, unpartitioned, which covers the union of CI's four `count:N/4` partitions.

| Job | Local command | Result | Wall | Log |
| --- | --- | --- | --- | --- |
| fmt | `cargo fmt --all --check` | green | 6 s | `fmt.log` |
| receipts | `node scripts/ci-key.mjs <suite>` for rust-core, rust-render, rust-wasm, rust-jev, node | green, five keys printed. The cache lookups (`actions/cache/restore`, lookup-only) are CI-only and were not run. | 2 s | `receipts.log` |
| core 1/4 to 4/4 | `cargo nextest run --cargo-profile ci -p telperion-core --no-fail-fast`, unpartitioned (the union of the four partitions), `NEXTEST_TEST_THREADS=1` | green, 532 passed, 59 skipped | 974 s | `core.log` |
| core without geometry | `cargo build --profile ci -p telperion-core --no-default-features`, then `cargo test --profile ci -p telperion-core --no-default-features --lib` | green, 142 passed, 4 ignored | 139 s | `core-plan.log` |
| render | `cargo nextest run --cargo-profile ci -p telperion-render --no-fail-fast` | green across two runs: 280 passed with two slots until the OOM killer stopped the run at the 5 GB cap. The 79 tests it had not passed were rerun with the same command plus an `-E` filterset of exactly those tests, one slot, and all 79 passed. 359 of 359 tests passed in total. | 1,422 s + 216 s | `render-2slots-oom.log`, `render-rest.log` |
| wasm, field and jev | `cargo nextest run --cargo-profile ci -p telperion-wasm -p telperion-space -p telperion-jev --no-fail-fast`, one slot | green across two runs: 594 passed until the OOM killer took `walks every_setting_changes_the_tree_by_degree...` at the 5 GB cap (5,396 MiB measured on `.config/gate-budgets.toml`). The 15 tests not yet passed were rerun with an `-E` filterset inside `telperion-heavy 8 --threads 2`, because that test alone cannot fit in 5 GB, and all 15 passed. 609 of 609 tests passed in total. | 456 s + 462 s | `wasm-jev-oom.log`, `wasm-jev-rest.log` |
| rust-receipts | none | Not runnable locally. The job only writes cache receipts (`actions/cache/save`) under CI's `RUNNER_TEMP` from the other jobs' results, and it has no local equivalent. Its inputs are the rows above, all of them green. | — | — |
| node | `npm ci`, `npm test`, `npm run typecheck`, each run alone | green. `npm test`: catalogue check, then vitest with 22 files and 177 tests passed. `npm run typecheck` passed. | 3 s, 233 s, 3 s | `node-npm-ci.log`, `node-npm-test.log`, `node-typecheck.log` |
| bindings | `npm run test:bindings`. Playwright's chromium-1243 was already installed, so `npx playwright install` was not needed. | green, all ten binding checks true | 48 s | `bindings.log` |
| package | `npm run build`, `node scripts/artifact-budgets.mjs`, `npm pack --json` with `sha256sum`, `rm -rf dist`, untar the tarball's `package/dist`, `npm run test:dist`, then remove the tarball, its `.sha256` and the unpacked `dist` | green. Every ceiling held, with `dist/module.js` at 64,631 of 70,000 bytes. The growth check is skipped outside a pull request, as in CI on a push. `test:dist` built all 24 cases from the tarball's dist. The tarball's sha256 was `aa62771b…c788e312`. | 258 s | `package.log` |

## The gate

`telperion-heavy 8 --threads 4 -- python3 scripts/gate.py` (base `master`) ran once, last of the heavy runs, and was green (`gate.log`, 1,455 s wall). 1,500 tests passed and 136 were skipped. The gate reported these per-check times:

- build and listing: 141 s
- package: 2 s (before the test run)
- the test run: 1,200 s against its 1,560 s budget
- core without geometry: 111 s (after the test run)

## Notes

- **Memory caps.** Four runs were ended by the OOM killer: two core attempts, one render run and one wasm, field and jev run. FRICTION.md explains why a 5 GB cap cannot hold them, in its entries from 14:26, 14:29, 15:14 and 18:57.
- **Working tree.** After the run the worktree holds no tarball and no unpacked dist. The gate's own package check rebuilt `dist/` with `npm run build`, which `.gitignore` covers, as before this task.

## Open

- **Unreproduced: a NaN in the tree.** The first core attempt ran one test slot per core and was ended by the OOM killer at 5 GB. In that attempt, `pipeline::space::repeat_tests::the_largest_tree_and_the_palm_grow_the_same_tree_again_and_on_any_cores` panicked at `repeat_tests.rs:40` with `a NaN in the tree`, and nextest's SIGTERM reached it afterwards. The same test passed three times after that: in the two-slot attempt, in the one-slot run above and in the gate. The cause is unknown. The rerun overwrote that attempt's log, so the panic text quoted here was read from it before the overwrite and is not retained. Not chased (host, 2026-10-24); it stays open until it reproduces or is ruled out.
