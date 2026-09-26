// A float RGB canvas, a pinhole camera and a PNG writer: what the drawn shots
// need to put a generator artifact on screen, and nothing else.
import { deflateSync } from 'node:zlib';
import { writeFileSync } from 'node:fs';

export type Vec3 = [number, number, number];
export type Rgb = [number, number, number];

export class Canvas {
  readonly width: number;
  readonly height: number;
  readonly pixels: Float32Array;

  constructor(width: number, height: number, background: Rgb) {
    this.width = width;
    this.height = height;
    this.pixels = new Float32Array(width * height * 3);
    for (let i = 0; i < width * height; i++) this.pixels.set(background, i * 3);
  }

  /// Lays `colour` over one pixel at `alpha`, clipped to the frame.
  blend(x: number, y: number, colour: Rgb, alpha: number): void {
    if (x < 0 || y < 0 || x >= this.width || y >= this.height || alpha <= 0) return;
    const i = (y * this.width + x) * 3;
    const a = Math.min(alpha, 1);
    for (let c = 0; c < 3; c++) this.pixels[i + c] += (colour[c] - this.pixels[i + c]) * a;
  }

  /// A soft disc: full inside `radius`, fading over one pixel at its edge.
  disc(cx: number, cy: number, radius: number, colour: Rgb, alpha: number): void {
    const r = Math.max(radius, 0.5);
    const x0 = Math.floor(cx - r - 1), x1 = Math.ceil(cx + r + 1);
    const y0 = Math.floor(cy - r - 1), y1 = Math.ceil(cy + r + 1);
    for (let y = y0; y <= y1; y++) {
      for (let x = x0; x <= x1; x++) {
        const d = Math.hypot(x + 0.5 - cx, y + 0.5 - cy);
        const cover = Math.min(1, Math.max(0, r + 0.5 - d));
        if (cover > 0) this.blend(x, y, colour, alpha * cover);
      }
    }
  }

  /// A stroke between two points with a radius at each end.
  segment(a: [number, number], b: [number, number], ra: number, rb: number,
          colour: Rgb, alpha: number): void {
    const length = Math.hypot(b[0] - a[0], b[1] - a[1]);
    const steps = Math.max(1, Math.ceil(length / Math.max(0.6, Math.min(ra, rb))));
    for (let s = 0; s <= steps; s++) {
      const t = s / steps;
      this.disc(a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, ra + (rb - ra) * t,
        colour, alpha);
    }
  }

  /// The canvas as an 8-bit RGB PNG, gamma-encoded from linear light.
  write(path: string): void {
    const { width, height } = this;
    const raw = Buffer.alloc((width * 3 + 1) * height);
    for (let y = 0; y < height; y++) {
      const row = y * (width * 3 + 1);
      raw[row] = 0;
      for (let i = 0; i < width * 3; i++) {
        const v = this.pixels[y * width * 3 + i];
        raw[row + 1 + i] = Math.round(255 * Math.min(1, Math.max(0, v)) ** (1 / 2.2));
      }
    }
    const chunk = (type: string, data: Buffer): Buffer => {
      const length = Buffer.alloc(4);
      length.writeUInt32BE(data.length);
      const body = Buffer.concat([Buffer.from(type, 'latin1'), data]);
      const crc = Buffer.alloc(4);
      crc.writeUInt32BE(crc32(body));
      return Buffer.concat([length, body, crc]);
    };
    const header = Buffer.alloc(13);
    header.writeUInt32BE(width, 0);
    header.writeUInt32BE(height, 4);
    header.set([8, 2, 0, 0, 0], 8);
    writeFileSync(path, Buffer.concat([
      Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
      chunk('IHDR', header),
      chunk('IDAT', deflateSync(raw, { level: 3 })),
      chunk('IEND', Buffer.alloc(0)),
    ]));
  }
}

const CRC = new Int32Array(256).map((_, n) => {
  let c = n;
  for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
  return c;
});

function crc32(bytes: Buffer): number {
  let c = -1;
  for (const byte of bytes) c = CRC[(c ^ byte) & 0xff] ^ (c >>> 8);
  return (c ^ -1) >>> 0;
}

/// A pinhole camera orbiting a target: `azimuth` degrees about +Y, raised by
/// `elevation` degrees, `distance` metres out, with a vertical field of view.
export class Camera {
  private readonly eye: Vec3;
  private readonly right: Vec3;
  private readonly up: Vec3;
  private readonly forward: Vec3;
  private readonly focal: number;
  readonly width: number;
  readonly height: number;

  constructor(target: Vec3, azimuth: number, elevation: number, distance: number,
              fov: number, width: number, height: number) {
    this.width = width;
    this.height = height;
    const a = (azimuth * Math.PI) / 180, e = (elevation * Math.PI) / 180;
    const back: Vec3 = [Math.sin(a) * Math.cos(e), Math.sin(e), Math.cos(a) * Math.cos(e)];
    this.eye = [target[0] + back[0] * distance, target[1] + back[1] * distance,
      target[2] + back[2] * distance];
    this.forward = [-back[0], -back[1], -back[2]];
    this.right = normalise(cross(this.forward, [0, 1, 0]));
    this.up = cross(this.right, this.forward);
    this.focal = height / 2 / Math.tan((fov * Math.PI) / 360);
  }

  /// Screen x, y and depth in metres; depth at or below zero is behind.
  project(p: Vec3): [number, number, number] {
    const d: Vec3 = [p[0] - this.eye[0], p[1] - this.eye[1], p[2] - this.eye[2]];
    const z = dot(d, this.forward);
    return [this.width / 2 + (dot(d, this.right) * this.focal) / z,
      this.height / 2 - (dot(d, this.up) * this.focal) / z, z];
  }

  /// Pixels a metre spans at `depth`.
  scale(depth: number): number {
    return this.focal / depth;
  }

  /// The world ray through pixel (x, y): origin and unit direction.
  ray(x: number, y: number): [Vec3, Vec3] {
    const u = (x - this.width / 2) / this.focal, v = (this.height / 2 - y) / this.focal;
    const d = normalise([
      this.forward[0] + this.right[0] * u + this.up[0] * v,
      this.forward[1] + this.right[1] * u + this.up[1] * v,
      this.forward[2] + this.right[2] * u + this.up[2] * v,
    ]);
    return [this.eye, d];
  }
}

export const dot = (a: Vec3, b: Vec3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross = (a: Vec3, b: Vec3): Vec3 =>
  [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export function normalise(v: Vec3): Vec3 {
  const l = Math.hypot(v[0], v[1], v[2]) || 1;
  return [v[0] / l, v[1] / l, v[2] / l];
}

/// Smoothstep between 0 and 1.
export const ease = (t: number): number => {
  const x = Math.min(1, Math.max(0, t));
  return x * x * (3 - 2 * x);
};

/// The distance that fits a subject `height` metres tall into `fill` of the
/// frame's height at a vertical field of view of `fov` degrees.
export const fit = (height: number, fill: number, fov: number): number =>
  height / fill / 2 / Math.tan((fov * Math.PI) / 360);
