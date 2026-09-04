import { readdirSync, readFileSync } from "node:fs";

import * as THREE from "three";
import { describe, expect, it } from "vitest";

import {
  describeSweep,
  disposeSubject,
  measureSubject,
  medianMs,
  pivotOn,
  solveRoom,
  SWEEP_RATIOS,
  type SweepResult,
} from "./stage";

/* ------------------------------------------------------------------ *
 * THE ROOM, AND THE LATCH THAT PLACES THE CAMERA IN IT
 *
 * The stage itself needs a GPU and a canvas, and this box has neither,
 * so what is testable is the part that is arithmetic: the room's own
 * numbers and where the orbit pivots. That is not a consolation prize
 * - it is where the invariant lives. The room may never move the
 * camera, and `OrbitControls` clamps the orbit radius to `minDistance`
 * and `maxDistance` on every `update()`, so writing either on the
 * wrong side of where the camera stands moves it.
 *
 * The bug this closes was invisible from the growing direction, which
 * is the only direction anyone had tried: a room that only ever grows
 * never crosses the camera. So every assertion below that matters runs
 * the subject DOWN.
 * ------------------------------------------------------------------ */

const NEAR = 0.1;

/** The bounds a tree measures at a given point on the height dial,
 *  taken in proportion from the 400 m crown that was measured on the
 *  real generator: 228.9 x 401.4 x 221.3. The span runs a little over
 *  the dial's number because what is fitted is the crown, not the
 *  dial. */
function treeBox(height: number): THREE.Box3 {
  const scale = height / 400;
  return new THREE.Box3(
    new THREE.Vector3(-114.45 * scale, 0, -110.65 * scale),
    new THREE.Vector3(114.45 * scale, 401.4 * scale, 110.65 * scale),
  );
}

function spanOf(box: THREE.Box3): number {
  const size = box.getSize(new THREE.Vector3());
  return Math.max(size.x, size.y, size.z);
}

/** The height dial, walked from the top down - which is the direction
 *  the defect lived in. */
const HEIGHTS = [400, 150, 66, 60, 24, 4] as const;

const SPANS = HEIGHTS.map((height) => spanOf(treeBox(height)));

describe("solveRoom", () => {
  it("never pulls the camera in when the subject shrinks", () => {
    /* The reported failure, replayed: grow to 400 m, scroll out to
       2408 m, then drag the height down. At 66 m the reach hits its
       400 m floor, and a `maxDistance` of 400 written under a camera
       2408 m out is a 2000 m jump on the very next frame - once for
       every intermediate value the slider passes through. */
    const radius = 2408;
    for (const span of SPANS) {
      const room = solveRoom(span, radius, NEAR);
      expect(room.maxDistance, `${span} m subject`).toBeGreaterThanOrEqual(
        radius,
      );
      expect(room.minDistance, `${span} m subject`).toBeLessThanOrEqual(radius);
    }
  });

  it("never pushes the camera out when it is right in on the subject", () => {
    // The other side of the same rule: an eye on a twig, a metre out,
    // and then the subject grows under it.
    const radius = 0.4;
    for (const span of [...SPANS].reverse()) {
      const room = solveRoom(span, radius, NEAR);
      expect(room.minDistance, `${span} m subject`).toBeLessThanOrEqual(radius);
      expect(room.maxDistance, `${span} m subject`).toBeGreaterThanOrEqual(
        radius,
      );
    }
  });

  it("sees the whole ground disc from wherever the camera stands", () => {
    // The disc reaches `reach` from the origin and the camera is
    // `radius` out from a target inside it, so the far plane has to
    // clear their sum or the world ends in front of the camera - which
    // is what a far plane sized off the room alone does to a camera
    // the shrinking room has left far outside it.
    for (const span of SPANS) {
      for (const radius of [0.4, 60, 2408]) {
        const room = solveRoom(span, radius, NEAR);
        expect(
          room.far,
          `${span} m subject, camera ${radius} m out`,
        ).toBeGreaterThanOrEqual(radius + room.reach);
      }
    }
  });

  it("grows the room to hold a bigger subject", () => {
    // The reason the room is refitted at all: a tree grown to 400 m
    // must not still be living in the room its 24 m predecessor was
    // framed in, unable to be zoomed out to.
    const small = solveRoom(33, 60, NEAR);
    const large = solveRoom(401, 60, NEAR);
    expect(large.reach).toBeGreaterThan(small.reach * 5);
    expect(large.maxDistance).toBeGreaterThan(401);
    expect(large.far).toBeGreaterThan(large.reach);
  });

  it("holds a subject that measured nothing at all", () => {
    /* A build that yields no renderable geometry measures a zero span,
       and a room solved from it used to set `minDistance` to zero -
       which lets the orbit radius collapse onto its own target, with
       no scroll direction that gets back out of it. */
    const room = solveRoom(0, 12, NEAR);
    expect(room.minDistance).toBeGreaterThanOrEqual(NEAR);
    expect(room.maxDistance).toBeGreaterThan(0);
    expect(room.far).toBeGreaterThan(0);
  });

  it("never lets the orbit inside the near plane", () => {
    // Nothing is visible closer than the near plane, so an orbit floor
    // below it only buys a way to lose the subject.
    for (const span of SPANS) {
      expect(solveRoom(span, 1000, NEAR).minDistance).toBeGreaterThanOrEqual(
        NEAR,
      );
    }
  });
});

describe("pivotOn", () => {
  it("pulls the pivot onto a subject that shrank away from it", () => {
    /* The other half of the same shrink: the camera stops being yanked
       and the pivot is still hanging where the 400 m crown's centre
       used to be, 200 m above a 24 m tree. Orbit from there and the
       subject swings clean off the frame - a pivot pointing at a tree
       that is no longer there. */
    const stale = new THREE.Vector3(0, 200, 0);
    for (const height of HEIGHTS) {
      const box = treeBox(height);
      const pivot = pivotOn(box, stale);
      expect(box.containsPoint(pivot), `${height} m subject`).toBe(true);
    }
    // Hands back a new point rather than moving the one it was given:
    // what gets written is the caller's decision, which is what keeps
    // this out of reach of anything but the pivot.
    expect(stale.y).toBe(200);
  });

  it("leaves a pivot that is already on the subject exactly alone", () => {
    // Panning across the crown to put an eye on one limb is the owner
    // moving the camera, and the room does not get a vote on it.
    const box = treeBox(24);
    for (const pivot of [
      new THREE.Vector3(0, 12, 0),
      new THREE.Vector3(6.8, 1, -6.6),
      box.max.clone(), // the very top of the crown still counts as on it
    ]) {
      expect(pivotOn(box, pivot).equals(pivot)).toBe(true);
    }
    // And a subject that grew already contains it.
    expect(pivotOn(treeBox(400), new THREE.Vector3(0, 12, 0)).y).toBe(12);
  });

  it("moves the pivot without ever moving the camera", () => {
    /* The whole shrink, run the way `fitRoom` runs it: correct the
       pivot, measure the orbit radius from where that leaves it, and
       solve the room from that. The camera stands still throughout -
       it is not an input to either half - and the room that comes back
       still has to hold it where it is. */
    const camera = new THREE.Vector3(1493, 604, 2408);
    const before = camera.clone();
    let pivot = new THREE.Vector3(0, 200, 0);

    for (const height of HEIGHTS) {
      const box = treeBox(height);
      pivot = pivotOn(box, pivot);
      const radius = camera.distanceTo(pivot);
      const room = solveRoom(spanOf(box), radius, NEAR);

      expect(box.containsPoint(pivot), `${height} m subject`).toBe(true);
      expect(room.minDistance, `${height} m subject`).toBeLessThanOrEqual(
        radius,
      );
      expect(room.maxDistance, `${height} m subject`).toBeGreaterThanOrEqual(
        radius,
      );
      expect(room.far, `${height} m subject`).toBeGreaterThanOrEqual(
        radius + room.reach,
      );
    }
    expect(camera.equals(before)).toBe(true);
  });
});

/* ------------------------------------------------------------------ *
 * THE SUBJECT, WHEN PART OF IT IS INSTANCED
 *
 * A crown of thousands of leaves arrives as one mesh carrying one
 * transform per element, and an InstancedMesh answers both of the
 * questions the stage asks of a subject differently from a plain one.
 * How big is it: from its own cached bounding box, computed once from
 * the instance transforms and never refreshed. What does it hold: an
 * attribute buffer that is not in its geometry and is not freed with
 * it. Both are the ways R8 and the regenerate path can fail silently,
 * so both are asserted here on the pure seams.
 * ------------------------------------------------------------------ */

/** A crown, near enough: one small element, twelve copies of it
 *  scattered through a 20 m box. Matrices written the way the harness
 *  writes them, so the mesh is in the state the stage will meet it
 *  in. */
function instancedCrown(count = 12, reach = 10): THREE.InstancedMesh {
  const geometry = new THREE.BoxGeometry(0.12, 0.12, 0.12);
  const mesh = new THREE.InstancedMesh(
    geometry,
    new THREE.MeshBasicMaterial(),
    count,
  );
  const matrix = new THREE.Matrix4();
  for (let i = 0; i < count; i += 1) {
    const angle = (i / count) * Math.PI * 2;
    matrix.makeTranslation(
      Math.cos(angle) * reach,
      reach + Math.sin(angle) * reach * 0.5,
      Math.sin(angle) * reach,
    );
    mesh.setMatrixAt(i, matrix);
  }
  mesh.instanceMatrix.needsUpdate = true;
  return mesh;
}

describe("measureSubject", () => {
  it("measures a crown whose bounds were never computed", () => {
    const crown = instancedCrown();
    expect(crown.boundingBox).toBeNull();
    const box = measureSubject(crown);
    expect(box.isEmpty()).toBe(false);
    expect(spanOf(box)).toBeGreaterThan(15);
  });

  it("measures a crown whose bounds were computed too early", () => {
    /* The failure R8 names, and it is not hypothetical: three computes
       an InstancedMesh's bounding box once and caches it, so a mesh
       measured before its transforms were written keeps a box the size
       of one element at the origin for the rest of its life. Against a
       tree that is metres across, that is zero extent - and the room,
       the pivot and the near and far planes would all be solved for
       the branches alone.

       So the stale box is measured here as well, to show what the
       guard is for: `setFromObject` on its own reports the element and
       `measureSubject` reports the crown. */
    const crown = new THREE.InstancedMesh(
      new THREE.BoxGeometry(0.12, 0.12, 0.12),
      new THREE.MeshBasicMaterial(),
      12,
    );
    crown.computeBoundingBox();
    const stale = crown.boundingBox!.clone();

    const placed = instancedCrown();
    crown.instanceMatrix.copy(placed.instanceMatrix);
    crown.instanceMatrix.needsUpdate = true;

    expect(spanOf(stale)).toBeLessThan(1);
    expect(spanOf(new THREE.Box3().setFromObject(crown))).toBeLessThan(1);
    expect(spanOf(measureSubject(crown))).toBeGreaterThan(15);
  });

  it("leaves a subject with no crown measuring exactly what it did", () => {
    /* R8's error case. A tree that grew no foliage carries no
       instanced mesh at all, and the guard must not move the branch-
       only framing by so much as a float - the bounds are the same box
       `setFromObject` has always returned. */
    const trunk = new THREE.Mesh(
      new THREE.CylinderGeometry(0.4, 0.6, 18, 8),
      new THREE.MeshBasicMaterial(),
    );
    trunk.position.y = 9;
    const subject = new THREE.Group();
    subject.add(trunk);

    const plain = new THREE.Box3().setFromObject(subject);
    const measured = measureSubject(subject);
    expect(measured.min.toArray()).toEqual(plain.min.toArray());
    expect(measured.max.toArray()).toEqual(plain.max.toArray());
  });

  it("takes the crown in with the branches it hangs on", () => {
    /* The whole subject, not the largest part of it. The crown here
       reaches wider than the wood and a little above it, which is what
       a canopy does to a tree - and both are what the room, the pivot
       and the far plane are then solved from. */
    const trunk = new THREE.Mesh(
      new THREE.CylinderGeometry(0.4, 0.6, 12, 8),
      new THREE.MeshBasicMaterial(),
    );
    trunk.position.y = 6;
    const subject = new THREE.Group();
    subject.add(trunk, instancedCrown());

    const box = measureSubject(subject);
    expect(box.min.x).toBeLessThan(-9);
    expect(box.max.x).toBeGreaterThan(9);
    expect(box.max.y).toBeGreaterThan(12);
  });
});

describe("disposeSubject", () => {
  it("releases the instance transforms, not only the geometry", () => {
    /* The instance transforms hang off the mesh rather than off its
       geometry, and the renderer frees them when the mesh dispatches
       its own `dispose` - which `geometry.dispose()` does not do. A
       sweep that only disposed geometries would leave one buffer per
       regenerate on the GPU, sized by the element count, which on a
       canopy is the largest buffer in the room. */
    const crown = instancedCrown();
    let released = false;
    let geometryReleased = false;
    crown.addEventListener("dispose", () => {
      released = true;
    });
    crown.geometry.addEventListener("dispose", () => {
      geometryReleased = true;
    });

    disposeSubject(crown);
    expect(released).toBe(true);
    expect(geometryReleased).toBe(true);
  });

  it("still releases the line work and the plain meshes under it", () => {
    // The skeleton is LineSegments and a Mesh-only sweep would leak
    // one geometry per regenerate. Unchanged by the canopy landing.
    const subject = new THREE.Group();
    const trunk = new THREE.Mesh(
      new THREE.BoxGeometry(),
      new THREE.MeshBasicMaterial(),
    );
    const lines = new THREE.LineSegments(
      new THREE.BufferGeometry(),
      new THREE.LineBasicMaterial(),
    );
    subject.add(trunk, lines);

    const released: string[] = [];
    trunk.geometry.addEventListener("dispose", () => released.push("trunk"));
    lines.geometry.addEventListener("dispose", () => released.push("lines"));

    disposeSubject(subject);
    expect(released.sort()).toEqual(["lines", "trunk"]);
  });
});

/* ------------------------------------------------------------------ *
 * THE CLAY ROOM'S DEFAULTS ARE THE ACCEPTANCE CRITERION
 *
 * R6 enumerates what the judging mode contains: one neutral sky light,
 * flat grey, no colour and no post, with every extra behind an opt-in
 * toggle that is off - and nothing in the library emitting a material,
 * a colour or a light at all. An enumerated default state is a test
 * and not a preference, which is exactly how a "harmless" second light
 * got into this file once already: a comment argued for it instead of
 * removing it.
 *
 * A canopy is the strongest pull yet on that line - foliage wants to
 * be green, and a leaf material is one word away from being one - so
 * the enumeration is written down here rather than trusted.
 * ------------------------------------------------------------------ */
describe("the clay room is the only judging mode", () => {
  const source = (file: string): string =>
    readFileSync(new URL(file, import.meta.url), "utf8");

  it("the library emits no material, no colour and no light", () => {
    /* The generator is a standalone library and the consumer brings
       the lights. It reaches for three's maths - vectors and matrices
       - and for nothing that decides how anything looks. */
    const root = new URL("../src/", import.meta.url);
    const files = readdirSync(root, {
      recursive: true,
      encoding: "utf8",
    }).filter((name) => name.endsWith(".ts") && !name.endsWith(".test.ts"));
    expect(files.length).toBeGreaterThan(0);

    const banned =
      /\b(?:new\s+THREE\.\w*(?:Material|Light|Texture)|THREE\.Color|THREE\.Fog|toneMapping|castShadow|receiveShadow)\b/;
    for (const name of files) {
      const text = readFileSync(new URL(name, root), "utf8");
      expect(
        banned.test(text),
        `src/${name} decides how something looks; the consumer owns that`,
      ).toBe(false);
    }
  });

  it("puts one light in the scene and the rest behind the toggle", () => {
    const stage = source("./stage.ts");
    // The sky is the judging mode's whole lighting rig.
    expect(stage).toContain("scene.add(sky)");
    // And the key and the fill reach the scene from one place only,
    // which is the toggle.
    expect(stage.split("scene.add(key, fill)").length - 1).toBe(1);
    expect(stage.indexOf("scene.add(key, fill)")).toBeGreaterThan(
      stage.indexOf("const setLightingCheck"),
    );
    // Nothing else is a light at all.
    expect(stage).not.toMatch(
      /new THREE\.(?:Ambient|Point|Spot|RectArea)Light/,
    );
    // No exposure games and no fog: what the geometry does to the
    // light is the only thing on screen.
    expect(stage).toContain("THREE.NoToneMapping");
    expect(stage).not.toContain("THREE.Fog");
  });

  it("cuts the foliage out rather than blending it", () => {
    /* R4. Blending across tens of thousands of overlapping elements
       needs a sort the canopy cannot afford and would get wrong; a
       cutout keeps the depth buffer honest. The cost is early-Z on the
       target machine, which is measured rather than assumed. */
    const stage = source("./stage.ts");
    expect(stage).toContain("alphaTest");
    expect(stage).not.toContain("transparent: true");
    expect(stage).not.toContain("blending:");
  });
});

/* The framing latch cannot be exercised from here: it belongs to a
   stage, a stage needs a WebGL context, and the double-mount that
   spends it needs React and a DOM. Both are out of this runner's reach
   by design - "pure units only", vitest.config.ts. So what is asserted
   is the structure that makes the bug unreachable, which is the thing
   that actually went wrong: the latch was a component-lifetime ref
   while the stage was not, so React's development double-mount - every
   load of the dev route, since the route exists in no other mode -
   framed stage A, disposed it, built stage B and found the latch
   already spent. B's camera never moved, and `controls.update()`
   clamped its zero radius up to `minDistance` with phi at 0: twelve
   centimetres above the root, looking straight down.

   Tie the latch to the stage and the failure has nowhere to live. */
describe("the framing latch belongs to the stage", () => {
  const source = (file: string): string =>
    readFileSync(new URL(file, import.meta.url), "utf8");

  it("is declared inside createStage, so each stage has its own", () => {
    const stage = source("./stage.ts");
    const declaration = stage.indexOf("let waitingToFrame");
    const factory = stage.indexOf("export function createStage");
    expect(declaration, "stage.ts declares no framing latch").toBeGreaterThan(
      -1,
    );
    expect(
      declaration,
      "the framing latch is module-scoped: every stage in the process would share one",
    ).toBeGreaterThan(factory);
    expect(stage.split("let waitingToFrame").length - 1).toBe(1);
  });

  it("is never held by the component that builds the stage", () => {
    const component = source("./GrowerDev.tsx");
    expect(
      component,
      "the component asks the stage to frame; it does not decide whether to",
    ).toContain("frameIfWaiting");
    expect(
      component,
      "a boolean ref in the component outlives the stage it was latched for",
    ).not.toMatch(/useRef(<boolean.*?>)?\((false|true)\)/);
  });
});

/* ------------------------------------------------------------------ *
 * THE MEASUREMENT RIG, AT THE ONE POINT IT IS TESTABLE
 *
 * The rig needs a GPU, a canvas and a timer-query extension, and this
 * box has none of the three - so what can be held to account is what
 * the panel is allowed to SAY. That is not a consolation prize either:
 * the failure this rig exists to prevent is a number that reads like a
 * measurement and is not one. A frame time pinned to a 60 Hz refresh
 * is 16.7 ms whatever the GPU did, and quoting it as the budget would
 * be worse than quoting nothing, because nothing is visibly nothing.
 *
 * So: without the extension there is no millisecond figure anywhere in
 * the string, and a run cut short says so and keeps what it got.
 * ------------------------------------------------------------------ */

/** Any millisecond figure at all. The unsupported case must not
 *  produce one, and this is what "must not" means. */
const A_TIMING_NUMBER = /\d+(\.\d+)?\s*ms/;

function result(over: Partial<SweepResult> = {}): SweepResult {
  return { supported: true, complete: true, points: [], note: "", ...over };
}

const FOUR_POINTS = [
  { pixelRatio: 1, gpuMs: 8.42, samples: 20 },
  { pixelRatio: 0.7, gpuMs: 4.31, samples: 20 },
  { pixelRatio: 0.5, gpuMs: 2.28, samples: 20 },
  { pixelRatio: 0.25, gpuMs: 0.71, samples: 20 },
];

describe("the sweep's shape", () => {
  it("runs four points, and the four the budget is read from", () => {
    /* Two points cannot separate the two things that scale here:
       covered-area work goes with a triangle's area and quad-overshading
       waste goes with its perimeter, and any two points lie on a line.
       Four is the smallest set with a shape to read. */
    expect([...SWEEP_RATIOS]).toEqual([1, 0.7, 0.5, 0.25]);
  });
});

describe("describeSweep", () => {
  it("reports no timing number at all when there is no gpu timer", () => {
    // The R5 error case. A vsync-pinned fallback would read exactly
    // like a measurement, so there is no fallback to write.
    const text = describeSweep(
      result({
        supported: false,
        complete: false,
        note: "EXT_disjoint_timer_query_webgl2 is not exposed by this browser",
      }),
    );
    expect(text).toContain("no gpu timing available");
    expect(text).not.toMatch(A_TIMING_NUMBER);
  });

  it("marks a run cut short as incomplete and keeps what it got", () => {
    // The other R5 error case: a context loss or a resize mid-sweep.
    // The points either side of a resize are not comparable, so the
    // run stops - but two honest points labelled as two are still
    // evidence, and throwing them away would not be more honest.
    const text = describeSweep(
      result({
        complete: false,
        points: FOUR_POINTS.slice(0, 2),
        note: "canvas resized mid-sweep",
      }),
    );
    expect(text).toContain("incomplete");
    expect(text).toContain("canvas resized mid-sweep");
    expect(text).toContain("dpr 1.00 8.42 ms");
    expect(text).toContain("dpr 0.70 4.31 ms");
    expect(text).not.toContain("dpr 0.25");
  });

  it("says so when it was cut short before any point landed", () => {
    const text = describeSweep(
      result({ complete: false, note: "webgl context lost" }),
    );
    expect(text).toContain("incomplete");
    expect(text).toContain("no points");
    expect(text).not.toMatch(A_TIMING_NUMBER);
  });

  it("reads out every point of a clean run", () => {
    const text = describeSweep(result({ points: FOUR_POINTS }));
    expect(text).not.toContain("incomplete");
    for (const point of FOUR_POINTS) {
      expect(text).toContain(`dpr ${point.pixelRatio.toFixed(2)}`);
    }
    // The condition no code can assert, said out loud where the number
    // is quoted, because the number means nothing without it.
    expect(text).toContain("vsync");
  });
});

describe("medianMs", () => {
  it("takes the middle sample, not the mean of a hitch", () => {
    // One frame that stalled must not move the point it is in. A mean
    // over these is 24 ms; the middle of them is 4.
    expect(medianMs([4.1, 3.9, 4, 4.2, 104])).toBe(4.1);
  });

  it("averages the two middles of an even run, in any order", () => {
    expect(medianMs([4, 2, 8, 6])).toBe(5);
  });

  it("reports zero rather than a NaN when there is nothing to take", () => {
    // The sweep never files a point it took no samples for, so this is
    // about what the panel would render if it ever did.
    expect(medianMs([])).toBe(0);
  });
});
