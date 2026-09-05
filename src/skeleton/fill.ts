import {
  distanceToProfile,
  envelopeMaxRadius,
  envelopeProfile,
  envelopeRadiusAt,
  type Envelope,
} from "../envelope";
import type { Skeleton } from "./colonize";
import { DEFAULT_SHED } from "./shed";

/* ------------------------------------------------------------------ *
 * FILL, MEASURED ON THE WOOD THE CALLER CLASSIFIES
 *
 * `terminal` names the nodes under measurement. The old pass supplies
 * appended nodes without children; the branch pass will supply its twig
 * marks. A zero-order skeleton therefore has no tested fine wood.
 *
 * Shell cells are equal-volume voxels whose CENTRES lie inside the
 * crown and pass shedding's predicate. The world-origin grid uses
 * `cellSize` as a fraction of envelope height (0.02 at rest). A cell
 * counts once if it holds at least one terminal position in the shell.
 * The fraction approximates occupied shell volume at this resolution;
 * it does not measure wood volume or credit a segment crossing a cell.
 * Surface-straddling cells are classified by centre on both sides of
 * the ratio. Out-of-crown terminals cannot inflate shell occupancy.
 *
 * Shell depth is DEFAULT_SHED.shellDepth * envelopeMaxRadius in METRES,
 * exactly as in shed.ts. The radial-slack shortcut and distanceToProfile
 * predicate are the shedder's too; the finite crown domain additionally
 * excludes the outside points that shedding conservatively retains.
 *
 * Clustering uses Euclidean distance in metres, inclusive at the bound.
 * Colonization tips have no children WITHIN [0, crossover), even if the
 * local pass later appended children. Each terminal counts once, however
 * many tips are nearby. No terminals, no tips, invalid inputs, or a grid
 * too large to measure are explicitly untested, never an invented zero.
 * ------------------------------------------------------------------ */

export type FillMeasurement =
  | { tested: true; fraction: number; numerator: number; denominator: number }
  | { tested: false; reason: string };

type Untested = Extract<FillMeasurement, { tested: false }>;
const untested = (reason: string): Untested => ({ tested: false, reason });
const fraction = (numerator: number, denominator: number): FillMeasurement =>
  ({ tested: true, fraction: numerator / denominator, numerator, denominator });

function terminals(
  skeleton: Skeleton,
  terminal: Uint8Array,
): number[] | Untested {
  if (terminal.length !== skeleton.nodes.length) {
    return untested("terminal-size-mismatch");
  }
  const indices: number[] = [];
  for (let i = 0; i < terminal.length; i += 1) {
    if (terminal[i] === 0) continue;
    const { x, y, z } = skeleton.nodes[i].position;
    if (![x, y, z].every(Number.isFinite)) return untested("non-finite-terminal");
    indices.push(i);
  }
  return indices.length ? indices : untested("no-terminals");
}

/** Fraction of shell voxels containing caller-classified terminal wood.
 *  `cellSize` is a height fraction, held constant across comparisons.
 *  More than two million candidate cells is reported as untested; the
 *  function never silently coarsens the caller's measurement resolution. */
export function shellOccupancy(
  skeleton: Skeleton,
  terminal: Uint8Array,
  envelope: Envelope,
  cellSize = 0.02,
): FillMeasurement {
  const selected = terminals(skeleton, terminal);
  if (!Array.isArray(selected)) return selected;
  if (!(cellSize > 0) || !Number.isFinite(cellSize)) {
    return untested("invalid-cell-size");
  }
  const { height, crownBase, spread, fullness, shoulder } = envelope;
  if (![height, crownBase, spread, fullness, shoulder].every(Number.isFinite)
    || height <= 0 || crownBase < 0 || crownBase >= 1 || spread <= 0 || shoulder <= 0) {
    return untested("invalid-envelope");
  }
  const size = cellSize * height;
  const maxRadius = envelopeMaxRadius(envelope);
  const base = height * crownBase;
  if (!Number.isFinite(size) || size <= 0 || !Number.isFinite(maxRadius)) {
    return untested("invalid-envelope");
  }
  const lo = Math.floor(-maxRadius / size);
  const hi = Math.ceil(maxRadius / size);
  const bottom = Math.floor(base / size);
  const top = Math.ceil(height / size);
  const cells = (hi - lo) ** 2 * (top - bottom);
  if (![lo, hi, bottom, top, cells].every(Number.isSafeInteger) || cells > 2_000_000) {
    return untested("grid-too-large");
  }

  const shell = DEFAULT_SHED.shellDepth * maxRadius;
  const profile = envelopeProfile(envelope);
  const inShell = (x: number, y: number, z: number): boolean => {
    if (y < base || y > height) return false;
    const r = Math.hypot(x, z);
    const slack = envelopeRadiusAt(envelope, y) - r;
    if (slack < 0) return false;
    return !(slack > shell) || !(distanceToProfile(profile, r, y) > shell);
  };
  const key = (x: number, y: number, z: number): string => `${x},${y},${z}`;
  const occupied = new Set<string>();
  for (const i of selected) {
    const { x, y, z } = skeleton.nodes[i].position;
    if (inShell(x, y, z)) {
      occupied.add(key(Math.floor(x / size), Math.floor(y / size), Math.floor(z / size)));
    }
  }

  let denominator = 0;
  let numerator = 0;
  for (let y = bottom; y < top; y += 1) {
    for (let x = lo; x < hi; x += 1) {
      for (let z = lo; z < hi; z += 1) {
        if (!inShell((x + 0.5) * size, (y + 0.5) * size, (z + 0.5) * size)) {
          continue;
        }
        denominator += 1;
        if (occupied.has(key(x, y, z))) numerator += 1;
      }
    }
  }
  return denominator ? fraction(numerator, denominator) : untested("no-shell-cells");
}

/** Fraction of terminals within `distance` metres of any colonization
 *  tip. Appended children do not remove a tip from the reference set. */
export function tipClustering(
  skeleton: Skeleton,
  crossover: number,
  terminal: Uint8Array,
  distance: number,
): FillMeasurement {
  const selected = terminals(skeleton, terminal);
  if (!Array.isArray(selected)) return selected;
  if (!Number.isInteger(crossover) || crossover < 1 || crossover > skeleton.nodes.length) {
    return untested("invalid-crossover");
  }
  if (!Number.isFinite(distance) || distance < 0) return untested("invalid-distance");
  const children = new Uint8Array(crossover);
  for (let i = 1; i < crossover; i += 1) {
    const parent = skeleton.nodes[i].parent;
    if (!Number.isInteger(parent) || parent < 0 || parent >= i) {
      return untested("invalid-colonization-topology");
    }
    children[parent] = 1;
  }
  const tips = [];
  for (let i = 1; i < crossover; i += 1) {
    if (children[i] !== 0) continue;
    const position = skeleton.nodes[i].position;
    if (![position.x, position.y, position.z].every(Number.isFinite)) {
      return untested("non-finite-colonization-tip");
    }
    tips.push(position);
  }
  if (!tips.length) return untested("no-colonization-tips");
  let clustered = 0;
  for (const i of selected) {
    const position = skeleton.nodes[i].position;
    for (const tip of tips) {
      if (Math.hypot(position.x - tip.x, position.y - tip.y, position.z - tip.z) <= distance) {
        clustered += 1;
        break;
      }
    }
  }
  return fraction(clustered, selected.length);
}
