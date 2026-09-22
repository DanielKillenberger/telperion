# Design handoff - fn-110, the palm's trunk organs

Written for the fn-80 gap loop (role design, tier high_reasoning, effort medium), 2026-09-22.
Contract: `.flow/specs/fn-110-the-palms-trunk-organs-persistent-leaf.md`, R1 to R4.
Source of the need: `.flow/evidence/fn80/capability.json`, the two `unsupported-anatomy` rows
`organ:persistent-leaf-base` (confidence medium) and `organ:acanthophyll` (confidence high).
Predecessor: `.flow/evidence/fn80/design-fn-109.md` and the code it landed.

No code is written here. Signatures only.

## What is built

Two organs, one capability set, both off by default, both value-table rows on `CanopyParams`.

**The retained leaf base is wood, appended after the radius solve.** A base is one node hung on a
stem node, swept by the surface builder as a run of its own, so the existing socket and swell draw
the junction as one piece of wood (`surface/samples.rs:94-127`) and the bark material, the bounds,
the shadow caster and the wood LOD all inherit it with no new code. The seam already exists.
`mesh::grow` (`mesh.rs:75-81`) mutates the tree before it is drawn and already does so on a canopy
row, so `clothe_leaf_bases` goes beside `clear_apical_twigs` there.

Appending after `branching::generate` is load-bearing. `radius::solve` runs the
pipe model over `0..tree.crossover` only (`radius.rs:54, 62-90`), summing each child's area into its
parent at `fork_exponent`. A base added before the solve would thicken the trunk it hangs on. A base
added after it carries explicit radii and the trunk is untouched.

**The bases are the rosette's own history.** Base `k`, counting from the newest downward, takes the
spiral angle `(rosette_fronds + k) * rosette_divergence` in the same frame `rosette::frame` builds
for the crown (`foliage/rosette.rs:213-221`), so the crown's spiral and the trunk's lattice are one
sequence with one divergence. Nothing authors a second spiral. The bases begin one `rosette_depth`
below the apex, where the oldest living frond leaves the axis (`foliage/rosette.rs:149`).

**The acanthophyll is a basal leaflet drawn as a spine.** `fan` (`foliage/rosette.rs:175-209`)
already strings `leaflet_count` leaflets along the rachis and already varies their pitch at the
terminal end (`:199-205`). The spine is the same seam at the other end. The first
`acanthophylls` leaflets of a frond take `acanthophyll_pitch` instead of `leaflet_pitch` and are
drawn at `acanthophyll_length` of their own size, by scaling the matrix's three basis columns before
`out.push`. No new instance, no new element, no rng draw moved.

## Interfaces

### New parameter rows

All eight go on `CanopyParams` (`foliage/placement.rs:15-144`), for fn-109's reason
(`design-fn-109.md:50-58`) and one more: `mesh::grow` holds `family.canopy` already
(`mesh.rs:77`), so the base pass reads its rows without a signature change.

| Wire path | Rust field | Type | Validated range | Default | Blend bucket |
| --- | --- | --- | --- | --- | --- |
| `/canopy/leafBases` | `canopy.leaf_bases` | `u32` | `0..=MAX_LEAF_BASES` (256) | `0` | `count` |
| `/canopy/leafBaseLength` | `canopy.leaf_base_length` | `f64` | `0..=10` | `0.` | `linear` |
| `/canopy/leafBaseRadius` | `canopy.leaf_base_radius` | `f64` | `0..=1` | `0.35` | `linear` |
| `/canopy/leafBasePitch` | `canopy.leaf_base_pitch` | `f64` | `0..=180` | `60.` | `degrees` |
| `/canopy/leafBaseWeathering` | `canopy.leaf_base_weathering` | `f64` | `0..=1` | `0.` | `linear` |
| `/canopy/acanthophylls` | `canopy.acanthophylls` | `u32` | `0..=MAX_LEAFLETS` (256) | `0` | `count` |
| `/canopy/acanthophyllLength` | `canopy.acanthophyll_length` | `f64` | `0..=1` | `0.35` | `linear` |
| `/canopy/acanthophyllPitch` | `canopy.acanthophyll_pitch` | `f64` | `0..=90` | `80.` | `degrees` |

Doc comments, which are also the dial table's `meaning_basis: "doc comment"`:

- `leaf_bases`: Bases of shed fronds the stem keeps below its crown, clothing the trunk. At zero the
  trunk is bare and the bark is what it always was; any rise carries the crown's own spiral down it.
- `leaf_base_length`: Metres a retained base stands out from the bark. At zero no base is drawn
  whatever the count says.
- `leaf_base_radius`: How thick a base is where it leaves the bark, as a share of the stem's own
  radius there. Raising it leaves a broader boot.
- `leaf_base_pitch`: Degrees from the stem's axis a base points: 0 flat against the trunk, 90 square
  out of it, 180 turned back down.
- `leaf_base_weathering`: How far the lowest and oldest base is worn back against the newest, in both
  its length and its girth. At zero every base stands full down the whole trunk, and any rise wears
  the foot away.
- `acanthophylls`: Leaflets at a frond's base borne as spines rather than blades. At zero the frond
  carries blades all the way down; any rise hardens that many of them.
- `acanthophyll_length`: The share of a leaflet's own size a spine is drawn at. At zero no spine is
  drawn whatever the count says.
- `acanthophyll_pitch`: The degrees a spine leaves the rachis, in place of the leaflet's own pitch.

The four non-zero defaults get `const fn` neighbours in `ranges.rs` beside fn-109's
(`ranges.rs:33-50`), named `default_leaf_base_radius`, `default_leaf_base_pitch`,
`default_acanthophyll_length` and `default_acanthophyll_pitch`.

The six authoring sites per row:

1. **Struct field** with its doc comment on `CanopyParams`, in the `foliage/placement.rs:87-140`
   block, carrying `serde(default = "...")` for the four shipped defaults and plain
   `serde(default)` for the four neutral rows.
2. **Default** in `impl Default for CanopyParams`, `foliage/placement.rs:145-187`.
3. **Validation** in the `for (v, l, h, n)` table in `rosette::validate`
   (`foliage/rosette.rs:59-79`), each float refused by its own name, and the two counts beside the
   existing `MAX_FRONDS` and `MAX_LEAFLETS` checks (`:72-77`).
4. **`fields!` line** in the `"canopy"` block, `params.rs:129-139`, which gives encode, decode,
   unknown-key refusal, overlay and browser metadata in one edit.
5. **Blend bucket** in `blend.rs`, as the table above names it (`blend.rs:65-71` `linear`,
   `:170-172` `degrees`, `:184` `count`).
6. **Dial row** in `crates/telperion-jev/data/dials.json`, `group: "canopy"`,
   `score_visible: true`, `meaning_basis: "doc comment"`, `range_basis: "validated bound"` or
   `"authored"`, `source` the `placement.rs` line, and `small < substantial <= max - min` with
   `small <= (max - min)/2` (`crates/telperion-jev/tests/dial_table.rs:142-156`).

Four of the eight guard a feature on and so join `SWITCHES_AT_ZERO`
(`dial_table.rs:35-62`), whose length moves 28 to 32: `leaf_bases`, `leaf_base_length`,
`acanthophylls`, `acanthophyll_length`. Each needs `min: 0` and the word "zero" in its meaning
(`dial_table.rs:230-245`). The doc text above already carries it.

### Layer boundaries

A new module `crates/telperion-core/src/branching/leaf_bases.rs`, sibling to the tree passes in
`branching.rs`, well under the 400-line rule:

```rust
/// One retained base: the stem node it is borne on, the point it leaves the
/// bark, the heading it stands on and how far along the spiral it is.
pub struct LeafBase { pub at: usize, pub from: Vec3, pub heading: Vec3, pub age: f64 }

/// Where this table's retained bases stand on this tree, newest first, in the
/// rosette's own spiral order. Reads the tree and changes nothing.
pub fn leaf_bases(tree: &Tree, p: &CanopyParams) -> Vec<LeafBase>;

/// Hang the retained bases on every stem as wood: one node a base, appended
/// after the radius solve, so no base enters the pipe model.
pub fn clothe_leaf_bases(tree: &mut Tree, p: &CanopyParams) -> Result<()>;
```

`mesh::grow` (`mesh.rs:75-81`) gains one guarded line after `clear_apical_twigs`, and its signature
does not move. `rosette::validate` becomes `pub(crate)` and `foliage.rs` re-exports it inside the
crate, so `clothe_leaf_bases` refuses a bad row by the same name `foliage::place` does
(`foliage/placement.rs:427`) even on a path that places no foliage.

One node a base, built the way `branching/local/advance.rs:304-321` builds a twig node:

| Field | Value | Why |
| --- | --- | --- |
| `position` | `from + heading * length_k` | `length_k = leaf_base_length * (1 - weathering * age)` |
| `parent` | the stem node it is borne on | parent-before-child holds, the base is appended last |
| `start_radius` | stem radius there `* leaf_base_radius` | its girth at the bark |
| `radius` | `start_radius * (1 - weathering * age)`, floored | `validate_range` refuses `start < radius` and a zero radius (`tree.rs:142-150`) |
| `base_radius` | `start_radius` | run origin, which `radius.rs:92-96` reads for nodes past the crossover |
| `branch` | its own index | a run of its own, so `bearing_runs` never continues a stem run into it (`foliage/placement.rs:520-523`) |
| `kind` | `NodeKind::Branch` | never `Twig`: `bearing_runs` clothes any twig node with foliage (`foliage/placement.rs:505-512`) |
| `stem` | `false` | `stem_apices` must not move (`tree.rs:120-132`) |

No rng is drawn. The spiral, the weathering and the pitch are closed form, so a base's geometry is a
function of the rows and the tree alone.

The spine's seam is inside `fan`:

```rust
/// The share of its own size a basal leaflet is drawn at and the degrees it
/// leaves the rachis, where the rows borne it as a spine.
fn spine(index: usize, p: &CanopyParams) -> Option<(f64, f64)>;
```

`fan` calls it per leaflet and, on `Some`, scales the matrix's three basis columns before
`out.push` (`foliage/rosette.rs:206`). The packed leaf keeps one uniform scale, the mean of the
three column lengths (`foliage/packed.rs:98-120`), so a uniform column scale survives the round trip
exactly and a non-uniform one would not. See Difficult cases.

### Capability vocabulary

`crates/telperion-core/src/capability.rs`:

- Move `acanthophyll` (`:85-88`) and `persistent-leaf-base` (`:89-92`) from `UNEXPRESSED` into
  `EXPRESSED`, meanings unchanged. The version moves by construction (`capability.rs:144-166`).
- Add both to `DERIVABLE` (`:103-113`), 9 names to 11. Without this the gate does not check that a
  table produces the name (`crates/telperion-jev/src/pipeline/stages/gate.rs:235-239`).
- `derived` (`:190-227`) gains two clauses, thresholds on shipped values like every clause above:

```rust
if f.canopy.leaf_bases > 0 && f.canopy.leaf_base_length > 0. && rosette {
    produced.push("persistent-leaf-base");
}
if f.canopy.acanthophylls > 0 && f.canopy.acanthophyll_length > 0. && pinnate {
    produced.push("acanthophyll");
}
```

`rosette` and `pinnate` are the locals already bound at `capability.rs:215-216`. The leaf base
requires the rosette because its spiral index starts at `rosette_fronds` and has no start without
one, and the spine requires the pinnate grouping because a spine is a modified leaflet. Both read as
the co-dependency `pinnate-frond` already has (`:223-225`). This matches the gap loop's own record
for this option, "generalizes: no".

## Invariants

1. **Byte-identity with both organs absent.** `leaf_bases == 0` and `acanthophylls == 0` are the
   defaults and every guard is on them together with their two length rows. No shipped preset sets
   any of the four. The proof is `crates/telperion-core/tests/species/digests.json`, whose four
   species (silver-birch, oregon-white-oak, european-beech, norway-spruce) are checked by
   `tests/species.rs:37-75`, and `catalogue/<id>/pins.json` through `tests/catalogue/pins.rs`. The
   date palm is in neither, so its own output is free to change, which is what R3 asks for.
   The trap fn-109 met does not recur here. `foliage::Reference::reach`
   (`foliage/reference.rs:61-111`) needs no new arm, because a base is wood rather than a station
   and a spine is shorter than the leaflet it replaces. The `fan` edit must skip the column scale
   entirely when no spine is borne rather than multiply by one.
2. **No species branch.** Nothing in `branching/`, `foliage/`, `surface/` or the renderer reads a
   preset id. Both organs fire on values, and the date palm is a value table like any other
   (`presets/species.rs:324-348`).
3. **The bases follow the crown's spiral.** One frame a stem, from `rosette::frame` on the apex's
   axis, and the angle of base `k` is `(rosette_fronds + k) * rosette_divergence`, continuing the
   crown's own sequence. A per-node frame is refused on purpose, because `frame` derives its reference from
   `Vec3::Y - axis * axis.y` (`foliage/rosette.rs:214`), which on a near-vertical trunk is a tiny
   vector whose direction is decided by rounding, so per-node frames would scatter the lattice's
   phase. The radial is re-projected square to the local axis at the node it is borne on, and falls
   back to the stem axis where that projection vanishes.
4. **Determinism.** No rng stream is opened by either organ, and the bases are visited in stem
   apex order, which `stem_apices` already sorts by identity (`tree.rs:127-131`).
5. **No renderer change.** A base is wood in the one `SurfaceMesh` the renderer already consumes
   (`mesh.rs:91-119`), and a spine is one instance of the one element. `CLAUDE.md`'s "supported
   parameter changes must require no renderer code changes" holds.

## Difficult cases

**Instanced geometry is not available, so a base is wood.** `TreeMesh` carries exactly one element
and one flat instance list (`mesh.rs:20-33, 91-119`), and the renderer holds one set of element
buffers for the whole tree (`telperion-render/src/foliage.rs:72-83`), so a second instanced mesh is
a renderer change the code rules forbid for a parameter change. Reusing the one element would draw
the frond's blade, and the packed leaf carries a rotation, a position and one uniform scale in
twelve bytes (`foliage/packed.rs:1-20, 98-120`), so no instance can be squashed into a wedge.

**A surface term is the rejected alternative.** A radius modulation in `angular::profile`
(`surface/angular.rs:16-22`), the way `lobes` and `twist_rate` already work, would need no new
buffers. Three things count against it. It emits a smooth bump rather than a base with a lip,
because the modulation only pushes a ring vertex out along its own radial (`surface.rs:480-510`). It
has no access to the canopy rows, so it would author the second spiral the spec asks the design to
avoid, or force `surface::build` and `surface::extent` to take `CanopyParams` and change their
public signatures. And its angular resolution is `max(radial_segments, lobes * 4)`
(`surface.rs:173-175`), 20 on the family defaults, which is four or five vertices for a whole turn
of a 40-frond spiral.

**Axial resolution is the real limit and both routes share it.** A base hangs on a stem node, and a
surface run can only start at a node (`surface/paths.rs:33-50, 62-80`), so the lattice's vertical
pitch is the trunk's own node spacing, `skeleton.growth.stepDistance`, 0.5 m by default
(`colonization.rs:35`). Where `leaf_bases` exceeds the nodes available, several bases share a node
and stand at one height. At 137.508 degrees they are spread right round the trunk rather than
banded, so three or four to a node still reads as a spiral. The two remedies beyond that are a finer
`stepDistance` for the palm, which is fn-82's dial, and splitting the stem's own polyline to put an
attach point at an exact height, which changes the trunk's ring count and is a larger edit. The
design ships the simple form and the still decides.

**Weathering toward the ground.** One row does both. `age` rises from 0 at the newest base to 1 at
the lowest, and `length_k` and the distal radius are both multiplied by `(1 - weathering * age)`,
the same shape the rosette uses for its pitch spread (`foliage/rosette.rs:139-146`). Colour
weathering is already the material layer's and needs nothing here.
`material.orientation_strength` colours "the side away from the sun and the foot of the trunk"
(`material.rs:139-143`) and `material.weathering_strength` greys a weathered face (`:131-133`).

**The crown base.** The rosette spreads its insertions over `rosette_depth` below the apex
(`foliage/rosette.rs:149`). The bases start below that and the spiral index starts at
`rosette_fronds`, so the newest base sits immediately under the oldest living frond and the two
sequences meet without a gap or an overlap. This is checkable and is R3's first assertion.

**The spine's shape, honestly.** With one element and one uniform scale, a spine is the frond's own
leaflet at a smaller size and a steeper pitch. Its disposition is what the capability's meaning
names, "a leaflet hardened into a spine, borne at the base of a frond" (`capability.rs:85-88`), and
its cross-section stays the leaflet's. A spine that has to read as a different shape needs a second
element, which is a renderer change. The limit is stated here so a failing verdict on the R3 still
lands on the right cause.

**Level of detail and the far draw.** Nothing new is needed. `SurfaceMesh.run_table` is ordered by
descending largest sample radius (`surface.rs:24-29`, built at `surface.rs:309-314`), and the
renderer drops runs below a radius threshold by partitioning that table
(`telperion-render/src/wood.rs:290`), with `submit.rs:157-171` enforcing the ordering. A base is the
thinnest run on the tree, so it sorts last and the far draw stops paying for it first. A spine is a
leaflet and coarsens with the crown's own level selection.

**Multi-stem presets.** `stems` validates `1..=6` (`branching/traits.rs:134`). Bases hang on every
stem, visited in `stem_apices` order, each with its own frame from its own apex, exactly as the
rosette stands at every apex (`foliage/rosette.rs:84-102`).

**Mesh budget.** A base is one run of two samples: `2 * segments` ring vertices plus two caps, and
`2 * segments * 6` indices (`surface.rs:180-210`, `surface.rs:283-295`). At the family default 20 segments
(`radial_segments` 12, `lobes` 5, `surface.rs:173-175`), 100 bases cost 4,200 vertices and 24,000
indices, against a palm trunk of roughly 48 rings at 20 segments, near 960 vertices. The bases
dominate the wood mesh in relative terms and are trivial in absolute ones. Overflow is already
refused by the reservation arithmetic (`surface.rs:180-210`), and `MAX_LEAF_BASES` at 256 with
`stems` at 6 caps the pass at 1,536 runs. Spines cost no instances at all, because they replace
leaflets the frond already carried.

## Verification expectations

| Criterion | The test that is red before the implementation |
| --- | --- |
| R1 | `crates/telperion-core/tests/capability.rs`: `the_date_palms_recorded_needs_read_as_three_met_and_three_absent` (`:119-131`) is rewritten to five `Expressed` and `infructescence` alone `Absent`, red on the base where both new names are `Absent`. `DERIVABLE.len()` moves 9 to 11 (`:56`), so `the_nine_names_the_derivation_produces_are_all_in_the_vocabulary` is renamed and `the_derivation_produces_no_name_the_vocabulary_does_not_carry` (`:66-84`) forces the palm's table to set both organs, because the listed presets must between them produce every derivable name. |
| R1, run | The gate rerun at the landed commit over `.flow/evidence/date-palm/pipeline`, expecting the capability detail to name `infructescence` and nothing else (`telperion-jev/src/pipeline/stages/gate.rs:172-222`) and `preset-capability` to report no unproduced name (`:235-250`). |
| R2 | `tests/species.rs:37-75` against an unchanged `tests/species/digests.json`, and `tests/catalogue/pins.rs` against `catalogue/*/pins.json`. Red-before is not the shape, so add the positive control fn-109 added: one test asserting the palm's own wood digest changes when `leaf_bases` is raised from zero, which proves the switch is not inert. |
| R3 | A new `crates/telperion-core/tests/leaf_bases.rs`, modelled on `tests/rosette.rs`: build `Preset::from_id("date-palm")` at seed 1 through `mesh::grow`, and assert (a) the tree carries `stems * leaf_bases` nodes that are neither `stem` nor `Twig` and whose parent is a stem node, (b) the azimuth of base `k` about its stem's axis equals `(rosette_fronds + k) * rosette_divergence` modulo 360 within a tolerance, which is the spiral claim stated as arithmetic, (c) every base stands at or below `rosette_depth` under its apex, (d) base lengths fall monotonically with depth once `leaf_base_weathering` is positive, and (e) at `leaf_bases = 0` the tree is byte-equal to the one `mesh::grow` returns today. A sixth, in `tests/rosette.rs`: with `acanthophylls = n`, the first `n` leaflets of each frond are smaller than the rest by `acanthophyll_length` and at `acanthophylls = 0` every instance is unchanged. |
| R4 | `crates/telperion-jev/tests/dial_table.rs::every_numeric_row_of_every_family_is_a_dial_or_an_excluded_row` (`:101-132`) goes red the moment the eight wire rows land and before `dials.json` gains them. Then `every_authored_row_is_a_dial_the_loop_can_ask_about` (`:134-185`), `every_dial_steps_to_a_value_the_generator_accepts` (`:187-228`) and `every_row_that_switches_a_feature_on_says_what_zero_does` (`:230-245`) over the four new switching rows. |

Gate: `cargo test --profile ci --workspace --no-fail-fast`, run once, at the end
(`CLAUDE.md`, "Gates and checked claims"). Not `npm run rust:test`.

**The owner's eye.** One trunk-view still, the date palm at seed 1, through the headless renderer:
`npm run species:quick` (`package.json:55`). One capture, no forest, under the evidence budget. The
verdict on that still belongs to fn-82 and the owner, never to this spec.

## Remaining unknowns

1. **Does the date palm's stem carry enough nodes for a lattice?** At `stepDistance` 0.5 m and an
   envelope height of 24 m (`colonization.rs:35`, `envelope.rs:35-47`) the stem holds roughly 48
   nodes, so 100 bases stand two to a node. Settled by: one test that grows the palm and counts the
   nodes on its stem run, then the R3 still. If the lattice reads banded, the choice between a finer
   `stepDistance` and splitting the stem's polyline is a system-design call and escalates to the
   host.
2. **Do the bases need a growth-path hook?** `mesh::grow` is the direct build's seam and the growth
   path builds its own tree (`specimen/view.rs`), so a grown specimen would carry no bases, the way
   fn-109 needed `place_rosette` for parity. Under "Mature trees are the product" that may be
   correct as it stands. Settled by the host's word on whether parity is owed; the code is a
   public `clothe_leaf_bases` call either way.
3. **May several base nodes share one `NodeIdentity`?** `..Node::root()` gives every appended node
   the default identity, `birth: u64::MAX` (`tree/identity.rs:12-19`). Nothing in
   `crates/telperion-core/tests/identity.rs` asserts uniqueness and no reader keys on a base's
   identity, but the digest serialises the whole tree (`tests/species.rs:44`). Settled by: grep the
   crate for an identity-uniqueness assertion and, if one exists, mint `birth = u64::MAX - k`.
4. **Should the base's taper be its own row?** The distal radius here is the start radius times the
   weathering term, so an unweathered base is a blunt truncation, which is what a palm boot is. If
   the owner wants a base that narrows independently of its wear, that is a ninth row. Settled by
   the R3 still.
5. **Does `persistent-leaf-base` belong to the rosette?** The derivation above requires
   `rosette_fronds > 0`. A tree that keeps leaf bases without a frond crown exists in nature and
   could not state it here. Widening the clause is a vocabulary judgment about what the name means,
   so it escalates to the host. The code is the same either way.
6. **What does the gate expect the palm's manifest to say?** R1 reads "halts on `infructescence`
   alone, or passes if fn-33's slot has landed". fn-33 is open and this design does not depend on
   it. Settled by the conductor rerunning the stages at the landed commit.

## Implementation complexity, in my reading

**Moderate, and lower than fn-109.** The spine is roughly fifteen lines inside a function that
already does the same job at the rachis's other end. The base pass is one new module with no rng, no
new mesh path, no renderer contact and no prediction to keep in step, and it has two working
templates beside it: `clear_apical_twigs` (`branching.rs:243-280`) for appending and re-reading a
tree, and `branching/local/advance.rs:304-321` for building a node with explicit radii.

Three places will cost more than they look. The node's four radius fields have to satisfy
`validate_range` (`tree.rs:139-155`), which refuses a zero radius and refuses `start_radius <
radius`, so the weathering term needs a floor and the right ordering. The base has to carry
`NodeKind::Branch`, because `bearing_runs` clothes every twig node with foliage regardless of
radius (`foliage/placement.rs:505-512`), and that trap stays invisible until a family turns the
rosette off and the bases on. And the capability edit cascades into three assertions in
`tests/capability.rs` written around nine derivable names, plus the length of `SWITCHES_AT_ZERO`
(`dial_table.rs:35-62`), which moves 28 to 32. The eight rows in six places each are the bulk of
the diff and none of the difficulty.
