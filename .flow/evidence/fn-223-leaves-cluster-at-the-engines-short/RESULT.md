# fn-223 result: leaves cluster at the engine's short shoots

Measured on 2026-10-08 on the branch at task fn-223.1, against base 891c3bd0, on an RTX 3080.

## Neutral rows are byte-identical (R2, R5)

No shipped preset sets `clusterLeaves`, so every tree should come out as it did at the base.

**Artifacts.** `artifact_digest` prints every artifact of one build. Its text from the base commit was compared with this branch's, after stripping the two empty fields this branch adds to `Tree`'s debug text (`, clusters: [], clustered: []`). A scratch script ran both binaries in turn.

| preset | base | branch |
|---|---|---|
| ordinary | c1d72ee7d1089d10 | same |
| oregon-white-oak | d3e83f43f18f7ac7 | same |
| european-beech | 0f5fe4d4bac709ee | same |
| date-palm | 796d92897fe5f91f | same |
| norway-spruce | 4eb3e4ba6abca52e | same |

**Stills.** `headless --preset <p> --seed 1`, 1024², built from the base and from the branch, compared with `cmp`. All five PNGs are byte-identical:

| preset | still sha256 (16) |
|---|---|
| ordinary | 401565e91c6a5c58 |
| oregon-white-oak | ee165f6d7b4df6a8 |
| european-beech | 93bfe2a08ce355e3 |
| date-palm | 523fdfd6ef6fbf17 |
| norway-spruce | 64d35ff4933d9e10 |

The beech still covers R3's migration. Deleting its four dormant `shortShoot*` rows changed no byte, because its `shortShootSpacing` was zero and they drew nothing.

**Catalogue identity.** The four digests were re-pinned (`tests/catalogue_identity.rs` header). The family, walk and override texts were rebuilt with fn-223's changes undone in the text: the cluster fields stripped, and the five rows put back at each family's values. Each rebuilt text reproduced the fn-215 pin to the bit. The refusal digest pairs rows by index, and the deleted rows re-paired it.

## Clustering on (R1, R4, R5)

The fixture is the beech at 15, seed 1, with PA 14 (its short shoots) at `clusterLeaves 3`, `clusterSpread 0.6`.

**Device path against the whole tree on the CPU** (`fine::cluster_tests::clusters_match_across_the_seam`):
- 128 clusters stand above the cut and 195 in fine roots.
- Every cluster matches by its tip's stream key, with an equal leaf count.
- Tips are at most 9.9e-7 m apart (screen 5e-5) and headings at most 5.2e-4 rad apart (screen 0.01).
- The leaf totals are equal.

**CPU** (`suite::clusters`):
- There is one 3-leaf segment per tip, on `cluster_key(tip)`.
- No station sits on short-shoot wood.
- `place_on` seats exactly the planned count, at least 3 of them at each tip.
- Run stations plus 3 per tip equal the total, which is below the unclustered run count.

**Conversion** (`space::cluster_tests`):
- Tips are exactly the last drawn node of each PA-14 axis.
- Axes that drew no node bear no cluster, and the case occurs in the fixture.
- Marks fall exactly on PA-14 nodes.

**Still** (`raw/european-beech-clustered.png`, beech at 40, seed 1): 59,113 instances. The crown reads as before, with its short shoots' leaves gathered at their tips. The birch's values are fn-213's.

## Cost (R5)

The beech at 40, CPU, `cost_of_clustering`, best of 5:

| clusterLeaves | build | stations prepared | stations |
|---|---|---|---|
| 0 | 303.0 ms | 61.2 ms | 240,047 |
| 3 | 260.2 ms | 30.5 ms | 190,382 |

Clustering costs nothing measurable. It replaces each short shoot's evenly spaced stations with three, so the build is cheaper.
