# fn-274-the-gate-fits-beside-the-owners-desktop.1 Whole-preset tests hold one tree at a time

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The repeat test compares builds by a build-order fold of every field (the derived `Debug`, a NaN refused) and drops each tree before the next; a one-bit change, two leaf axes swapped (canonically the same tree) and a NaN are checked at its start. The eleven telperion-core unit tests over 0.4 GB, each building every shipped preset, take one process-wide turn (`one_tree::turn`), so only one whole tree is built at a time.

Measured (`.flow/evidence/fn-274-the-gate-fits-beside-the-owners-desktop/measurements.md`): the repeat test alone 5,036 to 3,752 MiB; the other heavy tests are unchanged at 0.4 to 6.2 GB, since each was already one Telperion build at a time. telperion-core's unit tests at 4 threads: before, killed at 13 GiB (over 11,651 MiB anon at 81 s); after, 7,541 MiB anon alone. The workspace gate at 4 threads passed at a 9 GiB cap (8,860 MiB anon, 1,551 s) and was OOM-killed at 8 GiB. R3's under-8-GB target holds for the binary alone, not for the gate. Test counts unchanged (451 in telperion-core; 1,348 run and 87 ignored in the workspace). `npm test` passed at an 8 GB cap.

Open: the last gigabyte would need the heavy tests to share one build per preset (packed_leaf x2, mesh and pipeline::tests each build every preset's whole mesh), which is a test-coverage call for the host.

stage: impl-review - ran (codex, 3 draws NEEDS_WORK on a test-count and NaN finding, fixed in 1a8c26f29; re-review SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 5bb776dd525744fc872725a41f13bc6e121df0af, 6f9a8d545ae428ad84e0b945af22a945d4fc5a72, 1a8c26f29586c9f4234f90e4691808b0ca6b17df
- Tests: telperion-heavy 9 --threads 4 -- cargo test --profile ci --workspace --no-fail-fast, telperion-heavy 8 --threads 4 -- npm test
- PRs: