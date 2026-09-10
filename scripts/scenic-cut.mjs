// The scenic cut: the whole-tree walk and the leaf walk, crossfaded into one
// clip, graded once and encoded once. The frame sequences are the artefact and
// this is what is posted, so nothing here re-renders anything: it reads two
// sequences off disk, runs ffmpeg once, and writes down exactly what it ran.
import { spawnSync } from 'node:child_process';
import { readdirSync, mkdirSync, writeFileSync } from 'node:fs';
import { basename, dirname, join, resolve } from 'node:path';

const FPS = 24;
// Seconds. What the post allows, and what the cut is checked against before a
// single frame is encoded.
const LIMIT = 24;
// A little contrast, a little warmth in the mids, a soft vignette. The
// renderer's clay is untouched; the grade is ffmpeg's and lives only here.
const GRADE = 'eq=contrast=1.06:saturation=1.05,' +
  'colorbalance=rm=0.020:gm=0.008:bm=-0.020,vignette=PI/6';
const USAGE = 'usage: scenic-cut [--whole <frame.png>] [--leaf <frame.png>] ' +
  '[--out <mp4>] [--fade <seconds>] [--size WxH]';

const defaults = {
  whole: '.flow/evidence/fn25/whole/frame.png',
  leaf: '.flow/evidence/fn25/leaf/frame.png',
  out: '.flow/evidence/fn25/scenic.mp4',
  fade: '0.5',
  size: '1920x1080',
};

function fail(message) {
  console.error(message);
  process.exit(1);
}

function options() {
  const given = { ...defaults };
  const argv = process.argv.slice(2);
  for (let i = 0; i < argv.length; i += 2) {
    const flag = argv[i].startsWith('--') ? argv[i].slice(2) : null;
    if (!flag || !(flag in defaults)) fail(`unknown argument "${argv[i]}"\n${USAGE}`);
    if (argv[i + 1] === undefined) fail(`${argv[i]} needs a value\n${USAGE}`);
    given[flag] = argv[i + 1];
  }
  return given;
}

// A numbered PNG sequence as the headless target writes one: `frame.png` names
// `frame-0001.png` onward beside it. A path with no frames in it is named.
function sequence(view, path) {
  const png = /\.png$/i.test(path);
  const directory = png ? dirname(path) : path;
  const stem = png ? basename(path).replace(/\.png$/i, '') : 'frame';
  let entries;
  try {
    entries = readdirSync(directory);
  } catch {
    fail(`no ${view} sequence at ${resolve(directory)}`);
  }
  const numbered = new RegExp(`^${stem.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}-\\d{4}\\.png$`);
  const frames = entries.filter(name => numbered.test(name)).sort();
  if (!frames.length) fail(`no ${stem}-0001.png frames in the ${view} sequence at ${resolve(directory)}`);
  if (frames[0] !== `${stem}-0001.png`) {
    fail(`the ${view} sequence at ${resolve(directory)} starts at ${frames[0]}, not ${stem}-0001.png`);
  }
  return {
    view,
    pattern: join(directory, `${stem}-%04d.png`),
    frames: frames.length,
    seconds: frames.length / FPS,
  };
}

function number(flag, raw, wants, holds) {
  const value = Number(raw);
  if (!Number.isFinite(value) || !holds(value)) fail(`${flag} wants ${wants}, not "${raw}"`);
  return value;
}

// What the encoder made, read back rather than assumed: the clip is never
// opened here, so its own record is what says it is the clip that was asked for.
function probe(path) {
  const probed = spawnSync('ffprobe', [
    '-v', 'error', '-select_streams', 'v:0',
    '-show_entries', 'stream=codec_name,width,height,pix_fmt,r_frame_rate:format=duration',
    '-of', 'json', path,
  ], { encoding: 'utf8' });
  if (probed.error || probed.status !== 0) return null;
  const { streams: [stream] = [], format = {} } = JSON.parse(probed.stdout);
  if (!stream) return null;
  return {
    codec: stream.codec_name,
    size: [stream.width, stream.height],
    pixelFormat: stream.pix_fmt,
    rate: stream.r_frame_rate,
    seconds: Number(Number(format.duration).toFixed(3)),
  };
}

const given = options();
const [width, height] = (() => {
  const parts = /^(\d+)[xX](\d+)$/.exec(given.size);
  if (!parts) fail(`--size wants WxH in pixels, not "${given.size}"`);
  return [Number(parts[1]), Number(parts[2])];
})();
const whole = sequence('whole', given.whole);
const leaf = sequence('leaf', given.leaf);
const fade = number('--fade', given.fade, 'seconds above zero', value => value > 0);
const shortest = Math.min(whole.seconds, leaf.seconds);
if (fade >= shortest) fail(`a ${fade} s crossfade does not fit a ${shortest} s sequence`);
const seconds = Number((whole.seconds + leaf.seconds - fade).toFixed(3));
if (seconds >= LIMIT) fail(`the cut runs ${seconds} s; the post wants under ${LIMIT} s`);

const filter = [
  `[0:v]scale=${width}:${height}:flags=lanczos,setsar=1,fps=${FPS}[whole]`,
  `[1:v]scale=${width}:${height}:flags=lanczos,setsar=1,fps=${FPS}[leaf]`,
  `[whole][leaf]xfade=transition=fade:duration=${fade}:` +
    `offset=${Number((whole.seconds - fade).toFixed(3))}[cut]`,
  `[cut]${GRADE}[graded]`,
].join(';');
const argv = [
  '-y',
  '-framerate', String(FPS), '-i', whole.pattern,
  '-framerate', String(FPS), '-i', leaf.pattern,
  '-filter_complex', filter,
  '-map', '[graded]',
  '-c:v', 'libx264', '-crf', '18', '-pix_fmt', 'yuv420p', '-r', String(FPS),
  '-movflags', '+faststart',
  given.out,
];

mkdirSync(dirname(given.out), { recursive: true });
const run = spawnSync('ffmpeg', argv, { stdio: ['ignore', 'ignore', 'pipe'], encoding: 'utf8' });
if (run.error?.code === 'ENOENT') {
  console.log('no ffmpeg on the path; the sequences stand');
  process.exit(0);
}
if (run.status !== 0) fail(`ffmpeg refused the cut (${run.status}):\n${(run.stderr ?? '').trim()}`);

const record = {
  clip: resolve(given.out),
  size: [width, height],
  fps: FPS,
  fade,
  planned: { seconds },
  grade: GRADE,
  inputs: [whole, leaf],
  ffmpeg: { command: 'ffmpeg', arguments: argv },
  shell: ['ffmpeg', ...argv].map(word => (/[^\w./:=-]/.test(word) ? `'${word.replace(/'/g, "'\\''")}'` : word)).join(' '),
  encoded: probe(given.out),
};
const beside = join(dirname(given.out), 'scenic.json');
writeFileSync(beside, `${JSON.stringify(record, null, 2)}\n`);
const made = record.encoded;
console.log(
  `${record.clip}: ${made ? `${made.codec} ${made.pixelFormat} ${made.size.join('x')} ` +
    `at ${made.rate}, ${made.seconds} s` : `${seconds} s planned, unprobed`} ` +
  `from ${whole.frames} whole and ${leaf.frames} leaf frames; ${beside} beside it`,
);
