**Use one continuous attachment resolved by the space engine, and materialize it as a join node in the drawing.** The inserted-node option is the smallest defensible solution with the current representation. There is no existing interpolated lateral attachment that makes it free.

Laterals today have an important distinction:

- **Placement:** a normal lateral starts at its phytomer’s tip; a *woken* lateral starts halfway along the preceding internode ([geometry.rs:290](/home/daniel/Projects/telperion/.worktrees/dents/crates/telperion-space/src/geometry.rs:290)).
- **Drawing:** both attach to `drawn[parent][node]`, with no attachment fraction ([space.rs:359](/home/daniel/Projects/telperion/.worktrees/dents/crates/telperion-core/src/pipeline/space.rs:359)). Their first edge starts at their own radius ([space.rs:473](/home/daniel/Projects/telperion/.worktrees/dents/crates/telperion-core/src/pipeline/space.rs:473)). The surface sinks the branch’s base into that **node’s position**, using socket containment and swelling ([samples.rs:110](/home/daniel/Projects/telperion/.worktrees/dents/crates/telperion-core/src/pipeline/surface/samples.rs:110)). The device likewise records `held.pos` as the edge’s base ([dress.wgsl:354](/home/daniel/Projects/telperion/.worktrees/dents/crates/telperion-render/src/generation/fine/dress.wgsl:354)).

Thus a relay can reuse the lateral’s **separate base radius and socket treatment for its stub**, but cannot reuse an existing between-node attachment mechanism.

The options resolve as follows:

| Option | Assessment |
|---|---|
| Split by `t`, remove old-tip share | Necessary for continuous sampled girth, insufficient for drawing. Redistribute the **whole** relay section, not merely retain `b` of it. The upper sample still contains `t` of the relay. |
| Snap to the lower node | Fails geometric continuity at node crossings, even if R4 passes. |
| Read phytomer radii unchanged; change only the stub radius | Leaves the parent sample below full relay girth. A small stub does not repair the constriction leading into the thick relay. |
| Insert join node | Can satisfy all constraints, provided girth supplies its radii and retains continuous phytomer samples. Insertion alone cannot fix R4. |

**One further correction matters:** today’s `t` is not the final attachment. Geometry blends the stand toward the parent tip by `b`, and the stand itself blends across growth units ([geometry.rs:215](/home/daniel/Projects/telperion/.worktrees/dents/crates/telperion-space/src/geometry.rs:215), [stand.rs:46](/home/daniel/Projects/telperion/.worktrees/dents/crates/telperion-space/src/geometry/stand.rs:46)). Simply deleting `1-b` from girth leaves another positional disagreement.

I recommend:

1. Resolve one attachment as **carrier span plus fraction**, including `relay_at`, unit extent and `b`. Blend along the carrier’s length so the attachment stays on it; derive the relay base from that.
2. Girth and sag use that attachment. Project the whole section/load onto neighbouring samples for continuity. Girth also supplies the physical join and stub radii, computed before secondary-growth weighting, without reverse-engineering drawn radii.
3. Insert one ordinary drawing node there. Continue the bole into the relay; start the old head with its own wood, including any laterals it bears. Mirror the rule on the device.

A virtual attachment could avoid the node, but would introduce span-attachment handling into downstream consumers. The current node-based pipeline makes the ordinary join node the simpler choice.

This is a code-grounded design recommendation; I changed no files and ran no validation.