---
satisfies: [R1, R2]
---
# fn-209-tree-space-f1-fine-orders-grown-in-the.2 Fine roots grown on the GPU executor

## Description
WGSL executor of task 1's steps for fine roots, inside fn-125's GPU expansion, device-resident through rings, stations and curves (Astra change 3). Bounded execution per item with an overflow rule that preserves the botanical result. fn-12's lessons: precision and cold cost including transfer.

## Acceptance
- Task 1's adversarial cases and the four species at seeds 1 and 7, ages 40 and 80 give identical topology by lineage on both executors; geometry and contacts within fn-125's screen (50 µm max, 10 µm RMS).
- Repeated with different item order and chunking.
- Per-item and whole-tree GPU time, cold and warm, including uploads.

## Done summary
Host decision 1 built on the CPU reference (fixed.rs, fixed/decide.rs): time in ticks of 2^-20 cycle, every per-bud threshold (D5, D6, D10, D11, D15, D18) one integer procedure. Topology identical in all 28 dumped trees; tips at most 24.5 um, radii at most 1.56 % on woken buds; stills of the four species at 80, bare and in leaf, indistinguishable (FINE-CONTRACT.md, "Host decision 1 built").

GPU executor (crates/telperion-render/src/generation/fine/): the contract in WGSL bit for bit with the CPU (12 functions x 262,144 cases); a unit kernel; P2's chain grower with dormancy, waking, relay nodes and shedding (D15), one invocation per chain; fine roots expanded wave by wave, the overflow rule being the chain (an outgrown chain regrown in a region 8x larger). Topology by lineage identical to the CPU reference executor before geometry for beech, spruce and oak (light off) at 25, every chain of five adversarial variants at 25, and beech and spruce at 40 and 80, seeds 1 and 7, forward and reversed, in batches of 65,536 / 4,096 / 1,000 / 777. Timing (grow and shed only, uploads included, load 0.2-4): spruce 80 about 165 ms warm / 280 ms cold on the GPU, against 6.8-7.4 s for every chain on one CPU core and 1.1 s for the CPU executor's whole tree on all cores (that includes geometry).

Not built: scale, girth and lay on the GPU (so no geometry check against fn-125's screen yet), and the device-resident handoff to rings, stations and curves. That handoff is a design question for the host (GPU-FINE.md section 5): those stages exist only on the CPU today, and options (a), (b) and (c) are listed there. Workspace gate green under the GPU lock.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: d924da9723e3af922501694e4326400b216341cb, cbdd0a5fec57355419ee64578634d9787c540045, a1cc788ae97ebcd9178abb72f0c4060bf379571a, e861b02f740cfe068e886e3cf1583f72391c6d66, 2c90033049bf20abb955f8926aeacc1f7ca3b6bd, bf99fae6c756b1c05eb963527a99aafa63677e72, 6a36b02b1590b58f4a15208e927d5459caa0b05e, 834bbe38dab70df5100a275c562a7412ae98a6a4, 550528b7d8f6c3e01d3157329a90ce1dd6c4f3f3, 1c8b311b18ce93992269a8aa8025fc6543e63812, 15ca59de78e78876ba23b210eae8b733a1be1752
- Tests: cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-render fine_roots_at_40_and_80 -- --ignored --nocapture, cargo test --profile ci -p telperion-render chains_at_40_and_80 -- --ignored --nocapture, cargo test --profile ci -p telperion-render contract_cost -- --ignored --nocapture, cargo run --profile ci -p telperion-space --example contract -- dump/compare
- PRs: