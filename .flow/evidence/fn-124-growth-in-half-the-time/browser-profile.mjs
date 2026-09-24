// V8 sampling profile of `setTreeGpu` in the benchmark's browser, one page per
// fixture: one cold call, then PROFILE_CALLS profiled calls. Run from the
// checkout whose built modules are measured; writes PROFILE_OUTPUT-<fixture>.json.
import { createServer } from 'vite';
import { chromium } from 'playwright';
import { writeFile } from 'node:fs/promises';
const output = process.env.PROFILE_OUTPUT;
if (!output) throw Error('PROFILE_OUTPUT required');
const calls = Number(process.env.PROFILE_CALLS || 5);
const fixtures = (process.env.PROFILE_FIXTURES || 'oregon-white-oak:1,oregon-white-oak:7,norway-spruce:1,norway-spruce:7').split(',');
const server = await createServer({ server: { host: '127.0.0.1', port: 0 } });
await server.listen();
const url = `http://127.0.0.1:${server.httpServer.address().port}`;
const flags = ['--no-sandbox','--enable-unsafe-webgpu','--enable-features=Vulkan','--use-angle=vulkan','--disable-vulkan-surface','--ignore-gpu-blocklist'];
const browser = await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE||'/usr/bin/chromium',headless:false,args:flags});
try {
 for (const fixture of fixtures) {
  const [preset, seed] = [fixture.split(':')[0], Number(fixture.split(':')[1])];
  const page = await browser.newPage({viewport:{width:1280,height:720}});
  await page.route(url+'/',r=>r.fulfill({contentType:'text/html',body:'<canvas style="width:1280px;height:720px"></canvas>'}));
  await page.goto(url);
  await page.evaluate(async ({preset,seed})=>{
   const {createRenderer}=await import('/src/browser/render.ts');
   const {presetById}=await import('/src/browser/core.ts');
   const {familyJson,presetToParams}=await import('/harness/family.ts');
   window.family=familyJson({...presetToParams(presetById(preset)),seed});
   window.renderer=await createRenderer(document.querySelector('canvas'));
   await window.renderer.setTreeGpu(window.family);
  },{preset,seed});
  const cdp = await page.context().newCDPSession(page);
  await cdp.send('Profiler.enable');
  await cdp.send('Profiler.setSamplingInterval',{interval:50});
  await cdp.send('Profiler.start');
  const skeleton = await page.evaluate(async (calls)=>{
   const out=[];
   for(let i=0;i<calls;i++) out.push((await window.renderer.setTreeGpu(window.family)).stages.skeletonMs);
   return out;
  },calls);
  const {profile} = await cdp.send('Profiler.stop');
  await writeFile(`${output}-${preset}-${seed}.json`, JSON.stringify({preset,seed,skeletonMs:skeleton,profile}));
  console.log(preset, seed, skeleton);
  await page.close();
 }
} finally { await browser.close(); await server.close(); }
