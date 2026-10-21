# A tree rests on its ground

## Goal & Context

The owner judged fn-283's stills (2026-10-20): "at 12 doesn't look right it's clipping leaves into the floor". Between the two fixes offered, the owner chose ground contact (option A) over tuning the palm's rows.

fn-283 measured the clipping and showed that its own changes did not cause it. The base, 147dae260, clips the same way to within 1 mm. The figures, from `.flow/evidence/fn-283-a-frond-draws-its-rachis/raw/clip-*.tsv`:

| Palm age | Leaflets with a vertex below y = 0 | Of them, on the dead skirt | Lowest leaflet vertex |
|---|---|---|---|
| 8 | 138–145 | | |
| 12 | 1,547–1,560 | 939–948 | −5.4 m |
| 20 | 302–316 | | |
| 40 | 0 | | |

The cause is the young palm's shape. It wears fronds close to full length (7 m, `frondEstablishment` 6.75 years) on a stem still only 3.5 m tall. Those fronds leave the stem 27.5° below horizontal and arch further down (`rachisArch` −0.6), so a frond falls about 0.99 m for every metre of its length. The dead skirt hangs at 165° and falls about 6.6 m.

Real young palms rest their lower fronds and their dead fronds on the ground. So does any tree whose limbs reach the ground. The fix is a property of the pipeline, not of the palm: nothing a tree draws passes below its ground. Contact is dormant wherever nothing reaches the ground. It is never a species branch, and presets stay value tables.

The film grows the palm through ages 8 to 20, so this spec joins the release (fn-237). It also blocks fn-283's R5 verdict on the young palm.

## Checked before the plan (fn-301 conductor, 2026-10-21)

- **The ground** is the plane through the root, y = 0. Nothing moves it. The CPU and the device each hard-code 0 (`tree.rs:63-77`, `scene/room.rs:23-26`).
- **Wood already lands.** The lay walk eases a descending limb toward the ground and clamps any step below it (CPU `telperion-space/src/geometry/lay.rs:357-379`, device `cycle/lay_walk.wgsl:206-223`). The clamp breaks this spec twice:
  - It shortens the step.
  - It sets the centreline, not the bark, at 0.
  - Trunks are exempt.
- **No single stage positions everything a tree draws.** Three things make positions:
  - Engine axes come from the lay walk.
  - Frond axes come from one parabola, `point + heading·L·t + lift·arch·L·t²`. On the CPU that is `foliage/leaflet.rs:26` `rachis`, which both `rachis.rs` and the leaflets call. The device has three copies: `generation/pack.wgsl` `on_rachis`, `resident/fronds.wgsl` `rachises` and `resident/field.wgsl`. The dead skirt is the rosette's dead fronds (`skirtFronds`, `skirtPitch` 165), drawn through the same curve.
  - Leaf and flower instances get their matrices in `station::matrix`, `pack.wgsl` and the bloom matrix.
- **R1 on the base** (`tests/ground_reach.rs`, 24704052c; `r1-base.tsv`): the palm is not the only preset that reaches the ground.

  | Preset | Reaches the ground | Lowest leaf |
  |---|---|---|
  | date palm | ages 8–20, deeply | −5.39 m at 12 |
  | silver birch | most rows | −0.09 m |
  | Norway spruce | age 40 | −0.025 m |
  | ordinary | age 40, seed 42 | −0.124 m |
  | Oregon white oak | age 40, seed 42 | −0.062 m |
  | beech, Telperion, Laurelin | clear on every row | – |

## Design (host, 2026-10-21)

Contact is a rule of the curve each axis is drawn on, plus one pose rule for instances. On each executor it runs where those positions are already made, never as a second pass over the finished tree.

- **Candidates weighed:**
  - **(b)** One contact pass over the drawn records, with the leaflets made to read the hung rachis nodes. This is the cleanest end state, because it deletes the three WGSL copies of the frond curve. It also restructures how leaflets are placed, which is too large for the release. It is a post-release Open item.
  - **(c)** Engine lay contact alone. This leaves the fronds, which are the defect.
  - **Chosen:** (a) together with (c).
- **The frond curve is one function per executor.** The CPU already has one. On the device, the three copies fold into one WGSL function that all three call. If the modules cannot share it, the plan says why and names the copies a test holds equal. Contact lives inside that function, so the rachis, its leaflets' stations and run, and the skirt all agree.
- **Resting keeps length and rests the bark.** On both curves, a resting axis keeps its arc length, and its centreline lies one radius above the ground.

- **Rulings after plan review round 1 (host, 2026-10-21):**
  - **Bark clearance.** The engine rests the girth. In the lay walk on both executors, the centreline of resting wood lies one girth radius above the ground. The engine is never handed surface rows.
  - **What the surface draws past the girth.** The flare and the lobes are the surface stage's own shape, so the surface stage keeps them off the ground. Where the sweep emits ring vertices, on each executor, a vertex of non-trunk wood below the ground is set onto it (y = max(y, 0)). That is the underside of wood pressed into the ground. The rule is an identity wherever no ring reaches the ground, so trees R1 finds clear keep their bytes. The trunk's buried root flare is unchanged. This is surface clearance, not a third way of resting: axes still rest only in their curves.
  - **Thinned leaves.** The renderer's thinning enlargement (`shaders/foliage.wgsl`, kept leaves scaled 1/kept about their seat) is a view-dependent draw. R4 judges the built tree, and the enlargement is outside it, so the renderer gets no pose. R5 adds one far-view still of the palm at 12, where thinning is active, to measure how visible the dip is. If it shows, the owner rules on it as an Open item.
  - **Quantisation, flower stalks, field boxes, proof-point wording.** These are the conductor's fixes as proposed. The pose clears by the encoder's worst-case error for the element's reach, with a packing round-trip test. Flowers use the stalk reach. The field's leaflet boxes take the posed frame. Task .2 gates .4 and .5.

- **Rulings on task .4's findings (host, 2026-10-21):**
  - **Element bounds are a wedge, not the box.** The axis-aligned box carries a leaf's full droop below its attachment plane right at the base edge, where turning about the attachment cannot lift it, so only an upright leaf clears it. The bound each element hands the pose is still eight corners, but the bottom face is tilted about the base edge. Its depth at the base edge is how far the element's vertices at the base row fall below the attachment plane (floored at 0). Its slope is the smallest that still bounds every other vertex. Both are closed forms, computed once per element where the element is prepared, never per instance. A leaf then lies nearly flat with its tip slightly raised. This is option (a)'s intent at option (c)'s cost.
  - **The solve (confirmed).** Take the up direction nearest world up for which every corner stays at or above the ground, measured from the attachment height (floored at 0). Turn the instance about a horizontal axis through the attachment to match. It is closed form, in the same order on both executors, and one 8-corner function per executor serves leaves and flowers alike.
  - **No clearing rotation.** This is not an error, and no refusals stand. The defined best effort is the rotation that maximises the lowest corner. Its residual is bounded by the element's base depth times its scale, minus the attachment height. R4's test reports the largest residual per row, and the report brings it up. It is not hidden in the tolerance.
  - **Flowers.** An inflorescence is posed as one about its seat, with the box of its point set as its 8 corners. The exact solve over 649 points is not used. No shipped bloom comes near the ground, so the slightly larger turn is accepted.
  - **Quantisation.** The pose clears by a margin derived from the encoder's formats. The margin is position rounding at the stored box's scale, plus the half-float scale's relative error times the reach, plus the 10-bit angle error times the reach. The packing round-trip test proves it. The test's own allowance stays `BELOW`, and the margin is never folded into it.
  - **Touches.** `pipeline/drawn.rs` passes the element's bounds instead of its blade length. `pipeline/foliage.rs` gets the module line. The shared device pose function sits next to `generation/rachis.wgsl`. All three are approved.

- **Ruling on the stalk's depth (host, 2026-10-21):** the wedge equals the box, because every shipped leaf's base row is its stalk's round cross-section around the attachment. That stalk is the leaf's seat on the wood, the counterpart of the bark the wood rests on. A leaf is never stood upright to lift its own stalk.
  - The pose solves against the ground lowered to the seat's depth: `v·d ≥ −max(h, b·scale)`, where `b` is the element's base depth. Lying flat then always qualifies, and the least turn is taken from there.
  - A leaf's dip below the ground is bounded by `b·scale − h`. That covers only its stalk, at most 3 mm at size 1.
  - R4 changes accordingly. Nothing lies below the ground except a leaf stalk's dip within that bound. `ground_reach` asserts the bound per vertex and reports the largest dip per row.
  - The best-effort branch stays as ruled for flowers.

## Requirements

- **R1 (repro):** a test reads, for every shipped preset at ages 6, 8, 10, 12, 15, 20 and its default age, at seeds 1, 7 and 42:
  - the lowest drawn vertex of wood, of leaves and of flowers;
  - the count of instances with any vertex below the ground.

  The test is red on the base for the palm. Record which other presets reach the ground, if any.
- **R2 (contact):**
  - **An axis.** An axis whose bark would pass below the ground meets it and lies along it from that point on, with its centreline one radius above the ground. It keeps its length: on the frond's parabola this is the closed-form arc length, and in the lay walk the step keeps its length. It carries on in the horizontal direction it was travelling. Where that direction is vertical, the host's ruling of 2026-10-21 replaces "it takes its heading's horizontal direction". On the frond's curve, the tangent at contact is `heading + lift·2·arch·t`, and when its horizontal part is zero the frond rests along the horizontal part of its frame's `lift`: +lift when arch is 0 and the heading points straight down, sign(arch)·lift otherwise. In the lay walk, a step heading straight down at the ground rests along the phytomer's `side`, which is horizontal for a vertical step. Neither case is refused; both executors use the same rule. Its children are attached to the axis as it now lies. Trunks stay exempt, as they are today.
  - **Engine wood.** The lay walk's clamp becomes this rule. The landing ease is measured from the bark, `support(tip.z − radius, …)`, on both executors (host ruling, 2026-10-21; this replaces "the landing ease is unchanged"), so a limb still lands without a corner one radius up. Decision 4's 0.8 rad bound stays. R3's byte-identity on clear trees is measured through the digests, never inferred; if a clear tree changes, the run stops and reports it.
  - **Fronds.** The frond curve function, one per executor, applies the rule. On the ground the centreline sits at r(t), and each step keeps its 3D path length, so its horizontal advance is sqrt(max(0, ds² − dr²)). Where the taper drops faster than the step, the step goes straight down onto its own bark, which stays at or above the ground. No frond is refused and no point is non-finite (host ruling, 2026-10-21).
  - **Leaves and flowers.** Leaves and flowers borne on a resting axis rest with it. The instance pose is one function per executor, applied where each leaf and flower matrix is finished. An instance whose element bounds would cross the ground under its matrix is turned about its attachment point by the least rotation that brings those bounds onto the ground. Lying flat always qualifies, since the attachment sits on bark above the ground. It is never culled, never scaled and never translated.
  - **The dead skirt.** The skirt is the rosette's dead fronds, so it rests through the frond curve.
  - **Both executors.** The CPU and the device apply contact in the same stage, through the one pipeline.
- **R3 (dormant, and what changes):**
  - Trees whose wood never enters the landing ease's zone (and never touches the ground) build byte-identical meshes; trees with low wood in the zone ease from the bark and change (host, 2026-10-21; measured in .5: the ordinary tree and the oak at 40, seed 7, and the spruce at 20, seed 42, are clear by their bark on the base and return to the base's bytes with the ease alone put back on the centreline). That covers the beech, Telperion and Laurelin on every row, as measured (host ruling, 2026-10-21: the palm at 40 moved to the changing rows).
  - The rows R1 finds touching change, under AGENTS.md "Generator evolution" (host, 2026-10-21): the palm at 6–20, the birch, the spruce at 40, the ordinary tree and the oak at 40, seed 42. Real lower limbs and skirts rest on the ground. The palm at 40 changes too, by one vertex per seed of at most 2.4 mm: the corner of its lowest leaf-base cell, which sat 1.7 to 2.4 mm below the ground on the base and which the surface clearance sets onto it. Its neighbouring normals shift by at most 344/32767. R1's base probe listed it clear only because it read the wood from the curve (host, 2026-10-21).
  - The cost is measured with a paired `build_time` run. It stays within noise on trees that never touch the ground. On the palm at 12 and the spruce at 40 it is recorded.
- **R4 (agreement):**
  - The CPU and the device agree on the palm at 12, to `agree_tests`' tolerances.
  - On every R1 row, no vertex lies below the ground.
- **R5 (look):**
  - Stills of the palm at 8, 12, 15 and 20 at seed 1, base against change, viewed by the agent and then judged by the owner.
  - The spruce at 40 and the birch at 40, seed 1, base against change, at the default view.
  - One still from the walk that passes through a young palm, if the film has one.
  - One far-view still of the palm at 12, seed 1, where thinning is active, to measure the dip of enlarged leaves (host ruling, outside R4).
- **R6:** the gate and `npm test` are green, with a codex review on each task. `tests/ground_reach.rs` is un-ignored and green. `docs/pipeline.md` line 25, which still says four families have no engine species, is corrected.

## Boundaries

- Not terrain, slopes or the ground's material.
- Not collision between branches.
- Not tuning the palm's frond pose. The owner chose contact over that.
- Not (b), the leaflets reading the hung rachis. That is a post-release Open item, which would delete the frond curve's device copies.


## Edge cases and constraints (plan, 2026-10-21)

- **Dormant means untouched floats.** Contact and pose are gated by a predicate evaluated before any new arithmetic. A curve whose bark never reaches the ground, and an instance whose bounds never cross it, take exactly today's operations in today's order. That is what R3's byte-identity rests on, on both executors.
- **The frond's contact point.** First contact is the least `t` in [0, 1] where the centreline's height equals the rachis radius there, `r0·(1 − t)` floored at the finest wood. On the palm's descending arch it is the one root; with an upward arch, the first root wins and the axis stays down from there. A frond whose base already sits at or below its radius rests from `t = 0`. Past contact the curve runs flat at its radius in the horizontal direction of its tangent at contact (the frame's `lift` per the R2 ruling when that tangent is vertical), and its parameter maps to the parabola's remaining closed-form arc length. Every frond whose path meets the ground has a defined resting direction, so none is refused (host, 2026-10-21).
- **The frond's frame past contact.** The function returns the point and the tangent. The leaflets' run and axis read the returned tangent, so they lie with the rachis.
- **Stations.** The CPU's rachis and leaflets sample `t` from `station()`. The device's `rachises` samples `k / count · split`. Task 1 checks whether the two are the same values. If they differ, the shared device function takes `station()`'s form, and the difference is recorded.
- **Engine lay.** A non-trunk step whose girth would pass below the ground ends at height `radius`, the phytomer's girth radius (host ruling). It keeps the step's length along the horizontal part of its travel (the heading's horizontal part when the step is vertical). The landing ease and the trunk fault are unchanged. Sag reads tip heights, so trees whose wood rests change through the sag pass too. That is part of R3's "rows that change".
- **Surface clearance.** Where the sweep emits ring vertices, on each executor (the CPU's sweep and the device's surfacing `emit`), a vertex of non-trunk wood gets `y = max(y, 0)`. That keeps the flare and the lobes off the ground. It is an identity where no ring reaches the ground. The trunk's buried root flare is unchanged. R1's wood measure reads the drawn vertices, so it sees this clearance.
- **Establishment.** The palm's stem is lowered after the lay (`establish`). Every frond is hung from the lowered stem, so the frond curve's contact sees the final heights. The palm has no engine laterals for the lowering to carry below the ground. R1's rows hold this.
- **Instance pose.** The pose turns about the attachment point, so the stored translation, and with it the quantisation box's containment, is unchanged. The element's local box comes from its positions. The device needs it in the config it already uploads.
- **Quantisation.** Rotations are stored as 10-bit quaternion components (`packed.rs`), so a decoded vertex can stand off its exact pose by an angle the encoder bounds, times its distance from the attachment. No stored pose can do better: a leaf whose attachment sits on a terminal rachis at the finest wood (0.2 mm) cannot be held clear of a 1.7 mm encoder error at its tip. So the pose solves on the exact matrix (its lowest vertex onto the ground), and R1's leaf and flower measure admits, per vertex, the encoder's worst-case angle times that vertex's distance from its attachment, plus `BELOW` (conductor, from the stored format; accepted, host, 2026-10-21). A packing round-trip test checks a rested leaf, attached at the finest wood, against that bound.
- **Flowers (host ruling D, 2026-10-21).** An inflorescence's florets and pedicels are posed as one about its seat. The point set the pose lifts is its axis's samples, its florets' element corners and its pedicels' ends. `Element::stalk`'s scalar reach screens first, so an inflorescence whose seat stands higher than its reach takes no new arithmetic. The rachis the renderer rebuilds in `stalk.wgsl` (droop toward world −Y, unchanged) gets the sweep's surface clearance, `y = max(y, 0)` on its vertices, in tree space, before the view projection. That is an identity wherever a stalk is clear, so Telperion and Laurelin keep their bytes and the stored format is unchanged. R1 and R4 measure the stalk from the CPU's mirror of the rebuilt rachis (`bloom.rs` `Axis::laid`, the same curve as `rachis_point`), the stalk's radius below its centreline. Rejected: (A) refusing low flowers, which makes a parameter a switch; (B) a new encoding for every flowering tree; (C) florets alone, which fails R4. Where an inflorescence rests, its posed florets can stand off the clamped rachis. No shipped preset's flowers come within 0.44 m of the ground (R1 on the base).
- **The field's leaflet boxes.** The CPU leaf plan and the device field build leaflet boxes from the drawn frame. Both take the posed frame.
- **"Below" admits 1e-4 m.** A leaf transform is quantised against the crown's box, so a vertex laid on the ground can read a hair under it. R1's test states this bound (`BELOW`, this spec).

## Quick commands

```bash
# R1 and R4: nothing drawn below the ground, every preset, ages and seeds
telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-core --test ground_reach -- --nocapture
# R4: the CPU and the device agree on the palm at 12
telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render agree
# R3: a preset's every artifact, digested
cargo run --profile ci -p telperion-core --example artifact_digest -- european-beech 1 | sha256sum
```

## Early proof point

Task fn-301-a-tree-rests-on-its-ground.2 validates the core approach: contact inside the one frond curve rests the palm at 12 on both executors, and they still agree. If it fails, re-evaluate contact-in-the-curve, against the post-release (b), before continuing with tasks .4 and .5, which depend on it. Task .3, the engine's lay and the surface clearance, does not depend on it and runs beside .1.

## Strategy Alignment

Active tracks served by this plan:
- **The tree space**: contact is a property of the pipeline. It is dormant where nothing reaches the ground and wakes by degree, with no species branch and no switch.
- **The core and integration**: one rule per executor, applied where positions are already made. The device's frond curve folds into one function.

## Memory findings

| Track | Category | Entry | Why relevant |
|---|---|---|---|
| bug | runtime-errors | rows-hoisted-before-the-expansion-read-2026-10-20 | Device frond rows must be read from `Family::applied`, not the raw family. The shared curve takes its rows from the same table. |
| bug | integration | wood-hung-after-the-solve-bore-cauline-2026-10-19 | Frond axes are hung wood. A resting axis must not change what bears leaves. |

## Requirement coverage

| Req | Task(s) |
| --- | --- |
| R1 | .5 (the repro landed at 24704052c) |
| R2 | .1, .2, .3, .4 |
| R3 | .3, .5 |
| R4 | .2, .4, .5 |
| R5 | .5 |
| R6 | .5, and a codex review on every task |

Waves: 1 = .1, .3 · 2 = .2 · 3 = .4 (after .2 and .3, which share `ground_reach.rs`) · 4 = .5.

## Decision Context

Maintainability (plan review): duplication - none identified; structure - Task .3 adds horizontal-direction and named-stop branches to geometry::lay::Layer::step.

## Open

- Sag's ground support still measures from the centreline (`sag.rs:258`, `sagged.rs:71`), where the landing ease now measures from the bark. It is outside this spec unless a test fails (host, 2026-10-21).
- (b) from the Design section: the leaflets read the hung rachis nodes, which would delete the frond curve's device copies. Post-release.
