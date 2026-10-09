# Where the trunks' dents come from (fn-244 R1)

Base `1f9afd53` (F4 with fn-243). Stills from `space_<species>` (`--shots base,limb,bare`, ages 40 and 80, seeds 1, 7, 42) and `space_two_trees` (40 and 80, seed 1), all viewed as contact sheets in `raw/sheets/`.

## What the stills show

- **European beech: collars.** Every 80-year beech and most 40-year ones have rings along the bole where the radius steps in by 15 to 25 % and back out above. Worst cases: `beech-80-7-base`, `beech-80-42-base`.
- **Ordinary: waists.** In `ordinary-40-42-base` and `ordinary-80-42-base` the stem pinches to a quarter of its radius at one node. `ordinary-80-1-base` has a terrace near the ground.
- **Oak: hairline rings.** In `oak-40-1-base` and `oak-40-7-base` there are faint seams above each lateral. Each is a 1 to 4 % step.
- **Birch, palm and spruce: no collars on the bole.** The ledge in `spruce-80-7-base` is the flare's top edge among the whorl branches. The trunk radii there fall by no more than 1 % per node.

## The stage and the rows

The dents are already in the converted tree's node radii, before any surface stage. A probe printed each trunk node's height and radius from the root up. For beech 80-42 the radius runs 0.628 m, then 0.554 m from 0.14 to 0.42 m, then 0.628 m again, then 0.483 m at 2.9 to 3.0 m, then 0.628 m. For ordinary 40-42 it runs 0.136 m, then 0.043 m at 1.35 m, then 0.134 m.

The cause is in `crates/telperion-core/src/pipeline/space.rs`, `converted`, in the `Origin::Relay` branch. The girth model and the drawing put a relay in two different places:

- **Girth model.** `telperion-space` `girth::thicken`, `relay_parts`, enters the relay's pipe at its bud's node by `t` and at the node below by `1 - t`, where `t = relay_span(axis, relay_at, node)`. When the relay stands early in that span (small `t`), the bud's node carries almost none of it. Beech at `relay_at: 0.15` (`beech.rs:106,199`) always does. Ordinary often does.
- **Drawing.** The drawing hangs the relay from `joint`, at or above the bud. It patches the stem from `bud + 1` to `joint` up to `girth = radius at the bud's node`, and that radius lacks the relay's pipe. The stem between the bud and the joint is therefore drawn without the relay's wood, and the relay leaves it at full girth: a constriction, then a collar.
- **The tip-relay case.** A relay whose joint is its parent's last node goes through the `Continuation | Relay` arm instead and hangs from the parent's end node. That node has the same deficit; it makes the ordinary-80-1 terrace at 0.25 m.

Probe output (beech 80-42, first trunk relay), with `bud 1 joint 5`:

| Source | Radii from the base up (m) |
|---|---|
| Parent phytomers | 0.6284, 0.5535, 0.5320, ... |
| Relay's own | 0.6284 |

The pipe went in at node 0. Node 1, the `girth`, did not get it.

## Each candidate switched off in turn (beech 80-42 and ordinary 80-42, base shot)

The sheets are `raw/sheets/switch-european-beech-80-42.png` and `raw/sheets/switch-ordinary-80-42.png`. They hold five stills each:

1. As shipped.
2. Lobes off (`/surface/lobes 0`, `lobeDepth 0`).
3. Flare off (`flareRadius 1`).
4. Bark relief off (`ridgeScale 0`, `plateCellScale 0`, `barkGrainStrength 0`).
5. Wood level forced to 0 (`Level::Forced(0)`) in place of the chosen screen-error level.

The collars and the waist stand unchanged in all five. None of these surface stages makes them.

The positive check was an env-gated diagnostic, kept out of the tree as `raw/diagnostic-relay-girth.patch`. It does two things:

- It takes the girth from the node below the bud, where the pipe has fully entered.
- It patches from the bud itself.

The bole's rises vanish in the probe. The collars and waists are gone in `raw/sheets/probe-relay-girth-{beech,ordinary}-base.png`. What remains is the tip-relay case (the ordinary-80-1 terrace), which the diagnostic does not cover, and the oak-style hairlines.

The oak hairlines are the pipe model's own step at each lateral. They are sharp because each step falls between two drawn nodes about 4 cm apart. They are not the relay defect, and they are minor at the harness views.

## Reference photographs

The only real trunks in `.refs/` are `fn9/quga788B.jpg` (Oregon white oak) and `fn9/piab977.jpg` (Norway spruce). Both are whole-tree photographs, not trunk close-ups. `raw/sheets/refs-oak-spruce.png` sets them beside the 80-year stills. Neither shows a ring or collar on the bole. A beech has a smooth, near-cylindrical bole, so the collars are not its bark form. No photograph was downloaded.

## Verdict for R1

This is not a values fix. The defect is how `converted` draws a relay's stem against where the girth model enters its pipe. The design goes to the host, and this task adds no code before then.

`relay_at` is a species' botany (Troll's relay in the curvature zone), and moving it to 1.0 would only hide the defect in beech. Ordinary dents at its own `relay_at`.

## Host decision (2026-10-09): one source for where a relay joins

**Option 2.** The relay is drawn from the node where the girth model already adds its wood. The girth model's attachment, `relay_at` and the node it resolves to, becomes the one place that says where a relay joins its parent. The drawing, `converted`'s `Origin::Relay` branch and its last-node case, reads that attachment. It no longer picks its own "node nearest its base". The trunk below the join then already carries the relay's wood, because girth computed it. No second thickening rule is added (option 1), and no value is moved to hide the dent (`relay_at` stays).

- **Both cases:** the mid-axis relay and the relay at the parent's last node.
- **The device path:** check whether it has its own copy of this conversion. If it does, it reads the same attachment. CPU and device agree within fn-125's screen.
- **The oak's hairline rings** (a 1-4 % step above each side branch) are the pipe model working as intended. They are not this defect, and no change is made.
- **Proof:**
  - A red/green test: no radius along a stem rises again just below a relay join beyond the pipe model's own step, on beech 80/42 and ordinary 80/42 and 40/42.
  - The before and after still sheets for every species, viewed.
  - `space_hash` changes only where relays are drawn, and each change is listed.
  - The gate, npm test and impl-review.

Keep the task open for the owner's verdict.

## Option 2 built and measured: it moves the dent, it does not remove it (2026-10-09)

I built option 2 on the CPU path. It is saved as `raw/option2-lowest-entry.patch` and reverted, so the code tree is unchanged:

- `telperion_space::relay_join` derives the join from `girth::relay_shares`.
- `converted` hangs the relay there.
- `joint` and the bud-to-joint thickening loop are deleted.

**What the girth model actually says.** It splits a relay's pipe three ways:
- `b * t` enters at the bud's node;
- `b * (1 - t)` enters at the node below it;
- `1 - b` enters at the parent's tip, through the stopped module's head.

Trunk relays keep a blend `b` well short of one for life. Beech at seeds 42 and 7, both ages, shows `b` = 0.35, 0.47, 0.62 and 1.0, with `t` = 0.2 or 0.9. Ordinary 40-7 shows `b` = 0.09. So most of a relay's pipe often still runs up the old leader's head.

**What a single drawn join therefore does.** The patch joins at the lowest entry taking a share, which is the node below the bud whenever `b > 0` and `t < 1`.

- **The trunk.** Every node below the join carries the whole relay. The probe finds no rise along any trunk sampled, and the 80-year beech collars are gone.
- **The head.** The head is turned aside at the join and carries `(1 - b) + b * t` of the relay. It is drawn as a lateral nearly as thick as the stem (beech 40-42: heads of 0.18 to 0.20 m on a 0.21 m trunk).
- **The new defect.** The head's thick base appears as a new flange ring at the join. See `beech-40-42`, `beech-40-1`, `ordinary-40-1` and `ordinary-40-7` in `raw/sheets/ba-*-base.png`, and `raw/sheets/option2-stubs.png`.

**The other rules would not help.**
- **The largest share as the join** leaves the trunk short of the minority share below the join, which is the original dip at a smaller size.
- **Any single node** leaves the minority share drawn in the wrong place, because the girth model spreads the pipe over the bud span and the tip while the drawing must hang the relay from one node.

**Byte changes.** Oak, palm and spruce stills are byte-identical. Birch changes in 7 of 18 stills. Beech, ordinary, Laurelin and Telperion change in all of theirs.

**Still open, a host design call.** Option 2 as written cannot be dent-free while `b < 1`. Two ways forward:
- **Change the girth side.** The drawing reads the join, and the girth model enters all of a drawn relay's pipe at that join. This changes girth, sag and the device's girth.
- **Change the drawing side.** The drawing hangs the relay at the join and draws the head as carrying only its own wood. The girth model and the drawing would then disagree on the head.

**The device path.** `crates/telperion-render/src/generation/fine/dress.wgsl` has its own copy (`joint` at line 241, the thickening loop at lines 484-499). Whichever rule is chosen goes there as well. `girth.wgsl` already has `relay_span_q`.

## Host decision, second (2026-10-09): change girth, so a relay's wood enters at its join

**Option 1.** Botanically, a relay's conducting pipes reach the parent axis through the node it grows from, and then run down the stem below. They do not pass through the old leader's tip, which becomes a stub carrying only its own wood. The girth model's split, which sends `1 - b` of the relay's wood to the old leader's tip, is the one rule out of step with that, and the dents follow from it. So the fix goes where the cause is, and girth and drawing stay one source (option 2 would make them disagree about the stub).

1. **Girth:** a drawn relay's wood enters the parent at its join. The node below carries it, and the stub carries its own wood only. Apply the same rule in the device's girth shader, and in `dress.wgsl`'s `joint` and thickening loop, so CPU and device stay one rule.
2. **The drawing:** it joins at that node, as option 2 built. Reuse the saved patch.
3. **`relay_at` and the blend `b`:** state what each still means once the wood enters at the join.
   - **If either becomes inert,** say so with a test. It is retired only on the host's word, by name in the report. Do not keep a row that does nothing.
   - **If either still shapes something real,** such as where along the parent's last unit the join sits, keep it and document it.
4. **Sag and everything else that reads girth:** these change by design. Measure the change on the beech and the ordinary tree (sag of the trunk and limbs at 40 and 80).
5. **Proof:**
   - The red/green test from before, with no rise along a stem below or above a join, plus a stub-radius check.
   - Before/after sheets for every species, viewed.
   - `space_hash` changes listed for every tree with relays. Oak, palm and spruce stay identical.
   - CPU and device agree within fn-125's screen.
   - The gate, npm test and impl-review.

Keep the task open for the owner's verdict.

On your friction note: a probe line for `b` and `t` before a decision is a good rule. It belongs in the onboarding method's "measure before building" step. The host carries it to fn-237's friction list.

## Option 1 built: the dents are gone, and fn-199 R4 breaks (2026-10-09)

The change is on branch `fn-244-option1-wip` (`5d01f0b3`), not on the task branch. The task branch has no code change.

### What it does

- **CPU:**
  - `girth::thicken` and the bounded girth enter a relay's section at the node it grows from, the way a lateral's enters.
  - `sag` enters a relay's load there too; see the sag measurements below.
  - `converted` hangs the relay from that node. Its `joint` function and the bud-to-joint thickening loop are deleted.
- **Device:**
  - `girth.wgsl` uses `borne_w` at the node.
  - `settle.wgsl` uses `load_at` at the node.
  - `dress.wgsl` draws from `ax(i, NODE)`. Its `joint` and thickening loop are deleted.
- **Now dead, and deleted:**
  - `relay_span` and `relay_span_q`, plus the device's `relay_span`;
  - `relay_parts`, `relay_shares`, `Entry`, `Load::split` and `split_q`;
  - the `S_BASE` slots;
  - `relay_at` in the device girth table.

### `relay_at` and `b`

Both still shape the tree, so neither row is inert:

- **`relay_at`** still picks the node a relay grows from, and so the join. It also places the relay's base and frame within that node's span (`stand.rs`), and the closed-form path and mass read it (`closed_form/kept/path.rs`, `mass.rs`).
- **`blend` (`b`)** still sets a relay's place and heading between its bud and the continuation (`geometry.rs` and `stand.rs` through `carried_on`).

What they no longer do is split the relay's wood or load. Their docs say so (`species.rs`, `structure.rs`).

### Red and green

| Test | Base | Option 1 |
|---|---|---|
| `relay_tests::a_stem_never_thickens_on_its_way_up` (beech 80/42, ordinary 80/42 and 40/42) | Red: beech stem node at 1.02 m is 0.6284 m over 0.5535 m | Green |
| `girth::tests::a_relays_wood_enters_at_its_node_and_its_stub_carries_its_own` | Red: the parent at and below the join is not the lateral's | Green |

The stub-radius check is exact: the stub equals the parent with no relay, and the parent at and below the join equals the parent with a lateral there. A threshold version of the check (stub below 0.4 of the relay) was wrong. A crown relay's stub keeps its own laterals' wood (0.0365 m beside 0.0378 m).

### Stills

- Sheets: `raw/sheets/ba2-*.png`, all viewed.
- Identity check: `raw/sheets/ba2-cmp.txt`.
- Oak, palm and spruce are byte-identical.
- Birch changes in 5 of 18 stills; beech, ordinary, Laurelin and Telperion change in all of theirs.
- **Bases:** the beech collars, the ordinary waists and the ordinary-80-1 terrace are gone, and no flange appears.

### Sag

Length-weighted mean pitch of the limbs, in degrees (radius at least 5 % of the root's, non-stem):

| Tree | Base | Wood at join, load split as before | Wood and load at join |
|---|---|---|---|
| ordinary 80/1 | 15.81 | 8.68 | 15.86 |
| ordinary 80/7 | 22.42 | 9.72 | 22.47 |
| ordinary 80/42 | 20.85 | 10.59 | 20.87 |
| ordinary 40/7 | 19.91 | 18.72 | 19.92 |
| beech 80/42 | 37.14 | 37.20 | 37.20 |

- **Why the load moved too.** With only the wood moved, the stub kept the `1 - b` of the relay's load on its own wood, and the ordinary's limbs dropped 7 to 13 degrees. Moving the load with the wood puts them back within 0.05 degrees of base.
- **Elsewhere.** Beech changes by at most 0.7 degrees (limb pitch at 40/1). Stem pitch and lean are unchanged on beech and ordinary to 0.2 degrees. Birch changes by 0.02 degrees.
- **Host to confirm.** Moving sag's load is my reading of "one rule". The host did not name it.

### The gate, per package (option 1)

| Package | Result |
|---|---|
| `telperion-core` | 454 passed |
| `telperion-render` | 222 passed, with the device and CPU agreement tests (trunk and grazing trunks against the box reduction) |
| `telperion-space` | 221 passed, 1 failed |

The failure is `tests/relays.rs::a_relay_moving_along_its_parent_thickens_it_by_degree`. That is fn-199 R4: walking `relay_at` moves the parent's girth by degree.

- **On base,** the `t` split made the steepest step shrink twentyfold when it was refined.
- **Now,** the relay's wood enters at one node, so crossing a node boundary moves it a whole internode at once. Seeds 1, 2 and 3 step 0.42, 0.49 and 0.36 of the thickest girth, and the step does not shrink.

### For the host

This is the principle "every parameter changes the tree by degree", applied to `relay_at`. It is an owner requirement, so the agent does not retire the test. Three ways forward:

1. **Retire R4 for girth.** A drawn join is a node, and the wood enters there. `relay_at` changes the tree by degree in place and frame, but moves the wood a node at a time.
2. **Draw the join where the relay stands.** The drawing adds a node at the relay's base, `t` along the span. Below it the parent carries the relay; above it, the stub. The wood then moves by degree with `relay_at`. This changes construction in `converted` and `dress.wgsl`.
3. **Keep the `t` split of the wood between the two nodes and drop only the `1 - b` tip share.** The stub's first node then carries `b * t` of the relay. That is option 2's flange at a smaller size, so this is not recommended.

`npm test`, `space_hash` and impl-review have not run, because the code is not on the task branch.

## Host decision, third (2026-10-09): the join stands where the relay stands

**Option 2.** The drawing gains a node at the relay's own position along the span, the point `relay_at` and `b` already place it. Below that node the parent carries the relay's wood and load; the stub above carries only its own. As `relay_at` moves, the join moves with it continuously, so fn-199's R4 (the owner's by-degree rule) holds and no requirement is retired.

- **Both executors build the drawing this way:** `converted` on the CPU and `dress.wgsl` on the device. The inserted node is an ordinary drawn node, interpolated along the span the way every other one is.
- **Sag's load moves with the wood to the join, on both executors. Confirmed.** The ordinary's limbs back within 0.05° of base show the load belongs there.
- **Proof:**
  - fn-199's `a_relay_moving_along_its_parent_thickens_it_by_degree` passes unchanged.
  - The no-thickening-upward and stub tests pass.
  - The sheets are viewed.
  - `space_hash` changes are listed; oak, palm and spruce stay identical.
  - CPU and device agree.
  - Then the gate, npm test and impl-review.

Build on the parked `fn-244-option1-wip` and bring it onto the task branch.

## Before building the third decision: what an inserted join node costs (2026-10-09)

I checked against the code before writing any of it. Four constraints make this a construction design question, and the host should see them first.

1. **R4 reads the girth model, not the drawing.** `tests/relays.rs::radii` reads each phytomer's `radius` in the `Structure`. An inserted drawn node cannot make it pass by itself. The girth has to enter the relay's wood by degree again: `t` at its node and `1 - t` at the node below, with the blend's tip share dropped. Sag's load does the same.
2. **The join node's two radii are not in the structure.** With that split, the phytomer the relay stands in carries `t` of the relay's wood: neither the full wood (below the join) nor none (the stub above it). The drawn join needs both radii. They come from the girth's sections and exponents, so `telperion-space` has to compute them, either recorded during `thicken` or by a public function. The conversion cannot hold a second copy of that arithmetic. With `secondary < 1` the weighing has to be undone too; only the palm has 0, and it has no relays.
3. **Node order.** A node's parent is "strictly earlier than this node" (`tree.rs:37`), and the conversion draws a relay after its parent axis. The join sits between the parent's node below and the stub's first node, so it has to be created while the parent axis is drawn: the parent's drawing reads where its relays stand. Its identity (fn-220) cannot be a phytomer's key, because no phytomer makes it. It needs a new rule, for example the relay's lineage with a place of its own, stable across ages and cuts.
4. **The device has one record per laid phytomer** (`dress.rs`: `phyts * CV` words; node ids are record indices). The join needs a record of its own per drawn relay. That changes the laid wave's sizes, the record layout read by `dress.wgsl`, `fold.wgsl` and the stations, and the node-for-node agreement tests.

**What the build involves:**
- the girth and sag split by `t` (CPU and device);
- a join-radii source in `telperion-space`;
- the join node in `converted` with an identity rule;
- an extra record per relay on the device;
- the tests.

This is several hours. Most of it is the device's records and identity, and both need a host decision.

**A cheaper variant that fails the decision's continuity.** Keep the girth's `t` split, join the relay at the node below, and draw the stub's first node at its radius without the relay. That needs no new node or record. But the join jumps one internode each time `relay_at` crosses a node, so the drawing does not move by degree. R4 itself, which reads the girth, would pass.

Option 1 stays parked on `fn-244-option1-wip` (`5d01f0b3`). Its only failing test is R4.

## Host decision, fourth (2026-10-09): one resolved attachment, the full build (Astra, ASTRA-JOIN.md)

Astra's review agrees with the join node and settles the two open calls. There is no existing between-node attachment to reuse, so the node is the simplest design that meets all the constraints.

1. **One attachment, resolved once in `telperion-space`.** It is a carrier span plus a fraction along it, built from `relay_at`, the unit extent and `b`, with the blend along the carrier. Geometry, girth, sag and the drawing all read it, and nothing else decides where a relay joins.
2. **Girth and sag project the relay's whole section and load onto the two neighbouring phytomer samples by that fraction.** That keeps per-phytomer radii continuous, so fn-199's R4 holds. Girth also supplies the join radius (the full bole below) and the stub radius (the old head's own wood). Both come from its own arithmetic, before secondary-growth weighting, as one function the drawing calls. No second copy of the arithmetic.
3. **The drawing inserts one ordinary node at the attachment,** while the parent axis is drawn, so the parent-before-child order holds. The bole runs on into the relay, and the old head starts from the stub radius with any laterals it bears.
   - **Its identity:** the relay's lineage under a new registry key (`JOIN`), mirrored in WGSL. That keeps it stable across ages and cuts (fn-220).
4. **The device mirrors this rule.**
   - **Records:** one extra record per relay, sized from the relay count known after topology.
   - **Layout:** `dress.wgsl`, `fold.wgsl` and the stations read the new layout.
   - **Tests:** the node-for-node CPU-against-device tests are updated to include join nodes.

**Proof:**
- fn-199's R4 passes unchanged.
- The no-upward-thickening and stub tests pass.
- The sheets are viewed.
- `space_hash` changes are listed, with oak, palm and spruce identical.
- CPU and device agree within fn-125's screen.
- The gate, npm test and impl-review.

On the friction entry about constraints surfacing after each decision: it is right. Before writing a decision, the host now asks the worker for a 10-minute read of every file the decision names. That is recorded for fn-237's friction review.

## Decision 4, the pre-build read: two contradictions (2026-10-09)

I read the attachment's files before writing code:
- `geometry.rs` `framed_q`, `bend`, `reaches` and `phase_by`;
- `geometry/stand.rs`;
- `stand.wgsl` and `walk.wgsl`;
- the girth and sag files, the bounded girth, and `closed_form/kept`.

A census over the space crate's own species tables (`beech()`, `birch()`, `oak()`, `spruce()`, `palm()`), at seed 42 and ages 40 and 80, counted:
- relays;
- those whose blend `b` lies strictly between 0 and 1;
- those whose parent's last growth unit is still growing in (extent below one);
- those whose stand therefore falls on an ancestor axis.

| Species, age | Relays | 0 < b < 1 | Last unit growing | Stand on an ancestor |
|---|---|---|---|---|
| beech 40 | 273 | 226 | 0 | 0 |
| beech 80 | 2,444 | 1,106 | 7 | 7 |
| birch 40 | 257 | 212 | 5 | 2 |
| birch 80 | 2,496 | 2,143 | 27 | 10 |
| oak 40 | 230 | 203 | 31 | 5 |
| oak 80 | 1,180 | 1,018 | 185 | 22 |

Spruce and palm grow no relays at either age.

**C1. The carrier is not always the relay's parent.**
- `stand` reads units "along the whole line of axes carried on, as one axis". When the parent's last unit is still growing in, it blends back toward the unit before, which can lie on the grandparent or further up the line. That happens to 7 relays of the beech at 80, 10 of the birch and 22 of the oak.
- An attachment resolved there has a carrier span on an ancestor. But girth's and sag's upward pass, and both drawings, only attach a relay to its `Origin` parent.
- **Option (a):** attach across axes. The relay's section enters the ancestor's sample, the whole parent axis becomes stub, and the drawing hangs the relay from the ancestor's run. This needs new handling in girth, sag, bounded girth, `converted` and the device.
- **Option (b):** clamp the attachment to the parent's last unit. This changes the stand geometry for those growing relays.

**C2. "Oak identical" cannot hold.**
- The oak's `relay_at` is 1, so its relays stand at the parent's tip, and options 1 and 2 left the oak byte-identical.
- Once the attachment includes the unit extent and blends along the carrier, every oak relay whose last unit is growing (185 at 80, 31 at 40) moves back to the unit before. That moves its girth and base too.
- The birch also changes more widely than before: the chord blend by `b` becomes a blend along the carrier for its 2,143 partial blends at 80.
- Spruce and palm have no relays and stay identical.

**Two smaller points for the scope:**
- **Contract-A geometry on the device.** `stand.wgsl` and `walk.wgsl` (`frame_of`) mirror `stand.rs` and `framed_q` op for op under contract A. A new attachment geometry means mirrored Q32 changes there too, on top of the device's girth, sag and dress.
- **The closed form.** `closed_form/kept` (`path.rs`, `mass.rs`, `events.rs`) places relays in expectation from `relay_at` alone, for the size predictions. It cannot read a grown tree's attachment. I would leave it unchanged unless the host says otherwise.

**Waiting for the host on:**
- C1, option (a) or (b);
- whether the oak and birch may change (C2);
- confirmation that the device's contract-A geometry is in scope.

No code is written.
