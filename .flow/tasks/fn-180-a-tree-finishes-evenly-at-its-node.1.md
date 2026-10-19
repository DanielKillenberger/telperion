---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-180-a-tree-finishes-evenly-at-its-node.1 Implement A tree finishes evenly at its node budget

## Description
At `maxNodes` give up twig detail evenly so every branch reaches its tips; make `maxNodes` a dial with its cost.

## Acceptance
- R1-R5 of the spec.

## Done summary
Detail is thinned in sixteenths of a generation from the finest order; every branch reaches its tips. Stubs at the default budget: plane 55,626 to 178, beech candidate 34,087 to 585. Diagnostics split into `reduced` and `incomplete`. `maxNodes` is a dial stepping from `ranges::DEFAULT_MAX_NODES`, about 0.2 to 0.9 s and 90 to 150 MB per 50,000 nodes. Under-budget trees byte-identical on every preset at seeds 1 and 7. Skeleton 3-4x slower at the budget; whole request plane 1,003 to 1,887 ms, beech 5,127 to 4,898 ms; memory same or lower. Size trim: regrow per level instead of cloning (field wasm +0.5%). Owner question (R4): do the stubs go away? Codex SHIP.

## Evidence
- Commits: 11a3682917cb9e00e445d9b5a1f19cc414eb7a7a, d42bd8eee558782b94fb581e26674e4329e8d881, b7218ba804460e9a44c63eeb8519b91717bd92f3, eaaa796311ac4ef5613e4f706937aace9dd7ba19, 1eb62455fa4aeb96042a52a238f1139691cb10df, e9f5b7e5abaeb332cc02f95caabb000999435f6f, 4102e838e289cee7b6c4b3f979a023a97457cbb6
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs:
