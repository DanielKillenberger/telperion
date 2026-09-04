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
}

export interface Stage {
  /** Replaces the subject, disposing whatever stood there before. */
  setTree(build: (clay: Clay) => THREE.Object3D): void;
  /** Opt-in key and fill. Off - sky light alone - is the judging mode. */
  setLightingCheck(on: boolean): void;
  /** Pulls the camera back to frame a tree of `height` metres. */
  frame(height: number): void;
  dispose(): void;
}

export function createStage(canvas: HTMLCanvasElement): Stage {
  const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  // No tone mapping and no exposure games: what the geometry does to
  // the light is the only thing on screen.
  renderer.toneMapping = THREE.NoToneMapping;
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;

  const scene = new THREE.Scene();
  scene.background = new THREE.Color(CLAY_BACKGROUND);

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
  key.position.set(24, 40, 18);
  key.castShadow = true;
  key.shadow.mapSize.set(2048, 2048);
  const shadowCamera = key.shadow.camera;
  shadowCamera.near = 1;
  shadowCamera.far = 400;
  shadowCamera.left = -80;
  shadowCamera.right = 80;
  shadowCamera.top = 120;
  shadowCamera.bottom = -20;

  const fill = new THREE.DirectionalLight(0xff_ff_ff, 0.9);
  fill.position.set(-30, 20, -14);

  const clay = new THREE.MeshStandardMaterial({
    color: CLAY,
    roughness: 0.92,
    metalness: 0,
  });
  const clayLine = new THREE.LineBasicMaterial({ color: CLAY_LINE });
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

  const groundGeometry = new THREE.CircleGeometry(400, 96);
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

  const disposeTree = (): void => {
    if (tree === null) return;
    scene.remove(tree);
    tree.traverse((node) => {
      // Lines as well as meshes: the skeleton is LineSegments, and a
      // Mesh-only sweep would leak a geometry on every regenerate.
      if (node instanceof THREE.Mesh || node instanceof THREE.Line) {
        node.geometry.dispose();
      }
    });
    tree = null;
  };

  const setTree: Stage["setTree"] = (build) => {
    disposeTree();
    tree = build({ surface: clay, line: clayLine });
    tree.traverse((node) => {
      if (node instanceof THREE.Mesh) {
        node.castShadow = true;
        node.receiveShadow = true;
      }
    });
    scene.add(tree);
  };

  const frame: Stage["frame"] = (height) => {
    // Stand the figure clear of the root flare, and put the camera far
    // enough back that both it and the crown are in shot.
    figure.position.set(height * 0.16 + 1.2, FIGURE_HEIGHT / 2, height * 0.2);
    controls.target.set(0, height * 0.45, 0);
    camera.position.set(height * 0.85, height * 0.55, height * 1.35);
    controls.update();
  };

  const setLightingCheck: Stage["setLightingCheck"] = (on) => {
    if (on === lightingCheck) return;
    lightingCheck = on;
    renderer.shadowMap.enabled = on;
    if (on) scene.add(key, fill);
    else scene.remove(key, fill);
    // A material compiled without shadows has to be recompiled with
    // them; three only notices when it is told.
    for (const material of [clay, groundMaterial, figureMaterial]) {
      material.needsUpdate = true;
    }
  };

  const resize = (): void => {
    const width = canvas.clientWidth || 1;
    const height = canvas.clientHeight || 1;
    renderer.setSize(width, height, false);
    camera.aspect = width / height;
    camera.updateProjectionMatrix();
  };
  resize();

  const observer = new ResizeObserver(resize);
  observer.observe(canvas);

  let frameHandle = 0;
  const tick = (): void => {
    frameHandle = requestAnimationFrame(tick);
    controls.update();
    renderer.render(scene, camera);
  };
  tick();

  return {
    setTree,
    setLightingCheck,
    frame,
    dispose(): void {
      cancelAnimationFrame(frameHandle);
      observer.disconnect();
      controls.dispose();
      disposeTree();
      groundGeometry.dispose();
      figureGeometry.dispose();
      clay.dispose();
      clayLine.dispose();
      groundMaterial.dispose();
      figureMaterial.dispose();
      renderer.dispose();
    },
  };
}
