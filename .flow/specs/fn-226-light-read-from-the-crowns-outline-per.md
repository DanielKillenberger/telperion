# Light read from the crown's outline, per bud

## Goal & Context

The lit oak builds 5.6-6.2 times slower than unlit at 80 (OAK-LIGHT.md in fn-211's evidence: seed 7, 14.9 s against 2.4 s), because every cycle deposits leaves into a 0.5 m light lattice, samples shade at every bud, allots by light and re-lays every ten cycles: a whole-tree pass per year that stops the oak running ahead, keeps its coarse field (fn-214) refused and made its GPU path slower than the CPU (fn-211 task 10). Light earns its place: the owner and host compared light on and off against the reference photographs (2026-10-08): light on is wider (25-45 % at 80), open, with a clear trunk; light off is a dense column. Owner (2026-10-08): "Couldn't we approximate effects of light in a way that's easily computed for each bud in isolation => parallelization on gpu?" and "ok go on the light simplification".

## Requirements

- **R1 (measure first):** the lattice light's cost by pass (deposit, sample, allot, re-lay) for the oak at 40 and 80, and which light effects the sheet shows come from which pass.
- **R2 (outline light A):** each bud's light from its position in the crown's outline as grown, folded once a cycle (the exact whole-tree folds that exist: height, crown base, widest, shoulder): Beer-Lambert transmission exp(-k * leaf area density * path length through the outline) averaged over a few fixed sky directions, the leaf area density the cycle's total leaf area over the outline's volume; extinction and sky from today's site-light rows; computed per bud with no read of any neighbour; in contract A's fixed point.
- **R3 (outline light B):** the same with the outline the closed form expects at the cycle's age, so nothing crosses between roots and the structure above per cycle.
- **R4 (head-to-head):** today's lattice, A and B on the oak at 40 and 80, seeds 1, 7, 42: the comparison sheet with the oak's reference photographs (catalogue/oregon-white-oak references O-WHOLE, O-BARE; local cache .refs/fn9), measured crown width, openness (shed share), clear trunk height and build time; the owner's verdict.
- **R5 (one model):** the chosen model replaces the lattice; the others and the lattice are deleted (one light model); the per-bud light runs on the GPU executor; a terrain or obstacle occluder per bud (ray against a height field, still per bud) recorded as a named follow-up for the owner's cliff scenes; walks, census, gate, npm test green.

## Boundaries

The engine's light only; the oak's own crown shape against its reference (ragged against domed) is a species round, not this spec. The release scope is the owner's separate decision. Depends on fn-198 (F4).