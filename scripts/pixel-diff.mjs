import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { inflateSync } from 'node:zlib';

/* How many pixels two stills differ in, counted over the whole frame
 * (fn-259 R4). ImageMagick 7's `compare -metric AE` weighs each pixel by
 * how much it differs and understated a difference 100× (fn-258
 * FRICTION.md); this counts pixels, and says so on every line it prints.
 *
 *   node scripts/pixel-diff.mjs <a.png> <b.png>
 *
 * Reads 8-bit RGB and RGBA PNGs, not interlaced (what the native stills
 * and ImageMagick write); refuses any other form and two images of
 * different sizes, naming both.
 */

/** The second count's threshold, in 1/255 steps of a channel (fn-259 R4). */
export const NEAR = 8;

/**
 * @typedef {{ width: number, height: number, rgba: Uint8Array }} Image
 * @typedef {{ width: number, height: number, pixels: number, differing: number, over: number }} Diff
 */

/** Decodes an 8-bit RGB or RGBA PNG to RGBA. @returns {Image} */
export function decodePng(/** @type {Uint8Array} */ bytes, name = 'image') {
  const buf = Buffer.from(bytes);
  if (buf.readUInt32BE(0) !== 0x89504e47) throw new Error(`${name}: not a PNG`);
  let width = 0, height = 0, channels = 0;
  const data = [];
  for (let at = 8; at < buf.length; ) {
    const length = buf.readUInt32BE(at);
    const type = buf.toString('latin1', at + 4, at + 8);
    const body = buf.subarray(at + 8, at + 8 + length);
    if (type === 'IHDR') {
      width = body.readUInt32BE(0);
      height = body.readUInt32BE(4);
      const [depth, colour, , , interlace] = body.subarray(8, 13);
      channels = depth === 8 && interlace === 0 ? ({ 2: 3, 6: 4 })[colour] ?? 0 : 0;
      if (!channels) {
        throw new Error(
          `${name}: unsupported PNG (bit depth ${depth}, colour type ${colour}, interlace ${interlace}); 8-bit RGB or RGBA, not interlaced, is read`,
        );
      }
    } else if (type === 'IDAT') data.push(body);
    at += 12 + length;
  }
  const raw = inflateSync(Buffer.concat(data));
  const stride = width * channels;
  const rows = new Uint8Array(stride * height);
  for (let y = 0; y < height; y++) unfilter(raw, rows, y, stride, channels);
  const rgba = new Uint8Array(width * height * 4);
  for (let p = 0; p < width * height; p++) {
    for (let c = 0; c < 4; c++) rgba[p * 4 + c] = c < channels ? rows[p * channels + c] : 255;
  }
  return { width, height, rgba };
}

/** Undoes row `y`'s PNG filter from `raw` into `out`. */
function unfilter(
  /** @type {Uint8Array} */ raw,
  /** @type {Uint8Array} */ out,
  /** @type {number} */ y,
  /** @type {number} */ stride,
  /** @type {number} */ bpp,
) {
  const filter = raw[y * (stride + 1)];
  const src = y * (stride + 1) + 1;
  const row = y * stride;
  for (let i = 0; i < stride; i++) {
    const a = i >= bpp ? out[row + i - bpp] : 0;
    const b = y > 0 ? out[row - stride + i] : 0;
    const c = i >= bpp && y > 0 ? out[row - stride + i - bpp] : 0;
    const predictor = [0, a, b, (a + b) >> 1, paeth(a, b, c)][filter];
    if (predictor === undefined) throw new Error(`row ${y}: unknown PNG filter ${filter}`);
    out[row + i] = (raw[src + i] + predictor) & 255;
  }
}

function paeth(/** @type {number} */ a, /** @type {number} */ b, /** @type {number} */ c) {
  const p = a + b - c;
  const [pa, pb, pc] = [Math.abs(p - a), Math.abs(p - b), Math.abs(p - c)];
  return pa <= pb && pa <= pc ? a : pb <= pc ? b : c;
}

/**
 * Pixels where any channel differs, and where any channel differs by more
 * than `NEAR`/255, each a count of pixels over the whole frame.
 * @returns {Diff}
 */
export function pixelDiff(/** @type {Image} */ a, /** @type {Image} */ b, names = ['a', 'b']) {
  if (a.width !== b.width || a.height !== b.height) {
    throw new Error(
      `images differ in size: ${names[0]} ${a.width}x${a.height}, ${names[1]} ${b.width}x${b.height}`,
    );
  }
  let differing = 0, over = 0;
  for (let p = 0; p < a.rgba.length; p += 4) {
    let most = 0;
    for (let c = 0; c < 4; c++) most = Math.max(most, Math.abs(a.rgba[p + c] - b.rgba[p + c]));
    if (most > 0) differing++;
    if (most > NEAR) over++;
  }
  return { width: a.width, height: a.height, pixels: a.width * a.height, differing, over };
}

/** The two lines the command prints, each naming its metric. */
export function report(/** @type {Diff} */ d) {
  const share = (/** @type {number} */ n) => `${((100 * n) / d.pixels).toFixed(4)}%`;
  const whole = `of ${d.pixels} pixels (${d.width}x${d.height}, whole frame)`;
  return [
    `pixels where any channel differs: ${d.differing} ${whole}, ${share(d.differing)}`,
    `pixels where any channel differs by more than ${NEAR}/255: ${d.over} ${whole}, ${share(d.over)}`,
  ];
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const names = process.argv.slice(2);
  if (names.length !== 2) {
    console.error('usage: node scripts/pixel-diff.mjs <a.png> <b.png>');
    process.exit(2);
  }
  try {
    const [a, b] = await Promise.all(names.map(async n => decodePng(await readFile(n), n)));
    console.log(report(pixelDiff(a, b, names)).join('\n'));
  } catch (error) {
    console.error(`pixel-diff: ${error instanceof Error ? error.message : error}`);
    process.exit(1);
  }
}
