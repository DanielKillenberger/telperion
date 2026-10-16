---
satisfies: [R4]
---
# fn-270-growth-continues-from-any-age-in-less.3 Memory, one lever at a time

## Description
R4, a labelled census after each lever.

## Acceptance
R4's targets or each miss measured; guard and full-card run held.

## Done summary
R4's levers in order, a census after each: lever 1 split state and scratch; levers 2+4 merged (tables start at margin 1.0, grow 1.5x by observed counts); lever 3 froze a closed subtree's girth and leaf-mean sums, records and inside axes gone; compactor sharing dropped. Telperion 4,209 -> 2,484 MB (-41 %), continuation 3,072; spruce 1,522, oak 782, birch 520, beech 282 (targets missed, floors measured). Girth shift at most 5.4e-5. Gate green after a girth bound; npm 162/162; codex SHIP.

## Evidence
- Commits: 5f21707587d6b2abe3b0f99f1e0a3d636b0d0ebe, bcc2e2b6e161ae38de69dc08500b956174d2626d, f59662392e2df0176013a8c55a77b9d6de4528de, 46865341ffe489c6ff4abd5f3aaa42362eac8167, c980c11e63ea8aff705b6bc2580182f9b8050f4a, 9acb54755676cc7ab3501235f8257302030a82d3, 95bc4b7f89a801bad2341304d2b6b0afa46acb7f, d4519b88f9158d252cf7fa7d1b3b5366ef64892d, 1616c7204af597bcc176e54518820f49c10e1967, 9581fed64c2edacabb342ab849f2bebb3f91f496, db538af8b567ae15d27c253bf1d8f45fa7b3ac0d, 1479a98b6213d533499db23f2241ac8abae2c72c
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: