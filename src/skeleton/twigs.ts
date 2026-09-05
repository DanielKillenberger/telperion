import * as THREE from "three";
import type { RadiusField } from "../radius";
import { DEFAULT_MAX_TURN_PER_STEP, limitTurn, type GrowthConfig, type Skeleton, type SkeletonNode } from "./colonize";
import { branchLength, childRadius, DEFAULT_BRANCH_LAW, DEFAULT_TWIG_ANATOMY, type TwigAnatomy } from "./law";

/** Records are parallel to nodes[crossover..], indexed by node - crossover.
 * Branch ids are absolute node indices naming the first internode of a run.
 * A twig is its own one-internode branch with the anatomy's fixed radius.
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
  /** Steps in a branch run, rounded and held to 1..32. */
  internodes: number;
  /** Laterals at each interior station, rounded and held to 0..7. */
  laterals: number;
  /** Lateral departure in degrees, held to 0..90. */
  angle: number;
  /** Lineage phyllotaxis in degrees; any finite angle. */
  divergence: number;
}

/** The measured three-internode topology has two lateral stations and
 * one terminal twig. Anatomy and allometry are sourced in law.ts;
 * 45 degrees and golden-angle phyllotaxis follow Weber & Penn's fine
 * branches and Jean's spiral arrangement. */
export const DEFAULT_TWIGS: TwigParams = {
  get twig() { return DEFAULT_TWIG_ANATOMY; },
  get lengthRatio() { return DEFAULT_BRANCH_LAW.lengthRatio; },
  get ratioPower() { return DEFAULT_BRANCH_LAW.ratioPower; },
  internodes: 3,
  laterals: 1,
  angle: 45,
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
      diameter: pinned(held(twig.diameter, DEFAULT_TWIG_ANATOMY.diameter), 1e-6, 1e6),
      internodeLength: pinned(held(twig.internodeLength, DEFAULT_TWIG_ANATOMY.internodeLength), 1e-6, 1e6),
      stationsPerInternode: pinned(Math.round(held(twig.stationsPerInternode, DEFAULT_TWIG_ANATOMY.stationsPerInternode)), 1, 32),
    },
    lengthRatio: pinned(held(asked.lengthRatio, DEFAULT_TWIGS.lengthRatio), 0.05, 1),
    ratioPower: pinned(held(asked.ratioPower, DEFAULT_TWIGS.ratioPower), 0, 8),
    internodes: pinned(Math.round(held(asked.internodes, DEFAULT_TWIGS.internodes)), 1, 32),
    laterals: pinned(Math.round(held(asked.laterals, DEFAULT_TWIGS.laterals)), 0, 7),
    angle: pinned(held(asked.angle, DEFAULT_TWIGS.angle), 0, 90),
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
}

/** Continues each tip under the supplied radius field. Leaders keep a
 * branch's radius and length allocation through its internodes; laterals
 * start the next generation. Every completed run bears one fixed twig.
 * The bias receives colonization's step at every internode and limitTurn
 * binds each accepted direction to the node's own arrival direction.
 * Pure, breadth-first and deterministic, with no shared random stream. */
export function branchTwigs(
  skeleton: Skeleton,
  field: RadiusField,
  config: GrowthConfig,
  params: TwigParams,
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
  const step = config.stepDistance;
  const maxTurn = Math.max(0, held(config.maxTurnPerStep ?? NaN, DEFAULT_MAX_TURN_PER_STEP)) * DEG_TO_RAD;
  const tilt = twigs.angle * DEG_TO_RAD;
  const divergence = twigs.divergence * DEG_TO_RAD;
  const separation = Math.cos(Math.max(MIN_SEPARATION, Math.min(tilt, maxTurn) / 2));
  const children = new Int32Array(base.length);
  for (let i = 1; i < base.length; i++) children[base[i].parent]++;
  let frontier: Shoot[] = [];
  for (let i = 1; i < base.length; i++) {
    if (children[i] !== 0) continue;
    const direction = base[i].position.clone().sub(base[base[i].parent].position);
    if (!(direction.lengthSq() > 0)) continue;
    direction.normalize();
    const radius = Math.max(0, held(field.radius[i], 0));
    frontier.push({ at: i, direction, normal: perpendicular(direction), phase: 0,
      radius, length: branchLength(radius), branch: -1, completed: 0, generation: 0 });
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
      // Only interior stations bear laterals, as in the measured topology.
      const laterals = shoot.completed > 0 && shoot.completed < twigs.internodes ? twigs.laterals : 0;
      for (let c = 0; c <= laterals; c++) {
        const lateral = c > 0;
        const radius = lateral ? childRadius(shoot.radius, twigs.lengthRatio, twigs.ratioPower) : shoot.radius;
        const length = lateral ? shoot.length * twigs.lengthRatio : shoot.length;
        const generation = shoot.generation + Number(lateral);
        const terminal = !lateral && shoot.completed === twigs.internodes;
        const isTwig = terminal || radius <= twigRadius || length < twigs.twig.internodeLength;
        if (!isTwig && generation >= MAX_TWIG_LEVELS) { levelCapped = true; continue; }
        if (!lateral) wanted.copy(from);
        else {
          const azimuth = phase + ((c - 1) * Math.PI * 2) / laterals;
          across.copy(shoot.normal).multiplyScalar(Math.cos(azimuth)).addScaledVector(binormal, Math.sin(azimuth));
          wanted.copy(from).multiplyScalar(Math.cos(tilt)).addScaledVector(across, Math.sin(tilt));
        }
        const heading = limitTurn(
          from,
          config.bias ? config.bias(position, wanted, step) : wanted.clone().normalize(),
          maxTurn,
        );
        if (lateral) {
          let collides = from.dot(heading) >= separation;
          for (let a = 0; a < accepted.length && !collides; a++) collides = accepted[a].dot(heading) >= separation;
          if (collides) continue;
        }
        const distance = isTwig ? twigs.twig.internodeLength : length / twigs.internodes;
        candidate.copy(position).addScaledVector(heading, distance);
        if (candidate.y < config.trunkHeight) continue;
        if (nodes.length >= config.maxNodes) { nodeCapped = true; return result(); }
        if (lateral) accepted.push(heading);
        const id = nodes.length;
        const branch = isTwig || lateral || shoot.branch < 0 ? id : shoot.branch;
        nodes.push({ position: candidate.clone(), parent: shoot.at });
        branchIds.push(branch);
        radii.push(isTwig ? twigRadius : radius);
        marks.push(Number(isTwig));
        if (!isTwig) next.push({ at: id, direction: heading,
          normal: transport(shoot.normal, binormal, heading), phase, radius, length,
          branch, completed: lateral ? 1 : shoot.completed + 1, generation });
      }
    }
    frontier = next;
  }
  return result();
}
