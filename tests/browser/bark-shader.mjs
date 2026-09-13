// Compile the actual renderer in Chromium's WGSL implementation. No tree,
// frame, screenshot or timing run is needed to catch uniformity failures.
import assert from 'node:assert/strict';
import { chromium } from 'playwright';

const origin = process.env.BROWSER_URL;
assert.ok(origin, 'BROWSER_URL must name the already-running harness');
const browser = await chromium.launch({ channel: 'chromium', headless: false, args: [
  '--no-sandbox', '--enable-unsafe-webgpu', '--enable-features=Vulkan',
  '--use-angle=vulkan', '--disable-vulkan-surface', '--ignore-gpu-blocklist',
] });
try {
  const page = await browser.newPage();
  await page.route(`${origin}/shader-validation`, route => route.fulfill({
    contentType: 'text/html', body: '<!doctype html><canvas></canvas>',
  }));
  await page.goto(`${origin}/shader-validation`);
  const errors = await page.evaluate(async () => {
    const pending = [];
    const create = GPUDevice.prototype.createShaderModule;
    GPUDevice.prototype.createShaderModule = function (descriptor) {
      const module = create.call(this, descriptor);
      pending.push(module.getCompilationInfo());
      return module;
    };
    const { createRenderer } = await import('/src/browser/render.ts');
    const renderer = await createRenderer(document.querySelector('canvas'));
    try {
      const reports = await Promise.all(pending);
      if (reports.length === 0) throw Error('No production shader was compiled');
      return reports.flatMap(report => [...report.messages])
        .filter(message => message.type === 'error')
        .map(message => `${message.lineNum}:${message.linePos} ${message.message}`);
    } finally { renderer.dispose(); }
  });
  console.log('Production shader compilation errors:', errors);
  assert.deepEqual(errors, [], 'Chromium rejected a production shader');
} finally { await browser.close(); }
