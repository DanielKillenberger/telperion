# Large mature trees at full fidelity

## Goal & Context

The owner's species review (2026-10-15, fn-237 R3) asks for larger trees in four of eight species: the beech "big crowns with leader branches and full foliage", the birch "larger species with big leading branches", the spruce "fuller curtains and larger species", and the oak, whose closed dome no shipped age reaches (fn-236). The owner's reading: "i think all have a similar problem is that we're not efficient enough with the generator to make large species at full fidelity. We need to have a plan on how to figure this out post release."

What is known today: the engine refuses a tree past 20,000,000 phytomers (`pipeline/space.rs`, "engine phytomer budget"); the oak at 120 is refused by it, and the oak at 80 at 6 of 50 seeds (fn-281, `SWEEP.md`); the release bar holds every shipped tree under 0.5 s warm at its preset age (fn-237 R0a), Telperion and Laurelin about 1.25 s; Telperion's device peak was 4,209 MB (fn-270). The owner (2026-10-15): "it's also memory and node constraints generally". Three limits bind a large tree, and each is charted on its own: time to build, memory (the build's peak and the drawn tree's resident size, on the device and in Wasm), and node count (the engine's phytomer budget, and the drawn nodes and leaves a frame must render). Unknown: how build time, memory and node count grow with age per species, and how much of a large tree's cost is detail no supported view can see. fn-245 charted generation time at today's ages; this spec charts it at the ages and sizes the owner asks for.

## Requirements

- **R1 (measure first; no code changes):** per shipped species, at seeds 1, 7 and 42, from its preset age to its sourced mature age or the first refusal: phytomers, drawn nodes and leaves, build time warm on the RTX 3080, build peak memory and the drawn tree's resident memory, frame time whole and close, each stage's share of time and of memory, and the refusal that stops it (budget, memory or time, named). Plotted per species against age.
- **R2 (is it cost or values?):** for each of the four verdicts, whether the look the owner asks for is out of reach by cost (budget, memory, time) or by values the preset does not ask for (a fuller crown or longer curtains at today's node count). The second kind leaves this spec as a species values spec.
- **R3 (targets):** each species' sourced mature size (height, crown spread, age) checked against its source text, and the views it must hold at (the harness whole-tree and close, the site's grove), with the bar for a tree of that size agreed with the owner on all three limits: build time, peak and resident memory, and node and leaf count (or frame time).
- **R4 (where detail is invisible):** at each R3 view, how much of R1's count lies below a pixel (twigs, leaves and wood under the view's resolution), measured from the built tree, not estimated.
- **R5 (directions, each probed cheaply before any decision):** the candidates R1-R4 support, for example detail by view inside the one pipeline (no second pipeline), the per-phytomer cost of the grower as the live set grows, a node budget set by what the view can show rather than a fixed 20,000,000, the build's memory sizing (fn-270 R4) and the drawn tree's memory per node, growth continued across frames (fn-270), and the network of fn-268. Each is put to the owner one at a time; each decision becomes its own spec.

## Boundaries

After the release (fn-237). A chart: no production change in this spec. Not covered here: the oak's and birch's low branches (fn-282) and the flowers of Telperion and Laurelin (owner's verdict, 2026-10-15).

## The kernel programme (host, 2026-10-16)

Astra's release review (2026-10-16, `.flow/evidence/fn-237-release-the-tree-space-engine-replaces/ASTRA-RELEASE-REVIEW.md`), "After 0.2.0" 2, and the owner (2026-10-16): the kernels' own speed is after the release. After fn-257 lands, growth is reprofiled before any work is chosen; GROWTH-PROFILE.md ranks the spruce's units and wake (82-86 ms stamped) and balance and allotment (62 ms) first, as ranking evidence, not additive savings. Redundant work and kernel dependency depth are investigated with yearly growth and the measured look kept. fn-245's still-live performance obligations, and fn-209's and fn-211's, are carried here. The rejected aggregate, hierarchy-light and grown-at-birth designs are not restarted without a new bounded oracle (fn-245 COUNCIL.md).

## Carried from fn-259 (owner, 2026-10-19)

- Cap `Program::make_many`'s native compile threads to a small pool in place of one thread per entry (about −300 MB native peak, per fn-259 `CPU-PEAK.md`); measure cold start (`fresh_ms`) before and after. The field-call cost (fn-259 VERDICT.md L13-L19, accepted for 0.2.0 on 2026-10-19) is growth speed and is carried here too.
