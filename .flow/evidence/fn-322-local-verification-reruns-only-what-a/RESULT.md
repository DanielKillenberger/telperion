# fn-322 result

The local gate now skips any suite whose `scripts/ci-key.mjs` key has a green receipt, and the skip is proven. The child cannot close: its first full run found two failures on the goal's base, both reproducing alone, and the spec's rule for that stops the goal (`NEEDS_HUMAN`).

All figures are measured on this machine on 1 Nov 2026, warm unless stated, through `telperion-heavy 8 --threads 4`. Raw logs are in `raw/`.

## The two failures on the base

No Rust changed in this child (`git diff 0e544d803 HEAD -- crates` is empty).

| Test | Alone on the goal head | Cause |
| --- | --- | --- |
| `generation::tests::preparation_capabilities_and_failures_are_explicit` | failed 3 of 3 | The test is stale (inferred, then confirmed by the probe). `size` is now refused during `expand`, so the helper's `unwrap` panics on NaN at `tests.rs:224` before `.stations().is_err()` is reached. |
| `generation::resident::field_params_tests::every_preset_grows_its_field_from_its_parameters_as_from_its_id` | failed 3 of 6, silver birch at seed 1 or 5678 | The nondeterminism open spec fn-320 records: the device field is not repeatable within one process. The test passed in both full runs and failed in the later rerun, so it can pass a whole gate by chance. |

The first has a proven one-line fix on local branch `scratch/fn-322-probe` (`eeb12c8ea`), which is not on the integration branch: assert `executor::expand(tree.clone(), &f).is_err()` for the NaN size. With it the test passes alone. The second has no fix here; fn-320 owns it, and its cause is not yet named.

## R1, R2: the skip and its proof

`scripts/gate.py` reads each suite's key, prints skipped suites with their key, and writes a receipt only after the suite passes. A dirty tree outside `.flow/`, or a dirty `.flow/` file a key reads, is refused by name.

`scripts/gate.test.mjs` holds 21 tests that run the real `gate.py` and `ci-key.mjs` in a fixture git repository, with nextest, npm and cargo faked. Each of 23 temporary breaks of a rule turned the file red (`raw/mutations.txt`), including a dropped shared input, a receipt written before or despite a failure, and an unreported package counted as green.

## R3: what each change runs

| Case | Suites run | Wall | Note |
| --- | --- | --- | --- |
| Before this spec, any change | all | over 20 min | The figure in the spec. |
| Full run, `--full` at `73865198e` | all | 1,412 s, test run 1,179 s | One failure (the first above). |
| Nothing changed | none | 3 s | Render receipt placed by hand. |
| `.flow/` only | none | 3 s | Render receipt placed by hand. |
| `STRATEGY.md` only | none | 3 s | Render receipt placed by hand. |
| `src/` only | `rust-jev`, package, node | 127 s | With a real rerun of the render failure first, which stopped the Rust run. |
| Render crate test file only | `rust-jev`, `rust-render`, package, node | 599 s | `rust-core` and `rust-wasm`, which holds the space crate, were skipped. |

The hand-placed receipt: `rust-render` cannot earn one while fn-320's flake stands, so one was placed on disposable commits, only to time the skip path, and removed afterwards. It says nothing about the suite. Those three rows prove the timing and which suites were skipped, not a green render suite.

**Deviation from R3:** the spec says a `src/`-only change runs "only the node suite and the package check". It also runs `rust-jev` (13 s), because that suite's key reads `src`: its isolation test walks the browser source. Dropping that input would weaken a check, which R5 and the spec's playbook rule forbid, so the suite stays.

A Markdown file inside a key's input directory (`crates/`, `src/`, `catalogue/`, `docs/parameters.md`) does move that key and rerun its suites. Top-level Markdown and `.flow/` do not.

## R4: the failed set first

After a failed run the gate reruns nextest's failed set first (`-R <run>`, recording set by `.config/nextest-user.toml`) and runs the suites only if it passes. Seen in real runs: the rerun took 6 s to report the flake and stopped there, against 8 minutes for the suites. If the recording has nothing for these packages, the run falls through to the suites. `AGENTS.md` carries the new command and figures.

## R5: nothing weakened

The listed tests are the same 1,514, by package: core 534, jev 389, render 369, space 212, wasm 10, with 140 ignored. Both full runs ran all 1,514. No Rust test was touched on the integration branch, so the lists before and after match by construction, and the gate now fails if a workspace package is in no suite.

**Budget:** separate nextest runs per suite, the first build, took 1,649 s against the 1,560 s budget, a regression. They now share one pool, as before, and its JUnit report says which suites are green: 1,179 s. A run over budget writes no Rust receipt, so a rerun cannot hide it.

**Not rerun by a receipt:** `.config/gate-budgets.toml` and `scripts/gate_test.py` are outside the keys, since the spec names the shared inputs and CI's keys stay as they are. A tightened budget is only proven on a run that the key already triggers, or under `--full`. The gate gained `npm test` and `npm run typecheck`, as the node suite (86 s warm), which it did not run before.

## R6, R7

R6: `AGENTS.md`'s review section now says a review covers the child's own diff, and a `.flow/`-only task skips it with the reason recorded. R7: this child ran in the goal's one checkout, and no cold build happened.
