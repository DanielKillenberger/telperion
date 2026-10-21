# fn-301 .5: R3 digests, base d600fa12c against the change 4d21146c8

## Method

`t5-digest-main.rs` is a scratch program, kept here and not committed to a crate. It builds every R1 row through `pipeline::build`, with the request `artifact_digest` makes (wood, leaves, the field at n = 36, structure). A row is each shipped preset at ages 6, 8, 10, 12, 15, 20 and its default age, at seeds 1, 7 and 42, so 168 rows. Each artifact's Debug text, as `artifact_digest` writes it, is folded by `DefaultHasher`. The program ran as two scratch crates outside both worktrees, one with a path dependency on the base worktree (`.worktrees/f4-301-base`) and one on the change. Each crate had its own `target/`, and both used toolchain 1.98.1 with the `ci` profile's settings. Raw output: `raw/t5-digest-base.txt` and `raw/t5-digest-change.txt`. The per-row comparison is `raw/t5-digest-compare.md` (`t5-compare.py`).

## Result

- **Held rows.** The beech, Telperion and Laurelin are byte-identical on all 63 of their rows, in every artifact.
- **Rows expected to change.** All 47 changed: the palm on all 21 rows, the birch on all 21, the spruce at 40 (3 rows), and the ordinary tree and the oak at 40, seed 42.
- **Rows outside the host's list that changed.** There are three, so the run stopped here (the host's instruction):

| Row | Changed artifacts | Base R1 (`r1-base.tsv`): wood_low, leaf_low |
|---|---|---|
| ordinary 40, seed 7 | skeleton, wood, leaves, structure (field and element identical) | 0.039, 0.041 |
| oregon-white-oak 40, seed 7 | skeleton, wood, leaves, structure (field and element identical) | 0.041, 0.242 |
| norway-spruce 20, seed 42 | skeleton, wood, structure, field (leaves and element identical) | 0.020, 0.075 |

Digests (base → change):

```
ordinary 40 7          skeleton c78ec1b0d341b297 → b45431bbfb24ddf4  wood 51bd4ca18945748d → 3c9586e8a40f254d
                       leaves 2a0f7d052740f1e4 → bdd3083b4f0679f6    structure fa259c02f0bdb658 → d205d2e9560ad2e1
oregon-white-oak 40 7  skeleton 783fd15b66669e5b → e4db443deadab474  wood dfee17037d91d2b7 → eb6662bd985af5f6
                       leaves 14afdd8fc66223f6 → b31d1e95694984f8    structure 8d06c59bc0fb7871 → 03837b71c73bece9
norway-spruce 20 42    skeleton 236355bd58c72396 → 982d01ec1ef4a205  wood 27541025c709c839 → 97a90e7088d28fc2
                       structure 5a7afbfc47666121 → fcda9bd94d90e4e2 field f58f30832d977b80 → 9fccdfb7213145d9
```

The skeleton changes on all three rows, so the change enters at the lay walk or sag. Inferred, not measured: the base R1 probe read the wood from its curve's centreline, which stood 2 to 4 cm up on these rows. The lay now rests the bark (one girth radius up), and its landing ease and sag read the bark, so a low limb whose bark, not its centreline, reached the ground now moves. The palm at 40 was found the same way in .3. Whether these three rows join R3's changing rows is the host's call.
