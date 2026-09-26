// Generated from the parameter catalogue (crates/telperion-core/src/catalogue.rs)
// by `the_browser_reads_the_catalogue` in crates/telperion-core/tests/parameter_reference.rs;
// run it with TELPERION_WRITE_REFERENCE=1 to rewrite this file. Do not edit.

/** A family: every wire row, under the meaning the catalogue gives it. */
export interface Family {
  /** The specimen's age in years. Only the growth path reads it: the direct build is the mature tree whatever the row says. */
  age: number;
  /** How deep into the crown leaves are kept, as a share of the crown's widest radius; a leaf further in than that is dropped. Raising it keeps more of the crown's interior foliage, and one keeps it all. */
  shellDepth: number;
  growth: {
    /** Geometric work quanta available over this specimen's life. */
    workBudget: number;
    /** Chapman–Richards rate, in inverse years. */
    rate: number;
    /** Chapman–Richards shape; values above one give a sigmoidal height curve. */
    shape: number;
    /** Consecutive active slices below the habit shedding threshold, in years. */
    sheddingTolerance: number;
    /** Annual loss of the habit apical control (zero retains its authored value). */
    apicalControlLoss: number;
    /** Years of annual foliage cohorts held by a living shoot; zero bears none. */
    leafLifetime: number;
    /** Minimum thickening in metres before recording another annual radius frame. */
    resizeTolerance: number;
  };
  skeleton: {
    /** The specimen: every stage keys its random stream by it, so another seed draws another tree of the same family. */
    seed: number;
    /** How many pull points are scattered through the crown for the branches to grow toward. Raising it fills the crown with more and finer branching; at an `attractor_weight` of zero none are scattered and the row does nothing. */
    attractors: number;
    /** How many random tries the sampler may spend on each pull point before it gives up. Raising it lets a narrow or deeply lobed crown reach its full count of points instead of settling for fewer. */
    samplingAttemptsPerAttractor: number;
    /** How far the crown grows in one step, as a share of the tree's height; the distance at which a pull point is used up is twice it. Raising it grows the crown in longer, coarser strides. */
    step: number;
    habit: {
      /** Samples available to find an axis's room within the crown. */
      reachProbeSteps: number;
      /** How far the leader persists into the crown, 0 to 1. */
      apicalDominance: number;
      /** Clustering of laterals at a station against scattering along the axis. */
      whorlStrength: number;
      /** Spacing between lateral stations on the leader, in metres. */
      leaderInternode: number;
      /** Laterals borne by one station of the leader. */
      lateralsPerStation: number;
      /** Initial angle of a lateral from its parent axis, in degrees; on the upright leader this is degrees from vertical. */
      lateralPitch: number;
      /** Spread of the lateral pitch, in degrees. */
      pitchVariation: number;
      /** Signed bend over the length of a first-order axis; positive rises. */
      risePrimary: number;
      /** Signed bend over the length of a deeper axis; negative hangs. */
      riseSecondary: number;
      /** Heading change between successive growth units, in degrees. */
      crookedness: number;
      /** Spacing between lateral stations away from the leader, in metres. */
      lateralSpacing: number;
      /** Length of a lateral against its supporting axis. */
      lateralLengthRatio: number;
      /** Depth of rule-built orders below the leader. */
      lateralOrders: number;
      /** Weight of the attractor pull beside the axis's own rule heading. At zero no pull points are scattered and every axis holds its own rule heading, and any rise switches the pull on. */
      attractorWeight: number;
      /** Distal twig radius against the nominal twig radius. */
      twigTipTaper: number;
      /** Monthly vigour threshold; zero disables shedding. The legacy envelope builder interprets it as shell depth. */
      sheddingThreshold: number;
      /** Stems leaving the root. One is the single trunk every tree was, to the byte; a birch, a hazel or a coppiced oak stands on more. */
      stems: number;
      /** Degrees of bearing between neighbouring stems, about a bearing the seed alone decides. Inert at one stem, which has no neighbour. At zero every stem leaves the root on one bearing, and any rise starts to fan them apart. */
      stemDivergence: number;
      /** Degrees from vertical the outermost stems tilt away from the root; the ones between tilt in proportion to how far out they stand. Inert at one stem, which stands at the centre and so tilts by none of it. At zero every stem stands upright, and any rise starts the tilt. */
      stemLean: number;
      /** How unequally a clump's stems lean, 0 to 1. None of it is the lean above, shared about the clump's centre; all of it leans the stems in their order instead, the first upright and the last by all of `stem_lean`. Inert at one stem, which has nothing to lean against. */
      stemLeanSpread: number;
      /** Where a clump's later stems leave the first, as a share of the bole's height, 0 to 0.5. None of it parts them at the ground; half of it parts them halfway up the bole, with one trunk below. Inert at one stem. */
      stemForkHeight: number;
    };
    envelope: {
      /** The tree's height in metres. Everything else in the crown is measured against it: the crown base sits at `crown_base` of it and the widest radius is `spread` times it. */
      height: number;
      /** Where the crown starts, as a share of the height: below it the crown has no radius at all. Raising it lifts the crown and leaves a longer bare trunk. */
      crownBase: number;
      /** The crown's widest radius as a share of the height, so raising it widens the crown without making the tree taller. */
      spread: number;
      /** Where the crown is widest, as a share of the way from the crown base to the top. Raising it carries the widest part higher, so the crown reads top-heavy. */
      fullness: number;
      /** How square the crown's outline is. Raising it holds the crown near its full width further toward the top and the base, so the profile reads boxier; lowering it tapers the outline to a point. */
      shoulder: number;
      /** How far the outline departs from the smooth shell, as a fraction of the radius there. 0 is the axisymmetric superellipse every tree was before, and every shipped table that leaves it there is untouched. */
      irregularity: number;
      /** The wavelength of that departure over the shell's own surface, as a fraction of the tree's height: small is many small lumps, 1 is a lobe as long as the tree is tall. */
      lobeScale: number;
    };
    bias: {
      /** How strongly growth is pulled upward, most near the ground. Raising it makes the tree grow more erect. At zero nothing pulls growth upright, and any rise starts that pull. */
      gravitropism: number;
      /** How far the whole tree leans off vertical, increasing with height. Raising it tips the trunk further in one direction. At zero the tree stands plumb, and any rise starts the lean. */
      lean: number;
      supernatural: {
        /** Whether the supernatural field bends the wood. Off, the bias reads the amplitude, wavelength and spiral as zero. */
        enabled: boolean;
        /** How far a branch may wander from a straight course, as a share of the tree's height. Raising it makes the wood wind and stray more. At zero the wood holds a straight course and the spiral rate does nothing, and any rise starts the wander. */
        writheAmplitude: number;
        /** How long each of those wanders runs, as a share of the height. Raising it gives fewer, lazier bends; lowering it gives tighter kinks. */
        writheWavelength: number;
        /** How many full turns the wander winds around the trunk over the tree's height. Raising it tightens the spiral. At zero the wander winds around nothing, and any rise starts the spiral. */
        spiralRate: number;
        /** The ceiling on how hard the wander may pull in any one step, so the other writhe rows cannot bend the wood arbitrarily. */
        maxWritheMagnitude: number;
      };
    };
    twigs: {
      /** A shoot's length as a share of the one that bore it. Raising it makes each generation of twigs longer relative to its parent. */
      lengthRatio: number;
      /** How much thinner a shoot is than its parent for the same drop in length. Raising it leaves side shoots finer. */
      ratioPower: number;
      /** The fewest of its own diameters a segment of wood may span. Raising it makes segments longer for the same thickness, so there are fewer joints and the wood reads straighter. */
      internodeFactor: number;
      /** The ceiling on segments one length of wood may be cut into. It binds only where the two lengths above would cut more, and there it caps the cost rather than states a look. */
      maxInternodes: number;
      /** How many side shoots leave each station along a twig. Raising it crowds more twigs onto the same length of wood. */
      laterals: number;
      /** Twig-law generations of branching, 1 to 6. A lateral born at or past this generation is a twig whatever the pipe model left its radius, so the twig layer's depth is a row a table states rather than a consequence of how thick the wood is. */
      generations: number;
      /** The share of the trunk's radius at or below which wood starts bearing twigs. Raising it lets twigs start on thicker wood, so they reach further back toward the trunk. */
      limbRadius: number;
      /** How deep the outer skin of the crown is that only twigs may fill, as a share of the crown: the scaffold is grown into what is left inside it. Raising it holds the structural wood further in and leaves a deeper twig layer. */
      reach: number;
      /** The degrees a side shoot leaves its parent. Raising it swings twigs further out toward square with the branch. */
      angle: number;
      /** How many degrees that departure angle varies shoot to shoot. Raising it makes the twig layer less uniform. */
      angleVariation: number;
      /** How much shoot length varies shoot to shoot. Raising it gives a more uneven, less combed twig layer. */
      vigourVariation: number;
      /** The degrees each successive shoot is turned around the wood that bears it. Raising it swings the next shoot further around, so the twigs spiral differently. */
      divergence: number;
      /** How strongly a shoot hangs, 0 to 3. At 0 nothing hangs and the local law is the ordinary one; at 1 a curtain takes its full droop. */
      hang: number;
      /** Metres a pendulous shoot grows before it stops, and the length its droop reaches the cap over. */
      pendulousLength: number;
      /** Fraction of the root radius at or below which a station's shoots hang. */
      pendulousRadius: number;
      /** Degrees between neighbouring shoots in a curtain. */
      curtainSeparation: number;
      /** How far toward straight down a hanging shoot's course has turned by the end of its pendulous length, 0 to 1: at 0 the shoot holds the direction it departed with and the curtain is a set of rods, at 1 it hangs vertical, and the turn is spread along the run as an arc steepest at the wood that bears it. */
      sag: number;
      /** How much shorter than the pendulous length a hanging shoot may run, 0 to 1: each shoot's own length is the pendulous length times one minus this times a draw in 0 to 1 keyed by the shoot and the seed. At 0 every shoot has the one length the table states. */
      pendulousVariation: number;
      /** How far below the shell's lower surface a hanging shoot may fall, 0 to 1, as a share of the way from that surface down to the clearance: at 0 the shell binds a hanging shoot as it binds every other, at 1 the shoot may fall to the clearance. Only a shoot that hangs, only under the crown's footprint. */
      curtainDrop: number;
      /** Metres above the ground no hanging shoot falls below, 0 to 5. Never above the crown's own base, whatever the row says. */
      curtainClearance: number;
      /** The furthest a hanging shoot may bend toward straight down. Raising it lets curtains hang more heavily. */
      maxDroop: number;
      /** How close to the ground a hanging curtain may reach before it stops growing. Raising it lets curtains hang nearer the floor. */
      curtainStepClearance: number;
      twig: {
        /** The finished thickness of a twig in metres. Wood at or below half of it is drawn as a twig, so raising it thickens the twig layer and hands more of the fine wood to it. */
        diameter: number;
        /** The length in metres a twig shoot grows before it stops, and the whole of one internode on leaf-bearing wood. Raising it lengthens every twig, so the crown carries a deeper, shaggier skin. */
        length: number;
        /** Metres between the joints on wood thicker than the bearing diameter, and the spacing of the stations a leaf sits on. Raising it gives longer segments, so laterals and leaves sit further apart. */
        internodeLength: number;
        /** How many leaf stations sit at each joint, each turned its own share of a full turn around the shoot. Raising it crowds more leaves onto the same joints. */
        stationsPerInternode: number;
        /** The thickness in metres at or below which a shoot bears leaves and side shoots of its own. Raising it lets thicker wood bear, so foliage reaches further back down the branch. */
        bearingDiameter: number;
      };
    };
    growth: {
      /** Metres a pull point may reach to steer the wood nearest it; unset, the crown's own volume and the point count decide it. Raising it lets distant points draw a branch across the crown. */
      influenceRadius?: number;
      /** Metres within which a pull point counts as reached and stops pulling; unset, twice the step distance. Raising it uses the points up sooner, so branches stop shorter and the crown fills coarsely. */
      killDistance?: number;
      /** Metres of wood laid down in one growth step; unset, the tree's height times `step`. Raising it lays down longer, coarser segments. */
      stepDistance?: number;
      /** Metres of bare trunk before the crown may start; unset, the envelope's own crown base. Raising it lifts the whole crown and leaves a longer clear bole. */
      trunkHeight?: number;
      /** The ceiling on nodes the crown may grow; unset, the shipped default. Growth stops at it, so raising it changes only a crown that reached it. */
      maxNodes?: number;
      /** The most a growing shoot may turn in one step, in degrees; unset, 35. At 180 or more nothing is limited. */
      maxTurnPerStep?: number;
    };
  };
  radii: {
    /** The trunk's radius at the ground as a share of the tree's height, so raising it thickens every piece of wood in proportion. */
    trunkRadius: number;
    /** How wood divides at a fork. The parent's area is the sum of the children's radii raised to this power, so raising it leaves the children thicker for the same parent. */
    forkExponent: number;
    /** How fast wood thins along its own length. Raising it makes a branch narrow more sharply from its base to its tip. */
    lengthTaper: number;
    /** The ceiling on accumulated taper, so no single long branch can thin away to nothing. Raising it lets long branches taper further. */
    maxTaperExponent: number;
  };
  surface: {
    /** How many sides each piece of wood is drawn with. Raising it makes the wood rounder and smoother, and costs triangles. */
    radialSegments: number;
    /** How many ridges run up around the trunk. Raising it gives the bark more flutes; zero is a plain round bole. */
    lobes: number;
    /** How deep the flutes between those ridges cut, as a share of the wood's own radius. Raising it makes the fluting more pronounced. At zero the bole is plainly round whatever the ridge count says, and any rise starts cutting the flutes. */
    lobeDepth: number;
    /** How many turns those ridges make over the tree's height. Raising it winds them more tightly around the trunk. */
    twistRate: number;
    /** How much wider the trunk is where it meets the ground, as a multiple of its own radius. Raising it gives a broader buttress. */
    flareRadius: number;
    /** How far up the trunk that flare reaches, as a share of the height. Raising it carries the swelling further up the bole. */
    flareFalloff: number;
    /** How deep the trunk's base is sunk below the ground, as a share of the height. Raising it buries more of the flare. */
    flareDepth: number;
    /** How deeply a child branch is set into its parent at a fork. Raising it sinks the junction further in, so the two read as one piece of wood rather than two tubes meeting. */
    forkSocket: number;
    /** Fraction of the parent's inscribed radius available for a socket. */
    socketContainment: number;
    /** How much wood thickens at a fork. Raising it leaves a more pronounced collar where a branch leaves its parent. */
    forkSwell: number;
  };
  canopy: {
    /** Wood at or below this fraction of the root radius bears foliage of its own, beside whatever the twig layer marks. Zero leaves the twigs alone with it; without a twig layer it is what selects the terminal shoots. */
    shootRadius: number;
    /** Metres between leaves along a shoot, as a share of the tree's height. Raising it spreads the leaves further apart, so the crown carries fewer of them. @deprecated */
    spacing: number;
    /** The degrees each successive leaf is turned around its shoot. Raising it turns the next leaf further round, so the leaves spiral differently. */
    divergence: number;
    /** How many extra leaves are gathered at the end of a shoot that has no twig layer. Raising it packs a denser tuft at the tip. @deprecated */
    clump: number;
    /** How far back from the tip that tuft is scattered, as a share of the shoot's length. Raising it spreads the tuft further down the shoot. @deprecated */
    clumpSpan: number;
    /** How far a leaf turns away from the trunk. Raising it points the leaves outward, away from the tree's axis. */
    outward: number;
    /** How far a leaf turns toward the sky. Raising it tips the leaves up. */
    upward: number;
    /** Lean along the shoot, as a fraction of the radial off the wood. */
    forwardLean: number;
    /** Further lean along the shoot on radials that face upward. */
    leanRise: number;
    /** The station sits on the shoot axis at 0 and on the wood's own contact surface at 1; the surface is built whenever it is positive. */
    surfaceContact: number;
    /** The degrees a leaf may be turned at random from where it was placed. Raising it leaves the crown less combed. */
    scatter: number;
    /** The size every leaf is drawn at, as a multiple of the element's own dimensions. Raising it enlarges every leaf. */
    size: number;
    /** How far leaf size varies leaf to leaf, as a share of that size. Raising it mixes larger and smaller leaves more widely. */
    sizeVariation: number;
    /** Metres between short shoots along limb and branch wood: spurs a few centimetres long, each ending in a cluster of leaves. Zero grows none. */
    shortShootSpacing: number;
    /** Wood thicker than this fraction of the stem's radius carries no short shoot, and neither does twig wood or anything below the crown base. */
    shortShootRadius: number;
    /** Metres from the bark to the cluster a short shoot carries. */
    shortShootLength: number;
    /** Leaves in one short shoot's cluster, 1 to 8. */
    shortShootLeaves: number;
    /** Degrees either side of its short shoot's bearing a cluster's leaves fan across, held level: 90 is a half circle, 0 stacks them. */
    shortShootSpread: number;
    /** How far into each limb system the gap between it and its neighbours reaches, as a share of the way from their shared boundary to the system's centre: each limb system then keeps a rounded leaf mass of its own. Zero, the neutral, thins nothing. */
    limbClumping: number;
    /** How deep a lateral may be and still start a limb system of its own. Raising it parts the crown into more and smaller leaf masses; it does nothing until `limb_clumping` is above zero. */
    clumpSystemOrder: number;
    /** Nearest neighbours and cell crossings in the clumping approximation. */
    clumpNeighbours: number;
    /** Fronds the rosette bears at the apex of each stem. At zero no rosette stands and the canopy clothes wood as it always did; any rise makes the rosette the tree's only foliage. */
    rosetteFronds: number;
    /** The degrees each successive frond is turned about the apex. */
    rosetteDivergence: number;
    /** Degrees from the axis the youngest frond stands: 0 upright, 90 level, 180 hanging. */
    rosettePitch: number;
    /** How many degrees further than the youngest the oldest frond leans, so the crown opens from a spike to a skirt. */
    rosettePitchSpread: number;
    /** Metres below the apex the frond insertions are spread down the axis. At zero every frond leaves one point. */
    rosetteDepth: number;
    /** Leaflets one placement carries along its rachis. One is the single blade every family drew. */
    leafletCount: number;
    /** Metres of rachis the leaflets are strung along. At zero the placement is one blade whatever the count says. */
    rachisLength: number;
    /** The degrees a leaflet leaves its rachis. */
    leafletPitch: number;
    /** How far the rachis bends out of the straight line from its station, as a share of its length. Positive arches up, negative droops. */
    rachisArch: number;
    /** Whether a single leaflet closes the rachis's end, blended 0 to 1: the last leaflet turns from standing off the rachis to lying along it. */
    terminalLeaflet: number;
    /** Bases of shed fronds the stem keeps below its crown, clothing the trunk. At zero the trunk is bare and the bark is what it always was; any rise carries the crown's own spiral down it. */
    leafBases: number;
    /** Metres a retained base stands out from the bark. At zero no base is drawn whatever the count says. */
    leafBaseLength: number;
    /** How thick a base is where it leaves the bark, as a share of the stem's own radius there. Raising it leaves a broader boot. */
    leafBaseRadius: number;
    /** Degrees from the stem's axis a base points: 0 flat against the trunk, 90 square out of it, 180 turned back down. */
    leafBasePitch: number;
    /** How far the lowest and oldest base is worn back against the newest, in both its length and its girth. At zero every base stands full down the whole trunk, and any rise wears the foot away. */
    leafBaseWeathering: number;
    /** How broad a retained base is across the trunk, as a share of the cell the crown's spiral gives it on the bark: at 1 every base meets its neighbours edge to edge whatever the count and the trunk's girth, below it the bark shows between them and above it they crowd into each other. At zero the base is the round peg the radius row sizes and the lattice rows say nothing; any rise packs the bases into the lattice. */
    leafBaseWidth: number;
    /** How flat-sided a lattice base is drawn: 0 the ellipse through its cell's corners, 1 the cell itself, a diamond with flat faces that meets each neighbour along a straight edge and is cut square at its outer end. At zero the section stays round, and at no width it reads nothing. */
    leafBaseFlatness: number;
    /** Leaflets at a frond's base borne as spines rather than blades. At zero the frond carries blades all the way down; any rise hardens that many of them. */
    acanthophylls: number;
    /** The share of a leaflet's own size a spine is drawn at. At zero no spine is drawn whatever the count says. */
    acanthophyllLength: number;
    /** The degrees a spine leaves the rachis, in place of the leaflet's own pitch. */
    acanthophyllPitch: number;
    /** Dead fronds a rosette keeps below its living crown, continuing the crown's own spiral down the stem. At zero no frond is kept and the crown ends at its oldest living frond; any rise hangs that many. */
    skirtFronds: number;
    /** Degrees from the axis a dead frond hangs: 0 upright, 90 level, 180 collapsed straight down against the stem. */
    skirtPitch: number;
    /** A dead frond's length as a share of a living one's, rachis and leaflets alike. At zero no dead frond is drawn whatever the count says. */
    skirtLength: number;
    /** Hard total budget. Exceeding it returns an error, never partial foliage. */
    maxInstances: number;
  };
  element: {
    /** Metres of petiole or woody peg below the blade, excluded from unit dimensions. Every element carries one. */
    connectorLength: number;
    /** Blade/needle longitudinal extent, excluding connector, in metres. */
    length: number;
    /** The blade's greatest width in metres, across the midrib. Raising it makes every leaf broader. */
    width: number;
    /** Where the blade is widest, as a share of the way from its base to its tip. Raising it carries the widest point toward the tip. */
    widestAt: number;
    /** How fast the blade fills out above its stalk. Raising it draws the base in, so the leaf reads wedge-shaped rather than rounded. */
    baseFullness: number;
    /** How fast the blade narrows toward its point. Raising it draws the tip out into a sharper point. */
    tipSharpness: number;
    /** How far the blade's margins lift out of its own plane, as a share of the half-width there. Raising it dishes the leaf more deeply along the midrib. */
    cup: number;
    /** How far the blade bends along its length, as a share of its length at the tip. Raising it curls the tip further out of the plane its base stands in. */
    curl: number;
    /** Lobes along each margin; 0 is an entire margin. */
    lobeCount: number;
    /** How far each sinus cuts toward the midrib, 0 to 1. At zero the margin is entire whatever the lobe count says, and any rise starts cutting the sinuses. */
    lobeDepth: number;
    /** Flat blade at 0, four-sided shaft at 1. */
    sectionRoundness: number;
    /** How many sections the blade is built from along its length. Raising it draws the outline and any lobes more smoothly, at more triangles per leaf. */
    axialSegments: number;
    /** How many columns the blade is built from across its width. Raising it draws the section and the margins more smoothly, at more triangles per leaf. */
    crossSegments: number;
    /** A flat two-triangle card in place of the modelled blade: no outline, no cup or curl, no sections and no connector. */
    card: boolean;
  };
  material: {
    /** The bark's own colour, a linear reflectance per channel. Raising a channel pushes mature bark toward that colour. */
    barkRed: number;
    /** The green channel of `bark_red`. */
    barkGreen: number;
    /** The blue channel of `bark_red`. */
    barkBlue: number;
    /** How diffuse the bark is: 0 is a mirror, 1 is chalk. */
    barkRoughness: number;
    /** The young wood's own colour, before its bark has formed. Wood thinner than `shoot_radius` takes it, and gives it up to the bark colour on a smoothstep of its radius by twice that; zero means no wood is young. */
    shootRed: number;
    /** The green channel of `shoot_red`. */
    shootGreen: number;
    /** The blue channel of `shoot_red`. */
    shootBlue: number;
    /** Metres. The radius below which wood is young. */
    shootRadius: number;
    /** The colour of a leaf's upper face, a linear reflectance per channel. Raising a channel pushes the sunlit face toward it. */
    leafFrontRed: number;
    /** The green channel of `leaf_front_red`. */
    leafFrontGreen: number;
    /** The blue channel of `leaf_front_red`. */
    leafFrontBlue: number;
    /** The colour of a leaf's underside, shown wherever the eye sees the back of a blade. Raising a channel pushes that face toward it. */
    leafBackRed: number;
    /** The green channel of `leaf_back_red`. */
    leafBackGreen: number;
    /** The blue channel of `leaf_back_red`. */
    leafBackBlue: number;
    /** The colour a dead frond has aged to, both faces alike, shown only on the fronds a rosette keeps below its living crown. Raising a channel pushes the skirt toward it. */
    leafDeadRed: number;
    /** The green channel of `leaf_dead_red`. */
    leafDeadGreen: number;
    /** The blue channel of `leaf_dead_red`. */
    leafDeadBlue: number;
    /** The hue offsets one leaf may take, as a fraction of the colour circle. */
    hueRangeLow: number;
    /** The upper end of the hue offsets `hue_range_low` opens. */
    hueRangeHigh: number;
    /** The brightness offsets one leaf may take, about no change at all. */
    brightnessRangeLow: number;
    /** The upper end of the brightness offsets `brightness_range_low` opens. */
    brightnessRangeHigh: number;
    /** How far a leaf deep inside the crown is darkened towards a shaded mass. */
    interiorDarkening: number;
    /** Circumferential ridge spacing in metres; zero disables relief. */
    ridgeScale: number;
    /** Axial scale control in metres; spacing is bounded to 1.5–2 ridge widths. Larger ratios lengthen and deepen furrows; zero omits breaks. */
    plateScale: number;
    /** Furrow width and depth together; zero leaves tightly packed scales. */
    furrowStrength: number;
    /** Roughness variation about the base value, clamped to 0..1. */
    roughnessDetail: number;
    /** Secondary vein pairs per blade, continuously interpolated. */
    veinScale: number;
    /** How far the veins are lightened and the blade between them darkened. Raising it makes the venation read more sharply. */
    veinContrast: number;
    /** How brightly a backlit leaf glows with the light that came through it. Raising it lifts that glow. */
    transmissionStrength: number;
    /** Linear transmission tint, in 0..1 per channel. */
    transmissionRed: number;
    /** The green channel of `transmission_red`. */
    transmissionGreen: number;
    /** The blue channel of `transmission_red`. */
    transmissionBlue: number;
    /** Optical thickness: attenuation is exp(-thickness). */
    thickness: number;
    /** The tint carried by the floors of the bark's furrows, as an offset per channel, and how far it is laid over the bark colour there. */
    fissureRed: number;
    /** The green channel of `fissure_red`. */
    fissureGreen: number;
    /** The blue channel of `fissure_red`. */
    fissureBlue: number;
    /** How far `fissure_red`'s tint is laid over the bark colour in the furrows. */
    fissureStrength: number;
    /** The tint carried by the crests of the bark's ridges, as an offset per channel, and how far it is laid over the bark colour there. */
    crestRed: number;
    /** The green channel of `crest_red`. */
    crestGreen: number;
    /** The blue channel of `crest_red`. */
    crestBlue: number;
    /** How far `crest_red`'s tint is laid over the bark colour on the crests. */
    crestStrength: number;
    /** The size of the blotches in the bark's mottling, and how far they lighten and darken its colour. A scale of zero leaves none. */
    barkMottleScale: number;
    /** How far the blotches `bark_mottle_scale` sizes lighten and darken the bark. */
    barkMottleStrength: number;
    /** How far the hollows of the bark - furrow floors and the sockets where a limb joins - are darkened. Raising it sinks them deeper. */
    cavityStrength: number;
    /** The size of the blotches in a leaf's mottling, and how far they vary its colour. A scale of zero leaves none. */
    bladeMottleScale: number;
    /** How far the blotches `blade_mottle_scale` sizes vary a leaf's colour. */
    bladeMottleStrength: number;
    /** How wide a band along the leaf's edge takes the margin colour, and what that colour is as an offset per channel. Raising the width broadens the rim around every leaf. */
    marginWidth: number;
    /** The margin colour's red channel, as an offset; `margin_width` sets its band. */
    marginRed: number;
    /** The margin colour's green channel, as an offset; `margin_width` sets its band. */
    marginGreen: number;
    /** The margin colour's blue channel, as an offset; `margin_width` sets its band. */
    marginBlue: number;
    /** How tight the highlight on a leaf's upper face is. Raising it draws the glint into a smaller, glossier spot. */
    cuticleGloss: number;
    /** How much of the sky is withheld from bark and leaves the deeper they sit in the crown. Raising it darkens the crown's interior. */
    skyOcclusionStrength: number;
    /** Circumferential size of one bark plate in metres, before girth scales it. Zero leaves the field the ridges it has always been. */
    plateCellScale: number;
    /** How much longer a plate runs than it is wide: nought is as long as it is wide, one is twice as long. */
    plateElongation: number;
    /** How far a plate's face rises from its own edge towards its middle. At zero a plate's face lies flat, and any rise starts the doming. */
    plateDome: number;
    /** How far a plate's rim stands off the furrow it borders: the scale that lifts rather than the plate that sits flat. At zero a plate's rim lies flush with its furrow, and any rise starts the lift. */
    plateEdgeLift: number;
    /** How wide the flat floor of a furrow is cut, as a fraction of a plate's own width, so a bigger plate carries a wider furrow off one row. Zero leaves the hairline the network has always cut between two faces. */
    plateFurrowWidth: number;
    /** Blend from rounded plate edges to narrow chipped scales; independent of lichen/lenticel/peel. */
    plateEdgeShape: number;
    /** How much of its own a plate keeps: how proud it stands, how it leans, and the value and cast it holds against its neighbours. At zero every plate stands and reads exactly like its neighbours, and any rise starts the difference. */
    plateIdentity: number;
    /** How far a weathered face is greyed and tinted against a fresh furrow. At zero no face is weathered at all, and any rise starts the greying. */
    weatheringStrength: number;
    /** The tint a weathered face takes, as an offset per channel; how far it is laid on is `weathering_strength` above. */
    weatheringRed: number;
    /** The green channel of `weathering_red`. */
    weatheringGreen: number;
    /** The blue channel of `weathering_red`. */
    weatheringBlue: number;
    /** How far the side away from the sun and the foot of the trunk take a colour of their own - what damp growth would look like, not what it is. At zero the shaded side and the foot take no colour of their own, and any rise starts it. */
    orientationStrength: number;
    /** The tint that damp side takes, as an offset per channel; how far it is laid on is `orientation_strength` above. */
    orientationRed: number;
    /** The green channel of `orientation_red`. */
    orientationGreen: number;
    /** The blue channel of `orientation_red`. */
    orientationBlue: number;
    /** How far a furrow floor is darkened by its own crest standing between it and the sun. Zero leaves the sun on both sides of every furrow alike. */
    directionalOcclusion: number;
    /** How far the relief is given depth beyond the shaded normal. At zero the relief is given no depth beyond that normal, and any rise starts it. */
    depthStrength: number;
    /** How far a leaf is lit as part of its crown rather than as a lone card: its lighting normal bends from the blade's toward the crown's outward direction at its placement, so the sunward shell of the mass is lit whichever way its blades turn. Zero lights the blade alone. */
    canopyNormal: number;
    /** How far the leaf's sunlight wraps past the terminator, as a fraction of a right angle's cosine; a face square to the sun takes what it always took. Zero is the plain cosine. */
    lightWrap: number;
    /** The share of the blade's transmission that leaves it diffusely, as a thin leaf's does, rather than on the forward lobe toward the sun; the diffuse share also carries the sky through the blade. Zero is the lobe. */
    diffuseTransmission: number;
    /** The cuticle's reflectance of the sky at normal incidence, rising to the whole sky at grazing by Schlick's Fresnel; zero reflects no sky. */
    leafSheen: number;
    /** How much of the sky one crown radius of leaves takes from a leaf that reads it through the mass - the sky over it, behind it and in its sheen - so the underside of a crown falls into its own shade. Zero sees the sky through the mass. */
    crownShade: number;
    /** Smooth bark's lichen: the size in metres of the cells its patches are scattered over. Zero leaves no patch anywhere. */
    lichenScale: number;
    /** The share of those cells that hold a patch. */
    lichenCoverage: number;
    /** A patch's own colour, a linear reflectance like the bark's. */
    lichenRed: number;
    /** The green channel of `lichen_red`. */
    lichenGreen: number;
    /** The blue channel of `lichen_red`. */
    lichenBlue: number;
    /** How far a patch covers the bark with that colour. */
    lichenStrength: number;
    /** Rows of lenticel dashes per metre along the wood. */
    lenticelDensity: number;
    /** The longest dash across the wood, in metres; the shortest is under half. */
    lenticelLength: number;
    /** How far a dash shows: its tint, and the shallow groove it cuts. */
    lenticelStrength: number;
    /** A dash's value against the bark it marks: -1 is black, 0 no change. */
    lenticelTint: number;
    /** How far the plate network's strips curl away: they stretch across the wood into bands, lift at their lower edge, and this share of them has peeled off to show the inner bark. At zero no strip has peeled at all, and any rise switches the peel on. */
    peelCurl: number;
    /** The inner bark a peeled strip leaves showing. */
    peelRed: number;
    /** The green channel of `peel_red`. */
    peelGreen: number;
    /** The blue channel of `peel_red`. */
    peelBlue: number;
    /** How much of the sky and of what passes through the blade a leaf loses to the leaves of its own lobe standing over it, read from the crown's own placements rather than from one smooth ellipsoid: a lobe's face is lit and what hangs under it falls into its shade. Zero sees none of it. */
    lobeShade: number;
    /** What the bark mirrors of the sun at normal incidence, the foot of its one highlight by Schlick's Fresnel: 0.04 is a dielectric. What the highlight mirrors is taken from the diffuse; zero mirrors none. */
    barkReflectance: number;
    /** The same for the cuticle on the blade's front face; the highlight's width follows `cuticle_gloss`. */
    leafReflectance: number;
    /** A grain below the relief: the size in metres of its cells. Zero leaves the bark smooth between the relief's features. */
    barkGrainScale: number;
    /** How far the grain varies the bark's colour and tilts its normal. */
    barkGrainStrength: number;
    /** The blade's cell grain, in cells per blade length; zero is none. */
    bladeGrainScale: number;
    /** How far that grain varies the blade's colour and tilts its normal. */
    bladeGrainStrength: number;
  };
}

/** One catalogue row, as a control reads it. */
export interface Parameter {
  /** The row's JSON pointer on the wire. */
  path: string;
  kind: "real" | "count" | "switch";
  /** An unset value is admitted and means "none stated". */
  optional: boolean;
  meaning: string;
  unit: string;
  /** The admitted values: `lowOpen` refuses `low` itself, `zero` admits zero besides. */
  low: number;
  high: number;
  lowOpen: boolean;
  zero: boolean;
  /** Where the row lies dormant; empty where it always acts. */
  applies: string;
  /** Who reads it: the direct build, only the growth path, or no production stage. */
  reach: "mature" | "growth" | "deprecated";
  /** The tuning dial's window and small step, where the row offers one. */
  dial?: { window: [number, number]; step: number };
}

export const PARAMETERS: readonly Parameter[] = [
  { path: "/age", kind: "real", optional: false, meaning: "The specimen's age in years. Only the growth path reads it: the direct build is the mature tree whatever the row says.", unit: "years", low: 0, high: 1000000, lowOpen: false, zero: false, applies: "", reach: "growth" },
  { path: "/shellDepth", kind: "real", optional: false, meaning: "How deep into the crown leaves are kept, as a share of the crown's widest radius; a leaf further in than that is dropped. Raising it keeps more of the crown's interior foliage, and one keeps it all.", unit: "share of the crown's widest radius", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/growth/workBudget", kind: "count", optional: false, meaning: "Geometric work quanta available over this specimen's life.", unit: "work quanta", low: 1, high: 4294967295, lowOpen: false, zero: false, applies: "", reach: "growth" },
  { path: "/growth/rate", kind: "real", optional: false, meaning: "Chapman–Richards rate, in inverse years.", unit: "per year", low: 0.001, high: 10, lowOpen: false, zero: false, applies: "", reach: "growth" },
  { path: "/growth/shape", kind: "real", optional: false, meaning: "Chapman–Richards shape; values above one give a sigmoidal height curve.", unit: "-", low: 1, high: 8, lowOpen: false, zero: false, applies: "", reach: "growth" },
  { path: "/growth/sheddingTolerance", kind: "real", optional: false, meaning: "Consecutive active slices below the habit shedding threshold, in years.", unit: "years", low: 0, high: 1000000, lowOpen: false, zero: false, applies: "`sheddingThreshold` zero", reach: "growth" },
  { path: "/growth/apicalControlLoss", kind: "real", optional: false, meaning: "Annual loss of the habit apical control (zero retains its authored value).", unit: "per year", low: 0, high: 10, lowOpen: false, zero: false, applies: "", reach: "growth" },
  { path: "/growth/leafLifetime", kind: "real", optional: false, meaning: "Years of annual foliage cohorts held by a living shoot; zero bears none.", unit: "years", low: 0, high: 1000000, lowOpen: false, zero: false, applies: "zero bears no leaves on the growth path", reach: "growth" },
  { path: "/growth/resizeTolerance", kind: "real", optional: false, meaning: "Minimum thickening in metres before recording another annual radius frame.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "growth" },
  { path: "/skeleton/seed", kind: "count", optional: false, meaning: "The specimen: every stage keys its random stream by it, so another seed draws another tree of the same family.", unit: "-", low: 0, high: 4294967295, lowOpen: false, zero: false, applies: "", reach: "mature" },
  { path: "/skeleton/attractors", kind: "count", optional: false, meaning: "How many pull points are scattered through the crown for the branches to grow toward. Raising it fills the crown with more and finer branching; at an `attractor_weight` of zero none are scattered and the row does nothing.", unit: "points", low: 0, high: 1000000, lowOpen: false, zero: false, applies: "`attractorWeight` zero", reach: "mature", dial: { window: [1, 1000000], step: 100 } },
  { path: "/skeleton/samplingAttemptsPerAttractor", kind: "count", optional: false, meaning: "How many random tries the sampler may spend on each pull point before it gives up. Raising it lets a narrow or deeply lobed crown reach its full count of points instead of settling for fewer.", unit: "tries", low: 1, high: 4294967295, lowOpen: false, zero: false, applies: "`attractorWeight` zero", reach: "mature" },
  { path: "/skeleton/step", kind: "real", optional: false, meaning: "How far the crown grows in one step, as a share of the tree's height; the distance at which a pull point is used up is twice it. Raising it grows the crown in longer, coarser strides.", unit: "share of height", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "", reach: "mature", dial: { window: [0.011, 0.033], step: 0.0025 } },
  { path: "/skeleton/habit/reachProbeSteps", kind: "count", optional: false, meaning: "Samples available to find an axis's room within the crown.", unit: "samples", low: 1, high: 4294967295, lowOpen: false, zero: false, applies: "needs `lateralOrders` of one or more: only first-order laterals probe their room", reach: "mature" },
  { path: "/skeleton/habit/apicalDominance", kind: "real", optional: false, meaning: "How far the leader persists into the crown, 0 to 1.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/habit/whorlStrength", kind: "real", optional: false, meaning: "Clustering of laterals at a station against scattering along the axis.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "needs `lateralOrders` of one or more", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/habit/leaderInternode", kind: "real", optional: false, meaning: "Spacing between lateral stations on the leader, in metres.", unit: "m", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "", reach: "mature", dial: { window: [0.05, 3.45], step: 0.5 } },
  { path: "/skeleton/habit/lateralsPerStation", kind: "count", optional: false, meaning: "Laterals borne by one station of the leader.", unit: "laterals", low: 1, high: 12, lowOpen: false, zero: false, applies: "needs `lateralOrders` of one or more; leader stations only", reach: "mature", dial: { window: [1, 4], step: 1 } },
  { path: "/skeleton/habit/lateralPitch", kind: "real", optional: false, meaning: "Initial angle of a lateral from its parent axis, in degrees; on the upright leader this is degrees from vertical.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "needs `lateralOrders` of one or more", reach: "mature", dial: { window: [0, 180], step: 20 } },
  { path: "/skeleton/habit/pitchVariation", kind: "real", optional: false, meaning: "Spread of the lateral pitch, in degrees.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "needs `lateralOrders` of one or more", reach: "mature", dial: { window: [0, 90], step: 5 } },
  { path: "/skeleton/habit/risePrimary", kind: "real", optional: false, meaning: "Signed bend over the length of a first-order axis; positive rises.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [-1, 1], step: 0.025 } },
  { path: "/skeleton/habit/riseSecondary", kind: "real", optional: false, meaning: "Signed bend over the length of a deeper axis; negative hangs.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "needs `lateralOrders` of two or more", reach: "mature", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/skeleton/habit/crookedness", kind: "real", optional: false, meaning: "Heading change between successive growth units, in degrees.", unit: "degrees", low: 0, high: 60, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 15], step: 3 } },
  { path: "/skeleton/habit/lateralSpacing", kind: "real", optional: false, meaning: "Spacing between lateral stations away from the leader, in metres.", unit: "m", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "needs `lateralOrders` of one or more", reach: "mature", dial: { window: [0.000001, 2.925], step: 0.5 } },
  { path: "/skeleton/habit/lateralLengthRatio", kind: "real", optional: false, meaning: "Length of a lateral against its supporting axis.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "needs `lateralOrders` of two or more: first-order laterals take their reach from the probe", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/habit/lateralOrders", kind: "count", optional: false, meaning: "Depth of rule-built orders below the leader.", unit: "orders", low: 0, high: 8, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 8], step: 1 } },
  { path: "/skeleton/habit/attractorWeight", kind: "real", optional: false, meaning: "Weight of the attractor pull beside the axis's own rule heading. At zero no pull points are scattered and every axis holds its own rule heading, and any rise switches the pull on.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/habit/twigTipTaper", kind: "real", optional: false, meaning: "Distal twig radius against the nominal twig radius.", unit: "share", low: 0, high: 1, lowOpen: true, zero: false, applies: "", reach: "mature", dial: { window: [0.05, 0.6], step: 0.1 } },
  { path: "/skeleton/habit/sheddingThreshold", kind: "real", optional: false, meaning: "Monthly vigour threshold; zero disables shedding. The legacy envelope builder interprets it as shell depth.", unit: "share of height·spread", low: 0, high: 1, lowOpen: false, zero: false, applies: "zero sheds nothing", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/habit/stems", kind: "count", optional: false, meaning: "Stems leaving the root. One is the single trunk every tree was, to the byte; a birch, a hazel or a coppiced oak stands on more.", unit: "stems", low: 1, high: 6, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [1, 6], step: 1 } },
  { path: "/skeleton/habit/stemDivergence", kind: "real", optional: false, meaning: "Degrees of bearing between neighbouring stems, about a bearing the seed alone decides. Inert at one stem, which has no neighbour. At zero every stem leaves the root on one bearing, and any rise starts to fan them apart.", unit: "degrees", low: 0, high: 120, lowOpen: false, zero: false, applies: "one stem", reach: "mature", dial: { window: [0, 15], step: 2.5 } },
  { path: "/skeleton/habit/stemLean", kind: "real", optional: false, meaning: "Degrees from vertical the outermost stems tilt away from the root; the ones between tilt in proportion to how far out they stand. Inert at one stem, which stands at the centre and so tilts by none of it. At zero every stem stands upright, and any rise starts the tilt.", unit: "degrees", low: 0, high: 45, lowOpen: false, zero: false, applies: "one stem", reach: "mature", dial: { window: [0, 45], step: 5 } },
  { path: "/skeleton/habit/stemLeanSpread", kind: "real", optional: false, meaning: "How unequally a clump's stems lean, 0 to 1. None of it is the lean above, shared about the clump's centre; all of it leans the stems in their order instead, the first upright and the last by all of `stem_lean`. Inert at one stem, which has nothing to lean against.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "one stem, or `stemLean` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/habit/stemForkHeight", kind: "real", optional: false, meaning: "Where a clump's later stems leave the first, as a share of the bole's height, 0 to 0.5. None of it parts them at the ground; half of it parts them halfway up the bole, with one trunk below. Inert at one stem.", unit: "share of the bole", low: 0, high: 0.5, lowOpen: false, zero: false, applies: "one stem", reach: "mature", dial: { window: [0, 0.5], step: 0.1 } },
  { path: "/skeleton/envelope/height", kind: "real", optional: false, meaning: "The tree's height in metres. Everything else in the crown is measured against it: the crown base sits at `crown_base` of it and the widest radius is `spread` times it.", unit: "m", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [6.5, 40.5], step: 5 } },
  { path: "/skeleton/envelope/crownBase", kind: "real", optional: false, meaning: "Where the crown starts, as a share of the height: below it the crown has no radius at all. Raising it lifts the crown and leaves a longer bare trunk.", unit: "share of height", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/envelope/spread", kind: "real", optional: false, meaning: "The crown's widest radius as a share of the height, so raising it widens the crown without making the tree taller.", unit: "share of height", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 0.675], step: 0.1 } },
  { path: "/skeleton/envelope/fullness", kind: "real", optional: false, meaning: "Where the crown is widest, as a share of the way from the crown base to the top. Raising it carries the widest part higher, so the crown reads top-heavy.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/envelope/shoulder", kind: "real", optional: false, meaning: "How square the crown's outline is. Raising it holds the crown near its full width further toward the top and the base, so the profile reads boxier; lowering it tapers the outline to a point.", unit: "-", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "", reach: "mature", dial: { window: [0.4, 2.8], step: 0.5 } },
  { path: "/skeleton/envelope/irregularity", kind: "real", optional: false, meaning: "How far the outline departs from the smooth shell, as a fraction of the radius there. 0 is the axisymmetric superellipse every tree was before, and every shipped table that leaves it there is untouched.", unit: "share of radius", low: 0, high: 0.5, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 0.5], step: 0.08 } },
  { path: "/skeleton/envelope/lobeScale", kind: "real", optional: false, meaning: "The wavelength of that departure over the shell's own surface, as a fraction of the tree's height: small is many small lumps, 1 is a lobe as long as the tree is tall.", unit: "share of height", low: 0.05, high: 1, lowOpen: false, zero: false, applies: "`irregularity` zero, except that the curtain's drop search steps by it", reach: "mature", dial: { window: [0.05, 1], step: 0.15 } },
  { path: "/skeleton/bias/gravitropism", kind: "real", optional: false, meaning: "How strongly growth is pulled upward, most near the ground. Raising it makes the tree grow more erect. At zero nothing pulls growth upright, and any rise starts that pull.", unit: "-", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1.05], step: 0.15 } },
  { path: "/skeleton/bias/lean", kind: "real", optional: false, meaning: "How far the whole tree leans off vertical, increasing with height. Raising it tips the trunk further in one direction. At zero the tree stands plumb, and any rise starts the lean.", unit: "-", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 0.075], step: 0.01 } },
  { path: "/skeleton/bias/supernatural/enabled", kind: "switch", optional: false, meaning: "Whether the supernatural field bends the wood. Off, the bias reads the amplitude, wavelength and spiral as zero.", unit: "switch", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature" },
  { path: "/skeleton/bias/supernatural/writheAmplitude", kind: "real", optional: false, meaning: "How far a branch may wander from a straight course, as a share of the tree's height. Raising it makes the wood wind and stray more. At zero the wood holds a straight course and the spiral rate does nothing, and any rise starts the wander.", unit: "share of height", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "`enabled` off", reach: "mature", dial: { window: [0, 0.165], step: 0.025 } },
  { path: "/skeleton/bias/supernatural/writheWavelength", kind: "real", optional: false, meaning: "How long each of those wanders runs, as a share of the height. Raising it gives fewer, lazier bends; lowering it gives tighter kinks.", unit: "share of height", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "`enabled` off, or `writheAmplitude` zero", reach: "mature", dial: { window: [0.225, 0.675], step: 0.05 } },
  { path: "/skeleton/bias/supernatural/spiralRate", kind: "real", optional: false, meaning: "How many full turns the wander winds around the trunk over the tree's height. Raising it tightens the spiral. At zero the wander winds around nothing, and any rise starts the spiral.", unit: "turns over the height", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "`enabled` off, or `writheAmplitude` zero", reach: "mature", dial: { window: [0, 3.9], step: 0.5 } },
  { path: "/skeleton/bias/supernatural/maxWritheMagnitude", kind: "real", optional: false, meaning: "The ceiling on how hard the wander may pull in any one step, so the other writhe rows cannot bend the wood arbitrarily.", unit: "-", low: 0, high: 8, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 8], step: 1 } },
  { path: "/skeleton/twigs/twig/diameter", kind: "real", optional: false, meaning: "The finished thickness of a twig in metres. Wood at or below half of it is drawn as a twig, so raising it thickens the twig layer and hands more of the fine wood to it.", unit: "m", low: 0.000001, high: 1000000, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0.000001, 1000000], step: 0.001 } },
  { path: "/skeleton/twigs/twig/length", kind: "real", optional: false, meaning: "The length in metres a twig shoot grows before it stops, and the whole of one internode on leaf-bearing wood. Raising it lengthens every twig, so the crown carries a deeper, shaggier skin.", unit: "m", low: 0.000001, high: 1000000, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0.000001, 1000000], step: 0.05 } },
  { path: "/skeleton/twigs/twig/internodeLength", kind: "real", optional: false, meaning: "Metres between the joints on wood thicker than the bearing diameter, and the spacing of the stations a leaf sits on. Raising it gives longer segments, so laterals and leaves sit further apart.", unit: "m", low: 0.000001, high: 1000000, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0.000001, 1000000], step: 0.015 } },
  { path: "/skeleton/twigs/twig/stationsPerInternode", kind: "count", optional: false, meaning: "How many leaf stations sit at each joint, each turned its own share of a full turn around the shoot. Raising it crowds more leaves onto the same joints.", unit: "stations", low: 1, high: 32, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [1, 32], step: 1 } },
  { path: "/skeleton/twigs/twig/bearingDiameter", kind: "real", optional: false, meaning: "The thickness in metres at or below which a shoot bears leaves and side shoots of its own. Raising it lets thicker wood bear, so foliage reaches further back down the branch.", unit: "m", low: 0.000001, high: 1000000, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0.000001, 1000000], step: 0.01 } },
  { path: "/skeleton/twigs/lengthRatio", kind: "real", optional: false, meaning: "A shoot's length as a share of the one that bore it. Raising it makes each generation of twigs longer relative to its parent.", unit: "share", low: 0.05, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0.05, 1], step: 0.15 } },
  { path: "/skeleton/twigs/ratioPower", kind: "real", optional: false, meaning: "How much thinner a shoot is than its parent for the same drop in length. Raising it leaves side shoots finer.", unit: "-", low: 0, high: 8, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 8], step: 1 } },
  { path: "/skeleton/twigs/internodeFactor", kind: "real", optional: false, meaning: "The fewest of its own diameters a segment of wood may span. Raising it makes segments longer for the same thickness, so there are fewer joints and the wood reads straighter.", unit: "-", low: 0.05, high: 32, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0.05, 32], step: 0.5 } },
  { path: "/skeleton/twigs/maxInternodes", kind: "count", optional: false, meaning: "The ceiling on segments one length of wood may be cut into. It binds only where the two lengths above would cut more, and there it caps the cost rather than states a look.", unit: "internodes", low: 1, high: 4294967295, lowOpen: false, zero: false, applies: "", reach: "mature" },
  { path: "/skeleton/twigs/laterals", kind: "count", optional: false, meaning: "How many side shoots leave each station along a twig. Raising it crowds more twigs onto the same length of wood.", unit: "laterals", low: 0, high: 7, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 7], step: 1 } },
  { path: "/skeleton/twigs/generations", kind: "count", optional: false, meaning: "Twig-law generations of branching, 1 to 6. A lateral born at or past this generation is a twig whatever the pipe model left its radius, so the twig layer's depth is a row a table states rather than a consequence of how thick the wood is.", unit: "generations", low: 1, high: 6, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [1, 6], step: 1 } },
  { path: "/skeleton/twigs/limbRadius", kind: "real", optional: false, meaning: "The share of the trunk's radius at or below which wood starts bearing twigs. Raising it lets twigs start on thicker wood, so they reach further back toward the trunk.", unit: "share of root radius", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/twigs/reach", kind: "real", optional: false, meaning: "How deep the outer skin of the crown is that only twigs may fill, as a share of the crown: the scaffold is grown into what is left inside it. Raising it holds the structural wood further in and leaves a deeper twig layer.", unit: "share", low: 0, high: 0.9, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 0.9], step: 0.15 } },
  { path: "/skeleton/twigs/angle", kind: "real", optional: false, meaning: "The degrees a side shoot leaves its parent. Raising it swings twigs further out toward square with the branch.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 90], step: 1.5 } },
  { path: "/skeleton/twigs/angleVariation", kind: "real", optional: false, meaning: "How many degrees that departure angle varies shoot to shoot. Raising it makes the twig layer less uniform.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 90], step: 1.5 } },
  { path: "/skeleton/twigs/vigourVariation", kind: "real", optional: false, meaning: "How much shoot length varies shoot to shoot. Raising it gives a more uneven, less combed twig layer.", unit: "share", low: 0, high: 0.95, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 0.95], step: 0.15 } },
  { path: "/skeleton/twigs/divergence", kind: "real", optional: false, meaning: "The degrees each successive shoot is turned around the wood that bears it. Raising it swings the next shoot further around, so the twigs spiral differently.", unit: "degrees", low: -Infinity, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [68.752, 206.256], step: 20 } },
  { path: "/skeleton/twigs/hang", kind: "real", optional: false, meaning: "How strongly a shoot hangs, 0 to 3. At 0 nothing hangs and the local law is the ordinary one; at 1 a curtain takes its full droop.", unit: "-", low: 0, high: 3, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 3], step: 0.5 } },
  { path: "/skeleton/twigs/pendulousLength", kind: "real", optional: false, meaning: "Metres a pendulous shoot grows before it stops, and the length its droop reaches the cap over.", unit: "m", low: 0.05, high: 5, lowOpen: false, zero: false, applies: "`hang` zero", reach: "mature", dial: { window: [0.05, 5], step: 1 } },
  { path: "/skeleton/twigs/pendulousRadius", kind: "real", optional: false, meaning: "Fraction of the root radius at or below which a station's shoots hang.", unit: "share of stem radius", low: 0, high: 1, lowOpen: false, zero: false, applies: "`hang` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/twigs/curtainSeparation", kind: "real", optional: false, meaning: "Degrees between neighbouring shoots in a curtain.", unit: "degrees", low: 1, high: 45, lowOpen: false, zero: false, applies: "`hang` zero", reach: "mature", dial: { window: [1, 45], step: 1.5 } },
  { path: "/skeleton/twigs/sag", kind: "real", optional: false, meaning: "How far toward straight down a hanging shoot's course has turned by the end of its pendulous length, 0 to 1: at 0 the shoot holds the direction it departed with and the curtain is a set of rods, at 1 it hangs vertical, and the turn is spread along the run as an arc steepest at the wood that bears it.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "`hang` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/twigs/pendulousVariation", kind: "real", optional: false, meaning: "How much shorter than the pendulous length a hanging shoot may run, 0 to 1: each shoot's own length is the pendulous length times one minus this times a draw in 0 to 1 keyed by the shoot and the seed. At 0 every shoot has the one length the table states.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "`hang` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/twigs/curtainDrop", kind: "real", optional: false, meaning: "How far below the shell's lower surface a hanging shoot may fall, 0 to 1, as a share of the way from that surface down to the clearance: at 0 the shell binds a hanging shoot as it binds every other, at 1 the shoot may fall to the clearance. Only a shoot that hangs, only under the crown's footprint.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/twigs/curtainClearance", kind: "real", optional: false, meaning: "Metres above the ground no hanging shoot falls below, 0 to 5. Never above the crown's own base, whatever the row says.", unit: "m", low: 0, high: 5, lowOpen: false, zero: false, applies: "unless `sag` or `curtainDrop` is above zero", reach: "mature", dial: { window: [0, 5], step: 1 } },
  { path: "/skeleton/twigs/maxDroop", kind: "real", optional: false, meaning: "The furthest a hanging shoot may bend toward straight down. Raising it lets curtains hang more heavily.", unit: "-", low: 0, high: 10, lowOpen: false, zero: false, applies: "`hang` zero", reach: "mature", dial: { window: [0, 10], step: 1.5 } },
  { path: "/skeleton/twigs/curtainStepClearance", kind: "real", optional: false, meaning: "How close to the ground a hanging curtain may reach before it stops growing. Raising it lets curtains hang nearer the floor.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "`hang` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/growth/influenceRadius", kind: "real", optional: true, meaning: "Metres a pull point may reach to steer the wood nearest it; unset, the crown's own volume and the point count decide it. Raising it lets distant points draw a branch across the crown.", unit: "m", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "`attractorWeight` zero", reach: "mature" },
  { path: "/skeleton/growth/killDistance", kind: "real", optional: true, meaning: "Metres within which a pull point counts as reached and stops pulling; unset, twice the step distance. Raising it uses the points up sooner, so branches stop shorter and the crown fills coarsely.", unit: "m", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "`attractorWeight` zero", reach: "mature" },
  { path: "/skeleton/growth/stepDistance", kind: "real", optional: true, meaning: "Metres of wood laid down in one growth step; unset, the tree's height times `step`. Raising it lays down longer, coarser segments.", unit: "m", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "", reach: "mature" },
  { path: "/skeleton/growth/trunkHeight", kind: "real", optional: true, meaning: "Metres of bare trunk before the crown may start; unset, the envelope's own crown base. Raising it lifts the whole crown and leaves a longer clear bole.", unit: "m", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "", reach: "mature" },
  { path: "/skeleton/growth/maxNodes", kind: "count", optional: true, meaning: "The ceiling on nodes the crown may grow; unset, the shipped default. Growth stops at it, so raising it changes only a crown that reached it.", unit: "nodes", low: 1, high: 4294967295, lowOpen: false, zero: false, applies: "", reach: "mature" },
  { path: "/skeleton/growth/maxTurnPerStep", kind: "real", optional: true, meaning: "The most a growing shoot may turn in one step, in degrees; unset, 35. At 180 or more nothing is limited.", unit: "degrees", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "mature" },
  { path: "/radii/trunkRadius", kind: "real", optional: false, meaning: "The trunk's radius at the ground as a share of the tree's height, so raising it thickens every piece of wood in proportion.", unit: "share of height", low: 0.000004, high: 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0.000004, 0.023], step: 0.002 } },
  { path: "/radii/forkExponent", kind: "real", optional: false, meaning: "How wood divides at a fork. The parent's area is the sum of the children's radii raised to this power, so raising it leaves the children thicker for the same parent.", unit: "-", low: 1, high: 8, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [1, 8], step: 1 } },
  { path: "/radii/lengthTaper", kind: "real", optional: false, meaning: "How fast wood thins along its own length. Raising it makes a branch narrow more sharply from its base to its tip.", unit: "per height", low: 0, high: 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 0.8], step: 0.1 } },
  { path: "/radii/maxTaperExponent", kind: "real", optional: false, meaning: "The ceiling on accumulated taper, so no single long branch can thin away to nothing. Raising it lets long branches taper further.", unit: "-", low: 0, high: 64, lowOpen: false, zero: false, applies: "`lengthTaper` zero", reach: "mature", dial: { window: [0, 64], step: 2 } },
  { path: "/surface/radialSegments", kind: "count", optional: false, meaning: "How many sides each piece of wood is drawn with. Raising it makes the wood rounder and smoother, and costs triangles.", unit: "sides", low: 3, high: 64, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [3, 64], step: 2 } },
  { path: "/surface/lobes", kind: "count", optional: false, meaning: "How many ridges run up around the trunk. Raising it gives the bark more flutes; zero is a plain round bole.", unit: "lobes", low: 0, high: 16, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 16], step: 2 } },
  { path: "/surface/lobeDepth", kind: "real", optional: false, meaning: "How deep the flutes between those ridges cut, as a share of the wood's own radius. Raising it makes the fluting more pronounced. At zero the bole is plainly round whatever the ridge count says, and any rise starts cutting the flutes.", unit: "share of radius", low: 0, high: 0.9, lowOpen: false, zero: false, applies: "`lobes` zero", reach: "mature", dial: { window: [0, 0.9], step: 0.15 } },
  { path: "/surface/twistRate", kind: "real", optional: false, meaning: "How many turns those ridges make over the tree's height. Raising it winds them more tightly around the trunk.", unit: "turns over the height", low: -64, high: 64, lowOpen: false, zero: false, applies: "unless `lobes` and `lobeDepth` are both above zero", reach: "mature", dial: { window: [-64, 64], step: 1 } },
  { path: "/surface/flareRadius", kind: "real", optional: false, meaning: "How much wider the trunk is where it meets the ground, as a multiple of its own radius. Raising it gives a broader buttress.", unit: "multiple of radius", low: 1, high: 8, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [1, 8], step: 0.2 } },
  { path: "/surface/flareFalloff", kind: "real", optional: false, meaning: "How far up the trunk that flare reaches, as a share of the height. Raising it carries the swelling further up the bole.", unit: "share of height", low: 0.0001, high: 1, lowOpen: false, zero: false, applies: "`flareRadius` one", reach: "mature", dial: { window: [0.0001, 1], step: 0.15 } },
  { path: "/surface/flareDepth", kind: "real", optional: false, meaning: "How deep the trunk's base is sunk below the ground, as a share of the height. Raising it buries more of the flare.", unit: "share of height", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/surface/forkSocket", kind: "real", optional: false, meaning: "How deeply a child branch is set into its parent at a fork. Raising it sinks the junction further in, so the two read as one piece of wood rather than two tubes meeting.", unit: "share", low: 0, high: 0.9, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 0.9], step: 0.15 } },
  { path: "/surface/socketContainment", kind: "real", optional: false, meaning: "Fraction of the parent's inscribed radius available for a socket.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "`forkSocket` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/surface/forkSwell", kind: "real", optional: false, meaning: "How much wood thickens at a fork. Raising it leaves a more pronounced collar where a branch leaves its parent.", unit: "multiple of radius", low: 1, high: 4, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [1, 4], step: 0.5 } },
  { path: "/canopy/shootRadius", kind: "real", optional: false, meaning: "Wood at or below this fraction of the root radius bears foliage of its own, beside whatever the twig layer marks. Zero leaves the twigs alone with it; without a twig layer it is what selects the terminal shoots.", unit: "share of root radius", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.005 } },
  { path: "/canopy/spacing", kind: "real", optional: false, meaning: "Metres between leaves along a shoot, as a share of the tree's height. Raising it spreads the leaves further apart, so the crown carries fewer of them.", unit: "share of height", low: 0.001, high: 1000000, lowOpen: false, zero: false, applies: "only a placement with no twig table reads it, which no production caller passes", reach: "deprecated" },
  { path: "/canopy/divergence", kind: "real", optional: false, meaning: "The degrees each successive leaf is turned around its shoot. Raising it turns the next leaf further round, so the leaves spiral differently.", unit: "degrees", low: -1000000000, high: 1000000000, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [-1000000000, 1000000000], step: 15 } },
  { path: "/canopy/clump", kind: "count", optional: false, meaning: "How many extra leaves are gathered at the end of a shoot that has no twig layer. Raising it packs a denser tuft at the tip.", unit: "leaves", low: 0, high: 64, lowOpen: false, zero: false, applies: "only a placement with no twig table reads it, which no production caller passes", reach: "deprecated" },
  { path: "/canopy/clumpSpan", kind: "real", optional: false, meaning: "How far back from the tip that tuft is scattered, as a share of the shoot's length. Raising it spreads the tuft further down the shoot.", unit: "share of the shoot", low: 0, high: 1, lowOpen: false, zero: false, applies: "only a placement with no twig table reads it, which no production caller passes", reach: "deprecated" },
  { path: "/canopy/outward", kind: "real", optional: false, meaning: "How far a leaf turns away from the trunk. Raising it points the leaves outward, away from the tree's axis.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/canopy/upward", kind: "real", optional: false, meaning: "How far a leaf turns toward the sky. Raising it tips the leaves up.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/canopy/forwardLean", kind: "real", optional: false, meaning: "Lean along the shoot, as a fraction of the radial off the wood.", unit: "share of the radial", low: -1, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/canopy/leanRise", kind: "real", optional: false, meaning: "Further lean along the shoot on radials that face upward.", unit: "share of the radial", low: -2, high: 2, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [-2, 2], step: 0.5 } },
  { path: "/canopy/surfaceContact", kind: "real", optional: false, meaning: "The station sits on the shoot axis at 0 and on the wood's own contact surface at 1; the surface is built whenever it is positive.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/canopy/scatter", kind: "real", optional: false, meaning: "The degrees a leaf may be turned at random from where it was placed. Raising it leaves the crown less combed.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 90], step: 15 } },
  { path: "/canopy/size", kind: "real", optional: false, meaning: "The size every leaf is drawn at, as a multiple of the element's own dimensions. Raising it enlarges every leaf.", unit: "multiple of the element", low: 0, high: 1000, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1000], step: 0.15 } },
  { path: "/canopy/sizeVariation", kind: "real", optional: false, meaning: "How far leaf size varies leaf to leaf, as a share of that size. Raising it mixes larger and smaller leaves more widely.", unit: "share", low: 0, high: 0.9, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 0.9], step: 0.15 } },
  { path: "/canopy/shortShootSpacing", kind: "real", optional: false, meaning: "Metres between short shoots along limb and branch wood: spurs a few centimetres long, each ending in a cluster of leaves. Zero grows none.", unit: "m", low: 0.01, high: 1000, lowOpen: false, zero: true, applies: "", reach: "mature", dial: { window: [0.01, 0.08], step: 0.01 } },
  { path: "/canopy/shortShootRadius", kind: "real", optional: false, meaning: "Wood thicker than this fraction of the stem's radius carries no short shoot, and neither does twig wood or anything below the crown base.", unit: "share of stem radius", low: 0, high: 1, lowOpen: false, zero: false, applies: "`shortShootSpacing` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/canopy/shortShootLength", kind: "real", optional: false, meaning: "Metres from the bark to the cluster a short shoot carries.", unit: "m", low: 0, high: 0.5, lowOpen: false, zero: false, applies: "`shortShootSpacing` zero", reach: "mature", dial: { window: [0, 0.5], step: 0.1 } },
  { path: "/canopy/shortShootLeaves", kind: "count", optional: false, meaning: "Leaves in one short shoot's cluster, 1 to 8.", unit: "leaves", low: 1, high: 8, lowOpen: false, zero: false, applies: "`shortShootSpacing` zero", reach: "mature", dial: { window: [2, 12], step: 2 } },
  { path: "/canopy/shortShootSpread", kind: "real", optional: false, meaning: "Degrees either side of its short shoot's bearing a cluster's leaves fan across, held level: 90 is a half circle, 0 stacks them.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "`shortShootSpacing` zero", reach: "mature", dial: { window: [0, 90], step: 10 } },
  { path: "/canopy/limbClumping", kind: "real", optional: false, meaning: "How far into each limb system the gap between it and its neighbours reaches, as a share of the way from their shared boundary to the system's centre: each limb system then keeps a rounded leaf mass of its own. Zero, the neutral, thins nothing.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/canopy/clumpSystemOrder", kind: "count", optional: false, meaning: "How deep a lateral may be and still start a limb system of its own. Raising it parts the crown into more and smaller leaf masses; it does nothing until `limb_clumping` is above zero.", unit: "order", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [1, 5], step: 1 } },
  { path: "/canopy/clumpNeighbours", kind: "count", optional: false, meaning: "Nearest neighbours and cell crossings in the clumping approximation.", unit: "neighbours", low: 1, high: 4294967295, lowOpen: false, zero: false, applies: "`limbClumping` zero", reach: "mature" },
  { path: "/canopy/rosetteFronds", kind: "count", optional: false, meaning: "Fronds the rosette bears at the apex of each stem. At zero no rosette stands and the canopy clothes wood as it always did; any rise makes the rosette the tree's only foliage.", unit: "fronds", low: 0, high: 128, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 128], step: 1 } },
  { path: "/canopy/rosetteDivergence", kind: "real", optional: false, meaning: "The degrees each successive frond is turned about the apex.", unit: "degrees", low: -1000000000, high: 1000000000, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [90, 180], step: 2 } },
  { path: "/canopy/rosettePitch", kind: "real", optional: false, meaning: "Degrees from the axis the youngest frond stands: 0 upright, 90 level, 180 hanging.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "`rosetteFronds` zero", reach: "mature", dial: { window: [0, 180], step: 5 } },
  { path: "/canopy/rosettePitchSpread", kind: "real", optional: false, meaning: "How many degrees further than the youngest the oldest frond leans, so the crown opens from a spike to a skirt.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "`rosetteFronds` zero", reach: "mature", dial: { window: [0, 180], step: 5 } },
  { path: "/canopy/rosetteDepth", kind: "real", optional: false, meaning: "Metres below the apex the frond insertions are spread down the axis. At zero every frond leaves one point.", unit: "m", low: 0, high: 100, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 4], step: 0.05 } },
  { path: "/canopy/leafletCount", kind: "count", optional: false, meaning: "Leaflets one placement carries along its rachis. One is the single blade every family drew.", unit: "leaflets", low: 1, high: 256, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [1, 256], step: 2 } },
  { path: "/canopy/rachisLength", kind: "real", optional: false, meaning: "Metres of rachis the leaflets are strung along. At zero the placement is one blade whatever the count says.", unit: "m", low: 0, high: 1000, lowOpen: false, zero: false, applies: "`leafletCount` one", reach: "mature", dial: { window: [0, 8], step: 0.1 } },
  { path: "/canopy/leafletPitch", kind: "real", optional: false, meaning: "The degrees a leaflet leaves its rachis.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "no leaflet grouping (`leafletCount` one or `rachisLength` zero)", reach: "mature", dial: { window: [0, 90], step: 3 } },
  { path: "/canopy/rachisArch", kind: "real", optional: false, meaning: "How far the rachis bends out of the straight line from its station, as a share of its length. Positive arches up, negative droops.", unit: "share of the rachis", low: -1, high: 1, lowOpen: false, zero: false, applies: "no leaflet grouping (`leafletCount` one or `rachisLength` zero)", reach: "mature", dial: { window: [-1, 1], step: 0.05 } },
  { path: "/canopy/terminalLeaflet", kind: "real", optional: false, meaning: "Whether a single leaflet closes the rachis's end, blended 0 to 1: the last leaflet turns from standing off the rachis to lying along it.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "no leaflet grouping (`leafletCount` one or `rachisLength` zero)", reach: "mature", dial: { window: [0, 1], step: 0.05 } },
  { path: "/canopy/leafBases", kind: "count", optional: false, meaning: "Bases of shed fronds the stem keeps below its crown, clothing the trunk. At zero the trunk is bare and the bark is what it always was; any rise carries the crown's own spiral down it.", unit: "bases", low: 0, high: 256, lowOpen: false, zero: false, applies: "`leafBaseLength` zero", reach: "mature", dial: { window: [0, 256], step: 4 } },
  { path: "/canopy/leafBaseLength", kind: "real", optional: false, meaning: "Metres a retained base stands out from the bark. At zero no base is drawn whatever the count says.", unit: "m", low: 0, high: 10, lowOpen: false, zero: false, applies: "`leafBases` zero", reach: "mature", dial: { window: [0, 10], step: 0.05 } },
  { path: "/canopy/leafBaseRadius", kind: "real", optional: false, meaning: "How thick a base is where it leaves the bark, as a share of the stem's own radius there. Raising it leaves a broader boot.", unit: "share of stem radius", low: 0, high: 1, lowOpen: false, zero: false, applies: "no leaf base", reach: "mature", dial: { window: [0, 1], step: 0.05 } },
  { path: "/canopy/leafBasePitch", kind: "real", optional: false, meaning: "Degrees from the stem's axis a base points: 0 flat against the trunk, 90 square out of it, 180 turned back down.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "no leaf base", reach: "mature", dial: { window: [0, 180], step: 5 } },
  { path: "/canopy/leafBaseWeathering", kind: "real", optional: false, meaning: "How far the lowest and oldest base is worn back against the newest, in both its length and its girth. At zero every base stands full down the whole trunk, and any rise wears the foot away.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "no leaf base", reach: "mature", dial: { window: [0, 1], step: 0.05 } },
  { path: "/canopy/leafBaseWidth", kind: "real", optional: false, meaning: "How broad a retained base is across the trunk, as a share of the cell the crown's spiral gives it on the bark: at 1 every base meets its neighbours edge to edge whatever the count and the trunk's girth, below it the bark shows between them and above it they crowd into each other. At zero the base is the round peg the radius row sizes and the lattice rows say nothing; any rise packs the bases into the lattice.", unit: "share of the lattice cell", low: 0, high: 2, lowOpen: false, zero: false, applies: "no leaf base", reach: "mature", dial: { window: [0, 2], step: 0.05 } },
  { path: "/canopy/leafBaseFlatness", kind: "real", optional: false, meaning: "How flat-sided a lattice base is drawn: 0 the ellipse through its cell's corners, 1 the cell itself, a diamond with flat faces that meets each neighbour along a straight edge and is cut square at its outer end. At zero the section stays round, and at no width it reads nothing.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "no leaf base, or `leafBaseWidth` zero", reach: "mature", dial: { window: [0, 1], step: 0.1 } },
  { path: "/canopy/acanthophylls", kind: "count", optional: false, meaning: "Leaflets at a frond's base borne as spines rather than blades. At zero the frond carries blades all the way down; any rise hardens that many of them.", unit: "leaflets", low: 0, high: 256, lowOpen: false, zero: false, applies: "`acanthophyllLength` zero, or no leaflet grouping (`leafletCount` one or `rachisLength` zero)", reach: "mature", dial: { window: [0, 256], step: 1 } },
  { path: "/canopy/acanthophyllLength", kind: "real", optional: false, meaning: "The share of a leaflet's own size a spine is drawn at. At zero no spine is drawn whatever the count says.", unit: "share of a leaflet", low: 0, high: 1, lowOpen: false, zero: false, applies: "`acanthophylls` zero, or no leaflet grouping (`leafletCount` one or `rachisLength` zero)", reach: "mature", dial: { window: [0, 1], step: 0.05 } },
  { path: "/canopy/acanthophyllPitch", kind: "real", optional: false, meaning: "The degrees a spine leaves the rachis, in place of the leaflet's own pitch.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "no spine", reach: "mature", dial: { window: [0, 90], step: 3 } },
  { path: "/canopy/skirtFronds", kind: "count", optional: false, meaning: "Dead fronds a rosette keeps below its living crown, continuing the crown's own spiral down the stem. At zero no frond is kept and the crown ends at its oldest living frond; any rise hangs that many.", unit: "fronds", low: 0, high: 128, lowOpen: false, zero: false, applies: "`skirtLength` zero, or `rosetteFronds` zero", reach: "mature", dial: { window: [0, 128], step: 2 } },
  { path: "/canopy/skirtPitch", kind: "real", optional: false, meaning: "Degrees from the axis a dead frond hangs: 0 upright, 90 level, 180 collapsed straight down against the stem.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "no dead frond", reach: "mature", dial: { window: [0, 180], step: 5 } },
  { path: "/canopy/skirtLength", kind: "real", optional: false, meaning: "A dead frond's length as a share of a living one's, rachis and leaflets alike. At zero no dead frond is drawn whatever the count says.", unit: "share of a living frond", low: 0, high: 1, lowOpen: false, zero: false, applies: "`skirtFronds` zero, or `rosetteFronds` zero", reach: "mature", dial: { window: [0, 1], step: 0.05 } },
  { path: "/canopy/maxInstances", kind: "count", optional: false, meaning: "Hard total budget. Exceeding it returns an error, never partial foliage.", unit: "leaves", low: 1, high: 18446744073709552000, lowOpen: false, zero: false, applies: "", reach: "mature" },
  { path: "/element/connectorLength", kind: "real", optional: false, meaning: "Metres of petiole or woody peg below the blade, excluded from unit dimensions. Every element carries one.", unit: "m", low: 0.000001, high: 1000, lowOpen: false, zero: false, applies: "`card`", reach: "mature", dial: { window: [0.000001, 0.0325], step: 0.005 } },
  { path: "/element/length", kind: "real", optional: false, meaning: "Blade/needle longitudinal extent, excluding connector, in metres.", unit: "m", low: 0.0001, high: 1000, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0.0001, 1000], step: 0.025 } },
  { path: "/element/width", kind: "real", optional: false, meaning: "The blade's greatest width in metres, across the midrib. Raising it makes every leaf broader.", unit: "m", low: 0.0001, high: 1000, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0.0001, 1000], step: 0.015 } },
  { path: "/element/widestAt", kind: "real", optional: false, meaning: "Where the blade is widest, as a share of the way from its base to its tip. Raising it carries the widest point toward the tip.", unit: "share of the blade", low: 0.05, high: 0.95, lowOpen: false, zero: false, applies: "`card`", reach: "mature", dial: { window: [0.05, 0.95], step: 0.15 } },
  { path: "/element/baseFullness", kind: "real", optional: false, meaning: "How fast the blade fills out above its stalk. Raising it draws the base in, so the leaf reads wedge-shaped rather than rounded.", unit: "-", low: 0.2, high: 8, lowOpen: false, zero: false, applies: "`card`", reach: "mature", dial: { window: [0.2, 8], step: 0.15 } },
  { path: "/element/tipSharpness", kind: "real", optional: false, meaning: "How fast the blade narrows toward its point. Raising it draws the tip out into a sharper point.", unit: "-", low: 0.2, high: 8, lowOpen: false, zero: false, applies: "`card`", reach: "mature", dial: { window: [0.2, 8], step: 1 } },
  { path: "/element/cup", kind: "real", optional: false, meaning: "How far the blade's margins lift out of its own plane, as a share of the half-width there. Raising it dishes the leaf more deeply along the midrib.", unit: "share of the half-width", low: -2, high: 2, lowOpen: false, zero: false, applies: "`card`", reach: "mature", dial: { window: [-2, 2], step: 0.5 } },
  { path: "/element/curl", kind: "real", optional: false, meaning: "How far the blade bends along its length, as a share of its length at the tip. Raising it curls the tip further out of the plane its base stands in.", unit: "share of the length", low: -2, high: 2, lowOpen: false, zero: false, applies: "`card`", reach: "mature", dial: { window: [-2, 2], step: 0.5 } },
  { path: "/element/lobeCount", kind: "count", optional: false, meaning: "Lobes along each margin; 0 is an entire margin.", unit: "lobes", low: 0, high: 8, lowOpen: false, zero: false, applies: "`lobeDepth` zero", reach: "mature", dial: { window: [0, 8], step: 1 } },
  { path: "/element/lobeDepth", kind: "real", optional: false, meaning: "How far each sinus cuts toward the midrib, 0 to 1. At zero the margin is entire whatever the lobe count says, and any rise starts cutting the sinuses.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/element/sectionRoundness", kind: "real", optional: false, meaning: "Flat blade at 0, four-sided shaft at 1.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/element/axialSegments", kind: "count", optional: false, meaning: "How many sections the blade is built from along its length. Raising it draws the outline and any lobes more smoothly, at more triangles per leaf.", unit: "sections", low: 2, high: 64, lowOpen: false, zero: false, applies: "`card`", reach: "mature", dial: { window: [2, 64], step: 8 } },
  { path: "/element/crossSegments", kind: "count", optional: false, meaning: "How many columns the blade is built from across its width. Raising it draws the section and the margins more smoothly, at more triangles per leaf.", unit: "columns", low: 2, high: 64, lowOpen: false, zero: false, applies: "`card`", reach: "mature", dial: { window: [2, 64], step: 1 } },
  { path: "/element/card", kind: "switch", optional: false, meaning: "A flat two-triangle card in place of the modelled blade: no outline, no cup or curl, no sections and no connector.", unit: "switch", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature" },
  { path: "/material/barkRed", kind: "real", optional: false, meaning: "The bark's own colour, a linear reflectance per channel. Raising a channel pushes mature bark toward that colour.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/barkGreen", kind: "real", optional: false, meaning: "The green channel of `bark_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/barkBlue", kind: "real", optional: false, meaning: "The blue channel of `bark_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/barkRoughness", kind: "real", optional: false, meaning: "How diffuse the bark is: 0 is a mirror, 1 is chalk.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/shootRed", kind: "real", optional: false, meaning: "The young wood's own colour, before its bark has formed. Wood thinner than `shoot_radius` takes it, and gives it up to the bark colour on a smoothstep of its radius by twice that; zero means no wood is young.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`shootRadius` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/shootGreen", kind: "real", optional: false, meaning: "The green channel of `shoot_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`shootRadius` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/shootBlue", kind: "real", optional: false, meaning: "The blue channel of `shoot_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`shootRadius` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/shootRadius", kind: "real", optional: false, meaning: "Metres. The radius below which wood is young.", unit: "m", low: 0, high: 0.1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 0.1], step: 0.015 } },
  { path: "/material/leafFrontRed", kind: "real", optional: false, meaning: "The colour of a leaf's upper face, a linear reflectance per channel. Raising a channel pushes the sunlit face toward it.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.02 } },
  { path: "/material/leafFrontGreen", kind: "real", optional: false, meaning: "The green channel of `leaf_front_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/leafFrontBlue", kind: "real", optional: false, meaning: "The blue channel of `leaf_front_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.015 } },
  { path: "/material/leafBackRed", kind: "real", optional: false, meaning: "The colour of a leaf's underside, shown wherever the eye sees the back of a blade. Raising a channel pushes that face toward it.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/leafBackGreen", kind: "real", optional: false, meaning: "The green channel of `leaf_back_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.025 } },
  { path: "/material/leafBackBlue", kind: "real", optional: false, meaning: "The blue channel of `leaf_back_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.025 } },
  { path: "/material/leafDeadRed", kind: "real", optional: false, meaning: "The colour a dead frond has aged to, both faces alike, shown only on the fronds a rosette keeps below its living crown. Raising a channel pushes the skirt toward it.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no dead frond (`canopy.skirtFronds` zero)", reach: "mature", dial: { window: [0, 1], step: 0.05 } },
  { path: "/material/leafDeadGreen", kind: "real", optional: false, meaning: "The green channel of `leaf_dead_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no dead frond (`canopy.skirtFronds` zero)", reach: "mature", dial: { window: [0, 1], step: 0.05 } },
  { path: "/material/leafDeadBlue", kind: "real", optional: false, meaning: "The blue channel of `leaf_dead_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no dead frond (`canopy.skirtFronds` zero)", reach: "mature", dial: { window: [0, 1], step: 0.05 } },
  { path: "/material/hueRangeLow", kind: "real", optional: false, meaning: "The hue offsets one leaf may take, as a fraction of the colour circle.", unit: "-", low: -0.5, high: 0.5, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [-0.5, 0], step: 0.0025 } },
  { path: "/material/hueRangeHigh", kind: "real", optional: false, meaning: "The upper end of the hue offsets `hue_range_low` opens.", unit: "-", low: -0.5, high: 0.5, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 0.5], step: 0.0025 } },
  { path: "/material/brightnessRangeLow", kind: "real", optional: false, meaning: "The brightness offsets one leaf may take, about no change at all.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [-1, 0], step: 0.015 } },
  { path: "/material/brightnessRangeHigh", kind: "real", optional: false, meaning: "The upper end of the brightness offsets `brightness_range_low` opens.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.015 } },
  { path: "/material/interiorDarkening", kind: "real", optional: false, meaning: "How far a leaf deep inside the crown is darkened towards a shaded mass.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/ridgeScale", kind: "real", optional: false, meaning: "Circumferential ridge spacing in metres; zero disables relief.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.02 } },
  { path: "/material/plateScale", kind: "real", optional: false, meaning: "Axial scale control in metres; spacing is bounded to 1.5–2 ridge widths. Larger ratios lengthen and deepen furrows; zero omits breaks.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`ridgeScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/furrowStrength", kind: "real", optional: false, meaning: "Furrow width and depth together; zero leaves tightly packed scales.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`ridgeScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/roughnessDetail", kind: "real", optional: false, meaning: "Roughness variation about the base value, clamped to 0..1.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/veinScale", kind: "real", optional: false, meaning: "Secondary vein pairs per blade, continuously interpolated.", unit: "-", low: 0, high: 32, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 32], step: 0.5 } },
  { path: "/material/veinContrast", kind: "real", optional: false, meaning: "How far the veins are lightened and the blade between them darkened. Raising it makes the venation read more sharply.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/transmissionStrength", kind: "real", optional: false, meaning: "How brightly a backlit leaf glows with the light that came through it. Raising it lifts that glow.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/transmissionRed", kind: "real", optional: false, meaning: "Linear transmission tint, in 0..1 per channel.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`transmissionStrength` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/transmissionGreen", kind: "real", optional: false, meaning: "The green channel of `transmission_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`transmissionStrength` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/transmissionBlue", kind: "real", optional: false, meaning: "The blue channel of `transmission_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`transmissionStrength` zero", reach: "mature", dial: { window: [0, 1], step: 0.01 } },
  { path: "/material/thickness", kind: "real", optional: false, meaning: "Optical thickness: attenuation is exp(-thickness).", unit: "-", low: 0, high: 8, lowOpen: false, zero: false, applies: "`transmissionStrength` zero", reach: "mature", dial: { window: [0, 8], step: 1 } },
  { path: "/material/fissureRed", kind: "real", optional: false, meaning: "The tint carried by the floors of the bark's furrows, as an offset per channel, and how far it is laid over the bark colour there.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`fissureStrength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/material/fissureGreen", kind: "real", optional: false, meaning: "The green channel of `fissure_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`fissureStrength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/material/fissureBlue", kind: "real", optional: false, meaning: "The blue channel of `fissure_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`fissureStrength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/material/fissureStrength", kind: "real", optional: false, meaning: "How far `fissure_red`'s tint is laid over the bark colour in the furrows.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/crestRed", kind: "real", optional: false, meaning: "The tint carried by the crests of the bark's ridges, as an offset per channel, and how far it is laid over the bark colour there.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`crestStrength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.025 } },
  { path: "/material/crestGreen", kind: "real", optional: false, meaning: "The green channel of `crest_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`crestStrength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.025 } },
  { path: "/material/crestBlue", kind: "real", optional: false, meaning: "The blue channel of `crest_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`crestStrength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.025 } },
  { path: "/material/crestStrength", kind: "real", optional: false, meaning: "How far `crest_red`'s tint is laid over the bark colour on the crests.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/barkMottleScale", kind: "real", optional: false, meaning: "The size of the blotches in the bark's mottling, and how far they lighten and darken its colour. A scale of zero leaves none.", unit: "-", low: 0, high: 8, lowOpen: false, zero: false, applies: "`barkMottleStrength` zero", reach: "mature", dial: { window: [0, 8], step: 1 } },
  { path: "/material/barkMottleStrength", kind: "real", optional: false, meaning: "How far the blotches `bark_mottle_scale` sizes lighten and darken the bark.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`barkMottleScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/cavityStrength", kind: "real", optional: false, meaning: "How far the hollows of the bark - furrow floors and the sockets where a limb joins - are darkened. Raising it sinks them deeper.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/bladeMottleScale", kind: "real", optional: false, meaning: "The size of the blotches in a leaf's mottling, and how far they vary its colour. A scale of zero leaves none.", unit: "-", low: 0, high: 32, lowOpen: false, zero: false, applies: "`bladeMottleStrength` zero", reach: "mature", dial: { window: [0, 32], step: 2 } },
  { path: "/material/bladeMottleStrength", kind: "real", optional: false, meaning: "How far the blotches `blade_mottle_scale` sizes vary a leaf's colour.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`bladeMottleScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/marginWidth", kind: "real", optional: false, meaning: "How wide a band along the leaf's edge takes the margin colour, and what that colour is as an offset per channel. Raising the width broadens the rim around every leaf.", unit: "-", low: 0, high: 0.5, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 0.5], step: 0.1 } },
  { path: "/material/marginRed", kind: "real", optional: false, meaning: "The margin colour's red channel, as an offset; `margin_width` sets its band.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`marginWidth` zero", reach: "mature", dial: { window: [-1, 1], step: 0.01 } },
  { path: "/material/marginGreen", kind: "real", optional: false, meaning: "The margin colour's green channel, as an offset; `margin_width` sets its band.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`marginWidth` zero", reach: "mature", dial: { window: [-1, 1], step: 0.015 } },
  { path: "/material/marginBlue", kind: "real", optional: false, meaning: "The margin colour's blue channel, as an offset; `margin_width` sets its band.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`marginWidth` zero", reach: "mature", dial: { window: [-1, 1], step: 0.0025 } },
  { path: "/material/cuticleGloss", kind: "real", optional: false, meaning: "How tight the highlight on a leaf's upper face is. Raising it draws the glint into a smaller, glossier spot.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/skyOcclusionStrength", kind: "real", optional: false, meaning: "How much of the sky is withheld from bark and leaves the deeper they sit in the crown. Raising it darkens the crown's interior.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/plateCellScale", kind: "real", optional: false, meaning: "Circumferential size of one bark plate in metres, before girth scales it. Zero leaves the field the ridges it has always been.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`ridgeScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.02 } },
  { path: "/material/plateElongation", kind: "real", optional: false, meaning: "How much longer a plate runs than it is wide: nought is as long as it is wide, one is twice as long.", unit: "-", low: 0, high: 16, lowOpen: false, zero: false, applies: "`plateCellScale` or `ridgeScale` zero", reach: "mature", dial: { window: [0, 16], step: 2.5 } },
  { path: "/material/plateDome", kind: "real", optional: false, meaning: "How far a plate's face rises from its own edge towards its middle. At zero a plate's face lies flat, and any rise starts the doming.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`plateCellScale` or `ridgeScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/plateEdgeLift", kind: "real", optional: false, meaning: "How far a plate's rim stands off the furrow it borders: the scale that lifts rather than the plate that sits flat. At zero a plate's rim lies flush with its furrow, and any rise starts the lift.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`plateCellScale` or `ridgeScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/plateFurrowWidth", kind: "real", optional: false, meaning: "How wide the flat floor of a furrow is cut, as a fraction of a plate's own width, so a bigger plate carries a wider furrow off one row. Zero leaves the hairline the network has always cut between two faces.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`plateCellScale` or `ridgeScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/plateEdgeShape", kind: "real", optional: false, meaning: "Blend from rounded plate edges to narrow chipped scales; independent of lichen/lenticel/peel.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/plateIdentity", kind: "real", optional: false, meaning: "How much of its own a plate keeps: how proud it stands, how it leans, and the value and cast it holds against its neighbours. At zero every plate stands and reads exactly like its neighbours, and any rise starts the difference.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`plateCellScale` or `ridgeScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/weatheringStrength", kind: "real", optional: false, meaning: "How far a weathered face is greyed and tinted against a fresh furrow. At zero no face is weathered at all, and any rise starts the greying.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/weatheringRed", kind: "real", optional: false, meaning: "The tint a weathered face takes, as an offset per channel; how far it is laid on is `weathering_strength` above.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`weatheringStrength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.005 } },
  { path: "/material/weatheringGreen", kind: "real", optional: false, meaning: "The green channel of `weathering_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`weatheringStrength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.01 } },
  { path: "/material/weatheringBlue", kind: "real", optional: false, meaning: "The blue channel of `weathering_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`weatheringStrength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.01 } },
  { path: "/material/orientationStrength", kind: "real", optional: false, meaning: "How far the side away from the sun and the foot of the trunk take a colour of their own - what damp growth would look like, not what it is. At zero the shaded side and the foot take no colour of their own, and any rise starts it.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/orientationRed", kind: "real", optional: false, meaning: "The tint that damp side takes, as an offset per channel; how far it is laid on is `orientation_strength` above.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`orientationStrength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.02 } },
  { path: "/material/orientationGreen", kind: "real", optional: false, meaning: "The green channel of `orientation_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`orientationStrength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.01 } },
  { path: "/material/orientationBlue", kind: "real", optional: false, meaning: "The blue channel of `orientation_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`orientationStrength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.05 } },
  { path: "/material/directionalOcclusion", kind: "real", optional: false, meaning: "How far a furrow floor is darkened by its own crest standing between it and the sun. Zero leaves the sun on both sides of every furrow alike.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/depthStrength", kind: "real", optional: false, meaning: "How far the relief is given depth beyond the shaded normal. At zero the relief is given no depth beyond that normal, and any rise starts it.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/canopyNormal", kind: "real", optional: false, meaning: "How far a leaf is lit as part of its crown rather than as a lone card: its lighting normal bends from the blade's toward the crown's outward direction at its placement, so the sunward shell of the mass is lit whichever way its blades turn. Zero lights the blade alone.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lightWrap", kind: "real", optional: false, meaning: "How far the leaf's sunlight wraps past the terminator, as a fraction of a right angle's cosine; a face square to the sun takes what it always took. Zero is the plain cosine.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/diffuseTransmission", kind: "real", optional: false, meaning: "The share of the blade's transmission that leaves it diffusely, as a thin leaf's does, rather than on the forward lobe toward the sun; the diffuse share also carries the sky through the blade. Zero is the lobe.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/leafSheen", kind: "real", optional: false, meaning: "The cuticle's reflectance of the sky at normal incidence, rising to the whole sky at grazing by Schlick's Fresnel; zero reflects no sky.", unit: "-", low: 0, high: 0.5, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 0.5], step: 0.1 } },
  { path: "/material/crownShade", kind: "real", optional: false, meaning: "How much of the sky one crown radius of leaves takes from a leaf that reads it through the mass - the sky over it, behind it and in its sheen - so the underside of a crown falls into its own shade. Zero sees the sky through the mass.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lichenScale", kind: "real", optional: false, meaning: "Smooth bark's lichen: the size in metres of the cells its patches are scattered over. Zero leaves no patch anywhere.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lichenStrength` zero", reach: "mature", dial: { window: [0, 1], step: 0.01 } },
  { path: "/material/lichenCoverage", kind: "real", optional: false, meaning: "The share of those cells that hold a patch.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lichenStrength` or `lichenScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lichenRed", kind: "real", optional: false, meaning: "A patch's own colour, a linear reflectance like the bark's.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lichenStrength` or `lichenScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lichenGreen", kind: "real", optional: false, meaning: "The green channel of `lichen_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lichenStrength` or `lichenScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lichenBlue", kind: "real", optional: false, meaning: "The blue channel of `lichen_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lichenStrength` or `lichenScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lichenStrength", kind: "real", optional: false, meaning: "How far a patch covers the bark with that colour.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lichenScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lenticelDensity", kind: "real", optional: false, meaning: "Rows of lenticel dashes per metre along the wood.", unit: "-", low: 0, high: 400, lowOpen: false, zero: false, applies: "`lenticelStrength` or `lenticelLength` zero", reach: "mature", dial: { window: [0, 400], step: 5 } },
  { path: "/material/lenticelLength", kind: "real", optional: false, meaning: "The longest dash across the wood, in metres; the shortest is under half.", unit: "m", low: 0, high: 0.5, lowOpen: false, zero: false, applies: "`lenticelStrength` zero", reach: "mature", dial: { window: [0, 0.5], step: 0.1 } },
  { path: "/material/lenticelStrength", kind: "real", optional: false, meaning: "How far a dash shows: its tint, and the shallow groove it cuts.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lenticelLength` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lenticelTint", kind: "real", optional: false, meaning: "A dash's value against the bark it marks: -1 is black, 0 no change.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "`lenticelStrength` or `lenticelLength` zero", reach: "mature", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/material/peelCurl", kind: "real", optional: false, meaning: "How far the plate network's strips curl away: they stretch across the wood into bands, lift at their lower edge, and this share of them has peeled off to show the inner bark. At zero no strip has peeled at all, and any rise switches the peel on.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`ridgeScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/peelRed", kind: "real", optional: false, meaning: "The inner bark a peeled strip leaves showing.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`peelCurl` or `ridgeScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/peelGreen", kind: "real", optional: false, meaning: "The green channel of `peel_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`peelCurl` or `ridgeScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/peelBlue", kind: "real", optional: false, meaning: "The blue channel of `peel_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`peelCurl` or `ridgeScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lobeShade", kind: "real", optional: false, meaning: "How much of the sky and of what passes through the blade a leaf loses to the leaves of its own lobe standing over it, read from the crown's own placements rather than from one smooth ellipsoid: a lobe's face is lit and what hangs under it falls into its shade. Zero sees none of it.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/barkReflectance", kind: "real", optional: false, meaning: "What the bark mirrors of the sun at normal incidence, the foot of its one highlight by Schlick's Fresnel: 0.04 is a dielectric. What the highlight mirrors is taken from the diffuse; zero mirrors none.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/leafReflectance", kind: "real", optional: false, meaning: "The same for the cuticle on the blade's front face; the highlight's width follows `cuticle_gloss`.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/barkGrainScale", kind: "real", optional: false, meaning: "A grain below the relief: the size in metres of its cells. Zero leaves the bark smooth between the relief's features.", unit: "m", low: 0, high: 0.05, lowOpen: false, zero: false, applies: "`barkGrainStrength` zero", reach: "mature", dial: { window: [0, 0.05], step: 0.0005 } },
  { path: "/material/barkGrainStrength", kind: "real", optional: false, meaning: "How far the grain varies the bark's colour and tilts its normal.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`barkGrainScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/bladeGrainScale", kind: "real", optional: false, meaning: "The blade's cell grain, in cells per blade length; zero is none.", unit: "-", low: 0, high: 256, lowOpen: false, zero: false, applies: "`bladeGrainStrength` zero", reach: "mature", dial: { window: [0, 256], step: 20 } },
  { path: "/material/bladeGrainStrength", kind: "real", optional: false, meaning: "How far that grain varies the blade's colour and tilts its normal.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`bladeGrainScale` zero", reach: "mature", dial: { window: [0, 1], step: 0.15 } },
];
