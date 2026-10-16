// Compiles every dumped shader in headless Chromium (Dawn's Tint) and creates
// each compute entry point's pipeline under the adapter's limits; prints errors.
import { chromium } from '/home/daniel/Projects/telperion/node_modules/playwright/index.mjs';
import { readdir, readFile } from 'node:fs/promises';
const [base, dir] = process.argv.slice(2);
const files = (await readdir(dir)).filter((f) => f.endsWith('.wgsl')).sort();
const shaders = await Promise.all(files.map(async (f) => [f, await readFile(`${dir}/${f}`, 'utf8')]));
const browser = await chromium.launch({
  executablePath: '/usr/bin/chromium', headless: true, timeout: 30000,
  args: ['--headless=new', '--no-sandbox', '--enable-unsafe-webgpu',
    '--enable-features=Vulkan,VulkanFromANGLE,DefaultANGLEVulkan', '--use-angle=vulkan',
    '--use-vulkan=native', '--disable-vulkan-surface', '--ignore-gpu-blocklist', '--disable-gpu-sandbox'],
});
const page = await browser.newPage();
await page.goto(base + 'LICENSE');
const out = await page.evaluate(async (shaders) => {
  let adapter = null;
  for (let i = 0; i < 10 && !adapter; i++) {
    adapter = await navigator.gpu.requestAdapter();
    if (!adapter) await new Promise((r) => setTimeout(r, 1000));
  }
  if (!adapter) return ['no adapter'];
  const device = await adapter.requestDevice({ requiredLimits: {
    maxBufferSize: adapter.limits.maxBufferSize,
    maxStorageBufferBindingSize: adapter.limits.maxStorageBufferBindingSize } });
  const lines = [];
  for (const [name, code] of shaders) {
    device.pushErrorScope('validation');
    const module = device.createShaderModule({ code });
    const info = await module.getCompilationInfo();
    for (const m of info.messages) if (m.type === 'error') lines.push(`${name}:${m.lineNum}:${m.linePos} ${m.message}`);
    const entries = [...code.matchAll(/@compute[^]*?fn\s+(\w+)/g)].map((m) => m[1]);
    for (const entry of entries) {
      try { await device.createComputePipelineAsync({ layout: 'auto', compute: { module, entryPoint: entry } }); }
      catch (e) { lines.push(`${name} ${entry}: ${String(e.message ?? e).slice(0, 300)}`); }
    }
    const err = await device.popErrorScope();
    if (err) lines.push(`${name} scope: ${err.message.slice(0, 300)}`);
    lines.push(`${name}: ${entries.length} entries checked`);
  }
  return lines;
}, shaders);
console.log(out.join('\n'));
await browser.close();
