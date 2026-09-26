# Parameter reference

Generated from the parameter catalogue (`crates/telperion-core/src/catalogue.rs`)
by `every_row_is_in_the_reference` in `crates/telperion-core/tests/parameter_reference.rs`;
run it with `TELPERION_WRITE_REFERENCE=1` to rewrite this file. Every wire row
is declared once, beside the code that reads it; edit the declaration, never this file.

Each row lists its type, unit, bounds and the ordinary family's default; the
stages that read it on the direct build (grow, plan, expand, cull, draw) and
how the growth path reads it; where it is refused and by what name; where it
lies dormant; its couplings and conflicting bounds; how a walk between two
families moves it; and the tuning dial it offers. A deprecated row is read by
no production stage and is kept on the wire for compatibility.

## `/ (the family)`

### `/age`

The specimen's age in years. Only the growth path reads it: the direct build is the mature tree whatever the row says.

- real, years, [0, 1000000], default 100
- read by no stage of the direct build; growth path: only the growth path reads it
- checked: by a named check or not at all (see note)
- note: Refused by `Age::from_years` in `params::parse` and `Family::validate`, which also quantises it to the growth path's tick; the direct build is the mature tree whatever it says.
- blend: linear, clamped to its ends; dial: none: Identity of a grown specimen, not a look: only the growth path reads it, and the direct build the tuner measures is the mature tree whatever it says (crates/telperion-core/src/branching/specimen/timeline.rs:30).

### `/shellDepth`

How deep into the crown leaves are kept, as a share of the crown's widest radius; a leaf further in than that is dropped. Raising it keeps more of the crown's interior foliage, and one keeps it all.

- real, share of the crown's widest radius, [0, 1], default 0.45
- read by cull; growth path: measured against the envelope at the specimen's age
- checked: Shell site, rank 0, as invalid input `shell depth`
- note: `foliage::cull` and the GPU executor's preparation check it again; the planned field counts stations before the cull and does not read it.
- blend: linear; dial: `shell_depth` (how deep into the crown leaves are kept, as a share of its widest radius), window its bounds (validated bound), steps 0.15 and 0.3

## `/growth`

### `/growth/workBudget`

Geometric work quanta available over this specimen's life.

- count (u32), work quanta, [1, 4294967295], default 250000
- read by no stage of the direct build; growth path: only the growth path reads it
- checked: Growth site, rank 0, as invalid value `growth.workBudget`
- note: Also sets when growth snaps to maturity, so it moves the growth path's mature year.
- blend: rounded to the nearest; dial: none: Growth-path only, and a work budget.

### `/growth/rate`

Chapman–Richards rate, in inverse years.

- real, per year, [0.001, 10], default 0.08
- read by no stage of the direct build; growth path: only the growth path reads it
- checked: Growth site, rank 1, as invalid value `growth.rate`
- note: Also ages shoot vigour for survival and bud sampling.
- blend: linear, clamped to its ends; dial: none: Growth-path only: `mesh::build` and `branching::generate` never read the growth traits (crates/telperion-core/src/mesh.rs:73).

### `/growth/shape`

Chapman–Richards shape; values above one give a sigmoidal height curve.

- real, -, [1, 8], default 2
- read by no stage of the direct build; growth path: only the growth path reads it
- checked: Growth site, rank 2, as invalid value `growth.shape`
- blend: linear, clamped to its ends; dial: none: Growth-path only, as above.

### `/growth/sheddingTolerance`

Consecutive active slices below the habit shedding threshold, in years.

- real, years, [0, 1000000], default 2
- read by no stage of the direct build; growth path: only the growth path reads it
- checked: Growth site, rank 5, as invalid value `growth.sheddingTolerance`
- dormant: `sheddingThreshold` zero
- blend: linear, clamped to its ends; dial: none: Growth-path only, as above.

### `/growth/apicalControlLoss`

Annual loss of the habit apical control (zero retains its authored value).

- real, per year, [0, 10], default 0
- read by no stage of the direct build; growth path: only the growth path reads it
- checked: Growth site, rank 6, as invalid value `growth.apicalControlLoss`
- note: Divides `apicalDominance` each year and adds the released share to `lateralLengthRatio`.
- blend: linear, clamped to its ends; dial: none: Growth-path only, as above.

### `/growth/leafLifetime`

Years of annual foliage cohorts held by a living shoot; zero bears none.

- real, years, [0, 1000000], default 1
- read by no stage of the direct build; growth path: only the growth path reads it
- checked: Growth site, rank 3, as invalid value `growth.leafLifetime`
- dormant: zero bears no leaves on the growth path
- blend: linear, clamped to its ends; dial: none: Growth-path only, as above.

### `/growth/resizeTolerance`

Minimum thickening in metres before recording another annual radius frame.

- real, m, [0, 1], default 0.0001
- read by no stage of the direct build; growth path: only the growth path reads it
- checked: Growth site, rank 4, as invalid value `growth.resizeTolerance`
- blend: linear, clamped to its ends; dial: none: Growth-path only, and an internal recording tolerance rather than a look.

## `/skeleton`

### `/skeleton/seed`

The specimen: every stage keys its random stream by it, so another seed draws another tree of the same family.

- count (u32), -, [0, 4294967295], default 42
- read by grow, plan, expand; growth path: as the direct build
- checked: by a named check or not at all (see note)
- note: It reshapes the crown outline only where `irregularity` is above zero. Any u32 is a seed; nothing checks it.
- blend: the first family's; dial: none: Identity: the seed names which tree is drawn, and the protocol fixes it. Stepping it is not a tuning move.

### `/skeleton/attractors`

How many pull points are scattered through the crown for the branches to grow toward. Raising it fills the crown with more and finer branching; at an `attractor_weight` of zero none are scattered and the row does nothing.

- count (usize), points, [0, 1000000], default 500
- read by grow; growth path: as the direct build
- checked: Sampling site, rank 1, as invalid input `attractors`
- dormant: `attractorWeight` zero
- note: `attractorWeight` above zero with none is refused (`attractor weight and attractor count`); the scattered count sets the default `influenceRadius`.
- blend: rounded to the nearest; dial: `attractors` (pull points scattered through the crown for the branches to grow toward), window [1, 1000000] (validated bound), steps 100 and 200, presets span [250, 750]

### `/skeleton/samplingAttemptsPerAttractor`

How many random tries the sampler may spend on each pull point before it gives up. Raising it lets a narrow or deeply lobed crown reach its full count of points instead of settling for fewer.

- count (u32), tries, [1, 4294967295], default 64
- read by grow; growth path: as the direct build
- checked: Sampling site, rank 0, as invalid value `samplingAttemptsPerAttractor`
- dormant: `attractorWeight` zero
- note: Too few tries for the envelope's shape is a hard error at the scatter.
- blend: rounded to the nearest; dial: none: A sampling budget: it decides how hard the sampler looks for room for a pull point, not what the tree looks like (crates/telperion-core/src/envelope.rs:170).

### `/skeleton/step`

How far the crown grows in one step, as a share of the tree's height; the distance at which a pull point is used up is twice it. Raising it grows the crown in longer, coarser strides.

- real, share of height, (0, ∞], default 0.022
- read by grow; growth path: as the direct build
- checked: Step site, rank 0, as invalid input `growth step`
- note: Sets the default step, kill and influence distances (`default_growth`); an overriding `stepDistance` leaves kill and influence on `height·step`.
- blend: linear; dial: `step` (how far the crown grows in one step, as a share of the height), window [0.011, 0.033] (capped: capped both ways: the generator validates no closed range here (only positive and finite); the row stays at the preset span until the generator authors one), steps 0.0025 and 0.005, presets span [0.011, 0.033]

## `/skeleton/habit`

### `/skeleton/habit/reachProbeSteps`

Samples available to find an axis's room within the crown.

- count (u32), samples, [1, 4294967295], default 96
- read by grow; growth path: as the direct build
- checked: Habit site, rank 0, as invalid value `reachProbeSteps`
- dormant: needs `lateralOrders` of one or more: only first-order laterals probe their room
- blend: rounded to the nearest; dial: none: A sampling budget: samples spent finding an axis's room, not a look the owner can ask for (crates/telperion-core/src/branching/traits.rs:15).

### `/skeleton/habit/apicalDominance`

How far the leader persists into the crown, 0 to 1.

- real, share, [0, 1], default 0.5
- read by grow; growth path: also splits the structural and local budget, and `apicalControlLoss` divides it each year
- checked: Habit site, rank 1, as invalid input `apical dominance`
- note: The leader runs to `bole + (height - bole)·apicalDominance`, the bole being `max(trunkHeight, height·crownBase)`.
- blend: linear; dial: `apical_dominance` (how far the leader keeps going into the crown), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/habit/whorlStrength`

Clustering of laterals at a station against scattering along the axis.

- real, share, [0, 1], default 0.3
- read by grow; growth path: as the direct build
- checked: Habit site, rank 2, as invalid input `whorl strength`
- dormant: needs `lateralOrders` of one or more
- blend: linear; dial: `whorl_strength` (how tightly laterals cluster at one station rather than scattering along the axis), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/habit/leaderInternode`

Spacing between lateral stations on the leader, in metres.

- real, m, (0, ∞], default 1.5
- read by grow; growth path: as the direct build
- checked: Habit site, rank 3, as invalid input `leader internode`
- note: Also the unit stems are placed in.
- blend: linear; dial: `leader_internode` (metres between lateral stations on the leader), window [0.05, 3.45] (capped: capped both ways: the generator validates no closed range here (only positive and finite); the row stays at the preset span until the generator authors one), steps 0.5 and 1, presets span [0.05, 3.45]

### `/skeleton/habit/lateralsPerStation`

Laterals borne by one station of the leader.

- count (u32), laterals, [1, 12], default 3
- read by grow; growth path: as the direct build
- checked: Habit site, rank 4, as invalid input `laterals per station`
- dormant: needs `lateralOrders` of one or more; leader stations only
- blend: rounded to the nearest; dial: `limbs` (limbs born at each station), window [1, 4] (authored), steps 1 and 2

### `/skeleton/habit/lateralPitch`

Initial angle of a lateral from its parent axis, in degrees; on the upright leader this is degrees from vertical.

- real, degrees, [0, 180], default 60
- read by grow; growth path: as the direct build
- checked: Habit site, rank 5, as invalid input `lateral pitch`
- dormant: needs `lateralOrders` of one or more
- blend: degrees along the shorter arc; dial: `lateral_pitch` (the degrees a lateral leaves its parent axis, from vertical on the leader), window its bounds (validated bound), steps 20 and 40, presets span [0, 118]

### `/skeleton/habit/pitchVariation`

Spread of the lateral pitch, in degrees.

- real, degrees, [0, 90], default 15
- read by grow; growth path: as the direct build
- checked: Habit site, rank 6, as invalid input `lateral pitch variation`
- dormant: needs `lateralOrders` of one or more
- blend: degrees along the shorter arc; dial: `pitch_variation` (the degrees that departure angle varies lateral to lateral), window its bounds (validated bound), steps 5 and 10, presets span [0, 28]

### `/skeleton/habit/risePrimary`

Signed bend over the length of a first-order axis; positive rises.

- real, -, [-1, 1], default 0.05
- read by grow; growth path: as the direct build
- checked: Habit site, rank 7, as invalid input `primary rise per order`
- note: Acts at orders zero and one, so it bends leaning stems; an upright stem has no rise to take.
- blend: linear; dial: `rise_primary` (the bend over a first-order limb's length; positive rises), window its bounds (validated bound), steps 0.025 and 0.05, presets span [-0.03, 0.17]

### `/skeleton/habit/riseSecondary`

Signed bend over the length of a deeper axis; negative hangs.

- real, -, [-1, 1], default 0
- read by grow; growth path: as the direct build
- checked: Habit site, rank 8, as invalid input `secondary rise per order`
- dormant: needs `lateralOrders` of two or more
- blend: linear; dial: `rise_secondary` (the bend over a deeper axis's length; negative hangs), window its bounds (validated bound), steps 0.25 and 0.5

### `/skeleton/habit/crookedness`

Heading change between successive growth units, in degrees.

- real, degrees, [0, 60], default 12
- read by grow; growth path: as the direct build
- checked: Habit site, rank 9, as invalid input `crookedness`
- note: Starts above `trunkHeight` in the scaffold, and also makes the local twig layer wander.
- blend: degrees along the shorter arc; dial: `crookedness` (limb turning and zigzag amount), window [0, 15] (authored), steps 3 and 6

### `/skeleton/habit/lateralSpacing`

Spacing between lateral stations away from the leader, in metres.

- real, m, (0, ∞], default 0.9
- read by grow; growth path: as the direct build
- checked: Habit site, rank 10, as invalid input `lateral spacing`
- dormant: needs `lateralOrders` of one or more
- blend: linear; dial: `lateral_spacing` (metres between lateral stations away from the leader), window [0.000001, 2.925] (capped: capped both ways: the generator validates no closed range here (only positive and finite); the row stays at the preset span until the generator authors one), steps 0.5 and 1, presets span [0.000001, 2.925]

### `/skeleton/habit/lateralLengthRatio`

Length of a lateral against its supporting axis.

- real, share, [0, 1], default 0.4
- read by grow; growth path: `apicalControlLoss` adds the dominance it releases to it
- checked: Habit site, rank 11, as invalid input `lateral length ratio`
- dormant: needs `lateralOrders` of two or more: first-order laterals take their reach from the probe
- blend: linear; dial: `lateral_length_ratio` (a lateral's length against the axis that bears it), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/habit/lateralOrders`

Depth of rule-built orders below the leader.

- count (u32), orders, [0, 8], default 3
- read by grow; growth path: as the direct build
- checked: Habit site, rank 12, as invalid input `lateral orders`
- note: At zero every other lateral row lies dormant.
- blend: rounded to the nearest; dial: `lateral_orders` (how many rule-built orders grow below the leader), window its bounds (validated bound), steps 1 and 2

### `/skeleton/habit/attractorWeight`

Weight of the attractor pull beside the axis's own rule heading. At zero no pull points are scattered and every axis holds its own rule heading, and any rise switches the pull on.

- real, share, [0, 1], default 1
- read by grow; growth path: as the direct build
- checked: Habit site, rank 13, as invalid input `attractor weight`
- note: At zero no pull point is scattered, so `attractors`, `samplingAttemptsPerAttractor`, `influenceRadius` and `killDistance` lie dormant.
- blend: linear; dial: `attractor_weight` (how far the pull points steer an axis beside its own rule heading; at zero no pull points are scattered and every axis holds its own rule heading, and any rise switches the pull on), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/habit/twigTipTaper`

Distal twig radius against the nominal twig radius.

- real, share, (0, 1], default 1
- read by grow; growth path: as the direct build
- checked: Habit site, rank 14, as invalid input `twig tip taper`
- note: Scales half the twig diameter at every childless tip, structural or twig.
- blend: linear; dial: `taper` (remaining wood thickness toward the crown edge; lower means thinner tips), window [0.05, 0.6] (authored), steps 0.1 and 0.2

### `/skeleton/habit/sheddingThreshold`

Monthly vigour threshold; zero disables shedding. The legacy envelope builder interprets it as shell depth.

- real, share of height·spread, [0, 1], default 0.45
- read by grow; growth path: an annual vigour threshold, not a shell depth (fn-161 splits the two)
- checked: Habit site, rank 15, as invalid input `shedding threshold`
- dormant: zero sheds nothing
- note: On the direct build it is the shed shell's depth, a share of `height·spread`, apart from `shellDepth`.
- blend: linear; dial: `shedding_threshold` (the vigour below which a shoot is shed; zero sheds nothing), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/habit/stems`

Stems leaving the root. One is the single trunk every tree was, to the byte; a birch, a hazel or a coppiced oak stands on more.

- count (u32), stems, [1, 6], default 1
- read by grow; growth path: as the direct build
- checked: Habit site, rank 16, as invalid input `stems`
- note: At one the four stem rows lie dormant.
- blend: rounded to the nearest; dial: `stems` (stems leaving the root), window its bounds (validated bound), steps 1 and 2

### `/skeleton/habit/stemDivergence`

Degrees of bearing between neighbouring stems, about a bearing the seed alone decides. Inert at one stem, which has no neighbour. At zero every stem leaves the root on one bearing, and any rise starts to fan them apart.

- real, degrees, [0, 120], default 0
- read by grow; growth path: as the direct build
- checked: Habit site, rank 17, as invalid input `stem divergence`
- dormant: one stem
- note: With more than one stem, refused where it and `stemLean` are both zero (`stems_placed`).
- blend: degrees along the shorter arc; dial: `stem_divergence` (the degrees of bearing between neighbouring stems of a clump; at zero every stem leaves the root on one bearing, and any rise starts to fan them apart), window [0, 15] (validated bound), steps 2.5 and 5

### `/skeleton/habit/stemLean`

Degrees from vertical the outermost stems tilt away from the root; the ones between tilt in proportion to how far out they stand. Inert at one stem, which stands at the centre and so tilts by none of it. At zero every stem stands upright, and any rise starts the tilt.

- real, degrees, [0, 45], default 0
- read by grow; growth path: as the direct build
- checked: Habit site, rank 18, as invalid input `stem lean`
- dormant: one stem
- note: With more than one stem, refused at zero (`stems_placed`); `risePrimary` bends the leaning stems.
- blend: degrees along the shorter arc; dial: `stem_lean` (the degrees from vertical the outermost stems tilt away from the root; at zero every stem stands upright, and any rise starts the tilt), window its bounds (validated bound), steps 5 and 10, presets span [0, 42]

### `/skeleton/habit/stemLeanSpread`

How unequally a clump's stems lean, 0 to 1. None of it is the lean above, shared about the clump's centre; all of it leans the stems in their order instead, the first upright and the last by all of `stem_lean`. Inert at one stem, which has nothing to lean against.

- real, share, [0, 1], default 0
- read by grow; growth path: as the direct build
- checked: Habit site, rank 19, as invalid input `stem lean spread`
- dormant: one stem, or `stemLean` zero
- blend: linear; dial: `stem_lean_spread` (how unequally a clump's stems lean), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/habit/stemForkHeight`

Where a clump's later stems leave the first, as a share of the bole's height, 0 to 0.5. None of it parts them at the ground; half of it parts them halfway up the bole, with one trunk below. Inert at one stem.

- real, share of the bole, [0, 0.5], default 0
- read by grow; growth path: as the direct build
- checked: Habit site, rank 20, as invalid input `stem fork height`
- dormant: one stem
- note: The bole is `max(trunkHeight, height·crownBase)`.
- blend: linear; dial: `stem_fork_height` (how far up the bole a clump's later stems part from the first; at zero every stem leaves the root, and any rise starts the one trunk below the fork), window its bounds (validated bound), steps 0.1 and 0.2

## `/skeleton/envelope`

### `/skeleton/envelope/height`

The tree's height in metres. Everything else in the crown is measured against it: the crown base sits at `crown_base` of it and the widest radius is `spread` times it.

- real, m, [0, ∞], default 24
- read by grow, plan, expand, cull; growth path: as the direct build
- checked: Envelope site, rank 0, as invalid input `envelope`
- note: The family also refuses zero (`surface height`), and `height·spread` must be finite (`envelope`). It scales trunk radius, length taper, surface twist and flare, canopy spacing and the writhe.
- blend: linear; dial: `envelope_height` (the tree's height in metres), window [6.5, 40.5] (capped: capped both ways: the envelope validates only a floor of 0 (envelope.rs:58) and no ceiling, and a clump refuses a crown that low (its stems stand outside the envelope), so the row stays at the preset span until the generator authors one), steps 5 and 10, presets span [6.5, 40.5]

### `/skeleton/envelope/crownBase`

Where the crown starts, as a share of the height: below it the crown has no radius at all. Raising it lifts the crown and leaves a longer bare trunk.

- real, share of height, [0, 1], default 0.3
- read by grow, expand, cull; growth path: as the direct build
- checked: Envelope site, rank 1, as invalid input `envelope`
- note: `height·crownBase` is the default `trunkHeight`, the floor of the stems' bole and the lowest a short shoot grows.
- blend: linear; dial: `crown_base` (where the crown starts, as a share of the height), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/envelope/spread`

The crown's widest radius as a share of the height, so raising it widens the crown without making the tree taller.

- real, share of height, [0, ∞], default 0.3
- read by grow, plan, cull; growth path: as the direct build
- checked: Envelope site, rank 2, as invalid input `envelope`
- note: The cull shell is `shellDepth·height·spread` and the shed shell `sheddingThreshold·height·spread`.
- blend: linear; dial: `spread` (the crown's widest radius as a share of the height), window [0, 0.675] (capped: ceiling capped: the generator validates only a floor here (0), so there is no validated ceiling to widen to; the ceiling stays at the preset span until the generator authors one), steps 0.1 and 0.2, presets span [0.175, 0.675]

### `/skeleton/envelope/fullness`

Where the crown is widest, as a share of the way from the crown base to the top. Raising it carries the widest part higher, so the crown reads top-heavy.

- real, share, [0, 1], default 0.45
- read by grow, cull; growth path: as the direct build
- checked: Envelope site, rank 3, as invalid input `envelope`
- blend: linear; dial: `fullness` (where the crown is widest between its base and the top), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/envelope/shoulder`

How square the crown's outline is. Raising it holds the crown near its full width further toward the top and the base, so the profile reads boxier; lowering it tapers the outline to a point.

- real, -, (0, ∞], default 2.2
- read by grow, cull; growth path: as the direct build
- checked: Envelope site, rank 4, as invalid input `envelope`
- blend: linear; dial: `shoulder` (how square the crown's outline is; lower tapers it to a point), window [0.4, 2.8] (capped: capped both ways: the generator validates no closed range here (only positive and finite); the row stays at the preset span until the generator authors one), steps 0.5 and 1, presets span [0.4, 2.8]

### `/skeleton/envelope/irregularity`

How far the outline departs from the smooth shell, as a fraction of the radius there. 0 is the axisymmetric superellipse every tree was before, and every shipped table that leaves it there is untouched.

- real, share of radius, [0, 0.5], default 0
- read by grow, plan; growth path: as the direct build
- checked: Outline site, rank 0, as invalid input `envelope irregularity`
- note: Growth containment and the leaf box read it; the leaf cull, the shed and the GPU cull profile read the smooth outline.
- blend: linear; dial: `irregularity` (crown envelope lobes and hollows), window [0, 0.5] (authored), steps 0.08 and 0.16

### `/skeleton/envelope/lobeScale`

The wavelength of that departure over the shell's own surface, as a fraction of the tree's height: small is many small lumps, 1 is a lobe as long as the tree is tall.

- real, share of height, [0.05, 1], default 0.5
- read by grow; growth path: as the direct build
- checked: Outline site, rank 1, as invalid input `envelope lobe scale`
- dormant: `irregularity` zero, except that the curtain's drop search steps by it
- blend: linear; dial: `lobe_scale` (how long one of those lobes runs, as a share of the height), window its bounds (validated bound), steps 0.15 and 0.3

## `/skeleton/bias`

### `/skeleton/bias/gravitropism`

How strongly growth is pulled upward, most near the ground. Raising it makes the tree grow more erect. At zero nothing pulls growth upright, and any rise starts that pull.

- real, -, [0, ∞], default 0.7
- read by grow; growth path: also decides when growth-path shoots sleep
- checked: Bias site, rank 0, as invalid input `growth bias`
- blend: linear; dial: `gravitropism` (how strongly growth is pulled upright, most near the ground; at zero nothing pulls growth upright, and any rise starts that pull), window [0, 1.05] (capped: ceiling capped: the generator validates only a floor here (0), so there is no validated ceiling to widen to; the ceiling stays at the preset span until the generator authors one), steps 0.15 and 0.3, presets span [0, 1.05]

### `/skeleton/bias/lean`

How far the whole tree leans off vertical, increasing with height. Raising it tips the trunk further in one direction. At zero the tree stands plumb, and any rise starts the lean.

- real, -, [0, ∞], default 0.05
- read by grow; growth path: as the direct build
- checked: Bias site, rank 1, as invalid input `growth bias`
- note: Capped together with the writhe by `maxWritheMagnitude`.
- blend: linear; dial: `lean` (how far the whole tree leans off vertical; at zero the tree stands plumb, and any rise starts the lean), window [0, 0.075] (capped: ceiling capped: the generator validates only a floor here (0), so there is no validated ceiling to widen to; the ceiling stays at the preset span until the generator authors one), steps 0.01 and 0.02, presets span [0, 0.075]

## `/skeleton/bias/supernatural`

### `/skeleton/bias/supernatural/enabled`

Whether the supernatural field bends the wood. Off, the bias reads the amplitude, wavelength and spiral as zero.

- switch, switch, [0, 1], default 0
- read by grow; growth path: as the direct build
- checked: by a named check or not at all (see note)
- note: Does not gate `maxWritheMagnitude`. A walk interpolates what each side applies.
- blend: with the rows it is coupled to; dial: none: Boolean, not a numeric scalar.

### `/skeleton/bias/supernatural/writheAmplitude`

How far a branch may wander from a straight course, as a share of the tree's height. Raising it makes the wood wind and stray more. At zero the wood holds a straight course and the spiral rate does nothing, and any rise starts the wander.

- real, share of height, [0, ∞], default 0
- read by grow; growth path: as the direct build
- checked: Bias site, rank 2, as invalid input `growth bias`
- dormant: `enabled` off
- note: `TAU·spiralRate·amplitude` and `TAU·amplitude/wavelength` must be finite (`supernatural numeric range`).
- blend: with the rows it is coupled to; dial: `writhe_amplitude` (how far a branch wanders from a straight course, as a share of the height; at zero the wood holds a straight course and the spiral rate does nothing, and any rise starts the wander), window [0, 0.165] (capped: ceiling capped: the generator validates only a floor here (0), so there is no validated ceiling to widen to; the ceiling stays at the preset span until the generator authors one), steps 0.025 and 0.05, presets span [0, 0.165]

### `/skeleton/bias/supernatural/writheWavelength`

How long each of those wanders runs, as a share of the height. Raising it gives fewer, lazier bends; lowering it gives tighter kinks.

- real, share of height, (0, ∞], default 0.45
- read by grow; growth path: as the direct build
- checked: Writhe site, rank 1, as invalid input `writheWavelength`
- dormant: `enabled` off, or `writheAmplitude` zero
- note: Checked while dormant too.
- blend: with the rows it is coupled to; dial: `writhe_wavelength` (how long one of those wanders runs, as a share of the height), window [0.225, 0.675] (capped: capped both ways: the generator validates no closed range here (only positive and finite); the row stays at the preset span until the generator authors one), steps 0.05 and 0.1, presets span [0.225, 0.675]

### `/skeleton/bias/supernatural/spiralRate`

How many full turns the wander winds around the trunk over the tree's height. Raising it tightens the spiral. At zero the wander winds around nothing, and any rise starts the spiral.

- real, turns over the height, [0, ∞], default 0
- read by grow; growth path: as the direct build
- checked: Bias site, rank 3, as invalid input `growth bias`
- dormant: `enabled` off, or `writheAmplitude` zero
- blend: with the rows it is coupled to; dial: `spiral_rate` (how many turns the wander winds around the trunk over the height; at zero the wander winds around nothing, and any rise starts the spiral), window [0, 3.9] (capped: ceiling capped: the generator validates only a floor here (0), so there is no validated ceiling to widen to; the ceiling stays at the preset span until the generator authors one), steps 0.5 and 1, presets span [0, 3.9]

### `/skeleton/bias/supernatural/maxWritheMagnitude`

The ceiling on how hard the wander may pull in any one step, so the other writhe rows cannot bend the wood arbitrarily.

- real, -, [0, 8], default 0.9
- read by grow; growth path: as the direct build
- checked: Writhe site, rank 0, as invalid value `maxWritheMagnitude`
- note: Applies with `enabled` off, and caps the lean term with the writhe.
- blend: with the rows it is coupled to; dial: `max_writhe_magnitude` (the ceiling on how hard the wander may pull in one step), window its bounds (validated bound), steps 1 and 2

## `/skeleton/twigs/twig`

### `/skeleton/twigs/twig/diameter`

The finished thickness of a twig in metres. Wood at or below half of it is drawn as a twig, so raising it thickens the twig layer and hands more of the fine wood to it.

- real, m, [0.000001, 1000000], default 0.005
- read by grow, plan; growth path: as the direct build
- checked: Twig site, rank 0, as invalid value `twig diameter`
- note: Wood at or below half of it is a twig; it is also the twig radius, the cap on childless tips (with `twigTipTaper`), and the leaf box's size where `canopy.shootRadius` is zero.
- blend: linear; dial: `twig_diameter` (the finished thickness of a twig, in metres), window its bounds (validated bound), steps 0.001 and 0.002, presets span [0.0005, 0.0065]

### `/skeleton/twigs/twig/length`

The length in metres a twig shoot grows before it stops, and the whole of one internode on leaf-bearing wood. Raising it lengthens every twig, so the crown carries a deeper, shaggier skin.

- real, m, [0.000001, 1000000], default 0.25
- read by grow; growth path: also sets how far a waiting shoot reaches
- checked: Twig site, rank 1, as invalid value `twig length`
- note: The internode floor on bearing wood; a structural run the shell stops is trimmed by it.
- blend: linear; dial: `twig_length` (the metres a twig shoot grows before it stops), window its bounds (validated bound), steps 0.05 and 0.1, presets span [0.15, 0.55]

### `/skeleton/twigs/twig/internodeLength`

Metres between the joints on wood thicker than the bearing diameter, and the spacing of the stations a leaf sits on. Raising it gives longer segments, so laterals and leaves sit further apart.

- real, m, [0.000001, 1000000], default 0.02
- read by grow, plan, expand; growth path: as the direct build
- checked: Twig site, rank 2, as invalid value `twig internodeLength`
- note: Also the spacing of leaf stations; the canopy check bounds it again at [1e-6, 1e6] (`twig internode`).
- blend: linear; dial: `twig_internode_length` (metres between the joints on wood thicker than the bearing diameter), window its bounds (validated bound), steps 0.015 and 0.03, presets span [0.000001, 0.08875]

### `/skeleton/twigs/twig/stationsPerInternode`

How many leaf stations sit at each joint, each turned its own share of a full turn around the shoot. Raising it crowds more leaves onto the same joints.

- count (u32), stations, [1, 32], default 1
- read by plan, expand; growth path: as the direct build
- checked: Twig site, rank 4, as invalid value `twig stationsPerInternode`
- note: Conflicting bounds: the canopy check admits 1 to 64 (`twig stations`), and this check's 1 to 32 answers first. The skeleton never reads it; with `canopy.divergence` it sets the phyllotaxis.
- blend: rounded to the nearest; dial: `twig_stations_per_internode` (leaf stations at each joint, spread around the shoot), window its bounds (validated bound), steps 1 and 2, presets span [1, 4]

### `/skeleton/twigs/twig/bearingDiameter`

The thickness in metres at or below which a shoot bears leaves and side shoots of its own. Raising it lets thicker wood bear, so foliage reaches further back down the branch.

- real, m, [0.000001, 1000000], default 0.05
- read by grow; growth path: also marks leaf-bearing wood and caps `canopy.shootRadius`
- checked: Twig site, rank 3, as invalid value `twig bearingDiameter`
- note: Wood at or below half of it takes one lateral per internode, ignoring `laterals`, and `twig.length` as its internode floor.
- blend: linear; dial: `twig_bearing_diameter` (the thickness in metres at or below which a shoot bears leaves and side shoots), window its bounds (validated bound), steps 0.01 and 0.02, presets span [0.005, 0.065]

## `/skeleton/twigs`

### `/skeleton/twigs/lengthRatio`

A shoot's length as a share of the one that bore it. Raising it makes each generation of twigs longer relative to its parent.

- real, share, [0.05, 1], default 0.4
- read by grow; growth path: as the direct build
- checked: Twig site, rank 0, as invalid value `twig lengthRatio`
- note: With `ratioPower` it sets the child radius; a varied draw is clamped to the bounds.
- blend: linear; dial: `twig_length_ratio` (a shoot's length as a share of the one that bore it), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/twigs/ratioPower`

How much thinner a shoot is than its parent for the same drop in length. Raising it leaves side shoots finer.

- real, -, [0, 8], default 1.3
- read by grow; growth path: as the direct build
- checked: Twig site, rank 1, as invalid value `twig ratioPower`
- blend: linear; dial: `twig_ratio_power` (how much finer a shoot is than its parent for the same drop in length), window its bounds (validated bound), steps 1 and 2

### `/skeleton/twigs/internodeFactor`

The fewest of its own diameters a segment of wood may span. Raising it makes segments longer for the same thickness, so there are fewer joints and the wood reads straighter.

- real, -, [0.05, 32], default 2.5
- read by grow; growth path: as the direct build
- checked: Twig site, rank 2, as invalid value `twig internodeFactor`
- blend: linear; dial: `twig_internode_factor` (the fewest of its own diameters a segment of wood may span), window its bounds (validated bound), steps 0.5 and 1, presets span [1.25, 3.75]

### `/skeleton/twigs/maxInternodes`

The ceiling on segments one length of wood may be cut into. It binds only where the two lengths above would cut more, and there it caps the cost rather than states a look.

- count (u32), internodes, [1, 4294967295], default 32
- read by grow; growth path: as the direct build
- checked: Twig site, rank 9, as invalid value `twig maxInternodes`
- blend: rounded to the nearest; dial: none: A cost cap: it binds only where the two internode lengths would cut more segments (crates/telperion-core/src/twigs.rs:221).

### `/skeleton/twigs/laterals`

How many side shoots leave each station along a twig. Raising it crowds more twigs onto the same length of wood.

- count (u32), laterals, [0, 7], default 2
- read by grow; growth path: also sizes the rollback checkpoint
- checked: Twig site, rank 3, as invalid value `twig laterals`
- note: Held as a bitmask, which is why it stops at 7; ignored on bearing wood; at zero non-bearing shoots may sleep.
- blend: rounded to the nearest; dial: `twig_laterals` (side shoots leaving each station along a twig), window its bounds (validated bound), steps 1 and 2

### `/skeleton/twigs/generations`

Twig-law generations of branching, 1 to 6. A lateral born at or past this generation is a twig whatever the pipe model left its radius, so the twig layer's depth is a row a table states rather than a consequence of how thick the wood is.

- count (u32), generations, [1, 6], default 6
- read by grow; growth path: as the direct build
- checked: Twig site, rank 13, as invalid input `twig generations`
- blend: rounded to the nearest; dial: `twig_generations` (generations of twig branching below the limbs), window its bounds (validated bound), steps 1 and 2

### `/skeleton/twigs/limbRadius`

The share of the trunk's radius at or below which wood starts bearing twigs. Raising it lets twigs start on thicker wood, so they reach further back toward the trunk.

- real, share of root radius, [0, 1], default 0.1
- read by grow; growth path: as the direct build
- checked: Twig site, rank 4, as invalid value `twig limbRadius`
- note: Measured against the thickest stem where laterals are seeded and against the root node where they advance.
- blend: linear; dial: `twig_limb_radius` (the share of the trunk's radius at or below which wood starts bearing twigs), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/twigs/reach`

How deep the outer skin of the crown is that only twigs may fill, as a share of the crown: the scaffold is grown into what is left inside it. Raising it holds the structural wood further in and leaves a deeper twig layer.

- real, share, [0, 0.9], default 0.2
- read by grow; growth path: as the direct build
- checked: Twig site, rank 5, as invalid value `twig reach`
- note: Shrinks the scaffold's room and, with `attractorWeight` above zero, the attractor volume; the twig layer itself is not bound by it.
- blend: linear; dial: `twig_reach` (the depth of the crown's outer skin that only twigs fill), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/twigs/angle`

The degrees a side shoot leaves its parent. Raising it swings twigs further out toward square with the branch.

- real, degrees, [0, 90], default 45
- read by grow; growth path: as the direct build
- checked: Twig site, rank 6, as invalid value `twig angle`
- note: Also sets the laterals' separation, `min(angle, maxTurnPerStep)`; a varied draw is clamped.
- blend: degrees along the shorter arc; dial: `twig_angle` (the degrees a side shoot leaves its parent), window its bounds (validated bound), steps 1.5 and 3, presets span [37.5, 47.5]

### `/skeleton/twigs/angleVariation`

How many degrees that departure angle varies shoot to shoot. Raising it makes the twig layer less uniform.

- real, degrees, [0, 90], default 10
- read by grow; growth path: as the direct build
- checked: Twig site, rank 7, as invalid value `twig angleVariation`
- blend: degrees along the shorter arc; dial: `twig_angle_variation` (the degrees that departure angle varies shoot to shoot), window its bounds (validated bound), steps 1.5 and 3, presets span [5, 15]

### `/skeleton/twigs/vigourVariation`

How much shoot length varies shoot to shoot. Raising it gives a more uneven, less combed twig layer.

- real, share, [0, 0.95], default 0.15
- read by grow; growth path: as the direct build
- checked: Twig site, rank 8, as invalid value `twig vigourVariation`
- blend: linear; dial: `twig_vigour_variation` (how far shoot length varies shoot to shoot), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/twigs/divergence`

The degrees each successive shoot is turned around the wood that bears it. Raising it swings the next shoot further around, so the twigs spiral differently.

- real, degrees, any finite value, default 137.508
- read by grow; growth path: as the direct build
- checked: Twig site, rank 12, as invalid input `twig divergence`
- blend: degrees along the shorter arc; dial: `twig_divergence` (the degrees each successive shoot is turned around the wood that bears it), window [68.752, 206.256] (capped: capped both ways: the generator validates no closed range here (only finite); the row stays at the preset span until the generator authors one), steps 20 and 40, presets span [68.752, 206.256]

### `/skeleton/twigs/hang`

How strongly a shoot hangs, 0 to 3. At 0 nothing hangs and the local law is the ordinary one; at 1 a curtain takes its full droop.

- real, -, [0, 3], default 0
- read by grow; growth path: as the direct build
- checked: Twig site, rank 14, as invalid input `hang`
- note: Gates the curtain: the pendulous, separation, sag, variation, droop and step-clearance rows do nothing at zero. Above one it scales droop and separation, while the floor, drop and length terms take one.
- blend: linear; dial: `twig_hang` (how strongly a shoot hangs; zero hangs nothing), window its bounds (validated bound), steps 0.5 and 1

### `/skeleton/twigs/pendulousLength`

Metres a pendulous shoot grows before it stops, and the length its droop reaches the cap over.

- real, m, [0.05, 5], default 0.25
- read by grow; growth path: as the direct build
- checked: Twig site, rank 15, as invalid input `pendulous length`
- dormant: `hang` zero
- note: Without `sag` it caps a hanging run; with it, it is the run's length.
- blend: linear; dial: `twig_pendulous_length` (the metres a hanging shoot grows before it stops), window its bounds (validated bound), steps 1 and 2

### `/skeleton/twigs/pendulousRadius`

Fraction of the root radius at or below which a station's shoots hang.

- real, share of stem radius, [0, 1], default 1
- read by grow; growth path: as the direct build
- checked: Twig site, rank 16, as invalid input `pendulous radius`
- dormant: `hang` zero
- blend: linear; dial: `twig_pendulous_radius` (the share of the root radius at or below which a station's shoots hang), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/twigs/curtainSeparation`

Degrees between neighbouring shoots in a curtain.

- real, degrees, [1, 45], default 4
- read by grow; growth path: as the direct build
- checked: Twig site, rank 17, as invalid input `curtain separation`
- dormant: `hang` zero
- blend: degrees along the shorter arc; dial: `twig_curtain_separation` (the degrees between neighbouring shoots in a curtain), window its bounds (validated bound), steps 1.5 and 3, presets span [1.5, 11.5]

### `/skeleton/twigs/sag`

How far toward straight down a hanging shoot's course has turned by the end of its pendulous length, 0 to 1: at 0 the shoot holds the direction it departed with and the curtain is a set of rods, at 1 it hangs vertical, and the turn is spread along the run as an arc steepest at the wood that bears it.

- real, share, [0, 1], default 0
- read by grow; growth path: as the direct build
- checked: Twig site, rank 18, as invalid input `sag`
- dormant: `hang` zero
- blend: linear; dial: `twig_sag` (how far a hanging shoot's course has turned toward straight down by its end; at zero a hanging shoot holds the direction it left on and the curtain is a set of rods, and any rise starts the turn), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/twigs/pendulousVariation`

How much shorter than the pendulous length a hanging shoot may run, 0 to 1: each shoot's own length is the pendulous length times one minus this times a draw in 0 to 1 keyed by the shoot and the seed. At 0 every shoot has the one length the table states.

- real, share, [0, 1], default 0
- read by grow; growth path: as the direct build
- checked: Twig site, rank 19, as invalid input `pendulous variation`
- dormant: `hang` zero
- blend: linear; dial: `twig_pendulous_variation` (how much shorter than the pendulous length a hanging shoot may run; at zero every shoot runs the one length the table states, and any rise starts the variation), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/twigs/curtainDrop`

How far below the shell's lower surface a hanging shoot may fall, 0 to 1, as a share of the way from that surface down to the clearance: at 0 the shell binds a hanging shoot as it binds every other, at 1 the shoot may fall to the clearance. Only a shoot that hangs, only under the crown's footprint.

- real, share, [0, 1], default 0
- read by grow; growth path: as the direct build
- checked: Twig site, rank 20, as invalid input `curtain drop`
- note: Above zero admits wood below the shell and stops shoots sleeping.
- blend: linear; dial: `twig_curtain_drop` (how far below the crown's own surface a hanging shoot may fall; at zero the shell binds a hanging shoot as it binds every other, and any rise starts the fall below it), window its bounds (validated bound), steps 0.15 and 0.3

### `/skeleton/twigs/curtainClearance`

Metres above the ground no hanging shoot falls below, 0 to 5. Never above the crown's own base, whatever the row says.

- real, m, [0, 5], default 0.5
- read by grow; growth path: as the direct build
- checked: Twig site, rank 21, as invalid input `curtain clearance`
- dormant: unless `sag` or `curtainDrop` is above zero
- note: Capped by `trunkHeight` in the floor and by `height·crownBase` in the band.
- blend: linear; dial: `twig_curtain_clearance` (the metres above the ground no hanging shoot falls below), window its bounds (validated bound), steps 1 and 2

### `/skeleton/twigs/maxDroop`

The furthest a hanging shoot may bend toward straight down. Raising it lets curtains hang more heavily.

- real, -, [0, 10], default 0.35
- read by grow; growth path: as the direct build
- checked: Twig site, rank 10, as invalid value `twig maxDroop`
- dormant: `hang` zero
- blend: linear; dial: `twig_max_droop` (the furthest a hanging shoot may bend toward straight down), window its bounds (validated bound), steps 1.5 and 3

### `/skeleton/twigs/curtainStepClearance`

How close to the ground a hanging curtain may reach before it stops growing. Raising it lets curtains hang nearer the floor.

- real, share, [0, 1], default 0.8
- read by grow; growth path: as the direct build
- checked: Twig site, rank 11, as invalid value `twig curtainStepClearance`
- dormant: `hang` zero
- blend: linear; dial: `twig_curtain_step_clearance` (how close to the ground a hanging curtain may reach before it stops), window its bounds (validated bound), steps 0.15 and 0.3

## `/skeleton/growth`

### `/skeleton/growth/influenceRadius`

Metres a pull point may reach to steer the wood nearest it; unset, the crown's own volume and the point count decide it. Raising it lets distant points draw a branch across the crown.

- optional real, m, [0, ∞], default unset
- read by grow; growth path: as the direct build
- checked: by a named check or not at all (see note)
- dormant: `attractorWeight` zero
- note: Unset: `max(9·height·step, 2·cbrt(crown volume / points))`. Judged in the resolved configuration (`colonization configuration`).
- blend: with the rows it is coupled to; dial: none: An unset `Option` in every shipped preset, so the wire row reads null and there is no current value to step (crates/telperion-core/src/branching.rs:72).

### `/skeleton/growth/killDistance`

Metres within which a pull point counts as reached and stops pulling; unset, twice the step distance. Raising it uses the points up sooner, so branches stop shorter and the crown fills coarsely.

- optional real, m, [0, ∞], default unset
- read by grow; growth path: as the direct build
- checked: by a named check or not at all (see note)
- dormant: `attractorWeight` zero
- note: Acts as `min(kill, growth unit)`, so above one unit it does nothing; unset, `2·height·step`. Judged in the resolved configuration (`colonization configuration`).
- blend: with the rows it is coupled to; dial: none: Null in every shipped preset, as above.

### `/skeleton/growth/stepDistance`

Metres of wood laid down in one growth step; unset, the tree's height times `step`. Raising it lays down longer, coarser segments.

- optional real, m, (0, ∞], default unset
- read by grow; growth path: as the direct build
- checked: by a named check or not at all (see note)
- note: Unset: `height·step`; it does not move the default kill or influence distance. Judged in the resolved configuration (`colonization configuration`).
- blend: with the rows it is coupled to; dial: none: Null in every shipped preset, as above.

### `/skeleton/growth/trunkHeight`

Metres of bare trunk before the crown may start; unset, the envelope's own crown base. Raising it lifts the whole crown and leaves a longer clear bole.

- optional real, m, (0, ∞], default unset
- read by grow; growth path: as the direct build
- checked: by a named check or not at all (see note)
- note: Refused at or below zero (`trunkHeight`) by `resolved_growth`, then judged in the resolved configuration. Unset: `height·crownBase`; below that the stems' bole, fork and top keep `height·crownBase`.
- blend: with the rows it is coupled to; dial: none: Null in every shipped preset, as above.

### `/skeleton/growth/maxNodes`

The ceiling on nodes the crown may grow; unset, the shipped default. Growth stops at it, so raising it changes only a crown that reached it.

- optional count (usize), nodes, [1, 4294967295], default unset
- read by grow; growth path: zero builds an empty capped seedling
- checked: by a named check or not at all (see note)
- note: Unset: 250 000. Zero is refused by `Family::validate` (`maxNodes`) and above u32::MAX by `ranges::max_nodes`.
- blend: with the rows it is coupled to; dial: none: Null in every shipped preset, and a resource cap besides.

### `/skeleton/growth/maxTurnPerStep`

The most a growing shoot may turn in one step, in degrees; unset, 35. At 180 or more nothing is limited.

- optional real, degrees, [0, ∞], default unset
- read by grow; growth path: as the direct build
- checked: by a named check or not at all (see note)
- note: `presets::by_identity` sets 35 where a table leaves it unset, so a table and its identity differ here. Judged in the resolved configuration (`colonization configuration`).
- blend: with the rows it is coupled to; dial: none: Null in five of the seven families, including the beech a run is pointed at. A proposal batch reads every dial's current value off the wire and refuses the whole batch when one is null (src/tuning/judgments.rs:137).

## `/radii`

### `/radii/trunkRadius`

The trunk's radius at the ground as a share of the tree's height, so raising it thickens every piece of wood in proportion.

- real, share of height, [0.000004, 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000], default 0.02
- read by grow, plan; growth path: as the direct build
- checked: Radius site, rank 1, as invalid value `trunkRadius`
- note: Also sizes the leaf box where `canopy.shootRadius` is above zero or short shoots grow.
- blend: linear; dial: `trunk_radius` (the trunk's radius at the ground as a share of the height), window [0.000004, 0.023] (capped: ceiling capped: the generator validates only a floor here (4e-6), so there is no validated ceiling to widen to; the ceiling stays at the preset span until the generator authors one), steps 0.002 and 0.004, presets span [0.011, 0.023]

### `/radii/forkExponent`

How wood divides at a fork. The parent's area is the sum of the children's radii raised to this power, so raising it leaves the children thicker for the same parent.

- real, -, [1, 8], default 2
- read by grow; growth path: as the direct build
- checked: Radius site, rank 2, as invalid value `forkExponent`
- blend: linear; dial: `fork_exponent` (how wood divides at a fork; higher leaves the children thicker), window its bounds (validated bound), steps 1 and 2

### `/radii/lengthTaper`

How fast wood thins along its own length. Raising it makes a branch narrow more sharply from its base to its tip.

- real, per height, [0, 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000], default 0.6
- read by grow; growth path: as the direct build
- checked: Radius site, rank 3, as invalid value `lengthTaper`
- blend: linear; dial: `length_taper` (how fast wood thins along its own length), window [0, 0.8] (capped: ceiling capped: the generator validates only a floor here (0), so there is no validated ceiling to widen to; the ceiling stays at the preset span until the generator authors one), steps 0.1 and 0.2, presets span [0, 0.8]

### `/radii/maxTaperExponent`

The ceiling on accumulated taper, so no single long branch can thin away to nothing. Raising it lets long branches taper further.

- real, -, [0, 64], default 12
- read by grow; growth path: as the direct build
- checked: Radius site, rank 0, as invalid value `maxTaperExponent`
- dormant: `lengthTaper` zero
- blend: linear; dial: `max_taper_exponent` (the ceiling on accumulated taper along one long branch), window its bounds (validated bound), steps 2 and 4, presets span [6, 18]

## `/surface`

### `/surface/radialSegments`

How many sides each piece of wood is drawn with. Raising it makes the wood rounder and smoother, and costs triangles.

- count (u32), sides, [3, 64], default 12
- read by expand; growth path: as the direct build
- checked: Surface site, rank 1, as invalid input `surface parameters`
- note: Wood is drawn with `max(radialSegments, 4·lobes)` sides.
- blend: rounded to the nearest; dial: `radial_segments` (how many sides each piece of wood is drawn with), window its bounds (validated bound), steps 2 and 4, presets span [6, 18]

### `/surface/lobes`

How many ridges run up around the trunk. Raising it gives the bark more flutes; zero is a plain round bole.

- count (u32), lobes, [0, 16], default 5
- read by expand; growth path: as the direct build
- checked: Surface site, rank 2, as invalid input `surface parameters`
- note: Raises the side count even at `lobeDepth` zero; the GPU executor draws only unlobed wood itself.
- blend: rounded to the nearest; dial: `surface_lobes` (how many ridges run up around the trunk), window its bounds (validated bound), steps 2 and 4

### `/surface/lobeDepth`

How deep the flutes between those ridges cut, as a share of the wood's own radius. Raising it makes the fluting more pronounced. At zero the bole is plainly round whatever the ridge count says, and any rise starts cutting the flutes.

- real, share of radius, [0, 0.9], default 0
- read by plan, expand; growth path: as the direct build
- checked: Surface site, rank 3, as invalid input `surface parameters`
- dormant: `lobes` zero
- note: With `surfaceContact` above zero, leaf seating and the leaf box scale by `1 + lobeDepth`; the socket's inscribed radius shrinks by it.
- blend: linear; dial: `surface_lobe_depth` (how deep the flutes between those ridges cut, as a share of the radius; at zero the bole is plainly round whatever the ridge count says, and any rise starts cutting the flutes), window its bounds (validated bound), steps 0.15 and 0.3

### `/surface/twistRate`

How many turns those ridges make over the tree's height. Raising it winds them more tightly around the trunk.

- real, turns over the height, [-64, 64], default 0
- read by expand; growth path: as the direct build
- checked: Surface site, rank 4, as invalid input `surface parameters`
- dormant: unless `lobes` and `lobeDepth` are both above zero
- blend: linear; dial: `twist_rate` (how many turns those ridges make over the tree's height), window its bounds (validated bound), steps 1 and 2, presets span [-1.2, 3.6]

### `/surface/flareRadius`

How much wider the trunk is where it meets the ground, as a multiple of its own radius. Raising it gives a broader buttress.

- real, multiple of radius, [1, 8], default 2.1
- read by plan, expand; growth path: as the direct build
- checked: Surface site, rank 5, as invalid input `surface parameters`
- note: Applies by height, to branches near the ground too; seats leaves where surface contact is on.
- blend: linear; dial: `flare_radius` (how much wider the trunk is at the ground, as a multiple of its radius), window its bounds (validated bound), steps 0.2 and 0.4, presets span [1.2, 2.4]

### `/surface/flareFalloff`

How far up the trunk that flare reaches, as a share of the height. Raising it carries the swelling further up the bole.

- real, share of height, [0.0001, 1], default 0.022
- read by expand; growth path: as the direct build
- checked: Surface site, rank 6, as invalid input `surface parameters`
- dormant: `flareRadius` one
- blend: linear; dial: `flare_falloff` (how far up the bole that flare reaches, as a share of the height), window its bounds (validated bound), steps 0.15 and 0.3

### `/surface/flareDepth`

How deep the trunk's base is sunk below the ground, as a share of the height. Raising it buries more of the flare.

- real, share of height, [0, 1], default 0.004
- read by expand; growth path: as the direct build
- checked: Surface site, rank 7, as invalid input `surface parameters`
- note: Buries one ring per trunk run even with no flare.
- blend: linear; dial: `flare_depth` (how deep the trunk's base is sunk below the ground, as a share of the height), window its bounds (validated bound), steps 0.15 and 0.3

### `/surface/forkSocket`

How deeply a child branch is set into its parent at a fork. Raising it sinks the junction further in, so the two read as one piece of wood rather than two tubes meeting.

- real, share, [0, 0.9], default 0.5
- read by expand; growth path: as the direct build
- checked: Surface site, rank 8, as invalid input `surface parameters`
- blend: linear; dial: `fork_socket` (how deeply a child branch is set into its parent at a fork), window its bounds (validated bound), steps 0.15 and 0.3

### `/surface/socketContainment`

Fraction of the parent's inscribed radius available for a socket.

- real, share, [0, 1], default 0.9
- read by expand; growth path: as the direct build
- checked: Surface site, rank 0, as invalid value `socketContainment`
- dormant: `forkSocket` zero
- blend: linear; dial: `socket_containment` (the share of the parent's inscribed radius a socket may use), window its bounds (validated bound), steps 0.15 and 0.3

### `/surface/forkSwell`

How much wood thickens at a fork. Raising it leaves a more pronounced collar where a branch leaves its parent.

- real, multiple of radius, [1, 4], default 1.35
- read by plan, expand; growth path: as the direct build
- checked: Surface site, rank 9, as invalid input `surface parameters`
- note: Seats leaves and sizes the leaf box where surface contact is on.
- blend: linear; dial: `fork_swell` (how much wood thickens at a fork), window its bounds (validated bound), steps 0.5 and 1

## `/canopy`

### `/canopy/shootRadius`

Wood at or below this fraction of the root radius bears foliage of its own, beside whatever the twig layer marks. Zero leaves the twigs alone with it; without a twig layer it is what selects the terminal shoots.

- real, share of root radius, [0, 1], default 0
- read by plan, expand; growth path: measured against the root node, and capped by `twig.bearingDiameter`
- checked: Canopy site, rank 0, as invalid input `shoot radius`
- note: Above zero the leaf box uses the trunk radius.
- blend: linear; dial: `canopy_shoot_radius` (the share of the root radius at or below which wood bears foliage of its own; at zero no wood beyond the twig layer bears foliage of its own, and any rise starts clothing it), window its bounds (validated bound), steps 0.005 and 0.01, presets span [0, 0.0375]

### `/canopy/spacing` (deprecated)

Metres between leaves along a shoot, as a share of the tree's height. Raising it spreads the leaves further apart, so the crown carries fewer of them.

- real, share of height, [0.001, 1000000], default 0.006
- read by no stage of the direct build; growth path: as the direct build
- checked: Canopy site, rank 1, as invalid input `foliage spacing`
- dormant: only a placement with no twig table reads it, which no production caller passes
- blend: linear; dial: none

### `/canopy/divergence`

The degrees each successive leaf is turned around its shoot. Raising it turns the next leaf further round, so the leaves spiral differently.

- real, degrees, [-1000000000, 1000000000], default 137.508
- read by expand; growth path: as the direct build
- checked: Canopy site, rank 2, as invalid input `divergence`
- note: Past a phase-error bound, station preparation leaves the GPU for the CPU.
- blend: degrees along the shorter arc; dial: `foliage_divergence` (the degrees each successive leaf is turned around its shoot), window its bounds (validated bound), steps 15 and 30, presets span [116.262, 201.246]

### `/canopy/clump` (deprecated)

How many extra leaves are gathered at the end of a shoot that has no twig layer. Raising it packs a denser tuft at the tip.

- count (u32), leaves, [0, 64], default 5
- read by no stage of the direct build; growth path: as the direct build
- checked: Canopy site, rank 14, as invalid input `foliage clump`
- dormant: only a placement with no twig table reads it, which no production caller passes
- blend: rounded to the nearest; dial: none

### `/canopy/clumpSpan` (deprecated)

How far back from the tip that tuft is scattered, as a share of the shoot's length. Raising it spreads the tuft further down the shoot.

- real, share of the shoot, [0, 1], default 0.3
- read by no stage of the direct build; growth path: as the direct build
- checked: Canopy site, rank 3, as invalid input `clump span`
- dormant: only a placement with no twig table reads it, which no production caller passes
- blend: linear; dial: none

### `/canopy/outward`

How far a leaf turns away from the trunk. Raising it points the leaves outward, away from the tree's axis.

- real, -, [-1, 1], default 0.6
- read by expand; growth path: as the direct build
- checked: Canopy site, rank 4, as invalid input `outward`
- note: Rosette fronds ignore it.
- blend: linear; dial: `leaf_outward` (how far a leaf turns away from the trunk), window its bounds (validated bound), steps 0.25 and 0.5

### `/canopy/upward`

How far a leaf turns toward the sky. Raising it tips the leaves up.

- real, -, [-1, 1], default 0.35
- read by expand; growth path: as the direct build
- checked: Canopy site, rank 5, as invalid input `upward`
- note: Rosette fronds ignore it.
- blend: linear; dial: `leaf_upward` (how far a leaf turns toward the sky), window its bounds (validated bound), steps 0.25 and 0.5

### `/canopy/forwardLean`

Lean along the shoot, as a fraction of the radial off the wood.

- real, share of the radial, [-1, 1], default 0
- read by expand; growth path: as the direct build
- checked: Canopy site, rank 6, as invalid input `forward lean`
- blend: linear; dial: `leaf_forward_lean` (how far a leaf leans along its shoot), window its bounds (validated bound), steps 0.25 and 0.5

### `/canopy/leanRise`

Further lean along the shoot on radials that face upward.

- real, share of the radial, [-2, 2], default 0
- read by expand; growth path: as the direct build
- checked: Canopy site, rank 7, as invalid input `lean rise`
- blend: linear; dial: `leaf_lean_rise` (further lean along the shoot for leaves whose radial faces up), window its bounds (validated bound), steps 0.5 and 1

### `/canopy/surfaceContact`

The station sits on the shoot axis at 0 and on the wood's own contact surface at 1; the surface is built whenever it is positive.

- real, share, [0, 1], default 0
- read by plan, expand; growth path: as the direct build
- checked: Canopy site, rank 8, as invalid input `surface contact`
- note: Above zero leaves sit on the wood's rings, swept once for the wood and the leaves or by the leaves alone.
- blend: linear; dial: `leaf_surface_contact` (how far out a leaf is seated, from the shoot axis at nought to the wood's own skin at one), window its bounds (validated bound), steps 0.15 and 0.3

### `/canopy/scatter`

The degrees a leaf may be turned at random from where it was placed. Raising it leaves the crown less combed.

- real, degrees, [0, 90], default 18
- read by plan, expand; growth path: as the direct build
- checked: Canopy site, rank 9, as invalid input `scatter`
- note: Above zero a leaf draws four numbers instead of one, which moves every later leaf on the stream.
- blend: degrees along the shorter arc; dial: `leaf_scatter` (the degrees a leaf may be turned at random from where it was placed), window [0, 90] (validated bound), steps 15 and 30, presets span [0, 90]

### `/canopy/size`

The size every leaf is drawn at, as a multiple of the element's own dimensions. Raising it enlarges every leaf.

- real, multiple of the element, [0, 1000], default 1
- read by plan, expand; growth path: as the direct build
- checked: Canopy site, rank 10, as invalid input `foliage size`
- note: At zero no leaf is placed.
- blend: linear; dial: `leaf_size` (the size every leaf is drawn at, as a multiple of the element's own), window its bounds (validated bound), steps 0.15 and 0.3, presets span [0.525, 1.575]

### `/canopy/sizeVariation`

How far leaf size varies leaf to leaf, as a share of that size. Raising it mixes larger and smaller leaves more widely.

- real, share, [0, 0.9], default 0.35
- read by plan, expand; growth path: as the direct build
- checked: Canopy site, rank 11, as invalid input `size variation`
- blend: linear; dial: `leaf_size_variation` (how far leaf size varies leaf to leaf), window its bounds (validated bound), steps 0.15 and 0.3

### `/canopy/shortShootSpacing`

Metres between short shoots along limb and branch wood: spurs a few centimetres long, each ending in a cluster of leaves. Zero grows none.

- real, m, 0 or [0.01, 1000], default 0
- read by plan, expand; growth path: as the direct build
- checked: ShortShoots site, rank 1, as invalid input `short shoot spacing`
- note: Above zero there is no leaf plan: leaves are placed for the field, and the GPU executor falls back.
- blend: as the density it spaces; dial: `spacing` (metres between leaf clusters), window [0.01, 0.08] (authored), steps 0.01 and 0.02

### `/canopy/shortShootRadius`

Wood thicker than this fraction of the stem's radius carries no short shoot, and neither does twig wood or anything below the crown base.

- real, share of stem radius, [0, 1], default 0.15
- read by expand; growth path: as the direct build
- checked: ShortShoots site, rank 2, as invalid input `short shoot radius`
- dormant: `shortShootSpacing` zero
- blend: linear; dial: `short_shoot_radius` (the share of the stem's radius above which wood carries no short shoot), window its bounds (validated bound), steps 0.15 and 0.3

### `/canopy/shortShootLength`

Metres from the bark to the cluster a short shoot carries.

- real, m, [0, 0.5], default 0.04
- read by expand; growth path: as the direct build
- checked: ShortShoots site, rank 3, as invalid input `short shoot length`
- dormant: `shortShootSpacing` zero
- blend: linear; dial: `short_shoot_length` (the metres from the bark to the cluster a short shoot carries), window its bounds (validated bound), steps 0.1 and 0.2

### `/canopy/shortShootLeaves`

Leaves in one short shoot's cluster, 1 to 8.

- count (u32), leaves, [1, 8], default 3
- read by expand; growth path: as the direct build
- checked: ShortShoots site, rank 5, as invalid input `short shoot leaves`
- dormant: `shortShootSpacing` zero
- blend: rounded to the nearest; dial: `leaves` (leaves per cluster), window [2, 12] (authored), steps 2 and 4

### `/canopy/shortShootSpread`

Degrees either side of its short shoot's bearing a cluster's leaves fan across, held level: 90 is a half circle, 0 stacks them.

- real, degrees, [0, 90], default 45
- read by expand; growth path: as the direct build
- checked: ShortShoots site, rank 4, as invalid input `short shoot spread`
- dormant: `shortShootSpacing` zero
- blend: degrees along the shorter arc; dial: `short_shoot_spread` (the degrees either side of its bearing a cluster's leaves fan across), window its bounds (validated bound), steps 10 and 20, presets span [22.5, 90]

### `/canopy/limbClumping`

How far into each limb system the gap between it and its neighbours reaches, as a share of the way from their shared boundary to the system's centre: each limb system then keeps a rounded leaf mass of its own. Zero, the neutral, thins nothing.

- real, share, [0, 1], default 0
- read by plan, expand; growth path: as the direct build
- checked: Canopy site, rank 12, as invalid input `limb clumping`
- note: Above zero there is no leaf plan and leaves are thinned per limb system.
- blend: linear; dial: `limb_clumping` (how far the gap between neighbouring limb systems reaches into each), window its bounds (validated bound), steps 0.15 and 0.3

### `/canopy/clumpSystemOrder`

How deep a lateral may be and still start a limb system of its own. Raising it parts the crown into more and smaller leaf masses; it does nothing until `limb_clumping` is above zero.

- count (u32), order, [0, ∞], default 2
- read by plan, expand; growth path: as the direct build
- checked: by a named check or not at all (see note)
- note: No upper bound is checked. The planned field assigns limb systems by it even at `limbClumping` zero.
- blend: rounded to the nearest; dial: `clump_system_order` (how deep a lateral may be and still start a limb system of its own), window [1, 5] (capped: capped both ways: the generator validates no closed range here (an unchecked whole number); the row stays at the preset span until the generator authors one), steps 1 and 2, presets span [1, 5]

### `/canopy/clumpNeighbours`

Nearest neighbours and cell crossings in the clumping approximation.

- count (u32), neighbours, [1, 4294967295], default 12
- read by expand; growth path: as the direct build
- checked: ShortShoots site, rank 0, as invalid value `clumpNeighbours`
- dormant: `limbClumping` zero
- blend: rounded to the nearest; dial: none: An approximation budget: neighbours and cell crossings the clumping search may spend (crates/telperion-core/src/foliage/placement.rs:85).

### `/canopy/rosetteFronds`

Fronds the rosette bears at the apex of each stem. At zero no rosette stands and the canopy clothes wood as it always did; any rise makes the rosette the tree's only foliage.

- count (u32), fronds, [0, 128], default 0
- read by grow, plan, expand; growth path: as the direct build
- checked: Rosette site, rank 18, as invalid input `rosette fronds`
- note: Above zero the apical twigs are cleared, fronds are planned instead of runs and the rosette is placed instead of short shoots; the GPU executor falls back.
- blend: rounded to the nearest; dial: `rosette_fronds` (fronds in the crown at each stem apex; at zero no rosette stands), window its bounds (validated bound), steps 1 and 4

### `/canopy/rosetteDivergence`

The degrees each successive frond is turned about the apex.

- real, degrees, [-1000000000, 1000000000], default 137.508
- read by grow, plan, expand; growth path: as the direct build
- checked: Rosette site, rank 0, as invalid input `rosette divergence`
- note: Also the leaf bases' spiral, rosette or none.
- blend: degrees along the shorter arc; dial: `rosette_divergence` (the degrees each successive frond is turned), window [90, 180] (authored), steps 2 and 8

### `/canopy/rosettePitch`

Degrees from the axis the youngest frond stands: 0 upright, 90 level, 180 hanging.

- real, degrees, [0, 180], default 45
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 1, as invalid input `rosette pitch`
- dormant: `rosetteFronds` zero
- blend: linear; dial: `rosette_pitch` (degrees off the axis the youngest frond stands), window its bounds (validated bound), steps 5 and 15

### `/canopy/rosettePitchSpread`

How many degrees further than the youngest the oldest frond leans, so the crown opens from a spike to a skirt.

- real, degrees, [0, 180], default 60
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 2, as invalid input `rosette pitch spread`
- dormant: `rosetteFronds` zero
- blend: linear; dial: `rosette_pitch_spread` (degrees further than that the oldest frond leans), window its bounds (validated bound), steps 5 and 15

### `/canopy/rosetteDepth`

Metres below the apex the frond insertions are spread down the axis. At zero every frond leaves one point.

- real, m, [0, 100], default 0
- read by grow, plan, expand; growth path: as the direct build
- checked: Rosette site, rank 3, as invalid input `rosette depth`
- note: Leaf bases start below it, rosette or none; the deepest frond sets the leaf box's reach.
- blend: linear; dial: `rosette_depth` (metres the frond insertions spread down the axis), window [0, 4] (authored), steps 0.05 and 0.2

### `/canopy/leafletCount`

Leaflets one placement carries along its rachis. One is the single blade every family drew.

- count (u32), leaflets, [1, 256], default 1
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 20, as invalid input `leaflet count`
- note: Leaflets group only above one and with `rachisLength` above zero; the GPU executor draws one blade a station regardless (fn-163).
- blend: rounded to the nearest; dial: `leaflet_count` (leaflets along one frond's rachis), window its bounds (validated bound), steps 2 and 8

### `/canopy/rachisLength`

Metres of rachis the leaflets are strung along. At zero the placement is one blade whatever the count says.

- real, m, [0, 1000], default 0
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 4, as invalid input `rachis length`
- dormant: `leafletCount` one
- blend: linear; dial: `rachis_length` (metres of rachis the leaflets are strung along; at zero one blade), window [0, 8] (authored), steps 0.1 and 0.4

### `/canopy/leafletPitch`

The degrees a leaflet leaves its rachis.

- real, degrees, [0, 90], default 45
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 5, as invalid input `leaflet pitch`
- dormant: no leaflet grouping (`leafletCount` one or `rachisLength` zero)
- blend: linear; dial: `leaflet_pitch` (the degrees a leaflet leaves its rachis), window its bounds (validated bound), steps 3 and 10

### `/canopy/rachisArch`

How far the rachis bends out of the straight line from its station, as a share of its length. Positive arches up, negative droops.

- real, share of the rachis, [-1, 1], default 0
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 6, as invalid input `rachis arch`
- dormant: no leaflet grouping (`leafletCount` one or `rachisLength` zero)
- blend: linear; dial: `rachis_arch` (how far the rachis bends out of its straight line), window its bounds (validated bound), steps 0.05 and 0.2

### `/canopy/terminalLeaflet`

Whether a single leaflet closes the rachis's end, blended 0 to 1: the last leaflet turns from standing off the rachis to lying along it.

- real, share, [0, 1], default 0
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 7, as invalid input `terminal leaflet`
- dormant: no leaflet grouping (`leafletCount` one or `rachisLength` zero)
- blend: linear; dial: `terminal_leaflet` (how far a leaflet closes the rachis's end), window its bounds (validated bound), steps 0.05 and 0.2

### `/canopy/leafBases`

Bases of shed fronds the stem keeps below its crown, clothing the trunk. At zero the trunk is bare and the bark is what it always was; any rise carries the crown's own spiral down it.

- count (u32), bases, [0, 256], default 0
- read by grow; growth path: not read
- checked: Rosette site, rank 21, as invalid input `leaf bases`
- dormant: `leafBaseLength` zero
- note: Does not need a rosette: bases follow the rosette's spiral rows whether fronds stand or not.
- blend: rounded to the nearest; dial: `leaf_bases` (bases of shed fronds the stem keeps below its crown, clothing the trunk; at zero the trunk is bare), window its bounds (validated bound), steps 4 and 16

### `/canopy/leafBaseLength`

Metres a retained base stands out from the bark. At zero no base is drawn whatever the count says.

- real, m, [0, 10], default 0
- read by grow; growth path: not read
- checked: Rosette site, rank 8, as invalid input `leaf base length`
- dormant: `leafBases` zero
- blend: linear; dial: `leaf_base_length` (metres a retained base stands out from the bark; at zero no base is drawn), window its bounds (validated bound), steps 0.05 and 0.2

### `/canopy/leafBaseRadius`

How thick a base is where it leaves the bark, as a share of the stem's own radius there. Raising it leaves a broader boot.

- real, share of stem radius, [0, 1], default 0.35
- read by grow; growth path: not read
- checked: Rosette site, rank 9, as invalid input `leaf base radius`
- dormant: no leaf base
- blend: linear; dial: `leaf_base_radius` (how thick a base is where it leaves the bark, as a share of the stem's radius there), window its bounds (validated bound), steps 0.05 and 0.15

### `/canopy/leafBasePitch`

Degrees from the stem's axis a base points: 0 flat against the trunk, 90 square out of it, 180 turned back down.

- real, degrees, [0, 180], default 60
- read by grow; growth path: not read
- checked: Rosette site, rank 10, as invalid input `leaf base pitch`
- dormant: no leaf base
- note: Clamped clear of the axis where `leafBaseWidth` is above zero.
- blend: degrees along the shorter arc; dial: `leaf_base_pitch` (degrees from the stem's axis a base points: 0 flat against the trunk, 90 square out), window its bounds (validated bound), steps 5 and 15

### `/canopy/leafBaseWeathering`

How far the lowest and oldest base is worn back against the newest, in both its length and its girth. At zero every base stands full down the whole trunk, and any rise wears the foot away.

- real, share, [0, 1], default 0
- read by grow; growth path: not read
- checked: Rosette site, rank 11, as invalid input `leaf base weathering`
- dormant: no leaf base
- blend: linear; dial: `leaf_base_weathering` (how far the lowest and oldest base is worn back against the newest; at zero none is worn), window its bounds (validated bound), steps 0.05 and 0.2

### `/canopy/leafBaseWidth`

How broad a retained base is across the trunk, as a share of the cell the crown's spiral gives it on the bark: at 1 every base meets its neighbours edge to edge whatever the count and the trunk's girth, below it the bark shows between them and above it they crowd into each other. At zero the base is the round peg the radius row sizes and the lattice rows say nothing; any rise packs the bases into the lattice.

- real, share of the lattice cell, [0, 2], default 0
- read by grow; growth path: not read
- checked: Rosette site, rank 12, as invalid input `leaf base width`
- dormant: no leaf base
- note: Above zero bases pack into a lattice of flat-faced boots.
- blend: linear; dial: `leaf_base_width` (how broad a retained base is across the trunk as a share of the cell the crown's spiral gives it: 1 meets its neighbours edge to edge; at zero the base is a round peg and any rise packs the bases into the lattice), window its bounds (validated bound), steps 0.05 and 0.2

### `/canopy/leafBaseFlatness`

How flat-sided a lattice base is drawn: 0 the ellipse through its cell's corners, 1 the cell itself, a diamond with flat faces that meets each neighbour along a straight edge and is cut square at its outer end. At zero the section stays round, and at no width it reads nothing.

- real, share, [0, 1], default 0
- read by grow; growth path: not read
- checked: Rosette site, rank 13, as invalid input `leaf base flatness`
- dormant: no leaf base, or `leafBaseWidth` zero
- blend: linear; dial: `leaf_base_flatness` (how flat-sided a packed base is drawn: 1 the diamond cell with flat faces, cut square at its end; at zero the section stays round), window its bounds (validated bound), steps 0.1 and 0.25

### `/canopy/acanthophylls`

Leaflets at a frond's base borne as spines rather than blades. At zero the frond carries blades all the way down; any rise hardens that many of them.

- count (u32), leaflets, [0, 256], default 0
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 22, as invalid input `acanthophylls`
- dormant: `acanthophyllLength` zero, or no leaflet grouping (`leafletCount` one or `rachisLength` zero)
- blend: rounded to the nearest; dial: `acanthophylls` (leaflets at a frond's base borne as spines rather than blades; at zero the frond carries blades), window its bounds (validated bound), steps 1 and 4

### `/canopy/acanthophyllLength`

The share of a leaflet's own size a spine is drawn at. At zero no spine is drawn whatever the count says.

- real, share of a leaflet, [0, 1], default 0.35
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 14, as invalid input `acanthophyll length`
- dormant: `acanthophylls` zero, or no leaflet grouping (`leafletCount` one or `rachisLength` zero)
- blend: linear; dial: `acanthophyll_length` (the share of a leaflet's own size a spine is drawn at; at zero no spine is drawn), window its bounds (validated bound), steps 0.05 and 0.15

### `/canopy/acanthophyllPitch`

The degrees a spine leaves the rachis, in place of the leaflet's own pitch.

- real, degrees, [0, 90], default 80
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 15, as invalid input `acanthophyll pitch`
- dormant: no spine
- blend: degrees along the shorter arc; dial: `acanthophyll_pitch` (the degrees a spine leaves the rachis, in place of the leaflet's own pitch), window its bounds (validated bound), steps 3 and 10

### `/canopy/skirtFronds`

Dead fronds a rosette keeps below its living crown, continuing the crown's own spiral down the stem. At zero no frond is kept and the crown ends at its oldest living frond; any rise hangs that many.

- count (u32), fronds, [0, 128], default 0
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 19, as invalid input `skirt fronds`
- dormant: `skirtLength` zero, or `rosetteFronds` zero
- blend: rounded to the nearest; dial: `skirt_fronds` (dead fronds a rosette keeps below its living crown; at zero none is kept), window its bounds (validated bound), steps 2 and 6

### `/canopy/skirtPitch`

Degrees from the axis a dead frond hangs: 0 upright, 90 level, 180 collapsed straight down against the stem.

- real, degrees, [0, 180], default 140
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 16, as invalid input `skirt pitch`
- dormant: no dead frond
- blend: linear; dial: `skirt_pitch` (degrees from the axis a dead frond hangs: 90 level, 180 collapsed against the stem), window its bounds (validated bound), steps 5 and 15

### `/canopy/skirtLength`

A dead frond's length as a share of a living one's, rachis and leaflets alike. At zero no dead frond is drawn whatever the count says.

- real, share of a living frond, [0, 1], default 0.8
- read by plan, expand; growth path: as the direct build
- checked: Rosette site, rank 17, as invalid input `skirt length`
- dormant: `skirtFronds` zero, or `rosetteFronds` zero
- blend: linear; dial: `skirt_length` (a dead frond's length as a share of a living one's; at zero no dead frond is drawn), window its bounds (validated bound), steps 0.05 and 0.15

### `/canopy/maxInstances`

Hard total budget. Exceeding it returns an error, never partial foliage.

- count (usize), leaves, [1, 18446744073709552000], default 18446744073709552000
- read by plan, expand; growth path: `usize::MAX` turns on the sparse validation interval
- checked: Canopy site, rank 13, as invalid input `foliage instance budget`
- note: Exceeding it is an error, never fewer leaves.
- blend: rounded to the nearest; dial: none: A hard resource cap; exceeding it is an error, never a different tree (crates/telperion-core/src/foliage/placement.rs:88).

## `/element`

### `/element/connectorLength`

Metres of petiole or woody peg below the blade, excluded from unit dimensions. Every element carries one.

- real, m, [0.000001, 1000], default 0.01
- read by plan; growth path: as the direct build
- checked: by a named check or not at all (see note)
- dormant: `card`
- note: At most `length` and at least 1e-6, refused by name (`foliage connector length`) in `build_element`; a card carries none.
- blend: linear; dial: `connector_length` (the metres of stalk below the blade), window [0.000001, 0.0325] (capped: ceiling capped: the generator validates only a floor here (1e-6; its ceiling is the leaf's own length, another row), so there is no validated ceiling to widen to; the ceiling stays at the preset span until the generator authors one), steps 0.005 and 0.01, presets span [0.000001, 0.0325]

### `/element/length`

Blade/needle longitudinal extent, excluding connector, in metres.

- real, m, [0.0001, 1000], default 0.12
- read by plan; growth path: as the direct build
- checked: Leaf site, rank 0, as invalid input `leaf length`
- blend: linear; dial: `leaf_length` (the blade's length in metres, excluding its stalk), window its bounds (validated bound), steps 0.025 and 0.05, presets span [0.0001, 0.171]

### `/element/width`

The blade's greatest width in metres, across the midrib. Raising it makes every leaf broader.

- real, m, [0.0001, 1000], default 0.06
- read by plan; growth path: as the direct build
- checked: Leaf site, rank 1, as invalid input `leaf width`
- blend: linear; dial: `leaf_width` (the blade's greatest width in metres), window its bounds (validated bound), steps 0.015 and 0.03, presets span [0.0001, 0.11175]

### `/element/widestAt`

Where the blade is widest, as a share of the way from its base to its tip. Raising it carries the widest point toward the tip.

- real, share of the blade, [0.05, 0.95], default 0.42
- read by plan; growth path: as the direct build
- checked: Leaf site, rank 2, as invalid input `leaf widest point`
- dormant: `card`
- blend: linear; dial: `leaf_widest_at` (where the blade is widest, from its base at nought to its tip at one), window its bounds (validated bound), steps 0.15 and 0.3

### `/element/baseFullness`

How fast the blade fills out above its stalk. Raising it draws the base in, so the leaf reads wedge-shaped rather than rounded.

- real, -, [0.2, 8], default 0.85
- read by plan; growth path: as the direct build
- checked: Leaf site, rank 3, as invalid input `leaf base fullness`
- dormant: `card`
- blend: linear; dial: `leaf_base_fullness` (how fast the blade fills out above its stalk; higher reads wedge-shaped), window its bounds (validated bound), steps 0.15 and 0.3, presets span [0.2, 1.175]

### `/element/tipSharpness`

How fast the blade narrows toward its point. Raising it draws the tip out into a sharper point.

- real, -, [0.2, 8], default 1.6
- read by plan; growth path: as the direct build
- checked: Leaf site, rank 4, as invalid input `leaf tip sharpness`
- dormant: `card`
- blend: linear; dial: `leaf_tip_sharpness` (how fast the blade narrows to its point), window its bounds (validated bound), steps 1 and 2

### `/element/cup`

How far the blade's margins lift out of its own plane, as a share of the half-width there. Raising it dishes the leaf more deeply along the midrib.

- real, share of the half-width, [-2, 2], default 0.18
- read by plan; growth path: as the direct build
- checked: Leaf site, rank 5, as invalid input `leaf cup`
- dormant: `card`
- blend: linear; dial: `leaf_cup` (how far the blade's margins lift out of its own plane), window its bounds (validated bound), steps 0.5 and 1

### `/element/curl`

How far the blade bends along its length, as a share of its length at the tip. Raising it curls the tip further out of the plane its base stands in.

- real, share of the length, [-2, 2], default 0.12
- read by plan; growth path: as the direct build
- checked: Leaf site, rank 6, as invalid input `leaf curl`
- dormant: `card`
- blend: linear; dial: `leaf_curl` (how far the blade bends along its length), window its bounds (validated bound), steps 0.5 and 1

### `/element/lobeCount`

Lobes along each margin; 0 is an entire margin.

- count (u32), lobes, [0, 8], default 0
- read by plan; growth path: as the direct build
- checked: LeafCounts site, rank 0, as invalid input `leaf lobe count`
- dormant: `lobeDepth` zero
- note: Refused above zero on a card.
- blend: rounded down; dial: `leaf_lobe_count` (lobes along each margin; zero is an entire margin), window its bounds (validated bound), steps 1 and 2

### `/element/lobeDepth`

How far each sinus cuts toward the midrib, 0 to 1. At zero the margin is entire whatever the lobe count says, and any rise starts cutting the sinuses.

- real, share, [0, 1], default 0
- read by plan; growth path: as the direct build
- checked: Leaf site, rank 7, as invalid input `leaf lobe depth`
- blend: linear; dial: `leaf_lobe_depth` (how far each sinus cuts toward the midrib; at zero the margin is entire whatever the lobe count says, and any rise starts cutting the sinuses), window its bounds (validated bound), steps 0.15 and 0.3

### `/element/sectionRoundness`

Flat blade at 0, four-sided shaft at 1.

- real, share, [0, 1], default 0
- read by plan, draw; growth path: as the direct build
- checked: Leaf site, rank 8, as invalid input `leaf section roundness`
- note: Refused above zero on a card; 0.5 or more marks the unit a needle; the renderer shades by it.
- blend: linear; dial: `leaf_section_roundness` (the blade's section, flat at nought and a four-sided needle at one), window its bounds (validated bound), steps 0.15 and 0.3

### `/element/axialSegments`

How many sections the blade is built from along its length. Raising it draws the outline and any lobes more smoothly, at more triangles per leaf.

- count (u32), sections, [2, 64], default 5
- read by plan; growth path: as the direct build
- checked: LeafCounts site, rank 1, as invalid input `leaf segments`
- dormant: `card`
- note: A lobed margin needs `axialSegments + 1 >= 2·lobeCount + 2` (named check in `build_element`).
- blend: rounded up; dial: `leaf_axial_segments` (sections the blade is built from along its length), window its bounds (validated bound), steps 8 and 16, presets span [2, 58]

### `/element/crossSegments`

How many columns the blade is built from across its width. Raising it draws the section and the margins more smoothly, at more triangles per leaf.

- count (u32), columns, [2, 64], default 2
- read by plan; growth path: as the direct build
- checked: LeafCounts site, rank 2, as invalid input `leaf segments`
- dormant: `card`
- note: Rounded up to an even count.
- blend: rounded to the nearest; dial: `leaf_cross_segments` (columns the blade is built from across its width), window its bounds (validated bound), steps 1 and 2, presets span [2, 5]

### `/element/card`

A flat two-triangle card in place of the modelled blade: no outline, no cup or curl, no sections and no connector.

- switch, switch, [0, 1], default 0
- read by plan; growth path: as the direct build
- checked: by a named check or not at all (see note)
- note: Lobes and roundness are refused on a card. A walk from a card to a leaf is a leaf.
- blend: with the rows it is coupled to; dial: none: Boolean, not a numeric scalar.

## `/material`

### `/material/barkRed`

The bark's own colour, a linear reflectance per channel. Raising a channel pushes mature bark toward that colour.

- real, reflectance, [0, 1], default 0.147
- read by draw; growth path: as the direct build
- checked: Material site, rank 73, as invalid input `bark red`
- blend: linear; dial: `material_bark_red` (the red in the bark's own colour), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/barkGreen`

The green channel of `bark_red`.

- real, reflectance, [0, 1], default 0.105
- read by draw; growth path: as the direct build
- checked: Material site, rank 74, as invalid input `bark green`
- blend: linear; dial: `material_bark_green` (the green in the bark's own colour), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/barkBlue`

The blue channel of `bark_red`.

- real, reflectance, [0, 1], default 0.068
- read by draw; growth path: as the direct build
- checked: Material site, rank 75, as invalid input `bark blue`
- blend: linear; dial: `material_bark_blue` (the blue in the bark's own colour), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/barkRoughness`

How diffuse the bark is: 0 is a mirror, 1 is chalk.

- real, -, [0, 1], default 0.8
- read by draw; growth path: as the direct build
- checked: Material site, rank 76, as invalid input `bark roughness`
- blend: linear; dial: `material_bark_roughness` (how diffuse the bark is; nought is a mirror and one is chalk), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/shootRed`

The young wood's own colour, before its bark has formed. Wood thinner than `shoot_radius` takes it, and gives it up to the bark colour on a smoothstep of its radius by twice that; zero means no wood is young.

- real, reflectance, [0, 1], default 0.147
- read by draw; growth path: as the direct build
- checked: Material site, rank 77, as invalid input `young shoot red`
- dormant: `shootRadius` zero
- blend: linear; dial: `material_shoot_red` (the red in young wood's colour, before its bark has formed), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/shootGreen`

The green channel of `shoot_red`.

- real, reflectance, [0, 1], default 0.105
- read by draw; growth path: as the direct build
- checked: Material site, rank 78, as invalid input `young shoot green`
- dormant: `shootRadius` zero
- blend: linear; dial: `material_shoot_green` (the green in young wood's colour, before its bark has formed), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/shootBlue`

The blue channel of `shoot_red`.

- real, reflectance, [0, 1], default 0.068
- read by draw; growth path: as the direct build
- checked: Material site, rank 79, as invalid input `young shoot blue`
- dormant: `shootRadius` zero
- blend: linear; dial: `material_shoot_blue` (the blue in young wood's colour, before its bark has formed), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/shootRadius`

Metres. The radius below which wood is young.

- real, m, [0, 0.1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 80, as invalid input `young shoot radius`
- blend: linear; dial: `material_shoot_radius` (the metres of radius below which wood counts as young), window its bounds (validated bound), steps 0.015 and 0.03

### `/material/leafFrontRed`

The colour of a leaf's upper face, a linear reflectance per channel. Raising a channel pushes the sunlit face toward it.

- real, reflectance, [0, 1], default 0.068
- read by draw; growth path: as the direct build
- checked: Material site, rank 81, as invalid input `leaf front red`
- blend: linear; dial: `material_leaf_front_red` (the red in the colour of a leaf's upper face), window its bounds (validated bound), steps 0.02 and 0.04, presets span [0, 0.141]

### `/material/leafFrontGreen`

The green channel of `leaf_front_red`.

- real, reflectance, [0, 1], default 0.195
- read by draw; growth path: as the direct build
- checked: Material site, rank 82, as invalid input `leaf front green`
- blend: linear; dial: `material_leaf_front_green` (the green in the colour of a leaf's upper face), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/leafFrontBlue`

The blue channel of `leaf_front_red`.

- real, reflectance, [0, 1], default 0.036
- read by draw; growth path: as the direct build
- checked: Material site, rank 83, as invalid input `leaf front blue`
- blend: linear; dial: `material_leaf_front_blue` (the blue in the colour of a leaf's upper face), window its bounds (validated bound), steps 0.015 and 0.03, presets span [0, 0.082]

### `/material/leafBackRed`

The colour of a leaf's underside, shown wherever the eye sees the back of a blade. Raising a channel pushes that face toward it.

- real, reflectance, [0, 1], default 0.105
- read by draw; growth path: as the direct build
- checked: Material site, rank 84, as invalid input `leaf back red`
- blend: linear; dial: `material_leaf_back_red` (the red in the colour of a leaf's underside), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/leafBackGreen`

The green channel of `leaf_back_red`.

- real, reflectance, [0, 1], default 0.24
- read by draw; growth path: as the direct build
- checked: Material site, rank 85, as invalid input `leaf back green`
- blend: linear; dial: `material_leaf_back_green` (the green in the colour of a leaf's underside), window its bounds (validated bound), steps 0.025 and 0.05, presets span [0.1525, 0.3225]

### `/material/leafBackBlue`

The blue channel of `leaf_back_red`.

- real, reflectance, [0, 1], default 0.07
- read by draw; growth path: as the direct build
- checked: Material site, rank 86, as invalid input `leaf back blue`
- blend: linear; dial: `material_leaf_back_blue` (the blue in the colour of a leaf's underside), window its bounds (validated bound), steps 0.025 and 0.05, presets span [0.03, 0.19]

### `/material/leafDeadRed`

The colour a dead frond has aged to, both faces alike, shown only on the fronds a rosette keeps below its living crown. Raising a channel pushes the skirt toward it.

- real, reflectance, [0, 1], default 0.3
- read by draw; growth path: as the direct build
- checked: Material site, rank 87, as invalid input `leaf dead red`
- dormant: no dead frond (`canopy.skirtFronds` zero)
- blend: linear; dial: `material_leaf_dead_red` (the red in the colour a dead frond has aged to), window its bounds (validated bound), steps 0.05 and 0.1

### `/material/leafDeadGreen`

The green channel of `leaf_dead_red`.

- real, reflectance, [0, 1], default 0.25
- read by draw; growth path: as the direct build
- checked: Material site, rank 88, as invalid input `leaf dead green`
- dormant: no dead frond (`canopy.skirtFronds` zero)
- blend: linear; dial: `material_leaf_dead_green` (the green in the colour a dead frond has aged to), window its bounds (validated bound), steps 0.05 and 0.1

### `/material/leafDeadBlue`

The blue channel of `leaf_dead_red`.

- real, reflectance, [0, 1], default 0.18
- read by draw; growth path: as the direct build
- checked: Material site, rank 89, as invalid input `leaf dead blue`
- dormant: no dead frond (`canopy.skirtFronds` zero)
- blend: linear; dial: `material_leaf_dead_blue` (the blue in the colour a dead frond has aged to), window its bounds (validated bound), steps 0.05 and 0.1

### `/material/hueRangeLow`

The hue offsets one leaf may take, as a fraction of the colour circle.

- real, -, [-0.5, 0.5], default -0.03
- read by draw; growth path: as the direct build
- checked: Material site, rank 90, as invalid input `leaf hue range low`
- note: Refused above `hueRangeHigh` (`leaf hue range`).
- blend: linear; dial: `material_hue_range_low` (the lowest hue offset one leaf may take, as a share of the colour circle), window [-0.5, 0] (validated bound), steps 0.0025 and 0.005, presets span [-0.035, -0.015]

### `/material/hueRangeHigh`

The upper end of the hue offsets `hue_range_low` opens.

- real, -, [-0.5, 0.5], default 0.03
- read by draw; growth path: as the direct build
- checked: Material site, rank 91, as invalid input `leaf hue range high`
- note: Refused below `hueRangeLow` (`leaf hue range`).
- blend: linear; dial: `material_hue_range_high` (the highest hue offset one leaf may take, as a share of the colour circle), window [0, 0.5] (validated bound), steps 0.0025 and 0.005, presets span [0.015, 0.035]

### `/material/brightnessRangeLow`

The brightness offsets one leaf may take, about no change at all.

- real, -, [-1, 1], default -0.12
- read by draw; growth path: as the direct build
- checked: Material site, rank 92, as invalid input `leaf brightness range low`
- note: Refused above `brightnessRangeHigh` (`leaf brightness range`).
- blend: linear; dial: `material_brightness_range_low` (the lowest brightness offset one leaf may take), window [-1, 0] (validated bound), steps 0.015 and 0.03, presets span [-0.175, -0.075]

### `/material/brightnessRangeHigh`

The upper end of the brightness offsets `brightness_range_low` opens.

- real, -, [-1, 1], default 0.12
- read by draw; growth path: as the direct build
- checked: Material site, rank 93, as invalid input `leaf brightness range high`
- note: Refused below `brightnessRangeLow` (`leaf brightness range`).
- blend: linear; dial: `material_brightness_range_high` (the highest brightness offset one leaf may take), window [0, 1] (validated bound), steps 0.015 and 0.03, presets span [0.075, 0.175]

### `/material/interiorDarkening`

How far a leaf deep inside the crown is darkened towards a shaded mass.

- real, -, [0, 1], default 0.5
- read by draw; growth path: as the direct build
- checked: Material site, rank 94, as invalid input `leaf interior darkening`
- blend: linear; dial: `material_interior_darkening` (how far a leaf deep inside the crown is darkened toward a shaded mass), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/ridgeScale`

Circumferential ridge spacing in metres; zero disables relief.

- real, m, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 62, as invalid input `bark ridge scale`
- blend: linear; dial: `material_ridge_scale` (the metres between the bark's circumferential ridges; zero leaves no relief), window its bounds (validated bound), steps 0.02 and 0.04, presets span [0, 0.12]

### `/material/plateScale`

Axial scale control in metres; spacing is bounded to 1.5–2 ridge widths. Larger ratios lengthen and deepen furrows; zero omits breaks.

- real, m, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 63, as invalid input `bark plate scale`
- dormant: `ridgeScale` zero
- blend: linear; dial: `material_plate_scale` (the axial scale of the bark's plates, in metres; larger lengthens and deepens the furrows), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/furrowStrength`

Furrow width and depth together; zero leaves tightly packed scales.

- real, -, [0, 1], default 1
- read by draw; growth path: as the direct build
- checked: Material site, rank 64, as invalid input `bark furrow strength`
- dormant: `ridgeScale` zero
- blend: linear; dial: `material_furrow_strength` (the width and depth of the bark's furrows together; at zero no furrow is cut at all, and any rise starts them), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/roughnessDetail`

Roughness variation about the base value, clamped to 0..1.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 65, as invalid input `bark roughness detail`
- blend: linear; dial: `material_roughness_detail` (how far roughness varies about the bark's base value), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/veinScale`

Secondary vein pairs per blade, continuously interpolated.

- real, -, [0, 32], default 8
- read by draw; growth path: as the direct build
- checked: Material site, rank 66, as invalid input `leaf vein scale`
- blend: linear; dial: `material_vein_scale` (secondary vein pairs per blade), window its bounds (validated bound), steps 0.5 and 1, presets span [5, 9]

### `/material/veinContrast`

How far the veins are lightened and the blade between them darkened. Raising it makes the venation read more sharply.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 67, as invalid input `leaf vein contrast`
- blend: linear; dial: `material_vein_contrast` (how far the veins are lightened against the blade between them), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/transmissionStrength`

How brightly a backlit leaf glows with the light that came through it. Raising it lifts that glow.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 68, as invalid input `leaf transmission strength`
- blend: linear; dial: `material_transmission_strength` (how brightly a backlit leaf glows with the light that came through it), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/transmissionRed`

Linear transmission tint, in 0..1 per channel.

- real, reflectance, [0, 1], default 0.3
- read by draw; growth path: as the direct build
- checked: Material site, rank 69, as invalid input `leaf transmission red`
- dormant: `transmissionStrength` zero
- blend: linear; dial: `material_transmission_red` (the red in the tint of the light that comes through a leaf), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/transmissionGreen`

The green channel of `transmission_red`.

- real, reflectance, [0, 1], default 0.6
- read by draw; growth path: as the direct build
- checked: Material site, rank 70, as invalid input `leaf transmission green`
- dormant: `transmissionStrength` zero
- blend: linear; dial: `material_transmission_green` (the green in the tint of the light that comes through a leaf), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/transmissionBlue`

The blue channel of `transmission_red`.

- real, reflectance, [0, 1], default 0.1
- read by draw; growth path: as the direct build
- checked: Material site, rank 71, as invalid input `leaf transmission blue`
- dormant: `transmissionStrength` zero
- blend: linear; dial: `material_transmission_blue` (the blue in the tint of the light that comes through a leaf), window its bounds (validated bound), steps 0.01 and 0.02, presets span [0.055, 0.115]

### `/material/thickness`

Optical thickness: attenuation is exp(-thickness).

- real, -, [0, 8], default 1
- read by draw; growth path: as the direct build
- checked: Material site, rank 72, as invalid input `leaf thickness`
- dormant: `transmissionStrength` zero
- blend: linear; dial: `material_thickness` (the blade's optical thickness; what comes through falls off with it), window its bounds (validated bound), steps 1 and 2

### `/material/fissureRed`

The tint carried by the floors of the bark's furrows, as an offset per channel, and how far it is laid over the bark colour there.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 0, as invalid input `bark fissure red`
- dormant: `fissureStrength` zero
- blend: linear; dial: `material_fissure_red` (the red in the tint carried by the floors of the bark's furrows), window its bounds (validated bound), steps 0.25 and 0.5

### `/material/fissureGreen`

The green channel of `fissure_red`.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 1, as invalid input `bark fissure green`
- dormant: `fissureStrength` zero
- blend: linear; dial: `material_fissure_green` (the green in the tint carried by the floors of the bark's furrows), window its bounds (validated bound), steps 0.25 and 0.5

### `/material/fissureBlue`

The blue channel of `fissure_red`.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 2, as invalid input `bark fissure blue`
- dormant: `fissureStrength` zero
- blend: linear; dial: `material_fissure_blue` (the blue in the tint carried by the floors of the bark's furrows), window its bounds (validated bound), steps 0.25 and 0.5

### `/material/fissureStrength`

How far `fissure_red`'s tint is laid over the bark colour in the furrows.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 3, as invalid input `bark fissure strength`
- blend: linear; dial: `material_fissure_strength` (how far the furrow tint is laid over the bark colour; at zero the furrow tint is not laid on at all, and any rise starts it), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/crestRed`

The tint carried by the crests of the bark's ridges, as an offset per channel, and how far it is laid over the bark colour there.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 4, as invalid input `bark crest red`
- dormant: `crestStrength` zero
- blend: linear; dial: `material_crest_red` (the red in the tint carried by the crests of the bark's ridges), window its bounds (validated bound), steps 0.025 and 0.05, presets span [-0.05, 0.15]

### `/material/crestGreen`

The green channel of `crest_red`.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 5, as invalid input `bark crest green`
- dormant: `crestStrength` zero
- blend: linear; dial: `material_crest_green` (the green in the tint carried by the crests of the bark's ridges), window its bounds (validated bound), steps 0.025 and 0.05, presets span [-0.05, 0.15]

### `/material/crestBlue`

The blue channel of `crest_red`.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 6, as invalid input `bark crest blue`
- dormant: `crestStrength` zero
- blend: linear; dial: `material_crest_blue` (the blue in the tint carried by the crests of the bark's ridges), window its bounds (validated bound), steps 0.025 and 0.05, presets span [-0.05, 0.15]

### `/material/crestStrength`

How far `crest_red`'s tint is laid over the bark colour on the crests.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 7, as invalid input `bark crest strength`
- blend: linear; dial: `material_crest_strength` (how far the crest tint is laid over the bark colour; at zero the crest tint is not laid on at all, and any rise starts it), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/barkMottleScale`

The size of the blotches in the bark's mottling, and how far they lighten and darken its colour. A scale of zero leaves none.

- real, -, [0, 8], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 8, as invalid input `bark mottle scale`
- dormant: `barkMottleStrength` zero
- blend: linear; dial: `material_bark_mottle_scale` (the size of the blotches in the bark's mottling; zero leaves none), window its bounds (validated bound), steps 1 and 2

### `/material/barkMottleStrength`

How far the blotches `bark_mottle_scale` sizes lighten and darken the bark.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 9, as invalid input `bark mottle strength`
- dormant: `barkMottleScale` zero
- blend: linear; dial: `material_bark_mottle_strength` (how far that mottling lightens and darkens the bark), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/cavityStrength`

How far the hollows of the bark - furrow floors and the sockets where a limb joins - are darkened. Raising it sinks them deeper.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 10, as invalid input `bark cavity strength`
- blend: linear; dial: `material_cavity_strength` (how far the bark's hollows and the sockets at a fork are darkened), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/bladeMottleScale`

The size of the blotches in a leaf's mottling, and how far they vary its colour. A scale of zero leaves none.

- real, -, [0, 32], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 11, as invalid input `leaf blade mottle scale`
- dormant: `bladeMottleStrength` zero
- blend: linear; dial: `material_blade_mottle_scale` (the size of the blotches in a leaf's mottling; zero leaves none), window its bounds (validated bound), steps 2 and 4, presets span [0, 12]

### `/material/bladeMottleStrength`

How far the blotches `blade_mottle_scale` sizes vary a leaf's colour.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 12, as invalid input `leaf blade mottle strength`
- dormant: `bladeMottleScale` zero
- blend: linear; dial: `material_blade_mottle_strength` (how far that mottling varies the blade's colour), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/marginWidth`

How wide a band along the leaf's edge takes the margin colour, and what that colour is as an offset per channel. Raising the width broadens the rim around every leaf.

- real, -, [0, 0.5], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 13, as invalid input `leaf margin width`
- blend: linear; dial: `material_margin_width` (how wide a band along the leaf's edge takes the margin tint), window its bounds (validated bound), steps 0.1 and 0.2

### `/material/marginRed`

The margin colour's red channel, as an offset; `margin_width` sets its band.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 14, as invalid input `leaf margin red`
- dormant: `marginWidth` zero
- blend: linear; dial: `material_margin_red` (the red in the tint along a leaf's edge), window its bounds (validated bound), steps 0.01 and 0.02, presets span [-0.015, 0.045]

### `/material/marginGreen`

The margin colour's green channel, as an offset; `margin_width` sets its band.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 15, as invalid input `leaf margin green`
- dormant: `marginWidth` zero
- blend: linear; dial: `material_margin_green` (the green in the tint along a leaf's edge), window its bounds (validated bound), steps 0.015 and 0.03, presets span [-0.025, 0.075]

### `/material/marginBlue`

The margin colour's blue channel, as an offset; `margin_width` sets its band.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 16, as invalid input `leaf margin blue`
- dormant: `marginWidth` zero
- blend: linear; dial: `material_margin_blue` (the blue in the tint along a leaf's edge), window its bounds (validated bound), steps 0.0025 and 0.005, presets span [-0.005, 0.015]

### `/material/cuticleGloss`

How tight the highlight on a leaf's upper face is. Raising it draws the glint into a smaller, glossier spot.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 17, as invalid input `leaf cuticle gloss`
- blend: linear; dial: `material_cuticle_gloss` (how tight the highlight on a leaf's upper face is), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/skyOcclusionStrength`

How much of the sky is withheld from bark and leaves the deeper they sit in the crown. Raising it darkens the crown's interior.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 18, as invalid input `sky occlusion strength`
- blend: linear; dial: `material_sky_occlusion_strength` (how much of the sky is withheld from bark and leaves deep in the crown), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/plateCellScale`

Circumferential size of one bark plate in metres, before girth scales it. Zero leaves the field the ridges it has always been.

- real, m, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 19, as invalid input `bark plate cell scale`
- dormant: `ridgeScale` zero
- blend: linear; dial: `material_plate_cell_scale` (the metres across one bark plate, before girth scales it; at zero the bark has no plates at all, and any rise switches plates on), window its bounds (validated bound), steps 0.02 and 0.04, presets span [0, 0.126]

### `/material/plateElongation`

How much longer a plate runs than it is wide: nought is as long as it is wide, one is twice as long.

- real, -, [0, 16], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 20, as invalid input `bark plate elongation`
- dormant: `plateCellScale` or `ridgeScale` zero
- blend: linear; dial: `material_plate_elongation` (how much longer a plate runs than it is wide), window its bounds (validated bound), steps 2.5 and 5

### `/material/plateDome`

How far a plate's face rises from its own edge towards its middle. At zero a plate's face lies flat, and any rise starts the doming.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 21, as invalid input `bark plate dome`
- dormant: `plateCellScale` or `ridgeScale` zero
- blend: linear; dial: `material_plate_dome` (how far a plate's face rises from its edge toward its middle; at zero a plate's face lies flat, and any rise starts the doming), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/plateEdgeLift`

How far a plate's rim stands off the furrow it borders: the scale that lifts rather than the plate that sits flat. At zero a plate's rim lies flush with its furrow, and any rise starts the lift.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 22, as invalid input `bark plate edge lift`
- dormant: `plateCellScale` or `ridgeScale` zero
- blend: linear; dial: `material_plate_edge_lift` (how far a plate's rim stands off the furrow it borders; at zero a plate's rim lies flush with its furrow, and any rise starts the lift), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/plateFurrowWidth`

How wide the flat floor of a furrow is cut, as a fraction of a plate's own width, so a bigger plate carries a wider furrow off one row. Zero leaves the hairline the network has always cut between two faces.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 23, as invalid input `bark plate furrow width`
- dormant: `plateCellScale` or `ridgeScale` zero
- blend: linear; dial: `material_plate_furrow_width` (how wide the flat floor of a furrow is cut, as a share of a plate's width), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/plateEdgeShape`

Blend from rounded plate edges to narrow chipped scales; independent of lichen/lenticel/peel.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 24, as invalid input `bark plate edge shape`
- blend: linear; dial: `material_plate_edge_shape` (the blend from rounded plate edges to narrow chipped scales; at zero every plate edge is rounded, and any rise starts the chipping), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/plateIdentity`

How much of its own a plate keeps: how proud it stands, how it leans, and the value and cast it holds against its neighbours. At zero every plate stands and reads exactly like its neighbours, and any rise starts the difference.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 25, as invalid input `bark plate identity`
- dormant: `plateCellScale` or `ridgeScale` zero
- blend: linear; dial: `material_plate_identity` (how much of its own a plate keeps against its neighbours; at zero every plate stands and reads exactly like its neighbours, and any rise starts the difference), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/weatheringStrength`

How far a weathered face is greyed and tinted against a fresh furrow. At zero no face is weathered at all, and any rise starts the greying.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 26, as invalid input `bark weathering strength`
- blend: linear; dial: `material_weathering_strength` (how far a weathered face is greyed against a fresh furrow; at zero no face is weathered at all, and any rise starts the greying), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/weatheringRed`

The tint a weathered face takes, as an offset per channel; how far it is laid on is `weathering_strength` above.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 27, as invalid input `bark weathering red`
- dormant: `weatheringStrength` zero
- blend: linear; dial: `material_weathering_red` (the red in the tint a weathered face takes), window its bounds (validated bound), steps 0.005 and 0.01, presets span [-0.01125, 0.03375]

### `/material/weatheringGreen`

The green channel of `weathering_red`.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 28, as invalid input `bark weathering green`
- dormant: `weatheringStrength` zero
- blend: linear; dial: `material_weathering_green` (the green in the tint a weathered face takes), window its bounds (validated bound), steps 0.01 and 0.02, presets span [-0.0175, 0.0525]

### `/material/weatheringBlue`

The blue channel of `weathering_red`.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 29, as invalid input `bark weathering blue`
- dormant: `weatheringStrength` zero
- blend: linear; dial: `material_weathering_blue` (the blue in the tint a weathered face takes), window its bounds (validated bound), steps 0.01 and 0.02, presets span [-0.014875, 0.044625]

### `/material/orientationStrength`

How far the side away from the sun and the foot of the trunk take a colour of their own - what damp growth would look like, not what it is. At zero the shaded side and the foot take no colour of their own, and any rise starts it.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 30, as invalid input `bark orientation strength`
- blend: linear; dial: `material_orientation_strength` (how far the shaded side and the foot of the trunk take a colour of their own; at zero the shaded side and the foot take no colour of their own, and any rise starts it), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/orientationRed`

The tint that damp side takes, as an offset per channel; how far it is laid on is `orientation_strength` above.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 31, as invalid input `bark orientation red`
- dormant: `orientationStrength` zero
- blend: linear; dial: `material_orientation_red` (the red in the tint the shaded side and the foot of the trunk take), window its bounds (validated bound), steps 0.02 and 0.04, presets span [-0.093, 0.031]

### `/material/orientationGreen`

The green channel of `orientation_red`.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 32, as invalid input `bark orientation green`
- dormant: `orientationStrength` zero
- blend: linear; dial: `material_orientation_green` (the green in the tint the shaded side and the foot of the trunk take), window its bounds (validated bound), steps 0.01 and 0.02, presets span [-0.013, 0.039]

### `/material/orientationBlue`

The blue channel of `orientation_red`.

- real, offset, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 33, as invalid input `bark orientation blue`
- dormant: `orientationStrength` zero
- blend: linear; dial: `material_orientation_blue` (the blue in the tint the shaded side and the foot of the trunk take), window its bounds (validated bound), steps 0.05 and 0.1, presets span [-0.23025, 0.07675]

### `/material/directionalOcclusion`

How far a furrow floor is darkened by its own crest standing between it and the sun. Zero leaves the sun on both sides of every furrow alike.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 34, as invalid input `bark directional occlusion`
- blend: linear; dial: `material_directional_occlusion` (how far a furrow floor is darkened by its own crest; at zero the sun falls on both sides of every furrow alike, and any rise starts the shading), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/depthStrength`

How far the relief is given depth beyond the shaded normal. At zero the relief is given no depth beyond that normal, and any rise starts it.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 35, as invalid input `bark depth strength`
- blend: linear; dial: `material_depth_strength` (how far the bark's relief is given depth beyond the shaded normal; at zero the relief is given no depth beyond the shaded normal, and any rise starts it), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/canopyNormal`

How far a leaf is lit as part of its crown rather than as a lone card: its lighting normal bends from the blade's toward the crown's outward direction at its placement, so the sunward shell of the mass is lit whichever way its blades turn. Zero lights the blade alone.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 36, as invalid input `leaf canopy normal`
- blend: linear; dial: `material_canopy_normal` (how far a leaf is lit as part of its crown rather than as a lone card), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/lightWrap`

How far the leaf's sunlight wraps past the terminator, as a fraction of a right angle's cosine; a face square to the sun takes what it always took. Zero is the plain cosine.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 37, as invalid input `leaf light wrap`
- blend: linear; dial: `material_light_wrap` (how far a leaf's sunlight wraps past the terminator), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/diffuseTransmission`

The share of the blade's transmission that leaves it diffusely, as a thin leaf's does, rather than on the forward lobe toward the sun; the diffuse share also carries the sky through the blade. Zero is the lobe.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 38, as invalid input `leaf diffuse transmission`
- blend: linear; dial: `material_diffuse_transmission` (the share of what comes through a blade that leaves it diffusely), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/leafSheen`

The cuticle's reflectance of the sky at normal incidence, rising to the whole sky at grazing by Schlick's Fresnel; zero reflects no sky.

- real, -, [0, 0.5], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 39, as invalid input `leaf sheen`
- blend: linear; dial: `material_leaf_sheen` (what the leaf's cuticle reflects of the sky head-on), window its bounds (validated bound), steps 0.1 and 0.2

### `/material/crownShade`

How much of the sky one crown radius of leaves takes from a leaf that reads it through the mass - the sky over it, behind it and in its sheen - so the underside of a crown falls into its own shade. Zero sees the sky through the mass.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 40, as invalid input `leaf crown shade`
- blend: linear; dial: `material_crown_shade` (how much of the sky one crown radius of leaves takes from a leaf), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/lichenScale`

Smooth bark's lichen: the size in metres of the cells its patches are scattered over. Zero leaves no patch anywhere.

- real, m, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 41, as invalid input `bark lichen scale`
- dormant: `lichenStrength` zero
- blend: linear; dial: `material_lichen_scale` (the metres across the cells lichen patches are scattered over), window its bounds (validated bound), steps 0.01 and 0.02, presets span [0, 0.06]

### `/material/lichenCoverage`

The share of those cells that hold a patch.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 42, as invalid input `bark lichen coverage`
- dormant: `lichenStrength` or `lichenScale` zero
- blend: linear; dial: `material_lichen_coverage` (the share of those cells that hold a patch), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/lichenRed`

A patch's own colour, a linear reflectance like the bark's.

- real, reflectance, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 43, as invalid input `bark lichen red`
- dormant: `lichenStrength` or `lichenScale` zero
- blend: linear; dial: `material_lichen_red` (the red in a lichen patch's own colour), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/lichenGreen`

The green channel of `lichen_red`.

- real, reflectance, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 44, as invalid input `bark lichen green`
- dormant: `lichenStrength` or `lichenScale` zero
- blend: linear; dial: `material_lichen_green` (the green in a lichen patch's own colour), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/lichenBlue`

The blue channel of `lichen_red`.

- real, reflectance, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 45, as invalid input `bark lichen blue`
- dormant: `lichenStrength` or `lichenScale` zero
- blend: linear; dial: `material_lichen_blue` (the blue in a lichen patch's own colour), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/lichenStrength`

How far a patch covers the bark with that colour.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 46, as invalid input `bark lichen strength`
- dormant: `lichenScale` zero
- note: Above zero, with `lichenScale`, it compiles the smooth-bark wood pipeline (`telperion-render` `wood.rs`).
- blend: linear; dial: `material_lichen_strength` (how far a patch covers the bark with its colour), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/lenticelDensity`

Rows of lenticel dashes per metre along the wood.

- real, -, [0, 400], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 47, as invalid input `bark lenticel density`
- dormant: `lenticelStrength` or `lenticelLength` zero
- blend: linear; dial: `material_lenticel_density` (rows of lenticel dashes per metre along the wood), window its bounds (validated bound), steps 5 and 10, presets span [0, 27]

### `/material/lenticelLength`

The longest dash across the wood, in metres; the shortest is under half.

- real, m, [0, 0.5], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 48, as invalid input `bark lenticel length`
- dormant: `lenticelStrength` zero
- blend: linear; dial: `material_lenticel_length` (the longest lenticel dash across the wood, in metres), window its bounds (validated bound), steps 0.1 and 0.2

### `/material/lenticelStrength`

How far a dash shows: its tint, and the shallow groove it cuts.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 49, as invalid input `bark lenticel strength`
- dormant: `lenticelLength` zero
- note: Above zero, with `lenticelLength`, it compiles the smooth-bark wood pipeline (`telperion-render` `wood.rs`).
- blend: linear; dial: `material_lenticel_strength` (how far a lenticel dash shows), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/lenticelTint`

A dash's value against the bark it marks: -1 is black, 0 no change.

- real, -, [-1, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 50, as invalid input `bark lenticel tint`
- dormant: `lenticelStrength` or `lenticelLength` zero
- blend: linear; dial: `material_lenticel_tint` (a lenticel dash's value against the bark it marks), window its bounds (validated bound), steps 0.25 and 0.5

### `/material/peelCurl`

How far the plate network's strips curl away: they stretch across the wood into bands, lift at their lower edge, and this share of them has peeled off to show the inner bark. At zero no strip has peeled at all, and any rise switches the peel on.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 51, as invalid input `bark peel curl`
- dormant: `ridgeScale` zero
- note: Above zero it compiles the smooth-bark wood pipeline (`telperion-render` `wood.rs`).
- blend: linear; dial: `material_peel_curl` (the share of the bark's strips that have peeled away; at zero no strip has peeled at all, and any rise switches the peel on), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/peelRed`

The inner bark a peeled strip leaves showing.

- real, reflectance, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 52, as invalid input `bark peel red`
- dormant: `peelCurl` or `ridgeScale` zero
- blend: linear; dial: `material_peel_red` (the red in the inner bark a peeled strip shows), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/peelGreen`

The green channel of `peel_red`.

- real, reflectance, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 53, as invalid input `bark peel green`
- dormant: `peelCurl` or `ridgeScale` zero
- blend: linear; dial: `material_peel_green` (the green in the inner bark a peeled strip shows), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/peelBlue`

The blue channel of `peel_red`.

- real, reflectance, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 54, as invalid input `bark peel blue`
- dormant: `peelCurl` or `ridgeScale` zero
- blend: linear; dial: `material_peel_blue` (the blue in the inner bark a peeled strip shows), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/lobeShade`

How much of the sky and of what passes through the blade a leaf loses to the leaves of its own lobe standing over it, read from the crown's own placements rather than from one smooth ellipsoid: a lobe's face is lit and what hangs under it falls into its shade. Zero sees none of it.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 55, as invalid input `leaf lobe shade`
- blend: linear; dial: `material_lobe_shade` (how much a leaf loses to the leaves of its own lobe standing over it), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/barkReflectance`

What the bark mirrors of the sun at normal incidence, the foot of its one highlight by Schlick's Fresnel: 0.04 is a dielectric. What the highlight mirrors is taken from the diffuse; zero mirrors none.

- real, -, [0, 1], default 0.04
- read by draw; growth path: as the direct build
- checked: Material site, rank 56, as invalid input `bark reflectance`
- blend: linear; dial: `material_bark_reflectance` (what the bark mirrors of the sun head-on), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/leafReflectance`

The same for the cuticle on the blade's front face; the highlight's width follows `cuticle_gloss`.

- real, -, [0, 1], default 0.04
- read by draw; growth path: as the direct build
- checked: Material site, rank 57, as invalid input `leaf reflectance`
- blend: linear; dial: `material_leaf_reflectance` (what a leaf's cuticle mirrors of the sun head-on), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/barkGrainScale`

A grain below the relief: the size in metres of its cells. Zero leaves the bark smooth between the relief's features.

- real, m, [0, 0.05], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 58, as invalid input `bark grain scale`
- dormant: `barkGrainStrength` zero
- blend: linear; dial: `material_bark_grain_scale` (the metres across the cells of the grain below the bark's relief), window its bounds (validated bound), steps 0.0005 and 0.001, presets span [0, 0.003]

### `/material/barkGrainStrength`

How far the grain varies the bark's colour and tilts its normal.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 59, as invalid input `bark grain strength`
- dormant: `barkGrainScale` zero
- blend: linear; dial: `material_bark_grain_strength` (how far that grain varies the bark's colour and tilts its normal), window its bounds (validated bound), steps 0.15 and 0.3

### `/material/bladeGrainScale`

The blade's cell grain, in cells per blade length; zero is none.

- real, -, [0, 256], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 60, as invalid input `leaf blade grain scale`
- dormant: `bladeGrainStrength` zero
- blend: linear; dial: `material_blade_grain_scale` (the blade's cell grain, in cells per blade length), window its bounds (validated bound), steps 20 and 40, presets span [0, 135]

### `/material/bladeGrainStrength`

How far that grain varies the blade's colour and tilts its normal.

- real, -, [0, 1], default 0
- read by draw; growth path: as the direct build
- checked: Material site, rank 61, as invalid input `leaf blade grain strength`
- dormant: `bladeGrainScale` zero
- blend: linear; dial: `material_blade_grain_strength` (how far that grain varies the blade's colour and tilts its normal), window its bounds (validated bound), steps 0.15 and 0.3
