const { chromium } = await import(process.env.PLAYWRIGHT_MODULE || 'playwright');
import { mkdir, writeFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
const url=process.env.FN6_URL||'http://127.0.0.1:5173';
const out=process.env.FN6_OUTPUT||'/tmp/fn6-task7-measurements';
await mkdir(out,{recursive:true});
const flags=['--no-sandbox','--ozone-platform=x11','--use-angle=gl','--enable-gpu','--ignore-gpu-blocklist','--disable-software-rasterizer','--disable-gpu-vsync','--disable-frame-rate-limit'];
const browser=await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE || '/usr/bin/chromium',headless:false,args:flags});
const page=await browser.newPage({viewport:{width:1600,height:1000}});
page.setDefaultTimeout(120000);
const errors=[];page.on('pageerror',e=>errors.push(e.message));
await page.route(url+'/',route=>route.fulfill({contentType:'text/html',body:'<!doctype html><html><body style="margin:0;width:100vw;height:100vh;overflow:hidden"><canvas style="display:block;width:100%;height:100%"></canvas></body></html>'}));
await page.goto(url+'/');
const environment=await page.evaluate(async()=>{
 const {createStage,describeSweep,sweepRatios}=await import('/harness/stage.ts');
 const {buildPreset,buildComparison}=await import('/harness/skeleton-view.ts');
 const {TELPERION,LAURELIN}=await import('/src/presets/two-trees.ts');
 const canvas=document.querySelector('canvas');const stage=createStage(canvas);window.rig={stage,describeSweep,sweepRatios,buildPreset,buildComparison,TELPERION,LAURELIN};
 const gl=canvas.getContext('webgl2');const info=gl.getExtension('WEBGL_debug_renderer_info');
 return{renderer:gl.getParameter(info.UNMASKED_RENDERER_WEBGL),vendor:gl.getParameter(info.UNMASKED_VENDOR_WEBGL),timer:!!gl.getExtension('EXT_disjoint_timer_query_webgl2'),rawDpr:devicePixelRatio,canvasCss:[canvas.clientWidth,canvas.clientHeight],screenReported:[screen.width,screen.height],userAgent:navigator.userAgent,logDepth:stage.stats().logarithmicDepthBuffer,defaultRatios:sweepRatios(devicePixelRatio)};
});
if(!environment.renderer.includes('RTX 3080')||!environment.timer)throw new Error('Required RTX3080 GPU timer unavailable: '+JSON.stringify(environment));
const results={recordedAt:new Date().toISOString(),environment:{...environment,browserVersion:await browser.version(),flags,physicalMonitor:JSON.parse(execFileSync('hyprctl',['monitors','-j'],{encoding:'utf8'})).map(({width,height,scale,refreshRate,model})=>({width,height,scale,refreshRate,model})),gpu:execFileSync('nvidia-smi',['--query-gpu=name,driver_version,memory.total','--format=csv,noheader'],{encoding:'utf8'}).trim(),cpu:execFileSync('lscpu',[],{encoding:'utf8'}).split('\n').filter(x=>/Model name:|^CPU\(s\):/.test(x)),node:process.version},method:{cpuWarmups:1,cpuSamples:5,gpuWarmupFrames:8,gpuSamplesPerRatio:20,foliage:true,lighting:'harness neutral sky; lighting check off',camera:'stage.frame() for each subject; same default camera direction and framing margin',timing:'buildMs is full harness builder incl surface normals, leaf placement/cull, instance buffer/bounds; setTreeMs also includes prior tree disposal and scene attachment; GPU samples time complete harness render including room, exclude CPU builds; GPU budget2ms at native1, DPR2separate high-DPI test',vsync:'Chromium disable-gpu-vsync and disable-frame-rate-limit requested; compositor not disabled; GPU query times, never wall-frame fallback'},subjects:[],errors};
await writeFile(out+'/results.json',JSON.stringify(results,null,2));
for(const subject of ['telperion','laurelin','comparison']){
 const builds=[];
 for(let sample=-1;sample<5;sample++){
  const row=await page.evaluate(subject=>{
   const r=window.rig;let stats;const start=performance.now();r.stage.setTree(clay=>{const built=subject==='comparison'?r.buildComparison([r.TELPERION,r.LAURELIN],clay):r.buildPreset(subject==='telperion'?r.TELPERION:r.LAURELIN,clay);stats=built.stats;return built.tree??built.group;});
   const setTreeMs=performance.now()-start;r.stage.frame(subject==='laurelin'?r.LAURELIN.skeleton.envelope.height:r.TELPERION.skeleton.envelope.height);
   return{...stats,setTreeMs};
  },subject);
  builds.push({warmup:sample<0,...row});console.log(subject,'build',sample,row.buildMs.toFixed(1),'ms');
  await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
 }
 const gpu=await page.evaluate(async()=>{const r=window.rig;const sweep=await r.stage.sweep();return{sweep,description:r.describeSweep(sweep),frame:r.stage.stats()};});
 await page.screenshot({path:out+'/'+subject+'.png'});
 const row={subject,builds,gpu};results.subjects.push(row);await writeFile(out+'/results.json',JSON.stringify(results,null,2));console.log(subject,'GPU',gpu.description);
 if(!gpu.sweep.supported||!gpu.sweep.complete)throw new Error('Incomplete GPU evidence');
}
await page.evaluate(()=>window.rig.stage.dispose());
const unavailable=await browser.newPage({viewport:{width:800,height:600}});
await unavailable.route(url+'/',route=>route.fulfill({contentType:'text/html',body:'<!doctype html><canvas style="width:800px;height:600px"></canvas>'}));
await unavailable.addInitScript(()=>{const original=WebGL2RenderingContext.prototype.getExtension;WebGL2RenderingContext.prototype.getExtension=function(name){return name==='EXT_disjoint_timer_query_webgl2'?null:original.call(this,name)};});
await unavailable.goto(url+'/');
results.unavailable=await unavailable.evaluate(async()=>{const{createStage,describeSweep}=await import('/harness/stage.ts');const stage=createStage(document.querySelector('canvas'));const result=await stage.sweep();const description=describeSweep(result);stage.dispose();return{result,description};});
if(results.unavailable.result.supported||results.unavailable.result.points.length||!results.unavailable.description.startsWith('no gpu timing available'))throw new Error('Unavailable timer fallback dishonest');
results.finishedAt=new Date().toISOString();await writeFile(out+'/results.json',JSON.stringify(results,null,2));await browser.close();console.log('DONE',out);
