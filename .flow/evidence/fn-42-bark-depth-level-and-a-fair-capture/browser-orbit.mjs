// One fn26-compatible browser orbit on an isolated canvas, without the harness loop.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { resolve } from 'node:path';
import { createServer } from 'vite';

const root = fileURLToPath(new URL('../../../', import.meta.url));
const directory = resolve(process.env.BARK_BROWSER_DIR ??
  fileURLToPath(new URL('browser-candidate/', import.meta.url)));
const flags = ['--no-sandbox', '--enable-unsafe-webgpu', '--enable-features=Vulkan',
  '--use-angle=vulkan', '--disable-vulkan-surface', '--ignore-gpu-blocklist',
  '--disable-background-timer-throttling', '--disable-backgrounding-occluded-windows',
  '--disable-renderer-backgrounding'];
const { chromium } = await import(process.env.PLAYWRIGHT_MODULE
  ? pathToFileURL(process.env.PLAYWRIGHT_MODULE).href : 'playwright');
const git = (...args) => execFileSync('git', args, { cwd: root, encoding: 'utf8' }).trim();
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const artifacts = [
  'src/browser/telperion.wasm', 'src/browser/presets.generated.ts',
  'src/browser/render/telperion_render_bg.wasm',
  'crates/telperion-core/src/presets/materials.rs',
  'crates/telperion-render/src/shaders/wood.wgsl',
  'crates/telperion-render/src/shaders/plates.wgsl',
  'crates/telperion-render/src/shaders/common.wgsl',
  'crates/telperion-render/src/scene/frame.rs',
  'crates/telperion-core/src/material.rs',
];
await mkdir(directory, { recursive: true });
const provenance = {
  source_commit: git('rev-parse', 'HEAD'), source_status: git('status', '--porcelain'),
  tracked_diff_sha256: hash(git('diff', 'HEAD')), files_sha256: {},
};
for (const path of artifacts) provenance.files_sha256[path] = hash(await readFile(resolve(root, path)));
let server;
let browser;
const pageErrors = [];
try {
  server = await createServer({ root, configFile: resolve(root, 'vite.config.ts'),
    server: { host: '127.0.0.1', port: 0, strictPort: true, open: false } });
  await server.listen();
  const address = server.httpServer.address();
  if (!address || typeof address === 'string') throw Error('Vite returned no TCP port');
  const origin = `http://127.0.0.1:${address.port}`;
  browser = await chromium.launch({ executablePath: process.env.CHROMIUM_EXECUTABLE,
    channel: 'chromium', headless: process.env.RENDER_HEADLESS === '1', args: flags });
  const page = await browser.newPage({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
  page.setDefaultTimeout(600_000);
  page.on('pageerror', error => pageErrors.push(String(error)));
  const route = origin + '/render-timing';
  await page.route(route, request => request.fulfill({ contentType: 'text/html',
    body: '<!doctype html><html><body style="margin:0;overflow:hidden">' +
      '<canvas style="display:block;width:100vw;height:100vh"></canvas></body></html>' }));
  await page.goto(route);
  await page.bringToFront();
  const measured = await page.evaluate(async () => {
    const { presetById } = await import('/src/browser/core.ts');
    const { familyJson, presetToParams } = await import('/harness/family.ts');
    const { createRenderer } = await import('/src/browser/render.ts');
    const canvas = document.querySelector('canvas');
    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) throw Error('WebGPU adapter unavailable');
    const { vendor, architecture, device, description } = adapter.info;
    const renderer = await createRenderer(canvas);
    try {
      const family = familyJson({ ...presetToParams(presetById('oregon-white-oak')), seed: 7 });
      const submitted = renderer.setTree(family);
      const camera = renderer.hero();
      renderer.setCamera(camera);
      renderer.frame();
      const report = await renderer.orbit();
      return { submitted, camera, family, report, userAgent: navigator.userAgent,
        webgpu: { vendor, architecture, device, description },
        canvas: { width: canvas.width, height: canvas.height },
        visibility: document.visibilityState, devicePixelRatio };
    } finally { renderer.dispose(); }
  });
  const liveDevices = await page.evaluate(() => window.telperionLiveDevices);
  const record = { species: 'Oregon white oak', preset: 'oregon-white-oak', seed: 7,
    view: 'whole', session: 'orbit', orbit_seconds: 10, ...measured,
    browser: browser.version(), flags, origin, provenance, pageErrors, liveDevices,
    orbit_fps: measured.report.wall_frames === undefined ? null : measured.report.wall_frames / 10,
    quantization_us: 100,
    note: 'Browser animation wall clock is coarsened to 100 microseconds. GPU validity is the renderer report verdict; no 60fps acceptance ceiling is imposed.' };
  await writeFile(resolve(directory, 'oak-browser-orbit.json'), JSON.stringify(record, null, 2) + '\n');
  if (liveDevices !== 0) throw Error(`Timing page left ${liveDevices} renderer devices`);
  if (pageErrors.length) throw Error(`Timing page errors: ${pageErrors.join('; ')}`);
  console.log(JSON.stringify({ verdict: measured.report.verdict, reason: measured.report.reason,
    orbit_fps: record.orbit_fps, wall_p50_ms: measured.report.wall_p50_ms,
    wall_p95_ms: measured.report.wall_p95_ms, wall_max_ms: measured.report.wall_max_ms,
    path: resolve(directory, 'oak-browser-orbit.json') }));
  await page.close();
} catch (error) {
  await writeFile(resolve(directory, 'browser-orbit-error.json'), JSON.stringify({
    status: 'unavailable', reason: String(error), flags, provenance, pageErrors,
  }, null, 2) + '\n');
  throw error;
} finally {
  try { if (browser) await browser.close(); }
  finally { if (server) await server.close(); }
}
