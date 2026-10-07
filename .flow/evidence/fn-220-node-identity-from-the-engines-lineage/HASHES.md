# fn-220: node identity from the engine's lineage

The conversion (`crates/telperion-core/src/pipeline/space.rs`) sets each node's identity to the lineage key of the phytomer that made it (`Phytomer::key`, D1's node key). Base `db4194c8`, change `77b725fb`.

## R1: red, then green

`pipeline::space::tests::every_engine_node_has_its_own_identity` (the beech at 15, seed 7), on the base:

```
assertion `left == right` failed: two nodes share an identity
  left: 1
 right: 5224
```

All 5,224 nodes held the default identity. Green on `77b725fb`.

## R2: distinct, and stable across ages and cuts

- `every_engine_node_has_its_own_identity`: every node's identity is distinct and is the key of one of the structure's phytomers.
- `a_node_keeps_its_identity_across_ages`: grown at 14 and 15, every lineage (axis identity and place along it) present at both ages holds one key, and most of the nodes at 14 keep their identity at 15.
- `a_node_keeps_its_identity_across_cuts`: the tree grown whole (`grow_whole`) and expanded below cuts of 0, 16 and 1,024 converts to the same identities.

## R3: what changed

Each engine preset's artifacts through `artifact_digest` (its default age and seed), digested per line by `digest.py` here; the skeleton line is digested raw and with every node identity masked. First 16 hex digits of SHA-256.

| preset | skeleton raw, base → change | skeleton masked | element | wood | leaves | structure | field |
|---|---|---|---|---|---|---|---|
| european-beech | 41b59398 → fb1c5ab8 | 201fa50a = | e246de53 = | da586ddb = | 0c115c9f = | 17a41350 = | 0f70161d = |
| oregon-white-oak | 65b609e2 → d882e7ee | 03c99e2b = | b0cd0b4e = | 40414add = | d113155b = | 75e1fdfb = | 55d1f7aa = |
| norway-spruce | 1fc6d99a → 7905eb48 | 05500e50 = | c62c1de2 = | d952707f = | 49b2d2bc = | f8d6fed2 = | 47d6d111 = |
| date-palm | b9b17523 → 9e6183bb | 128fa5cc = | 7d59a475 = | 6eabe7c6 = | 376e743e → 2685990b | 68c2562c = | 099d9774 → 713fb0c4 |

Every converted tree changes in identity alone (the masked skeleton is equal). The beech, the oak and the spruce draw byte-identical output: nothing they draw reads identity (short shoots are off on every engine preset).

The date palm's fronds and the field read from them change. The rosette keys every frond's draws by its apex's identity (`foliage/rosette.rs`, `stream(seed, birth, frond)`); before this change that identity was the default `u64::MAX` on every engine tree, so the palm's frond jitter was drawn from one fixed stream. It now draws from the apex's own lineage. The count (6,554 leaflets) and the leaf box are unchanged; in the first leaflets read, the position word is unchanged and the rotation and scale words differ. This is the fix R2 asks for reaching its one current reader, not a regression; a still of the palm before (left) and after (right) is `raw/palm-before-after.png` (seed 7, the hero pose).

Viewed (the resumed worker, 2026-10-07, after the crash): the two palms are one tree. Height, crown spread, frond count and arch, the trunk's leaf-base lattice and the shadow match; only the frond jitter differs, leaflet by leaflet, as a fresh draw from each apex's own stream should. No frond collapses, doubles or leaves the crown.

## R4: gates

On `273e6772` (fn-216's flowers on top; the code tip of both tasks), 2026-10-07, after the crash:

- `cargo test --profile ci -p telperion-render --no-fail-fast` under the GPU lock: 161 passed, 0 failed, 11 ignored.
- `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render`: 994 passed, 0 failed, 41 ignored.
- `npm test`: 13 files, 143 tests passed.

The run before the crash (fn-216's `raw/gate-cpu.txt`) failed five `telperion-jev` tests on `Disk quota exceeded (os error 122)` and stopped in the Wasm boundary test; the machine's disk, not the code. Raw output: fn-216's `raw/gate-render.txt` and `raw/gate-cpu-resumed.txt`.
