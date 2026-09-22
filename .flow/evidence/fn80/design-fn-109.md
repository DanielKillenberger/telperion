# Design handoff — fn-109, the apical rosette

Written for fn-80 dispatch-4 (role design, tier high_reasoning, effort medium), 2026-09-22.
Contract: `.flow/specs/fn-109-the-apical-rosette-fronds-borne-only-at.md`, R1 to R4.
Source of the need: `.flow/evidence/date-palm/pipeline/packet/capability.json`, the two
`unsupported-anatomy` rows `apical-rosette` and `pinnate-frond`.

No code is written here. Signatures only.

## What is built

One shared generator capability in two halves, both off by default and both value-table rows.

**The rosette is a third foliage source.** Today a crown has two sources: the station walk over
the runs `placement::runs` returns (`crates/telperion-core/src/foliage/placement.rs:216-224`) and
the short-shoot walk (`foliage/placement.rs:308`, implemented in `foliage/short_shoots.rs:128-164`).
The rosette is a third, built on the short-shoot pattern exactly: it places matrices on wood the
skeleton already has, adds no node and no run, draws each frond from a stream keyed by the bearing
node's identity, and visits wood in identity order so storage order cannot move a frond
(`foliage/short_shoots.rs:204`, `:263-269`).

The rosette stands at the apex of every stem — the childless tip of each order-zero axis
(`branching/scaffold.rs:182-188` sets `stem: !crown`, so an order-zero axis is a stem). It bears
`rosette_fronds` placements on a phyllotactic spiral about the axis, spread over `rosette_depth`
metres below the apex, each leaning from the axis by a pitch that runs from the youngest frond to
the oldest across `rosette_pitch_spread`. When `rosette_fronds > 0` the rosette is the tree's only
foliage source: `runs` returns empty and the short-shoot walk is skipped.

**The frond is a pinnate grouping at the placement, not a compound element.** fn-33 already decided
this and the decision holds: `.flow/specs/fn-33-flowers-cones-and-compound-leaves-as.md:28` and
`:88` — "a multi-blade element breaks the level ladder's base, widest and tip anchors and would
count as one unit; expanding at the placement keeps the element single and the counting honest".
The code agrees: there is exactly one `Element` per tree (`mesh.rs:82`, `mesh.rs:101`), its level
ladder is built from one base-to-tip section ladder (`foliage/levels.rs:70-83`, `:172-186`), and
`Element::validate` requires the anatomy's sections to be disjoint and increasing
(`foliage/element.rs:121-143`). So one placement expands into `leaflet_count` leaflet instances
strung along a rachis line from the station, alternating sides at `leaflet_pitch`, the line bent by
`rachis_arch`, with a terminal leaflet when `terminal_leaflet` is set. The element stays one blade.
fn-109's spec text "each frond one pinnate element" is read as its own line 12, "one frond as
leaflets along a rachis in **one placement**".

Both names then fall out of the value table alone: `apical-rosette` where the rosette is on,
`pinnate-frond` where the pinnate grouping is on **and** the rosette is on (the rosette is what
makes it a frond rather than the ash's compound leaf).

## Interfaces

### New parameter rows

All five placement rows and all five grouping rows go on `CanopyParams`
(`foliage/placement.rs:15-90`), not on `ElementParams`. Reason: the expansion is a placement rule,
the element stays one blade, and `place`, `place_on_surface`, `leaf_count`
(`foliage/placement.rs:156`, `:169`, `:250`), `Reference::from_params` (`foliage/reference.rs:44`)
and `Placement::new` (`foliage/timeline.rs:88`) already carry `CanopyParams` and carry no
`ElementParams` — putting the rows on `element` would add a parameter to five signatures and
roughly a dozen call sites (`mesh.rs:84`, `specimen/view.rs:68-99`, four examples, five test files).
This diverges from fn-33's authored names (`element.leaflet_count`, …,
`fn-33 …:36`); see Remaining unknowns.

| Wire path | Rust field | Type | Validated range | Default | Doc comment |
| --- | --- | --- | --- | --- | --- |
| `/canopy/rosetteFronds` | `canopy.rosette_fronds` | `u32` | `0..=128` | `0` | Fronds the rosette bears at the apex of each stem. At zero no rosette stands and the canopy clothes wood as it always did; any rise makes the rosette the tree's only foliage. |
| `/canopy/rosetteDivergence` | `canopy.rosette_divergence` | `f64` | `-1e9..=1e9` | `137.508` | The degrees each successive frond is turned about the apex. |
| `/canopy/rosettePitch` | `canopy.rosette_pitch` | `f64` | `0..=180` | `45.` | Degrees from the axis the youngest frond stands: 0 upright, 90 level, 180 hanging. |
| `/canopy/rosettePitchSpread` | `canopy.rosette_pitch_spread` | `f64` | `0..=180` | `60.` | How many degrees further than the youngest the oldest frond leans, so the crown opens from a spike to a skirt. |
| `/canopy/rosetteDepth` | `canopy.rosette_depth` | `f64` | `0..=100` | `0.` | Metres below the apex the frond insertions are spread down the axis. At zero every frond leaves one point. |
| `/canopy/leafletCount` | `canopy.leaflet_count` | `u32` | `1..=256` | `1` | Leaflets one placement carries along its rachis. One is the single blade every family drew. |
| `/canopy/rachisLength` | `canopy.rachis_length` | `f64` | `0..=1e3` | `0.` | Metres of rachis the leaflets are strung along. At zero the placement is one blade whatever the count says. |
| `/canopy/leafletPitch` | `canopy.leaflet_pitch` | `f64` | `0..=90` | `45.` | The degrees a leaflet leaves its rachis. |
| `/canopy/rachisArch` | `canopy.rachis_arch` | `f64` | `-1..=1` | `0.` | How far the rachis bends out of the straight line from its station, as a share of its length. Positive arches up, negative droops. |
| `/canopy/terminalLeaflet` | `canopy.terminal_leaflet` | `f64` | `0..=1` | `0.` | Whether a single leaflet closes the rachis's end, blended 0 to 1 as fn-33 asks. |

Validation lands in the `for (v, l, h, n)` table at `foliage/placement.rs:322-341` for the floats and
beside the `p.clump > 64` check at `:342` for the two counts; each refusal names its own row, as
`short_shoots::validate` does (`foliage/short_shoots.rs:39-52`).

Wire and dial table:
- One `$op!` line per row in the `fields!` macro, in the `"canopy"` block at `params.rs:108-129`.
  That one edit gives encode, decode, unknown-key refusal, overlay and the browser metadata
  (`params.rs:284-361`).
- Ten rows in `crates/telperion-jev/data/dials.json` with `group: "canopy"`, `score_visible: true`,
  `meaning_basis: "doc comment"`, `range_basis: "validated bound"`, `source:
  "crates/telperion-core/src/foliage/placement.rs:<line>"`, and `small < substantial <= max-min`
  with `small <= (max-min)/2` (`crates/telperion-jev/tests/dial_table.rs:142-151`).
- `rosette_fronds` and `rachis_length` join `SWITCHES_AT_ZERO`
  (`dial_table.rs:35-62`): the code guards on both, so each must have `min: 0` and a meaning whose
  text contains the word "zero" (`dial_table.rs:230-245`). `leaflet_count` does not — its neutral
  is 1, not 0.

### Layer boundaries

Branching hands the rosette nothing new; the rosette reads the solved `Tree`. A new module
`crates/telperion-core/src/foliage/rosette.rs`, sibling to `short_shoots.rs` and under the 400-line
rule:

```rust
/// One rosette: the apex it crowns, the point it leaves and the axis it stands on.
pub struct Rosette { pub apex: usize, pub at: Vec3, pub axis: Vec3 }

/// Every stem apex this tree offers, in identity order.
pub fn rosettes(tree: &Tree, p: &CanopyParams) -> Vec<Rosette>;

/// Every row on its rail, each refused by its own name.
pub(super) fn validate(p: &CanopyParams) -> Result<()>;

/// Leaves the rosettes place before any cull: rosettes * fronds * leaflets.
pub(super) fn count(tree: &Tree, p: &CanopyParams) -> Result<usize>;

/// Hang every rosette's fronds into an already-sized crown.
pub(super) fn clothe(
    tree: &Tree, seed: u32, p: &CanopyParams,
    out: &mut Instances, owners: Option<&mut Vec<u32>>,
) -> Result<()>;

/// `clothe` for the growth path, which places its own recorded leaves first.
pub fn place_rosette(
    tree: &Tree, seed: u32, p: CanopyParams, out: &mut Instances,
) -> Result<()>;
```

The frond expansion is one function the station walk and the rosette both call, so there is one
implementation of "a placement becomes leaflets":

```rust
/// The matrices one placement stands for: one when the grouping is off, else
/// `leaflet_count` along the rachis from `point`, plus a terminal leaflet.
pub(super) fn fan(
    point: Vec3, axis: Vec3, face: Vec3, p: CanopyParams, rng: &mut Rng,
) -> Result<impl Iterator<Item = [f32; 16]>>;
```

Changed call sites, all signature-preserving:
- `placement::runs` (`foliage/placement.rs:216`) returns `Vec::new()` when `p.rosette_fronds > 0`.
- `placement::leaves_on` (`:226-245`) adds `rosette::count`, and `short_shoots::count` is skipped
  when the rosette is on; `leaf_count` (`:250`) and `footprint::predict` (`footprint.rs:55-60`)
  inherit it with no edit, which keeps the prediction and the reservation one number
  (`foliage/placement.rs:281-285`).
- `placement::place_impl` (`:263-313`) calls `rosette::clothe` where it calls
  `short_shoots::clothe` today, after the run loop and before the clumping thin.
- `specimen/view.rs:90-99` calls `place_rosette` beside `place_short_shoots`, so the growth path
  stays buildable and draws the same crown without becoming a gate on anything.
- `reference::reach` (`foliage/reference.rs:61-97`) gains one arm: when `canopy.rosette_fronds > 0`,
  `reach` is at least `rosette_depth + rachis_length + element reach`. This is not optional — a
  station outside the box is clamped in release and trips a `debug_assert` in test
  (`foliage/packed.rs:101-110`).

### Capability vocabulary

`crates/telperion-core/src/capability.rs`:
- Move `apical-rosette` (`:76-78`) and `pinnate-frond` (`:79-82`) from `UNEXPRESSED` into
  `EXPRESSED`, meanings unchanged. That is the one line `docs/species-pipeline.md:316-322` asks for,
  and it moves the vocabulary version by construction (`capability.rs:143-161`).
- Add both to `DERIVABLE` (`:101-108`). This is load-bearing: the gate only checks a table produces
  a name when the name is in `DERIVABLE` (`crates/telperion-jev/src/pipeline/stages/gate.rs:235-239`).
- `derived` (`:185-209`) gains two clauses, thresholds on shipped values like every clause above them:

```rust
if f.canopy.rosette_fronds > 0 { produced.push("apical-rosette"); }
if f.canopy.leaflet_count > 1 && f.canopy.rachis_length > 0.
    && f.canopy.rosette_fronds > 0 { produced.push("pinnate-frond"); }
```

## Invariants

1. **Byte-identity with the rosette absent.** `rosette_fronds == 0` and `leaflet_count == 1` are the
   defaults, and every guard is on those two. No shipped preset sets either
   (`presets/species.rs:324-335` is the whole date-palm table today). The proof is the committed
   digest of every fixed seed of every species, `crates/telperion-core/tests/species/digests.json`
   (silver-birch, oregon-white-oak, european-beech, norway-spruce × seeds 1, 2, 3, 5, 8), checked by
   `tests/species.rs:37-72`; plus `catalogue/<id>/pins.json` through `tests/catalogue/pins.rs`. The
   subtle one is `Reference::of` (`foliage/reference.rs:31-40`): the box is quantisation input, so a
   `reach` arm that fires at the default would move every packed leaf of every preset. Its new arm
   must be guarded on `rosette_fronds > 0`.
2. **No species branch.** Nothing in `foliage/`, `branching/` or the renderer reads a preset id. The
   rosette fires on a value, the frond fires on a value, and the date palm is a value table like any
   other (`CLAUDE.md`, "Code rules"; `presets::Preset::parameters` is the only species surface).
3. **No foliage on the stem below the apex.** `runs` empty plus the short-shoot walk skipped, both
   on `rosette_fronds > 0`. Note what this does *not* do: it suppresses foliage, not twig wood. See
   Difficult cases.
4. **No renderer change.** The renderer receives one element and a flat instance list
   (`mesh.rs:96-107`, `telperion-render/src/foliage.rs:197-229`). A leaflet is an instance; the
   element is unchanged; `MAX_LEVELS` is 16 (`telperion-render/src/select.rs:32`) and the ladder is
   untouched. `CLAUDE.md`'s "supported parameter changes must require no renderer code changes"
   holds.
5. **Determinism.** Every frond's transform comes from a stream keyed by the apex's identity, the
   frond index and the leaflet index, in the shape of `short_shoots::key`
   (`foliage/short_shoots.rs:263-269`). No storage order, no build order, no global counter.
6. **Counting stays honest.** A frond is one placement, a leaflet is one instance. `leaf_count` and
   `place` agree by construction because both go through `leaves_on`
   (`foliage/placement.rs:226-261`, `:281-285`).

## Difficult cases

**Radius-keyed placement and the twig layer.** `bearing_runs` clothes any node with
`kind == NodeKind::Twig` regardless of `shoot_radius` (`foliage/placement.rs:421-428`), and
`mesh::assemble` always passes `Some(TwigPlacement)` (`mesh.rs:88-92`), so a palm gets a twig layer
whether or not its table asks for one — `twigs.generations` validates `1..=6` with no off value
(`twigs.rs:5`, `capability.json`, the `canopy.shoot_radius / twigs` row). That is why the rosette
must switch `runs` off rather than rely on `shoot_radius = 0`. The twig *wood* is the skeleton's,
not the canopy's, and is not suppressed by this design — see the unknown below.

**The frond versus the one-blade element layer.** Settled by fn-33's recorded decision and by the
code: one `Element` per tree (`mesh.rs:82`), and `levels::build` reads a single base-to-tip section
ladder with base/widest/tip anchors (`foliage/levels.rs:172-186`). A compound element would either
have to abandon its anatomy (losing `species_metrics`' leaf measurement, which reads
`anatomy.vertices` and `anatomy.sections` — `examples/species_metrics/mod.rs:243-281`) or home every
leaflet vertex onto a rachis section (`foliage/levels.rs:146-166`, an O(vertices × section vertices)
fallback). Expanding at the placement avoids both.

**Level of detail and the far draw.** Nothing new is needed. The far draw is the near draw minus
what the eye cannot resolve, and at distance a frond's leaflets are instances the existing level
selection coarsens one blade at a time (`telperion-render/src/select.rs`,
`telperion-render/src/foliage.rs:266-309`). The rachis, drawn as the leaflets' own connector
geometry (fn-33's first candidate, `fn-33 …:29`), coarsens with them. The cost is instance count,
not ladder depth.

**The growth path.** It stays hidden and buildable: `specimen/view.rs:79-105` rebuilds instances
from recorded placements and then calls the short-shoot source live. `place_rosette` is called the
same way and from the same place. `timeline::Placement::new` (`foliage/timeline.rs:88-117`)
validates the canopy rows through `place` on an empty tree, so the new rows are rejected there by
name with no edit. fn-33's leaflet index in the placement identity is *not* needed here, because the
rosette is drawn live from the wood like a short shoot and never recorded.

**Mesh budget.** A date palm at 40 fronds × 120 leaflets is 4,800 instances, 12 bytes each
(`foliage.rs:84-100`) — about 58 KB, against `max_instances` defaulting to `usize::MAX`
(`foliage/placement.rs:118`) and crowns already measured in millions
(`foliage.rs:214-218`). The ceiling that matters is the product `rosettes × fronds × leaflets`,
which `rosette::count` must compute with `checked_mul` and refuse as
`Error::ResourceLimit("foliage instance budget")`, matching `station_count`
(`foliage/station.rs:144-162`).

**Multi-stem presets.** `stems` validates `1..=6` (`branching/traits.rs:134`) and a clumped date
palm is real. The rosette therefore stands at *every* stem apex, not one, visited in identity order.
`stem_fork_height` puts later stems partway up the first (`branching/traits.rs:68-71`), so "apex"
must be defined structurally — a childless node whose axis is order zero — not as "the highest
node".

## Verification expectations

| Criterion | The test that is red before the implementation |
| --- | --- |
| R1 | `tests/capability.rs`: the existing `the_date_palms_recorded_needs_read_as_one_met_and_five_absent` (`:118-132`) is rewritten to assert `Expressed` for the two and `Absent` for the remaining three — red on the base, where both are `Absent`. Beside it, `derived(Preset::from_id("date-palm"))` contains both names. Also `DERIVABLE.len()` moves 6 → 8 (`tests/capability.rs:55`) and `"date-palm"` joins that file's `PRESETS` list (`:9-17`), because `the_derivation_produces_no_name_the_vocabulary_does_not_carry` (`:66-83`) asserts the listed presets between them produce **every** derivable name and only the palm produces these two. European beech is already listed although it is `IN_WORK` (`params.rs:259-262`), so the precedent exists. |
| R1, run | The gate rerun at the landed commit: `species-pipeline` over `.flow/evidence/date-palm/pipeline`, expecting the capability detail to name `acanthophyll, persistent-leaf-base, infructescence` and nothing else (`telperion-jev/src/pipeline/stages/gate.rs:172-222`), and `preset-capability` to report no `unproduced` name (`:235-250`). |
| R2 | `cargo test --profile ci --workspace --no-fail-fast` with `tests/species.rs` unchanged and `tests/species/digests.json` not re-pinned; `tests/catalogue/pins.rs` over `catalogue/*/pins.json`. Red-before is not the shape here, so add the positive control: one test asserting the palm's digest **changes** when `rosette_fronds` is raised from 0, proving the switch is not inert code. |
| R3 | A new `tests/rosette.rs`: build `Preset::from_id("date-palm")` at seed 1 through `branching::generate` + `foliage::place`, and assert (a) the placed count is `stems × rosette_fronds × leaflet_count` exactly, (b) every instance position sits within `rosette_depth + rachis_length` of a stem apex, so nothing is borne lower down the stem, and (c) with `rosette_fronds = 0` the same family places what it places today. Red on the base: today the palm's crown is the terminal twig's blades at the wrong count and the wrong place. Then `npm run species:measure` for `species_measure` inside fn-82's gates. |
| R4 | `crates/telperion-jev/tests/dial_table.rs::every_numeric_row_of_every_family_is_a_dial_or_an_excluded_row` (`:101-132`) goes red the moment the ten wire rows land and before `dials.json` gains them — it reads `CATALOGUE` and `IN_WORK`, so the date palm is covered. Then `every_authored_row_is_a_dial_the_loop_can_ask_about` (`:134-185`), `every_dial_steps_to_a_value_the_generator_accepts` (`:187-228`) and `every_row_that_switches_a_feature_on_says_what_zero_does` (`:230-245`). |

Gate: `cargo test --profile ci --workspace --no-fail-fast`, run once, at the end
(`CLAUDE.md`, "Gates and checked claims"). Not `npm run rust:test`.

**The owner's eye.** One still, the date palm at seed 1, through the headless renderer:
`npm run species:quick` (`package.json:55`, `cargo build --release -p telperion-render --example
headless && node tests/species.mjs`). One capture, no forest, per the budget rule. The verdict on
that still is fn-82's and the owner's, never this spec's.

## Remaining unknowns

1. **Does the palm's apex grow twig wood that the rosette should also suppress?** The capability
   assessment says "a childless tip above `trunk_height` still gets a terminal twig"
   (`capability.json`, the `canopy.shoot_radius / twigs` row), and this design suppresses twig-borne
   *foliage*, not twig *geometry*. Settled by: one test that runs `branching::generate` on the
   date-palm family and counts nodes with `kind == NodeKind::Twig`. If the count is non-zero, the
   host decides whether a bare twig at the apex under the rosette is acceptable or whether a
   separate row is owed — that is a system-design call and escalates.
2. **Do the pinnate rows live on `canopy` or on `element`?** fn-33 (open) authored them as
   `element.leaflet_count`, `element.rachis_length`, `element.leaflet_pitch`,
   `element.terminal_leaflet` (`fn-33 …:36`, `:60`). This design puts them on `canopy` to keep five
   signatures and a dozen call sites unchanged. Settled by the host deciding whether fn-33 yields
   its leaflet half to fn-109 and adopts the `canopy` names, or fn-109 pays the signature change.
   Whichever way, the rows must be authored once, not twice.
3. **Does fn-109 also move `pinnate-compound` into `EXPRESSED`?** Its meaning — "one leaf divided
   into leaflets along a rachis and borne as a single placement" (`capability.rs:71-74`) — is
   exactly what this implementation draws, minus the rosette. Leaving it in `UNEXPRESSED` makes the
   vocabulary say the generator cannot draw something it can. Moving it widens the spec and changes
   the ash's gate (fn-56). Settled by the host's word; the code is the same either way.
4. **What does `species_measure` report for a frond?** `species_metrics` measures one element's
   anatomy and reports units per instance (`examples/species_metrics/mod.rs:243-306`). With the
   grouping at the placement, a leaflet is the unit and a frond is not measured as one object, so a
   leaf-area or leaf-length gate authored for a frond would read the leaflet. Settled by fn-82
   stating which number its gates hold, which is fn-82's to say.
5. **What the rosette's pitch does at `rosette_depth = 0` on a leaning stem.** `stem_lean` tilts
   outer stems up to 45° (`branching/traits.rs:140`), and the fronds are pitched from the axis, not
   from vertical. Whether a leaning palm's rosette should follow its own axis or the sky is a
   botanical judgment. Settled by: the axis is the default here; an owner verdict on the still can
   ask for the other.

## Implementation complexity, in my reading

**Complex, but for a bounded reason.** The new code itself is straightforward and has a working
template beside it: `foliage/rosette.rs` is `foliage/short_shoots.rs` with a different walk, and the
pinnate fan is one function over one placement. Neither touches the level ladder, the element, the
packed leaf, the renderer or the growth path's recorded placements.

What makes it complex is surface, not logic. Ten rows have to be authored in five places each
(struct field, default, validation, `fields!`, `dials.json`); the byte-identity invariant has a
non-obvious trap in `Reference::reach`, where a well-meant unguarded arm silently re-quantises every
leaf of every shipped preset; the capability vocabulary edit cascades into three assertions in
`tests/capability.rs` that were written assuming six derivable names and seven presets; and two of
the five unknowns above (the pinnate rows' home, `pinnate-compound`) are coordination with an open
spec rather than code. A competent implementer should expect the guard placement and the test
cascade to cost more than the rosette walk.

One friction note for the host: the fn-109 spec's phrase "each frond one pinnate element" reads, on
its face, as the compound-element design that fn-33 explicitly rejected and that the level ladder
does not support. The spec's own line 12 ("in one placement") resolves it, but the sentence should
be restated before an implementer takes the literal reading.
