# A breakthrough in generation time

## Goal & Context

The owner (2026-10-09), after the release branch's harness: "build time isn't fine for the longterm goals we have. It's fine for now. We still need to make some sort of breakthrough in generation time." The long-term goals on record: retuning a tree in the harness near-instant (2026-10-06), many trees in a game environment (2026-10-06), realtime and never baked offline (2026-10-05). Today, warm: beech at 80 about 0.8 s, Laurelin 0.9 s, Telperion 1.2 s on the GPU route; the site's coarse oak 1.1-2.7 s in Wasm; first load 15-20 s of shader compilation; a blend frame up to 10 s. fn-209 established the cost is node count (about 0.2 µs a node per CPU stage) and that camera-driven laziness does not shorten a near tree's rebuild. After the release.

## Findings to chart from (host, 2026-10-09)

- **Exact arithmetic (contract A) can go:** the experiment (`EXACT.md`, scratchpad `exact-out/`) nudged every decision boundary at 1e-7 and 1e-6; 0-2 of 3.5-19M decisions flipped, height moved at most 0.1 mm, no branch appeared or vanished, stills indistinguishable. Contract A's emulated 64-bit fixed point is the main cause of the `grow` shader's 13-17 s cold compile and a large share of the Wasm cost (`PROFILE.md`).
- **Light is a switch between ways of building:** `Species::reads_light()` (`crates/telperion-space/src/species.rs:423`) is any of four light rows non-zero, and it selects a different grower (the stepped CPU executor, `fine/coarse.rs::lit`, `grow.rs::apart` refusing the GPU's fine roots). Only the oak sets those rows, which is why the oak is slow on every path and why the oak→birch walk escaped the size factor (fn-251). The owner (2026-10-09): "i'm still not comfortable with only the oak growing with light... seems like it breaks our design principles". It does: a parameter must change the tree by degree, never switch the way it is built. Light belongs in the one growth path for every species, cheap and parallel (the owner's earlier ask: per-bud light inside the GPU expansion, no extra passes), its rows only setting how strongly a species responds.
- **The release bar (fn-237 R0)** and the scorecard (`EQZB24YMij8rnJgEXh36rz`) show memory, build time and the site behind master.

## Requirements

- **R1 (shape it first):** this is an unshaped, consequential idea; it is charted (`/flow-next:chart`) before any spec is planned: where generation time goes today per stage and per species (CPU structure above the cut, closed-form predictions, device expansion, upload), the targets that would count as a breakthrough for each goal (a retune, a forest of N trees, a site grove), and the candidate directions with their evidence, decided one at a time with the owner.
- **R2:** the chart's decisions become their own specs.
- **Charted (host, 2026-10-09):** the owner said yes to three pillars (one growth path for every species with light by degree, ordinary floats in place of contract A, growth on the GPU) and asked for Astra's view; Astra's architecture review is `ASTRA-ARCH.md`. Staged as fn-254 (floats) → fn-255 (one light-reading grower, CPU first, light model gated) → fn-256 (whole grower on the GPU) and fn-257 (the site's field at its resolution); fn-258 (memory and frame rate, independent) and fn-259 (release qualification).

## Boundaries

Map for fn-254..259, which are release items (the owner, 2026-10-09, on the release bar). No code in this spec.
