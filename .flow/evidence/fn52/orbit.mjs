// The browser orbit for fn-52's frame bound: tests/browser/render.mjs's own
// timing session (a bare canvas, the hero pose, the renderer's orbit) for the
// presets named on the command line, without the rest of that suite.
import { writeFile } from 'node:fs/promises';
import { chromium } from 'playwright';

const url = process.env.BROWSER_URL ?? 'http://localhost:5189';
const out = process.env.RENDER_EVIDENCE ?? '.flow/evidence/fn52';
const flags = ['--no-sandbox', '--enable-unsafe-webgpu', '--enable-features=Vulkan',
  '--use-angle=vulkan', '--disable-vulkan-surface', '--ignore-gpu-blocklist',
  '--disable-background-timer-throttling', '--disable-backgrounding-occluded-windows',
  '--disable-renderer-backgrounding'];
const browser = await chromium.launch({ channel: 'chromium', headless: false, args: flags });
for (const id of process.argv.slice(2)) {
  const page = await browser.newPage({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
  page.setDefaultTimeout(600_000);
  const route = url + '/render-timing';
  await page.route(route, request => request.fulfill({ contentType: 'text/html',
    body: '<!doctype html><html><body style="margin:0;overflow:hidden">' +
      '<canvas style="display:block;width:100vw;height:100vh"></canvas></body></html>' }));
  await page.goto(route);
  await page.bringToFront();
  const measured = await page.evaluate(async ({ id }) => {
    const { presetById } = await import('/src/browser/core.ts');
    const { familyJson, presetToParams } = await import('/harness/family.ts');
    const { createRenderer } = await import('/src/browser/render.ts');
    const canvas = document.querySelector('canvas');
    const renderer = await createRenderer(canvas);
    try {
      const submitted = renderer.setTree(familyJson({ ...presetToParams(presetById(id)), seed: 7 }));
      renderer.setCamera(renderer.hero());
      renderer.frame();
      return { submitted, report: await renderer.orbit(), userAgent: navigator.userAgent,
        canvas: { width: canvas.width, height: canvas.height } };
    } finally { renderer.dispose(); }
  }, { id });
  await page.close();
  const { report } = measured;
  await writeFile(`${out}/${id}-browser-orbit.json`,
    JSON.stringify({ preset: id, seed: 7, session: 'orbit', ...measured, browser: browser.version(), flags }, null, 2) + '\n');
  console.log(id, report.verdict, `total p50 ${report.total_p50_ms} ms`,
    `| wall p50 ${report.wall_p50_ms} p95 ${report.wall_p95_ms} max ${report.wall_max_ms} over ${report.wall_frames} frames`);
}
await browser.close();
