---
satisfies: [R5, R6]
---
# fn-5-branch-until-the-tips-bear-leaves-one.5 What it costs: the curve, the counts, and a build you can still drag

## Description
Measures what the recursion costs and whether the counts reach the botanical order (R5, R6). Separate from the branching work because the measurement is most of the job and because the answer decides whether the spec's Boundary against an impostor survives.

**Size:** M
**Files:** `src/skeleton/colonize.ts`, `src/skeleton/grow.ts` (the node ceiling must scale with twig orders as well as step), `src/skeleton/shed.ts` (interior shedding as its own pass after the twigs, so it never collides with .4's continuity work in twigs.ts), `harness/skeleton-view.ts`, `harness/GrowerDev.tsx`
**Touches:** [src/skeleton/colonize.ts, src/skeleton/grow.ts, src/skeleton/shed.ts, src/skeleton/shed.test.ts, harness/skeleton-view.ts, harness/GrowerDev.tsx]

### Approach

- **Attractor association is the first cost.** `settle()` at `src/skeleton/colonize.ts:243-257` is an O(new nodes x attractors) linear scan with no spatial index. Every serious implementation uses a grid or kd-tree; at the node counts this spec reaches it stops being optional. A uniform grid is the smaller change and the envelope gives natural bounds.
- **The rig from the canopy work already measures GPU frame cost** with timer queries at vsync off across a four-point resolution sweep. Use it; do not build a second one.
- **Record the curve across the depth rail on both presets**: nodes, tips, finest wood, triangles, build time and GPU frame cost. This is the artifact R6 asks for and it belongs in the spec, not in a scratch script.
- **Interactive build is the user-facing cost.** A full build was 194 ms at today's depth and 413 ms at a 0.89 m step, before the local pass exists. Measure first, then decide between debouncing, a coarse preview while dragging, or nothing.
- **Report reaching `maxNodes`** in the panel rather than truncating silently.
- **Count leaves and state the reachable ceiling.** R5 wants the order compared against the botanical 10^5 to 10^7, and the count reachable at the top of the rail reported rather than clamped.

### Investigation targets

**Required** (read before coding):
- `src/skeleton/colonize.ts:229-260` - the attractor arrays and `settle()`, the scan to index
- `harness/stage.ts:205-225` - the renderer, the log-depth toggle and the pixel-ratio pinning
- `harness/skeleton-view.ts:132-152` - `TreeStats`, already carrying draw calls, instances and both pixel ratios
- `harness/GrowerDev.tsx:280-300` - the panel line and the sweep control

**Optional** (reference as needed):
- `src/mesh/surface.ts:319-320` - `positions`/`indices` as plain arrays pushed one number at a time before a final typed-array conversion, a likely allocation hotspot at a million triangles

### Key context

- A 4x4 per-instance transform is 64 bytes, so eight million elements is about 512 MB before geometry. A packed format is the difference between the top of R5's range being reachable and being arithmetic.
- The spec's Boundaries admit a far impostor only if the measurement demands one, and the research says it will somewhere inside R5's own range. Report the number that settles it; do not relax the Boundary by assertion and do not discover it at 400 MB.
- WebGL2 has no compute and no writable storage buffers, so instance transforms are built on the CPU and uploaded. Changing renderer is out of scope; the constraint is recorded because it bounds what the top of the rail can mean.
- A geometry crossing 65,535 vertices silently doubles its index memory when three promotes the index to `Uint32`. Put it in the table.
- Vitest runs in Node with no GPU, so anything about draw counts or frame cost is asserted on CPU-side data and the frame budget is a harness procedure the owner runs.

### Acceptance
## Acceptance
- [ ] Attractor association uses a spatial index rather than a linear scan, with the speedup measured at the top of the rail
- [ ] The cost curve is recorded across the depth range on both presets: nodes, tips, finest wood, triangles, build time, GPU frame cost (R6)
- [ ] Leaf count is measured on both presets and stated against the 10^5 to 10^7 range, with the reachable ceiling reported (R5)
- [ ] A count the frame budget cannot carry is reported as the measured ceiling, never quietly clamped (R5 error case)
- [ ] Reaching `maxNodes` is reported rather than silently truncating (R6)
- [ ] Building at depth stays usable while a dial is dragged, by whatever the measurement shows is needed (R6)
- [ ] When the timer-query extension is unavailable the panel says so and reports no timing number (R6 error case)
- [ ] The done summary states whether the measurement demands an impostor, with the number behind the answer
- [ ] `npx vitest run`, `npx tsc --noEmit` and `npm run build` green
- [ ] Interior twigs are shed by the shell rule the leaf culler already uses, and the leaf count is reported after shedding (R5)
- [ ] A shell rule that sheds everything or nothing fails, by the same floor-and-ceiling guard the leaf culler carries (R5 error case)
- [ ] The depth both presets ship at renders inside 16.7 ms at the display's native pixel ratio on the RTX 3080, measured with the rig at vsync off (R6)
- [ ] The reachable ceiling in R5 is the count inside that frame, not the count before the tab dies (R5)
- [ ] A shipping depth that cannot meet the frame is reported as the reason an impostor is demanded (R6 error case)
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
