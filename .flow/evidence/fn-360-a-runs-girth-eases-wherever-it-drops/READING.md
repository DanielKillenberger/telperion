# Reading before the change (R1)

Builder's reading of the base, `b93621b5b`, 13 Nov 2026, written before any product change. Each item is the spec's "Not checked by the host" list, read against the code. "Read" means the code was read; "measured" is marked where a number was run.

## Where the girth is drawn today

`surface/samples.rs::girths` walks one run from its first node. Each node's girth is its own `nodes[i].radius`, except past a fork node the run passes (`paths.forks[i]`, not at k 0): there it is `own + (at − own)·h(t)`, `at = nodes[f].radius`, `t = (d − d_f)/(2·at)`, `h = (1 − t)²(1 + 2t)` for t < 1. A later fork on the same run replaces `f`, so the ease restarts from the later fork node's own radius. Then the 1-in-1 floor: `girth = max(girth, below − Δd)` against the drawn girth at the node before, skipped for a branch run's k 1. The trunk run multiplies the girth by the flare; a branch run multiplies it by the collar swell. Nothing else in the surface stage computes a girth.

## The four questions

**1. Where a run starts on its parent.** A branch run's path is `[attach, first, …]` (`paths.rs`). `girths` skips the floor at k 1 (`starts = !trunk && k == 1`), so the lateral's first own node is drawn at its own radius, never eased down from the bearer. k 0 (the attach node) is yielded but `branch_run` skips it: the socket ring is built from the bearer's node radius `pr = nodes[attach].radius` (sink `min(fork_socket·pr, socket_containment·inscribed)`, radius `min(start_radius·swell, contained)`). The collar is the swell, `1 + (swell − 1)·exp(−Δd/pr)`, a multiplier on the girth from k 1 on. So the collar has its own rule, applied on top of the girth, and the new rule must start every branch run's walk at k 1 at its own radius as today; it then does not fight the collar. A trunk run's walk starts at its root node (k 0), as today.

**2. The bark coordinate and the curve clusters.** The bark's first coordinate is the ring's distance along the wood, `Sample::d` = `distance[i]` (`curve.rs` `along: s.d`, `rings.rs` `encode::coord(d, angle, along)`); it never reads the girth. Clusters read each ring's drawn radius only for their bounds, reach, largest and smallest radius and level errors (`curve.rs` 200–231), recomputed from whatever radius is drawn. A shaped run (`section::reshape`) overwrites every ring's radius with its cell's extent, so its girth is not read at all. None of them needs a particular girth.

**3. Consumers that might need the drawn girth to equal the node radius at a node.**
- Children of the wood (lateral runs): sockets read the bearer's node radius `nodes[attach].radius`, not the drawn girth (`samples.rs::branch_run`). See 4.
- Leaves and needles: `foliage/station.rs` seats a station by casting along its radial onto the drawn rings (`AttachmentSurface::point`), blended with `centre + radial·wood` by `surface_contact`, where `wood` is the node radius interpolated along the segment. The seat follows the drawn surface whatever its radius; the blend at contact under 1 sits between the node radius and the drawn surface, as it already does wherever the fork ease, the floor or the swell draws wider than the node.
- The wire and the distant levels: the render crate draws from the curve (`telperion-render/src/curve.rs`), which carries the drawn radius; nothing there reads a node radius against it. Other readers of `nodes[i].radius` (foliage plan and placement, field splat, the stage's structure, sag, measures) read the tree's radius directly and never the drawn one.
- No consumer found needs drawn = node radius at a node. Today the drawn girth already differs from the node radius at every ring the fork ease, the 1-in-1 floor, the flare and the swell touch.

**4. Does a child attach to the bearer's drawn surface or to its node radius?** To its node radius (`pr`), for both the socket's sink and its contained radius. The new rule only ever draws the bearer at or above its node radius (the spec: never below the grown tree's radius), so a child cannot start off the bearer's drawn wood: the gap between them can only close. A child may start deeper inside the bearer's drawn wood where the bearer is drawn above its node radius after a drop. This is measured after the change (each lateral's first own ring against the bearer's drawn girth at the attach node, before and after, counted by flips from outside to inside); if any child newly starts inside, the run stops there with the site and the numbers (spec, Stops).

## What the change touches

`girths` only, and the docs' sentence on the sweep's ease (`docs/pipeline.md`, the wood paragraph) and the module doc of `samples.rs`. `paths.forks` stays as it is (`FORK_FROM`, `FORK_AT` in `pipeline/space.rs` decide the codominant mark upstream; `paths.rs` reads the mark); whether `girths` still needs `forks` once the ease is one rule is read off the code while building.
