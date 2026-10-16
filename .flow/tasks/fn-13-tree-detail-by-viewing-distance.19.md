---
satisfies: [R3, R5]
---
# fn-13-tree-detail-by-viewing-distance.19 Bound retained CPU memory for distant trees

## Description
Bound retained CPU memory for distant trees.

## Acceptance
- Retained memory bounded with farthest-first eviction on the 1,024-tree forest.

## Done summary
Distant residents drop dead structure and prototypes, share one wood cylinder, coarsen by distance and evict farthest-first against a 3 GiB soft watermark: 1,024 trees peaked at 1.73 GB with zero rejections (4.28 GB and a rejection at 664 trees before). The view still did not settle within 170 s (carried by task .22, then superseded).

## Evidence
- Commits: dedf536, 6437a72, 646b533, 39c85d3, 9899585, f62f558
- Tests: vitest run (248 passed); npm run typecheck
