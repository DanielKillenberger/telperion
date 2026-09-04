import type { SurfaceParams } from "../mesh/surface";
import type { RadiusParams } from "../radius";
import type { SkeletonParams } from "../skeleton/grow";
import type { BiasParams } from "../torsion";

/* ------------------------------------------------------------------ *
 * WHAT A PRESET IS
 *
 * A parameter set. Nothing else, and deliberately nothing else: a
 * preset is the three argument objects `growSkeleton`, `solveRadii`
 * and `buildSurface` already take, written down under a name. There is
 * no preset hook in the generator, no branch keyed on an id, and
 * nothing here a caller could not have typed out by hand.
 *
 * That is the spec's acceptance test stated as a type. Telperion and
 * Laurelin have to come out of one algorithm with only their
 * parameters differing, so the moment a preset needs its own `if`
 * somewhere in the library, the parameter it actually wanted was
 * missing and the honest answer is to say so rather than to write the
 * branch.
 *
 * Every term is stated, none inherited. A preset that spread the
 * library's defaults and overrode four fields would be a tree whose
 * look is partly authored here and partly a constant somewhere else,
 * and "there are no magic constants" is exactly the principle that
 * rules out. Reading a preset tells you the whole tree.
 * ------------------------------------------------------------------ */

/** The skeleton half of a preset: `SkeletonParams` with the two
 *  optional members stated in full, because a preset is a complete
 *  answer and not a set of overrides. `growth` carries only bending
 *  stiffness - every other growth distance in `defaultGrowth` is a
 *  fraction of envelope height and so already scales with the tree,
 *  and neither of the Two Trees has asked to depart from one. */
export interface PresetSkeleton extends SkeletonParams {
  bias: BiasParams;
  growth: { maxTurnPerStep: number };
}

/** One named tree: a seed, an envelope, a bias field, a thickness
 *  solve and a surface. Handed to the same three library calls any
 *  other set of parameters is. */
export interface TreePreset {
  /** Stable identifier, used to pick a preset from a panel or a URL.
   *  Never read by the generator. */
  readonly id: string;
  /** What the owner sees on the button. */
  readonly name: string;
  /** One line on what this tree is, for a panel that has room for it. */
  readonly note: string;
  readonly skeleton: PresetSkeleton;
  readonly radii: RadiusParams;
  readonly surface: SurfaceParams;
}
