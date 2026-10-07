# The Two Trees, and an ordinary tree: the host's vision (2026-10-07)

Owner, 2026-10-07: "I give you full creative freedom to make them look as much as Tolkien would have envisioned them!" (flow memory: the four missing species). This document is the host's design brief for Telperion, Laurelin and the ordinary tree on the tree-space engine. It keeps fn-10's standing rules: supernatural traits are separate from natural ones, off by default and on in the legendary presets; each legendary tree stands on a real natural base with a supernatural group; colours come from the text.

## What Tolkien gives

From *The Silmarillion*, "Of the Beginning of Days" (paraphrased; the text is not reproduced here):

- **Telperion**, the elder: leaves dark green above and shining silver beneath; countless flowers from which a dew of silver light was ever falling. Called the White Tree; the silver light is cool, the light of the later Moon. His line survives in Galathilion, Celeborn and the White Tree of Gondor (fn-10 sets that one on the silver birch).
- **Laurelin**, the younger: leaves of young green like the new-opened beech, with edges of glittering gold; flowers swinging on her branches in clusters of yellow flame, each shaped like a glowing horn that spilled a golden rain on the ground. Her light is warm, the light of the later Sun.
- **Both** waxed and waned in a cycle of seven hours each, overlapping at a mingling of the lights; light pooled from them, gathered as dew in great vats. They stood on the green mound Ezellohar, and at their height they were the tallest of the Valinor trees.

## The design

**Natural bases, by the text.** Laurelin's own description points at the European beech (already a passed engine species): her natural base is the beech, open-grown and broad. Telperion is the elder and the taller: his base is the beech's architecture made older and loftier (the engine's own beech at a greater age, a high narrow-shouldered crown), so the two trees read as kin of one form, as the Silmarillion makes them, and differ by their light, their leaves and their flowers rather than by being two unrelated species. With the supernatural group off, each is a convincing old beech.

**The supernatural group** (one group, off by default; fn-10's rule):

| Trait | Telperion | Laurelin | Engine or renderer gap |
|---|---|---|---|
| Scale | tall, the taller of the two | broad, nearly as tall | fn-10's `scale` row on the engine path |
| Leaves | dark green above, silver beneath; the silver shows as leaves turn in wind | young green, a thin gold rim that glints | two-sided leaf colour with an edge term (renderer) |
| Flowers | countless small white-silver blossoms over the whole outer crown | hanging racemes of golden horn-shaped flowers, like laburnum but flame-coloured | flowers on the engine path: blossoms and pendant racemes as organs on leaf-bearing nodes |
| Light | cool silver emission from the blossoms, a slow pulse, a falling dew of light | warm gold emission from the racemes, a golden rain of light | emitted light: the tree lights its surroundings and itself; dew as falling particles |
| Cycle | waxes while Laurelin wanes | the reverse | a shared time parameter, the two in counterphase, mingling at the crossover |

**What "looks better than the harness" means here.** Today's harness Two Trees are 148 m and 132 m envelopes with default brown and green. The new ones are recognisable trees first (an old beech's architecture with real branching), legendary second: their scale, their silver and gold, their flowers and their light.

## The ordinary tree

`ordinary` was the harness's default generic tree. On the engine it becomes **the space's centroid among the passed broadleaves**: a point between the beech, the oak and the birch, reading as a plausible, unremarkable deciduous tree (single trunk, rounded crown) with no reference species. It doubles as the visible proof of the space's claim that a point between species is itself a tree. It is judged as the species are, against the passed broadleaves' sheets rather than photographs.

## Order and specs

1. Silver birch (fn-213, running).
2. The ordinary tree (its own spec; needs no new capability).
3. Flowers on the engine path (gap spec; Laurelin and Telperion need it, and natural trees too, as fn-10 noted).
4. The supernatural group on the engine path: scale, two-sided leaf colour with an edge, emitted light with a cycle and falling dew (gap spec; the owner's "leave luminosity for later" of 2026-09-14 is lifted by the creative brief of 2026-10-07).
5. Laurelin, then Telperion (one spec each), judged by the host and Astra on sheets beside the text's description, with the group off (an old beech) and on.
