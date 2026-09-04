# telperion

A procedural tree generator. Parameters and a seed in, geometry out.

Named for the elder of the Two Trees of Valinor, from which every other notable tree in Tolkien's legendarium descends. It ships with two presets, and the other one is Laurelin.

```ts
import { growSkeleton, solveRadii, buildSurface, getPreset } from "telperion";

const { skeleton: params, radii, surface } = getPreset("telperion")!;
const skeleton = growSkeleton(params);
const field = solveRadii(skeleton, radii);
const mesh = buildSurface(skeleton, field, surface);
// mesh.positions / .normals / .indices — hand them to three, or to anything.
```

`three` is a peer dependency and the only one.

## Why it does not look like other procedural trees

Most procedural trees are recursive branching with random angle jitter, and that approach is self-similar and statistically uniform by construction. Every branch is drawn from the same distribution as every other branch, so nothing in the tree was ever *chosen*, and the silhouette is emergent, which means it is always a blob.

This inverts the problem. **You author the silhouette and the algorithm finds a plausible branching structure that fills it.** Shape becomes a design decision; only the organic detail is generated. That is also what makes it repeatable rather than a dice roll: the envelope controls the outcome and the seed varies the details.

Four things follow from that, and together they are most of the difference:

- **Space colonization** ([Runions et al. 2007](http://algorithmicbotany.org/papers/colonization.egwnp2007.html)) grows branches toward attractor points scattered inside the envelope. Its ancestor is the same authors' [leaf venation work](http://algorithmicbotany.org/papers/venation.sig2005.html) from 2005, which is a good hint about where this library goes next.
- **A growth bias field** with named terms: gravitropism, lean, writhe amplitude and wavelength, spiral rate, and a per-step turn limit. A tree with no upward bias wanders down through its own crown and reads as brambles; a tree with no turn limit reverses on itself and draws visible zigzags. Both were measured, not guessed.
- **Thickness that conserves cross-sectional area through a fork**, roughly da Vinci's rule, with a tunable exponent. This is the single biggest reason CG branch junctions read as wrong.
- **One continuous swept surface** with a non-circular cross section that rotates along its length, which gives the plaited, rope-like trunk. Everyone else extrudes circles.

## The pipeline

Four stages, each usable on its own.

| stage | in | out |
|---|---|---|
| `envelope` | height, spread, crown base, fullness, shoulder | a solid of revolution, and points sampled inside it |
| `skeleton` | envelope, seed, bias field | nodes and parent links |
| `radius` | skeleton, fork exponent, trunk radius | a thickness per node |
| `surface` | skeleton, radii, lobes, twist, flare | one continuous mesh |

## Everything that affects the look is a named parameter

There are no magic constants. A quality that mixes two independent things gets two parameters, not one: a single `torsion` number cannot express both a slow S-curve and a corkscrew, while amplitude and wavelength can.

That rule is what makes the presets possible. **Telperion and Laurelin are not two algorithms, they are two parameter sets** — one narrow, upright and finely made, wrung about its own axis; the other broad, domed and spreading, carrying its mass sideways on a plaited trunk. Neither required a branch in the generator. Anything hardcoded would have been a difference between them that could not be authored.

Scale is the preset's business too. Every length in the library is a fraction of envelope height, so the library is scale-free and a 4 m sapling and a 150 m landmark come out with the same branching character. Physics that is genuinely size-dependent lives in the preset: elastic similarity puts a self-supporting trunk's diameter at height^1.5, so as a fraction of height it goes as height^0.5, and the presets say so out loud.

## It knows nothing about light

The library emits geometry and attachment frames. Materials, lights, exposure, bloom and post are the consumer's business, always.

Which is why the development harness renders in **clay** — flat grey, one neutral sky light, no bloom, no shadows for drama. If a tree is beautiful naked it is beautiful anywhere, and nothing is covering for weak geometry. Lighting is a toggle for checking, never the mode anything is judged in.

```
npm install
npm run dev     # the clay harness, with every parameter on a dial
npm test
```

## Status

The generator is complete and its output has been judged. Foliage is not built yet: the library grows branches and emits the frames foliage would attach to, and nothing more. Leaves, and the procedural leaf texturing that should come with them, are the next two pieces.

## License

MIT
