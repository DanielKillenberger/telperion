# fn-256 task 2: the settle on the device, its plan (worker's planning agent, 2026-10-11)

This plan is read from the code; nothing in it has run. It refines DESIGN.md section 2.4. Line numbers are as of commit 1f1174d20.

## The two index spaces

The settle runs in two index spaces:

- **Phase G** runs in the grown index space, which is the CPU's index before shedding (DESIGN.md section 0). Its passes are mark, assign, attachments, fades and shed, disused, and the lineage order.
- **Phase S** runs in a compacted space that is already in final lineage order. Its passes are scale, thicken, place, levers and the sag place.

The order can be fixed before the geometry because nothing in `lineage_order` depends on geometry. `identity` (canonical.rs:24) reads only parent, node, slot, woken and pa, and shedding never changes them. So the final order is fixed straight after shed, and the tree is compacted once.

**Notation.** `A_*` are axis columns (arena.wgsl:63-92, layout.wgsl:54-73). `U_*` are unit records (arena.wgsl:106-115). `PH_*` are phytomer columns (layout.wgsl:76-87). Phytomers are reached through the unit lists (`prow`, lay_state.wgsl:160).

## Phase G, in the CPU's order

### G0. Phytomer index (new)
`phyt_start[a]` is an exclusive scan of `A_PHYTS`. One invocation per axis walks its units and writes `row_of[phyt_start[a]+j]`. This makes `parent.phytomers[node]` O(1).

### G1. mark (settle.rs:16-33)
- **Living apexes:** one invocation per living-list entry computes `Grower::living` (grow.rs:522) in f32.
- **Woken buds:** one invocation per woken record computes `Woken::kept` (dormant.rs:329) and writes `A_B1 = real(q(b1)·q(kept))`.
- The two sets are independent: successors are never woken buds.
- **To rewrite:** carry.wgsl `living_q`, `woken_kept_q` and `successors_q`, moved onto the arena columns.

### G2. assign (presence.rs:246-344)
- **Relay nodes:** `relay_node`, as lay_stand.wgsl:303 has it, fused into the pass.
- **`assign_one`, per axis over its units:**
  - `running *= q(survive)`;
  - the unit's presence goes to `U_PRES`;
  - PH_SCALE and PH_GROWN are overwritten with the settled values;
  - PH_SIZE is the narrowed `shade[0]`;
  - then the persist, the end, `alive`, and the unit count.
- **Rank** is an f64 accumulation (presence.rs:334). Each product is exact, but the sum rounds, so it takes `f64_add`. It is deferred to S0, kept axes only.
- **Vigour** (presence.rs:284-293) is a second per-axis dispatch, because it reads the parent's end.
- **To rewrite:** assign.wgsl `units_q` and `assign_q`. The chain's asks and failure fixups are dropped.

### G3. Attachments, only where `keeps` (settle.rs:58-64; disused.rs:184-271)
- **Bases** (`bases_past`): top-down, in f32.
- **Leaf partials** (disused.rs:142-168), only where some `leaf_girth > 0`: two whole-tree sequential f32 sums, in axis order then phytomer order (risk 1).
- **Radii** (`attachments_with`): bottom-up.
  - The CPU takes children last first, by index descending.
  - The hierarchy's CSR, read in reverse, gives that order within each node bucket. Each bucket's sum depends only on its own order, because attachments never split relays, so the CSR is reused as is.
- **To rewrite:** girth.wgsl `section_q`, `radius_q` and `weighed`, with `1/e` from a host table per PA.

### G4. shed_past (shed.rs:100-140; `fades` 146-241)
- **Fades up:** bottom-up maxima, which are order-free.
  - `lived = q(sleep) + Σ unit presences`, with each unit's prefix kept in `U_PRES_SUM`.
  - Each child carries `carry(before, weight, gap, lived_c)`.
- **The fade** per axis: `faded(q(d+1), birth, lived, alive, age)`.
- **Keep down:** top-down. `kept = kept[parent] && (seed/continuation || fade > 0)`.
- **`vigour *= fade`** is exact in f64 (24×24 bits), so vigour and fade are kept as two f32s.
- Shed keeps relative order, so the grown index is the order key and no renumbering map is needed.
- **Fused into the keep-down sweep:** disused's `above` and the lineage identity.
- **To rewrite:** fades.wgsl `up` and `down`; carry.wgsl `before_q` and `prefix_q`.

### G5. disused (disused.rs:285-323)
Per grown axis, independent: the radius and share go to two axis columns. The CPU's order per bearer is ascending grown index, and S2 replays it.

### G6. Counts
- **Collapsed check 1:** kept phytomers, an order-free count.
- **One readback:** kept axes, kept phytomers and kept depth, which size phase S.

### G7. lineage_order (canonical.rs:167-231)
- The BFS from the seed orders by (depth, parent's final index, key).
- **Depth starts:** a histogram of kept depths, then a scan.
- **Per depth:**
  1. Gather the previous depth's parents' kept-child counts, already contiguous in final order.
  2. Scan them.
  3. Place each child by its rank among its kept siblings in the parent's CSR run: O(siblings), at most 234.
- **Equal keys** raise a fault, as the CPU's `distinct` assertion does.

## Phase S, in final order

### S0. Compaction
- **Phytomer starts:** a scan over the final axes.
- **Axes:** gathered with their parents renumbered, plus a `grown_of[f]` back-map.
- **Phytomers:** copied through `row_of`; rank is taken in soft f64.
- **Units:** the unit presences.
- **`kid_by_index`:** each parent's child run ordered by grown index descending, the CPU `Tree`'s order.

### S1. scale (geometry.rs:52-103)
Top-down by depth, in f32: `base[i] = (node scale × base[p]) × vigour × dominance`, then each phytomer's `scale·base·size`.

### S2. thicken (girth.rs:170-258), fused with `reaches` (geometry.rs:127-140)
- **Thicken:** bottom-up by depth.
  1. Disused sections, by walking the grown CSR forwards, ascending.
  2. Children in `kid_by_index` order. A relay is split by `joining` at `attached`; lay_stand.wgsl:267 has `attach`.
  3. A reverse walk gives the radii, and `Join` is written on the relay child.
- **Leaf mean:** G3's value. Where `keeps` is false, the `LeafMean` pass over the settled tree costs nothing.
- **Reaches:** `reach[p] = own Σscale + Σ (1-b)·reach[c]`, over the carried-on children, index descending.

### S3. place (place.rs:82-144)
Top-down by depth, one invocation per axis:
- `framed()`, factored out of lay_frame.wgsl's `frame`;
- `hand_lifts`;
- the bend from ticks rounded to f32 once;
- `lay` through lay_walk.wgsl's `step`;
- the Layer's end written to a table.

Errors go to an atomic min of the grown index, which equals the CPU's first error under `take_while`.

### S4. Sag levers, only where `sag::any` (sag.rs:87-271)
Bottom-up by depth. The far tip follows continuations. The loads go in `kid_by_index` order, relays split as in S2.

### S5. The second place
S3 again, with the load. `sag_turn`, `held` and `torque` are ported into lay_walk's `step`.

### S6. Collapsed check 2
An atomic OR of `scale != 0`.

### Arithmetic in phase S
There is no f64 in phase S beyond the rank and vigour words already formed. `1/e` comes from a host table. Everything else is f32 under q.wgsl's markers, so `Arith::Exact` holds it to the CPU's operations.

## Dispatches (Telperion: about 63 depths, 29 block levels)

- **Phase G sweeps** use the hierarchy's blocks, with new entry points in hier_sweep.wgsl's pattern.
- **Phase S sweeps** are one flat dispatch per depth, since BFS order makes each depth a contiguous range.
- **Total:** about 840 dispatches, roughly 10 ms of overhead. The heaviest is G7's BFS, at about 5 a depth.
- **Bindings:** the settle's columns are grouped into one `settle_axes` buffer and one `settle_phyts` buffer, under the 8 storage bindings a stage.
- **Reuse:** a `settle_state.wgsl` gives the same accessor names over the settled columns, so lay_stand, lay_frame and lay_walk compile unchanged.

## The node table

`Structure` (structure.rs:213) is laid out as columns in final order:

- **Axis rows:**
  - kind, parent (final index), node, slot, woken;
  - whorl (f64), lineage, pa, birth, apex_end;
  - vigour (f64), blend (f64), alive, rank (f64), sleep, phase;
  - base, heading and side;
  - the units' range;
  - joined;
  - the phytomers' range.
- **Phytomer rows:** cycle; tip, heading and side; radius, scale, key, rank (f64), size, light and grown.
- **Size:** Telperion keeps 3.8M phytomers at 21 words, about 320 MB.

**The CPU reference:** `grow_capture_settled(_at)` beside capture_grown.rs. It sets up the grower with `Working::reference`, then runs `settle`, then `lineage_order`. It also records the intermediates: `leaf_mean`, the disused list and the shed index.

**The comparison:** by index, every word.

## Risks

1. **The leaf mean** (DESIGN.md section 8, open item 1). It is a sequential f32 chain over up to 18.2M grown phytomers, only for presets with `leaf_girth > 0`.
   - Interim form: one workgroup stages tiles in workgroup memory and one lane sums them, the running pair carried across dispatches so each stays under 1 ms.
   - A blocked order is the host's call.
2. **Order traps:**
   - G3's bucket shortcut is valid only because attachments never split relays.
   - S2 and S4 must use `kid_by_index`.
   - Disused sections go before the children, ascending.
   - Reaches run index descending.
   - Rust's `.sum()` starts at -0.0 (`neg0`, q.wgsl:18).
   - G3's `own_at` and `terms` take the contract's `exp2` and `log2`.
3. **Long sequential axes:** the place walk and thicken are sequential per axis. The longest axis bounds its depth's dispatch, which may pass 2 ms. Measure it on Telperion 82.
4. **Memory:** the settle adds about 1 GB beside the 3.7 GB of state and scratch. Free or reuse the light scratch and the pencils (433 MB) before S0.
5. **Retirement:** the device keeps every axis, and the CPU reference is `Working::reference`. One test asserts that `grow_cycle` equals the reference.

## Steps (each tested exact on young trees of all eight presets)

1. **Shed and order** (G0-G7, then S0 without geometry):
   - compare the topology, vigour, alive, units, rank, relay nodes, grown, size, light, key and cycle, plus `leaf_mean` and disused;
   - include the Collapsed case;
   - on the CPU, split `Grower::settle` after the shed (settle.rs:86) into `settle_shed` and `settle_lay`, which `settle` calls in turn.
2. **Scale and thicken** (S1-S2): the reference is `settle` with `Options { settle: true, .. }`, which returns before `place`.
3. **Place and sag** (S3-S6):
   - the full node table against `run(.., Working::reference())`;
   - error parity;
   - determinism over 10 contexts and caps 64, 128 and 1,024;
   - timing on Telperion 82 and the oak at 80.
