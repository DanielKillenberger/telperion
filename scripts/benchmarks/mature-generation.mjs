// Renderer delivery and owned CPU geometry. Completion mode fences the actual
// renderer queue without reading geometry back or changing production APIs.
import { createServer } from 'vite';
import { chromium } from 'playwright';
import { writeFile, readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
const output = process.env.GENERATION_OUTPUT;
if (!output) throw Error('GENERATION_OUTPUT required');
const completed = process.env.GENERATION_COMPLETED === '1';
const server = await createServer({ server: { host: '127.0.0.1', port: 0 } });
await server.listen();
const url = `http://127.0.0.1:${server.httpServer.address().port}`;
const flags = ['--no-sandbox','--enable-unsafe-webgpu','--enable-features=Vulkan','--use-angle=vulkan','--disable-vulkan-surface','--ignore-gpu-blocklist'];
const browser = await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE||'/usr/bin/chromium',headless:false,args:flags});
const result = {date:new Date().toISOString(),browser:browser.version(),flags,viewport:{width:1280,height:720},protocol:completed?'hero frame with renderer queue completion':'default camera frame submission and owned CPU output',wasmSha256:createHash('sha256').update(await readFile('src/browser/render/telperion_render_bg.wasm')).digest('hex'),rows:[],gaps:[...(completed?['display compositor presentation is not measured']:['GPU completion is not exposed: frame timing measures submission only']),'phone hardware','native rendering delivery','peak CPU/GPU memory','attachment setup separate from placement']};
try {
 for (const preset of ['oregon-white-oak','norway-spruce']) for (const seed of [1,7]) {
  const page = await browser.newPage({viewport:result.viewport});
  await page.route(url+'/',r=>r.fulfill({contentType:'text/html',body:'<canvas style="width:1280px;height:720px"></canvas>'}));
  await page.goto(url);
  const navigation = await page.evaluate(() => ({ url: location.href, base: document.baseURI }));
  console.log('navigation', navigation);
  if (!navigation.url.startsWith(url)) throw Error('unexpected navigation: ' + JSON.stringify(navigation));
  const row = await page.evaluate(async ({preset,seed,completed})=>{
   const begin=performance.now();
   const {createRenderer}=await import('/src/browser/render.ts');
   const {TreeEngine,presetById}=await import('/src/browser/core.ts');
   const {familyJson,presetToParams}=await import('/harness/family.ts');
   const family=familyJson({...presetToParams(presetById(preset)),seed});
   let device, hardware, deviceCount=0;
   const originalRequest=GPUAdapter.prototype.requestDevice;
   if(completed) GPUAdapter.prototype.requestDevice=async function(...args) {
    const created=await originalRequest.apply(this,args);
    device=created;hardware=this.info;deviceCount++;
    return created;
   };
   let renderer;
   try { renderer=await createRenderer(document.querySelector('canvas')); }
   finally { if(completed) GPUAdapter.prototype.requestDevice=originalRequest; }
   if(completed && deviceCount!==1) throw Error(`expected one renderer device, captured ${deviceCount}`);
   const initializationMs=performance.now()-begin;
   if(!completed) hardware=(await navigator.gpu.requestAdapter())?.info;
   const samples=[];
   for(let sample=0;sample<6;sample++) {
    const start=performance.now(); const submitted=renderer.setTree(family);
    const setTreeMs=performance.now()-start;
    if(completed) renderer.hero();
    const frameStart=performance.now();renderer.frame();
    const frameSubmitMs=performance.now()-frameStart;
    const totalToFrameSubmissionMs=performance.now()-start;
    const record={sample,coldFirstBuild:sample===0,setTreeMs,frameSubmitMs,totalToFrameSubmissionMs,submitted};
    if(completed) {
     await device.queue.onSubmittedWorkDone();
     record.totalToCompletedFrameMs=performance.now()-start;
     record.frameAndCompletionMs=performance.now()-frameStart;
    }
    samples.push(record);
   }
   renderer.dispose();
   let engineInitializationMs=null;
   const cpu=[];
   if(!completed) {
    const engineStart=performance.now();const engine=await TreeEngine.create();
    engineInitializationMs=performance.now()-engineStart;
    for(let sample=0;sample<6;sample++) {
     const built=engine.build(JSON.parse(family),{surface:true,foliage:true});
     cpu.push({sample,diagnostics:built.diagnostics,wasmMemoryBytes:engine.memoryBytes});engine.release();
    }
    engine.dispose();
   }
   return {preset,seed,initializationMs,engineInitializationMs,adapter:hardware?{vendor:hardware.vendor,architecture:hardware.architecture,device:hardware.device,description:hardware.description}:null,samples,cpu};
  },{preset,seed,completed});
  result.rows.push(row);await writeFile(output,JSON.stringify(result,null,2)+'\n');
  console.log(preset,seed,row.samples.map(x=>x.setTreeMs));await page.close();
 }
} catch(error) { result.error=String(error);throw error; }
finally {await writeFile(output,JSON.stringify(result,null,2)+'\n');await browser.close();await server.close();}
