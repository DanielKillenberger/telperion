import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";

/* ------------------------------------------------------------------ *
 * THE CLAY ROOM
 *
 * One grey material, a neutral sky, a ground plane, an orbit camera,
 * and a 1.8 m figure at the roots. That is the whole room, and it is
 * deliberately the whole room: the acceptance test for the generator
 * is a clay render, so nothing in here is allowed to flatter the
 * geometry. No bloom, no atmosphere, no colour, no darkness. If the
 * tree is beautiful naked it is beautiful anywhere.
 *
 * `setLightingCheck` is the one exception and it is explicit, opt-in
 * and off by default: a shadow-casting key and a fill, for checking
 * how form reads under a hard light. Every light beyond the sky lives
 * behind it. It is a check, never the mode the owner judges in.
 *
 * The scale figure is not decoration. Every Two Trees reference
 * establishes monumentality with something tiny at the base; without
 * one, a 24 m tree and a 4 m tree render identically.
 * ------------------------------------------------------------------ */

/** The subject stays a warm neutral clay; the room around it is cool.
 *  Hue does the separating so the tree reads as a silhouette against
 *  the background and off the ground, while the subject itself keeps a
 *  single flat value and still shows form honestly. Nothing here is
 *  allowed to colour the tree - the warm/cool split is between the
 *  subject and everything that is not the subject. */
const CLAY = 0x9d_96_8c;
/** The same subject hue taken darker, because a one-pixel line at the
 *  clay value disappears into the room. Structural line work is still
 *  the subject, so it stays on the warm side of the split. */
const CLAY_LINE = 0x6f_66_5c;
const CLAY_BACKGROUND = 0xc6_ce_d5;
const CLAY_GROUND = 0xa9_b1_b8;
const CLAY_FIGURE = 0x6b_67_63;

/** 1.8 m: radius 0.28 twice, plus a 1.24 m body. */
const FIGURE_RADIUS = 0.28;
const FIGURE_BODY = 1.24;
const FIGURE_HEIGHT = FIGURE_BODY + FIGURE_RADIUS * 2;

/** How much of the frame is air around the subject. Framing is the one
 *  composition decision the clay room makes, so it is a number with a
 *  reason rather than a camera distance someone once liked: a quarter
 *  again the tree's own extent leaves it clear of every edge while it
 *  is being orbited, and clear of the panel down the left. */
const FRAME_MARGIN = 1.3;

/** Where the camera stands, as a direction from the subject's centre:
 *  a three-quarter view from the front right, a little above the
 *  middle. The distance along it is solved from the subject; only the
 *  angle is authored. */
const FRAME_DIRECTION = new THREE.Vector3(0.62, 0.28, 1).normalize();

/** The ground disc's built radius, in metres. Everything else about
 *  the room is scaled off the subject at framing time, this included -
 *  it is the unit the scale is applied to, not a size in its own
 *  right. */
const GROUND_RADIUS = 400;

/** How far the room reaches, as a multiple of the subject's own
 *  longest side. The horizon has to be well outside the frame or the
 *  tree stands on a visible disc floating in the void; the far plane
 *  has to be outside the horizon or the disc is cut off instead; and
 *  the orbit has to reach past the framing distance, which is about
 *  twice the tree's height, or the owner cannot pull back far enough
 *  to see what he just grew. Six clears all three.
 *
 *  A multiple rather than a number of metres because the subject spans
 *  two orders of magnitude: a 4 m sapling and a 400 m Telperion need
 *  the same room in proportion and wildly different rooms in metres. */
const ROOM_REACH = 6;

/** The smallest subject the stage will measure, in metres. A build
 *  that yields nothing renderable measures an empty box, and a box
 *  that is a single point is not something a room can be fitted to or
 *  a camera can pivot about. One metre is a subject the owner can at
 *  least find while they work out why nothing was built. */
const MIN_SUBJECT = 1;

/** Where the lighting check's key light stands, as a direction. Its
 *  distance, like the room's, comes from the subject. */
const KEY_DIRECTION = new THREE.Vector3(24, 40, 18).normalize();

/** The clay, in the two forms a subject can take. A builder is handed
 *  these rather than choosing a material, because clay is the only
 *  mode the tree is ever judged in and a caller must not be able to
 *  opt out of it. Which form it uses is its own business; what colour
 *  clay is, is not. */
export interface Clay {
  /** For solid geometry: the surface the tree is judged on. */
  surface: THREE.Material;
  /** For line work: a skeleton, before there is a surface to judge. */
  line: THREE.Material;
  /** For foliage: the same clay, cut out rather than blended and drawn
   *  from both faces. A leaf is an open sheet with no back, and a
   *  canopy is the one thing in this room that needs a material the
   *  trunk's cannot be. Same colour, because the room is one clay. */
  element: THREE.Material;
}

/* ------------------------------------------------------------------ *
 * THE MEASUREMENT RIG
 *
 * What the room costs to draw, measured rather than guessed, and
 * measured honestly enough to be worth quoting. Three things make the
 * difference between a number and a story:
 *
 * 1. GPU TIME, NOT FRAME TIME. A frame time read off the main thread
 *    is pinned to the display's refresh: 16.7 ms on a monitor doing
 *    60 Hz, whatever the GPU actually did. `EXT_disjoint_timer_query_webgl2`
 *    times the draw on the GPU's own clock. Without the extension
 *    there is no number, and the panel says exactly that rather than
 *    quoting a vsync-pinned one.
 *
 * 2. VSYNC OFF, which no code here can assert. It is a browser and
 *    driver condition, so it is procedure:
 *      google-chrome --disable-gpu-vsync --disable-frame-rate-limit
 *    Even with GPU timing, a vsync-throttled renderer changes what the
 *    GPU is asked to do per second, so the sweep is run with it off.
 *
 * 3. FOUR POINTS, NOT TWO. Fill work scales with the AREA a triangle
 *    covers; the quad-overshading waste at its edges scales with its
 *    PERIMETER. Two resolutions cannot tell those apart - any two
 *    points lie on a line - so the sweep includes 1.0, 0.7, 0.5 and 0.25
 *    alongside the capped native ratio and an explicit 2.0 high-DPI point.
 *    The shape of the curve is the reading.
 *
 * And the term the harness was hiding: `setPixelRatio(min(dpr, 2))`.
 * On a HiDPI display that is four times the fragments of a ratio of 1,
 * which is larger than overdraw and larger than triangle size, and it
 * never appeared in the panel. Both ratios are reported now - what the
 * display asks for and what the renderer applied.
 * ------------------------------------------------------------------ */

/** Applied native resolution (capped as in the renderer), plus the 2.0
 * high-DPI and lower-resolution diagnostics. Duplicate ratios run once. */
export function sweepRatios(rawPixelRatio: number): readonly number[] {
  return [...new Set([Math.min(rawPixelRatio, PIXEL_RATIO_CAP), 2, 1, 0.7, 0.5, 0.25])]
    .sort((a, b) => b - a);
}

/** What the renderer is actually doing, which no build can know. */
export interface FrameStats {
  /** `renderer.info.render.calls` for the last frame: the whole scene,
   *  ground disc and scale figure included, so it reads above the
   *  subject's own count by the room's fixtures. */
  drawCalls: number;
  /** The renderer's own triangle count for the last frame. */
  triangles: number;
  /** `window.devicePixelRatio`, uncapped - what the display asks for. */
  pixelRatioRaw: number;
  /** What the renderer was told to use. Where the fill bill is paid. */
  pixelRatioApplied: number;
  /** Which way the flag under suspicion is set on this renderer. */
  logarithmicDepthBuffer: boolean;
  /** Whether a GPU timing number is reachable at all in this browser. */
  gpuTimer: boolean;
}

/** One resolution's result. `gpuMs` is a median over the samples that
 *  came back undisjoint, because a mean is one hitched frame away from
 *  being about the hitch. */
export interface SweepPoint {
  pixelRatio: number;
  gpuMs: number;
  samples: number;
  /** Undisjoint GPU query results in acquisition order, in milliseconds. */
  sampleMs?: readonly number[];
}

export interface SweepResult {
  /** Whether GPU timing was reachable. False means no number, ever -
   *  not a fallback to a frame time. */
  supported: boolean;
  /** Whether every requested ratio produced a point. */
  complete: boolean;
  points: readonly SweepPoint[];
  /** Why it is unsupported or incomplete. Empty on a clean run. */
  note: string;
}

/** The middle sample. Total, so an empty run reports zero rather than
 *  a NaN the panel would render as "NaN ms" - though the sweep never
 *  files a point it took no samples for. */
export function medianMs(samples: readonly number[]): number {
  if (samples.length === 0) return 0;
  const sorted = [...samples].sort((a, b) => a - b);
  const middle = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1
    ? sorted[middle]
    : (sorted[middle - 1] + sorted[middle]) / 2;
}

/** The sweep as the panel says it, and the one place the rig's honesty
 *  rule is enforced: WITHOUT THE EXTENSION THERE IS NO MILLISECOND
 *  FIGURE IN THIS STRING. A vsync-pinned fallback would read exactly
 *  like a measurement and mean nothing, so there is no fallback to
 *  write. A run cut short by a lost context or a resize keeps the
 *  points it got and is marked incomplete, because partial evidence
 *  labelled as partial is still evidence. Pure, and the rig's only
 *  testable surface: everything else it does needs a GPU. */
export function describeSweep(result: SweepResult): string {
  if (!result.supported) {
    return `no gpu timing available${result.note === "" ? "" : `: ${result.note}`} - no frame time reported, a vsync-pinned one would not be a measurement`;
  }
  const points = result.points
    .map(
      (point) =>
        `dpr ${point.pixelRatio.toFixed(2)} ${point.gpuMs.toFixed(2)} ms`,
    )
    .join(" · ");
  if (result.complete) return `gpu ${points} (vsync must be off)`;
  const partial = result.points.length === 0 ? "no points" : points;
  return `incomplete: ${partial}${result.note === "" ? "" : ` - ${result.note}`}`;
}

export interface StageOptions {
  /** The flag under suspicion. Set on the renderer at construction, so
   *  changing it means a new renderer on a new canvas - which is the
   *  caller's job, and why it arrives here as an option rather than as
   *  a setter. */
  logarithmicDepthBuffer?: boolean;
}

export interface Stage {
  /** Replaces the subject, disposing whatever stood there before. */
  setTree(build: (clay: Clay) => THREE.Object3D): void;
  /** Opt-in key and fill. Off - sky light alone - is the judging mode. */
  setLightingCheck(on: boolean): void;
  /** What the renderer did on the last frame, and at what resolution. */
  stats(): FrameStats;
  /** Pins the applied pixel ratio, or `null` for the harness default
   *  of `min(devicePixelRatio, 2)`. */
  setPixelRatio(ratio: number | null): void;
  /** Runs the resolution sweep, timing each point on the GPU's clock.
   *  Restores the pixel ratio it found, whether it finishes or not. */
  sweep(ratios?: readonly number[]): Promise<SweepResult>;
  /** Frames the subject currently on the stage, fitting the camera to
   *  what it actually measures. `height` places the scale figure, and
   *  stands in for the subject on the first frame, before there is
   *  one. */
  frame(height: number): void;
  /** Frames the subject, but only while this stage is still waiting to
   *  be framed - on its first tree, or on the next one after
   *  `frameNext`. Every other call is a no-op, because the camera
   *  moves when it is asked to and at no other time.
   *
   *  The latch is the stage's own and not the caller's, and that is
   *  the whole point of the method existing: see `waitingToFrame`. */
  frameIfWaiting(height: number): void;
  /** Asks for the NEXT subject to arrive to be framed. For a caller
   *  that has just requested a new tree and cannot frame it yet
   *  because it does not exist: the tree standing on the stage at that
   *  moment is the one being replaced. */
  frameNext(): void;
  dispose(): void;
}

/** The room's numbers, in metres: how far the ground reaches, how far
 *  the camera can see, and how far the orbit may travel either way. */
export interface Room {
  reach: number;
  far: number;
  minDistance: number;
  maxDistance: number;
}

/** Solves the room around a subject `span` metres across, for a camera
 *  standing `orbitRadius` from its target with its near plane at
 *  `near`.
 *
 *  Pure, and exported, because the room has one invariant worth
 *  asserting and the stage it belongs to needs a GPU to build: THE
 *  ROOM MAY NEVER MOVE THE CAMERA. `OrbitControls` clamps the orbit
 *  radius to `minDistance` and `maxDistance` on every `update()`, so a
 *  limit written on the wrong side of where the camera already stands
 *  is a camera move however silent the room claims to be - and the
 *  growing case cannot see it, because a room that only grows never
 *  crosses the camera. Grow to 400 m, pull back, then drag the height
 *  down: at 66 m the reach hits its 400 m floor and the camera is
 *  yanked from 2408 m to 400 m in one frame, on every intermediate
 *  value of the slider. So both limits open around where the camera is
 *  rather than closing on it. */
/** Where the orbit's pivot belongs once the subject has been replaced:
 *  exactly where it was while that is somewhere on the subject, and
 *  pulled back onto the nearest point of it when it is not.
 *
 *  Pure, and returns a new point rather than moving the one it was
 *  given, so the caller decides what to write and the camera's own
 *  position is never in reach of this at all. Panning across a crown
 *  survives it - the whole subject is inside its own bounds - and a
 *  subject that grows survives it for the same reason, since a box
 *  that has grown already contains the pivot. What does not survive is
 *  a pivot left 200 m up when the tree under it shrank to 24: orbit
 *  from there and the subject swings clean off the frame, which is a
 *  pivot pointing at a tree that is no longer there. */
export function pivotOn(box: THREE.Box3, pivot: THREE.Vector3): THREE.Vector3 {
  return box.clampPoint(pivot, new THREE.Vector3());
}

/** World-space bounds. Native adapter meshes carry bounds for immutable buffers;
 * arbitrary caller-owned instances are remeasured to repair stale cached bounds. */
export function measureSubject(object: THREE.Object3D): THREE.Box3 {
  object.traverse((node) => {
    if (node instanceof THREE.InstancedMesh && node.userData.nativeBounds !== true) node.computeBoundingBox();
  });
  return new THREE.Box3().setFromObject(object);
}

/** Releases everything a subject holds on the GPU.
 *
 *  Pure, and exported, for the same reason `pivotOn` is: it is the
 *  corrective half of a change and the stage it belongs to needs a GPU
 *  to build, so this is the only surface a test can hold to account.
 *
 *  Two kinds of buffer, and the second is the one that is easy to
 *  leak. `geometry.dispose()` releases the vertex buffers, and it
 *  catches lines as well as meshes because a skeleton is
 *  `LineSegments`. The instance transforms are NOT in the geometry -
 *  they hang off the mesh as its own attribute - and the renderer
 *  frees them only when the mesh dispatches its `dispose` event, which
 *  is `InstancedMesh.dispose()` and nothing else. A canopy regenerated
 *  on every slider drag would otherwise leave one Float32Array per
 *  build on the GPU, sized by the element count, which is the largest
 *  buffer in the room. */
export function disposeSubject(object: THREE.Object3D): void {
  object.traverse((node) => {
    // InstancedMesh extends Mesh, so both of these run for one: the
    // instance attribute first, then the geometry it shares.
    if (node instanceof THREE.InstancedMesh) node.dispose();
    if (node instanceof THREE.Mesh || node instanceof THREE.Line) {
      node.geometry.dispose();
    }
  });
}

export function solveRoom(
  span: number,
  orbitRadius: number,
  near: number,
): Room {
  const reach = Math.max(GROUND_RADIUS, span * ROOM_REACH);
  /* Half a percent of the subject, capped at a metre: close enough to
     put an eye on a twig. Floored at the near plane because nothing is
     visible inside it, which is also what stops a subject that
     measured nothing - a build with no renderable geometry - from
     collapsing the orbit radius onto its own target. */
  const close = Math.max(near, Math.min(1, span * 0.005));
  return {
    reach,
    /* The far plane has to clear the whole ground disc from wherever
       the camera stands: the disc reaches `reach` from the origin and
       the camera is `orbitRadius` out from a target inside it. Sized
       off the camera and not off the room alone, or a camera left far
       out by a shrinking subject sees the world end in front of it. */
    far: orbitRadius + reach * 2,
    minDistance: Math.min(close, Math.max(orbitRadius, near)),
    maxDistance: Math.max(reach, orbitRadius),
  };
}

/** The harness's own cap on the applied pixel ratio when nothing has
 *  pinned it. Kept because it is what the harness has always drawn at
 *  and the sweep is measured against that, not because 2 is a
 *  defensible number - it is the term the rig exists to expose. */
const PIXEL_RATIO_CAP = 2;

/** Frames drawn at a new resolution before any of them is timed: long
 *  enough for the driver to have finished reallocating the targets and
 *  for the pipeline to be full. */
const SWEEP_WARMUP_FRAMES = 8;
/** Timed frames per point. Twenty at 60 Hz is a third of a second per
 *  ratio, which the median has enough of to shrug off one hitch. */
const SWEEP_SAMPLE_FRAMES = 20;
/** How long one query is waited on before the sweep gives up on it. A
 *  timer query normally resolves within a frame or two of the draw. */
const SWEEP_QUERY_FRAMES = 120;

/** The extension's two constants. It is not in lib.dom, so its shape
 *  is stated here rather than cast away to `any`. */
interface DisjointTimerQuery {
  readonly TIME_ELAPSED_EXT: number;
  readonly GPU_DISJOINT_EXT: number;
}

export function createStage(
  canvas: HTMLCanvasElement,
  options: StageOptions = {},
): Stage {
  const logarithmicDepthBuffer = options.logarithmicDepthBuffer ?? true;

  /* The depth buffer is logarithmic, and it has to be: this room spans
     four orders of magnitude - a 1.8 m figure at the roots of a 400 m
     tree, with the orbit allowed within a metre of either.

     A perspective depth buffer quantizes at view distance z by
     dz = dd * z^2 * (f - n) / (f * n), and here f is thousands of
     times n, so the whole (f - n) / (f * n) term collapses to 1 / n.
     PRECISION IS GOVERNED BY THE NEAR PLANE. Moving `far` does nothing
     to it: halving the far plane from 4816 to 2408 leaves 0.16 m of
     quantization at the near face of a 400 m crown either way, against
     twigs one to two metres across, and limbs z-fight where they
     cross. Nor can `near` be scaled up with the subject, because
     `minDistance` is a metre at every size and the owner may
     legitimately dolly to a metre off the crown, where a small near
     plane is exactly what is wanted. A logarithmic buffer spends its
     precision per octave of distance rather than per metre, which is
     the shape this scene actually has. It costs a per-fragment depth
     write, and "which a dev harness with one tree in it can afford"
     was an assumption, not a measurement: writing depth from the
     fragment shader defeats early-Z, and the canopy about to land is
     fill-bound and alpha-tested, which is exactly the case that pays
     for it. So the flag is now an argument and both settings are
     measurable. It stays on by default until the sweep says
     otherwise - the z-fighting it fixes is real. */
  const renderer = new THREE.WebGLRenderer({
    canvas,
    antialias: true,
    logarithmicDepthBuffer,
  });

  /* What the display asks for, and what we give it. The two are not
     the same number and the gap is the largest single fill term in
     this scene, so both are reported and the applied one can be
     pinned - by the sweep, or by hand from the panel. */
  const rawPixelRatio = (): number => window.devicePixelRatio;
  let pixelRatioPin: number | null = null;
  const appliedPixelRatio = (): number =>
    pixelRatioPin ?? Math.min(rawPixelRatio(), PIXEL_RATIO_CAP);
  renderer.setPixelRatio(appliedPixelRatio());
  // No tone mapping and no exposure games: what the geometry does to
  // the light is the only thing on screen.
  renderer.toneMapping = THREE.NoToneMapping;
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;

  const scene = new THREE.Scene();
  scene.background = new THREE.Color(CLAY_BACKGROUND);

  // `near` is fixed and stays fixed at every subject size; the room is
  // what moves. See the renderer's note on why.
  const camera = new THREE.PerspectiveCamera(38, 1, 0.1, 4000);
  const controls = new OrbitControls(camera, canvas);
  controls.enableDamping = true;
  controls.maxPolarAngle = Math.PI * 0.495; // never below the ground
  controls.minDistance = 1;
  controls.maxDistance = 1200;

  /* The judging mode is one neutral sky light and nothing else. A
     hemisphere alone is the clay dome: form reads off the surface
     normal, and there is no key, no fill and no direction for weak
     geometry to hide behind. Everything else is an extra. */
  const sky = new THREE.HemisphereLight(0xff_ff_ff, 0x6a_69_66, 3.1);
  scene.add(sky);

  /* The extras, built once and only ever added to or removed from the
     scene, so toggling costs nothing. A shadow-casting key and a weak
     shadowless fill opposite it: the pair that shows how form takes a
     hard light. Neither is in the scene until asked for. */
  const key = new THREE.DirectionalLight(0xff_ff_ff, 2.2);
  key.castShadow = true;
  key.shadow.mapSize.set(2048, 2048);
  const shadowCamera = key.shadow.camera;
  shadowCamera.near = 1;

  const fill = new THREE.DirectionalLight(0xff_ff_ff, 0.9);
  fill.position.set(-30, 20, -14);

  const clay = new THREE.MeshStandardMaterial({
    color: CLAY,
    roughness: 0.92,
    metalness: 0,
  });
  const clayLine = new THREE.LineBasicMaterial({ color: CLAY_LINE });
  /* The canopy's clay. Three departures from the trunk's, and each one
     is the geometry's rather than a look:
       - DOUBLE SIDED, because a leaf is an open sheet. Backface
         culling would delete half of every element seen from behind.
       - ALPHA TEST, NEVER BLENDING. A cutout keeps the depth buffer
         honest and needs no sort; blending across tens of thousands of
         overlapping elements does, and gets it wrong. It costs early-Z
         on the target machine, which is exactly the cost the rig
         measures and the reason the log-depth question was parked for
         this task. The placeholder element carries no mask yet, so
         nothing is discarded today - the shader's discard is compiled
         in either way, which is what makes the measurement the real
         one, and the texturing spec drops a mask in without touching
         this line.
       - FLAT SHADED, because the blade's own vertex normals are
         smoothed around a fold and a leaf reads as a leaf by its
         silhouette, not by a gradient across three centimetres.
     The colour is CLAY, unchanged. Nothing here colours the foliage
     differently from the wood: the canopy is judged on placement. */
  const clayElement = new THREE.MeshStandardMaterial({
    color: CLAY,
    roughness: 0.92,
    metalness: 0,
    side: THREE.DoubleSide,
    flatShading: true,
    alphaTest: 0.5,
    transparent: false,
  });
  const groundMaterial = new THREE.MeshStandardMaterial({
    color: CLAY_GROUND,
    roughness: 1,
    metalness: 0,
  });
  const figureMaterial = new THREE.MeshStandardMaterial({
    color: CLAY_FIGURE,
    roughness: 0.95,
    metalness: 0,
  });

  const groundGeometry = new THREE.CircleGeometry(GROUND_RADIUS, 96);
  const ground = new THREE.Mesh(groundGeometry, groundMaterial);
  ground.rotation.x = -Math.PI / 2;
  ground.receiveShadow = true;
  scene.add(ground);

  const figureGeometry = new THREE.CapsuleGeometry(
    FIGURE_RADIUS,
    FIGURE_BODY,
    6,
    16,
  );
  const figure = new THREE.Mesh(figureGeometry, figureMaterial);
  figure.name = "scale-figure-1.8m";
  figure.castShadow = true;
  scene.add(figure);

  let tree: THREE.Object3D | null = null;
  let lightingCheck = false;
  /* Whether this stage is still waiting for its camera to be placed.
     It lives here, with the stage, because it describes THIS stage's
     lifetime and nothing else's. A latch held by the component that
     builds the stage outlives it: React's development double-mount -
     which is every load of the dev route, the only mode this route
     exists in - builds a stage, frames it, disposes it and builds a
     second one, and a component-scoped latch is already spent by the
     time the second one asks. The second stage's camera then sits
     unmoved at the origin, where `controls.update()` clamps the zero
     radius up to `minDistance` with phi at 0: twelve centimetres above
     the root, looking straight down at it, until someone clicks
     reframe. That was every cold load of the route. */
  let waitingToFrame = true;

  const disposeTree = (): void => {
    if (tree === null) return;
    scene.remove(tree);
    disposeSubject(tree);
    tree = null;
  };

  /** The subject's bounds, or a column of `fallbackHeight` when there
   *  is nothing on the stage yet. Never degenerate: a build that
   *  yields no renderable geometry measures an empty box, and a room
   *  solved from a zero-sized subject has a zero-sized everything. */
  const subjectBox = (fallbackHeight: number): THREE.Box3 => {
    const box = tree === null ? new THREE.Box3() : measureSubject(tree);
    if (box.isEmpty()) {
      const height = Math.max(fallbackHeight, MIN_SUBJECT);
      const half = height * 0.35;
      box.set(
        new THREE.Vector3(-half, 0, -half),
        new THREE.Vector3(half, height, half),
      );
    }
    return box;
  };

  /** Sizes the room to the subject: the ground, the far plane, how far
   *  the orbit may pull back, and where the lighting check's key
   *  stands.
   *
   *  It runs on every new tree and NOT only when the camera is placed,
   *  which is the whole point. The subject runs from a 4 m sapling to
   *  a tree of the Two Trees' order, hundreds of metres, and the room
   *  has to be able to hold whichever is on the stage right now: too
   *  small and the tree stands on a visible disc in the void, or the
   *  orbit stops before you are far enough out to see it at all; too
   *  large and the ground disc is a horizon that never arrives. Tying
   *  it to the camera's last framing instead meant a tree grown to
   *  300 m was still living in the room its 24 m predecessor was
   *  framed in, and could not be zoomed out to. (What it is NOT about
   *  is depth precision - that is the near plane's, and the renderer's
   *  note says why.)
   *
   *  Silent by design, in both directions: it never moves the camera,
   *  and `solveRoom` is where that is held to account. */
  const fitRoom = (box: THREE.Box3): void => {
    const size = box.getSize(new THREE.Vector3());
    const span = Math.max(size.x, size.y, size.z);

    /* The pivot is put back on the subject before anything is measured
       from it, because the orbit radius is measured from it. Not a
       camera move: `camera.position` is untouched, and what the pivot
       does to the radius is then handed straight to `solveRoom`, which
       opens the orbit's limits around wherever that leaves it. */
    controls.target.copy(pivotOn(box, controls.target));

    const room = solveRoom(
      span,
      camera.position.distanceTo(controls.target),
      camera.near,
    );
    ground.scale.setScalar(room.reach / GROUND_RADIUS);
    camera.far = room.far;
    camera.updateProjectionMatrix();
    controls.maxDistance = room.maxDistance;
    controls.minDistance = room.minDistance;

    /* The shadow camera is an orthographic box and it has to contain
       the subject, or the lighting check drops every shadow outside
       it. */
    const extent = span * 0.75 + 1;
    key.position.copy(KEY_DIRECTION).multiplyScalar(extent * 3);
    shadowCamera.left = -extent * 2;
    shadowCamera.right = extent * 2;
    shadowCamera.top = extent * 2;
    shadowCamera.bottom = -extent * 2;
    shadowCamera.far = extent * 8;
    shadowCamera.updateProjectionMatrix();
  };

  const setTree: Stage["setTree"] = (build) => {
    const replacement = build({ surface: clay, line: clayLine, element: clayElement });
    replacement.traverse((node) => {
      if (node instanceof THREE.Mesh) {
        node.castShadow = true;
        node.receiveShadow = true;
      }
    });
    disposeTree();
    tree = replacement;
    scene.add(tree);
    // The room fits the tree that is there now, whether or not anyone
    // asks for the camera to be moved.
    fitRoom(subjectBox(0));
  };

  const frame: Stage["frame"] = (height) => {
    waitingToFrame = false;
    // Stand the figure clear of the root flare.
    figure.position.set(height * 0.16 + 1.2, FIGURE_HEIGHT / 2, height * 0.2);

    /* Frame the subject that is actually there, not a column of the
       height the dial says. The two differ by more than a nicety: the
       crown's mass sits well above the middle of the envelope, its
       spread runs to one and a third of its height, and both move
       under the dials - so a camera placed at fixed multiples of
       `height` clips the crown at one setting and leaves the tree a
       speck at another. */
    const detail = tree?.userData.specimenView === "foliage-detail";
    figure.visible = !detail;
    controls.maxPolarAngle = detail ? Math.PI : Math.PI * 0.495;
    camera.near = detail ? 0.0001 : 0.1;
    const box = subjectBox(height);
    const size = box.getSize(new THREE.Vector3());
    const centre = box.getCenter(new THREE.Vector3());

    /* Far enough back that the subject fits both ways. The vertical
       half-angle is the camera's own; the horizontal one is that times
       the aspect, so a wide window pulls in and a tall one pulls
       back. */
    const halfAngle = THREE.MathUtils.degToRad(camera.fov) / 2;
    const across = Math.max(size.x, size.z) / 2;
    const distance =
      Math.max(
        size.y / 2 / Math.tan(halfAngle),
        across / (Math.tan(halfAngle) * Math.max(0.1, camera.aspect)),
      ) * FRAME_MARGIN;

    controls.target.copy(centre);
    camera.position
      .copy(centre)
      .addScaledVector(detail ? tree!.userData.detailDirection ?? FRAME_DIRECTION : FRAME_DIRECTION,
        Math.max(distance + size.length() / 2, detail ? 0.001 : 1));
    // Never underground, however low the subject's centre sits.
    if (!detail) camera.position.y = Math.max(camera.position.y, FIGURE_HEIGHT);

    // On the first frame the room has not seen a tree yet, so it is
    // sized from the same fallback the camera just used.
    fitRoom(box);
    controls.update();
  };

  const frameIfWaiting: Stage["frameIfWaiting"] = (height) => {
    if (!waitingToFrame) return;
    frame(height);
  };

  const frameNext: Stage["frameNext"] = () => {
    waitingToFrame = true;
  };

  const setLightingCheck: Stage["setLightingCheck"] = (on) => {
    if (on === lightingCheck) return;
    lightingCheck = on;
    renderer.shadowMap.enabled = on;
    if (on) scene.add(key, fill);
    else scene.remove(key, fill);
    // A material compiled without shadows has to be recompiled with
    // them; three only notices when it is told.
    for (const material of [
      clay,
      clayElement,
      groundMaterial,
      figureMaterial,
    ]) {
      material.needsUpdate = true;
    }
  };

  /* Two things a measurement cannot survive, both of them outside the
     rig's control: the canvas changing size under it - which changes
     the fragment count, so the points either side of it are not
     comparable - and the context going away, which invalidates every
     query in flight. Neither is an error. Both end the sweep, and what
     it had is reported as partial. */
  let resizeEpoch = 0;
  let contextLost = false;
  const onContextLost = (): void => {
    contextLost = true;
  };
  canvas.addEventListener("webglcontextlost", onContextLost);

  let viewWidth = 0;
  let viewHeight = 0;
  const resize = (): void => {
    const width = canvas.clientWidth || 1;
    const height = canvas.clientHeight || 1;
    // Only a real change counts as one: the observer fires once on
    // being attached, and a sweep that read that as a resize would
    // abort itself on its own first frame.
    if (width === viewWidth && height === viewHeight) return;
    viewWidth = width;
    viewHeight = height;
    resizeEpoch += 1;
    renderer.setSize(width, height, false);
    camera.aspect = width / height;
    camera.updateProjectionMatrix();
  };
  resize();

  const observer = new ResizeObserver(resize);
  observer.observe(canvas);

  /* While the sweep is running it owns the renders, because each timed
     one is bracketed by a query and an untimed one in between would be
     work the query never sees. The controls keep updating either way -
     damping that stops for the length of a sweep reads as a stutter. */
  let sweeping = false;
  let frameHandle = 0;
  const tick = (): void => {
    frameHandle = requestAnimationFrame(tick);
    controls.update();
    if (!sweeping) renderer.render(scene, camera);
  };
  tick();

  const gl = renderer.getContext();
  /** The extension, or null. Asked for once: `getExtension` is cheap
   *  but the answer cannot change for the life of a context. */
  const timer: DisjointTimerQuery | null =
    typeof (gl as WebGL2RenderingContext).createQuery === "function"
      ? ((gl.getExtension(
          "EXT_disjoint_timer_query_webgl2",
        ) as DisjointTimerQuery | null) ?? null)
      : null;

  const stats: Stage["stats"] = () => ({
    drawCalls: renderer.info.render.calls,
    triangles: renderer.info.render.triangles,
    pixelRatioRaw: rawPixelRatio(),
    pixelRatioApplied: renderer.getPixelRatio(),
    logarithmicDepthBuffer,
    gpuTimer: timer !== null,
  });

  const setPixelRatio: Stage["setPixelRatio"] = (ratio) => {
    pixelRatioPin = ratio;
    // three's setPixelRatio re-applies the current size itself, so
    // this is the whole of applying it.
    renderer.setPixelRatio(appliedPixelRatio());
  };

  const nextFrame = (): Promise<void> =>
    new Promise((resolve) => {
      requestAnimationFrame(() => resolve());
    });

  /** Why the sweep must stop, or empty while it may go on. */
  const interruption = (epoch: number): string => {
    if (contextLost) return "webgl context lost";
    if (resizeEpoch !== epoch) return "canvas resized mid-sweep";
    return "";
  };

  /** One timed frame, in milliseconds, or null when the sample is not
   *  usable - a disjoint (the GPU was preempted, so the number is
   *  meaningless and the spec says to throw it away) or a query that
   *  never resolved. A dropped sample is not an interruption; the
   *  point simply rests on the ones that did come back. */
  const sampleFrame = async (
    gl2: WebGL2RenderingContext,
    ext: DisjointTimerQuery,
  ): Promise<number | null> => {
    const query = gl2.createQuery();
    if (query === null) return null;
    gl2.beginQuery(ext.TIME_ELAPSED_EXT, query);
    renderer.render(scene, camera);
    gl2.endQuery(ext.TIME_ELAPSED_EXT);

    for (let waited = 0; waited < SWEEP_QUERY_FRAMES; waited += 1) {
      await nextFrame();
      if (gl2.getParameter(ext.GPU_DISJOINT_EXT) === true) {
        gl2.deleteQuery(query);
        return null;
      }
      if (
        gl2.getQueryParameter(query, gl2.QUERY_RESULT_AVAILABLE) === true
      ) {
        const nanoseconds = gl2.getQueryParameter(
          query,
          gl2.QUERY_RESULT,
        ) as number;
        gl2.deleteQuery(query);
        return nanoseconds / 1e6;
      }
    }
    gl2.deleteQuery(query);
    return null;
  };

  const sweep: Stage["sweep"] = async (ratios = sweepRatios(rawPixelRatio())) => {
    if (timer === null) {
      return {
        supported: false,
        complete: false,
        points: [],
        note: "EXT_disjoint_timer_query_webgl2 is not exposed by this browser",
      };
    }
    if (sweeping) {
      return {
        supported: true,
        complete: false,
        points: [],
        note: "a sweep is already running",
      };
    }

    const gl2 = gl as WebGL2RenderingContext;
    const epoch = resizeEpoch;
    const restore = pixelRatioPin;
    const points: SweepPoint[] = [];
    let note = "";

    sweeping = true;
    try {
      for (const ratio of ratios) {
        setPixelRatio(ratio);
        for (let i = 0; i < SWEEP_WARMUP_FRAMES; i += 1) {
          await nextFrame();
          renderer.render(scene, camera);
        }

        const samples: number[] = [];
        for (let i = 0; i < SWEEP_SAMPLE_FRAMES; i += 1) {
          note = interruption(epoch);
          if (note !== "") break;
          const elapsed = await sampleFrame(gl2, timer);
          if (elapsed !== null) samples.push(elapsed);
        }

        if (note !== "") break;
        if (samples.length === 0) {
          note = `no usable sample at dpr ${ratio.toFixed(2)}`;
          break;
        }
        points.push({
          pixelRatio: ratio,
          gpuMs: medianMs(samples),
          samples: samples.length,
          sampleMs: samples,
        });
      }
    } finally {
      sweeping = false;
      setPixelRatio(restore);
    }

    return {
      supported: true,
      complete: note === "" && points.length === ratios.length,
      points,
      note,
    };
  };

  return {
    setTree,
    setLightingCheck,
    stats,
    setPixelRatio,
    sweep,
    frame,
    frameIfWaiting,
    frameNext,
    dispose(): void {
      cancelAnimationFrame(frameHandle);
      observer.disconnect();
      canvas.removeEventListener("webglcontextlost", onContextLost);
      controls.dispose();
      disposeTree();
      groundGeometry.dispose();
      figureGeometry.dispose();
      clay.dispose();
      clayLine.dispose();
      clayElement.dispose();
      groundMaterial.dispose();
      figureMaterial.dispose();
      renderer.dispose();
    },
  };
}
