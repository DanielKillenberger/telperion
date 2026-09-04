import * as THREE from "three";

import type { ElementMesh } from "./element";
import type { Canopy } from "./place";

/* ------------------------------------------------------------------ *
 * THE SCREEN-SPACE SILHOUETTE
 *
 * Culling is only worth doing up to the moment it eats the outline,
 * and the outline is what the eye reads. That makes "the silhouette is
 * unchanged" the one claim interior culling has to answer for - and
 * nothing else in this library projects geometry to a screen at all,
 * so the measurement has to exist before the claim can be made. This
 * file is that measurement and nothing more. It draws no material, it
 * needs no GPU and no canvas, and it runs in the same Node box the
 * rest of the suite runs in.
 *
 * WHAT A SILHOUETTE IS HERE. Every element's triangles are pushed
 * through its own instance transform, projected along a view
 * direction, and their edges are stamped into a fixed pixel grid.
 * Then, for each column of that grid, the topmost and bottommost
 * stamped pixel are kept and everything between them is counted as
 * covered. That last step is the whole definition: the outline is the
 * boundary of the canopy as drawn, and a canopy thinned from the
 * inside has holes in it that the eye never sees against a tree's
 * own depth. Filling each column between its extremes is what
 * separates "the outline moved" from "the interior got thinner",
 * which is precisely the distinction culling is judged on.
 *
 * WHY EDGES RATHER THAN FILLED TRIANGLES. A filled triangle's topmost
 * and bottommost pixel in any column both sit on its boundary, so
 * stamping the three edges gives the same per-column extremes for a
 * fraction of the work. It also survives the scale this actually runs
 * at: a 12 cm leaf on a 24 m tree covers about a pixel and a half at
 * this resolution, and a sub-pixel triangle rasterized by sampling
 * pixel centres can vanish entirely. An edge walk always stamps its
 * endpoints, so no element drops out of the outline for being small.
 *
 * ONE FRAME, TWO CANOPIES. A before-and-after comparison is only
 * meaningful in a fixed frame, so the bounds are computed once from
 * the canopy being compared against and reused verbatim for the other.
 * Building the after-canopy its own frame would rescale the picture
 * and hide exactly the shrinkage the comparison exists to catch.
 * ------------------------------------------------------------------ */

/** Pixels across and down. A 24 m tree at 256 puts a default 12 cm
 *  leaf at about a pixel and a half, which is the regime the
 *  edge-walk above is chosen for: fine enough that the outline is a
 *  curve rather than a staircase, coarse enough that a canopy of
 *  tens of thousands of elements is a few hundred milliseconds. */
export const DEFAULT_RESOLUTION = 256;

/** Fraction of the frame left empty around the geometry, so the
 *  outline is never flush against the edge where a clamped pixel
 *  would read as a straight edge of the tree. */
const MARGIN = 0.04;

/** A stop, not a target: an edge longer than this in pixels is
 *  sampled at this many steps rather than at one step per pixel.
 *  Nothing reachable through `screenFor` produces one - the frame is
 *  built from the geometry it will hold - so this bounds a hostile
 *  or non-finite transform rather than the working case. */
const MAX_EDGE_STEPS = 4 * DEFAULT_RESOLUTION;

const TINY = 1e-12;

/** A direction to look from, with the screen basis that goes with it.
 *  `right` and `up` span the image plane and `direction` is what the
 *  projection throws away. */
export interface View {
  /** Unit vector the camera looks along. */
  direction: THREE.Vector3;
  /** Unit vector, screen x. */
  right: THREE.Vector3;
  /** Unit vector, screen y, pointing up in world terms wherever the
   *  view is not straight down the world's own axis. */
  up: THREE.Vector3;
}

/** The screen basis for looking along `direction`. World up is the
 *  roll reference, and a view straight up or down - where world up
 *  says nothing about roll - falls back to the world's z axis. */
export function viewAlong(direction: THREE.Vector3): View {
  const along = direction.clone();
  if (!(along.lengthSq() > TINY)) along.set(0, 0, 1);
  along.normalize();

  const right = new THREE.Vector3().crossVectors(
    new THREE.Vector3(0, 1, 0),
    along,
  );
  if (!(right.lengthSq() > TINY)) {
    right.crossVectors(new THREE.Vector3(0, 0, 1), along);
  }
  right.normalize();

  const up = new THREE.Vector3().crossVectors(along, right).normalize();
  return { direction: along, right, up };
}

/** The fixed set of directions a canopy is judged from.
 *
 *  Four horizontal quarter turns and two looking down from thirty
 *  degrees up. Horizontal is where a tree is looked at and where the
 *  crown's own width is on show; the elevated pair is what catches a
 *  culler that keeps a convincing waistline and hollows out the top,
 *  which no horizontal view can distinguish from honest thinning.
 *  Only a half turn is needed at each elevation, because the outline
 *  seen from the opposite side is this one mirrored. */
export const SILHOUETTE_VIEWS: readonly View[] = [
  viewAlong(new THREE.Vector3(1, 0, 0)),
  viewAlong(new THREE.Vector3(1, 0, 1)),
  viewAlong(new THREE.Vector3(0, 0, 1)),
  viewAlong(new THREE.Vector3(-1, 0, 1)),
  viewAlong(new THREE.Vector3(0.866, -0.5, 0)),
  viewAlong(new THREE.Vector3(0, -0.5, 0.866)),
];

/** Where the geometry lands in pixels. Built from one canopy and then
 *  reused for the other, which is what makes two silhouettes
 *  comparable at all. */
export interface Screen {
  view: View;
  /** Pixels across and down. */
  resolution: number;
  /** Pixels per world metre. */
  scale: number;
  /** Screen-space u of the left edge, in world units. */
  originU: number;
  /** Screen-space v of the top edge, in world units. */
  originV: number;
}

/** The outline of one canopy, as columns of a fixed grid.
 *
 *  `top` and `bottom` hold the extreme stamped row in each column, or
 *  -1 for a column the canopy never reaches. `area` counts every pixel
 *  between those extremes inclusive - the filled outline, not the
 *  drawn coverage. */
export interface Silhouette {
  screen: Screen;
  top: Int32Array;
  bottom: Int32Array;
  /** Filled pixels: the size of the shape the eye reads. */
  area: number;
  /** Columns the canopy reaches at all: the width of that shape. */
  columns: number;
}

/** What one canopy lost against another, in that canopy's own frame. */
export interface SilhouetteChange {
  /** Filled pixels in `before` that `after` does not cover. */
  lost: number;
  /** `before`'s filled area, the denominator below. */
  area: number;
  /** `lost / area` - and 1, not 0, when there was no silhouette to
   *  begin with. A comparison against nothing is not evidence that
   *  nothing changed, and a fraction of 0/0 read as "unchanged" is
   *  how a vacuous test passes on a culler that removed everything. */
  fraction: number;
}

/** Transforms every vertex of `element` under instance `index` of
 *  `canopy` into `out` as world xyz, and returns how many it wrote. */
function instanceVertices(
  element: ElementMesh,
  canopy: Canopy,
  index: number,
  out: Float64Array,
): number {
  const m = canopy.matrices;
  const o = index * 16;
  const positions = element.positions;
  const vertices = (positions.length / 3) | 0;

  for (let v = 0; v < vertices; v += 1) {
    const x = positions[v * 3];
    const y = positions[v * 3 + 1];
    const z = positions[v * 3 + 2];
    out[v * 3] = m[o] * x + m[o + 4] * y + m[o + 8] * z + m[o + 12];
    out[v * 3 + 1] = m[o + 1] * x + m[o + 5] * y + m[o + 9] * z + m[o + 13];
    out[v * 3 + 2] = m[o + 2] * x + m[o + 6] * y + m[o + 10] * z + m[o + 14];
  }
  return vertices;
}

/** How many whole instances `canopy` actually carries, which is the
 *  smaller of what it claims and what its buffer holds. */
function instanceCount(canopy: Canopy): number {
  const claimed = Number.isFinite(canopy.count) ? Math.floor(canopy.count) : 0;
  return Math.max(0, Math.min(claimed, Math.floor(canopy.matrices.length / 16)));
}

/**
 * The frame `canopy` fits in, looked at along `view`.
 *
 * Square pixels and a square grid: the two screen axes share one
 * scale, so an outline compared across views is compared at one
 * measure. Non-finite vertices are left out of the bounds rather than
 * poisoning them, and a canopy with no extent at all still yields a
 * usable frame - it simply has no silhouette in it.
 */
export function screenFor(
  element: ElementMesh,
  canopy: Canopy,
  view: View,
  resolution: number = DEFAULT_RESOLUTION,
): Screen {
  const size = Math.max(2, Math.floor(resolution));
  const count = instanceCount(canopy);
  const vertices = (element.positions.length / 3) | 0;
  const world = new Float64Array(Math.max(3, vertices * 3));

  let minU = Infinity;
  let maxU = -Infinity;
  let minV = Infinity;
  let maxV = -Infinity;

  for (let i = 0; i < count; i += 1) {
    instanceVertices(element, canopy, i, world);
    for (let v = 0; v < vertices; v += 1) {
      const x = world[v * 3];
      const y = world[v * 3 + 1];
      const z = world[v * 3 + 2];
      const u = x * view.right.x + y * view.right.y + z * view.right.z;
      const w = x * view.up.x + y * view.up.y + z * view.up.z;
      if (!Number.isFinite(u) || !Number.isFinite(w)) continue;
      if (u < minU) minU = u;
      if (u > maxU) maxU = u;
      if (w < minV) minV = w;
      if (w > maxV) maxV = w;
    }
  }

  if (!Number.isFinite(minU) || !Number.isFinite(minV)) {
    minU = 0;
    maxU = 0;
    minV = 0;
    maxV = 0;
  }

  const span = Math.max(maxU - minU, maxV - minV, TINY);
  const scale = (size * (1 - 2 * MARGIN)) / span;
  const centreU = (minU + maxU) / 2;
  const centreV = (minV + maxV) / 2;

  return {
    view,
    resolution: size,
    scale,
    originU: centreU - size / (2 * scale),
    originV: centreV + size / (2 * scale),
  };
}

/**
 * The outline of `canopy` in `screen`'s frame.
 *
 * Every triangle edge of every instance is walked in pixel space and
 * the extreme rows per column are kept. Pixels off the grid are
 * dropped: the frame is built from the geometry it holds, so this only
 * discards a transform that reaches outside its own canopy's bounds.
 */
export function silhouetteOf(
  element: ElementMesh,
  canopy: Canopy,
  screen: Screen,
): Silhouette {
  const size = screen.resolution;
  const top = new Int32Array(size).fill(-1);
  const bottom = new Int32Array(size).fill(-1);

  const count = instanceCount(canopy);
  const vertices = (element.positions.length / 3) | 0;
  const world = new Float64Array(Math.max(3, vertices * 3));
  const px = new Float64Array(Math.max(1, vertices));
  const py = new Float64Array(Math.max(1, vertices));
  const indices = element.indices;
  const view = screen.view;

  const stamp = (column: number, row: number): void => {
    if (column < 0 || column >= size || row < 0 || row >= size) return;
    if (top[column] < 0 || row < top[column]) top[column] = row;
    if (row > bottom[column]) bottom[column] = row;
  };

  const edge = (a: number, b: number): void => {
    const x0 = px[a];
    const y0 = py[a];
    const x1 = px[b];
    const y1 = py[b];
    if (
      !Number.isFinite(x0) ||
      !Number.isFinite(y0) ||
      !Number.isFinite(x1) ||
      !Number.isFinite(y1)
    ) {
      return;
    }
    const steps = Math.min(
      MAX_EDGE_STEPS,
      Math.max(1, Math.ceil(Math.max(Math.abs(x1 - x0), Math.abs(y1 - y0)))),
    );
    for (let s = 0; s <= steps; s += 1) {
      const t = s / steps;
      stamp(
        Math.floor(x0 + (x1 - x0) * t),
        Math.floor(y0 + (y1 - y0) * t),
      );
    }
  };

  for (let i = 0; i < count; i += 1) {
    instanceVertices(element, canopy, i, world);
    for (let v = 0; v < vertices; v += 1) {
      const x = world[v * 3];
      const y = world[v * 3 + 1];
      const z = world[v * 3 + 2];
      const u = x * view.right.x + y * view.right.y + z * view.right.z;
      const w = x * view.up.x + y * view.up.y + z * view.up.z;
      px[v] = (u - screen.originU) * screen.scale;
      py[v] = (screen.originV - w) * screen.scale;
    }
    for (let t = 0; t + 2 < indices.length; t += 3) {
      const a = indices[t];
      const b = indices[t + 1];
      const c = indices[t + 2];
      edge(a, b);
      edge(b, c);
      edge(c, a);
    }
  }

  let area = 0;
  let columns = 0;
  for (let column = 0; column < size; column += 1) {
    if (top[column] < 0) continue;
    columns += 1;
    area += bottom[column] - top[column] + 1;
  }

  return { screen, top, bottom, area, columns };
}

/**
 * What `after` lost of `before`'s outline, column by column.
 *
 * Both silhouettes must have been measured in the same frame; the
 * comparison is meaningless otherwise, and there is no way to detect
 * that from the numbers alone. The intersection is taken per column
 * rather than assuming `after` is contained in `before`, so the
 * measure stays honest if it is ever handed two unrelated canopies.
 */
export function silhouetteChange(
  before: Silhouette,
  after: Silhouette,
): SilhouetteChange {
  const size = Math.min(before.top.length, after.top.length);
  let lost = 0;

  for (let column = 0; column < size; column += 1) {
    const from = before.top[column];
    if (from < 0) continue;
    const to = before.bottom[column];
    const span = to - from + 1;

    const afterFrom = after.top[column];
    if (afterFrom < 0) {
      lost += span;
      continue;
    }
    const afterTo = after.bottom[column];
    const overlap = Math.min(to, afterTo) - Math.max(from, afterFrom) + 1;
    lost += span - Math.max(0, overlap);
  }

  // Columns `before` reaches that `after`'s grid does not even hold.
  for (let column = size; column < before.top.length; column += 1) {
    if (before.top[column] < 0) continue;
    lost += before.bottom[column] - before.top[column] + 1;
  }

  return {
    lost,
    area: before.area,
    fraction: before.area > 0 ? lost / before.area : 1,
  };
}

/**
 * One pixel of the raster's own resolution, as a fraction of
 * `silhouette`'s filled area: the outline moved inward by a single
 * pixel at the top and the bottom of every column it reaches.
 *
 * This is the floor under any comparison made at this resolution.
 * A change smaller than this is not a change the measurement can
 * distinguish from where it chose to put its pixel boundaries, and a
 * tolerance is only honest when it is sized by the failure it is
 * correcting for - here, the raster's own quantisation - rather than
 * by the size of the thing it is protecting.
 */
export function silhouettePixelFloor(silhouette: Silhouette): number {
  if (!(silhouette.area > 0)) return 0;
  return (2 * silhouette.columns) / silhouette.area;
}
