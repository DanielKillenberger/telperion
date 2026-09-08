/* ------------------------------------------------------------------ *
 * WALKING ROUND THE TREE
 *
 * Where the camera stands, as the three numbers a hand at a mouse
 * actually moves: how far round, how far up, how far back. Pure, and
 * deliberately so - the renderer owns the pose and the page owns the
 * pointer, and this is the arithmetic in between, which is the only
 * part of an orbit a box with no GPU can hold to account.
 *
 * The clamps are the old clay room's, kept because they are about the
 * room and not about the library that drew it: the eye never sinks
 * below the ground the tree stands on, never reaches the pole where
 * the up direction stops being a direction, and never travels so far
 * either way that the subject is a speck or the near plane is inside
 * the trunk.
 * ------------------------------------------------------------------ */

/** Metres, Y up, right-handed - the renderer's own frame. Stated here
 *  rather than imported so this module needs no renderer at all. */
export type Point = readonly [number, number, number];

export interface Orbit {
  /** What the camera looks at, and turns about. */
  readonly target: Point;
  /** Radians about the vertical axis, measured from +Z toward +X. */
  readonly yaw: number;
  /** Radians above the horizon. Never at or below it, never at the pole. */
  readonly elevation: number;
  /** Metres from the target. */
  readonly distance: number;
  /** How near and how far this subject may be looked at from. */
  readonly closest: number;
  readonly furthest: number;
}

/** A full turn across eight hundred pixels of drag: a whole circuit of
 *  the tree in one comfortable sweep of the hand, and fine enough that
 *  a single pixel is a fraction of a degree. */
const TURN_PER_PIXEL = (Math.PI * 2) / 800;

/** What one notch of the wheel does to the distance. Multiplicative,
 *  because the subject spans two orders of magnitude and a fixed step
 *  in metres is either nothing at a Telperion or the whole way in at a
 *  sapling. */
const PUSH_PER_NOTCH = 1.12;

/** How close to the horizon and to the pole the eye may come, in
 *  radians. Below the horizon the camera is underground; at the pole
 *  the up direction is undefined and the picture rolls. */
const MARGIN = Math.PI * 0.005;

/** How far in and out of the framed distance the orbit reaches. Enough
 *  to get in among the branches and enough to pull back and see the
 *  tree whole, and no more: a limit on the wrong side of where the
 *  camera stands is a camera move however silently it happens. */
const CLOSEST = 1 / 16;
const FURTHEST = 8;

function clamp(value: number, low: number, high: number): number {
  return Math.min(high, Math.max(low, value));
}

/** The orbit that stands where this pose stands. The framed distance
 *  sets the travel limits, so every subject is orbited in proportion to
 *  its own size rather than in absolute metres. */
export function orbitOf(position: Point, target: Point): Orbit {
  const [x, y, z] = [position[0] - target[0], position[1] - target[1], position[2] - target[2]];
  const distance = Math.hypot(x, y, z);
  const flat = Math.hypot(x, z);
  return {
    target,
    yaw: Math.atan2(x, z),
    elevation: clamp(Math.atan2(y, flat), MARGIN, Math.PI / 2 - MARGIN),
    distance,
    closest: distance * CLOSEST,
    furthest: distance * FURTHEST,
  };
}

/** The orbit after a drag of this many pixels. Right drags the tree's
 *  near side toward the hand, which is the direction a hand on a model
 *  expects, and up lifts the eye. */
export function turn(orbit: Orbit, dx: number, dy: number): Orbit {
  return {
    ...orbit,
    yaw: orbit.yaw - dx * TURN_PER_PIXEL,
    elevation: clamp(
      orbit.elevation + dy * TURN_PER_PIXEL,
      MARGIN,
      Math.PI / 2 - MARGIN,
    ),
  };
}

/** The orbit after this many notches of the wheel. Positive pushes the
 *  eye away, which is what a wheel scrolled down does everywhere else. */
export function push(orbit: Orbit, notches: number): Orbit {
  return {
    ...orbit,
    distance: clamp(
      orbit.distance * PUSH_PER_NOTCH ** notches,
      orbit.closest,
      orbit.furthest,
    ),
  };
}

/** A drag across eight hundred pixels slides the subject one framed
 *  distance across the picture: the same sweep of the hand that turns
 *  it once round, so the two moves feel like one instrument. */
const SLIDE_PER_PIXEL = 1 / 800;

/** The orbit after a shifted drag: the target slides across the picture
 *  plane so the subject follows the hand, and the eye goes with it.
 *  Distance and angles are untouched, so a slide never changes what the
 *  wheel and the turn are calibrated to. */
export function pan(orbit: Orbit, dx: number, dy: number): Orbit {
  const step = orbit.distance * SLIDE_PER_PIXEL;
  const [sinYaw, cosYaw] = [Math.sin(orbit.yaw), Math.cos(orbit.yaw)];
  const [sinUp, cosUp] = [Math.sin(orbit.elevation), Math.cos(orbit.elevation)];
  // The picture's right and up, in the room's metres.
  const right: Point = [cosYaw, 0, -sinYaw];
  const up: Point = [-sinUp * sinYaw, cosUp, -sinUp * cosYaw];
  const [x, y, z] = orbit.target;
  return {
    ...orbit,
    target: [
      x + (up[0] * dy - right[0] * dx) * step,
      y + (up[1] * dy - right[1] * dx) * step,
      z + (up[2] * dy - right[2] * dx) * step,
    ],
  };
}

/** Where the eye stands, in the renderer's own metres. */
export function eyeOf(orbit: Orbit): Point {
  const flat = orbit.distance * Math.cos(orbit.elevation);
  return [
    orbit.target[0] + flat * Math.sin(orbit.yaw),
    orbit.target[1] + orbit.distance * Math.sin(orbit.elevation),
    orbit.target[2] + flat * Math.cos(orbit.yaw),
  ];
}
