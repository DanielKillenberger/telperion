import { deflateSync } from 'node:zlib';
import { describe, expect, it } from 'vitest';
import { decodePng, pixelDiff, report } from './pixel-diff.mjs';

/* The helper counts pixels, not intensity: two PNGs that differ in exactly
 * 37 pixels, 25 of them by 1/255 in one channel, are 37 differing pixels
 * and 12 over 8/255. An intensity-weighted metric (what ImageMagick 7's
 * `compare -metric AE` reported in fn-258) reads 25 pixels at 1/255 as
 * about a tenth of one. Rows are written with every PNG filter in turn, so
 * the decoder's unfiltering is in the count. */
const W = 64, H = 32;

const table = Array.from({ length: 256 }, (_, n) => {
  let c = n;
  for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
  return c >>> 0;
});
const crc = (/** @type {Buffer} */ b) => {
  let c = 0xffffffff;
  for (const x of b) c = table[(c ^ x) & 255] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
};
const chunk = (/** @type {string} */ type, /** @type {Buffer} */ body) => {
  const head = Buffer.alloc(8);
  head.writeUInt32BE(body.length, 0);
  head.write(type, 4, 'latin1');
  const tail = Buffer.alloc(4);
  tail.writeUInt32BE(crc(Buffer.concat([head.subarray(4), body])), 0);
  return Buffer.concat([head, body, tail]);
};

/** An RGBA PNG whose row y is written with filter y % 5. */
function encode(/** @type {Uint8Array} */ rgba, width = W, height = H, declared = width) {
  const stride = width * 4;
  const raw = Buffer.alloc((stride + 1) * height);
  const at = (/** @type {number} */ y, /** @type {number} */ i) =>
    y < 0 || i < 0 ? 0 : rgba[y * stride + i];
  for (let y = 0; y < height; y++) {
    const filter = y % 5;
    raw[y * (stride + 1)] = filter;
    for (let i = 0; i < stride; i++) {
      const [a, b, c] = [at(y, i - 4), at(y - 1, i), at(y - 1, i - 4)];
      const p = a + b - c;
      const [pa, pb, pc] = [Math.abs(p - a), Math.abs(p - b), Math.abs(p - c)];
      const paeth = pa <= pb && pa <= pc ? a : pb <= pc ? b : c;
      const predictor = [0, a, b, Math.floor((a + b) / 2), paeth][filter];
      raw[y * (stride + 1) + 1 + i] = (at(y, i) - predictor) & 255;
    }
  }
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(declared, 0);
  ihdr.writeUInt32BE(height, 4);
  ihdr.set([8, 6, 0, 0, 0], 8);
  const signature = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);
  return Buffer.concat([
    signature,
    chunk('IHDR', ihdr),
    chunk('IDAT', deflateSync(raw)),
    chunk('IEND', Buffer.alloc(0)),
  ]);
}

const base = Uint8Array.from({ length: W * H * 4 }, (_, i) => (i * 37 + (i >> 7) * 11) & 255);
const changed = base.slice();
for (let k = 0; k < 37; k++) {
  const channel = (k * 7 + 40) * 4 + (k % 3);
  changed[channel] = k < 25 ? base[channel] ^ 1 : (base[channel] + 200) & 255;
}

describe('pixel-diff', () => {
  it('counts the pixels that differ, not how much they differ', () => {
    const a = decodePng(encode(base));
    const b = decodePng(encode(changed));
    expect(Buffer.from(a.rgba).equals(Buffer.from(base))).toBe(true);
    const d = pixelDiff(a, b);
    expect([d.pixels, d.differing, d.over]).toEqual([W * H, 37, 12]);
    expect(report(d)).toEqual([
      'pixels where any channel differs: 37 of 2048 pixels (64x32, whole frame), 1.8066%',
      'pixels where any channel differs by more than 8/255: 12 of 2048 pixels (64x32, whole frame), 0.5859%',
    ]);
  });

  it('refuses two images of different sizes, naming both', () => {
    const wide = decodePng(encode(new Uint8Array(65 * H * 4), 65, H));
    expect(() => pixelDiff(decodePng(encode(base)), wide, ['old.png', 'new.png'])).toThrow(
      'images differ in size: old.png 64x32, new.png 65x32',
    );
  });

  it('refuses image data shorter than its header declares, naming the file', () => {
    expect(() => decodePng(encode(new Uint8Array(4), 1, 1, 2), 'short.png')).toThrow(
      'short.png: 5 bytes of image data, 9 for 2x1',
    );
  });
});
