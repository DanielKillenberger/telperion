# fn-241 friction

## 2026-10-09: a fresh worktree builds everything cold

- **Doing:** starting R1 to R3 in `.worktrees/found`, which has its own `target/` (as AGENTS.md requires), no `node_modules` and no built Wasm.
- **Slowed:** the first `cargo test --profile ci -p telperion-render --no-run` compiles the whole dependency tree, and npm test needs `npm ci` plus both Wasm builds before vitest can start.
- **Cost:** minutes of wall time before the first measurement; numbers recorded below as they land.
- **Would remove it:** nothing in the repository; a separate target per checkout is the rule, so a cold first build is its price.

## 2026-10-09: the first R1 instrument ran out of device memory

- **Doing:** measuring every shipped species at 40 and 80 on both paths in one test.
- **Slowed:** holding the CPU mesh and both prepared trees at once ran the spruce at 80 out of device memory ("Out of Memory", `DeviceRefused`) after 80 s.
- **Cost:** one 80 s GPU run and a rerun (91 s).
- **Would remove it:** nothing to change in the repository; the instrument now drops each tree before the next is made.
