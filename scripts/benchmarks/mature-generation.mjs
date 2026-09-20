// Current renderer delivery and owned CPU geometry; no capture or timing readback.
import { createServer } from 'vite';
import { chromium } from 'playwright';
import { writeFile, readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
const output = process.env.GENERATION_OUTPUT;
if (!output) throw Error('GENERATION_OUTPUT required');
const server = await createServer({ server: { host: '127.0.0.1', port: 0 } });
await server.listen();
const url = `http://127.0.0.1:${server.httpServer.address().port}`;
const flags = ['--no-sandbox','--enable-unsafe-webgpu','--enable-features=Vulkan','--use-angle=vulkan','--disable-vulkan-surface','--ignore-gpu-blocklist'];
const browser = await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE||'/usr/bin/chromium',headless:false,args:flags});
const result = {date:new Date().toISOString(),browser:browser.version(),flags,viewport:{width:1280,height:720},wasmSha256:createHash('sha256').update(await readFile('src/browser/render/telperion_render_bg.wasm')).digest('hex'),rows:[],gaps:['GPU completion is not exposed: frame timing measures submission only','phone hardware','native rendering delivery','peak CPU/GPU memory','attachment setup separate from placement']};
try {
 for (const preset of ['oregon-white-oak','norway-spruce']) for (const seed of [1,7]) {
  const page = await browser.newPage({viewport:result.viewport});
  await page.route(url+'/',r=>r.fulfill({contentType:'text/html',body:'<canvas style="width:1280px;height:720px"></canvas>'}));
  await page.goto(url);
  const row = await page.evaluate(async ({preset,seed})=>{
   const begin=performance.now();
   const {createRenderer}=await import('/src/browser/render.ts');
   const {TreeEngine,presetById}=await import('/src/browser/core.ts');
   const {familyJson,presetToParams}=await import('/harness/family.ts');
   const family=familyJson({...presetToParams(presetById(preset)),seed});
   const renderer=await createRenderer(document.querySelector('canvas'));
   const initializationMs=performance.now()-begin;
   const adapter=await navigator.gpu.requestAdapter();
   const hardware=adapter?.info;
   const samples=[];
   for(let sample=0;sample<6;sample++) {
    const start=performance.now(); const submitted=renderer.setTree(family);
    const setTreeMs=performance.now()-start;
    const frameStart=performance.now();renderer.frame();
    samples.push({sample,coldFirstBuild:sample===0,setTreeMs,frameSubmitMs:performance.now()-frameStart,totalToFrameSubmissionMs:performance.now()-start,submitted});
   }
   renderer.dispose();
   const engineStart=performance.now();const engine=await TreeEngine.create();
   const engineInitializationMs=performance.now()-engineStart;
   const cpu=[];
   for(let sample=0;sample<6;sample++) {
    const built=engine.build(JSON.parse(family),{surface:true,foliage:true});
    cpu.push({sample,diagnostics:built.diagnostics,wasmMemoryBytes:engine.memoryBytes});engine.release();
   }
   engine.dispose();
   return {preset,seed,initializationMs,engineInitializationMs,adapter:hardware?{vendor:hardware.vendor,architecture:hardware.architecture,device:hardware.device,description:hardware.description}:null,samples,cpu};
  },{preset,seed});
  result.rows.push(row);await writeFile(output,JSON.stringify(result,null,2)+'\n');
  console.log(preset,seed,row.samples.map(x=>x.setTreeMs));await page.close();
 }
} catch(error) { result.error=String(error);throw error; }
finally {await writeFile(output,JSON.stringify(result,null,2)+'\n');await browser.close();await server.close();}
