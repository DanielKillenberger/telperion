import { mkdir, writeFile } from 'node:fs/promises';
import { pathToFileURL } from 'node:url';

/* ------------------------------------------------------------------ *
 * THE PAGE, ON A REAL GPU
 *
 * Everything here is done the way the owner does it: click the preset,
 * drag the dial, change the view, press the timing button. Nothing
 * reaches past the page into the renderer to make an assertion easier,
 * with one exception that is not a shortcut - the timing sessions run
 * on a bare canvas of their own, because a session that shares its
 * device with a live frame loop measures the loop as well.
 *
 * The renderer refuses a software adapter, and a headless Chromium on
 * Linux is offered nothing else. So this run needs a display, and when
 * there is no hardware behind it the test says why and exits zero: a
 * machine without a GPU has not failed anything.
 * ------------------------------------------------------------------ */

/** What this machine needs to hand Chromium the hardware adapter. The
 *  evidence records them, because a timing number without the flags it
 *  was measured under is not reproducible. */
const HARDWARE_FLAGS = ['--no-sandbox', '--enable-unsafe-webgpu', '--enable-features=Vulkan',
  '--use-angle=vulkan', '--disable-vulkan-surface', '--ignore-gpu-blocklist',
  /* The orbit session is judged on the page's own animation clock, and a
     browser slows that clock down when it decides nobody is looking. This
     display belongs to somebody, so a window can be covered mid-run; what
     is being measured is a page a viewer is watching, not a background tab. */
  '--disable-background-timer-throttling', '--disable-backgrounding-occluded-windows',
  '--disable-renderer-backgrounding'];
/** The other half of R1: a browser with no GPU at all. */
const SOFTWARE_FLAGS = ['--no-sandbox', '--enable-unsafe-webgpu', '--disable-gpu'];

/** Chrome rounds every WebGPU timestamp to this unless it is running
 *  with developer features or the unsafe-WebGPU flag, so a browser
 *  percentile can be a multiple of it and no finer. Whether this run was
 *  rounded is read off the percentiles rather than assumed: they are the
 *  only samples the record keeps. */
const QUANTIZATION_US = 100;
const quantizationNote = (report) => {
  const step = QUANTIZATION_US / 1000;
  const rounded = report.verdict !== 'valid' || [report.p50_ms, report.p95_ms]
    .every(ms => Math.abs(ms / step - Math.round(ms / step)) < 1e-6);
  /* The page's own clock is coarsened to the same step, so an orbit's wall
     numbers land on that grid whatever the GPU timestamps did. Said here
     rather than left for a reader to wonder at a frame time of exactly
     10.00 ms. */
  const wall = report.wall_p50_ms === undefined ? ''
    : ` The animation clock the wall numbers are taken from is coarsened to that same ` +
      `${QUANTIZATION_US} microsecond step, so they sit on the grid by construction.`;
  return `Chrome quantizes WebGPU timestamps to ${QUANTIZATION_US} microseconds unless developer ` +
    `features or --enable-unsafe-webgpu are on. ${rounded
      ? 'The percentiles here sit on that grid, so read them as quantized.'
      : 'The percentiles here are finer than that step, so this session was not quantized.'}${wall}`;
};

/** The seed the native evidence was measured at, so the browser rows
 *  beside it are the same trees. */
const SEED = 7;
/** Sixty frames a second, as a page has to hold it: no more than 16.7 ms
 *  from one frame to the next at the tail, and never a frame past 33 ms,
 *  which is where a viewer sees the picture stop. */
const ORBIT_P95_MS = 16.7;
const ORBIT_MAX_MS = 33;
const SOAK_MINUTES = 5;
const SOAK_POLL_MS = 30_000;
/** The hardware adapter needs a display, and this machine's display
 *  belongs to somebody. A window somebody clicks in is not idle, so the
 *  soak watches for input it did not send itself and starts again. */
const SOAK_ATTEMPTS = 3;
class Disturbed extends Error {}

const url = process.env.BROWSER_URL ?? 'http://127.0.0.1:5173';
/* The spec being measured, so a plain run files its records where this
   spec's evidence lives and never writes over a closed spec's numbers. */
const evidence = process.env.RENDER_EVIDENCE ?? '.flow/evidence/fn23';
const { chromium } = await import(process.env.PLAYWRIGHT_MODULE
  ? pathToFileURL(process.env.PLAYWRIGHT_MODULE).href : 'playwright');

const check = (ok, message) => { if (!ok) throw Error(message); };
const skip = (reason) => { console.log(`SKIP: ${reason}`); process.exit(0); };
const save = (name, value) => writeFile(`${evidence}/${name}`, JSON.stringify(value, null, 2) + '\n');
const counts = (text) => {
  const found = /([\d,]+) wood tris, ([\d,]+) wood verts, ([\d,]+) foliage instances/.exec(text);
  check(found !== null, `panel reported no counts: ${text}`);
  return found.slice(1, 4).map(number => Number(number.replaceAll(',', '')));
};
const drawn = (text) => {
  const found = /([\d,]+) scene draws, ([\d,]+) tris drawn, ([\d,]+) instances drawn/.exec(text);
  check(found !== null, `panel reported no frame statistics: ${text}`);
  return found.slice(1, 4).map(number => Number(number.replaceAll(',', '')));
};

/** Reads the canvas the way a viewer sees it. A WebGPU canvas hands its
 *  pixels to `toDataURL` and to nothing else here - `drawImage` off the
 *  canvas itself comes back transparent - so the picture goes through an
 *  image and lands in a small 2D canvas as colours and a hash. */
function readCanvas() {
  window.readCanvas = async () => {
    const canvas = document.querySelector('canvas');
    const image = new Image();
    image.src = canvas.toDataURL('image/png');
    await image.decode();
    const off = document.createElement('canvas');
    [off.width, off.height] = [96, 60];
    const context = off.getContext('2d', { willReadFrequently: true });
    context.drawImage(image, 0, 0, off.width, off.height);
    const pixels = context.getImageData(0, 0, off.width, off.height).data;
    const colours = new Set();
    let hash = 2166136261;
    for (let i = 0; i < pixels.length; i += 4) {
      colours.add(`${pixels[i] >> 3},${pixels[i + 1] >> 3},${pixels[i + 2] >> 3}`);
      hash = Math.imul(hash ^ pixels[i], 16777619) ^ pixels[i + 1];
    }
    return { colours: colours.size, hash: (hash >>> 0).toString(16) };
  };
  /* Every rebuild the panel does, counted where it shows: the note that
     carries the build's own milliseconds. The soak's claim is that
     nothing rebuilds while nobody is touching anything, and only a
     counter that was running the whole time can say so. The document
     has no element yet at this point, so the observer waits for one -
     which is still before the page's own module has run. */
  window.rebuilds = 0;
  /* And every event that could have caused one. The test drives the
     page with input of its own, so these are read only across a window
     the test spends idle, where anything at all came from outside. */
  window.input = [];
  for (const type of ['pointerdown', 'keydown', 'wheel', 'input']) {
    addEventListener(type, event => window.input.push(
      { at: Math.round(performance.now()), type, on: event.target?.id || event.target?.nodeName }), true);
  }
  addEventListener('DOMContentLoaded', () => {
    new MutationObserver(records => {
      for (const record of records) {
        if (/wood tris/.test(record.target.textContent ?? '')) window.rebuilds++;
      }
    }).observe(document.body, { subtree: true, characterData: true, childList: true });
  });
}

async function open(browser, width = 1280, height = 800) {
  const page = await browser.newPage({ viewport: { width, height }, deviceScaleFactor: 1 });
  page.setDefaultTimeout(600_000);
  page.on('pageerror', error => console.error('page error:', String(error).slice(0, 300)));
  await page.addInitScript(readCanvas);
  return page;
}

/** Waits for the renderer to have drawn its first tree, or for the panel
 *  to say why it never will - in the renderer's own words, which is what
 *  both the skip and the no-GPU assertion are read from. */
async function start(page) {
  await page.goto(url + '/');
  const stats = page.locator('.gd-note').filter({ hasText: /wood tris/ });
  const alert = page.getByRole('alert');
  await Promise.race([stats.waitFor(), alert.waitFor()]);
  return await alert.count() ? { failed: await alert.textContent() } : { stats };
}

/** One species, timed on a canvas that draws nothing else. A turning
 *  session orbits the hero pose and reports the page's own frame cadence
 *  beside the GPU numbers; a still one holds the pose, as fn-22 did. */
async function session(browser, preset, flags, turning = false) {
  const page = await open(browser, 1600, 1000);
  const route = url + '/render-timing';
  await page.route(route, request => request.fulfill({ contentType: 'text/html',
    body: '<!doctype html><html><body style="margin:0;overflow:hidden">' +
      '<canvas style="display:block;width:100vw;height:100vh"></canvas></body></html>' }));
  await page.goto(route);
  /* The animation clock the orbit is measured on is the clock of a page
     somebody is looking at, so this one is in front. */
  await page.bringToFront();
  const measured = await page.evaluate(async ({ id, seed, turning }) => {
    const { presetById } = await import('/src/browser/core.ts');
    const { familyJson, presetToParams } = await import('/harness/family.ts');
    const { createRenderer } = await import('/src/browser/render.ts');
    const canvas = document.querySelector('canvas');
    /* Who the browser says is drawing. The Rust report cannot name the
       adapter in a browser - WebGPU tells the module nothing about the
       hardware - so the page's own view of it is recorded beside it. */
    const { vendor, architecture, device, description } = (await navigator.gpu.requestAdapter()).info;
    const renderer = await createRenderer(canvas);
    try {
      const submitted = renderer.setTree(familyJson({ ...presetToParams(presetById(id)), seed }));
      renderer.setCamera(renderer.hero());
      renderer.frame();
      return { submitted, report: turning ? await renderer.orbit() : await renderer.timing(),
        userAgent: navigator.userAgent,
        webgpu: { vendor, architecture, device, description },
        canvas: { width: canvas.width, height: canvas.height } };
    } finally { renderer.dispose(); }
  }, { id: preset.id, seed: SEED, turning });
  check(await page.evaluate(() => window.telperionLiveDevices) === 0, 'timing page left a device behind');
  await page.close();
  const record = { species: preset.name, preset: preset.id, seed: SEED, view: 'whole',
    session: turning ? 'orbit' : 'still', ...measured, browser: browser.version(), flags,
    quantization_us: QUANTIZATION_US, note: quantizationNote(measured.report) };
  const species = preset.id === 'norway-spruce' ? 'spruce' : 'oak';
  await save(`${species}-browser-${turning ? 'orbit' : 'timing'}.json`, record);
  const { report } = measured;
  console.log(`${turning ? 'orbit' : 'timing'} ${preset.name}: ${report.verdict}`,
    report.verdict === 'valid'
      ? `p50 ${report.p50_ms.toFixed(3)} ms, p95 ${report.p95_ms.toFixed(3)} ms over ${report.samples} frames`
      : report.reason ?? '',
    report.wall_p50_ms === undefined ? '' : `| wall p50 ${report.wall_p50_ms.toFixed(2)} ms, ` +
      `p95 ${report.wall_p95_ms.toFixed(2)} ms, worst ${report.wall_max_ms.toFixed(2)} ms ` +
      `over ${report.wall_frames} frames`);
  return record;
}

/** The frame rate the orbit held, judged. The wall clock is the page's own
 *  and stands whether or not the GPU could be timed, so a record without it
 *  is a session that never turned rather than one that ran slowly. */
function judgeOrbit(record) {
  const { report } = record;
  check(report.wall_p50_ms !== undefined,
    `the ${record.species} orbit recorded no frame cadence: ${report.verdict} ${report.reason ?? ''}`);
  check(report.wall_p95_ms < ORBIT_P95_MS, `the ${record.species} orbit's p95 frame took ` +
    `${report.wall_p95_ms.toFixed(2)} ms, past the ${ORBIT_P95_MS} ms a 60 fps page has`);
  check(report.wall_max_ms < ORBIT_MAX_MS, `the ${record.species} orbit's worst frame took ` +
    `${report.wall_max_ms.toFixed(2)} ms, past ${ORBIT_MAX_MS} ms, over ${report.wall_frames} frames`);
}

/** Five minutes of nobody touching anything, then a drag. */
async function soakOnce(browser, flags) {
  const page = await open(browser);
  try {
    const { failed, stats } = await start(page);
    check(failed === undefined, `soak could not start: ${failed}`);
    await page.getByRole('button', { name: 'oregon white oak', exact: true }).click();
    await page.waitForFunction(() => window.rebuilds > 0);
    await page.waitForTimeout(2000);
    /* From here the test sends nothing until the drag at the end, so
       the page's own counters are what the five minutes are made of.
       Focus is dropped first: this display belongs to somebody, and a
       stray keystroke that lands on a focused preset button would press
       it. What still arrives is recorded rather than assumed away. */
    await page.evaluate(() => {
      document.activeElement?.blur();
      window.soakCanvas = document.querySelector('canvas');
      window.soakFrom = window.rebuilds;
      window.input.length = 0;
    });
    const started = Date.now();
    const samples = [];
    while (Date.now() - started < SOAK_MINUTES * 60_000) {
      await page.waitForTimeout(SOAK_POLL_MS);
      const sample = await page.evaluate(async () => ({
        minutes: 0,
        devices: window.telperionLiveDevices,
        canvases: document.querySelectorAll('canvas').length,
        sameCanvas: document.querySelector('canvas') === window.soakCanvas,
        rebuilds: window.rebuilds - window.soakFrom,
        input: window.input,
        picture: await window.readCanvas(),
      }));
      sample.minutes = Number(((Date.now() - started) / 60_000).toFixed(2));
      sample.stray = sample.input.length;
      delete sample.input;
      samples.push(sample);
      check(sample.devices === 1, `soak at ${sample.minutes} min: ${sample.devices} live devices`);
      check(sample.canvases === 1 && sample.sameCanvas, `soak at ${sample.minutes} min: the canvas was replaced`);
      check(await page.getByRole('alert').count() === 0, `soak at ${sample.minutes} min: the panel raised an alert`);
      /* A rebuild is the one thing a stray keystroke could have asked
         for, so a rebuild with input behind it is this display's fault
         and the soak starts again; a rebuild with nothing behind it is
         the page rebuilding itself, which is the defect R6 is about. */
      if (sample.rebuilds > 0 && sample.stray > 0) {
        throw new Disturbed(`somebody used the window at ${sample.minutes} min ` +
          `(${sample.stray} events, and ${sample.rebuilds} rebuilds after them)`);
      }
      check(sample.rebuilds === 0, `soak at ${sample.minutes} min: ${sample.rebuilds} rebuilds nobody asked for`);
    }
    const before = await page.evaluate(() => window.readCanvas());
    const box = await page.locator('canvas').boundingBox();
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.down();
    await page.mouse.move(box.x + box.width / 2 + 200, box.y + box.height / 2 + 40, { steps: 10 });
    await page.mouse.up();
    await page.waitForTimeout(1000);
    const after = await page.evaluate(() => window.readCanvas());
    check(before.hash !== after.hash, 'the orbit did not move after the soak');
    const stray = samples.reduce((total, sample) => total + sample.stray, 0);
    const record = { minutes: SOAK_MINUTES, poll_seconds: SOAK_POLL_MS / 1000, preset: 'oregon-white-oak',
      tree: counts(await stats.textContent()), browser: browser.version(), flags, samples,
      /* This run shares a display with a person, so what arrived from
         outside is part of the record rather than swept out of it. */
      stray_input: stray, orbit: { before, after, responded: true },
      verdict: 'one canvas, one device, no rebuild nobody asked for, orbit responds' };
    await save('soak.json', record);
    console.log(`soak: ${samples.length} polls over ${SOAK_MINUTES} minutes, one device throughout,` +
      ` ${stray} stray events from the desktop, no rebuild, orbit responded`);
    return record;
  } finally { await page.close().catch(() => undefined); }
}

/** The soak, as many times as it takes to get five undisturbed minutes. */
async function soak(browser, flags) {
  for (let attempt = 1; ; attempt++) {
    try {
      return await soakOnce(browser, flags);
    } catch (error) {
      const interrupted = error instanceof Disturbed || /closed/.test(error.message);
      check(interrupted && attempt < SOAK_ATTEMPTS,
        interrupted ? `the soak never got five undisturbed minutes: ${error.message}` : error.message);
      console.log(`soak attempt ${attempt}: ${error.message} - starting again`);
    }
  }
}

await mkdir(evidence, { recursive: true });
const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_EXECUTABLE,
  channel: 'chromium', headless: process.env.RENDER_HEADLESS === '1', args: HARDWARE_FLAGS });
try {
  const page = await open(browser);
  const { failed, stats } = await start(page);
  /* The renderer's own words decide whether this machine can run the
     suite: no WebGPU, or a software adapter, is a skip and not a
     failure. Anything else the panel says is a real defect. */
  if (failed !== undefined) {
    if (/WebGPU is unavailable|software fallback/.test(failed)) skip(failed);
    throw Error(`the page failed to start: ${failed}`);
  }

  const presets = await page.evaluate(async () =>
    (await import('/src/browser/core.ts')).PRESETS.map(preset => ({ id: preset.id, name: preset.name })));
  const frame = page.locator('.gd-note').filter({ hasText: /scene draws/ });
  const pictures = new Map();
  for (const preset of presets) {
    const before = await stats.textContent();
    await page.getByRole('button', { name: preset.name.toLowerCase(), exact: true }).click();
    await page.waitForFunction(text => [...document.querySelectorAll('.gd-note')]
      .some(node => /wood tris/.test(node.textContent) && node.textContent !== text), before);
    await page.waitForTimeout(1000);
    check(await page.getByRole('alert').count() === 0, `${preset.name} raised an alert`);
    const [tris, verts, instances] = counts(await stats.textContent());
    check(tris > 0 && verts > 0, `${preset.name} generated no wood`);
    const picture = await page.evaluate(() => window.readCanvas());
    check(picture.colours > 8, `${preset.name} left the canvas blank (${picture.colours} colours)`);
    pictures.set(preset.id, picture.hash);
    console.log(`preset ${preset.name}: ${tris} wood tris, ${instances} foliage instances, ${picture.colours} colours`);
  }
  check(new Set(pictures.values()).size === pictures.size, 'two presets drew the same picture');

  const before = await stats.textContent();
  await page.locator('#gd-height').focus();
  for (let notch = 0; notch < 8; notch++) await page.keyboard.press('ArrowRight');
  await page.waitForFunction(text => [...document.querySelectorAll('.gd-note')]
    .some(node => /wood tris/.test(node.textContent) && node.textContent !== text), before);
  check(counts(await stats.textContent()).join() !== counts(before).join(), 'the height dial changed no counts');
  console.log('height dial:', before.trim(), '->', (await stats.textContent()).trim());

  const whole = drawn(await frame.textContent());
  const views = { whole };
  for (const view of ['bare', 'leaf']) {
    await page.getByLabel('view', { exact: true }).selectOption(view);
    await page.waitForTimeout(1000);
    check(await page.getByRole('alert').count() === 0, `the ${view} view raised an alert`);
    views[view] = drawn(await frame.textContent());
    check(views[view][2] !== whole[2], `the ${view} view drew the whole tree's ${whole[2]} instances`);
  }
  check(views.bare[2] === 0 && views.bare[1] > 0, 'the bare view is wood and no foliage');
  check(views.leaf[2] === 1, 'the leaf view is one placed element');
  console.log('views:', JSON.stringify(views));
  await page.close();

  const timings = [];
  const orbits = [];
  for (const id of ['oregon-white-oak', 'norway-spruce']) {
    const preset = presets.find(entry => entry.id === id);
    timings.push(await session(browser, preset, HARDWARE_FLAGS));
    orbits.push(await session(browser, preset, HARDWARE_FLAGS, true));
  }
  /* The oak is what the frame budget is judged on. The spruce's needles are
     a later spec's problem, so its orbit is recorded and not gated. */
  judgeOrbit(orbits[0]);

  const soaked = process.env.SOAK === '1' ? await soak(browser, HARDWARE_FLAGS) : null;
  if (soaked === null) console.log('soak: not run (SOAK=1 runs the five minute soak)');

  /* The other browser: no GPU at all, and the panel has to say so
     rather than show an empty canvas. */
  const blind = await chromium.launch({ executablePath: process.env.CHROMIUM_EXECUTABLE,
    channel: 'chromium', headless: true, args: SOFTWARE_FLAGS });
  try {
    const page = await open(blind);
    const { failed } = await start(page);
    check(failed !== undefined, 'a browser with no GPU drew a tree anyway');
    check(/WebGPU|adapter/.test(failed), `the no-GPU message names no condition: ${failed}`);
    console.log('no GPU:', failed.replace('retry build', '').trim());
    /* The other half of the orbit: a session with no timestamp feature keeps
       its wall numbers and reads GPU time as unavailable. This browser never
       gets that far - the renderer refuses a software adapter before a
       session exists - so the case is skipped here with the reason printed,
       and the record's own rule is asserted in the Rust timing suite. */
    console.log('orbit without a GPU clock: skipped -',
      failed.replace('retry build', '').trim());
  } finally { await blind.close(); }

  console.log(`PASS: ${presets.length} presets, dial, views, ${timings.length} timing sessions,` +
    ` ${orbits.length} orbit sessions${soaked ? ', five minute soak' : ''}`);
} finally { await browser.close(); }
