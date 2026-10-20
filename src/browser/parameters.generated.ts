// Generated from the parameter catalogue (crates/telperion-core/src/catalogue.rs)
// by `the_browser_reads_the_catalogue` in crates/telperion-core/tests/parameter_reference.rs;
// run it with TELPERION_WRITE_REFERENCE=1 to rewrite this file. Do not edit.

/** A family: every wire row, under the meaning the catalogue gives it. */
export interface Family {
  /** The specimen's age in years: the engine grows the family's species for this many years, so the tree is the one it is at that age (fn-198). A whole number of years. */
  age: number;
  /** How deep into the crown leaves were kept, as a share of the crown's widest radius. Checked and read by nothing past the check: the cull it set kept every leaf on every engine preset and was removed (fn-198 R1, fn-212 R2); whether the row goes is the owner's call. */
  shellDepth: number;
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
      /** Degrees the lateral pitch moves from the crown's base to its top: a station's laterals leave at `lateralPitch` plus as much of this as the station stands up the crown, none at its base and all at its top. Negative sets the upper limbs more upright than the lower; zero is one pitch everywhere. */
      pitchByHeight: number;
      /** The most a first-order axis may stop short of the shell, as a share of its room; each axis draws its own share from its own stream, and what it bears grows within the shell scaled about its station by the share it kept. Zero reaches the shell. */
      raggedReach: number;
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
      /** The chance, 0 to 1, that a structural axis forks codominantly: each axis draws once from its own stream when it is born. At zero no axis forks and the tree stands on one stem. */
      codominance: number;
      /** Where a codominant fork falls, as a share of the tree's height: the centre of the bell each axis draws its fork height from. Zero is a fork at the root, which is a clump. */
      forkHeight: number;
      /** Width of the bell a fork height is drawn from, as a share of the tree's height. Zero forks every axis that forks at exactly `forkHeight`. */
      forkHeightSpread: number;
      /** The parts a codominant fork divides into, the primary among them, 2 to 4. The whole part is the full parts; the fraction grows one more in from the fork, its length and its wood that share of a whole part's. */
      forkWays: number;
      /** Degrees of bearing between neighbouring slots of a fork, slot `k` standing `k` times this round from the primary's, about a bearing the axis's own stream decides. At zero every part leaves on one bearing. */
      forkDivergence: number;
      /** Degrees every sibling of a fork leans from the axis it leaves; the primary leans by what `forkLeanSpread` leaves it of this. */
      forkLean: number;
      /** How far a fork's primary stands back along its axis, 0 to 1. None of it leans the primary by all of `forkLean`, as every sibling leans; all of it carries the axis straight on through the fork. */
      forkLeanSpread: number;
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
        /** The leaf box's reach where foliage sits on twigs, as the thickness of the wood a leaf station is seated on, in metres; the engine sizes the grown wood itself (fn-198). */
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
      /** The nodes a tree may grow; unset, the shipped default. A tree that would grow more gives up its finest twigs evenly and finishes every branch, so raising it restores fine detail only to a tree that reached it. */
      maxNodes?: number;
      /** The most a growing shoot may turn in one step, in degrees; unset, 35. At 180 or more nothing is limited. */
      maxTurnPerStep?: number;
    };
  };
  radii: {
    /** The leaf box's reach where foliage sits on slender wood, as a trunk radius at the ground that is a share of the tree's height; the engine sizes the grown wood itself (fn-198). */
    trunkRadius: number;
    /** How wood divides at a fork. The parent's area is the sum of the children's radii raised to this power, so raising it leaves the children thicker for the same parent. */
    forkExponent: number;
    /** How fast wood thins along its own length. Raising it makes a branch narrow more sharply from its base to its tip. */
    lengthTaper: number;
    /** The ceiling on accumulated taper, so no single long branch can thin away to nothing. Raising it lets long branches taper further. */
    maxTaperExponent: number;
    /** The wood a lateral takes at a fork against what its own subtree asks, 0 to 1: the parent carries a lateral's pipe at this share and the lateral's wood thins by its root. At one every fork divides by the pipe model alone. */
    lateralShare: number;
    /** The wood a codominant sibling takes at a fork against its own subtree's, 0 to 1, beside the primary that carries the axis on. At one the parts divide by the pipe model alone. */
    forkBalance: number;
    /** The share of each structural axis's reach over which it holds the girth it starts with, 0 to 0.9: past it the wood falls to the pipe model's radius by the axis's tip. At zero wood is the pipe model's. */
    girthHold: number;
    /** How short the fall after `girthHold` is: the fall lasts the hold's share divided by this, so raising it breaks the wood into twigs over a shorter distance. */
    girthFall: number;
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
    /** How far into each limb system the gap between it and its neighbours reaches, as a share of the way from their shared boundary to the system's centre: each limb system then keeps a rounded leaf mass of its own. Zero, the neutral, thins nothing. */
    limbClumping: number;
    /** How deep a lateral may be and still start a limb system of its own. Raising it parts the crown into more and smaller leaf masses; it does nothing until `limb_clumping` is above zero. */
    clumpSystemOrder: number;
    /** Nearest neighbours and cell crossings in the clumping approximation. */
    clumpNeighbours: number;
    /** Fronds the rosette bears at the apex of each stem. At zero no rosette stands. A fraction is split by a keyed draw at each apex, which bears the whole count either side of it (fn-277). */
    rosetteFronds: number;
    /** The share of leaf-bearing wood that bears its leaves along it, cauline foliage as against a crown's alone: each run of leaf-bearing wood and each short shoot keeps its leaves by a keyed draw at this share. One, every one bears; at zero none does, as a palm bears its leaves only in its crown (fn-277). */
    cauline: number;
    /** The degrees each successive frond is turned about the apex. */
    rosetteDivergence: number;
    /** Degrees from the axis the youngest frond stands: 0 upright, 90 level, 180 hanging. */
    rosettePitch: number;
    /** How many degrees further than the youngest the oldest frond leans, so the crown opens from a spike to a skirt. */
    rosettePitchSpread: number;
    /** Metres below the apex the frond insertions are spread down the axis. At zero every frond leaves one point. */
    rosetteDepth: number;
    /** Leaflets one placement carries along its rachis. One is the single blade every family drew. A fraction is split by a keyed draw at each placement (fn-277). */
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
    /** The years a living frond lives (fn-270 R5b): a rosette makes its living crown's count over that many years, its fronds aged by their place on the spiral, this year's emerging over its stem's elongation and unfolding by its expansion, the oldest passing into the skirt and out of it as new ones emerge above. At zero the crown makes none and stands as it is all year. */
    frondLife: number;
    /** The years a rosette's crown takes to make a mature frond (fn-275 R2, R2a): a frond is made at a share of its size set by the crown's age when it was made, along the logistic course from `frondEarlyShare` to whole over these years, and keeps it for its life; the stem the rosette crowns widens by the same course and waits at the ground until the crown is established. At zero every frond is made whole and the stem rises from its first year. */
    frondEstablishment: number;
    /** The share of a mature frond's size a crown's first frond is made at while it establishes, and of its stem's girth where the stem was first laid (fn-275 R2, R2a). */
    frondEarlyShare: number;
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
    /** The bark's own colour, a linear reflectance per channel, not a display (sRGB) value; every colour row is on this one scale (see the reference's header). Shipped barks run 0.017 (the beech's blue) to 0.78 (the birch's white). Raising a channel pushes mature bark toward that colour. */
    barkRed: number;
    /** The green channel of `bark_red`. */
    barkGreen: number;
    /** The blue channel of `bark_red`. */
    barkBlue: number;
    /** How diffuse the bark is: 0 is a mirror, 1 is chalk. */
    barkRoughness: number;
    /** The young wood's own colour, a linear reflectance per channel on the bark's scale, before its bark has formed. Wood thinner than `shoot_radius` takes it, and gives it up to the bark colour on a smoothstep of its radius by twice that; zero means no wood is young. */
    shootRed: number;
    /** The green channel of `shoot_red`. */
    shootGreen: number;
    /** The blue channel of `shoot_red`. */
    shootBlue: number;
    /** Metres. The radius below which wood is young. */
    shootRadius: number;
    /** The old bark's own colour, the one thick wood grows into. Wood takes it over the bark colour on a smoothstep of its radius from `mature_radius` to that plus `mature_span`. */
    matureRed: number;
    /** The green channel of `mature_red`. */
    matureGreen: number;
    /** The blue channel of `mature_red`. */
    matureBlue: number;
    /** The furrow strength old bark takes in place of `furrow_strength`, over the same radius range as its colour: how deep it fissures. */
    matureFurrowStrength: number;
    /** Metres. The radius at which bark starts to turn old; zero means no wood is old. */
    matureRadius: number;
    /** Metres. How much further the radius grows before the bark is wholly old; zero turns it at once. */
    matureSpan: number;
    /** The colour of a leaf's upper face, a linear reflectance per channel, not a display (sRGB) value. A dark broadleaf face is a few hundredths (the beech 0.021/0.016/0.007, the oak 0.021/0.048/0.021); shipped faces run 0.007 to 0.19. Raising a channel pushes the sunlit face toward it. */
    leafFrontRed: number;
    /** The green channel of `leaf_front_red`. */
    leafFrontGreen: number;
    /** The blue channel of `leaf_front_red`. */
    leafFrontBlue: number;
    /** The colour of a leaf's underside, a linear reflectance per channel on the upper face's scale (shipped: 0.011 to 0.28), shown wherever the eye sees the back of a blade. Raising a channel pushes that face toward it. */
    leafBackRed: number;
    /** The green channel of `leaf_back_red`. */
    leafBackGreen: number;
    /** The blue channel of `leaf_back_red`. */
    leafBackBlue: number;
    /** The colour a leaf turns as it dies, a linear reflectance per channel on the leaf faces' scale, both faces alike: the dead fronds a rosette keeps below its living crown, and a leaf colouring in autumn before it falls (fn-270 R5c), which a tree drawn within its year moves toward. Raising a channel pushes either toward it. */
    leafDeadRed: number;
    /** The green channel of `leaf_dead_red`. */
    leafDeadGreen: number;
    /** The blue channel of `leaf_dead_red`. */
    leafDeadBlue: number;
    /** The colour of a petal's upper face, a linear albedo per channel, the same quantity as the leaf faces' and not a display (sRGB) value (fn-216, fn-234): the flowers' own, drawn by the leaf shader, and the colour the petal passes when lit from behind. A white petal is about 0.8 a channel and a gold one about 0.85/0.65/0.1. */
    bloomFrontRed: number;
    /** The green channel of `bloom_front_red`. */
    bloomFrontGreen: number;
    /** The blue channel of `bloom_front_red`. */
    bloomFrontBlue: number;
    /** The colour of a petal's underside, a linear albedo per channel as `bloom_front_red` is (fn-216): the flowers' own, drawn by the leaf shader. */
    bloomBackRed: number;
    /** The green channel of `bloom_back_red`. */
    bloomBackGreen: number;
    /** The blue channel of `bloom_back_red`. */
    bloomBackBlue: number;
    /** A floret's eye: the colour its petals turn toward at their base, a linear reflectance on the petals' scale, and the stamens' colour. */
    bloomEyeRed: number;
    /** The green channel of `bloom_eye_red`. */
    bloomEyeGreen: number;
    /** The blue channel of `bloom_eye_red`. */
    bloomEyeBlue: number;
    /** How far up each petal from its base the eye reaches, as a share of the petal's length. Zero is no eye: the petal and its stamens are the petal's colour. */
    bloomEyeShare: number;
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
    /** How wide a band along the leaf's edge takes the margin colour, as a share of the leaf set in leaf space, and what that colour is as an offset per channel. Raising the width broadens the rim around every leaf; a leaf under a pixel takes the band's area share. */
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
    /** The inner bark a peeled strip leaves showing, a linear reflectance like the bark's. */
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
  bloom: {
    /** Florets on one inflorescence. The fractional part is the last floret's size, so a walk across counts grows one in by degree. */
    florets: number;
    /** Metres of the inflorescence's axis the florets stand along: zero is a cluster at the node, a few centimetres a raceme or catkin. */
    rachis: number;
    /** How the axis turns from the bud's side as it grows: 1 hangs straight down, -1 stands up as a candle, 0 holds the side. */
    droop: number;
    /** Metres of stalk between the axis and each floret. */
    pedicel: number;
    /** The degrees a floret's stalk leaves the axis: small packs a spike, ninety spreads a raceme, more fans a cluster back. */
    spread: number;
    /** How much smaller the floret at the axis's tip is than the one at its base, as a share. */
    taper: number;
    /** Petals on a floret, turned evenly about it by the count. The fractional part is the last petal's size (host decision 4), so a walk across counts changes a flower by degree. */
    petals: number;
    /** The degrees a petal opens from the floret's axis: a bud near zero, an open blossom near ninety. */
    open: number;
    /** A petal's length in metres. */
    petalLength: number;
    /** A petal's greatest width in metres. */
    petalWidth: number;
    /** Where a petal is widest, as a share from its base to its tip. */
    petalWidest: number;
    /** How far a petal's margins lift out of its plane, as a share of its half-width: a cupped blossom. */
    petalCup: number;
    /** How far a petal bends along its length, as a share of its length at the tip: positive turns the tip out, away from the floret's axis, so a narrow-opened floret flares into a horn's mouth. */
    petalCurl: number;
    /** Stamens in a floret, a ring of filaments standing up inside its petals. The fractional part is the last stamen's size, so a walk across counts grows one in by degree; zero is none. */
    stamens: number;
    /** A stamen's length in metres, from the floret's centre. */
    stamenLength: number;
    /** A stamen's width in metres. */
    stamenWidth: number;
    /** How far a floret's size varies from one to the next, as a share. */
    scatter: number;
    /** Where in the year the flowering window opens, as a share of the year from its start. */
    opens: number;
    /** How long the window lasts, as a share of the year. A floret grows in and shrinks out inside it; zero flowers never. */
    lasts: number;
    /** The share of the window the florets' openings are spread over: zero opens every floret at once. */
    stagger: number;
  };
  supernatural: {
    /** Whether the legendary traits act. Off, the group reads as none: the scale is one, the reflectances zero and every override leaves its natural row as it is. */
    enabled: boolean;
    /** How many times its natural size the tree is drawn: every position and length of the grown wood, after the engine grew it at its natural size, so its topology is the natural tree's. Leaves and flowers keep their own size; the leaves keep their spacing along the wood. One is the natural tree. */
    scale: number;
    /** What a leaf's underside mirrors at normal incidence, as a metal does: the sun's glint and the sky's sheen off the back of the blade, tinted by the back tint, taken from the face's diffuse. Zero mirrors nothing and leaves the back as the natural leaf's. */
    backReflectance: number;
    /** The colour the underside mirrors in, a linear reflectance per channel: a metal's reflection is its colour. Silver is near white. */
    backRed: number;
    /** The green channel of `back_red`. */
    backGreen: number;
    /** The blue channel of `back_red`. */
    backBlue: number;
    /** What a leaf's margin band mirrors at normal incidence, on either face, tinted by the margin's colour: a metallic rim, taken from the band's diffuse. Zero mirrors nothing. */
    marginReflectance: number;
    /** How rough the back and the margin are as mirrors: low is a sharp glint, high a broad sheen. */
    roughness: number;
    material: {
      /** `/material/leafFrontRed` while the group is on; unset, the natural row. */
      leafFrontRed?: number;
      /** `/material/leafFrontGreen` while the group is on; unset, the natural row. */
      leafFrontGreen?: number;
      /** `/material/leafFrontBlue` while the group is on; unset, the natural row. */
      leafFrontBlue?: number;
      /** `/material/leafBackRed` while the group is on; unset, the natural row. */
      leafBackRed?: number;
      /** `/material/leafBackGreen` while the group is on; unset, the natural row. */
      leafBackGreen?: number;
      /** `/material/leafBackBlue` while the group is on; unset, the natural row. */
      leafBackBlue?: number;
      /** `/material/marginRed` while the group is on; unset, the natural row. */
      marginRed?: number;
      /** `/material/marginGreen` while the group is on; unset, the natural row. */
      marginGreen?: number;
      /** `/material/marginBlue` while the group is on; unset, the natural row. */
      marginBlue?: number;
    };
    bloom: {
      /** `/bloom/florets` while the group is on; unset, the natural row. */
      florets?: number;
      /** `/bloom/rachis` while the group is on; unset, the natural row. */
      rachis?: number;
      /** `/bloom/droop` while the group is on; unset, the natural row. */
      droop?: number;
      /** `/bloom/pedicel` while the group is on; unset, the natural row. */
      pedicel?: number;
      /** `/bloom/spread` while the group is on; unset, the natural row. */
      spread?: number;
      /** `/bloom/taper` while the group is on; unset, the natural row. */
      taper?: number;
      /** `/bloom/petals` while the group is on; unset, the natural row. */
      petals?: number;
      /** `/bloom/open` while the group is on; unset, the natural row. */
      open?: number;
      /** `/bloom/petalLength` while the group is on; unset, the natural row. */
      petalLength?: number;
      /** `/bloom/petalWidth` while the group is on; unset, the natural row. */
      petalWidth?: number;
      /** `/bloom/petalWidest` while the group is on; unset, the natural row. */
      petalWidest?: number;
      /** `/bloom/petalCup` while the group is on; unset, the natural row. */
      petalCup?: number;
      /** `/bloom/petalCurl` while the group is on; unset, the natural row. */
      petalCurl?: number;
      /** `/bloom/stamens` while the group is on; unset, the natural row. */
      stamens?: number;
      /** `/bloom/stamenLength` while the group is on; unset, the natural row. */
      stamenLength?: number;
      /** `/bloom/stamenWidth` while the group is on; unset, the natural row. */
      stamenWidth?: number;
      /** `/bloom/scatter` while the group is on; unset, the natural row. */
      scatter?: number;
      /** `/bloom/opens` while the group is on; unset, the natural row. */
      opens?: number;
      /** `/bloom/lasts` while the group is on; unset, the natural row. */
      lasts?: number;
      /** `/bloom/stagger` while the group is on; unset, the natural row. */
      stagger?: number;
    };
    canopy: {
      /** `/canopy/size` while the group is on; unset, the natural row. */
      size?: number;
    };
    flowering: {
      /** `/space/ages/0/flowering` while the group is on; unset, the natural share. */
      age0?: number;
      /** `/space/ages/1/flowering` while the group is on; unset, the natural share. */
      age1?: number;
      /** `/space/ages/2/flowering` while the group is on; unset, the natural share. */
      age2?: number;
      /** `/space/ages/3/flowering` while the group is on; unset, the natural share. */
      age3?: number;
      /** `/space/ages/4/flowering` while the group is on; unset, the natural share. */
      age4?: number;
      /** `/space/ages/5/flowering` while the group is on; unset, the natural share. */
      age5?: number;
      /** `/space/ages/6/flowering` while the group is on; unset, the natural share. */
      age6?: number;
      /** `/space/ages/7/flowering` while the group is on; unset, the natural share. */
      age7?: number;
      /** `/space/ages/8/flowering` while the group is on; unset, the natural share. */
      age8?: number;
      /** `/space/ages/9/flowering` while the group is on; unset, the natural share. */
      age9?: number;
      /** `/space/ages/10/flowering` while the group is on; unset, the natural share. */
      age10?: number;
      /** `/space/ages/11/flowering` while the group is on; unset, the natural share. */
      age11?: number;
      /** `/space/ages/12/flowering` while the group is on; unset, the natural share. */
      age12?: number;
      /** `/space/ages/13/flowering` while the group is on; unset, the natural share. */
      age13?: number;
      /** `/space/ages/14/flowering` while the group is on; unset, the natural share. */
      age14?: number;
    };
    internode: {
      /** `/space/ages/0/internode` while the group is on; unset, the natural internode length. */
      age0?: number;
      /** `/space/ages/1/internode` while the group is on; unset, the natural internode length. */
      age1?: number;
      /** `/space/ages/2/internode` while the group is on; unset, the natural internode length. */
      age2?: number;
      /** `/space/ages/3/internode` while the group is on; unset, the natural internode length. */
      age3?: number;
      /** `/space/ages/4/internode` while the group is on; unset, the natural internode length. */
      age4?: number;
      /** `/space/ages/5/internode` while the group is on; unset, the natural internode length. */
      age5?: number;
      /** `/space/ages/6/internode` while the group is on; unset, the natural internode length. */
      age6?: number;
      /** `/space/ages/7/internode` while the group is on; unset, the natural internode length. */
      age7?: number;
      /** `/space/ages/8/internode` while the group is on; unset, the natural internode length. */
      age8?: number;
      /** `/space/ages/9/internode` while the group is on; unset, the natural internode length. */
      age9?: number;
      /** `/space/ages/10/internode` while the group is on; unset, the natural internode length. */
      age10?: number;
      /** `/space/ages/11/internode` while the group is on; unset, the natural internode length. */
      age11?: number;
      /** `/space/ages/12/internode` while the group is on; unset, the natural internode length. */
      age12?: number;
      /** `/space/ages/13/internode` while the group is on; unset, the natural internode length. */
      age13?: number;
      /** `/space/ages/14/internode` while the group is on; unset, the natural internode length. */
      age14?: number;
    };
    pipe: {
      /** `/space/ages/0/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age0?: number;
      /** `/space/ages/1/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age1?: number;
      /** `/space/ages/2/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age2?: number;
      /** `/space/ages/3/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age3?: number;
      /** `/space/ages/4/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age4?: number;
      /** `/space/ages/5/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age5?: number;
      /** `/space/ages/6/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age6?: number;
      /** `/space/ages/7/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age7?: number;
      /** `/space/ages/8/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age8?: number;
      /** `/space/ages/9/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age9?: number;
      /** `/space/ages/10/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age10?: number;
      /** `/space/ages/11/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age11?: number;
      /** `/space/ages/12/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age12?: number;
      /** `/space/ages/13/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age13?: number;
      /** `/space/ages/14/form/pipe` while the group is on; unset, the natural pipe a phytomer adds. */
      age14?: number;
    };
  };
}

/** The engine species: the indexed `/space` rows (fn-198); absent on a family with none. */
export interface Family {
  space?: SpaceRows;
}
/** An age's or a zone's rows by name or index; `never` is an unbounded delay. */
export type SpaceRows = { [key: string]: SpaceRows | number | boolean | "never" };

/** What a control does to the tree; the panel's groups. */
export type Group = "size" | "form" | "branching" | "leaves" | "flowers" | "bark" | "supernatural" | "advanced";

export const GROUPS: readonly { id: Group; title: string }[] = [
  { id: "size", title: "size and age" },
  { id: "form", title: "trunk and form" },
  { id: "branching", title: "branching" },
  { id: "leaves", title: "leaves and fronds" },
  { id: "flowers", title: "flowers and fruit" },
  { id: "bark", title: "bark and colour" },
  { id: "supernatural", title: "supernatural" },
  { id: "advanced", title: "advanced" },
];

/** One catalogue row, as a control reads it. */
export interface Parameter {
  /** The row's JSON pointer on the wire. */
  path: string;
  /** Its label: the key, spaced, unless the catalogue words it. */
  label: string;
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
  /** Who reads it, from the stages the catalogue names: the direct build, only the removed growth path, or no production stage. */
  reach: "mature" | "growth" | "deprecated";
  group: Group;
  /** The tuning dial's window and small step, where the row offers one. */
  dial?: { window: [number, number]; step: number };
}

export const PARAMETERS: readonly Parameter[] = [
  { path: "/age", label: "age", kind: "real", optional: false, meaning: "The specimen's age in years: the engine grows the family's species for this many years, so the tree is the one it is at that age (fn-198). A whole number of years.", unit: "years", low: 0, high: 1000000, lowOpen: false, zero: false, applies: "", reach: "mature", group: "size" },
  { path: "/shellDepth", label: "shell depth", kind: "real", optional: false, meaning: "How deep into the crown leaves were kept, as a share of the crown's widest radius. Checked and read by nothing past the check: the cull it set kept every leaf on every engine preset and was removed (fn-198 R1, fn-212 R2); whether the row goes is the owner's call.", unit: "share of the crown's widest radius", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/seed", label: "seed", kind: "count", optional: false, meaning: "The specimen: every stage keys its random stream by it, so another seed draws another tree of the same family.", unit: "-", low: 0, high: 4294967295, lowOpen: false, zero: false, applies: "", reach: "mature", group: "advanced" },
  { path: "/skeleton/attractors", label: "attractors", kind: "count", optional: false, meaning: "How many pull points are scattered through the crown for the branches to grow toward. Raising it fills the crown with more and finer branching; at an `attractor_weight` of zero none are scattered and the row does nothing.", unit: "points", low: 0, high: 1000000, lowOpen: false, zero: false, applies: "`attractorWeight` zero", reach: "deprecated", group: "advanced", dial: { window: [1, 1000000], step: 100 } },
  { path: "/skeleton/samplingAttemptsPerAttractor", label: "sampling attempts per attractor", kind: "count", optional: false, meaning: "How many random tries the sampler may spend on each pull point before it gives up. Raising it lets a narrow or deeply lobed crown reach its full count of points instead of settling for fewer.", unit: "tries", low: 1, high: 4294967295, lowOpen: false, zero: false, applies: "`attractorWeight` zero", reach: "deprecated", group: "advanced" },
  { path: "/skeleton/step", label: "step", kind: "real", optional: false, meaning: "How far the crown grows in one step, as a share of the tree's height; the distance at which a pull point is used up is twice it. Raising it grows the crown in longer, coarser strides.", unit: "share of height", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0.011, 0.033], step: 0.0025 } },
  { path: "/skeleton/habit/reachProbeSteps", label: "reach probe steps", kind: "count", optional: false, meaning: "Samples available to find an axis's room within the crown.", unit: "samples", low: 1, high: 4294967295, lowOpen: false, zero: false, applies: "needs `lateralOrders` of one or more: only first-order laterals probe their room", reach: "deprecated", group: "advanced" },
  { path: "/skeleton/habit/apicalDominance", label: "apical dominance", kind: "real", optional: false, meaning: "How far the leader persists into the crown, 0 to 1.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/habit/whorlStrength", label: "whorl strength", kind: "real", optional: false, meaning: "Clustering of laterals at a station against scattering along the axis.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "needs `lateralOrders` of one or more", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/habit/leaderInternode", label: "leader internode", kind: "real", optional: false, meaning: "Spacing between lateral stations on the leader, in metres.", unit: "m", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0.05, 3.45], step: 0.5 } },
  { path: "/skeleton/habit/lateralsPerStation", label: "laterals per station", kind: "count", optional: false, meaning: "Laterals borne by one station of the leader.", unit: "laterals", low: 1, high: 12, lowOpen: false, zero: false, applies: "needs `lateralOrders` of one or more; leader stations only", reach: "deprecated", group: "advanced", dial: { window: [1, 4], step: 1 } },
  { path: "/skeleton/habit/lateralPitch", label: "lateral pitch", kind: "real", optional: false, meaning: "Initial angle of a lateral from its parent axis, in degrees; on the upright leader this is degrees from vertical.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "needs `lateralOrders` of one or more", reach: "deprecated", group: "advanced", dial: { window: [0, 180], step: 20 } },
  { path: "/skeleton/habit/pitchVariation", label: "pitch variation", kind: "real", optional: false, meaning: "Spread of the lateral pitch, in degrees.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "needs `lateralOrders` of one or more", reach: "deprecated", group: "advanced", dial: { window: [0, 90], step: 5 } },
  { path: "/skeleton/habit/pitchByHeight", label: "pitch by height", kind: "real", optional: false, meaning: "Degrees the lateral pitch moves from the crown's base to its top: a station's laterals leave at `lateralPitch` plus as much of this as the station stands up the crown, none at its base and all at its top. Negative sets the upper limbs more upright than the lower; zero is one pitch everywhere.", unit: "degrees", low: -180, high: 180, lowOpen: false, zero: false, applies: "needs `lateralOrders` of one or more", reach: "deprecated", group: "advanced", dial: { window: [-180, 180], step: 5 } },
  { path: "/skeleton/habit/raggedReach", label: "ragged reach", kind: "real", optional: false, meaning: "The most a first-order axis may stop short of the shell, as a share of its room; each axis draws its own share from its own stream, and what it bears grows within the shell scaled about its station by the share it kept. Zero reaches the shell.", unit: "share of the room", low: 0, high: 1, lowOpen: false, zero: false, applies: "needs `lateralOrders` of one or more: only first-order laterals probe their room", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.05 } },
  { path: "/skeleton/habit/risePrimary", label: "rise primary", kind: "real", optional: false, meaning: "Signed bend over the length of a first-order axis; positive rises.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [-1, 1], step: 0.025 } },
  { path: "/skeleton/habit/riseSecondary", label: "rise secondary", kind: "real", optional: false, meaning: "Signed bend over the length of a deeper axis; negative hangs.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "needs `lateralOrders` of two or more", reach: "deprecated", group: "advanced", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/skeleton/habit/crookedness", label: "crookedness", kind: "real", optional: false, meaning: "Heading change between successive growth units, in degrees.", unit: "degrees", low: 0, high: 60, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 15], step: 3 } },
  { path: "/skeleton/habit/lateralSpacing", label: "lateral spacing", kind: "real", optional: false, meaning: "Spacing between lateral stations away from the leader, in metres.", unit: "m", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "needs `lateralOrders` of one or more", reach: "deprecated", group: "advanced", dial: { window: [0.000001, 2.925], step: 0.5 } },
  { path: "/skeleton/habit/lateralLengthRatio", label: "lateral length ratio", kind: "real", optional: false, meaning: "Length of a lateral against its supporting axis.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "needs `lateralOrders` of two or more: first-order laterals take their reach from the probe", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/habit/lateralOrders", label: "lateral orders", kind: "count", optional: false, meaning: "Depth of rule-built orders below the leader.", unit: "orders", low: 0, high: 8, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 8], step: 1 } },
  { path: "/skeleton/habit/attractorWeight", label: "attractor weight", kind: "real", optional: false, meaning: "Weight of the attractor pull beside the axis's own rule heading. At zero no pull points are scattered and every axis holds its own rule heading, and any rise switches the pull on.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/habit/twigTipTaper", label: "twig tip taper", kind: "real", optional: false, meaning: "Distal twig radius against the nominal twig radius.", unit: "share", low: 0, high: 1, lowOpen: true, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0.05, 0.6], step: 0.1 } },
  { path: "/skeleton/habit/sheddingThreshold", label: "shedding threshold", kind: "real", optional: false, meaning: "Monthly vigour threshold; zero disables shedding. The legacy envelope builder interprets it as shell depth.", unit: "share of height·spread", low: 0, high: 1, lowOpen: false, zero: false, applies: "zero sheds nothing", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/habit/codominance", label: "codominance", kind: "real", optional: false, meaning: "The chance, 0 to 1, that a structural axis forks codominantly: each axis draws once from its own stream when it is born. At zero no axis forks and the tree stands on one stem.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.1 } },
  { path: "/skeleton/habit/forkHeight", label: "fork height", kind: "real", optional: false, meaning: "Where a codominant fork falls, as a share of the tree's height: the centre of the bell each axis draws its fork height from. Zero is a fork at the root, which is a clump.", unit: "share of height", low: 0, high: 1, lowOpen: false, zero: false, applies: "`codominance` zero", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.05 } },
  { path: "/skeleton/habit/forkHeightSpread", label: "fork height spread", kind: "real", optional: false, meaning: "Width of the bell a fork height is drawn from, as a share of the tree's height. Zero forks every axis that forks at exactly `forkHeight`.", unit: "share of height", low: 0, high: 1, lowOpen: false, zero: false, applies: "`codominance` zero", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.05 } },
  { path: "/skeleton/habit/forkWays", label: "fork ways", kind: "real", optional: false, meaning: "The parts a codominant fork divides into, the primary among them, 2 to 4. The whole part is the full parts; the fraction grows one more in from the fork, its length and its wood that share of a whole part's.", unit: "parts", low: 2, high: 4, lowOpen: false, zero: false, applies: "`codominance` zero", reach: "deprecated", group: "advanced", dial: { window: [2, 4], step: 0.5 } },
  { path: "/skeleton/habit/forkDivergence", label: "fork divergence", kind: "real", optional: false, meaning: "Degrees of bearing between neighbouring slots of a fork, slot `k` standing `k` times this round from the primary's, about a bearing the axis's own stream decides. At zero every part leaves on one bearing.", unit: "degrees", low: 0, high: 120, lowOpen: false, zero: false, applies: "`codominance` zero", reach: "deprecated", group: "advanced", dial: { window: [0, 120], step: 5 } },
  { path: "/skeleton/habit/forkLean", label: "fork lean", kind: "real", optional: false, meaning: "Degrees every sibling of a fork leans from the axis it leaves; the primary leans by what `forkLeanSpread` leaves it of this.", unit: "degrees", low: 0, high: 45, lowOpen: false, zero: false, applies: "`codominance` zero", reach: "deprecated", group: "advanced", dial: { window: [0, 45], step: 5 } },
  { path: "/skeleton/habit/forkLeanSpread", label: "fork lean spread", kind: "real", optional: false, meaning: "How far a fork's primary stands back along its axis, 0 to 1. None of it leans the primary by all of `forkLean`, as every sibling leans; all of it carries the axis straight on through the fork.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "`codominance` zero, or `forkLean` zero", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/envelope/height", label: "height", kind: "real", optional: false, meaning: "The tree's height in metres. Everything else in the crown is measured against it: the crown base sits at `crown_base` of it and the widest radius is `spread` times it.", unit: "m", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [6.5, 40.5], step: 5 } },
  { path: "/skeleton/envelope/crownBase", label: "crown base", kind: "real", optional: false, meaning: "Where the crown starts, as a share of the height: below it the crown has no radius at all. Raising it lifts the crown and leaves a longer bare trunk.", unit: "share of height", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/envelope/spread", label: "spread", kind: "real", optional: false, meaning: "The crown's widest radius as a share of the height, so raising it widens the crown without making the tree taller.", unit: "share of height", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 0.675], step: 0.1 } },
  { path: "/skeleton/envelope/fullness", label: "fullness", kind: "real", optional: false, meaning: "Where the crown is widest, as a share of the way from the crown base to the top. Raising it carries the widest part higher, so the crown reads top-heavy.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/envelope/shoulder", label: "shoulder", kind: "real", optional: false, meaning: "How square the crown's outline is. Raising it holds the crown near its full width further toward the top and the base, so the profile reads boxier; lowering it tapers the outline to a point.", unit: "-", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0.4, 2.8], step: 0.5 } },
  { path: "/skeleton/envelope/irregularity", label: "irregularity", kind: "real", optional: false, meaning: "How far the outline departs from the smooth shell, as a fraction of the radius there. 0 is the axisymmetric superellipse every tree was before, and every shipped table that leaves it there is untouched.", unit: "share of radius", low: 0, high: 0.5, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 0.5], step: 0.08 } },
  { path: "/skeleton/envelope/lobeScale", label: "lobe scale", kind: "real", optional: false, meaning: "The wavelength of that departure over the shell's own surface, as a fraction of the tree's height: small is many small lumps, 1 is a lobe as long as the tree is tall.", unit: "share of height", low: 0.05, high: 1, lowOpen: false, zero: false, applies: "`irregularity` zero, except that the curtain's drop search steps by it", reach: "deprecated", group: "advanced", dial: { window: [0.05, 1], step: 0.15 } },
  { path: "/skeleton/bias/gravitropism", label: "gravitropism", kind: "real", optional: false, meaning: "How strongly growth is pulled upward, most near the ground. Raising it makes the tree grow more erect. At zero nothing pulls growth upright, and any rise starts that pull.", unit: "-", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 1.05], step: 0.15 } },
  { path: "/skeleton/bias/lean", label: "lean", kind: "real", optional: false, meaning: "How far the whole tree leans off vertical, increasing with height. Raising it tips the trunk further in one direction. At zero the tree stands plumb, and any rise starts the lean.", unit: "-", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 0.075], step: 0.01 } },
  { path: "/skeleton/bias/supernatural/enabled", label: "enabled", kind: "switch", optional: false, meaning: "Whether the supernatural field bends the wood. Off, the bias reads the amplitude, wavelength and spiral as zero.", unit: "switch", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced" },
  { path: "/skeleton/bias/supernatural/writheAmplitude", label: "writhe amplitude", kind: "real", optional: false, meaning: "How far a branch may wander from a straight course, as a share of the tree's height. Raising it makes the wood wind and stray more. At zero the wood holds a straight course and the spiral rate does nothing, and any rise starts the wander.", unit: "share of height", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "`enabled` off", reach: "deprecated", group: "advanced", dial: { window: [0, 0.165], step: 0.025 } },
  { path: "/skeleton/bias/supernatural/writheWavelength", label: "writhe wavelength", kind: "real", optional: false, meaning: "How long each of those wanders runs, as a share of the height. Raising it gives fewer, lazier bends; lowering it gives tighter kinks.", unit: "share of height", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "`enabled` off, or `writheAmplitude` zero", reach: "deprecated", group: "advanced", dial: { window: [0.225, 0.675], step: 0.05 } },
  { path: "/skeleton/bias/supernatural/spiralRate", label: "spiral rate", kind: "real", optional: false, meaning: "How many full turns the wander winds around the trunk over the tree's height. Raising it tightens the spiral. At zero the wander winds around nothing, and any rise starts the spiral.", unit: "turns over the height", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "`enabled` off, or `writheAmplitude` zero", reach: "deprecated", group: "advanced", dial: { window: [0, 3.9], step: 0.5 } },
  { path: "/skeleton/bias/supernatural/maxWritheMagnitude", label: "max writhe magnitude", kind: "real", optional: false, meaning: "The ceiling on how hard the wander may pull in any one step, so the other writhe rows cannot bend the wood arbitrarily.", unit: "-", low: 0, high: 8, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 8], step: 1 } },
  { path: "/skeleton/twigs/twig/diameter", label: "leaf box: twig thickness", kind: "real", optional: false, meaning: "The leaf box's reach where foliage sits on twigs, as the thickness of the wood a leaf station is seated on, in metres; the engine sizes the grown wood itself (fn-198).", unit: "m", low: 0.000001, high: 1000000, lowOpen: false, zero: false, applies: "`canopy.shootRadius` above zero", reach: "mature", group: "advanced", dial: { window: [0.000001, 1000000], step: 0.001 } },
  { path: "/skeleton/twigs/twig/length", label: "length", kind: "real", optional: false, meaning: "The length in metres a twig shoot grows before it stops, and the whole of one internode on leaf-bearing wood. Raising it lengthens every twig, so the crown carries a deeper, shaggier skin.", unit: "m", low: 0.000001, high: 1000000, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0.000001, 1000000], step: 0.05 } },
  { path: "/skeleton/twigs/twig/internodeLength", label: "internode length", kind: "real", optional: false, meaning: "Metres between the joints on wood thicker than the bearing diameter, and the spacing of the stations a leaf sits on. Raising it gives longer segments, so laterals and leaves sit further apart.", unit: "m", low: 0.000001, high: 1000000, lowOpen: false, zero: false, applies: "a crown of fronds (`rosetteFronds` above zero)", reach: "mature", group: "leaves", dial: { window: [0.000001, 1000000], step: 0.015 } },
  { path: "/skeleton/twigs/twig/stationsPerInternode", label: "stations per internode", kind: "count", optional: false, meaning: "How many leaf stations sit at each joint, each turned its own share of a full turn around the shoot. Raising it crowds more leaves onto the same joints.", unit: "stations", low: 1, high: 32, lowOpen: false, zero: false, applies: "a crown of fronds (`rosetteFronds` above zero)", reach: "mature", group: "leaves", dial: { window: [1, 32], step: 1 } },
  { path: "/skeleton/twigs/twig/bearingDiameter", label: "bearing diameter", kind: "real", optional: false, meaning: "The thickness in metres at or below which a shoot bears leaves and side shoots of its own. Raising it lets thicker wood bear, so foliage reaches further back down the branch.", unit: "m", low: 0.000001, high: 1000000, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0.000001, 1000000], step: 0.01 } },
  { path: "/skeleton/twigs/lengthRatio", label: "length ratio", kind: "real", optional: false, meaning: "A shoot's length as a share of the one that bore it. Raising it makes each generation of twigs longer relative to its parent.", unit: "share", low: 0.05, high: 1, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0.05, 1], step: 0.15 } },
  { path: "/skeleton/twigs/ratioPower", label: "ratio power", kind: "real", optional: false, meaning: "How much thinner a shoot is than its parent for the same drop in length. Raising it leaves side shoots finer.", unit: "-", low: 0, high: 8, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0, 8], step: 1 } },
  { path: "/skeleton/twigs/internodeFactor", label: "internode factor", kind: "real", optional: false, meaning: "The fewest of its own diameters a segment of wood may span. Raising it makes segments longer for the same thickness, so there are fewer joints and the wood reads straighter.", unit: "-", low: 0.05, high: 32, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0.05, 32], step: 0.5 } },
  { path: "/skeleton/twigs/maxInternodes", label: "max internodes", kind: "count", optional: false, meaning: "The ceiling on segments one length of wood may be cut into. It binds only where the two lengths above would cut more, and there it caps the cost rather than states a look.", unit: "internodes", low: 1, high: 4294967295, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves" },
  { path: "/skeleton/twigs/laterals", label: "laterals", kind: "count", optional: false, meaning: "How many side shoots leave each station along a twig. Raising it crowds more twigs onto the same length of wood.", unit: "laterals", low: 0, high: 7, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0, 7], step: 1 } },
  { path: "/skeleton/twigs/generations", label: "generations", kind: "count", optional: false, meaning: "Twig-law generations of branching, 1 to 6. A lateral born at or past this generation is a twig whatever the pipe model left its radius, so the twig layer's depth is a row a table states rather than a consequence of how thick the wood is.", unit: "generations", low: 1, high: 6, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [1, 6], step: 1 } },
  { path: "/skeleton/twigs/limbRadius", label: "limb radius", kind: "real", optional: false, meaning: "The share of the trunk's radius at or below which wood starts bearing twigs. Raising it lets twigs start on thicker wood, so they reach further back toward the trunk.", unit: "share of root radius", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/twigs/reach", label: "reach", kind: "real", optional: false, meaning: "How deep the outer skin of the crown is that only twigs may fill, as a share of the crown: the scaffold is grown into what is left inside it. Raising it holds the structural wood further in and leaves a deeper twig layer.", unit: "share", low: 0, high: 0.9, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0, 0.9], step: 0.15 } },
  { path: "/skeleton/twigs/angle", label: "angle", kind: "real", optional: false, meaning: "The degrees a side shoot leaves its parent. Raising it swings twigs further out toward square with the branch.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0, 90], step: 1.5 } },
  { path: "/skeleton/twigs/angleVariation", label: "angle variation", kind: "real", optional: false, meaning: "How many degrees that departure angle varies shoot to shoot. Raising it makes the twig layer less uniform.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0, 90], step: 1.5 } },
  { path: "/skeleton/twigs/vigourVariation", label: "vigour variation", kind: "real", optional: false, meaning: "How much shoot length varies shoot to shoot. Raising it gives a more uneven, less combed twig layer.", unit: "share", low: 0, high: 0.95, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0, 0.95], step: 0.15 } },
  { path: "/skeleton/twigs/divergence", label: "divergence", kind: "real", optional: false, meaning: "The degrees each successive shoot is turned around the wood that bears it. Raising it swings the next shoot further around, so the twigs spiral differently.", unit: "degrees", low: -Infinity, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [68.752, 206.256], step: 20 } },
  { path: "/skeleton/twigs/hang", label: "hang", kind: "real", optional: false, meaning: "How strongly a shoot hangs, 0 to 3. At 0 nothing hangs and the local law is the ordinary one; at 1 a curtain takes its full droop.", unit: "-", low: 0, high: 3, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0, 3], step: 0.5 } },
  { path: "/skeleton/twigs/pendulousLength", label: "pendulous length", kind: "real", optional: false, meaning: "Metres a pendulous shoot grows before it stops, and the length its droop reaches the cap over.", unit: "m", low: 0.05, high: 5, lowOpen: false, zero: false, applies: "`hang` zero", reach: "deprecated", group: "leaves", dial: { window: [0.05, 5], step: 1 } },
  { path: "/skeleton/twigs/pendulousRadius", label: "pendulous radius", kind: "real", optional: false, meaning: "Fraction of the root radius at or below which a station's shoots hang.", unit: "share of stem radius", low: 0, high: 1, lowOpen: false, zero: false, applies: "`hang` zero", reach: "deprecated", group: "leaves", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/twigs/curtainSeparation", label: "curtain separation", kind: "real", optional: false, meaning: "Degrees between neighbouring shoots in a curtain.", unit: "degrees", low: 1, high: 45, lowOpen: false, zero: false, applies: "`hang` zero", reach: "deprecated", group: "leaves", dial: { window: [1, 45], step: 1.5 } },
  { path: "/skeleton/twigs/sag", label: "sag", kind: "real", optional: false, meaning: "How far toward straight down a hanging shoot's course has turned by the end of its pendulous length, 0 to 1: at 0 the shoot holds the direction it departed with and the curtain is a set of rods, at 1 it hangs vertical, and the turn is spread along the run as an arc steepest at the wood that bears it.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "`hang` zero", reach: "deprecated", group: "leaves", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/twigs/pendulousVariation", label: "pendulous variation", kind: "real", optional: false, meaning: "How much shorter than the pendulous length a hanging shoot may run, 0 to 1: each shoot's own length is the pendulous length times one minus this times a draw in 0 to 1 keyed by the shoot and the seed. At 0 every shoot has the one length the table states.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "`hang` zero", reach: "deprecated", group: "leaves", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/twigs/curtainDrop", label: "curtain drop", kind: "real", optional: false, meaning: "How far below the shell's lower surface a hanging shoot may fall, 0 to 1, as a share of the way from that surface down to the clearance: at 0 the shell binds a hanging shoot as it binds every other, at 1 the shoot may fall to the clearance. Only a shoot that hangs, only under the crown's footprint.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "leaves", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/twigs/curtainClearance", label: "curtain clearance", kind: "real", optional: false, meaning: "Metres above the ground no hanging shoot falls below, 0 to 5. Never above the crown's own base, whatever the row says.", unit: "m", low: 0, high: 5, lowOpen: false, zero: false, applies: "unless `sag` or `curtainDrop` is above zero", reach: "deprecated", group: "leaves", dial: { window: [0, 5], step: 1 } },
  { path: "/skeleton/twigs/maxDroop", label: "max droop", kind: "real", optional: false, meaning: "The furthest a hanging shoot may bend toward straight down. Raising it lets curtains hang more heavily.", unit: "-", low: 0, high: 10, lowOpen: false, zero: false, applies: "`hang` zero", reach: "deprecated", group: "leaves", dial: { window: [0, 10], step: 1.5 } },
  { path: "/skeleton/twigs/curtainStepClearance", label: "curtain step clearance", kind: "real", optional: false, meaning: "How close to the ground a hanging curtain may reach before it stops growing. Raising it lets curtains hang nearer the floor.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "`hang` zero", reach: "deprecated", group: "leaves", dial: { window: [0, 1], step: 0.15 } },
  { path: "/skeleton/growth/influenceRadius", label: "influence radius", kind: "real", optional: true, meaning: "Metres a pull point may reach to steer the wood nearest it; unset, the crown's own volume and the point count decide it. Raising it lets distant points draw a branch across the crown.", unit: "m", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "`attractorWeight` zero", reach: "deprecated", group: "advanced" },
  { path: "/skeleton/growth/killDistance", label: "kill distance", kind: "real", optional: true, meaning: "Metres within which a pull point counts as reached and stops pulling; unset, twice the step distance. Raising it uses the points up sooner, so branches stop shorter and the crown fills coarsely.", unit: "m", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "`attractorWeight` zero", reach: "deprecated", group: "advanced" },
  { path: "/skeleton/growth/stepDistance", label: "step distance", kind: "real", optional: true, meaning: "Metres of wood laid down in one growth step; unset, the tree's height times `step`. Raising it lays down longer, coarser segments.", unit: "m", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "", reach: "deprecated", group: "advanced" },
  { path: "/skeleton/growth/trunkHeight", label: "trunk height", kind: "real", optional: true, meaning: "Metres of bare trunk before the crown may start; unset, the envelope's own crown base. Raising it lifts the whole crown and leaves a longer clear bole.", unit: "m", low: 0, high: Infinity, lowOpen: true, zero: false, applies: "", reach: "deprecated", group: "advanced" },
  { path: "/skeleton/growth/maxNodes", label: "max nodes", kind: "count", optional: true, meaning: "The nodes a tree may grow; unset, the shipped default. A tree that would grow more gives up its finest twigs evenly and finishes every branch, so raising it restores fine detail only to a tree that reached it.", unit: "nodes", low: 1, high: 4294967295, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [100000, 1000000], step: 50000 } },
  { path: "/skeleton/growth/maxTurnPerStep", label: "max turn per step", kind: "real", optional: true, meaning: "The most a growing shoot may turn in one step, in degrees; unset, 35. At 180 or more nothing is limited.", unit: "degrees", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced" },
  { path: "/radii/trunkRadius", label: "leaf box: wood radius", kind: "real", optional: false, meaning: "The leaf box's reach where foliage sits on slender wood, as a trunk radius at the ground that is a share of the tree's height; the engine sizes the grown wood itself (fn-198).", unit: "share of height", low: 0.000004, high: 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000, lowOpen: false, zero: false, applies: "`canopy.shootRadius` zero", reach: "mature", group: "advanced", dial: { window: [0.000004, 0.023], step: 0.002 } },
  { path: "/radii/forkExponent", label: "fork exponent", kind: "real", optional: false, meaning: "How wood divides at a fork. The parent's area is the sum of the children's radii raised to this power, so raising it leaves the children thicker for the same parent.", unit: "-", low: 1, high: 8, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [1, 8], step: 1 } },
  { path: "/radii/lengthTaper", label: "length taper", kind: "real", optional: false, meaning: "How fast wood thins along its own length. Raising it makes a branch narrow more sharply from its base to its tip.", unit: "per height", low: 0, high: 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 0.8], step: 0.1 } },
  { path: "/radii/maxTaperExponent", label: "max taper exponent", kind: "real", optional: false, meaning: "The ceiling on accumulated taper, so no single long branch can thin away to nothing. Raising it lets long branches taper further.", unit: "-", low: 0, high: 64, lowOpen: false, zero: false, applies: "`lengthTaper` zero", reach: "deprecated", group: "advanced", dial: { window: [0, 64], step: 2 } },
  { path: "/radii/lateralShare", label: "lateral share", kind: "real", optional: false, meaning: "The wood a lateral takes at a fork against what its own subtree asks, 0 to 1: the parent carries a lateral's pipe at this share and the lateral's wood thins by its root. At one every fork divides by the pipe model alone.", unit: "share", low: 0.01, high: 1, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0.01, 1], step: 0.1 } },
  { path: "/radii/forkBalance", label: "fork balance", kind: "real", optional: false, meaning: "The wood a codominant sibling takes at a fork against its own subtree's, 0 to 1, beside the primary that carries the axis on. At one the parts divide by the pipe model alone.", unit: "share", low: 0.01, high: 1, lowOpen: false, zero: false, applies: "`skeleton.habit.codominance` zero", reach: "deprecated", group: "advanced", dial: { window: [0.01, 1], step: 0.1 } },
  { path: "/radii/girthHold", label: "girth hold", kind: "real", optional: false, meaning: "The share of each structural axis's reach over which it holds the girth it starts with, 0 to 0.9: past it the wood falls to the pipe model's radius by the axis's tip. At zero wood is the pipe model's.", unit: "share of reach", low: 0, high: 0.9, lowOpen: false, zero: false, applies: "", reach: "deprecated", group: "advanced", dial: { window: [0, 0.9], step: 0.05 } },
  { path: "/radii/girthFall", label: "girth fall", kind: "real", optional: false, meaning: "How short the fall after `girthHold` is: the fall lasts the hold's share divided by this, so raising it breaks the wood into twigs over a shorter distance.", unit: "-", low: 0.5, high: 8, lowOpen: false, zero: false, applies: "`girthHold` zero", reach: "deprecated", group: "advanced", dial: { window: [0.5, 8], step: 0.5 } },
  { path: "/surface/radialSegments", label: "radial segments", kind: "count", optional: false, meaning: "How many sides each piece of wood is drawn with. Raising it makes the wood rounder and smoother, and costs triangles.", unit: "sides", low: 3, high: 64, lowOpen: false, zero: false, applies: "", reach: "mature", group: "advanced", dial: { window: [3, 64], step: 2 } },
  { path: "/surface/lobes", label: "lobes", kind: "count", optional: false, meaning: "How many ridges run up around the trunk. Raising it gives the bark more flutes; zero is a plain round bole.", unit: "lobes", low: 0, high: 16, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 16], step: 2 } },
  { path: "/surface/lobeDepth", label: "lobe depth", kind: "real", optional: false, meaning: "How deep the flutes between those ridges cut, as a share of the wood's own radius. Raising it makes the fluting more pronounced. At zero the bole is plainly round whatever the ridge count says, and any rise starts cutting the flutes.", unit: "share of radius", low: 0, high: 0.9, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 0.9], step: 0.15 } },
  { path: "/surface/twistRate", label: "twist rate", kind: "real", optional: false, meaning: "How many turns those ridges make over the tree's height. Raising it winds them more tightly around the trunk.", unit: "turns over the height", low: -64, high: 64, lowOpen: false, zero: false, applies: "unless `lobes` and `lobeDepth` are both above zero", reach: "mature", group: "bark", dial: { window: [-64, 64], step: 1 } },
  { path: "/surface/flareRadius", label: "flare radius", kind: "real", optional: false, meaning: "How much wider the trunk is where it meets the ground, as a multiple of its own radius. Raising it gives a broader buttress.", unit: "multiple of radius", low: 1, high: 8, lowOpen: false, zero: false, applies: "", reach: "mature", group: "form", dial: { window: [1, 8], step: 0.2 } },
  { path: "/surface/flareFalloff", label: "flare falloff", kind: "real", optional: false, meaning: "How far up the trunk that flare reaches, as a share of the height. Raising it carries the swelling further up the bole.", unit: "share of height", low: 0.0001, high: 1, lowOpen: false, zero: false, applies: "`flareRadius` one", reach: "mature", group: "form", dial: { window: [0.0001, 1], step: 0.15 } },
  { path: "/surface/flareDepth", label: "flare depth", kind: "real", optional: false, meaning: "How deep the trunk's base is sunk below the ground, as a share of the height. Raising it buries more of the flare.", unit: "share of height", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "form", dial: { window: [0, 1], step: 0.15 } },
  { path: "/surface/forkSocket", label: "fork socket", kind: "real", optional: false, meaning: "How deeply a child branch is set into its parent at a fork. Raising it sinks the junction further in, so the two read as one piece of wood rather than two tubes meeting.", unit: "share", low: 0, high: 0.9, lowOpen: false, zero: false, applies: "a stem that never forks", reach: "mature", group: "form", dial: { window: [0, 0.9], step: 0.15 } },
  { path: "/surface/socketContainment", label: "socket containment", kind: "real", optional: false, meaning: "Fraction of the parent's inscribed radius available for a socket.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "`forkSocket` zero", reach: "mature", group: "form", dial: { window: [0, 1], step: 0.15 } },
  { path: "/surface/forkSwell", label: "fork swell", kind: "real", optional: false, meaning: "How much wood thickens at a fork. Raising it leaves a more pronounced collar where a branch leaves its parent.", unit: "multiple of radius", low: 1, high: 4, lowOpen: false, zero: false, applies: "a stem that never forks", reach: "mature", group: "form", dial: { window: [1, 4], step: 0.5 } },
  { path: "/canopy/shootRadius", label: "shoot radius", kind: "real", optional: false, meaning: "Wood at or below this fraction of the root radius bears foliage of its own, beside whatever the twig layer marks. Zero leaves the twigs alone with it; without a twig layer it is what selects the terminal shoots.", unit: "share of root radius", low: 0, high: 1, lowOpen: false, zero: false, applies: "a crown of fronds (`rosetteFronds` above zero)", reach: "mature", group: "leaves", dial: { window: [0, 1], step: 0.005 } },
  { path: "/canopy/spacing", label: "spacing", kind: "real", optional: false, meaning: "Metres between leaves along a shoot, as a share of the tree's height. Raising it spreads the leaves further apart, so the crown carries fewer of them.", unit: "share of height", low: 0.001, high: 1000000, lowOpen: false, zero: false, applies: "only a placement with no twig table reads it, which no production caller passes", reach: "deprecated", group: "leaves" },
  { path: "/canopy/divergence", label: "divergence", kind: "real", optional: false, meaning: "The degrees each successive leaf is turned around its shoot. Raising it turns the next leaf further round, so the leaves spiral differently.", unit: "degrees", low: -1000000000, high: 1000000000, lowOpen: false, zero: false, applies: "a crown of fronds (`rosetteFronds` above zero)", reach: "mature", group: "leaves", dial: { window: [-1000000000, 1000000000], step: 15 } },
  { path: "/canopy/clump", label: "clump", kind: "count", optional: false, meaning: "How many extra leaves are gathered at the end of a shoot that has no twig layer. Raising it packs a denser tuft at the tip.", unit: "leaves", low: 0, high: 64, lowOpen: false, zero: false, applies: "only a placement with no twig table reads it, which no production caller passes", reach: "deprecated", group: "leaves" },
  { path: "/canopy/clumpSpan", label: "clump span", kind: "real", optional: false, meaning: "How far back from the tip that tuft is scattered, as a share of the shoot's length. Raising it spreads the tuft further down the shoot.", unit: "share of the shoot", low: 0, high: 1, lowOpen: false, zero: false, applies: "only a placement with no twig table reads it, which no production caller passes", reach: "deprecated", group: "leaves" },
  { path: "/canopy/outward", label: "outward", kind: "real", optional: false, meaning: "How far a leaf turns away from the trunk. Raising it points the leaves outward, away from the tree's axis.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "a crown of fronds (`rosetteFronds` above zero)", reach: "mature", group: "leaves", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/canopy/upward", label: "upward", kind: "real", optional: false, meaning: "How far a leaf turns toward the sky. Raising it tips the leaves up.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "a crown of fronds (`rosetteFronds` above zero)", reach: "mature", group: "leaves", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/canopy/forwardLean", label: "forward lean", kind: "real", optional: false, meaning: "Lean along the shoot, as a fraction of the radial off the wood.", unit: "share of the radial", low: -1, high: 1, lowOpen: false, zero: false, applies: "a crown of fronds (`rosetteFronds` above zero)", reach: "mature", group: "leaves", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/canopy/leanRise", label: "lean rise", kind: "real", optional: false, meaning: "Further lean along the shoot on radials that face upward.", unit: "share of the radial", low: -2, high: 2, lowOpen: false, zero: false, applies: "a crown of fronds (`rosetteFronds` above zero)", reach: "mature", group: "leaves", dial: { window: [-2, 2], step: 0.5 } },
  { path: "/canopy/surfaceContact", label: "surface contact", kind: "real", optional: false, meaning: "The station sits on the shoot axis at 0 and on the wood's own contact surface at 1; the surface is built whenever it is positive.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "leaves", dial: { window: [0, 1], step: 0.15 } },
  { path: "/canopy/scatter", label: "scatter", kind: "real", optional: false, meaning: "The degrees a leaf may be turned at random from where it was placed. Raising it leaves the crown less combed.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "", reach: "mature", group: "leaves", dial: { window: [0, 90], step: 15 } },
  { path: "/canopy/size", label: "size", kind: "real", optional: false, meaning: "The size every leaf is drawn at, as a multiple of the element's own dimensions. Raising it enlarges every leaf.", unit: "multiple of the element", low: 0, high: 1000, lowOpen: false, zero: false, applies: "", reach: "mature", group: "leaves", dial: { window: [0, 1000], step: 0.15 } },
  { path: "/canopy/sizeVariation", label: "size variation", kind: "real", optional: false, meaning: "How far leaf size varies leaf to leaf, as a share of that size. Raising it mixes larger and smaller leaves more widely.", unit: "share", low: 0, high: 0.9, lowOpen: false, zero: false, applies: "", reach: "mature", group: "leaves", dial: { window: [0, 0.9], step: 0.15 } },
  { path: "/canopy/limbClumping", label: "limb clumping", kind: "real", optional: false, meaning: "How far into each limb system the gap between it and its neighbours reaches, as a share of the way from their shared boundary to the system's centre: each limb system then keeps a rounded leaf mass of its own. Zero, the neutral, thins nothing.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "leaves", dial: { window: [0, 1], step: 0.15 } },
  { path: "/canopy/clumpSystemOrder", label: "clump system order", kind: "count", optional: false, meaning: "How deep a lateral may be and still start a limb system of its own. Raising it parts the crown into more and smaller leaf masses; it does nothing until `limb_clumping` is above zero.", unit: "order", low: 0, high: Infinity, lowOpen: false, zero: false, applies: "`limbClumping` zero, but for the limb tally of a requested leaf field", reach: "mature", group: "advanced", dial: { window: [1, 5], step: 1 } },
  { path: "/canopy/clumpNeighbours", label: "clump neighbours", kind: "count", optional: false, meaning: "Nearest neighbours and cell crossings in the clumping approximation.", unit: "neighbours", low: 1, high: 4294967295, lowOpen: false, zero: false, applies: "`limbClumping` zero", reach: "mature", group: "advanced" },
  { path: "/canopy/rosetteFronds", label: "rosette fronds", kind: "real", optional: false, meaning: "Fronds the rosette bears at the apex of each stem. At zero no rosette stands. A fraction is split by a keyed draw at each apex, which bears the whole count either side of it (fn-277).", unit: "fronds", low: 0, high: 128, lowOpen: false, zero: false, applies: "", reach: "mature", group: "leaves", dial: { window: [0, 128], step: 1 } },
  { path: "/canopy/cauline", label: "cauline", kind: "real", optional: false, meaning: "The share of leaf-bearing wood that bears its leaves along it, cauline foliage as against a crown's alone: each run of leaf-bearing wood and each short shoot keeps its leaves by a keyed draw at this share. One, every one bears; at zero none does, as a palm bears its leaves only in its crown (fn-277).", unit: "share of leaf-bearing wood", low: 0, high: 1, lowOpen: false, zero: false, applies: "a crown of fronds (`rosetteFronds` above zero)", reach: "mature", group: "leaves", dial: { window: [0, 1], step: 0.05 } },
  { path: "/canopy/rosetteDivergence", label: "rosette divergence", kind: "real", optional: false, meaning: "The degrees each successive frond is turned about the apex.", unit: "degrees", low: -360, high: 360, lowOpen: false, zero: false, applies: "no rosette and no leaf bases (`rosetteFronds` and `leafBases` zero)", reach: "mature", group: "leaves", dial: { window: [90, 180], step: 2 } },
  { path: "/canopy/rosettePitch", label: "rosette pitch", kind: "real", optional: false, meaning: "Degrees from the axis the youngest frond stands: 0 upright, 90 level, 180 hanging.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "`rosetteFronds` zero", reach: "mature", group: "leaves", dial: { window: [0, 180], step: 5 } },
  { path: "/canopy/rosettePitchSpread", label: "rosette pitch spread", kind: "real", optional: false, meaning: "How many degrees further than the youngest the oldest frond leans, so the crown opens from a spike to a skirt.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "`rosetteFronds` zero", reach: "mature", group: "leaves", dial: { window: [0, 180], step: 5 } },
  { path: "/canopy/rosetteDepth", label: "rosette depth", kind: "real", optional: false, meaning: "Metres below the apex the frond insertions are spread down the axis. At zero every frond leaves one point.", unit: "m", low: 0, high: 100, lowOpen: false, zero: false, applies: "no rosette and no leaf bases (`rosetteFronds` and `leafBases` zero)", reach: "mature", group: "leaves", dial: { window: [0, 4], step: 0.05 } },
  { path: "/canopy/leafletCount", label: "leaflet count", kind: "real", optional: false, meaning: "Leaflets one placement carries along its rachis. One is the single blade every family drew. A fraction is split by a keyed draw at each placement (fn-277).", unit: "leaflets", low: 1, high: 256, lowOpen: false, zero: false, applies: "`rachisLength` zero", reach: "mature", group: "leaves", dial: { window: [1, 256], step: 2 } },
  { path: "/canopy/rachisLength", label: "rachis length", kind: "real", optional: false, meaning: "Metres of rachis the leaflets are strung along. At zero the placement is one blade whatever the count says.", unit: "m", low: 0, high: 1000, lowOpen: false, zero: false, applies: "`leafletCount` one", reach: "mature", group: "leaves", dial: { window: [0, 8], step: 0.1 } },
  { path: "/canopy/leafletPitch", label: "leaflet pitch", kind: "real", optional: false, meaning: "The degrees a leaflet leaves its rachis.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "no leaflet grouping (`leafletCount` one or `rachisLength` zero)", reach: "mature", group: "leaves", dial: { window: [0, 90], step: 3 } },
  { path: "/canopy/rachisArch", label: "rachis arch", kind: "real", optional: false, meaning: "How far the rachis bends out of the straight line from its station, as a share of its length. Positive arches up, negative droops.", unit: "share of the rachis", low: -1, high: 1, lowOpen: false, zero: false, applies: "no leaflet grouping (`leafletCount` one or `rachisLength` zero)", reach: "mature", group: "leaves", dial: { window: [-1, 1], step: 0.05 } },
  { path: "/canopy/terminalLeaflet", label: "terminal leaflet", kind: "real", optional: false, meaning: "Whether a single leaflet closes the rachis's end, blended 0 to 1: the last leaflet turns from standing off the rachis to lying along it.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "no leaflet grouping (`leafletCount` one or `rachisLength` zero)", reach: "mature", group: "leaves", dial: { window: [0, 1], step: 0.05 } },
  { path: "/canopy/leafBases", label: "leaf bases", kind: "real", optional: false, meaning: "Bases of shed fronds the stem keeps below its crown, clothing the trunk. At zero the trunk is bare and the bark is what it always was; any rise carries the crown's own spiral down it.", unit: "bases", low: 0, high: 256, lowOpen: false, zero: false, applies: "`leafBaseLength` zero", reach: "mature", group: "form", dial: { window: [0, 256], step: 4 } },
  { path: "/canopy/leafBaseLength", label: "leaf base length", kind: "real", optional: false, meaning: "Metres a retained base stands out from the bark. At zero no base is drawn whatever the count says.", unit: "m", low: 0, high: 10, lowOpen: false, zero: false, applies: "`leafBases` zero", reach: "mature", group: "form", dial: { window: [0, 10], step: 0.05 } },
  { path: "/canopy/leafBaseRadius", label: "leaf base radius", kind: "real", optional: false, meaning: "How thick a base is where it leaves the bark, as a share of the stem's own radius there. Raising it leaves a broader boot.", unit: "share of stem radius", low: 0, high: 1, lowOpen: false, zero: false, applies: "no leaf base", reach: "mature", group: "form", dial: { window: [0, 1], step: 0.05 } },
  { path: "/canopy/leafBasePitch", label: "leaf base pitch", kind: "real", optional: false, meaning: "Degrees from the stem's axis a base points: 0 flat against the trunk, 90 square out of it, 180 turned back down.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "no leaf base", reach: "mature", group: "form", dial: { window: [0, 180], step: 5 } },
  { path: "/canopy/leafBaseWeathering", label: "leaf base weathering", kind: "real", optional: false, meaning: "How far the lowest and oldest base is worn back against the newest, in both its length and its girth. At zero every base stands full down the whole trunk, and any rise wears the foot away.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "no leaf base", reach: "mature", group: "form", dial: { window: [0, 1], step: 0.05 } },
  { path: "/canopy/leafBaseWidth", label: "leaf base width", kind: "real", optional: false, meaning: "How broad a retained base is across the trunk, as a share of the cell the crown's spiral gives it on the bark: at 1 every base meets its neighbours edge to edge whatever the count and the trunk's girth, below it the bark shows between them and above it they crowd into each other. At zero the base is the round peg the radius row sizes and the lattice rows say nothing; any rise packs the bases into the lattice.", unit: "share of the lattice cell", low: 0, high: 2, lowOpen: false, zero: false, applies: "no leaf base", reach: "mature", group: "form", dial: { window: [0, 2], step: 0.05 } },
  { path: "/canopy/leafBaseFlatness", label: "leaf base flatness", kind: "real", optional: false, meaning: "How flat-sided a lattice base is drawn: 0 the ellipse through its cell's corners, 1 the cell itself, a diamond with flat faces that meets each neighbour along a straight edge and is cut square at its outer end. At zero the section stays round, and at no width it reads nothing.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "no leaf base, or `leafBaseWidth` zero", reach: "mature", group: "form", dial: { window: [0, 1], step: 0.1 } },
  { path: "/canopy/acanthophylls", label: "acanthophylls", kind: "real", optional: false, meaning: "Leaflets at a frond's base borne as spines rather than blades. At zero the frond carries blades all the way down; any rise hardens that many of them.", unit: "leaflets", low: 0, high: 256, lowOpen: false, zero: false, applies: "`acanthophyllLength` zero, or no leaflet grouping (`leafletCount` one or `rachisLength` zero)", reach: "mature", group: "leaves", dial: { window: [0, 256], step: 1 } },
  { path: "/canopy/acanthophyllLength", label: "acanthophyll length", kind: "real", optional: false, meaning: "The share of a leaflet's own size a spine is drawn at. At zero no spine is drawn whatever the count says.", unit: "share of a leaflet", low: 0, high: 1, lowOpen: false, zero: false, applies: "`acanthophylls` zero, or no leaflet grouping (`leafletCount` one or `rachisLength` zero)", reach: "mature", group: "leaves", dial: { window: [0, 1], step: 0.05 } },
  { path: "/canopy/acanthophyllPitch", label: "acanthophyll pitch", kind: "real", optional: false, meaning: "The degrees a spine leaves the rachis, in place of the leaflet's own pitch.", unit: "degrees", low: 0, high: 90, lowOpen: false, zero: false, applies: "no spine", reach: "mature", group: "leaves", dial: { window: [0, 90], step: 3 } },
  { path: "/canopy/skirtFronds", label: "skirt fronds", kind: "real", optional: false, meaning: "Dead fronds a rosette keeps below its living crown, continuing the crown's own spiral down the stem. At zero no frond is kept and the crown ends at its oldest living frond; any rise hangs that many.", unit: "fronds", low: 0, high: 128, lowOpen: false, zero: false, applies: "`skirtLength` zero, or `rosetteFronds` zero", reach: "mature", group: "leaves", dial: { window: [0, 128], step: 2 } },
  { path: "/canopy/skirtPitch", label: "skirt pitch", kind: "real", optional: false, meaning: "Degrees from the axis a dead frond hangs: 0 upright, 90 level, 180 collapsed straight down against the stem.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "no dead frond", reach: "mature", group: "leaves", dial: { window: [0, 180], step: 5 } },
  { path: "/canopy/skirtLength", label: "skirt length", kind: "real", optional: false, meaning: "A dead frond's length as a share of a living one's, rachis and leaflets alike. At zero no dead frond is drawn whatever the count says.", unit: "share of a living frond", low: 0, high: 1, lowOpen: false, zero: false, applies: "`skirtFronds` zero, or `rosetteFronds` zero", reach: "mature", group: "leaves", dial: { window: [0, 1], step: 0.05 } },
  { path: "/canopy/frondLife", label: "frond life", kind: "real", optional: false, meaning: "The years a living frond lives (fn-270 R5b): a rosette makes its living crown's count over that many years, its fronds aged by their place on the spiral, this year's emerging over its stem's elongation and unfolding by its expansion, the oldest passing into the skirt and out of it as new ones emerge above. At zero the crown makes none and stands as it is all year.", unit: "years", low: 0, high: 100, lowOpen: false, zero: false, applies: "`rosetteFronds` zero", reach: "mature", group: "leaves", dial: { window: [0, 100], step: 3 } },
  { path: "/canopy/frondEstablishment", label: "frond establishment", kind: "real", optional: false, meaning: "The years a rosette's crown takes to make a mature frond (fn-275 R2, R2a): a frond is made at a share of its size set by the crown's age when it was made, along the logistic course from `frondEarlyShare` to whole over these years, and keeps it for its life; the stem the rosette crowns widens by the same course and waits at the ground until the crown is established. At zero every frond is made whole and the stem rises from its first year.", unit: "years", low: 0, high: 100, lowOpen: false, zero: false, applies: "`rosetteFronds` zero", reach: "mature", group: "leaves", dial: { window: [0, 100], step: 1 } },
  { path: "/canopy/frondEarlyShare", label: "frond early share", kind: "real", optional: false, meaning: "The share of a mature frond's size a crown's first frond is made at while it establishes, and of its stem's girth where the stem was first laid (fn-275 R2, R2a).", unit: "share of a mature frond", low: 0, high: 1, lowOpen: false, zero: false, applies: "`frondEstablishment` zero", reach: "mature", group: "leaves", dial: { window: [0, 1], step: 0.05 } },
  { path: "/canopy/maxInstances", label: "max instances", kind: "count", optional: false, meaning: "Hard total budget. Exceeding it returns an error, never partial foliage.", unit: "leaves", low: 1, high: 18446744073709552000, lowOpen: false, zero: false, applies: "", reach: "mature", group: "advanced" },
  { path: "/element/connectorLength", label: "connector length", kind: "real", optional: false, meaning: "Metres of petiole or woody peg below the blade, excluded from unit dimensions. Every element carries one.", unit: "m", low: 0.000001, high: 1000, lowOpen: false, zero: false, applies: "`card`", reach: "mature", group: "leaves", dial: { window: [0.000001, 0.0325], step: 0.005 } },
  { path: "/element/length", label: "length", kind: "real", optional: false, meaning: "Blade/needle longitudinal extent, excluding connector, in metres.", unit: "m", low: 0.0001, high: 1000, lowOpen: false, zero: false, applies: "", reach: "mature", group: "leaves", dial: { window: [0.0001, 1000], step: 0.025 } },
  { path: "/element/width", label: "width", kind: "real", optional: false, meaning: "The blade's greatest width in metres, across the midrib. Raising it makes every leaf broader.", unit: "m", low: 0.0001, high: 1000, lowOpen: false, zero: false, applies: "", reach: "mature", group: "leaves", dial: { window: [0.0001, 1000], step: 0.015 } },
  { path: "/element/widestAt", label: "widest at", kind: "real", optional: false, meaning: "Where the blade is widest, as a share of the way from its base to its tip. Raising it carries the widest point toward the tip.", unit: "share of the blade", low: 0.05, high: 0.95, lowOpen: false, zero: false, applies: "`card`", reach: "mature", group: "leaves", dial: { window: [0.05, 0.95], step: 0.15 } },
  { path: "/element/baseFullness", label: "base fullness", kind: "real", optional: false, meaning: "How fast the blade fills out above its stalk. Raising it draws the base in, so the leaf reads wedge-shaped rather than rounded.", unit: "-", low: 0.2, high: 8, lowOpen: false, zero: false, applies: "`card`", reach: "mature", group: "leaves", dial: { window: [0.2, 8], step: 0.15 } },
  { path: "/element/tipSharpness", label: "tip sharpness", kind: "real", optional: false, meaning: "How fast the blade narrows toward its point. Raising it draws the tip out into a sharper point.", unit: "-", low: 0.2, high: 8, lowOpen: false, zero: false, applies: "`card`", reach: "mature", group: "leaves", dial: { window: [0.2, 8], step: 1 } },
  { path: "/element/cup", label: "cup", kind: "real", optional: false, meaning: "How far the blade's margins lift out of its own plane, as a share of the half-width there. Raising it dishes the leaf more deeply along the midrib.", unit: "share of the half-width", low: -2, high: 2, lowOpen: false, zero: false, applies: "`card`", reach: "mature", group: "leaves", dial: { window: [-2, 2], step: 0.5 } },
  { path: "/element/curl", label: "curl", kind: "real", optional: false, meaning: "How far the blade bends along its length, as a share of its length at the tip. Raising it curls the tip further out of the plane its base stands in.", unit: "share of the length", low: -2, high: 2, lowOpen: false, zero: false, applies: "`card`", reach: "mature", group: "leaves", dial: { window: [-2, 2], step: 0.5 } },
  { path: "/element/lobeCount", label: "lobe count", kind: "count", optional: false, meaning: "Lobes along each margin; 0 is an entire margin.", unit: "lobes", low: 0, high: 8, lowOpen: false, zero: false, applies: "`lobeDepth` zero", reach: "mature", group: "leaves", dial: { window: [0, 8], step: 1 } },
  { path: "/element/lobeDepth", label: "lobe depth", kind: "real", optional: false, meaning: "How far each sinus cuts toward the midrib, 0 to 1. At zero the margin is entire whatever the lobe count says, and any rise starts cutting the sinuses.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lobeCount` zero", reach: "mature", group: "leaves", dial: { window: [0, 1], step: 0.15 } },
  { path: "/element/sectionRoundness", label: "section roundness", kind: "real", optional: false, meaning: "Flat blade at 0, four-sided shaft at 1.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "leaves", dial: { window: [0, 1], step: 0.15 } },
  { path: "/element/axialSegments", label: "axial segments", kind: "count", optional: false, meaning: "How many sections the blade is built from along its length. Raising it draws the outline and any lobes more smoothly, at more triangles per leaf.", unit: "sections", low: 2, high: 64, lowOpen: false, zero: false, applies: "`card`", reach: "mature", group: "advanced", dial: { window: [2, 64], step: 8 } },
  { path: "/element/crossSegments", label: "cross segments", kind: "count", optional: false, meaning: "How many columns the blade is built from across its width. Raising it draws the section and the margins more smoothly, at more triangles per leaf.", unit: "columns", low: 2, high: 64, lowOpen: false, zero: false, applies: "`card`", reach: "mature", group: "advanced", dial: { window: [2, 64], step: 1 } },
  { path: "/element/card", label: "card", kind: "switch", optional: false, meaning: "A flat two-triangle card in place of the modelled blade: no outline, no cup or curl, no sections and no connector.", unit: "switch", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "leaves" },
  { path: "/material/barkRed", label: "bark red", kind: "real", optional: false, meaning: "The bark's own colour, a linear reflectance per channel, not a display (sRGB) value; every colour row is on this one scale (see the reference's header). Shipped barks run 0.017 (the beech's blue) to 0.78 (the birch's white). Raising a channel pushes mature bark toward that colour.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/barkGreen", label: "bark green", kind: "real", optional: false, meaning: "The green channel of `bark_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/barkBlue", label: "bark blue", kind: "real", optional: false, meaning: "The blue channel of `bark_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/barkRoughness", label: "bark roughness", kind: "real", optional: false, meaning: "How diffuse the bark is: 0 is a mirror, 1 is chalk.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/shootRed", label: "shoot red", kind: "real", optional: false, meaning: "The young wood's own colour, a linear reflectance per channel on the bark's scale, before its bark has formed. Wood thinner than `shoot_radius` takes it, and gives it up to the bark colour on a smoothstep of its radius by twice that; zero means no wood is young.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`shootRadius` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/shootGreen", label: "shoot green", kind: "real", optional: false, meaning: "The green channel of `shoot_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`shootRadius` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/shootBlue", label: "shoot blue", kind: "real", optional: false, meaning: "The blue channel of `shoot_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`shootRadius` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/shootRadius", label: "shoot radius", kind: "real", optional: false, meaning: "Metres. The radius below which wood is young.", unit: "m", low: 0, high: 0.1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 0.1], step: 0.015 } },
  { path: "/material/matureRed", label: "mature red", kind: "real", optional: false, meaning: "The old bark's own colour, the one thick wood grows into. Wood takes it over the bark colour on a smoothstep of its radius from `mature_radius` to that plus `mature_span`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`matureRadius` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/matureGreen", label: "mature green", kind: "real", optional: false, meaning: "The green channel of `mature_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`matureRadius` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/matureBlue", label: "mature blue", kind: "real", optional: false, meaning: "The blue channel of `mature_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`matureRadius` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/matureFurrowStrength", label: "mature furrow strength", kind: "real", optional: false, meaning: "The furrow strength old bark takes in place of `furrow_strength`, over the same radius range as its colour: how deep it fissures.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`matureRadius` zero, or `ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/matureRadius", label: "mature radius", kind: "real", optional: false, meaning: "Metres. The radius at which bark starts to turn old; zero means no wood is old.", unit: "m", low: 0, high: 2, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 2], step: 0.015 } },
  { path: "/material/matureSpan", label: "mature span", kind: "real", optional: false, meaning: "Metres. How much further the radius grows before the bark is wholly old; zero turns it at once.", unit: "m", low: 0, high: 2, lowOpen: false, zero: false, applies: "`matureRadius` zero", reach: "mature", group: "bark", dial: { window: [0, 2], step: 0.015 } },
  { path: "/material/leafFrontRed", label: "leaf front red", kind: "real", optional: false, meaning: "The colour of a leaf's upper face, a linear reflectance per channel, not a display (sRGB) value. A dark broadleaf face is a few hundredths (the beech 0.021/0.016/0.007, the oak 0.021/0.048/0.021); shipped faces run 0.007 to 0.19. Raising a channel pushes the sunlit face toward it.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.02 } },
  { path: "/material/leafFrontGreen", label: "leaf front green", kind: "real", optional: false, meaning: "The green channel of `leaf_front_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/leafFrontBlue", label: "leaf front blue", kind: "real", optional: false, meaning: "The blue channel of `leaf_front_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.015 } },
  { path: "/material/leafBackRed", label: "leaf back red", kind: "real", optional: false, meaning: "The colour of a leaf's underside, a linear reflectance per channel on the upper face's scale (shipped: 0.011 to 0.28), shown wherever the eye sees the back of a blade. Raising a channel pushes that face toward it.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/leafBackGreen", label: "leaf back green", kind: "real", optional: false, meaning: "The green channel of `leaf_back_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.025 } },
  { path: "/material/leafBackBlue", label: "leaf back blue", kind: "real", optional: false, meaning: "The blue channel of `leaf_back_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.025 } },
  { path: "/material/leafDeadRed", label: "leaf dead red", kind: "real", optional: false, meaning: "The colour a leaf turns as it dies, a linear reflectance per channel on the leaf faces' scale, both faces alike: the dead fronds a rosette keeps below its living crown, and a leaf colouring in autumn before it falls (fn-270 R5c), which a tree drawn within its year moves toward. Raising a channel pushes either toward it.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no dead frond (`canopy.skirtFronds` zero) and no leaf that falls", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.05 } },
  { path: "/material/leafDeadGreen", label: "leaf dead green", kind: "real", optional: false, meaning: "The green channel of `leaf_dead_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no dead frond (`canopy.skirtFronds` zero) and no leaf that falls", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.05 } },
  { path: "/material/leafDeadBlue", label: "leaf dead blue", kind: "real", optional: false, meaning: "The blue channel of `leaf_dead_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no dead frond (`canopy.skirtFronds` zero) and no leaf that falls", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.05 } },
  { path: "/material/bloomFrontRed", label: "bloom front red", kind: "real", optional: false, meaning: "The colour of a petal's upper face, a linear albedo per channel, the same quantity as the leaf faces' and not a display (sRGB) value (fn-216, fn-234): the flowers' own, drawn by the leaf shader, and the colour the petal passes when lit from behind. A white petal is about 0.8 a channel and a gold one about 0.85/0.65/0.1.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/material/bloomFrontGreen", label: "bloom front green", kind: "real", optional: false, meaning: "The green channel of `bloom_front_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/material/bloomFrontBlue", label: "bloom front blue", kind: "real", optional: false, meaning: "The blue channel of `bloom_front_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/material/bloomBackRed", label: "bloom back red", kind: "real", optional: false, meaning: "The colour of a petal's underside, a linear albedo per channel as `bloom_front_red` is (fn-216): the flowers' own, drawn by the leaf shader.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/material/bloomBackGreen", label: "bloom back green", kind: "real", optional: false, meaning: "The green channel of `bloom_back_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/material/bloomBackBlue", label: "bloom back blue", kind: "real", optional: false, meaning: "The blue channel of `bloom_back_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/material/bloomEyeRed", label: "bloom eye red", kind: "real", optional: false, meaning: "A floret's eye: the colour its petals turn toward at their base, a linear reflectance on the petals' scale, and the stamens' colour.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering, or `bloomEyeShare` zero", reach: "mature", group: "flowers" },
  { path: "/material/bloomEyeGreen", label: "bloom eye green", kind: "real", optional: false, meaning: "The green channel of `bloom_eye_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering, or `bloomEyeShare` zero", reach: "mature", group: "flowers" },
  { path: "/material/bloomEyeBlue", label: "bloom eye blue", kind: "real", optional: false, meaning: "The blue channel of `bloom_eye_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering, or `bloomEyeShare` zero", reach: "mature", group: "flowers" },
  { path: "/material/bloomEyeShare", label: "bloom eye share", kind: "real", optional: false, meaning: "How far up each petal from its base the eye reaches, as a share of the petal's length. Zero is no eye: the petal and its stamens are the petal's colour.", unit: "share of the petal", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/material/hueRangeLow", label: "hue range low", kind: "real", optional: false, meaning: "The hue offsets one leaf may take, as a fraction of the colour circle.", unit: "-", low: -0.5, high: 0.5, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [-0.5, 0], step: 0.0025 } },
  { path: "/material/hueRangeHigh", label: "hue range high", kind: "real", optional: false, meaning: "The upper end of the hue offsets `hue_range_low` opens.", unit: "-", low: -0.5, high: 0.5, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 0.5], step: 0.0025 } },
  { path: "/material/brightnessRangeLow", label: "brightness range low", kind: "real", optional: false, meaning: "The brightness offsets one leaf may take, about no change at all.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [-1, 0], step: 0.015 } },
  { path: "/material/brightnessRangeHigh", label: "brightness range high", kind: "real", optional: false, meaning: "The upper end of the brightness offsets `brightness_range_low` opens.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.015 } },
  { path: "/material/interiorDarkening", label: "interior darkening", kind: "real", optional: false, meaning: "How far a leaf deep inside the crown is darkened towards a shaded mass.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/ridgeScale", label: "ridge scale", kind: "real", optional: false, meaning: "Circumferential ridge spacing in metres; zero disables relief.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.02 } },
  { path: "/material/plateScale", label: "plate scale", kind: "real", optional: false, meaning: "Axial scale control in metres; spacing is bounded to 1.5–2 ridge widths. Larger ratios lengthen and deepen furrows; zero omits breaks.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/furrowStrength", label: "furrow strength", kind: "real", optional: false, meaning: "Furrow width and depth together; zero leaves tightly packed scales.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/roughnessDetail", label: "roughness detail", kind: "real", optional: false, meaning: "Roughness variation about the base value, clamped to 0..1.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/veinScale", label: "vein scale", kind: "real", optional: false, meaning: "Secondary vein pairs per blade, continuously interpolated.", unit: "-", low: 0, high: 32, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 32], step: 0.5 } },
  { path: "/material/veinContrast", label: "vein contrast", kind: "real", optional: false, meaning: "How far the veins are lightened and the blade between them darkened. Raising it makes the venation read more sharply.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/transmissionStrength", label: "transmission strength", kind: "real", optional: false, meaning: "How brightly a backlit leaf glows with the light that came through it. Raising it lifts that glow.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/transmissionRed", label: "transmission red", kind: "real", optional: false, meaning: "Linear transmission tint, in 0..1 per channel.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`transmissionStrength` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/transmissionGreen", label: "transmission green", kind: "real", optional: false, meaning: "The green channel of `transmission_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`transmissionStrength` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/transmissionBlue", label: "transmission blue", kind: "real", optional: false, meaning: "The blue channel of `transmission_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`transmissionStrength` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.01 } },
  { path: "/material/thickness", label: "thickness", kind: "real", optional: false, meaning: "Optical thickness: attenuation is exp(-thickness).", unit: "-", low: 0, high: 8, lowOpen: false, zero: false, applies: "`transmissionStrength` zero", reach: "mature", group: "bark", dial: { window: [0, 8], step: 1 } },
  { path: "/material/fissureRed", label: "fissure red", kind: "real", optional: false, meaning: "The tint carried by the floors of the bark's furrows, as an offset per channel, and how far it is laid over the bark colour there.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`fissureStrength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/material/fissureGreen", label: "fissure green", kind: "real", optional: false, meaning: "The green channel of `fissure_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`fissureStrength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/material/fissureBlue", label: "fissure blue", kind: "real", optional: false, meaning: "The blue channel of `fissure_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`fissureStrength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/material/fissureStrength", label: "fissure strength", kind: "real", optional: false, meaning: "How far `fissure_red`'s tint is laid over the bark colour in the furrows.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/crestRed", label: "crest red", kind: "real", optional: false, meaning: "The tint carried by the crests of the bark's ridges, as an offset per channel, and how far it is laid over the bark colour there.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`crestStrength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.025 } },
  { path: "/material/crestGreen", label: "crest green", kind: "real", optional: false, meaning: "The green channel of `crest_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`crestStrength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.025 } },
  { path: "/material/crestBlue", label: "crest blue", kind: "real", optional: false, meaning: "The blue channel of `crest_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`crestStrength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.025 } },
  { path: "/material/crestStrength", label: "crest strength", kind: "real", optional: false, meaning: "How far `crest_red`'s tint is laid over the bark colour on the crests.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/barkMottleScale", label: "bark mottle scale", kind: "real", optional: false, meaning: "The size of the blotches in the bark's mottling, and how far they lighten and darken its colour. A scale of zero leaves none.", unit: "-", low: 0, high: 8, lowOpen: false, zero: false, applies: "`barkMottleStrength` zero", reach: "mature", group: "bark", dial: { window: [0, 8], step: 1 } },
  { path: "/material/barkMottleStrength", label: "bark mottle strength", kind: "real", optional: false, meaning: "How far the blotches `bark_mottle_scale` sizes lighten and darken the bark.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`barkMottleScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/cavityStrength", label: "cavity strength", kind: "real", optional: false, meaning: "How far the hollows of the bark - furrow floors and the sockets where a limb joins - are darkened. Raising it sinks them deeper.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/bladeMottleScale", label: "blade mottle scale", kind: "real", optional: false, meaning: "The size of the blotches in a leaf's mottling, and how far they vary its colour. A scale of zero leaves none.", unit: "-", low: 0, high: 32, lowOpen: false, zero: false, applies: "`bladeMottleStrength` zero", reach: "mature", group: "bark", dial: { window: [0, 32], step: 2 } },
  { path: "/material/bladeMottleStrength", label: "blade mottle strength", kind: "real", optional: false, meaning: "How far the blotches `blade_mottle_scale` sizes vary a leaf's colour.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`bladeMottleScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/marginWidth", label: "margin width", kind: "real", optional: false, meaning: "How wide a band along the leaf's edge takes the margin colour, as a share of the leaf set in leaf space, and what that colour is as an offset per channel. Raising the width broadens the rim around every leaf; a leaf under a pixel takes the band's area share.", unit: "-", low: 0, high: 0.5, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 0.5], step: 0.1 } },
  { path: "/material/marginRed", label: "margin red", kind: "real", optional: false, meaning: "The margin colour's red channel, as an offset; `margin_width` sets its band.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`marginWidth` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.01 } },
  { path: "/material/marginGreen", label: "margin green", kind: "real", optional: false, meaning: "The margin colour's green channel, as an offset; `margin_width` sets its band.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`marginWidth` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.015 } },
  { path: "/material/marginBlue", label: "margin blue", kind: "real", optional: false, meaning: "The margin colour's blue channel, as an offset; `margin_width` sets its band.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`marginWidth` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.0025 } },
  { path: "/material/cuticleGloss", label: "cuticle gloss", kind: "real", optional: false, meaning: "How tight the highlight on a leaf's upper face is. Raising it draws the glint into a smaller, glossier spot.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/skyOcclusionStrength", label: "sky occlusion strength", kind: "real", optional: false, meaning: "How much of the sky is withheld from bark and leaves the deeper they sit in the crown. Raising it darkens the crown's interior.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/plateCellScale", label: "plate cell scale", kind: "real", optional: false, meaning: "Circumferential size of one bark plate in metres, before girth scales it. Zero leaves the field the ridges it has always been.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.02 } },
  { path: "/material/plateElongation", label: "plate elongation", kind: "real", optional: false, meaning: "How much longer a plate runs than it is wide: nought is as long as it is wide, one is twice as long.", unit: "-", low: 0, high: 16, lowOpen: false, zero: false, applies: "`plateCellScale` or `ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 16], step: 2.5 } },
  { path: "/material/plateDome", label: "plate dome", kind: "real", optional: false, meaning: "How far a plate's face rises from its own edge towards its middle. At zero a plate's face lies flat, and any rise starts the doming.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`plateCellScale` or `ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/plateEdgeLift", label: "plate edge lift", kind: "real", optional: false, meaning: "How far a plate's rim stands off the furrow it borders: the scale that lifts rather than the plate that sits flat. At zero a plate's rim lies flush with its furrow, and any rise starts the lift.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`plateCellScale` or `ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/plateFurrowWidth", label: "plate furrow width", kind: "real", optional: false, meaning: "How wide the flat floor of a furrow is cut, as a fraction of a plate's own width, so a bigger plate carries a wider furrow off one row. Zero leaves the hairline the network has always cut between two faces.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`plateCellScale` or `ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/plateEdgeShape", label: "plate edge shape", kind: "real", optional: false, meaning: "Blend from rounded plate edges to narrow chipped scales; independent of lichen/lenticel/peel.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/plateIdentity", label: "plate identity", kind: "real", optional: false, meaning: "How much of its own a plate keeps: how proud it stands, how it leans, and the value and cast it holds against its neighbours. At zero every plate stands and reads exactly like its neighbours, and any rise starts the difference.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`plateCellScale` or `ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/weatheringStrength", label: "weathering strength", kind: "real", optional: false, meaning: "How far a weathered face is greyed and tinted against a fresh furrow. At zero no face is weathered at all, and any rise starts the greying.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/weatheringRed", label: "weathering red", kind: "real", optional: false, meaning: "The tint a weathered face takes, as an offset per channel; how far it is laid on is `weathering_strength` above.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`weatheringStrength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.005 } },
  { path: "/material/weatheringGreen", label: "weathering green", kind: "real", optional: false, meaning: "The green channel of `weathering_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`weatheringStrength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.01 } },
  { path: "/material/weatheringBlue", label: "weathering blue", kind: "real", optional: false, meaning: "The blue channel of `weathering_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`weatheringStrength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.01 } },
  { path: "/material/orientationStrength", label: "orientation strength", kind: "real", optional: false, meaning: "How far the side away from the sun and the foot of the trunk take a colour of their own - what damp growth would look like, not what it is. At zero the shaded side and the foot take no colour of their own, and any rise starts it.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/orientationRed", label: "orientation red", kind: "real", optional: false, meaning: "The tint that damp side takes, as an offset per channel; how far it is laid on is `orientation_strength` above.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`orientationStrength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.02 } },
  { path: "/material/orientationGreen", label: "orientation green", kind: "real", optional: false, meaning: "The green channel of `orientation_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`orientationStrength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.01 } },
  { path: "/material/orientationBlue", label: "orientation blue", kind: "real", optional: false, meaning: "The blue channel of `orientation_red`.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`orientationStrength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.05 } },
  { path: "/material/directionalOcclusion", label: "directional occlusion", kind: "real", optional: false, meaning: "How far a furrow floor is darkened by its own crest standing between it and the sun. Zero leaves the sun on both sides of every furrow alike.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/depthStrength", label: "depth strength", kind: "real", optional: false, meaning: "How far the relief is given depth beyond the shaded normal. At zero the relief is given no depth beyond that normal, and any rise starts it.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/canopyNormal", label: "canopy normal", kind: "real", optional: false, meaning: "How far a leaf is lit as part of its crown rather than as a lone card: its lighting normal bends from the blade's toward the crown's outward direction at its placement, so the sunward shell of the mass is lit whichever way its blades turn. Zero lights the blade alone.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lightWrap", label: "light wrap", kind: "real", optional: false, meaning: "How far the leaf's sunlight wraps past the terminator, as a fraction of a right angle's cosine; a face square to the sun takes what it always took. Zero is the plain cosine.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/diffuseTransmission", label: "diffuse transmission", kind: "real", optional: false, meaning: "The share of the blade's transmission that leaves it diffusely, as a thin leaf's does, rather than on the forward lobe toward the sun; the diffuse share also carries the sky through the blade. Zero is the lobe.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/leafSheen", label: "leaf sheen", kind: "real", optional: false, meaning: "The cuticle's reflectance of the sky at normal incidence, rising to the whole sky at grazing by Schlick's Fresnel; zero reflects no sky.", unit: "-", low: 0, high: 0.5, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 0.5], step: 0.1 } },
  { path: "/material/crownShade", label: "crown shade", kind: "real", optional: false, meaning: "How much of the sky one crown radius of leaves takes from a leaf that reads it through the mass - the sky over it, behind it and in its sheen - so the underside of a crown falls into its own shade. Zero sees the sky through the mass.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lichenScale", label: "lichen scale", kind: "real", optional: false, meaning: "Smooth bark's lichen: the size in metres of the cells its patches are scattered over. Zero leaves no patch anywhere.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lichenStrength` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.01 } },
  { path: "/material/lichenCoverage", label: "lichen coverage", kind: "real", optional: false, meaning: "The share of those cells that hold a patch.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lichenStrength` or `lichenScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lichenRed", label: "lichen red", kind: "real", optional: false, meaning: "A patch's own colour, a linear reflectance like the bark's.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lichenStrength` or `lichenScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lichenGreen", label: "lichen green", kind: "real", optional: false, meaning: "The green channel of `lichen_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lichenStrength` or `lichenScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lichenBlue", label: "lichen blue", kind: "real", optional: false, meaning: "The blue channel of `lichen_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lichenStrength` or `lichenScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lichenStrength", label: "lichen strength", kind: "real", optional: false, meaning: "How far a patch covers the bark with that colour.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lichenScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lenticelDensity", label: "lenticel density", kind: "real", optional: false, meaning: "Rows of lenticel dashes per metre along the wood.", unit: "-", low: 0, high: 400, lowOpen: false, zero: false, applies: "`lenticelStrength` or `lenticelLength` zero", reach: "mature", group: "bark", dial: { window: [0, 400], step: 5 } },
  { path: "/material/lenticelLength", label: "lenticel length", kind: "real", optional: false, meaning: "The longest dash across the wood, in metres; the shortest is under half.", unit: "m", low: 0, high: 0.5, lowOpen: false, zero: false, applies: "`lenticelStrength` zero", reach: "mature", group: "bark", dial: { window: [0, 0.5], step: 0.1 } },
  { path: "/material/lenticelStrength", label: "lenticel strength", kind: "real", optional: false, meaning: "How far a dash shows: its tint, and the shallow groove it cuts.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`lenticelLength` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lenticelTint", label: "lenticel tint", kind: "real", optional: false, meaning: "A dash's value against the bark it marks: -1 is black, 0 no change.", unit: "-", low: -1, high: 1, lowOpen: false, zero: false, applies: "`lenticelStrength` or `lenticelLength` zero", reach: "mature", group: "bark", dial: { window: [-1, 1], step: 0.25 } },
  { path: "/material/peelCurl", label: "peel curl", kind: "real", optional: false, meaning: "How far the plate network's strips curl away: they stretch across the wood into bands, lift at their lower edge, and this share of them has peeled off to show the inner bark. At zero no strip has peeled at all, and any rise switches the peel on.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/peelRed", label: "peel red", kind: "real", optional: false, meaning: "The inner bark a peeled strip leaves showing, a linear reflectance like the bark's.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`peelCurl` or `ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/peelGreen", label: "peel green", kind: "real", optional: false, meaning: "The green channel of `peel_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`peelCurl` or `ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/peelBlue", label: "peel blue", kind: "real", optional: false, meaning: "The blue channel of `peel_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`peelCurl` or `ridgeScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/lobeShade", label: "lobe shade", kind: "real", optional: false, meaning: "How much of the sky and of what passes through the blade a leaf loses to the leaves of its own lobe standing over it, read from the crown's own placements rather than from one smooth ellipsoid: a lobe's face is lit and what hangs under it falls into its shade. Zero sees none of it.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/barkReflectance", label: "bark reflectance", kind: "real", optional: false, meaning: "What the bark mirrors of the sun at normal incidence, the foot of its one highlight by Schlick's Fresnel: 0.04 is a dielectric. What the highlight mirrors is taken from the diffuse; zero mirrors none.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/leafReflectance", label: "leaf reflectance", kind: "real", optional: false, meaning: "The same for the cuticle on the blade's front face; the highlight's width follows `cuticle_gloss`.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/barkGrainScale", label: "bark grain scale", kind: "real", optional: false, meaning: "A grain below the relief: the size in metres of its cells. Zero leaves the bark smooth between the relief's features.", unit: "m", low: 0, high: 0.05, lowOpen: false, zero: false, applies: "`barkGrainStrength` zero", reach: "mature", group: "bark", dial: { window: [0, 0.05], step: 0.0005 } },
  { path: "/material/barkGrainStrength", label: "bark grain strength", kind: "real", optional: false, meaning: "How far the grain varies the bark's colour and tilts its normal.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`barkGrainScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/material/bladeGrainScale", label: "blade grain scale", kind: "real", optional: false, meaning: "The blade's cell grain, in cells per blade length; zero is none.", unit: "-", low: 0, high: 256, lowOpen: false, zero: false, applies: "`bladeGrainStrength` zero", reach: "mature", group: "bark", dial: { window: [0, 256], step: 20 } },
  { path: "/material/bladeGrainStrength", label: "blade grain strength", kind: "real", optional: false, meaning: "How far that grain varies the blade's colour and tilts its normal.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`bladeGrainScale` zero", reach: "mature", group: "bark", dial: { window: [0, 1], step: 0.15 } },
  { path: "/bloom/florets", label: "florets", kind: "real", optional: false, meaning: "Florets on one inflorescence. The fractional part is the last floret's size, so a walk across counts grows one in by degree.", unit: "florets", low: 1, high: 64, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/rachis", label: "rachis", kind: "real", optional: false, meaning: "Metres of the inflorescence's axis the florets stand along: zero is a cluster at the node, a few centimetres a raceme or catkin.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/droop", label: "droop", kind: "real", optional: false, meaning: "How the axis turns from the bud's side as it grows: 1 hangs straight down, -1 stands up as a candle, 0 holds the side.", unit: "share", low: -1, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/pedicel", label: "pedicel", kind: "real", optional: false, meaning: "Metres of stalk between the axis and each floret.", unit: "m", low: 0, high: 0.5, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/spread", label: "spread", kind: "real", optional: false, meaning: "The degrees a floret's stalk leaves the axis: small packs a spike, ninety spreads a raceme, more fans a cluster back.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/taper", label: "taper", kind: "real", optional: false, meaning: "How much smaller the floret at the axis's tip is than the one at its base, as a share.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/petals", label: "petals", kind: "real", optional: false, meaning: "Petals on a floret, turned evenly about it by the count. The fractional part is the last petal's size (host decision 4), so a walk across counts changes a flower by degree.", unit: "petals", low: 1, high: 12, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/open", label: "open", kind: "real", optional: false, meaning: "The degrees a petal opens from the floret's axis: a bud near zero, an open blossom near ninety.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/petalLength", label: "petal length", kind: "real", optional: false, meaning: "A petal's length in metres.", unit: "m", low: 0.0001, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/petalWidth", label: "petal width", kind: "real", optional: false, meaning: "A petal's greatest width in metres.", unit: "m", low: 0.0001, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/petalWidest", label: "petal widest", kind: "real", optional: false, meaning: "Where a petal is widest, as a share from its base to its tip.", unit: "share of the petal", low: 0.05, high: 0.95, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/petalCup", label: "petal cup", kind: "real", optional: false, meaning: "How far a petal's margins lift out of its plane, as a share of its half-width: a cupped blossom.", unit: "share of the half-width", low: -2, high: 2, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/petalCurl", label: "petal curl", kind: "real", optional: false, meaning: "How far a petal bends along its length, as a share of its length at the tip: positive turns the tip out, away from the floret's axis, so a narrow-opened floret flares into a horn's mouth.", unit: "share of the length", low: -2, high: 2, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/stamens", label: "stamens", kind: "real", optional: false, meaning: "Stamens in a floret, a ring of filaments standing up inside its petals. The fractional part is the last stamen's size, so a walk across counts grows one in by degree; zero is none.", unit: "stamens", low: 0, high: 64, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/stamenLength", label: "stamen length", kind: "real", optional: false, meaning: "A stamen's length in metres, from the floret's centre.", unit: "m", low: 0.0001, high: 0.1, lowOpen: false, zero: false, applies: "no age flowering, or `stamens` zero", reach: "mature", group: "flowers" },
  { path: "/bloom/stamenWidth", label: "stamen width", kind: "real", optional: false, meaning: "A stamen's width in metres.", unit: "m", low: 0.00001, high: 0.01, lowOpen: false, zero: false, applies: "no age flowering, or `stamens` zero", reach: "mature", group: "flowers" },
  { path: "/bloom/scatter", label: "scatter", kind: "real", optional: false, meaning: "How far a floret's size varies from one to the next, as a share.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/opens", label: "opens", kind: "real", optional: false, meaning: "Where in the year the flowering window opens, as a share of the year from its start.", unit: "share of the year", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/lasts", label: "lasts", kind: "real", optional: false, meaning: "How long the window lasts, as a share of the year. A floret grows in and shrinks out inside it; zero flowers never.", unit: "share of the year", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/bloom/stagger", label: "stagger", kind: "real", optional: false, meaning: "The share of the window the florets' openings are spread over: zero opens every floret at once.", unit: "share of the window", low: 0, high: 1, lowOpen: false, zero: false, applies: "no age flowering", reach: "mature", group: "flowers" },
  { path: "/supernatural/enabled", label: "enabled", kind: "switch", optional: false, meaning: "Whether the legendary traits act. Off, the group reads as none: the scale is one, the reflectances zero and every override leaves its natural row as it is.", unit: "switch", low: 0, high: 1, lowOpen: false, zero: false, applies: "every other `/supernatural` row unset or at its natural value", reach: "mature", group: "supernatural" },
  { path: "/supernatural/scale", label: "scale", kind: "real", optional: false, meaning: "How many times its natural size the tree is drawn: every position and length of the grown wood, after the engine grew it at its natural size, so its topology is the natural tree's. Leaves and flowers keep their own size; the leaves keep their spacing along the wood. One is the natural tree.", unit: "times the natural size", low: 0.1, high: 100, lowOpen: false, zero: false, applies: "`enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/backReflectance", label: "back reflectance", kind: "real", optional: false, meaning: "What a leaf's underside mirrors at normal incidence, as a metal does: the sun's glint and the sky's sheen off the back of the blade, tinted by the back tint, taken from the face's diffuse. Zero mirrors nothing and leaves the back as the natural leaf's.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/backRed", label: "back red", kind: "real", optional: false, meaning: "The colour the underside mirrors in, a linear reflectance per channel: a metal's reflection is its colour. Silver is near white.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`enabled` off, or `backReflectance` zero", reach: "mature", group: "supernatural" },
  { path: "/supernatural/backGreen", label: "back green", kind: "real", optional: false, meaning: "The green channel of `back_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`enabled` off, or `backReflectance` zero", reach: "mature", group: "supernatural" },
  { path: "/supernatural/backBlue", label: "back blue", kind: "real", optional: false, meaning: "The blue channel of `back_red`.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`enabled` off, or `backReflectance` zero", reach: "mature", group: "supernatural" },
  { path: "/supernatural/marginReflectance", label: "margin reflectance", kind: "real", optional: false, meaning: "What a leaf's margin band mirrors at normal incidence, on either face, tinted by the margin's colour: a metallic rim, taken from the band's diffuse. Zero mirrors nothing.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`enabled` off, or `/material/marginWidth` zero", reach: "mature", group: "supernatural" },
  { path: "/supernatural/roughness", label: "roughness", kind: "real", optional: false, meaning: "How rough the back and the margin are as mirrors: low is a sharp glint, high a broad sheen.", unit: "-", low: 0, high: 1, lowOpen: false, zero: false, applies: "`enabled` off, or both reflectances zero", reach: "mature", group: "supernatural" },
  { path: "/supernatural/material/leafFrontRed", label: "leaf front red", kind: "real", optional: true, meaning: "`/material/leafFrontRed` while the group is on; unset, the natural row.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/material/leafFrontGreen", label: "leaf front green", kind: "real", optional: true, meaning: "`/material/leafFrontGreen` while the group is on; unset, the natural row.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/material/leafFrontBlue", label: "leaf front blue", kind: "real", optional: true, meaning: "`/material/leafFrontBlue` while the group is on; unset, the natural row.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/material/leafBackRed", label: "leaf back red", kind: "real", optional: true, meaning: "`/material/leafBackRed` while the group is on; unset, the natural row.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/material/leafBackGreen", label: "leaf back green", kind: "real", optional: true, meaning: "`/material/leafBackGreen` while the group is on; unset, the natural row.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/material/leafBackBlue", label: "leaf back blue", kind: "real", optional: true, meaning: "`/material/leafBackBlue` while the group is on; unset, the natural row.", unit: "reflectance", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/material/marginRed", label: "margin red", kind: "real", optional: true, meaning: "`/material/marginRed` while the group is on; unset, the natural row.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/material/marginGreen", label: "margin green", kind: "real", optional: true, meaning: "`/material/marginGreen` while the group is on; unset, the natural row.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/material/marginBlue", label: "margin blue", kind: "real", optional: true, meaning: "`/material/marginBlue` while the group is on; unset, the natural row.", unit: "offset", low: -1, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/florets", label: "florets", kind: "real", optional: true, meaning: "`/bloom/florets` while the group is on; unset, the natural row.", unit: "florets", low: 1, high: 64, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/rachis", label: "rachis", kind: "real", optional: true, meaning: "`/bloom/rachis` while the group is on; unset, the natural row.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/droop", label: "droop", kind: "real", optional: true, meaning: "`/bloom/droop` while the group is on; unset, the natural row.", unit: "share", low: -1, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/pedicel", label: "pedicel", kind: "real", optional: true, meaning: "`/bloom/pedicel` while the group is on; unset, the natural row.", unit: "m", low: 0, high: 0.5, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/spread", label: "spread", kind: "real", optional: true, meaning: "`/bloom/spread` while the group is on; unset, the natural row.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/taper", label: "taper", kind: "real", optional: true, meaning: "`/bloom/taper` while the group is on; unset, the natural row.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/petals", label: "petals", kind: "real", optional: true, meaning: "`/bloom/petals` while the group is on; unset, the natural row.", unit: "petals", low: 1, high: 12, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/open", label: "open", kind: "real", optional: true, meaning: "`/bloom/open` while the group is on; unset, the natural row.", unit: "degrees", low: 0, high: 180, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/petalLength", label: "petal length", kind: "real", optional: true, meaning: "`/bloom/petalLength` while the group is on; unset, the natural row.", unit: "m", low: 0.0001, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/petalWidth", label: "petal width", kind: "real", optional: true, meaning: "`/bloom/petalWidth` while the group is on; unset, the natural row.", unit: "m", low: 0.0001, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/petalWidest", label: "petal widest", kind: "real", optional: true, meaning: "`/bloom/petalWidest` while the group is on; unset, the natural row.", unit: "share of the petal", low: 0.05, high: 0.95, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/petalCup", label: "petal cup", kind: "real", optional: true, meaning: "`/bloom/petalCup` while the group is on; unset, the natural row.", unit: "share of the half-width", low: -2, high: 2, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/petalCurl", label: "petal curl", kind: "real", optional: true, meaning: "`/bloom/petalCurl` while the group is on; unset, the natural row.", unit: "share of the length", low: -2, high: 2, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/stamens", label: "stamens", kind: "real", optional: true, meaning: "`/bloom/stamens` while the group is on; unset, the natural row.", unit: "stamens", low: 0, high: 64, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/stamenLength", label: "stamen length", kind: "real", optional: true, meaning: "`/bloom/stamenLength` while the group is on; unset, the natural row.", unit: "m", low: 0.0001, high: 0.1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/stamenWidth", label: "stamen width", kind: "real", optional: true, meaning: "`/bloom/stamenWidth` while the group is on; unset, the natural row.", unit: "m", low: 0.00001, high: 0.01, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/scatter", label: "scatter", kind: "real", optional: true, meaning: "`/bloom/scatter` while the group is on; unset, the natural row.", unit: "share", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/opens", label: "opens", kind: "real", optional: true, meaning: "`/bloom/opens` while the group is on; unset, the natural row.", unit: "share of the year", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/lasts", label: "lasts", kind: "real", optional: true, meaning: "`/bloom/lasts` while the group is on; unset, the natural row.", unit: "share of the year", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/bloom/stagger", label: "stagger", kind: "real", optional: true, meaning: "`/bloom/stagger` while the group is on; unset, the natural row.", unit: "share of the window", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/canopy/size", label: "size", kind: "real", optional: true, meaning: "`/canopy/size` while the group is on; unset, the natural row.", unit: "multiple of the element", low: 0, high: 1000, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age0", label: "age0", kind: "real", optional: true, meaning: "`/space/ages/0/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age1", label: "age1", kind: "real", optional: true, meaning: "`/space/ages/1/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age2", label: "age2", kind: "real", optional: true, meaning: "`/space/ages/2/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age3", label: "age3", kind: "real", optional: true, meaning: "`/space/ages/3/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age4", label: "age4", kind: "real", optional: true, meaning: "`/space/ages/4/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age5", label: "age5", kind: "real", optional: true, meaning: "`/space/ages/5/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age6", label: "age6", kind: "real", optional: true, meaning: "`/space/ages/6/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age7", label: "age7", kind: "real", optional: true, meaning: "`/space/ages/7/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age8", label: "age8", kind: "real", optional: true, meaning: "`/space/ages/8/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age9", label: "age9", kind: "real", optional: true, meaning: "`/space/ages/9/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age10", label: "age10", kind: "real", optional: true, meaning: "`/space/ages/10/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age11", label: "age11", kind: "real", optional: true, meaning: "`/space/ages/11/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age12", label: "age12", kind: "real", optional: true, meaning: "`/space/ages/12/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age13", label: "age13", kind: "real", optional: true, meaning: "`/space/ages/13/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/flowering/age14", label: "age14", kind: "real", optional: true, meaning: "`/space/ages/14/flowering` while the group is on; unset, the natural share.", unit: "share of the age's nodes", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age0", label: "age0", kind: "real", optional: true, meaning: "`/space/ages/0/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age1", label: "age1", kind: "real", optional: true, meaning: "`/space/ages/1/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age2", label: "age2", kind: "real", optional: true, meaning: "`/space/ages/2/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age3", label: "age3", kind: "real", optional: true, meaning: "`/space/ages/3/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age4", label: "age4", kind: "real", optional: true, meaning: "`/space/ages/4/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age5", label: "age5", kind: "real", optional: true, meaning: "`/space/ages/5/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age6", label: "age6", kind: "real", optional: true, meaning: "`/space/ages/6/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age7", label: "age7", kind: "real", optional: true, meaning: "`/space/ages/7/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age8", label: "age8", kind: "real", optional: true, meaning: "`/space/ages/8/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age9", label: "age9", kind: "real", optional: true, meaning: "`/space/ages/9/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age10", label: "age10", kind: "real", optional: true, meaning: "`/space/ages/10/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age11", label: "age11", kind: "real", optional: true, meaning: "`/space/ages/11/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age12", label: "age12", kind: "real", optional: true, meaning: "`/space/ages/12/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age13", label: "age13", kind: "real", optional: true, meaning: "`/space/ages/13/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/internode/age14", label: "age14", kind: "real", optional: true, meaning: "`/space/ages/14/internode` while the group is on; unset, the natural internode length.", unit: "m", low: 0.000001, high: 100, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age0", label: "age0", kind: "real", optional: true, meaning: "`/space/ages/0/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age1", label: "age1", kind: "real", optional: true, meaning: "`/space/ages/1/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age2", label: "age2", kind: "real", optional: true, meaning: "`/space/ages/2/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age3", label: "age3", kind: "real", optional: true, meaning: "`/space/ages/3/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age4", label: "age4", kind: "real", optional: true, meaning: "`/space/ages/4/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age5", label: "age5", kind: "real", optional: true, meaning: "`/space/ages/5/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age6", label: "age6", kind: "real", optional: true, meaning: "`/space/ages/6/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age7", label: "age7", kind: "real", optional: true, meaning: "`/space/ages/7/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age8", label: "age8", kind: "real", optional: true, meaning: "`/space/ages/8/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age9", label: "age9", kind: "real", optional: true, meaning: "`/space/ages/9/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age10", label: "age10", kind: "real", optional: true, meaning: "`/space/ages/10/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age11", label: "age11", kind: "real", optional: true, meaning: "`/space/ages/11/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age12", label: "age12", kind: "real", optional: true, meaning: "`/space/ages/12/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age13", label: "age13", kind: "real", optional: true, meaning: "`/space/ages/13/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
  { path: "/supernatural/pipe/age14", label: "age14", kind: "real", optional: true, meaning: "`/space/ages/14/form/pipe` while the group is on; unset, the natural pipe a phytomer adds.", unit: "m", low: 0, high: 1, lowOpen: false, zero: false, applies: "`/supernatural/enabled` off", reach: "mature", group: "supernatural" },
];

/** One engine setting, as a control over every age a species states. */
export interface SpaceSetting {
  /** The engine's name for it, `form.bend_length`. */
  name: string;
  /** Its place under an age's rows, or under a zone's where `zone`: `form/bendLength`. */
  key: string;
  zone: boolean;
  label: string;
  /** How a control moves it: `log` multiplies (above `floor`), `linear` offsets,
   *  `kept` offsets the share 1 / (1 + value) it keeps, `outcome` offsets a share. */
  scale: "linear" | "log" | "kept" | "outcome";
  floor: number;
  low: number;
  high: number;
  lowOpen: boolean;
  /** Admits `never`, an unbounded span, on the wire. */
  never: boolean;
  /** Its value at an age or zone that states none. */
  neutral: number;
  unit: string;
  meaning: string;
  group: Group;
}

export const SPACE_SETTINGS: readonly SpaceSetting[] = [
  { name: "lifespan", key: "lifespan", zone: false, label: "lifespan", scale: "log", floor: 1, low: 0, high: 1000000, lowOpen: false, never: false, neutral: 0, unit: "years", meaning: "The years an apex grows in this age before it moves on. Dormant while the tree is younger than it.", group: "size" },
  { name: "continuation", key: "continuation", zone: false, label: "continuation", scale: "outcome", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 1, unit: "probability", meaning: "The chance an apex that has spent its lifespan moves on to the next age. Dormant at the last age.", group: "size" },
  { name: "viability", key: "viability", zone: false, label: "viability", scale: "outcome", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 1, unit: "probability", meaning: "The chance the apex survives each year.", group: "size" },
  { name: "shedding", key: "shedding", zone: false, label: "shedding", scale: "kept", floor: 0, low: 0, high: Infinity, lowOpen: false, never: true, neutral: Infinity, unit: "years", meaning: "Years a lateral with no living apex is kept before it is shed; never keeps it. Dormant on an axis that bears no laterals.", group: "branching" },
  { name: "internode", key: "internode", zone: false, label: "internode", scale: "log", floor: 0.001, low: 0, high: 100, lowOpen: true, never: false, neutral: 0.04, unit: "m", meaning: "The internode length.", group: "size" },
  { name: "insertion", key: "insertion", zone: false, label: "insertion", scale: "linear", floor: 0, low: 0, high: 3.141592653589793, lowOpen: false, never: false, neutral: 0, unit: "rad", meaning: "The angle between an axis of this age and its parent. Dormant on an axis that bears no laterals.", group: "branching" },
  { name: "divergence", key: "divergence", zone: false, label: "divergence", scale: "linear", floor: 0, low: -6.283185307179586, high: 6.283185307179586, lowOpen: false, never: false, neutral: 3.141592653589793, unit: "rad", meaning: "The angle between successive nodes around the axis (phyllotaxis). Dormant on an axis that bears no laterals.", group: "branching" },
  { name: "abortion", key: "abortion", zone: false, label: "abortion", scale: "outcome", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "probability", meaning: "The chance the apex aborts after each growth unit, so its laterals carry on.", group: "branching" },
  { name: "abortion_rise", key: "abortionRise", zone: false, label: "abortion rise", scale: "linear", floor: 0, low: 0, high: 8, lowOpen: false, never: false, neutral: 0, unit: "-", meaning: "How the abortion chance rises with the units an axis has grown. Dormant without abortion.", group: "branching" },
  { name: "relay", key: "relay", zone: false, label: "relay", scale: "outcome", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "probability", meaning: "The chance an aborted apex is replaced by a relay bud of its own age. Dormant without abortion.", group: "branching" },
  { name: "relay_ended", key: "relayEnded", zone: false, label: "relay ended", scale: "outcome", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "probability", meaning: "The same, for an apex whose age ends and does not move on. Dormant where every age moves on.", group: "branching" },
  { name: "relay_failed", key: "relayFailed", zone: false, label: "relay failed", scale: "outcome", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "probability", meaning: "The same, for an apex whose growth unit failed. Dormant where every apex survives.", group: "branching" },
  { name: "relay_at", key: "relayAt", zone: false, label: "relay at", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 1, unit: "share of the unit", meaning: "Where along the stopped axis's last unit a relay bud stands. Dormant without relays.", group: "branching" },
  { name: "epitony", key: "epitony", zone: false, label: "epitony", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "share", meaning: "How far a relay bud turns to the upper side of its parent. Dormant without relays.", group: "branching" },
  { name: "takeover", key: "takeover", zone: false, label: "takeover", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "share", meaning: "How far a relay's base runs back onto the line of the axis it takes over. Dormant without relays.", group: "branching" },
  { name: "erection", key: "erection", zone: false, label: "erection", scale: "log", floor: 0.01, low: 0, high: 100, lowOpen: false, never: false, neutral: 0, unit: "per year", meaning: "How fast an axis's base straightens further towards the vertical with age.", group: "form" },
  { name: "readiness", key: "readiness", zone: false, label: "readiness", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 1, unit: "share", meaning: "How ready the axis is to branch; zero is an unbranched stem. Dormant without laterals.", group: "branching" },
  { name: "rhythm", key: "rhythm", zone: false, label: "rhythm", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 1, unit: "share", meaning: "Rhythmic growth: 1 keeps each zone's laterals, 0 spreads them evenly. Dormant in a unit of one zone.", group: "branching" },
  { name: "straightening", key: "straightening", zone: false, label: "straightening", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "share", meaning: "How far a lateral's base straightens towards the vertical. Dormant on an axis that bears no laterals.", group: "form" },
  { name: "leaf_area", key: "leafArea", zone: false, label: "leaf area", scale: "log", floor: 0.0001, low: 0, high: 5, lowOpen: false, never: false, neutral: 0, unit: "m²", meaning: "The leaf area a node bears in its year, which shades the light. Dormant where no leaf shades another.", group: "leaves" },
  { name: "flowering", key: "flowering", zone: false, label: "flowering", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "share of nodes", meaning: "The share of nodes that bear an inflorescence.", group: "flowers" },
  { name: "cluster_leaves", key: "clusterLeaves", zone: false, label: "cluster leaves", scale: "linear", floor: 0, low: 0, high: 8, lowOpen: false, never: false, neutral: 0, unit: "leaves", meaning: "Leaves clustered at each axis's tip, as a short shoot bears them.", group: "leaves" },
  { name: "cluster_spread", key: "clusterSpread", zone: false, label: "cluster spread", scale: "linear", floor: 0, low: 0, high: 1.5707963267948966, lowOpen: false, never: false, neutral: 0, unit: "rad", meaning: "How far a tip cluster's leaves fan either side of its level. Dormant without tip clusters.", group: "leaves" },
  { name: "leaf_years", key: "leafYears", zone: false, label: "leaf years", scale: "kept", floor: 0, low: 0, high: Infinity, lowOpen: false, never: true, neutral: Infinity, unit: "years", meaning: "How many years wood of this age bears its leaves; never, every age bears.", group: "leaves" },
  { name: "elongation", key: "elongation", zone: false, label: "elongation", scale: "log", floor: 1, low: 0, high: 365, lowOpen: false, never: false, neutral: 0, unit: "days", meaning: "Days a growth unit takes to elongate from bud burst.", group: "size" },
  { name: "expansion", key: "expansion", zone: false, label: "expansion", scale: "log", floor: 1, low: 0, high: 365, lowOpen: false, never: false, neutral: 0, unit: "days", meaning: "Days a leaf takes to expand to its whole size.", group: "leaves" },
  { name: "leaf_life", key: "leafLife", zone: false, label: "leaf life", scale: "kept", floor: 0, low: 0, high: Infinity, lowOpen: false, never: false, neutral: Infinity, unit: "years", meaning: "The years a leaf lives before it colours and falls; never, none falls. Dormant on the grown tree; drawn as the year plays.", group: "leaves" },
  { name: "budburst", key: "budburst", zone: false, label: "budburst", scale: "linear", floor: 0, low: 0, high: 365, lowOpen: false, never: false, neutral: 0, unit: "day of year", meaning: "The day of the year the buds burst. Dormant on the grown tree, and while no leaf falls.", group: "leaves" },
  { name: "senescence", key: "senescence", zone: false, label: "senescence", scale: "linear", floor: 0, low: 0, high: 365, lowOpen: false, never: false, neutral: 0, unit: "day of year", meaning: "The day of the year falling leaves start to colour. Dormant on the grown tree, and while no leaf falls.", group: "leaves" },
  { name: "colouring", key: "colouring", zone: false, label: "colouring", scale: "log", floor: 1, low: 0, high: 365, lowOpen: false, never: false, neutral: 0, unit: "days", meaning: "Days the falling leaves take to colour. Dormant on the grown tree, and while no leaf falls.", group: "leaves" },
  { name: "leaf_fall", key: "leafFall", zone: false, label: "leaf fall", scale: "log", floor: 1, low: 0, high: 365, lowOpen: false, never: false, neutral: 0, unit: "days", meaning: "Days the coloured leaves take to fall. Dormant on the grown tree, and while no leaf falls.", group: "leaves" },
  { name: "shade_hazard", key: "shadeHazard", zone: false, label: "shade hazard", scale: "linear", floor: 0, low: 0, high: 10, lowOpen: false, never: false, neutral: 0, unit: "exponent", meaning: "How shade raises an apex's yearly death hazard. Dormant where every apex survives or no leaf shades.", group: "branching" },
  { name: "ground_hazard", key: "groundHazard", zone: false, label: "ground hazard", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "share", meaning: "How the ground ends an apex whose tip rests on it. Dormant where no tip reaches the ground.", group: "branching" },
  { name: "shade_size", key: "shadeSize", zone: false, label: "shade size", scale: "linear", floor: 0, low: 0, high: 10, lowOpen: false, never: false, neutral: 0, unit: "exponent", meaning: "How a growth unit's size follows the light its bud gets. Dormant where no leaf shades.", group: "branching" },
  { name: "apical_control", key: "apicalControl", zone: false, label: "apical control", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0.5, unit: "share", meaning: "The share of vigour the continuing axis keeps against its laterals. Dormant without shade size.", group: "branching" },
  { name: "upkeep", key: "upkeep", zone: false, label: "upkeep", scale: "log", floor: 0.001, low: 0, high: 10, lowOpen: false, never: false, neutral: 0, unit: "light per metre", meaning: "What a metre of present wood costs in light. Dormant without a balance hazard.", group: "branching" },
  { name: "balance_hazard", key: "balanceHazard", zone: false, label: "balance hazard", scale: "log", floor: 0.01, low: 0, high: 10, lowOpen: false, never: false, neutral: 0, unit: "per year", meaning: "How fast a lateral whose carbon balance falls below tolerance is shed. Dormant without upkeep, and on an axis that bears no laterals.", group: "branching" },
  { name: "tolerance", key: "tolerance", zone: false, label: "tolerance", scale: "linear", floor: 0, low: -1, high: 1, lowOpen: false, never: false, neutral: 0, unit: "balance", meaning: "The carbon balance below which a lateral starts to be shed: shade tolerance. Dormant without a balance hazard.", group: "branching" },
  { name: "retained", key: "retained", zone: false, label: "retained", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "share", meaning: "The share of a shed branch's pipe that stays in its bearer's girth. Dormant where nothing is shed.", group: "form" },
  { name: "leaf_girth", key: "leafGirth", zone: false, label: "leaf girth", scale: "linear", floor: 0, low: 0, high: 10, lowOpen: false, never: false, neutral: 0, unit: "exponent", meaning: "How a phytomer's own pipe follows its leaves' light. Dormant where no leaf shades.", group: "form" },
  { name: "form.tropism", key: "form/tropism", zone: false, label: "tropism", scale: "log", floor: 0.1, low: 0, high: 100, lowOpen: false, never: false, neutral: 0, unit: "per m", meaning: "How fast the axis bends towards its elevation.", group: "form" },
  { name: "form.elevation", key: "form/elevation", zone: false, label: "elevation", scale: "linear", floor: 0, low: -1.5707963267948966, high: 1.5707963267948966, lowOpen: false, never: false, neutral: 0, unit: "rad", meaning: "The elevation the axis bends towards, above the horizontal.", group: "form" },
  { name: "form.wander", key: "form/wander", zone: false, label: "wander", scale: "log", floor: 0.05, low: 0, high: 100, lowOpen: false, never: false, neutral: 0, unit: "rad per m", meaning: "How far each node turns the axis at random: sinuosity.", group: "form" },
  { name: "form.plane", key: "form/plane", zone: false, label: "plane", scale: "linear", floor: 0, low: -6.283185307179586, high: 6.283185307179586, lowOpen: false, never: false, neutral: 0, unit: "rad", meaning: "The turn of the plane a lateral of this axis branches in. Dormant on an axis that bears no laterals.", group: "branching" },
  { name: "form.pipe", key: "form/pipe", zone: false, label: "pipe", scale: "log", floor: 0.00001, low: 0, high: 1, lowOpen: false, never: false, neutral: 0.005, unit: "m", meaning: "The pipe radius each phytomer adds below it: how thick the wood grows.", group: "form" },
  { name: "form.exponent", key: "form/exponent", zone: false, label: "exponent", scale: "linear", floor: 0, low: 1.5, high: 4, lowOpen: false, never: false, neutral: 2, unit: "-", meaning: "The pipe model's exponent; larger keeps limbs thicker beside their bearer. Dormant on an axis that bears no laterals.", group: "form" },
  { name: "form.ripening", key: "form/ripening", zone: false, label: "ripening", scale: "log", floor: 1, low: 0, high: 1000, lowOpen: false, never: false, neutral: 0, unit: "years", meaning: "The years a phytomer takes to lay down its own pipe. Dormant where the girth keeps its established width (secondary 0).", group: "form" },
  { name: "form.dominance", key: "form/dominance", zone: false, label: "dominance", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "share", meaning: "How unequal sibling laterals are, so a few dominate. Dormant on an axis that bears no laterals.", group: "branching" },
  { name: "form.roll", key: "form/roll", zone: false, label: "roll", scale: "linear", floor: 0, low: 0, high: 3.141592653589793, lowOpen: false, never: false, neutral: 0, unit: "rad", meaning: "How far a lateral turns about its parent, so siblings do not stack. Dormant on an axis that bears no laterals.", group: "branching" },
  { name: "form.sag", key: "form/sag", zone: false, label: "sag", scale: "log", floor: 0.00001, low: 0, high: 1000, lowOpen: false, never: false, neutral: 0, unit: "-", meaning: "How far the axis bends under the load it carries.", group: "form" },
  { name: "form.secondary", key: "form/secondary", zone: false, label: "secondary", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 1, unit: "share", meaning: "How far the girth follows the load; 0 keeps the established width.", group: "form" },
  { name: "form.bend_length", key: "form/bendLength", zone: false, label: "bend length", scale: "log", floor: 0.1, low: 0, high: 1000, lowOpen: false, never: false, neutral: 0, unit: "m", meaning: "How far the wander remembers its bend: slow arcs rather than kinks. Dormant without wander.", group: "form" },
  { name: "form.internode_spread", key: "form/internodeSpread", zone: false, label: "internode spread", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "share", meaning: "How far each internode's length departs from its age's.", group: "form" },
  { name: "form.insertion_spread", key: "form/insertionSpread", zone: false, label: "insertion spread", scale: "linear", floor: 0, low: 0, high: 1.5707963267948966, lowOpen: false, never: false, neutral: 0, unit: "rad", meaning: "How far each lateral's insertion angle departs from its age's. Dormant on an axis that bears no laterals.", group: "branching" },
  { name: "form.tropism_spread", key: "form/tropismSpread", zone: false, label: "tropism spread", scale: "linear", floor: 0, low: 0, high: 1, lowOpen: false, never: false, neutral: 0, unit: "share", meaning: "How far each axis's tropism departs from its age's.", group: "form" },
  { name: "zone.nodes.min", key: "nodes/min", zone: true, label: "min", scale: "linear", floor: 0, low: 0, high: 1000, lowOpen: false, never: false, neutral: 0, unit: "nodes", meaning: "The fewest nodes a zone of the growth unit holds.", group: "branching" },
  { name: "zone.nodes.max", key: "nodes/max", zone: true, label: "max", scale: "linear", floor: 0, low: 0, high: 1000, lowOpen: false, never: false, neutral: 0, unit: "nodes", meaning: "The most nodes a zone of the growth unit holds.", group: "branching" },
  { name: "zone.buds", key: "buds", zone: true, label: "buds", scale: "linear", floor: 0, low: 1, high: 6, lowOpen: false, never: false, neutral: 1, unit: "buds", meaning: "Lateral buds per node: 1 alternate, 2 opposite, more a whorl. Dormant on an axis that bears no laterals.", group: "branching" },
  { name: "zone.delay", key: "delay", zone: true, label: "delay", scale: "log", floor: 0.1, low: 0, high: 1000, lowOpen: false, never: false, neutral: 0, unit: "years", meaning: "The years before a sleeping bud can wake. Dormant without sleeping buds.", group: "branching" },
  { name: "zone.rate", key: "rate", zone: true, label: "rate", scale: "log", floor: 0.01, low: 0, high: 100, lowOpen: false, never: false, neutral: 0, unit: "per year", meaning: "A sleeping bud's yearly waking hazard after its delay. Dormant without sleeping buds.", group: "branching" },
];
