/* ------------------------------------------------------------------ *
 * THE FOLIAGE ELEMENT
 *
 * One leaf, as geometry. The canopy instances this mesh tens of
 * thousands of times, so every decision in here is a decision about
 * fill rate as much as about shape.
 *
 *   REAL GEOMETRY, NOT A CARD, and the vendor data is what settles
 *   it. A circular sprite drawn on its best-fit quad wastes 22% of the
 *   fragments the GPU processes; a dodecagon wastes 3%; and the
 *   recommendation is to spend the triangles on the fit, because
 *   rendering less transparency is worth more than the silhouette
 *   costs. A leaf fills less of its quad than a circle does, so the
 *   quad's waste is worse than 22% - the blade below covers 0.58 of
 *   its bounding quad, so two fifths of the fragments that quad would
 *   rasterize are bought back by sixteen triangles. The quad is
 *   still reachable, as `card`, because it is the comparison this
 *   claim is stated against - not because it is the cheaper option.
 *
 *   THE LOCAL FRAME is the seam between this file and placement. The
 *   element is authored with its petiole at the origin, its axis along
 *   +Y and its face normal along +Z. Placement emits a transform
 *   against that frame and never reads a vertex; this file never
 *   learns where the leaf ends up. That is the whole contract between
 *   them, and it is why the two are built at the same time rather than
 *   one waiting on the other.
 *
 *   THE BLADE IS A HEIGHT FIELD over the xy plane: x across the width,
 *   y along the axis, z from the two curvature dials and nothing else.
 *   Two things follow by construction rather than by tuning. Every
 *   triangle projects to a counter-clockwise triangle of positive
 *   area, so no parameter - including a hostile one - can produce a
 *   degenerate triangle; and every triangle's normal has a positive z
 *   component, so the face normal is a property of how the mesh is
 *   built rather than of the winding having been got right by hand.
 *
 *   FLAT, IN CLAY, AND ONE-SIDED. No colour, no material, no texture
 *   coordinates: venation, albedo and translucency belong to the
 *   texturing spec, and this one ships the shape the canopy is judged
 *   on. The blade is a single sheet meant to be drawn double-sided. A
 *   leaf encloses no volume, so a closed shell would double the
 *   triangle count to hide a back face of exactly the same shape.
 *
 * Pure and deterministic in its arguments. No seed reaches here: the
 * per-instance variation is placement's, not the element's.
 * ------------------------------------------------------------------ */

export interface ElementParams {
  /** How long the blade is, petiole to tip, in metres. This is the
   *  element's scale: nothing else in here is absolute. 0.05 to 0.2 is
   *  a broadleaf beside a 1.8 m figure; larger reads as a frond.
   *  Held above zero - a blade of no length is a band of degenerate
   *  triangles rather than a small leaf. */
  length: number;
  /** How wide the blade is at its widest point, in metres, across the
   *  full width rather than to one side. Against a 0.12 m length, 0.06
   *  is an ordinary leaf and 0.02 is a willow. Held above zero for the
   *  same reason as `length`. */
  width: number;
  /** Where along the length the blade is widest, as a fraction of it.
   *  0.5 is elliptic, below is ovate (widest near the base, the
   *  commonest broadleaf shape), above is obovate (widest near the
   *  tip, a teardrop). Held inside (0, 1): at either end one half of
   *  the outline has no length to be drawn over. */
  widestAt: number;
  /** How fast the blade fills out from the petiole, as an exponent on
   *  the basal half of the outline. 1 is a plain rounded base; below 1
   *  the blade widens immediately and reads as truncate or heart-
   *  shaped; above 1 it holds narrow and reads as a wedge. Held in
   *  0.2 to 8 - at 0 the outline is a rectangle, which is the card
   *  this element exists not to be. */
  baseFullness: number;
  /** How sharply the blade closes to its tip, as an exponent on the
   *  apical half of the outline. 1 is a plain rounded apex; above 1 it
   *  draws out to a point (2 or so is acuminate, the shape that reads
   *  as a leaf at a glance); below 1 is blunt or notched-looking. Same
   *  0.2 to 8 rail, for the same reason. */
  tipSharpness: number;
  /** How far the margins lift out of the blade's plane relative to the
   *  midrib, as a fraction of the local half width, so the fold angle
   *  is the same at the tip as at the base. 0 is a flat leaf, which
   *  reads as paper under any light; 0.2 is a shallow channel; 1 is
   *  nearly folded shut. Signed: negative cups the other way. This and
   *  `curl` are the whole of the element's third dimension. */
  cup: number;
  /** How far the tip swings out of the blade's plane along z, as a
   *  fraction of the length, applied as a quadratic bend so the blade
   *  leaves the petiole flat and the curvature accumulates toward the
   *  tip. 0 is a flat leaf; 0.15 is the droop of a leaf carrying its
   *  own weight. Signed: positive curls toward the face normal. */
  curl: number;
  /** Stations along the axis, petiole to tip. The resolution of the
   *  outline and half of the cost: triangles are
   *  `2 * crossSegments * (axialSegments - 1)`. Below about 4 the
   *  margin reads as a straight-sided kite; above about 10 the leaf is
   *  spending vertices on a curve a millimetre across, at 50,000
   *  instances. Rounded to an integer, held to 2 or more - at 1 there
   *  is nothing between the petiole and the tip. */
  axialSegments: number;
  /** Columns across the width. 2 is margin, midrib, margin, and it is
   *  enough: the outline's fit lives along the axis, not across it,
   *  and every column past the first two costs triangles without
   *  changing the silhouette. Higher only buys a smoother cup. Rounded
   *  to an even integer, so there is always a column of vertices on
   *  the midrib itself - the cup's ridge line, and the only column
   *  whose normal is the face normal. */
  crossSegments: number;
  /** Draw the bounding quad instead of the blade: two triangles, four
   *  vertices, flat, spanning the same length and width. Off, and the
   *  default is the fitted outline - this exists because R4's claim is
   *  stated against the quad and a claim wants its comparison
   *  reachable, not because a quad is a cheaper leaf. It is the only
   *  parameter here that is not a number; anything that is not exactly
   *  `true` is off. */
  card: boolean;
}

/** The element every canopy parameter set is a departure from: an
 *  ovate blade 12 cm long and 6 cm wide, drawn out to a point, cupped
 *  along its midrib and drooping a little at the tip. Sixteen
 *  triangles - inside the 8-to-20 band the canopy's triangle budget is
 *  stated in, and 0.58 of the fill its bounding quad would cost. Not
 *  either of the Two Trees: the presets author those. */
export const DEFAULT_ELEMENT: ElementParams = {
  length: 0.12,
  width: 0.06,
  widestAt: 0.42,
  baseFullness: 0.85,
  tipSharpness: 1.6,
  cup: 0.18,
  curl: 0.12,
  axialSegments: 5,
  crossSegments: 2,
  card: false,
};

/* Rails on the parameters, none of them art direction. Each one is a
   boundary past which the arithmetic stops describing a leaf: a blade
   with no length or no width is a band of degenerate triangles rather
   than a small element, an outline whose widest point sits at either
   end has half an outline to draw over no length at all, and every
   ceiling is here because these numbers are multiplied into vertices
   stored as float32 - a finite but enormous parameter arrives as
   Infinity, and a non-finite vertex takes the whole instanced draw
   with it. */
const MIN_LENGTH = 1e-4;
const MAX_LENGTH = 1e3;
const MIN_WIDTH = 1e-4;
const MAX_WIDTH = 1e3;
const MIN_WIDEST_AT = 0.05;
const MAX_WIDEST_AT = 0.95;
const MIN_PROFILE_EXPONENT = 0.2;
const MAX_PROFILE_EXPONENT = 8;
const MAX_CUP = 2;
const MAX_CURL = 2;
const MIN_AXIAL_SEGMENTS = 2;
const MAX_AXIAL_SEGMENTS = 64;
const MIN_CROSS_SEGMENTS = 2;
/** Even, so that rounding an odd request up to the next even column
 *  count can never step past the ceiling. */
const MAX_CROSS_SEGMENTS = 64;

/* Three of those rails - the segment ceiling, the widest-point rails
   and the exponent ceiling - are together what keep the blade from
   narrowing to nothing between its ends, which is the failure worth
   naming here: a row
   of zero width is not a finer leaf, it is a row of zero-area
   triangles wearing a valid index buffer, and the same mistake one
   dimension up - a predicate written only in terms of a width, in the
   region where the width is zero - is what let an earlier stage grow
   branches below its own envelope. The outline reaches zero only at
   t=0 and t=1, where a single vertex closes it. The nearest row to
   either end is 1/MAX_AXIAL_SEGMENTS away in t, the widest point can
   be no closer than MIN_WIDEST_AT to that end, and the exponent can
   be no larger than MAX_PROFILE_EXPONENT - so the narrowest row this
   can draw is about 2e-13 of the half width, twenty-eight orders
   above where a float32 underflows to zero. It is a bound with a
   proof rather than a clamp with a hope, and the "holds a hostile
   parameter" test drives exactly that corner, so loosening any of the
   three without redoing the argument fails there. */

const HALF_PI = Math.PI / 2;

/** One open sheet of triangles - a leaf has no volume to enclose - in
 *  the form three.js wants it, and the same field set the swept
 *  surface returns: positions and indices only, because normals come
 *  from the winding and materials are the consumer's business. */
export interface ElementMesh {
  /** xyz per vertex, in metres, in the element's local frame: petiole
   *  at the origin, axis along +Y, face normal along +Z. */
  positions: Float32Array;
  /** Three vertex indices per triangle, wound counter-clockwise seen
   *  from +Z, which is the face the leaf presents. */
  indices: Uint32Array;
  triangles: number;
  vertices: number;
}

/* NaN is the one that has to be named separately - `Math.max`
   propagates it rather than clamping it - and a NaN vertex is the kind
   of thing that reaches the screen as an invisible canopy rather than
   as an error. Deliberately local: the same rail block appears in
   radius.ts and surface.ts, and it is duplicated per call site rather
   than shared. */
const held = (value: number, fallback: number): number =>
  Number.isFinite(value) ? value : fallback;

/**
 * Builds one foliage element in its local frame.
 *
 * Guarantees, all structural rather than tuned:
 *   - the petiole is exactly at the origin, the blade spans `[0,
 *     length]` along +Y, and the tip is the single vertex farthest
 *     along it;
 *   - every triangle has area, and every vertex is finite, at any
 *     parameters whatever - including non-finite ones;
 *   - every triangle's normal has a positive z component, so the sheet
 *     faces +Z everywhere;
 *   - identical arguments give an identical mesh, vertex for vertex.
 */
export function buildElement(params: ElementParams): ElementMesh {
  const length = Math.min(
    MAX_LENGTH,
    Math.max(MIN_LENGTH, held(params.length, DEFAULT_ELEMENT.length)),
  );
  const halfWidth =
    Math.min(
      MAX_WIDTH,
      Math.max(MIN_WIDTH, held(params.width, DEFAULT_ELEMENT.width)),
    ) / 2;

  // Anything that is not exactly `true` is the documented default, so
  // a caller who passes a truthy non-boolean gets the fitted blade
  // rather than a quad they did not ask for by name.
  if (params.card === true) {
    /* The bounding quad of the blade above: same length, same width,
       flat, two triangles, and the petiole at the middle of its lower
       edge rather than at a vertex - a quad has no petiole. Kept
       exactly this plain because it is the baseline the fit is
       measured against; anything fitted about it would flatter the
       comparison. */
    return {
      positions: new Float32Array([
        -halfWidth, 0, 0,
        halfWidth, 0, 0,
        halfWidth, length, 0,
        -halfWidth, length, 0,
      ]),
      indices: new Uint32Array([0, 1, 2, 0, 2, 3]),
      triangles: 2,
      vertices: 4,
    };
  }

  const widestAt = Math.min(
    MAX_WIDEST_AT,
    Math.max(MIN_WIDEST_AT, held(params.widestAt, DEFAULT_ELEMENT.widestAt)),
  );
  const baseFullness = Math.min(
    MAX_PROFILE_EXPONENT,
    Math.max(
      MIN_PROFILE_EXPONENT,
      held(params.baseFullness, DEFAULT_ELEMENT.baseFullness),
    ),
  );
  const tipSharpness = Math.min(
    MAX_PROFILE_EXPONENT,
    Math.max(
      MIN_PROFILE_EXPONENT,
      held(params.tipSharpness, DEFAULT_ELEMENT.tipSharpness),
    ),
  );
  const cup = Math.min(
    MAX_CUP,
    Math.max(-MAX_CUP, held(params.cup, DEFAULT_ELEMENT.cup)),
  );
  const curl = Math.min(
    MAX_CURL,
    Math.max(-MAX_CURL, held(params.curl, DEFAULT_ELEMENT.curl)),
  );
  const stations = Math.min(
    MAX_AXIAL_SEGMENTS,
    Math.max(
      MIN_AXIAL_SEGMENTS,
      Math.round(held(params.axialSegments, DEFAULT_ELEMENT.axialSegments)),
    ),
  );
  const requested = Math.min(
    MAX_CROSS_SEGMENTS,
    Math.max(
      MIN_CROSS_SEGMENTS,
      Math.round(held(params.crossSegments, DEFAULT_ELEMENT.crossSegments)),
    ),
  );
  // Up to the next even count, never down: rounding down would spend
  // the caller's request on a mesh with no midrib column in it.
  const columns = requested % 2 === 0 ? requested : requested + 1;

  /* The outline. Two half-waves that meet at the widest point, each
     raised to its own exponent: sine from the petiole, cosine to the
     tip. Both are flat where they meet, so the widest point is a
     smooth shoulder rather than a corner, and both reach exactly zero
     at their end of the blade, so the outline closes on the petiole
     and on the tip without a seam. */
  const profile = (t: number): number =>
    t <= widestAt
      ? Math.sin(HALF_PI * (t / widestAt)) ** baseFullness
      : Math.cos(HALF_PI * ((t - widestAt) / (1 - widestAt))) ** tipSharpness;

  const rows = stations - 1;
  const vertices = 2 + rows * (columns + 1);
  const triangles = 2 * columns * rows;
  const positions = new Float32Array(vertices * 3);
  const indices = new Uint32Array(triangles * 3);

  // The petiole: the origin itself, and the vertex the whole local
  // frame is stated against.
  positions[0] = 0;
  positions[1] = 0;
  positions[2] = 0;

  for (let row = 0; row < rows; row += 1) {
    const t = (row + 1) / stations;
    const halfAt = halfWidth * profile(t);
    // Quadratic in t, so the blade leaves the petiole in the xy plane
    // and the bend accumulates toward the tip - the shape a leaf takes
    // under its own weight, rather than a hinge at the stem.
    const midrib = curl * length * t * t;
    for (let column = 0; column <= columns; column += 1) {
      const across = (2 * column) / columns - 1;
      const at = (1 + row * (columns + 1) + column) * 3;
      positions[at] = across * halfAt;
      positions[at + 1] = t * length;
      // Squared, so the midrib is a smooth ridge rather than a crease,
      // and scaled by the local half width, so the fold angle is
      // constant along the blade instead of pinching at the tip.
      positions[at + 2] = midrib + cup * halfAt * across * across;
    }
  }

  // The tip: on the axis, at the full length, carrying the whole of
  // the curl.
  const tip = (vertices - 1) * 3;
  positions[tip] = 0;
  positions[tip + 1] = length;
  positions[tip + 2] = curl * length;

  /* Winding, once, for the three kinds of triangle. Every one of them
     is stated left-to-right seen from +Z, which is what makes the
     projection to xy counter-clockwise and the normal +Z. */
  const base = 0;
  const apex = vertices - 1;
  const first = (row: number, column: number): number =>
    1 + row * (columns + 1) + column;
  let write = 0;
  const triangle = (a: number, b: number, c: number): void => {
    indices[write] = a;
    indices[write + 1] = b;
    indices[write + 2] = c;
    write += 3;
  };

  // A fan from the petiole into the first row.
  for (let column = 0; column < columns; column += 1) {
    triangle(base, first(0, column + 1), first(0, column));
  }
  // The blade between the rows.
  for (let row = 0; row + 1 < rows; row += 1) {
    for (let column = 0; column < columns; column += 1) {
      const a = first(row, column);
      const b = first(row, column + 1);
      const c = first(row + 1, column + 1);
      const d = first(row + 1, column);
      triangle(a, b, c);
      triangle(a, c, d);
    }
  }
  // A fan from the last row into the tip.
  for (let column = 0; column < columns; column += 1) {
    triangle(first(rows - 1, column), first(rows - 1, column + 1), apex);
  }

  return { positions, indices, triangles, vertices };
}
