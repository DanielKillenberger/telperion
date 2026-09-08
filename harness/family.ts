import type { Family, TreePreset } from "../src/browser/core";

import type { GrowerParams } from "./params";

type SkeletonParams = Family["skeleton"];
type RadiusParams = Family["radii"];
type SurfaceParams = Family["surface"];
type CanopyParams = Family["canopy"];

/* ------------------------------------------------------------------ *
 * THE DIALS, AS THE GENERATOR'S OWN ARGUMENTS
 *
 * One translation, in one place: what the panel holds becomes what the
 * generator takes. Nothing here draws anything, and nothing here knows
 * what will - the same composition feeds the renderer in the browser
 * and would feed a native caller unchanged.
 * ------------------------------------------------------------------ */

/** What the density dial spans, in attractors. The floor is a tree
 *  with a readable handful of limbs rather than a bare fork; the
 *  ceiling is where the crown stops gaining structure and starts
 *  gaining only cost. */
const ATTRACTORS_MIN = 250;
const ATTRACTORS_MAX = 1600;

/** The panel's dials, as the generator's arguments.
 *
 *  `height` and `spread` are the authored envelope and go straight
 *  through, and so do the five bias dials - they carry the library's
 *  own names and units, so there is nothing here to translate and
 *  nothing to drift. `torsion` is the one dial that is not a library
 *  term: it scales the supernatural bending terms, so one
 *  move takes the tree from straight to writhing without walking separate
 *  sliders. Lean and gravitropism are deliberately outside it - a tree that wants
 *  to grow up still wants to when it is not twisting.
 *  `density` is the attractor count, and `step` and the branch-law
 *  rules go through under the library's own names. Twig anatomy stays
 *  in metres. `taper` travels through `toRadiusParams`: the colonization
 *  radius solve uses it before branch growth, and the final solve
 *  preserves the appended branches' recorded local taper. */
export function toSkeletonParams(params: GrowerParams): SkeletonParams {
  return {
    ...params.family.skeleton,
    seed: params.seed,
    /* Every member of `Envelope`, named. Spreading the default and
       overriding three of them was fine while the other two were
       constants nobody could reach; now that they are dials, an
       envelope assembled by spread would silently drop whichever term
       the panel forgot to list, which is exactly the drift the
       preset round-trip test exists to catch. */
    envelope: {
      height: params.height,
      spread: params.spread,
      crownBase: params.crownBase,
      fullness: params.fullness,
      shoulder: params.shoulder,
    },
    attractors: Math.round(
      ATTRACTORS_MIN + params.density * (ATTRACTORS_MAX - ATTRACTORS_MIN),
    ),
    step: params.step,
    // Preserve every branch-law and anatomy field through preset round trips.
    twigs: {
      twig: { length: params.twigLength, diameter: params.twigDiameter, internodeLength: params.twigStationLength,
        stationsPerInternode: params.twigStations, bearingDiameter: params.twigBearing },
      ratioPower: params.ratioPower,
      limbRadius: params.limbRadius,
      reach: params.reach,
      laterals: params.laterals,
      angleVariation: params.angleVariation,
      vigourVariation: params.vigourVariation,
      angle: params.twigAngle,
      divergence: params.twigDivergence,
      internodeFactor: params.internodeFactor,
      lengthRatio: params.lengthRatio,
    },
    bias: {
      gravitropism: params.gravitropism,
      lean: params.lean,
      supernatural: {
        enabled: params.supernaturalEnabled,
        writheAmplitude: params.writheAmplitude * params.torsion,
        writheWavelength: params.writheWavelength,
        spiralRate: params.spiralRate * params.torsion,
      },
    },
    // Persistence is a growth distance's kind of parameter rather than
    // a bias term - it is about the step, not about the field - so it
    // travels in `growth`, which is where the library keeps the rest of
    // them. Outside `torsion` on purpose: a stiff tree is stiff whether
    // or not it is writhing.
    growth: { ...params.family.skeleton.growth, maxTurnPerStep: params.maxTurnPerStep },
  };
}

/** The panel's three thickness dials, as the radius solve's arguments.
 *  All three carry the library's own names and units, so there is
 *  nothing here to translate and nothing to drift.
 *
 *  `trunkRadius` is the one that answers "is this a big tree": the
 *  library states it as a fraction of height and does not scale it
 *  with height, so a 60 m tree is exactly as slender in proportion as
 *  a 4 m one until somebody says otherwise. Saying otherwise is the
 *  dial, and later the preset. */
export function toRadiusParams(params: GrowerParams): RadiusParams {
  return {
    forkExponent: params.taper,
    trunkRadius: params.trunkRadius,
    lengthTaper: params.lengthTaper,
  };
}


/** The panel's four surface dials, as the sweep's arguments. The rest
 *  of `SurfaceParams` - how finely the section is sampled, how deep a
 *  child sockets into its parent and how much it swells leaving it,
 *  how far the flare decays and how far it sinks - keep the library's
 *  defaults, on the same footing as native growth distances:
 *  they are structure and cost rather than look, nothing has asked to
 *  turn them live, and each is one line in SLIDERS the day something
 *  does.
 *
 *  `twistRate` is the surface's rotation and it is deliberately not
 *  scaled by the panel's `torsion` master: `torsion` gathers the supernatural
 *  terms that bend the CENTRELINE, and the plait is a different
 *  mechanism that happens to the skin. Folding them together would
 *  make one dial mean two things, which is the thing the spec's
 *  parameter principle exists to stop. */
export function toSurfaceParams(params: GrowerParams): SurfaceParams {
  return {
    ...params.family.surface,
    lobes: params.lobes,
    lobeDepth: params.lobeDepth,
    twistRate: params.twistRate,
    flareRadius: params.flareRadius,
  };
}

/** The panel's canopy values, as the placement stage's arguments.
 *
 *  A rename and nothing else: every term carries the library's own
 *  name and unit, so there is nothing here to translate and nothing to
 *  drift - the same footing the bias and surface dials are on. All ten
 *  are stated rather than spread over `DEFAULT_CANOPY`, for the reason
 *  the envelope in `toSkeletonParams` is: a set assembled by spread
 *  silently keeps a default for whichever term the panel forgot, and a
 *  preset loaded onto the dials would then be built from a canopy
 *  nobody authored. */
export function toCanopyParams(params: GrowerParams): CanopyParams {
  return {
    ...params.family.canopy,
    shootRadius: params.shootRadius,
    spacing: params.spacing,
    divergence: params.divergence,
    clump: params.clump,
    clumpSpan: params.clumpSpan,
    outward: params.outward,
    upward: params.upward,
    scatter: params.scatter,
    size: params.size,
    sizeVariation: params.sizeVariation,
  };
}

/** A preset's parameters, as the panel's dials.
 *
 *  The dials and a preset are two ways of writing down the same
 *  argument triple, so this is a rename and not a translation: every
 *  member of `GrowerParams` comes from the preset and nothing is
 *  invented here. `torsion` is the one term with no counterpart, and
 *  it is 1 by definition - a preset states the three bias terms it
 *  wants, and the master that scales them is a convenience for
 *  dragging, not part of the tree.
 *
 *  Exact in both directions: `toSkeletonParams`, `toRadiusParams`,
 *  `toSurfaceParams` and `toCanopyParams` applied to the result
 *  reproduce the preset member for member. That round trip is
 *  asserted, because a panel that silently dropped one of a preset's
 *  terms would show the owner a tree nobody authored - and the canopy
 *  is where that would bite hardest, since Laurelin's divergence is
 *  not the golden angle the default is. */
export function presetToParams(preset: TreePreset): GrowerParams {
  const { envelope, bias } = preset.skeleton;
  const canopy = preset.canopy;
  return {
    family: structuredClone((({ id: _id, name: _name, note: _note, ...family }) => family)(preset)),
    supernaturalEnabled: bias.supernatural.enabled,
    seed: preset.skeleton.seed,
    height: envelope.height,
    spread: envelope.spread,
    crownBase: envelope.crownBase,
    fullness: envelope.fullness,
    shoulder: envelope.shoulder,
    torsion: 1,
    gravitropism: bias.gravitropism,
    lean: bias.lean,
    writheAmplitude: bias.supernatural.writheAmplitude,
    writheWavelength: bias.supernatural.writheWavelength,
    spiralRate: bias.supernatural.spiralRate,
    maxTurnPerStep: preset.skeleton.growth.maxTurnPerStep,
    density: (preset.skeleton.attractors - ATTRACTORS_MIN) /
      (ATTRACTORS_MAX - ATTRACTORS_MIN),
    step: preset.skeleton.step,
    twigLength: preset.skeleton.twigs.twig.length,
    angleVariation: preset.skeleton.twigs.angleVariation,
    vigourVariation: preset.skeleton.twigs.vigourVariation,
    twigDiameter: preset.skeleton.twigs.twig.diameter,
    twigStationLength: preset.skeleton.twigs.twig.internodeLength,
    twigStations: preset.skeleton.twigs.twig.stationsPerInternode,
    twigBearing: preset.skeleton.twigs.twig.bearingDiameter,
    ratioPower: preset.skeleton.twigs.ratioPower,
    limbRadius: preset.skeleton.twigs.limbRadius,
    reach: preset.skeleton.twigs.reach,
    laterals: preset.skeleton.twigs.laterals,
    twigAngle: preset.skeleton.twigs.angle,
    twigDivergence: preset.skeleton.twigs.divergence,
    internodeFactor: preset.skeleton.twigs.internodeFactor,
    lengthRatio: preset.skeleton.twigs.lengthRatio,
    taper: preset.radii.forkExponent,
    trunkRadius: preset.radii.trunkRadius,
    lengthTaper: preset.radii.lengthTaper,
    lobes: preset.surface.lobes,
    lobeDepth: preset.surface.lobeDepth,
    twistRate: preset.surface.twistRate,
    flareRadius: preset.surface.flareRadius,
    shootRadius: canopy.shootRadius,
    spacing: canopy.spacing,
    divergence: canopy.divergence,
    clump: canopy.clump,
    clumpSpan: canopy.clumpSpan,
    outward: canopy.outward,
    upward: canopy.upward,
    scatter: canopy.scatter,
    size: canopy.size,
    sizeVariation: canopy.sizeVariation,
  };
}

/** The dials as the generator reads them: one family object, every term
 *  the panel can reach composed onto the family the preset carried.
 *
 *  The renderer parses this with the core's own schema, so the keys are
 *  the schema's keys and an unknown one is refused rather than ignored.
 *  Stringifying is deliberately not hidden inside it - a caller that
 *  wants to look at what it is about to send should be able to. */
export function toFamily(params: GrowerParams): Family {
  return {
    ...params.family,
    skeleton: toSkeletonParams(params),
    radii: toRadiusParams(params),
    surface: toSurfaceParams(params),
    canopy: toCanopyParams(params),
  };
}

/** The same family as the text the renderer is handed. A non-finite
 *  dial would otherwise travel as `null` and come back as the schema's
 *  general type complaint, which says nothing about which dial it was. */
export function familyJson(params: GrowerParams): string {
  return JSON.stringify(toFamily(params), (key, value: unknown) => {
    if (typeof value === "number" && !Number.isFinite(value)) {
      throw Error(`Tree parameter "${key}" is not a finite number`);
    }
    return value;
  });
}
