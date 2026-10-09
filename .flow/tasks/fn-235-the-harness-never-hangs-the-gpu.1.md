---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-235-the-harness-never-hangs-the-gpu.1 Find the hang natively, bound it, and fail by name

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The harness's hang was the chain grower's `grow` pipeline taking 92 to 145 s to compile on a cold NVIDIA cache (found natively on the browser route, `hang_tests::browser_route`; no tree hung on a warm cache). Slimmed in one pipeline to 13 to 17 s with output unchanged, and held as the named 20 s exception; every other pipeline is under 2.5 s and the first tree builds in 5 s besides grow, cold. The browser budget is 6 GiB with its derivation, and a tree that fits only alone lets the drawn tree go first, while one that does not fit even alone is refused by name with the drawn tree kept. A lost device is named and a fresh device rebuilds on the owner's retry. Every data-bound shader loop states its cap, and at 80 no submission runs past 177 ms, 256 dispatches or 2^23 workgroups. Device-limit probes run only when asked.

stage: impl-review - ran (codex, round 1 NEEDS_WORK: 4 findings fixed; round 2 SHIP, its P2 fixed after)

Decisions and open items:
- The split of grow_unit and move_on into their own pipelines was measured and not built: its dispatch floor adds 11 to 13 % to the beech (host's 5 % condition).
- Async pipeline creation: not built (host decision); wgpu 30 has no public API for it.
- Cold-compile bounds read the machine's load: grow made in 12 to 45 s across runs at load 7 to 40.
- Twice the spruce at 80 child exited 101 after the beech's; four runs since passed. Parallel GPU tests failed twice on NVIDIA VA-space exhaustion while other worktrees ran GPU tests outside this lock; serial runs pass.
- R5 (browser, cold profile, beech/birch/oak at 40 and 80) is the owner's, after the host's go.
- Scratch branch scratch-fn-235-birch holds fn-213's birch for the repro only; never merge it.
- chain.wgsl is 411 lines (from 397).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 98d128f70f11b761defec902d89776970cdbf66f, b0cd9821aec02dcd212172c554f4712f237a802a, 8dcb3b8c22642002973c1c60e6878b8cb9722cbd, b47801278d4843a703ef64a4faedb4a3d85c37f1, 75f1d9ef4ef2b0051280085f57993447277fe02c, 903d83e956da4d8721ea2340537006d2b1674f36, 691620d0454a8af5dfbe65730e829a59fa5e3a83, 80201566f7a61cf454484e49a89ef686cf12df1d, 953b2151a8687cadc24d9124d0916f17fd5a8544, cea78c32e6e7334f603d227a9c2698c797c517e3, 722d1262f79627290d74cf9a112ad8cc6f8f7806, 0a9f998972bd83a2dc9c0850e236b3010fe02784, 5f046438c2f30ecb893b16691a9675bf75ffafde, 65f8382343da5ba4d9a92403795e0c2ba58f67d1, 4fd690b77132fbf87c106710c4d9409ddd7d1b6d, dad34d533b660d9e54f1623de4d7762f9ef05243, 2ba7566a244b9de2f845cf293ffac60bd61f4cdd, 22847194c8a5cc220e33a8d74d80a4acf5527273
- Tests: flock gpu.lock cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, npm test, cargo test --profile ci -p telperion-render generation::fine --test-threads 1 (after review fixes)
- PRs: