# fn-274-the-gate-fits-beside-the-owners-desktop.1 Whole-preset tests hold one tree at a time

## Description
fn-274 R1-R3: the repeat test holds one tree at a time; heavy unit tests take one turn; gate peak measured.

## Acceptance
- [x] The spec's R1-R3 (R3's 8 GB target missed by under 1 GiB, accepted by the host).

## Done summary
The repeat test compares builds by a build-order fold of every field and drops each tree before the next (one-bit change, swapped axes and NaN checked). The eleven telperion-core unit tests over 0.4 GB take one process-wide turn (`one_tree::turn`). Repeat test alone 5,036 → 3,752 MiB; telperion-core units at 4 threads 7,541 MiB (was killed at 13 GiB); workspace gate at 4 threads passed at a 9 GiB cap (8,860 MiB, 1,551 s), OOM at 8 GiB. Counts unchanged. Codex SHIP after fix 1a8c26f29.

## Evidence
- Commits: 5bb776dd525744fc872725a41f13bc6e121df0af, 6f9a8d545ae428ad84e0b945af22a945d4fc5a72, 1a8c26f29586c9f4234f90e4691808b0ca6b17df
- Tests: telperion-heavy 9 --threads 4 -- cargo test --profile ci --workspace --no-fail-fast, telperion-heavy 8 --threads 4 -- npm test
- PRs:
