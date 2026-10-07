# Flowers on the engine path: the design (fn-216 R1)

A design proposal for flowering organs, written for the host to decide. No product code is changed. The spec asks for blossoms over the outer crown and pendant racemes, as species parameters by degree (none at zero), expanded by the pipeline as leaves are, with a flowering phase by the year. Laurelin (fn-218) and Telperion (fn-219) depend on it, and so does the supernatural group's emitted light (fn-217). Every code claim below names its file and line on this branch (`fn-216-tree-space-flowers-on-the-engine-path`, from fn-198's tip `96456e8f`). A claim marked *read* was checked by reading the code and was not run. A claim marked *unknown* was not checked.

## 1. What exists

### No organ exists anywhere

- No flower, blossom, raceme, inflorescence, catkin or petal code exists in `crates/telperion-core/src`, `crates/telperion-space`, `crates/telperion-render/src` or `src` (grep, 2026-10-07). The only hits are a comment in `crates/telperion-space/src/oak.rs:5` ("flowering laterally", Rauh's model) and the declared-but-unmet capability `infructescence` at `crates/telperion-core/src/capability.rs:104-106`.
- **fn-33** (open, no tasks) is the earlier design: one organ slot per family as a second foliage draw, with a sub-annual season. It predates fn-158's pipeline move and the tree-space engine, so its file references (`crates/telperion-core/src/foliage/...`, growth cohorts, `PlacementIdentity`) no longer resolve. Its decisions still apply and are adopted below: one slot rather than a list, a second draw rather than a per-instance kind, and a season expressed as a rule rather than a stamp. The owner named its proof set on 2026-09-14: "Spruce cones, Ash compound leaves, Oak catkins". **fn-111** (the palm's date cluster) waits on the same slot.

### The engine knows where flowers would stand; the conversion drops it

- An engine axis carries its physiological age `pa`, its `birth` cycle and its `lineage` (`crates/telperion-space/src/structure.rs:94-117`). A phytomer carries its `cycle`, its frame (`tip`, `heading`, `side`), its `scale`, a lineage-stable draw `key` (`:132-165`, public through `Phytomer::key` at `:161`) and, crate-private, the `light` its bud grew in.
- The shared reference axis has ages for spurs and short shoots, `SPUR` = 11 and `SHORT` = 14 (`crates/telperion-space/src/chain.rs:25-28`). `PaState::leaf_area` (`species.rs:136`) is the engine's only per-age statement about what a node bears.
- The conversion `convert` (`crates/telperion-core/src/pipeline/space.rs:190-326`) reads `axis.pa` only to decide `stem` (`:236`). It builds every node from `Node::root()` (`:265`) and never sets `identity`. *Read:* on the engine path every node therefore carries the default identity, `birth = u64::MAX` (`crates/telperion-core/src/tree/identity.rs:12-18`). Only the scaffold grower assigns identities (`pipeline/branching/specimen.rs:104`). No age, phytomer key or cycle reaches the pipeline's `Tree`.
- *Read, consequence:* any expansion draw keyed by node identity is the same on every node of an engine tree. Short shoots key by `identity.birth_order()` (`pipeline/foliage/short_shoots.rs:171, 180-181, 203, 230`), so spur `k` would draw the same bearing on every piece of wood. No shipped engine preset sets `shortShootSpacing` (the beech sets only the radius, length, leaves and spread rows, `presets/european-beech.values:98-101`; the spacing's default is 0, which leaves short shoots off), so nothing shipped shows the effect today. An organ must not inherit this keying.

### Leaves on the engine path

- Leaf-bearing wood is `NodeKind::Twig`, or wood more slender than `shootRadius` (`pipeline/foliage/plan.rs:270-315`). The conversion assigns the kinds (`space.rs:309-316`). Stations along a run are spaced by the canopy's twig internode, not by the engine's phytomers (`plan.rs:105`). Leaves therefore stand on the engine's wood but not on its nodes.
- The station sources that exist:
  - twig runs (`plan.rs` `Run`, `:31-37`);
  - short shoots (`short_shoots.rs:150-227`);
  - rosette fronds (`rosette.rs:108-201`);
  - leaflets along a rachis: arched, alternating, with a terminal leaflet and basal spines (`leaflet.rs:24-70`), and the plan's reach includes the rachis (`plan.rs:177-184`).
- **The GPU covers twig runs only.** `supports_stations` refuses short shoots, limb clumping and rosettes (`pipeline/foliage/prepared.rs:41-43`). Those families take the CPU fallback (`crates/telperion-render/src/generation/preparation.rs:22, 174-183`). fn-125, which removes the fallbacks, is still open.
- **GPU draws are keyed by the station's global ordinal.** `station(index)` draws with `random(index·4 + j)` (`generation/pack.wgsl:31-35, 66`). A leaf's scatter and size therefore depend on how many stations precede it in the whole tree, not on its lineage.
- **The cull.** A station is kept when any vertex lies in the crown's outer shell (`generation/place.wgsl:42-54`). The beech's `shellDepth = 1.0` keeps every station.

### What a leaf is, end to end

- One stored leaf is three words: a rotation, a position quantised to a box, and a half-float scale. The scale's sign bit marks a withered leaf (`pipeline/foliage/packed.rs:1-30`). No bit is free.
- The element is one blade built from rows: length, width, widest point, fullness, tip, cup, curl, lobes, section roundness and segments (`pipeline/foliage/element.rs:27-146`). Its level ladder anchors the blade's base, widest section and tip (`levels.rs:172-186`).
- A tree carries one foliage: `mesh::Foliage { element, instances }` (`mesh.rs:13-16`), and `TreeMesh` holds one `foliage` (`:24-30`). The renderer holds one `Foliage` and one `Select` (`telperion-render/src/lib.rs:106`; `foliage.rs:72`; `select.rs:79`), and it draws and shadows them once (`lib.rs:463, 494-495`).
- **The leaf shader** (`shaders/foliage.wgsl`) reads its colours from the frame uniforms, never per instance:
  - front, back and dead (withered) colours (`:130-135`);
  - per-leaf hue and brightness variation;
  - veins (`:76`) and a margin colour band (`:162`), the latter is the term Laurelin's gold rim can use;
  - transmission and scattering through the leaf (`:197-206`), gloss and sheen.

  The rows are `material/params.rs:68-130` and the packing is `scene/frame.rs:99-207`.
- **Time.** The pipeline has no time of year. The `Request` holds none (`pipeline.rs:43-54`), and the engine refuses a fractional age (`space.rs:52-59`).

## 2. The design space

Every option shares four answers, which follow from the principles (`docs/principles.md`) and fn-33's decisions:

1. **One organ slot, a second foliage.** The organ has its own element, its own placements and its own level ladder, drawn by a second `Foliage`/`Select` pair through the same `foliage.wgsl` with its own colour block. There is no per-instance kind, no shader branch and no species branch.
2. **By degree, inert at zero.** A flowering share of 0 means no organ list, no second upload and no second draw. Every passed species stays byte-identical (R3), and an identity test holds this before any row is set.
3. **A floret is a pure function of (source, index, seed)** (fn-125). Its draws are keyed by the bearing node's lineage key, never by a global ordinal. Adding a twig elsewhere moves no flower, which is fn-192's continuity claim.
4. **Phase by degree.** The flowering window scales each floret's size in and out over a staggered opening, keyed per floret. A walk through the season grows flowers from nothing and withers them, rather than switching them. A floret whose scale is zero is dropped by the compaction, so the count changes and nothing pops.

The options differ in **where** flowers stand and **what grows the inflorescence's axis**.

### Option A: flowers as a foliage-layer source (fn-33 updated)

- **Where.** A new `/bloom` canopy group chooses stations on the existing leaf-bearing wood. It takes a `share` of twig runs, at the run's tip (`terminal`, by degree) or along it (lateral), and an outer-crown bias from the shell cull (`bloom/shell`, the existing `in_shell` with its own depth). There is no engine change.
- **What.** The inflorescence is a rachis with `florets` along it, reusing `leaflet.rs`'s rachis math:
  - `rachis` length, 0 for a cluster;
  - `droop`, signed: −1 an upright candle, +1 hanging;
  - `pedicel` length;
  - floret size `taper` toward the tip (a laburnum raceme opens from its base);
  - `spread`, the angle the florets fan across when the rachis is 0 (the short shoot's fan, `short_shoots.rs`).
- **Element.** A floret is one instance of an organ element: the blade rows, plus a `petals` count that builds the blade turned `petals` times about its base inside one element (neutral 1 is today's blade). One instance per flower keeps the count honest and costs a fifth of five petal instances. The ladder rule runs on one petal and is replicated, so its anchors still hold.
- **Keys.** A source is a twig-run station. *Read:* on the engine path its only stable key is the station's place in the run, and runs have no lineage identity (see §1). Option A therefore also needs the conversion to carry the phytomer key (as Option B does), or it inherits ordinal keying, in which a twig added anywhere reshuffles which stations bloom.
- **Botany.** It is blind to physiological age. A cherry's flowers stand on spurs of two-year and older wood, and a laburnum's racemes on short shoots. Option A can approximate both only through wood thinness and the shell, which is the scaffold's way of placing leaves. Terminal against lateral flowering, the Hallé and Oldeman model distinction, is a placement row rather than the axis's own.
- **GPU.** The second list needs the GPU station path to expand a rachis into florets. Today nothing in `telperion-render` expands leaflets (fn-125 Architecture, "Leaflets"). Option A would build that expansion for organs and leaflets together.

### Option B: the engine says where, the expansion says what (recommended)

- **Where: a per-age row on the species.** `PaState` gains `flowering`, neutral 0: the share of a growth unit's nodes of this age that bear an inflorescence, preset rows `/space/ages/<k>/flowering`. It is a value on the shared reference axis, so a walk between species mixes it like every other setting (fn-206). A species flowers where its botany says, for example a cherry on `SPUR` and `SHORT`, Laurelin on her twig ages. Two further rows:
  - `bloomAge`: the cycles an axis lives before it can flower, by degree (cherries flower on wood two years and older). Dormant without flowering.
  - `bloomLight`: optional, neutral 0. Presence is multiplied by `light^bloomLight`, so a species whose buds read light flowers on the lit outer crown for a botanical cause, not through a shell rule. It is dormant in a species that reads no light: today only the oak reads light, and the fine path refuses light-reading species (`docs/pipeline.md`, "Fine growth"). Whether that holds Telperion's "whole outer crown" is *unknown* until tried. The fallback is Option A's shell rule as a row, and the host's call.
- **The engine does not grow flowers.** Topology is unchanged, so FINE-CONTRACT and the GPU fine executor (fn-209) are untouched. The engine only exposes what it already has: each kept phytomer's `key`, `cycle`, its axis's `pa` and `birth`, and (if `bloomLight` is wanted) its `light`.
- **The conversion carries a side table.** `Tree` gains `blooms: Vec<Bloom>`: node index, phytomer key, age share and the side the bud faces. It is empty where no age flowers, following the precedent of `Tree::sections` (`tree.rs:117-123`): "Empty on every tree that shapes none", and never stored. A passed species carries an empty table and keeps its bytes. Cost: 24 bytes per flowering node.
- **The expansion draws each inflorescence.** For each bloom it draws presence against `flowering` with the phytomer key: a 53-bit threshold draw, the engine's own representation (`docs/pipeline.md`, "Fine growth"). Each floret is then a pure function of (bloom, floret index, seed). What grows is Option A's rows: rachis, droop, pedicel, taper, florets, spread, the `petals` element, size and scatter. Its frame is the phytomer's own: an axillary inflorescence leaves along the bud's side at the age's insertion angle, and `terminal` (by degree) turns it toward the axis's heading at the node.
- **The season.** Rows `/bloom/open` and `/bloom/length` (shares of a year) and `/bloom/stagger`, and a time of year that the request carries (`season`, 0 to 1, default the window's middle so a still shows the tree in flower). A floret's scale follows the window, staggered by its keyed draw; past the window it withers (the withered bit, drawn in the organ's dead colour) and shrinks out. fn-217's light cycle reads the same clock.
- **Why B.** "Where" is botany, and the engine is the botany. The flowering age is a property of the architectural model (Rauh: lateral flowering; Scarrone and Leeuwenberg: terminal), which the space claims to hold. "What" is shape, and the expansion already owns shape and the GPU path. Neither crosses into the other's contract.

### Option C: the inflorescence as engine axes

- An inflorescence age on the reference axis (`AGES` 15 → 16) grows the rachis as wood: a slender, fast-sagging axis whose phytomers each bear a floret instead of a leaf. Racemes would hang from their own load through `sag.rs`, which is the true cause of laburnum's droop. Florets become the organ list on that age's nodes.
- **Cost.**
  - Topology changes: every floret is a phytomer, so FINE-CONTRACT, the GPU fine executor and its bit-equality tests all move.
  - A new age reshapes every preset's indexed rows, and the `lateral` vectors are sized `AGES`. Whether passed species keep their bytes is *unknown*.
  - Pedicels of 0.5 mm stand above `FINEST` = 0.2 mm (`space.rs:35-37`), so they would be drawn as wood: thousands of extra rings.
- It is the most faithful option and the most expensive one. It earns its place only if B's drawn droop reads wrong beside photographs. It stays open as B's successor for the rachis alone: B's rows can become an age's settings later, because both are values on the same axis.

## 3. What each option needs, to the GPU and the screen

| Step | A | B | C |
|---|---|---|---|
| Engine | none | `PaState.flowering`, `bloomAge`, `bloomLight`; validation; walk | a new age; topology; FINE-CONTRACT; GPU fine executor |
| Conversion | phytomer key on nodes (for honest keys) | `Tree::blooms` side table | ages and floret nodes |
| Plan / expansion | bloom stations, rachis to florets, `petals` element, season scale | the same, from `blooms` | florets on that age's nodes, season scale |
| Plan layout (fn-125 R1) | a bloom source kind | a bloom source kind: (node frame, key, share) per bloom | none new |
| GPU | a `bloom.wgsl` pass beside `place.wgsl`: one thread per floret, keyed, packed to the same three words; a second compaction | the same | the same, fewer source fields |
| `mesh` / `TreeMesh` | `organ: Option<Foliage>`, absent at zero | the same | the same |
| Renderer | a second `Foliage` + `Select`, own ladder and shadow casters; an organ colour block | the same | the same |

**The renderer terms (all options).** The organ draws through `foliage.wgsl` with its own uniform block of the same layout. No shader branch: each draw binds its block.

- **Petal colour:** front and back, from new material rows `bloomFront*`/`bloomBack*`, and a dead colour for spent florets (`bloomDead*`).
- **A base tint:** `bloomEye*` and `bloomEyeShare`, the colour toward the petal's base. It reuses the leaf coordinate `coord.x` the margin term already reads (`foliage.wgsl:162`), the margin's dual at the other end of the blade. It gives laburnum's dark nectar guide and the cherry's yellow stamens' read at distance. Dormant at share 0.
- **Veins off:** the vein strength row is per block, so a petal sets it to 0.
- **Translucency:** petals are thinner than leaves. The existing transmission and diffuse-through rows (`:197-206`) at their own values carry it, with no new term.
- **Emission:** not here. fn-217 adds it to the organ block only.

Because there is no free bit in the packed leaf (`packed.rs`), per-floret colour variation uses the seeded `vary(id)` the leaf already has, as hue and brightness spread rows.

## 4. A natural species to prove it on

R2 asks for blossoms and pendant racemes on a natural flowering species the host chooses and judges beside photographs. One species per spec (AGENTS.md) means the proof species is its own species spec, and this spec builds and tests the mechanism on a test tree. Candidates, best first:

1. **Bird cherry, *Prunus padus*: both halves on one tree.** Its white five-petalled flowers (Telperion's) hang in spreading or drooping racemes 7 to 15 cm long (Laurelin's form) on a tree up to 16 m (Wikipedia, *Prunus padus*). One species exercises the floret element, the rachis, droop, taper and the season. *Unknown:* its Hallé and Oldeman model. A source is needed before it is calibrated.
2. **Wild cherry, *Prunus avium*: blossoms, from the oak's point.**
   - It is Rauh's model, "rhythmic branching with acrotonic whorls of long shoots and lateral flowering on long and short shoots" (Lauri et al., *Sweet cherry tree architecture, physiology and management*; quoted from the search abstract, the paper not read). The oak is Rauh's model too (`oak.rs:1-9`), so calibrating it starts as a walk from a passed point, the cheapest new species the space can take.
   - Flowers are in corymbs of two to six, five white petals, 2.5 to 3.5 cm across, pendent on 2 to 5 cm stalks, with the new leaves in early spring; the tree is 5 to 25 m (Wikipedia, *Prunus avium*).
   - It proves blossoms over the outer crown, with short pendent pedicels (droop at a small rachis), but no long raceme.
3. **Laburnum, *Laburnum anagyroides*: the brief's own reference for Laurelin.** Pea-like yellow flowers densely packed in pendulous racemes 10 to 25 cm long; a small tree to 7 m (Wikipedia, *Laburnum anagyroides*). Its trifoliate leaf is in reach of the existing leaflet rows. *Unknown:* its architectural model. It proves racemes and no blossom.

A mechanism check needs no new species: **the oak's male catkins** (pendent strands, Rauh's lateral flowering, `oak.rs:5`; the owner's fn-33 proof set). It works as a test tree for the rachis and droop only (catkins are spikes: no pedicels and no showy petals), not as R2's judged species.

**Reference photographs** (Wikimedia Commons; licences from the Commons API on 2026-10-07; "viewed" means a 500 px thumbnail was looked at):

| Subject | File | Licence, author | Viewed |
|---|---|---|---|
| *P. avium*, crown in full bloom from below | [Prunus avium - Hohenwettersbach 01.jpg](https://commons.wikimedia.org/wiki/File:Prunus_avium_-_Hohenwettersbach_01.jpg) | CC BY-SA 3.0, H. Zell | yes: blossom covers the outer crown, wood visible between |
| *P. avium*, a corymb close | [Sweet-cherry bloom 2020 G1.jpg](https://commons.wikimedia.org/wiki/File:Sweet-cherry_bloom_2020_G1.jpg) | CC BY-SA 4.0, George Chernilevsky | yes: 5-petal florets on pendent pedicels from a spur, leaves just opening |
| *P. avium*, habit | [Prunus avium, habitus, Meiringskloof Natuurpark, a.jpg](https://commons.wikimedia.org/wiki/File:Prunus_avium,_habitus,_Meiringskloof_Natuurpark,_a.jpg) | CC BY-SA 3.0, JMK | no |
| *P. avium*, blossom series | [P1020555 Prunus avium blossoms.jpg](https://commons.wikimedia.org/wiki/File:P1020555_Prunus_avium_blossoms.jpg) | CC BY-SA 4.0, Thomas Schilling | no |
| *P. padus*, racemes among leaves | [Bird cherry in blossom - geograph.org.uk - 1281185.jpg](https://commons.wikimedia.org/wiki/File:Bird_cherry_in_blossom_-_geograph.org.uk_-_1281185.jpg) | CC BY-SA 2.0, Andrew Hill | yes: white drooping racemes over green leaves |
| *P. padus*, a tree in flower | [Hagg i nykarleby.jpg](https://commons.wikimedia.org/wiki/File:Hagg_i_nykarleby.jpg) | CC BY 2.0, Peter Portin | no |
| *P. padus*, racemes | [Prunus padus flowers.jpg](https://commons.wikimedia.org/wiki/File:Prunus_padus_flowers.jpg) | CC BY 2.5, Agnieszka Kwiecień (Nova) | no |
| *L. anagyroides*, racemes | [Golden rain (Laburnum anagyroides).jpg](https://commons.wikimedia.org/wiki/File:Golden_rain_(Laburnum_anagyroides).jpg) | CC BY-SA 3.0, Ввласенко | yes: dense pendent yellow racemes, florets largest at the base |
| *L. anagyroides*, tree | [Laburnum anagyroides Złotokap zwyczajny 2019-05-26 01.jpg](https://commons.wikimedia.org/wiki/File:Laburnum_anagyroides_Z%C5%82otokap_zwyczajny_2019-05-26_01.jpg) | CC BY-SA 4.0, Agnieszka Kwiecień (Nova) | no |
| *L. anagyroides*, inflorescence | [The yellow inflorescence of the Golden Chain Tree (Laburnum anagyroides).jpg](https://commons.wikimedia.org/wiki/File:The_yellow_inflorescence_of_the_Golden_Chain_Tree_(Laburnum_anagyroides).jpg) | CC0, Stch2022 | no |

They are catalogued and fetched into the ignored references directory under the proof species' spec, never redistributed, with the fn-19 reference record, as fn-33 required.

## 5. Cost (estimates, not measurements)

Every number here is an **estimate**, scaled from measured leaf counts. Measured base: the engine's beech at 40 bears 65,665 leaves and the oak at 40 bears 166,264, both on the GPU backend in the browser (fn-198 `BROWSER.md`; `TIMING.md`, quiet run).

**Instances.**
- A wild cherry in full bloom: about 10k to 40k corymbs × 2 to 6 florets, so **30k to 200k floret instances**, between the beech's and the oak's leaf counts.
- A bird cherry or laburnum at age 40: about 2k to 8k racemes × 15 to 30 florets, so **30k to 240k**.
- At 12 bytes an instance, that is 0.4 to 3 MB of placements, against today's leaf lists of 0.8 MB (beech 40) and 2 MB (oak 40).

**Draw.**
- One instance per floret, with a `petals`-turned element of perhaps 2 to 5 times a beech leaf's triangles at the finest level (the beech's `axialSegments` = 8; `european-beech.values:80-86`).
- The organ draw is estimated at **0.5× to 2× the leaf draw** at native resolution, less at distance where the ladder coarsens.
- fn-33 expected the second draw to cost in proportion to its instances. Measuring it by the fn-26 protocol is the build's R3 step.

**Build.**
- Option B's expansion is one keyed pass over blooms, linear in florets. It is estimated at **under 10 percent of the descriptor and wood time**: the beech at 40 measured 14 ms descriptors and 39 ms wood, so the pass is a few ms CPU and well under 1 ms GPU.
- B's side table is 24 bytes per flowering node, about 1 MB at 40k blooms.
- Option C adds the florets as phytomers: 30k to 240k more, against the beech-80's millions. Growth time scales roughly with phytomers, so +5 to 30 percent of the skeleton at age 40 (estimate).

**Work** (estimates; each estimate is a task of an hour or two of agent time):

| | A | B | C |
|---|---|---|---|
| Engine and conversion | 1 (key on nodes) | 2 (rows, walk, side table) | 5+ (age, topology, fine contract, GPU fine executor) |
| Expansion (CPU reference) and plan layout | 2 | 2 | 1 |
| GPU bloom pass and its equality test | 2 | 2 | 2 |
| Renderer second draw, organ block, shadow | 2 | 2 | 2 |
| Season and request time | 1 | 1 | 1 |
| Identity pins, cost measure, test-tree stills | 1 | 1 | 2 |
| **Total** | **≈9 tasks** | **≈10 tasks** | **≈13+ tasks, with engine risk** |

## 6. Recommendation

**Option B**: the engine's ages say where a tree flowers, and the expansion draws what flowers, each floret a pure function of (bloom, floret index, seed), keyed by the engine's phytomer key. The organ is one slot on a second foliage pass, drawn by the leaf shader with its own colour block and a base tint, and its season scales florets in and out by degree.

- It puts flowering where botany puts it, in the architectural model (the age), and so stays honest as the space grows. A walk mixes it like every other setting.
- It leaves the engine's topology and fn-209's fine contract alone, and every passed species keeps its bytes (empty `blooms`, no second list).
- The racemes for Laurelin and the blossoms for Telperion are the same rows at different values (rachis 0 against 20 cm, droop 0 against 1), and fn-217's light attaches to the organ block alone.
- It costs about one task more than A, and that task buys botanical placement and lineage keys. A needs half of that work anyway to key honestly.

**Decisions for the host before building:**

1. B over A and C, as above.
2. The proof species for R2. I recommend **bird cherry** (both halves on one tree), if a source for its architectural model is found in its species spec's first stage. Otherwise **wild cherry**, the walk from the oak, plus a raceme test tree.
3. Whether `bloomLight` is in the first build, or the outer crown is first left to the flowering ages and wood.
4. The `petals` count blends as a count, as fn-33 decided for leaflets. That is a discrete row (fn-33 precedent, `docs/principles.md` "One continuous tree space"), and the host should confirm it.
5. Whether the conversion's missing node identity (§1) is fixed for leaves too, in its own small spec. It affects short shoots on any engine preset that turns them on.

Not in this spec: emitted light and the dew (fn-217), the Two Trees' values (fn-218, fn-219), and fruit (fn-111 takes the slot once it exists).

## Host decisions (2026-10-07)

1. **Option B:** a `flowering` share per PA; the engine's topology unchanged; the expansion draws blossoms and racemes keyed by the engine's phytomer key; a second foliage pass through the leaf shader with its own colours; nothing drawn at zero flowering (passed species keep their bytes); a season scaling flowers in and out by degree.
2. **The proof (R2) on existing species, no new species:** the oak's catkins as pendant clusters (a real trait, judged against photographs), and a blossom test on the beech (the Two Trees' base), judged as an organ test. A bird cherry is its own species spec if the owner wants one later.
3. **The light-driven outer-crown row waits;** the first build uses the PA share only.
4. **Petal count** is a whole-number row whose fractional part scales the last petal's size, so a walk across counts changes the flower by degree.
5. **Node identity** gets its own small spec first: the conversion sets each node's identity from the engine's lineage, so every draw keyed by identity (flowers, short shoots) is stable and distinct. GPU leaf draws keyed by station number across the tree (a draw reshuffle when a twig is added) are recorded for fn-125's rewrite.
