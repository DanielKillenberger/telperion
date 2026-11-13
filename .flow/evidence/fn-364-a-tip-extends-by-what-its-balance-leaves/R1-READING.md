# fn-364 R1: every reader of a phytomer's length, read before the change

Builder (Opus), 13 Nov 2026, on `fn-364-a-tip-extends-by-what-its-balance-leaves` at `49ec1f379` (fn-363's product and evidence). This is a reading of the code; the record sizes at the end were measured on this commit with a scratch test that was not kept.

"Reads position" means the reader takes a length from laid tips (`tip`, `base`), so it follows the shortened internode without a change. "Reads scale times internode" means it computes a length from `scale` and must read the stretch through the one function after this.

## Where the stretch enters

- `grow/cycle/passes.rs`, `allot_one`: computes `size` for every living apex. The share s is computed here, after `sizes`, from the same light (`Pencil::light`) and the same height (`passes::charged`, the tip's last laid node above `axes[0].base`) that `balance_one` read. Nothing is laid between the balance and the allotment (`Grower::step`: open, retire, balance, allot, then growth), so the height and light are those the charge read.
- `grow/advance.rs` and `grow/stop.rs` push each unit's size into `Draws::shade` through `Reads::size`. The stretch goes into the same record (see "Bytes").
- **Whole-tree test engine (`sketch.rs`, `allot`): no test holds it to the one cycle.** The tests that run it (`grow/tests.rs`: the speed levers, `many_cores_grow_the_tree_one_does`, `light_no_bud_reads_is_not_worked`, `every_pass_is_reported`, `without_what_later_years_add_the_rough_layout_is_the_final_one`; `grow/growing_tests.rs`'s `a_growing_phytomer_is_read_as_it_was` through `grow::sketch`) compare it with itself. `grow/cycle/tests.rs`'s `reference()` is the one cycle (`Paused` with `Working::reference()`), which runs `allot_one`. So `allot` does not take the rule, and its pencils keep a stretch of 1.

## Readers in the space crate

| reader | what it reads | mark | at a zero-length internode |
|---|---|---|---|
| `grow/draw.rs`, `draw_bent` (growth's lay) | lays each phytomer at `scale × size` (in f32) and sums the pull's `total` from the same | scale times internode | `Layer::step` lays a zero step: the tip stays where it was, `run` does not advance; nothing divides by it. The probe grew this without error. |
| `geometry/lay.rs`, `Layer::step` (growth's and the settle's lay) | `phytomer.scale()` as the internode's length, the tropism's and wander's lever, the bend memory's relaxation and `run` | scale times internode | the same: no step, no turn (every turn is in proportion to the length), the pull's fade reads `run` unchanged. |
| `geometry/sagged.rs`, `sag_angle` | the `length` `step` hands it | follows `step` | a zero lever bends nothing. |
| `sag.rs`, the levers | `length(tip − base)` and the mass `(r² + leaves²) × length` | reads position | zero length, zero mass for that span. |
| `geometry.rs`, `reaches` (each axis's straightening fade length) | `Σ p.scale` | scale times internode | a shorter reach; a whole axis of zero length has reach 0, and `step`'s `l.total > 0` guard makes `along = 1`, no pull. |
| `geometry/stand.rs`, `length_of`, `length_to`, `node_x`, `zone_node`, `span_in` (where a lateral or relay stands along its bearer) | `scale()` as each span's length | scale times internode | already guarded: `scale > 0` before any division, and a unit of zero total falls back to counting nodes by share. |
| `geometry/stand.rs` on `Growing` axes (growth's own placement of laterals) | `Growing::scale`, the bare scale without the unit's size | scale times internode, without size | unchanged by this spec: the growing phytomer holds no size in its scale between steps today, so it holds no stretch either. Positions inside a zero-length unit coincide, so where it stands there is the same point. |
| `geometry.rs`, `attach`, `lateral` | through `stand.rs` | follows stand | as stand. |
| `unfold.rs`, `wood` (how far this year's shoot has unfolded) | `p.scale × internode_factor` | scale times internode | `piece == 0` takes the existing branch: unfolded once the reach has got there (`reach >= along && tau > 0`). |
| `unfold.rs`, `blend`, `(p.tip, p.scale) = (…, p.scale * r)` | mixes scale and tips between years | scale (a presence), positions | the stretch is a unit's, the same in both years, so it is not mixed. |
| `girth.rs` (established section, leaf), `light::leaf_area`, `grow/deposit.rs`, `floor.rs` | `scale` and `size` as leaf and girth | not a length | unchanged: leaf, deposit and sapwood demand keep the allotment's size. Deposits sit at the laid tips (positions). |
| `shed.rs` (weights), `presence.rs` | `scale` as a presence | not a length | unchanged. |
| `closed_form/*` | the species' rows | not the grown tree | unchanged. |
| `fine/canonical.rs`, `digest` | `tip`, `heading`, `side`, `radius`, `scale`, `rank`, `size`, `key` | the tree's identity | the stretch shows in the tips. It is left out of the digest as `grown`, `section` and `sapwood` are, so a tree with every stretch 1 keeps its digest. |
| `invariants.rs` | `scale >= 0`, `size >= 0` | checks | unchanged. |

## Readers in the pipeline (telperion-core)

| reader | what it reads | mark | at a zero-length internode |
|---|---|---|---|
| `pipeline/space.rs:373`, the conversion's `piece` | `internode * p.scale * internode_factor` | scale times internode | `pending += 0`; a piece whose wood since the last drawn node is under `SHORTEST` (1,000 µm) is not drawn: it merges into the next node (`continue`), its relays join the node before, and `lengths` carries its wood on. **A run of zero-length internodes is dropped, never drawn as coincident rings.** The surface stage never sees them. |
| `pipeline/space/join.rs`, `Draw::of`, `draw` | the piece, and the parent's tips for the join's place | follows the piece; position | a join inside a zero-length span has piece 0 and stands at the span's one point. |
| `pipeline/space/establish.rs` | `p.tip.z` | reads position | unchanged. |
| `Tree::grown` / `grown_length` (from `lengths`, the piece) | each drawn node's grown wood | follows the piece | zero where the wood is zero. |
| **`pipeline/foliage/plan.rs`, `walk` and the stations** | `tree.grown_length` in whole micrometres: **a run's leaf stations are counted from its grown wood** under a twig layer (`supports`: the oak, `twigs/twig/internodeLength = 0.008`) | follows the piece | **wood of zero length carries no station, so no drawn leaf.** See the open question below. |
| `tree/leafing.rs`, `suite/retention.rs`, `blend/size.rs`, `measures.rs` | retention by cycle, counts | not a length | unchanged. |

**The render crate reads no phytomer.** It has no `telperion_space` import; it draws the core tree's nodes (positions, radii) and the foliage's leaves.

## Open question for the host (scaling leaf; a Stop)

The design keeps a unit's leaf area at the allotment's size, and it asks the pipeline's piece to read length through the one function. Under a twig layer the drawn leaves are counted from that piece (`foliage/plan.rs`, `walk`, `station_count`). Read together, the settled tree keeps full leaf on short internodes, while the drawing places leaf stations in proportion to the shortened wood, and none on zero-length wood. Whether the drawing's stations should read length or the leaf's scale is a question about scaling leaf, so I build the piece to read length as the spec says and bring the measured effect to the host before any sheet is drawn (R5, MEASURES.md).

**The host's answer (13 Nov 2026):** stations stand for nodes. They are counted from the wood's leaf-bearing length (the piece without the stretch, `Tree::bearing`) and placed on the stretched run; a merged span hands its leaf-bearing length to the node that draws it, the axis's last drawn node where none follows. Built with two tests in `pipeline/space/stretch_tests.rs`; R5 measures the drawn stations beside the settled leaf.

**Measured on the build (13 Nov 2026, `pipeline/space/stretch_tests.rs`, the oak's family at 12, seed 7, no potential height, so every stretch 1 before the hand edit):**

| the same structure | drawn nodes | leaf-bearing wood m | wood in leaf-bearing runs m | stations |
|---|---|---|---|---|
| laid whole | 16,366 | 143.03 | 12.71 | 1,794 |
| every phytomer at 0.5, its places halved | 15,958 | 143.03 | 12.93 | 1,784 (−0.6%), spacing 0.50 of the whole |
| every axis's last two phytomers (past its first) at 0 | 12,692 | 143.03 | 37.28 | **4,389 (×2.4)** |
| every third phytomer at 0 | 10,914 | 143.03 | 20.85 | **2,662 (×1.5)** |

The leaf-bearing wood is kept to the metre in every case, and the spacing halves. The station count does not hold where spans merge, because which wood bears is decided per drawn node (`foliage/plan.rs`, `bearing_of`: a node bears where it is a twig, a node with no child, or as slender as the shoot radius). On the oak only 12.7 of the 143 m bears: mostly each axis's terminal node. A span merged forward hands its wood to the node that draws it, and where that node bears and the span's own node would not have (an axis's last drawn node, now terminal, taking its zero-length tip units), the wood starts bearing. That is a question about which wood bears leaf under a stretch, the host's; the test holds only that the leaf-bearing wood is kept.

## Zero-length internodes, by stage

- Growth's and the settle's lay: a zero step, no turn, no fade advance; nothing divides by the length.
- Stand: guarded today; positions in a zero-length span coincide.
- Settle: `Error::Collapsed` only when every phytomer of the tree has scale 0. The stretch does not touch scale, so it cannot cause it.
- Pipeline conversion: merged into the next drawn node, not drawn (the probe's V3 drew a wood mesh of 176,093 runs from 514,905 such phytomers without error).
- Foliage: no stations on that wood (above).
- Whether a run of coincident rings would draw as a fault is moot: the conversion never hands one to the surface stage.

## The record, its bytes and its type

Measured on this commit (64-bit): `Phytomer` 136 B, `Growing` 72 B, `Pencil` 208 B, `Draws` 144 B, `Axis` 232 B.

- **Settled phytomer: `stretch: f32`, +0 B.** `Phytomer` holds 120 B of 8-byte fields and 12 B of 4-byte ones (`cycle`, `section`, `sapwood`), 132 B padded to 136. An `f32` fills the padding. An `f32` because the share is computed in `f32` (the charge is `f32`), and growth lays in `f32`; a length is read as `scale × f64::from(stretch)`, which for a stretch of 1 is `scale` to the bit.
- **Per growth unit: +0 B.** `Draws::shade` is a `Vec<f64>` of one unit's size, which growth already requires to be an `f32`'s image (`draw.rs`, `image`). It becomes a vector of one record per unit holding the size and the stretch as two `f32`, 8 B as before. A second vector would add a 24 B header to every `Draws`, which `an_axis_and_its_draws_stay_compact` pins at 144 B; that budget is not raised.
- **Per axis: +0 B in `Draws`; `Pencil` gains the cycle's share as one `f32`.** `Pencil` is not pinned by any test. Its size after the change is measured in R4.
- **Growing phytomer: +0 B.** The stretch is folded into the scale only for the moment of the step, as the size is today.
- **What sees it:** `an_axis_and_its_draws_stay_compact` (`Axis`, `Draws`), the footprint's `draws.shade` and `pencils` stores (`grow/cycle/footprint.rs`, counted by type size), and `tree_digest`, which hashes the `Debug` text of the tree: adding a field changes the text, so **every `tree_digest` pin moves, even on trees whose every stretch is 1** (`growing_tests`, `cycle_cost_tests`). `catalogue_identity` hashes the catalogue's text, not a tree, and no row changes. R4 lists each with whether the tree moved (`fine::digest`) or only its text.

## Tests that read length from scale

- `geometry/internodes_tests.rs:33` (`internode * p.scale` against the laid span), `telperion-core` `pipeline/space/leafing_tests.rs:50` and `pipeline/space/tests.rs:76` (the piece): they read scale times internode, so they move to the one function.
- `tests/form.rs:263-271` (a unit's middle by its scales), `tests/closed_form.rs` (sums of scale, against the closed form's expected wood): their fixtures set no potential height or are read as wood in the closed form's sense, so every stretch is 1; read below in R4 if one moves.
- `death.rs`, `dormant.rs`, `walks.rs`, `walk/measure.rs`, `lever.rs`, `walk/bend.rs`, `girth/tests.rs`: scale as a presence or as leaf, not a length.

## The share's edge

The spec gives s = max(0, 1 − (h · inverse_height)² / light), with "light 0 gives 0; no potential height gives 1", and R2 asks that at h = 0 the tree be fn-363's to the bit. These meet at a tip of no charge (no potential height, at or below the collar) whose light is 0. I take s = 1 wherever the charge is 0, then 0 where the light is 0, then the formula, so a tip charged nothing is laid as before whatever its light. The probe's V8 tested the potential height first and the light second, which differs only at a tip at or below the collar in no light.
