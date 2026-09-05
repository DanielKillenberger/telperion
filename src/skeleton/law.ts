import { MAX_TWIG_LEVELS } from "./twigs";

/* ------------------------------------------------------------------ *
 * THE BRANCH LAW, BEFORE THE PASS
 *
 * Distances are metres; the input is RADIUS, the twig anatomy DIAMETER.
 * This module does not grow or alter a skeleton. The rules at rest are
 * a selected broadleaf model, with the evidence and its limits below.
 *
 *   lengthScale - L = C r^(2/3), C = 148 / 7.4^(2/3), about 38.97
 *                 m^(1/3). Calibrated to Telperion's 148 m height and
 *                 7.4 m trunk radius, giving about 21 m on 0.79 m
 *                 diameter wood. McMahon 1975, "The Mechanical Design
 *                 of Trees", doi:10.1038/scientificamerican0775-92,
 *                 supplies elastic similarity, not this calibration.
 *                 Niklas & Spatz 2004, doi:10.1073/pnas.0405857101,
 *                 finds the 2/3 exponent at LARGE diameters only and
 *                 rejects one exponent across sizes. Flow/hydraulic
 *                 similarity matters toward the twig. Here 2/3 sets
 *                 the first branch's length; descendant lengths take
 *                 their parent's length ratio, and the fixed twig
 *                 stops extrapolation at fine scale. This is an
 *                 explicit modelling choice, not a fit to twig data.
 *                 C is held to 1e-6..1e6 for finite arithmetic.
 *   lengthRatio - 0.4; ratioPower - 1.3. A lateral starts at
 *                 r_parent * (L_child / L_parent)^ratioPower.
 *                 Weber & Penn 1995, section 4.3 and p.126 table,
 *                 doi:10.1145/218380.218427. Aspen's Ratio/RatioPower
 *                 are 0.015/1.2, Tupelo's 0.015/1.3; fine nLength is
 *                 0.6 and Tupelo's final level 0.4. We select that
 *                 fine-level pair for repeated lateral reductions.
 *                 Their Ratio is a trunk radius/length ratio, not an
 *                 additional multiplier at each fork. This graphics
 *                 model is a structural precedent, not a universal
 *                 botanical law. Ratio is held to 0.05..1 and power
 *                 to 0..8; 1 or 0 can prevent convergence, reported
 *                 by the level cap rather than hidden by the rail.
 *   twig        - 5 mm diameter, 25 cm long, 20 mm internode, one leaf
 *                 station per internode (alternate broadleaf), borne at
 *                 every station of wood 5 cm and under and never on
 *                 thicker wood, spaced no closer than a twig's length.
 *                 The length is a current-year broadleaf shoot; the
 *                 bearing diameter is the one-to-three-year wood buds
 *                 break on. These are SELECTED
 *                 anatomy, not universal constants. Corner 1949,
 *                 doi:10.1093/oxfordjournals.aob.a083225, and Pickup
 *                 et al. 2005, doi:10.1111/j.0269-8463.2005.00927.x,
 *                 link twig size with leaf size. The 120 mm library
 *                 blade is 24 times this diameter, a design choice.
 *                 Bian et al. 2019, doi:10.3390/ijms20194722, fig.4,
 *                 reports 90% of wild-type birch internodes at
 *                 15..25 mm; 20 mm selects that range, not a
 *                 species-wide mean. One station selects the spiral
 *                 arrangement already used by the canopy (Jean 1994,
 *                 "Phyllotaxis", doi:10.1017/CBO9780511666933).
 *                 Anatomy never reads height or trunk size. The
 *                 radius stopping test holds diameter to 1e-6..1e6 m.
 *   generations - repeated LATERAL radii until diameter/2, at most
 *                 MAX_TWIG_LEVELS. Zero means the handoff is already
 *                 twig-sized. The leader is the continuing internode
 *                 run of a branch, not a balanced half-area daughter.
 *                 Its internodes do not increment lateral order.
 *
 * PIPE AREA IS CONTEXT, NOT A SECOND RADIUS OWNER. Shinozaki et al.
 * 1964, doi:10.18960/seitai.14.3_97, relates foliage to conducting
 * cross-section and retains disused pipes in older wood. A lateral
 * does not inherit a share of all the parent's accumulated wood.
 * Minamino & Tateno 2014, doi:10.1371/journal.pone.0093535
 * (PMC3979699), discusses daughter/mother area ratios near 1.04..1.3.
 * The 1.04 is a MODEL EXAMPLE with main/lateral weight ratio 10:1,
 * not a universal measured lower bound. Apical dominance makes an
 * area-halving logarithm the wrong estimate for lateral generations;
 * no area ratio from that paper enters these functions.
 *
 * Non-finite terms fall back to the resting value; non-finite or
 * negative input radius means zero wood. Rails are numerical bounds
 * and allow departures far beyond the dimensions of a living tree.
 * ------------------------------------------------------------------ */

export interface TwigAnatomy {
  /** Metres across the terminal shoot. */
  diameter: number;
  /** Metres of current-year shoot, represented by one skeleton edge. */
  length: number;
  /** Metres between leaf stations, independent of envelope height. */
  internodeLength: number;
  /** One for alternate, two for opposite leaves. */
  stationsPerInternode: number;
  /** Metres across the wood that bears twigs: a branch at or under this
   *  diameter carries a twig at every internode station and no further
   *  lateral branches. Shoots grow from buds on one- to three-year wood,
   *  which on a broadleaf is a few centimetres across; thicker wood bears
   *  branches, not shoots. Selected anatomy, held to 1e-6..1e6. */
  bearingDiameter: number;
}

export const DEFAULT_TWIG_ANATOMY: Readonly<TwigAnatomy> = Object.freeze({
  diameter: 0.005,
  length: 0.25,
  internodeLength: 0.02,
  stationsPerInternode: 1,
  bearingDiameter: 0.05,
});

export interface BranchLawParams {
  /** C in L = C r^(2/3), in metres^(1/3). Held to 1e-6..1e6. */
  lengthScale: number;
  /** Lateral length relative to its parent branch. Held to 0.05..1. */
  lengthRatio: number;
  /** Radius-from-length exponent. Held to 0..8. */
  ratioPower: number;
  /** Internode length as a multiple of branch diameter, held to 0.05..32. */
  internodeFactor: number;
  /** Stopping diameter in metres. Held to 1e-6..1e6. */
  twigDiameter: number;
}

export const DEFAULT_BRANCH_LAW: Readonly<BranchLawParams> = Object.freeze({
  lengthScale: 148 / 7.4 ** (2 / 3),
  lengthRatio: 0.4,
  ratioPower: 1.3,
  twigDiameter: DEFAULT_TWIG_ANATOMY.diameter,
  internodeFactor: 2.5,
});

const held = (value: number, fallback: number): number =>
  Number.isFinite(value) ? value : fallback;
const pinned = (value: number, low: number, high: number): number =>
  Math.min(high, Math.max(low, value));

/** First branch length in metres. Descendants use their parent's length
 *  ratio; callers collapse a run shorter than an internode to a twig. */
export function branchLength(
  radius: number,
  params: Partial<BranchLawParams> = {},
): number {
  const scale = pinned(
    held(params.lengthScale ?? NaN, DEFAULT_BRANCH_LAW.lengthScale),
    1e-6,
    1e6,
  );
  return scale * Math.max(0, held(radius, 0)) ** (2 / 3);
}

/** A lateral's own base radius in metres, with no sibling-area share. */
export function childRadius(
  parentRadius: number,
  lengthRatio: number,
  ratioPower: number,
): number {
  const ratio = pinned(held(lengthRatio, DEFAULT_BRANCH_LAW.lengthRatio), 0.05, 1);
  const power = pinned(held(ratioPower, DEFAULT_BRANCH_LAW.ratioPower), 0, 8);
  return Math.max(0, held(parentRadius, 0)) * ratio ** power;
}

/** Number of lateral reductions, excluding the terminal twig itself.
 *  Reaching twig radius on the last allowed reduction is uncapped;
 *  `capped` means wood is still thicker than the twig after the limit. */
export function generationsUntilTwig(
  radius: number,
  params: Partial<BranchLawParams> = {},
): { generations: number; capped: boolean } {
  const twigRadius = pinned(
    held(params.twigDiameter ?? NaN, DEFAULT_BRANCH_LAW.twigDiameter), 1e-6, 1e6,
  ) / 2;
  let remaining = Math.max(0, held(radius, 0));
  let generations = 0;
  while (remaining > twigRadius && generations < MAX_TWIG_LEVELS) {
    remaining = childRadius(
      remaining,
      params.lengthRatio ?? NaN,
      params.ratioPower ?? NaN,
    );
    generations += 1;
  }
  return { generations, capped: remaining > twigRadius };
}

/** Geometric step in metres, floored at the twig internode. This is
 * sampling resolution, not a count of botanical lateral buds. The
 * branch-length floor limits the resulting run to 32 internodes. */
export function internodeLength(
  radius: number,
  length: number,
  internodeFactor = DEFAULT_BRANCH_LAW.internodeFactor,
  twigInternode = DEFAULT_TWIG_ANATOMY.internodeLength,
): number {
  const factor = pinned(held(internodeFactor, DEFAULT_BRANCH_LAW.internodeFactor), 0.05, 32);
  const floor = pinned(held(twigInternode, DEFAULT_TWIG_ANATOMY.internodeLength), 1e-6, 1e6);
  return Math.max(floor, factor * 2 * Math.max(0, held(radius, 0)), Math.max(0, held(length, 0)) / 32);
}
