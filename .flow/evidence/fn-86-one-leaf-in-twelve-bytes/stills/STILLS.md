# fn-86 stills: the twelve-byte leaf, drawn

R8. One headless still per catalogue species that has a preset, at the
protocol's first fixed seed, captured after the numeric gates passed and
never before them. Taken on a real RTX 3080, one process a still, no browser
and no page in the path. The owner awards the verdict; this file records only
what was drawn.

Captured at `a5c16243`. The renderer change landed in `5b49227c`; the two
commits since it touch no file under `crates/telperion-render` or
`crates/telperion-core/src`, so these stills are the landing commit's
renderer, drawn from the landing commit's generator.

```
target/release/examples/headless --preset <id> --seed 1 --view whole \
  --size 960x720 --out <png>
```

| Still | sha256 | Foliage instances | Triangles drawn | Foliage casters |
|---|---|---|---|---|
| `oregon-white-oak-1-whole.png` | `48685f03` | 715,065 | 130,219,181 | 178,767 |
| `norway-spruce-1-whole.png` | `e2c64d8b` | 7,353,754 | 358,874,313 | 1,838,439 |
| `european-beech-1-whole.png` | `25941edc` | 4,928,812 | 227,632,969 | 1,232,203 |
| `silver-birch-1-whole.png` | `95694a08` | 226,057 | 35,729,741 | 56,515 |

Every instance count here is the retained count `species_measure` reported for
the same species at the same seed, so the renderer drew the leaves the
generator counted, decoded from the three words rather than from a matrix.
Each tree casts a foliage shadow, which is `shadow.wgsl` decoding the same
words the foliage pass does.

The European ash is the fifth catalogue folder and has no still. It carries
no preset (`catalogue/european-ash/README.md`: "Preset | none yet",
readiness `unready`), its `pins.json` is `{"empty": true}`, it appears in
neither profiles file, and `species_measure`'s own help says it "is not a
catalogue species". There is nothing to draw and nothing to gate; it is named
here so the gap is not mistaken for an omission.
