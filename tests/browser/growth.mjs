// The page's timeline controls, exercised through their labels on one hardware
// device, like render.mjs. Run separately: the shared GPU is never parallelized.
import { chromium } from 'playwright';
const browser = await chromium.launch({ headless: false, args: [
  '--no-sandbox', '--enable-unsafe-webgpu', '--enable-features=Vulkan',
  '--use-angle=vulkan', '--disable-vulkan-surface', '--ignore-gpu-blocklist',
] });
const check = (ok, message) => { if (!ok) throw Error(message); };
try {
  const page = await browser.newPage();
  page.setDefaultTimeout(120_000);
  await page.goto((process.env.BROWSER_URL ?? 'http://127.0.0.1:5176') + '/?preset=oregon-white-oak');
  const age = page.getByLabel('age (years)', { exact: true });
  await age.waitFor({ timeout: 10_000 });
  await page.waitForFunction(() => document.querySelector('[data-growth-frontier]')?.getAttribute('data-growth-frontier') !== '');
  const frontier = () => page.locator('[data-growth-frontier]').getAttribute('data-growth-frontier').then(Number);
  const original = await frontier();
  const geometry = () => page.getByText(/wood tris, .*wood verts, .*foliage instances/).textContent().then(text => text.split('foliage instances')[0]);
  const originalGeometry = await geometry();
  const change = async value => { await age.fill(String(value)); await age.dispatchEvent('change'); };
  await change(5);
  await page.waitForFunction(() => document.querySelector('[data-growth-age]')?.getAttribute('data-growth-age') === '5');
  check(await frontier() === original, 'backward dial advanced or rebuilt the frontier');
  const youngGeometry = await geometry();
  check(youngGeometry !== originalGeometry, 'age read did not replace the rendered geometry');
  await page.getByRole('button', { name: 'rebuild at age', exact: true }).click();
  await page.waitForFunction(() => document.querySelector('[data-growth-frontier]')?.getAttribute('data-growth-frontier') === '5');
  check(await geometry() === youngGeometry, 'rebuild disagrees with the earlier read');
  await change(7);
  await page.waitForFunction(() => document.querySelector('[data-growth-frontier]')?.getAttribute('data-growth-frontier') === '7');
  await page.getByLabel('years per second', { exact: true }).fill('2');
  await page.getByRole('button', { name: 'play growth', exact: true }).click();
  await page.waitForFunction(() => Number(document.querySelector('[data-growth-frontier]')?.getAttribute('data-growth-frontier')) >= 7.5);
  await page.getByRole('button', { name: 'pause growth', exact: true }).click();
  const paused = await frontier();
  await page.waitForTimeout(300);
  check(await frontier() === paused, 'pause kept advancing');
  check(await page.getByRole('alert').count() === 0, 'growth exposed a page error');
  console.log('PASS age scrub, rebuild, advance, chosen play rate and pause on one specimen');
} finally { await browser.close(); }
