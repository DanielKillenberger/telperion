/* ------------------------------------------------------------------ *
 * THE DIALS THE OWNER TURNS
 *
 * The harness's half of the generator contract. The panel renders
 * whatever is in SLIDERS and hands the generator a GrowerParams; it
 * knows nothing else about either. skeleton-view.ts translates these
 * into the generator's own arguments, so a new dial is added here and
 * the panel picks it up for free.
 *
 * Every name here points at a mechanism the spec already committed to:
 * `height` and `spread` are the authored envelope, the five bias dials
 * are the growth bias field's own five terms under their own names,
 * `taper` is the radius solve's fork exponent, `density` is how many
 * attractors the envelope gets, `growth step` is how finely the
 * growth answers them, the branch dials are the local pass's
 * allometric rules, and the four surface dials are the
 * swept section's own terms, and the canopy dials control leaf
 * orientation and size. Nothing is a knob
 * invented for the panel's sake.
 *
 * The two twists are two dials and are never folded together. `spiral`
 * bends the centreline - the path the limb takes through the air.
 * `surface twist` winds the cross section about that path - the plait
 * on the skin. A tree can have either without the other, and a single
 * dial for both would be exactly the lumping the spec's parameter
 * principle rules out.
 *
 * The radius solve's other two terms - how stout the trunk is and how
 * fast a limb thins along its length - now have dials too, and so does
 * the envelope's bare-trunk height. They are the three that decide
 * whether a tall tree reads as a tall tree rather than as a small one
 * photographed close up, and the library is scale-free on purpose: it
 * states every length as a fraction of height, so nothing in it makes
 * a 60 m trunk proportionally stouter than a 24 m one. That is the
 * right split - physics is a preset's business, not a hidden rule
 * inside the generator - but it only holds if the terms can be
 * reached, and until now they could not be.
 *
 * The bias dials are five and not one, because a single number mixes
 * qualities that are independent - a tree that leans is a different
 * tree from one that wanders, and amplitude and wavelength are the
 * difference between a slow S-curve and a corkscrew. Their names and
 * units are the library's, unchanged, so there is no translation to
 * drift.
 *
 * `torsion` sits above the three of them that are departures from
 * vertical and scales all three at once, which is the "straight to
 * writhing in one move" the spec asks for. It is a master over the
 * shape those five describe, not a sixth quality: at 0 the tree is
 * dead straight whatever the others say, at 1 it is exactly what they
 * say, and it runs to 2 for a look the individual dials would have to
 * be walked to one at a time.
 * ------------------------------------------------------------------ */

import { ORDINARY, type Family } from "../src/browser/core";
import { presetToParams } from "./family";

/** Seeds are unsigned 32-bit integers, and nothing else is a seed. */
export const SEED_MAX = 0xff_ff_ff_ff;

export interface GrowerParams {
  /** Preserve native anatomy and non-slider controls until viewer affordances land. */
  family: Family;
  supernaturalEnabled: boolean;
  /** uint32. Varies the detail; it does not gamble on the outcome. */
  seed: number;
  /** Envelope height, in metres. The tree is judged at human scale. */
  height: number;
  /** Envelope half-width as a fraction of height, so the crown's width
   *  over the tree's height is twice this. The range spans real trees:
   *  0.12 is a narrow upright Telperion at about a quarter as wide as
   *  tall, 0.65 a broad domed Laurelin at about one and a third. Above
   *  that a crown stops reading as a tree and starts reading as a
   *  hedge. */
  spread: number;
  /** Master over `lean`, `writheAmplitude` and `spiralRate`: 0 leaves
   *  the tree dead straight, 1 is the three of them as dialled. */
  torsion: number;
  /** Upward pull on every growth step. */
  gravitropism: number;
  /** Steady departure from vertical: horizontal metres per metre climbed. */
  lean: number;
  /** How far the centreline strays from its mean path, as a fraction
   *  of height. */
  writheAmplitude: number;
  /** The length of one bend, as a fraction of height. */
  writheWavelength: number;
  /** Turns about the trunk axis over the tree's full height. */
  spiralRate: number;
  /** Directional persistence: how far one growth step may turn from
   *  the step before it, in degrees. Bending stiffness - low is a limb
   *  that commits to a direction, high is one that follows whatever is
   *  nearest. */
  maxTurnPerStep: number;
  /** How thickly the envelope is populated: branch count, not leaves. */
  density: number;
  /** The growth step as a fraction of height: the branching depth.
   *  `density` says where the tree is asked to grow and this says how
   *  finely it answers, so a finer step branches further down into
   *  finer wood and more tips. Finer is to the left. */
  step: number;
  /** Fixed twig anatomy and branch-law terms carried through preset
   *  round trips; length ratio and fork exponent have separate owners. */
  twigLength: number;
  twigDiameter: number;
  twigStationLength: number;
  twigStations: number;
  /** Metres across the wood that bears twigs: at or under it a branch carries a twig at every station and no lateral branch. */
  twigBearing: number;
  ratioPower: number;
  limbRadius: number;
  /** Share of the crown's depth colonization leaves for the branches. */
  reach: number;
  laterals: number;
  angleVariation: number;
  vigourVariation: number;
  twigAngle: number;
  twigDivergence: number;
  internodeFactor: number;
  lengthRatio: number;
  /** The radius solve's fork exponent: what a fork does to thickness,
   *  and so the contrast between trunk and twig. 2 conserves
   *  cross-sectional area exactly. */
  taper: number;
  /** How stout the tree is at the ground, as a fraction of height.
   *  Scale-independent by design, which means a taller tree is NOT
   *  automatically a stouter one in proportion - a 60 m tree wants
   *  this dialled up from a 24 m tree's, and this is the dial that
   *  does it. */
  trunkRadius: number;
  /** How fast a limb thins where nothing branches off it, in
   *  e-foldings per envelope height. The bare trunk below the crown is
   *  the longest unbranched run in the tree, so this is most of what a
   *  trunk's silhouette does between the ground and the first fork. */
  lengthTaper: number;
  /** Fraction of the height below which there is no crown: bare trunk.
   *  Big trees shed their lower limbs, so a tall tree wants more of
   *  this than a small one. */
  crownBase: number;
  /** Where the crown is widest, 0 at its base and 1 at its tip. Low is
   *  bottom-heavy and spreading, high is a crown carrying its mass up
   *  top. */
  fullness: number;
  /** The envelope's profile exponent: 1 is a straight-sided cone, 2 an
   *  ellipse, and above that the shoulders square off into a dome. */
  shoulder: number;
  /** Lobes on the swept cross section: how many strands a limb reads
   *  as. 0 is the circle everyone else extrudes. */
  lobes: number;
  /** How deep the lobes cut, as a fraction of the radius. */
  lobeDepth: number;
  /** Turns of the cross section about its own axis over the tree's
   *  height. This is the SURFACE winding - the plaited rope quality -
   *  and it is a different mechanism from `spiralRate`, which bends the
   *  centreline. Outside `torsion` for that reason. */
  twistRate: number;
  /** How much wider the trunk is where it meets the ground, as a
   *  multiple of its radius there. 1 is no flare. */
  flareRadius: number;

  /* All canopy terms survive preset round trips. shootRadius, spacing,
     clump and clumpSpan apply only without marked twig anatomy and are
     therefore carried as values rather than offered as live dials. */
  /** The wood at or below this fraction of the trunk's radius bears
   *  foliage; everything thicker is bark. */
  shootRadius: number;
  /** Distance along a shoot between elements, as a fraction of
   *  envelope height. The density lever: halve it for twice the
   *  canopy. */
  spacing: number;
  /** The phyllotactic divergence angle, in degrees. 137.508 is the
   *  golden angle almost every plant uses; 99.502 is the Lucas angle
   *  Laurelin is authored with. */
  divergence: number;
  /** Elements gathered at the growing tip on top of what `spacing`
   *  already puts there: the difference between a beaded shoot and a
   *  spray. */
  clump: number;
  /** The stretch at the tip the clump gathers into, as a fraction of
   *  the shoot's length. */
  clumpSpan: number;
  /** How far an element turns away from the tree's axis, 0 to 1. */
  outward: number;
  /** How far an element turns toward the sky, 0 to 1. */
  upward: number;
  /** Random spread about the direction those two ask for, in degrees.
   *  Zero is a diagram. */
  scatter: number;
  /** Multiplier on the element's own authored size. The element owns
   *  its absolute dimensions - a leaf does not grow because its tree
   *  is tall - so this says whether a tree wants more or less of it. */
  size: number;
  /** Random variation of that multiplier, 0 to 1: at 0.3 elements run
   *  from 70% to 130% of `size`. */
  sizeVariation: number;
}

export interface SliderSpec {
  key: Exclude<keyof GrowerParams, "seed" | "family" | "supernaturalEnabled">;
  label: string;
  min: number;
  max: number;
  step: number;
  /** Suffix shown next to the value. Empty for a bare ratio. */
  unit: string;
  /** Names the stage this dial belongs to, on the FIRST dial of that
   *  stage only. The panel draws a heading where one appears. The list
   *  already runs in pipeline order; thirty dials without the stage
   *  boundaries drawn is a list you scroll rather than read, and the
   *  canopy's ten sit at the far end of it. */
  group?: string;
}

export const SLIDERS: readonly SliderSpec[] = [
  /* Up to four hundred metres, because the subject is the Two Trees
     and they are not a tall oak - the references put them on the scale
     of a landscape feature, with a city at their feet. The bottom of
     the range stays at a sapling: the generator is a standalone
     library and its acceptance is that one algorithm covers the range,
     not that it covers Valinor. */
  { group: "skeleton", key: "height", label: "height", min: 4, max: 400, step: 0.5, unit: "m" },
  { key: "spread", label: "spread", min: 0.12, max: 0.65, step: 0.01, unit: "" },
  // The bias dials run well past what looks good. The owner has to be
  // able to see where too much is, or the usable range sits at the
  // ceiling and reads as a limit rather than as a choice.
  { key: "torsion", label: "torsion", min: 0, max: 2, step: 0.01, unit: "x" },
  { key: "gravitropism", label: "gravitropism", min: 0, max: 1.3, step: 0.01, unit: "" },
  { key: "lean", label: "lean", min: 0, max: 0.5, step: 0.01, unit: "" },
  { key: "writheAmplitude", label: "writhe", min: 0, max: 0.25, step: 0.01, unit: "" },
  { key: "writheWavelength", label: "bend length", min: 0.18, max: 1.2, step: 0.01, unit: "" },
  { key: "spiralRate", label: "spiral", min: 0, max: 6, step: 0.1, unit: "" },
  // Stiffness, and the one dial that is a rail as well as a look: past
  // about 90 a step can turn back on the one before it and the crown
  // starts drawing the sawtooth fn-11.8 was about, so the dial stops
  // where the rail does rather than showing the owner a range whose top
  // end is a bug.
  { key: "maxTurnPerStep", label: "turn limit", min: 5, max: 90, step: 1, unit: "deg/step" },
  { key: "density", label: "density", min: 0, max: 1, step: 0.01, unit: "" },
  /* The branching depth, beside the density it works with: attractors
     decide where the tree is asked to grow, the step how finely it
     answers. The rail's ends are the measured range and not round
     numbers. The top is the step every tree so far was grown at, 3.26 m
     on Telperion, 808 nodes; the bottom is 0.44 m on the same tree,
     14,100 nodes and nine times the tips, and on the widest crown this
     panel can ask for at full density it is 42,000 nodes and a fifth of
     a second to grow. Past that the count runs away without the wood
     getting usefully finer, and the notch is fine enough to walk the
     bottom of the rail where each one costs the most. */
  { key: "step", label: "growth step", min: 0.003, max: 0.022, step: 0.0005, unit: "h" },
  /* Branch-law rails match resolveTwigs. At the shared rest, 0.4^1.3
     gives a 0.304 radius multiplier per lateral. Ratio 1 or power 0
     keeps the radius constant; the generation safety stop is reported.
     Three internodes give two lateral stations and a terminal twig.
     The integer rails allow 1..32 internodes and 0..7 laterals; their
     upper ends can reach the node ceiling and are exploration bounds.
     At rest Telperion / Laurelin have 49,713 / 104,335 surviving nodes.
     Task 6 measured ratio 0.35 at 26,961 / 67,107 and 0.45 at
     59,080 / 205,756, so the 0.01 notch can cross a generation boundary.
     Power 1.5 gives 26,073 / 54,658; two internodes give 8,330 / 18,268;
     zero laterals give 1,301 / 3,856. These are independent changes to
     rest, with raw measurements in .flow/tmp/task6-measure.log.
     Task 4 measured limbRadius 0.075 / 0.1 / 0.15 at 42,006 / 49,713 /
     56,378 Telperion nodes and 43,199 / 104,335 / 149,826 Laurelin nodes.
     The 0.005 notch resolves that transition; 0 disables limb laterals,
     and 1 admits wood below the root radius. */
  { group: "branches", key: "lengthRatio", label: "length ratio", min: 0.05, max: 1, step: 0.01, unit: "" },
  { key: "ratioPower", label: "radius power", min: 0, max: 8, step: 0.05, unit: "" },
  { key: "internodeFactor", label: "internode factor", min: 0.05, max: 32, step: 0.05, unit: "x" },
  { key: "angleVariation", label: "angle variation", min: 0, max: 90, step: 1, unit: "deg" },
  { key: "vigourVariation", label: "vigour variation", min: 0, max: 0.95, step: 0.01, unit: "" },
  { key: "laterals", label: "laterals", min: 0, max: 7, step: 1, unit: "" },
  { key: "limbRadius", label: "limbRadius", min: 0, max: 1, step: 0.005, unit: "r" },
  // Measured on Telperion: at 0 the 79 cm colonization tips end at the
  // shell and 21 m branches reach out past it, the cactus the owner saw;
  // the rest leaves the outer share of the crown to the pass.
  { key: "reach", label: "reach", min: 0, max: 0.9, step: 0.01, unit: "" },
  { key: "twigAngle", label: "branch angle", min: 0, max: 90, step: 1, unit: "deg" },
  { key: "twigDivergence", label: "branch divergence", min: 0, max: 180, step: 0.001, unit: "deg" },
  // The fork exponent, under the name the owner already turns. Below
  // 2 a fork sheds more than area and the tree runs from a heavy
  // trunk to threads; above 3 the limbs stop thinning enough to read
  // as limbs. The dial spans both sides of that so the good range is
  // visibly a choice.
  { group: "thickness", key: "taper", label: "taper", min: 1.4, max: 3.6, step: 0.05, unit: "n" },
  /* The other two terms of the radius solve, and the envelope's bare
     trunk. They are here because scale is one dial and everything else
     is stated as a fraction of it: nothing in the library makes a
     60 m tree stouter in proportion than a 24 m one, or bares more of
     its trunk, and both of those are things a big tree does. That is
     deliberate - the library stays scale-free and a preset says what
     size a tree is being - but it only works if the terms are
     reachable, and these three were the ones that were not. */
  // The ceiling is elastic similarity at the top of the height dial.
  // A self-supporting trunk's diameter goes as height^1.5, so as a
  // FRACTION of height it goes as height^0.5: from 0.020 at 24 m, a
  // 300 m tree wants 0.071 and a 400 m one 0.082. A 0.05 ceiling put
  // every tree over about 150 m at least a third too spindly to stand
  // up, on exactly the trees the 400 m height ceiling was added for.
  { key: "trunkRadius", label: "trunk", min: 0.004, max: 0.085, step: 0.001, unit: "h" },
  { key: "lengthTaper", label: "length taper", min: 0, max: 2, step: 0.05, unit: "" },
  { group: "envelope", key: "crownBase", label: "crown base", min: 0, max: 0.6, step: 0.01, unit: "" },
  /* The last two terms of the authored silhouette. `spread` says how
     far the crown reaches and these two say what shape it is on the
     way out, which is most of the difference between a narrow upright
     Telperion and a broad domed Laurelin - so leaving them pinned to a
     constant would have made the spec's own acceptance test the one
     thing the panel could not reach. Guarded away from the ends, where
     the profile is degenerate rather than extreme. */
  { key: "fullness", label: "fullness", min: 0.05, max: 0.95, step: 0.01, unit: "" },
  { key: "shoulder", label: "shoulder", min: 1, max: 4, step: 0.05, unit: "n" },
  // The surface dials. `lobes` is a count and steps by one; the other
  // three run from the circular, straight, unflared surface every other
  // procedural tree has out to well past what looks good, on the same
  // principle as the bias dials - the owner has to be able to see where
  // too much is.
  { group: "surface", key: "lobes", label: "lobes", min: 0, max: 9, step: 1, unit: "" },
  { key: "lobeDepth", label: "lobe depth", min: 0, max: 0.4, step: 0.01, unit: "" },
  // Signed, because a plait winding the other way is a different tree
  // and not a smaller one. Stops at three turns either side: the section
  // is sampled once per growth step, and past about six turns over the
  // height the winding outruns the sampling and reads as chatter.
  { key: "twistRate", label: "surface twist", min: -3, max: 3, step: 0.1, unit: "turns" },
  { key: "flareRadius", label: "root flare", min: 1, max: 4, step: 0.05, unit: "x" },
  /* Marked twigs place leaves from their fixed anatomy. shootRadius,
     spacing, clump and clumpSpan remain in preset round trips for the
     library's unmarked-skeleton fallback, but have no dials here.
     Divergence steps in thousandths to retain the authored phyllotaxis;
     the other leaf controls keep the placement stage's existing rails. */
  { group: "canopy", key: "divergence", label: "divergence", min: 0, max: 180, step: 0.001, unit: "deg" },
  { key: "outward", label: "leaf outward", min: 0, max: 1, step: 0.01, unit: "" },
  { key: "upward", label: "leaf upward", min: 0, max: 1, step: 0.01, unit: "" },
  { key: "scatter", label: "leaf scatter", min: 0, max: 90, step: 1, unit: "deg" },
  { key: "size", label: "leaf size", min: 0.2, max: 4, step: 0.05, unit: "x" },
  { key: "sizeVariation", label: "leaf size spread", min: 0, max: 0.9, step: 0.01, unit: "" },
];

export const DEFAULT_PARAMS: GrowerParams = {
  ...presetToParams({ ...ORDINARY, id: "ordinary", name: "Ordinary", note: "" }),
  seed: 1, density: 0.5,
};

/** The seed field is the one free-text surface on the panel, so it is
 *  the one place a value can arrive as anything at all. Parsed
 *  strictly: Number() would take "1e3", " 12", "0x4" and "", none of
 *  which anyone types into a seed box on purpose. Returns null when
 *  the text is not a seed, and the caller keeps the last good one. */
export function normalizeSeed(raw: string): number | null {
  if (!/^\d{1,10}$/.test(raw)) return null;
  const value = Number(raw);
  if (!Number.isInteger(value) || value > SEED_MAX) return null;
  return value;
}

/** A fresh seed, drawn from the platform CSPRNG so consecutive rerolls
 *  are not neighbours in a weak sequence the eye can learn. */
export function randomSeed(): number {
  const buffer = new Uint32Array(1);
  crypto.getRandomValues(buffer);
  return buffer[0];
}

/** A range input hands back a string, and a slider spec is the only
 *  authority on what that string is allowed to mean. Total on purpose:
 *  anything unreadable falls back to the default for that dial rather
 *  than propagating a NaN into the scene graph. */
export function readSlider(spec: SliderSpec, raw: string): number {
  const value = Number.parseFloat(raw);
  if (!Number.isFinite(value)) return DEFAULT_PARAMS[spec.key];
  return Math.min(spec.max, Math.max(spec.min, value));
}
