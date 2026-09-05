import * as THREE from "three";
import { createRng } from "../rng";
import type { RadiusField } from "../radius";
import { DEFAULT_MAX_TURN_PER_STEP, limitTurn, type GrowthConfig, type Skeleton, type SkeletonNode } from "./colonize";
import { branchLength, childRadius, internodeLength, DEFAULT_BRANCH_LAW, DEFAULT_TWIG_ANATOMY, type TwigAnatomy } from "./law";

/** Records are parallel to nodes[crossover..], indexed by node - crossover.
 * Branch ids are absolute node indices naming the first internode of a run.
 * A twig is its own single-edge branch with the anatomy's fixed radius.
 * Cap flags survive shedding; refusal returns the input nodes unchanged. */
export interface TwiggedSkeleton extends Skeleton {
  crossover: number;
  branchId: Int32Array;
  baseRadius: Float64Array;
  twig: Uint8Array;
  levelCapped: boolean;
  nodeCapped: boolean;
  refused?: "invalid-parent" | "radius-length-mismatch";
}

export interface TwigParams {
  twig: TwigAnatomy;
  /** Child branch length relative to its parent, held to 0.05..1. */
  lengthRatio: number;
  /** Radius-from-length exponent, held to 0..8. */
  ratioPower: number;
  /** Geometric internode length in branch diameters, held to 0.05..32. */
  internodeFactor: number;
  /** Side branches per branch, rounded and held to 0..7. */
  laterals: number;
  /** Colonization lateral threshold as a fraction of root radius, held to 0..1. */
  limbRadius: number;
  /** Lateral departure in degrees, held to 0..90. */
  angle: number;
  /** Symmetric departure spread in degrees, held to 0..90. */
  angleVariation: number;
  /** Symmetric lateral length-ratio spread, held to 0..0.95. */
  vigourVariation: number;
  /** Lineage phyllotaxis in degrees; any finite angle. */
  divergence: number;
}

/** Each branch bears its own lateral count and one terminal twig.
 * Anatomy and allometry are sourced in law.ts;
 * 45 degrees and golden-angle phyllotaxis follow Weber & Penn's fine
 * branches and Jean's spiral arrangement. */
export const DEFAULT_TWIGS: TwigParams = {
  get twig() { return DEFAULT_TWIG_ANATOMY; },
  get lengthRatio() { return DEFAULT_BRANCH_LAW.lengthRatio; },
  get ratioPower() { return DEFAULT_BRANCH_LAW.ratioPower; },
  get internodeFactor() { return DEFAULT_BRANCH_LAW.internodeFactor; },
  laterals: 2,
  limbRadius: 0.1,
  angle: 45,
  angleVariation: 10,
  vigourVariation: 0.15,
  divergence: 137.508,
};
export const MAX_TWIG_LEVELS = 12;
const MIN_SEPARATION = 1e-6;
const DEG_TO_RAD = Math.PI / 180;
const held = (value: number, fallback: number): number => Number.isFinite(value) ? value : fallback;
const pinned = (value: number, low: number, high: number): number => Math.min(high, Math.max(low, value));

export function resolveTwigs(twigs?: Partial<TwigParams>): TwigParams {
  const asked = { ...DEFAULT_TWIGS, ...twigs };
  const twig = { ...DEFAULT_TWIG_ANATOMY, ...asked.twig };
  return {
    twig: {
      length: pinned(held(twig.length, DEFAULT_TWIG_ANATOMY.length), 1e-6, 1e6),
      diameter: pinned(held(twig.diameter, DEFAULT_TWIG_ANATOMY.diameter), 1e-6, 1e6),
      internodeLength: pinned(held(twig.internodeLength, DEFAULT_TWIG_ANATOMY.internodeLength), 1e-6, 1e6),
      stationsPerInternode: pinned(Math.round(held(twig.stationsPerInternode, DEFAULT_TWIG_ANATOMY.stationsPerInternode)), 1, 32),
      bearingDiameter: pinned(held(twig.bearingDiameter, DEFAULT_TWIG_ANATOMY.bearingDiameter), 1e-6, 1e6),
    },
    lengthRatio: pinned(held(asked.lengthRatio, DEFAULT_TWIGS.lengthRatio), 0.05, 1),
    ratioPower: pinned(held(asked.ratioPower, DEFAULT_TWIGS.ratioPower), 0, 8),
    internodeFactor: pinned(held(asked.internodeFactor, DEFAULT_TWIGS.internodeFactor), 0.05, 32),
    laterals: pinned(Math.round(held(asked.laterals, DEFAULT_TWIGS.laterals)), 0, 7),
    limbRadius: pinned(held(asked.limbRadius, DEFAULT_TWIGS.limbRadius), 0, 1),
    angle: pinned(held(asked.angle, DEFAULT_TWIGS.angle), 0, 90),
    angleVariation: pinned(held(asked.angleVariation, DEFAULT_TWIGS.angleVariation), 0, 90),
    vigourVariation: pinned(held(asked.vigourVariation, DEFAULT_TWIGS.vigourVariation), 0, 0.95),
    divergence: held(asked.divergence, DEFAULT_TWIGS.divergence),
  };
}

/** A unit vector perpendicular to unit `direction`, the same one for
 *  the same input: the world axis the direction is least aligned with,
 *  crossed with it - the choice `limitTurn` makes for its antiparallel
 *  case, for the same reason. */
function perpendicular(direction: THREE.Vector3): THREE.Vector3 {
  const ax = Math.abs(direction.x);
  const ay = Math.abs(direction.y);
  const az = Math.abs(direction.z);
  const reference =
    ax <= ay && ax <= az
      ? new THREE.Vector3(1, 0, 0)
      : ay <= az
        ? new THREE.Vector3(0, 1, 0)
        : new THREE.Vector3(0, 0, 1);
  return reference.cross(direction).normalize();
}

/** `normal` carried onto the plane perpendicular to unit `direction`:
 *  its component along the direction removed, then made unit. Where
 *  the two have become parallel there is nothing to carry, and
 *  `fallback` - the binormal, which cannot also be parallel - stands
 *  in. This is what keeps the phyllotactic phase a spiral along a
 *  shoot rather than a frame that jumps at every node. */
function transport(
  normal: THREE.Vector3,
  fallback: THREE.Vector3,
  direction: THREE.Vector3,
): THREE.Vector3 {
  const carried = normal
    .clone()
    .addScaledVector(direction, -normal.dot(direction));
  if (carried.lengthSq() > 1e-12) return carried.normalize();
  return fallback
    .clone()
    .addScaledVector(direction, -fallback.dot(direction))
    .normalize();
}

interface Shoot {
  at: number;
  direction: THREE.Vector3;
  normal: THREE.Vector3;
  phase: number;
  radius: number;
  length: number;
  branch: number;
  completed: number;
  generation: number;
  internodes: number;
  key: number;
}

/** Continues tips and seeds laterals along eligible limbs under the
 * supplied radius field. Leaders keep a
 * branch's radius and length allocation through its internodes; laterals
 * start the next generation. Every completed run bears one fixed twig.
 * The bias receives colonization's step at every internode and limitTurn
 * binds curvature to arrival; a lateral starts from its anatomical angle.
 * Pure, breadth-first and deterministic, with no shared random stream. */
export function branchTwigs(
  skeleton: Skeleton,
  field: RadiusField,
  config: GrowthConfig,
  params: TwigParams,
  seed = 0,
): TwiggedSkeleton {
  const base = skeleton.nodes;
  const nodes: SkeletonNode[] = base.slice();
  const crossover = base.length;
  const branchIds: number[] = [];
  const radii: number[] = [];
  const marks: number[] = [];
  let levelCapped = false;
  let nodeCapped = false;
  const result = (refused?: TwiggedSkeleton["refused"]): TwiggedSkeleton => ({
    nodes, crossover, branchId: Int32Array.from(branchIds),
    baseRadius: Float64Array.from(radii), twig: Uint8Array.from(marks),
    levelCapped, nodeCapped, ...(refused ? { refused } : {}),
  });
  for (let i = 0; i < base.length; i++) {
    const parent = base[i].parent;
    if (i === 0 ? parent !== -1 : !Number.isInteger(parent) || parent < 0 || parent >= i) {
      return result("invalid-parent");
    }
  }
  if (field.radius.length !== base.length) return result("radius-length-mismatch");
  if (base.length < 2 || !(config.stepDistance > 0)) return result();
  const twigs = resolveTwigs(params);
  const twigRadius = twigs.twig.diameter / 2;
  /* On twig-bearing wood the internode is at least a twig's length:
     shoots are spaced no closer than their own length, which is what a
     fine branch looks like and what keeps the twig count bounded. */
  const branchInternodes = (radius: number, length: number) => Math.max(1, Math.round(
    length / internodeLength(radius, length, twigs.internodeFactor,
      radius <= twigs.twig.bearingDiameter / 2 ? twigs.twig.length : twigs.twig.internodeLength)));

  const step = config.stepDistance;
  const maxTurn = Math.max(0, held(config.maxTurnPerStep ?? NaN, DEFAULT_MAX_TURN_PER_STEP)) * DEG_TO_RAD;
  const tilt = twigs.angle * DEG_TO_RAD;
  const divergence = twigs.divergence * DEG_TO_RAD;
  const separation = Math.cos(Math.max(MIN_SEPARATION, Math.min(tilt, maxTurn) / 2));
  const children = new Int32Array(base.length);
  for (let i = 1; i < base.length; i++) children[base[i].parent]++;
  let frontier: Shoot[] = [];
  for (let i = 1; i < base.length; i++) {
    if (base[i].position.y < config.trunkHeight) continue;
    if (children[i] !== 0 && !(field.radius[i] < twigs.limbRadius * field.radius[0])) continue;
    const direction = base[i].position.clone().sub(base[base[i].parent].position);
    if (!(direction.lengthSq() > 0)) continue;
    direction.normalize();
    const radius = Math.max(0, held(field.radius[i], 0));
    frontier.push({ at: i, direction, normal: perpendicular(direction), phase: (i * divergence) % (2 * Math.PI),
      radius, length: branchLength(radius), branch: -1, completed: 0, generation: 0,
      internodes: branchInternodes(radius, branchLength(radius)), key: i });
  }
  const binormal = new THREE.Vector3();
  const wanted = new THREE.Vector3();
  const across = new THREE.Vector3();
  const candidate = new THREE.Vector3();
  const accepted: THREE.Vector3[] = [];
  while (frontier.length > 0) {
    const next: Shoot[] = [];
    for (const shoot of frontier) {
      const from = shoot.direction;
      const position = nodes[shoot.at].position;
      const phase = shoot.phase + divergence;
      binormal.crossVectors(from, shoot.normal);
      accepted.length = 0;
      const origin = shoot.at < crossover;
      /* Wood at or under the bearing diameter is the last order: it
         carries a twig at every internode station and no lateral
         branch. Thicker wood bears its stated laterals at evenly spaced
         stations. Real shoots grow from buds on the youngest wood; a
         limb bears branches, a branch bears branches until it is fine,
         and the fine branch bears the shoots. */
      const bearing = !origin && shoot.radius <= twigs.twig.bearingDiameter / 2;
      let laterals = 0;
      let firstLateral = 0;
      if (origin) {
        if (shoot.radius < twigs.limbRadius * field.radius[0]) laterals = twigs.laterals;
      } else if (bearing) {
        if (shoot.completed > 0 && shoot.completed < shoot.internodes) {
          laterals = 1;
          firstLateral = shoot.completed;
        }
      } else {
        for (let j = 0; j < twigs.laterals; j++) {
          const station = Math.max(1, Math.round((j + 1) * shoot.internodes / (twigs.laterals + 1)));
          if (station === shoot.completed) {
            if (laterals === 0) firstLateral = j;
            laterals++;
          }
        }
      }
      for (let c = 0; c <= laterals; c++) {
        const lateral = c > 0;
        if (!lateral && origin && children[shoot.at] !== 0) continue;
        // Colonization index roots the identity; lateral ordinals extend it.
        // Appended array indices change on a prefix or a finer resolution.
        const bud = firstLateral + c;
        const key = lateral ? (createRng(shoot.key ^ Math.imul(bud, 0x9e3779b9)).next() * 0x100000000) >>> 0 : shoot.key;
        const draw = (salt: number) => 2 * createRng(key ^ seed ^ salt).next() - 1;
        const ratio = lateral && twigs.vigourVariation !== 0
          ? pinned(twigs.lengthRatio * (1 + twigs.vigourVariation * draw(0x68bc21eb)), 0.05, 1)
          : twigs.lengthRatio;
        const departure = lateral && twigs.angleVariation !== 0
          ? pinned(twigs.angle + twigs.angleVariation * draw(0x02e5be93), 0, 90) * DEG_TO_RAD : tilt;
        const radius = lateral ? childRadius(shoot.radius, ratio, twigs.ratioPower) : shoot.radius;
        const length = lateral ? shoot.length * ratio : shoot.length;
        const generation = shoot.generation + Number(lateral);
        const terminal = !lateral && shoot.completed === shoot.internodes;
        const isTwig = terminal || (lateral && bearing) || radius <= twigRadius || length < twigs.twig.internodeLength;
        const startsRun = lateral || shoot.branch < 0 || terminal;
        const completed = startsRun ? 0 : shoot.completed;
        const internodes = lateral ? branchInternodes(radius, length) : shoot.internodes;
        if (!isTwig && generation >= MAX_TWIG_LEVELS) { levelCapped = true; continue; }
        if (!lateral) wanted.copy(from);
        else {
          const azimuth = origin
            ? phase + ((c - 1) * Math.PI * 2) / laterals
            : phase + (firstLateral + c - 1) * divergence;
          across.copy(shoot.normal).multiplyScalar(Math.cos(azimuth)).addScaledVector(binormal, Math.sin(azimuth));
          wanted.copy(from).multiplyScalar(Math.cos(departure)).addScaledVector(across, Math.sin(departure));
        }
        const heading = limitTurn(
          lateral ? wanted : from,
          config.bias ? config.bias(position, wanted, step) : wanted.clone().normalize(),
          maxTurn,
        );
        if (lateral) {
          let collides = from.dot(heading) >= separation;
          for (let a = 0; a < accepted.length && !collides; a++) collides = accepted[a].dot(heading) >= separation;
          if (collides) continue;
        }
        const distance = isTwig ? twigs.twig.length : length / internodes;
        candidate.copy(position).addScaledVector(heading, distance);
        if (candidate.y < config.trunkHeight) continue;
        if (nodes.length >= config.maxNodes) { nodeCapped = true; return result(); }
        if (lateral) accepted.push(heading);
        const id = nodes.length;
        const branch = startsRun ? id : shoot.branch;
        nodes.push({ position: candidate.clone(), parent: shoot.at });
        branchIds.push(branch);
        radii.push(isTwig ? twigRadius : radius);
        marks.push(Number(isTwig));
        if (!isTwig) next.push({ at: id, direction: heading,
          normal: transport(shoot.normal, binormal, heading), phase, radius, length,
          branch, completed: completed + 1, generation, internodes, key });
      }
    }
    frontier = next;
  }
  return result();
}
