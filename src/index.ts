/* ------------------------------------------------------------------ *
 * TELPERION
 *
 * A procedural tree generator. Parameters and a seed in, geometry out.
 *
 * The pipeline is four stages and each is usable on its own:
 *
 *   envelope  -> the authored silhouette, as a solid of revolution
 *   skeleton  -> space colonization fills it, biased by a growth field
 *   radius    -> a thickness per node, conserving area through forks
 *   surface   -> one continuous swept mesh, plaited and root-flared
 *
 * Everything that affects the look is a named parameter. There are no
 * magic constants, which is what makes a tree a parameter set rather
 * than a code path - see `presets/` for the two the library is named
 * after.
 *
 * The library knows nothing about light. It emits geometry and
 * attachment frames; materials, lights, exposure and post are the
 * consumer's. `three` is a peer dependency and the only one.
 * ------------------------------------------------------------------ */

// Chance, in one place, so determinism is checkable.
export { createRng, type Rng } from "./rng";
export { createNoise, type Noise } from "./noise";

// The authored silhouette.
export {
  type Envelope,
  DEFAULT_ENVELOPE,
  envelopeMaxRadius,
  envelopeRadiusAt,
  envelopeContains,
  sampleEnvelope,
} from "./envelope";

// The growth bias field: gravitropism, lean, writhe, spiral.
export {
  type BiasParams,
  DEFAULT_BIAS,
  NO_BIAS,
  MIN_STEPS_PER_BEND,
  createGrowthBias,
} from "./torsion";

// Space colonization, and the front door that drives it.
export {
  colonize,
  type Skeleton,
  type SkeletonNode,
  type GrowthConfig,
  type GrowthBias,
  DEFAULT_MAX_TURN_PER_STEP,
} from "./skeleton/colonize";
export {
  growSkeleton,
  defaultGrowth,
  type SkeletonParams,
} from "./skeleton/grow";

// Thickness.
export {
  solveRadii,
  DEFAULT_RADII,
  type RadiusParams,
  type RadiusField,
} from "./radius";

// The surface, and the two pieces it is built from.
export {
  buildSurface,
  DEFAULT_SURFACE,
  type SurfaceParams,
  type SurfaceMesh,
} from "./mesh/surface";
export { branchPaths, type BranchPath } from "./mesh/paths";
export { transportFrames, type Frame } from "./mesh/frames";

// The trees the library is named for.
export {
  TELPERION,
  LAURELIN,
  PRESETS,
  getPreset,
  type TreePreset,
  type PresetSkeleton,
} from "./presets";
