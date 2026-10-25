# Mature trees are the product; growth is hidden

## Goal

Telperion had two ways to make a tree from one preset and seed: the direct build (`mesh::build`, `branching::generate`), used by every still, `species:qa`, the protocol and every verdict since fn-34 round 2; and the growth path (fn-11, fn-30), which the harness drew by default. On 2026-09-17 the owner saw a different birch in the harness than on the judging page (73,337 nodes direct against 590,410 grown to age 100). The owner decided on 2026-09-18 that the mature tree is the product and growth is a hidden feature until it produces trees of judging quality.

## Decisions

- user (2026-09-17): "i swear i see some completely different build in the browser again"
- user (2026-09-17): "If this is seed 1. My seed 1 in the harness is 100% a different tree. More messy it has a ring the one you rendered in the pic doesn' thave. it's a different tree.."
- user (2026-09-18): "hm it seems growing the tree as of now seems not mature enough to produce high quality trees just yet. Should we just make it a hidden feature for now and work on just mature trees."
- user (2026-09-18): "alright let's do that"
- Hidden means not default, not deleted: the growth path stays buildable and pinned; fn-28, fn-30 and fn-31 are not rescoped. fn-31's "route production through growth" is superseded for master.

## Acceptance Criteria

- **R1:** `?species=<id>&seed=<n>` draws the direct build, the same tree as `headless --preset <id> --seed <n>`, with no growth controls — met.
- **R2:** `?growth=1` draws the specimen at the preset's age with growth controls, as before — met.
- **R3:** flag parser unit test beside the seed parser; `npm test` and typecheck pass — met (`growthFromQuery` in `harness/params.ts`, six cases).
- **R4:** CLAUDE.md carries the dated owner rule and the fn-31 note — met (now the "Mature trees are the product" rule in AGENTS.md).

## Result

The harness draws the mature tree unless opened with `?growth=1`; growth controls are absent on the mature path. No generator, renderer, preset, protocol or identity change. typecheck clean, vitest 83 passed.

## Open

None. (The growth path was later removed by fn-181.)
