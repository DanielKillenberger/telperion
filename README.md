# telperion

A procedural tree generator with an authored silhouette, seeded specimens and named species presets. One Rust core grows the structure; consumers choose surface meshes, instanced foliage, structural data or spatial fields.

```ts
import { TreeEngine, TELPERION } from "telperion";

const engine = await TreeEngine.create();
const family = structuredClone(TELPERION);
family.skeleton.seed = 7;
const tree = engine.build(family, { surface: true, foliage: true });
// tree.surface: Float32 positions/normals, Uint32 indices, bounds.
// tree.foliage: one leaf mesh, column-major Float32 instance matrices, bounds.
engine.release(); // returned arrays are owned copies and remain usable
engine.dispose();
```

`ORDINARY`, `TELPERION` and `LAURELIN` come from Rust preset metadata. Parameters define the family; the seed selects a specimen. `PRESETS` contains the Two Trees. The optional `materializeTree` / `disposeTreeGeometry` adapter supplies Three.js objects; the consumer owns materials, lights and rendering. Three.js is a peer dependency. The native core has no external Rust dependencies; `serde_json` belongs to the Wasm binding only.

For a block-based consumer, request occupancy without constructing a wood surface or transferring render buffers:

```ts
const engine = await TreeEngine.create();
const tree = engine.build(family, { field: true });
const flags = tree.field!.query(new Float64Array([
  0, 1, 0, 0.5, // cell centre x/y/z and half extent; zero means a point
]));
// Each byte: wood = 1, foliage = 2; zero is a valid empty cell.
engine.dispose();
```

Field handles expire on the next native build or release; copied query results remain owned. Field construction places retained foliage internally. `{ structure: true }` returns six f64 values per node (xyz, distal/proximal/base radius) and three u32 values (parent, branch, kind). The root parent is `0xffffffff`; kinds are structural 0, branch 1 and twig 2. An empty output selection still generates structure for diagnostics. Invalid inputs throw, and cap diagnostics distinguish incomplete growth from a finished tree.

## Architecture

The native entry is `branching::generate(&family.skeleton, family.radii)`. Its solved `Tree` can feed `surface::build`, foliage placement/culling, or `Field::new` independently. The Wasm binding assembles the requested stages; `src/browser` loads it and copies output arrays. There is no TypeScript generator.

| Owner in `crates/telperion-core/src` | Responsibility |
|---|---|
| `envelope`, `colonization`, `bias` | Authored crown, attractor growth, directional fields and turn constraints |
| `branching/local`, `twigs`, `radius` | Radius-driven branch generations, fixed twig anatomy, local taper and fork conservation |
| `surface` | Continuous swept wood, transported frames, lobes, twist and sockets |
| `foliage` | Leaf shape, anatomical stations, phyllotaxis and shell retention |
| `field` | Wood and foliage cell occupancy, independently of render meshes |
| `presets` | Named parameter sets and scale choices |

The crown envelope controls the silhouette. Space colonization establishes structural limbs, then local branch laws continue down to leaf-bearing twigs. Forks conserve cross-sectional area with a tunable exponent. Branch resolution and lateral count are independent. Terminal twig anatomy is measured in metres; the giant presets retain fine twigs instead of uniformly enlarging them.

Botanical and output changes have separate owners. A representative development exercise and the retained test mapping are in [the migration guide](tests/migration/README.md#changing-a-rule-or-an-output). Native mesh-free examples are exercised in `crates/telperion-core/tests/field.rs`; browser ownership and failure recovery are checked in `tests/browser/integration.mjs`.

## Build and develop

Install Rust through rustup, then:

```sh
rustup toolchain install 1.98.1 --profile minimal --component rustfmt --component clippy --target wasm32-unknown-unknown
npm ci
npm run rust:build
npm run wasm:build
npm run rust:test
npm test
npm run typecheck
npm run build
npx playwright install chromium
npm run rust:test:wasm
npm run dev
```

The repository pins Rust in `rust-toolchain.toml`. `dev` and `build` regenerate Wasm and its preset metadata. Build Wasm before running the Node harness tests from a clean checkout. The package embeds the Wasm binary; consumers do not need Rust. The development viewer exposes the parameters, both presets and comparison mode in neutral clay, with optional lighting inspection and GPU timing.

## Measurements and limits

The [FN8 report](.flow/evidence/fn8/REPORT.md) owns the matched full-build measurements, binding costs, native observations, memory-domain limits and GPU results. CPU generation latency and GPU frame time are separate measurements. Wasm linear-memory capacity is a high-water allocation, not live heap or total browser memory; release allows allocator reuse and dispose allows host reclamation once references are gone. Scene replacement retains the previous tree until the new build succeeds, so transient coexistence matters.

The archived [FN7 surface experiment](experiments/rust-surface-benchmark/REPORT.md) measured a narrower and older workload. Its numbers are historical, not a full-engine migration result. Further botanical realism and species visual QA remain future work; leaf appearance is currently judged as geometry in clay. Full lifecycle simulation is not implemented.

## License

MIT
