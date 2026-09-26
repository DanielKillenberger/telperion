// Draws one build's stage artifacts from a `dump stages` file as a frame
// sequence, the camera turning about the tree:
//   grow      the attractor cloud and the scaffold in the order the grower laid
//             it down, then the branches and twigs the local rules add past the
//             crossover; an attractor goes out once a scaffold node has landed
//             within the kill distance of it
//   skeleton  the whole grown skeleton (the GROW stage's artifact)
//   leaves    the kept leaves as points over a faint skeleton (CULL's)
// Run: node video/draw/skeleton.ts <stages.json> <mode> <outdir> <WxH> <frames>
//        <azimuth> <sweep>
import { mkdirSync, readFileSync } from 'node:fs';
import { Camera, Canvas, ease, fit, type Rgb, type Vec3 } from './canvas.ts';

interface Stages {
  crossover: number;
  nodes: number[];
  parents: number[];
  kinds: number[];
  attractors: number[];
  killDistance: number;
  leaves: number[];
  metrics: { height: number };
}

type Mode = 'grow' | 'skeleton' | 'leaves';

export const BACKGROUND: Rgb = [0.012, 0.014, 0.02];
const ATTRACTOR: Rgb = [0.3, 0.62, 1.0];
const SCAFFOLD: Rgb = [0.92, 0.9, 0.84];
const BRANCH: Rgb = [0.62, 0.55, 0.42];
const TWIG: Rgb = [0.3, 0.55, 0.26];
const LEAF: Rgb = [0.45, 0.8, 0.4];

const at = (s: Stages, i: number): Vec3 => [s.nodes[i * 4], s.nodes[i * 4 + 1], s.nodes[i * 4 + 2]];

/// The scaffold node index at which each attractor is first within the kill
/// distance of a grown node, or Infinity where none reaches it.
function spent(s: Stages): number[] {
  const kill2 = s.killDistance ** 2;
  const out: number[] = [];
  for (let a = 0; a < s.attractors.length / 3; a++) {
    const p = [s.attractors[a * 3], s.attractors[a * 3 + 1], s.attractors[a * 3 + 2]];
    let when = Infinity;
    for (let i = 1; i < s.crossover; i++) {
      const n = at(s, i);
      if ((n[0] - p[0]) ** 2 + (n[1] - p[1]) ** 2 + (n[2] - p[2]) ** 2 <= kill2) {
        when = i;
        break;
      }
    }
    out.push(when);
  }
  return out;
}

function edges(s: Stages, canvas: Canvas, camera: Camera, from: number, to: number,
               alpha: number): void {
  for (let i = Math.max(1, from); i < to; i++) {
    const parent = s.parents[i];
    if (parent < 0) continue;
    const a = camera.project(at(s, parent)), b = camera.project(at(s, i));
    if (a[2] <= 0 || b[2] <= 0) continue;
    const colour = i < s.crossover ? SCAFFOLD : s.kinds[i] === 2 ? TWIG : BRANCH;
    const width = (r: number, z: number) => Math.min(6, Math.max(0.45, 0.8 * r * camera.scale(z)));
    canvas.segment([a[0], a[1]], [b[0], b[1]], width(s.nodes[parent * 4 + 3], a[2]),
      width(s.nodes[i * 4 + 3], b[2]), colour, alpha);
  }
}

function frame(s: Stages, mode: Mode, camera: Camera, progress: number,
               spentAt: number[]): Canvas {
  const canvas = new Canvas(camera.width, camera.height, BACKGROUND);
  const count = s.parents.length;
  if (mode === 'leaves') {
    edges(s, canvas, camera, 1, count, 0.12);
    for (let i = 0; i < s.leaves.length; i += 3) {
      const p = camera.project([s.leaves[i], s.leaves[i + 1], s.leaves[i + 2]]);
      if (p[2] > 0) canvas.disc(p[0], p[1], 0.7, LEAF, 0.55);
    }
    return canvas;
  }
  if (mode === 'skeleton') {
    edges(s, canvas, camera, 1, count, 0.9);
    return canvas;
  }
  // The scaffold grows over the first 55%, the local rules over the next 40%.
  const scaffold = Math.round(ease(progress / 0.55) * s.crossover);
  const local = s.crossover + Math.round(ease((progress - 0.55) / 0.4) * (count - s.crossover));
  for (let a = 0; a < spentAt.length; a++) {
    const gone = spentAt[a] <= scaffold;
    const fade = progress > 0.55 ? 1 - ease((progress - 0.55) / 0.2) : 1;
    const p = camera.project([s.attractors[a * 3], s.attractors[a * 3 + 1],
      s.attractors[a * 3 + 2]]);
    if (p[2] > 0) canvas.disc(p[0], p[1], gone ? 0.8 : 1.6, ATTRACTOR,
      (gone ? 0.12 : 0.85) * fade);
  }
  edges(s, canvas, camera, 1, scaffold, 1);
  if (local > s.crossover) edges(s, canvas, camera, s.crossover, local, 0.85);
  return canvas;
}

function main(): void {
  const [file, mode, out, size, frames, azimuth, sweep] = process.argv.slice(2);
  if (!['grow', 'skeleton', 'leaves'].includes(mode) || !sweep) {
    throw new Error('usage: skeleton.ts <stages.json> <grow|skeleton|leaves> <outdir> <WxH> '
      + '<frames> <azimuth> <sweep>');
  }
  const s = JSON.parse(readFileSync(file, 'utf8')) as Stages;
  const [width, height] = size.split('x').map(Number);
  const count = Number(frames);
  mkdirSync(out, { recursive: true });
  const spentAt = mode === 'grow' ? spent(s) : [];
  const h = s.metrics.height;
  // The whole tree fills 90% of the frame's height; growing, the crown is
  // framed, from a little below the crossover up, so the cloud reads.
  const [aim, span] = mode === 'grow' ? [0.62, 0.8] : [0.5, 1];
  const fill = 0.9 * Math.min(1, width / height / 0.6);
  for (let f = 0; f < count; f++) {
    const t = count > 1 ? f / (count - 1) : 0;
    const camera = new Camera([0, h * aim, 0], Number(azimuth) + Number(sweep) * ease(t), 8,
      fit(h * span, fill, 36), 36, width, height);
    frame(s, mode as Mode, camera, t, spentAt)
      .write(`${out}/frame-${String(f + 1).padStart(4, '0')}.png`);
  }
}

main();
