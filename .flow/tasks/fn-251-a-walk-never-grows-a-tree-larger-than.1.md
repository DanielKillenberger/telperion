---
satisfies: [R1, R1b, R2, R3]
---
# fn-251-a-walk-never-grows-a-tree-larger-than.1 Repro, natural-scale rows, the size factor questioned, the sweep

## Description
As the title says; the spec states the requirements.

## Acceptance
- The spec's R-IDs named in `satisfies`.

## Done summary
The teaser's oak → birch frame at 80 grows within the 20M budget on F4's one cycle; the defect is gone. `examples/walk_size.rs`: 86 of 95 frames meet the seed-spread oracle (ends over seeds 1, 7, 42, widened by 10 % or two phytomers; ends grown once per age and light), none nears the budget. The nine misses are named exceptions held at their measured numbers by `tests/walk_sizes.rs` until fn-273 (host, 2026-10-14). The size factor stays shrink-only; a two-sided factor was measured (laterals 47 % short on oak ↔ spruce at 40) and not built. Evidence: WALKS.md. Impl-review SHIP.

## Evidence
- Commits: 958654d4890a0983bfe392c5d833a111c990bc3e, 20e856bbbf5af79c4b4a5d378fe5f43439566068, de87eef937d9ab36b575552314ae6846114c3532, 52160a8f25b94c7c9069ba876125629e7820ee3f, f9fc03bba02f9ec1c4de9a4bf0d58c416fc1a9b8, 37398bd2bb12f8fb570144ffa5d083ee56f20f81, e4088f1332db33b54831dca81b7228ef291a2f4d, 3485604088c6ccf9028bbe0b8687037274278511, bd532505921f9891dc19d527b43bd26d1bb52b88, 22a435a0c0417a2ac22c6c820126e13b51a020fd, 8cf010423250c7ef7c84b0a72623ec42f2b0ef78, f2a70e8864f71f378f6345a21234ce0efe22f741, 98b1e5ce9ee6bddca97e835e697d3633ece253e9
- Tests: telperion-heavy 13 --gpu 4 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1348 passed, 0 failed), telperion-heavy 8 -- npm test (162 passed), cargo test --profile ci -p telperion-core --test walk_sizes (red when a record is tightened, green as measured)
- PRs:
