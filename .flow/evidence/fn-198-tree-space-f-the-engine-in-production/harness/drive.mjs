// fn-198 task 3: drives the dev harness headless, every step under a timeout.
//   node drive.mjs <base-url> <out-dir> <plan.json | inline JSON>
// A plan is a list of steps: {open: "?species=..&age=.."} | {age: 80} |
// {preset: "silver birch"} | {shot: "name.png"}. After each open, age or
// preset it waits for the build that step started to settle, and records
// the time from the change to the first frame drawn after the build.
import { chromium } from 'playwright';
import { readFile, writeFile } from 'node:fs/promises';

const [base, out, planPath] = process.argv.slice(2);
const plan = JSON.parse(planPath.startsWith('[') ? planPath : await readFile(planPath, 'utf8'));
const STEP_MS = Number(process.env.STEP_MS ?? 120000);
const browser = await chromium.launch({
  executablePath: '/usr/bin/chromium', headless: true, timeout: 30000,
  // The hardware adapter headless: without native Vulkan and ANGLE on it,
  // headless Chromium offers only SwiftShader, which the stage refuses.
  args: ['--headless=new', '--no-sandbox', '--enable-unsafe-webgpu',
    '--enable-features=Vulkan,VulkanFromANGLE,DefaultANGLEVulkan', '--use-angle=vulkan',
    '--use-vulkan=native', '--disable-vulkan-surface', '--ignore-gpu-blocklist', '--disable-gpu-sandbox'],
});
const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
page.setDefaultTimeout(STEP_MS);
const errors = [];
/* A Wasm trap leaves the build's promise unsettled, so a trap ends the
   wait at once rather than at the step's timeout. */
let trap = () => {};
const trapped = new Promise((_, reject) => { trap = reject; });
trapped.catch(() => {});
page.on('pageerror', (e) => { errors.push(String(e)); if (/unreachable|RuntimeError/.test(String(e))) trap(new Error(`trap: ${e}`)); });
page.on('console', (m) => { if (m.type() === 'error') errors.push(m.text()); });
page.on('crash', () => errors.push('page crashed'));
await page.addInitScript(() => { window.__qa = { calls: [] }; });
/* Wraps setTreeGpu: each call's start, end, outcome, and the second
   animation frame after it resolved (the tree's first drawn frame). */
await page.route('**/harness/main.tsx*', async (route) => {
  const response = await route.fetch();
  const prefix = `
const {WebRenderer: QaRenderer} = await import('/src/browser/render/telperion_render.js');
const qaGpu = QaRenderer.prototype.setTreeGpu;
QaRenderer.prototype.setTreeGpu = async function (family) {
  const f = JSON.parse(family);
  const row = { age: f.age, seed: f.skeleton?.seed, start: performance.now() };
  window.__qa.calls.push(row);
  try {
    const result = await qaGpu.call(this, family);
    const v = JSON.parse(result);
    row.resolved = performance.now();
    Object.assign(row, { backend: v.backend, foliage: v.foliageInstances, woodTriangles: v.woodTriangles,
      skeletonMs: v.stages?.skeletonMs, totalMs: v.stages?.totalMs });
    requestAnimationFrame(() => requestAnimationFrame(() => { row.drawn = performance.now(); }));
    return result;
  } catch (e) { row.error = String(e?.message ?? e); row.resolved = performance.now(); throw e; }
};
`;
  await route.fulfill({ response, body: prefix + (await response.text()) });
});

const settle = async (count, changed) => {
  await Promise.race([trapped, page.waitForFunction((n) => {
    const c = window.__qa.calls;
    const last = c.at(-1);
    return c.length > n && last && (last.error !== undefined || last.drawn !== undefined);
  }, count, { timeout: STEP_MS })]);
  await page.waitForTimeout(400);
  const calls = await page.evaluate(() => window.__qa.calls);
  const last = calls.at(-1);
  const alert = await page.evaluate(() => [...document.querySelectorAll('[role=alert]')].map((x) => x.textContent));
  return { ...last, changeToDrawnMs: last.drawn === undefined ? undefined : last.drawn - changed,
    changeToResolvedMs: last.resolved - changed, buildCalls: calls.length - count, alert };
};
const log = [];
try {
  for (const step of plan) {
    const count = await page.evaluate(() => window.__qa?.calls.length ?? 0).catch(() => 0);
    if (step.open !== undefined) {
      await page.goto(base + step.open, { timeout: 30000 });
      /* Headless Chromium's GPU process can answer the page's first
         adapter request before Vulkan is up; the panel's own retry
         asks again. */
      for (let tries = 0; tries < 5; tries++) {
        const started = await page.waitForFunction(() => window.__qa.calls.length > 0
          || /adapter|WebGPU is unavailable/.test(document.body.innerText), null, { timeout: 60000 })
          .then(() => page.evaluate(() => window.__qa.calls.length > 0));
        if (started) break;
        await page.waitForTimeout(2000);
        await page.getByRole('button', { name: 'retry build' }).click({ timeout: 10000 });
        await page.waitForTimeout(1000);
      }
      await page.waitForFunction(() => window.__qa.calls.length > 0, null, { timeout: 60000 });
      const changed = await page.evaluate(() => window.__qa.calls[0].start);
      log.push({ step, ...(await settle(0, changed)) });
    } else if (step.age !== undefined) {
      const changed = await page.evaluate(() => performance.now());
      await page.locator('#gd-age').fill(String(step.age));
      log.push({ step, ...(await settle(count, changed)) });
    } else if (step.preset !== undefined) {
      const changed = await page.evaluate(() => performance.now());
      await page.getByRole('button', { name: step.preset, exact: true }).click();
      log.push({ step, ...(await settle(count, changed)) });
    } else if (step.panel !== undefined) {
      await page.evaluate(() => document.querySelector('.gd-panel').scrollTo(0, 0));
      await page.screenshot({ path: `${out}/${step.panel}`, timeout: 30000 });
      log.push({ step });
    } else if (step.shot !== undefined) {
      await page.evaluate(() => [...document.querySelectorAll('button')]
        .find((b) => b.textContent.trim() === 'reframe').click());
      await page.waitForTimeout(800);
      /* A headless page's screenshot does not carry the WebGPU canvas, so
         the canvas is read in the frame it was drawn in: a callback queued
         now runs after the stage's own in the next frame, before present. */
      const url = await page.evaluate(() => new Promise((r) => requestAnimationFrame(() =>
        r(document.querySelector('canvas').toDataURL('image/png')))));
      await writeFile(`${out}/${step.shot}`, Buffer.from(url.split(',')[1], 'base64'));
      log.push({ step });
    }
    console.log(JSON.stringify(log.at(-1)));
  }
} catch (error) {
  const page_ = await page.evaluate(() => ({ url: location.href, qa: window.__qa,
    body: document.body.innerText.slice(0, 1200) })).catch((e) => String(e));
  log.push({ failed: String(error), page: page_ });
  console.log('FAIL', String(error));
} finally {
  log.push({ errors });
  await writeFile(`${out}/drive-${Date.now()}.json`, JSON.stringify(log, null, 2));
  await browser.close();
}
