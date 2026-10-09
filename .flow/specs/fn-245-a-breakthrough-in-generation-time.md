# A breakthrough in generation time

## Goal & Context

The owner (2026-10-09), after the release branch's harness: "build time isn't fine for the longterm goals we have. It's fine for now. We still need to make some sort of breakthrough in generation time." The long-term goals on record: retuning a tree in the harness near-instant (2026-10-06), many trees in a game environment (2026-10-06), realtime and never baked offline (2026-10-05). Today, warm: beech at 80 about 0.8 s, Laurelin 0.9 s, Telperion 1.2 s on the GPU route; the site's coarse oak 1.1-2.7 s in Wasm; first load 15-20 s of shader compilation; a blend frame up to 10 s. fn-209 established the cost is node count (about 0.2 µs a node per CPU stage) and that camera-driven laziness does not shorten a near tree's rebuild. After the release.

## Requirements

- **R1 (shape it first):** this is an unshaped, consequential idea; it is charted (`/flow-next:chart`) before any spec is planned: where generation time goes today per stage and per species (CPU structure above the cut, closed-form predictions, device expansion, upload), the targets that would count as a breakthrough for each goal (a retune, a forest of N trees, a site grove), and the candidate directions with their evidence, decided one at a time with the owner.
- **R2:** the chart's decisions become their own specs.

## Boundaries

Not a release item. Discovery first, no code in this spec.
