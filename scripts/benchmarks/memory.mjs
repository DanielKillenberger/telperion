const {chromium}=await import(process.env.PLAYWRIGHT_MODULE || 'playwright');
import {mkdir,writeFile} from 'node:fs/promises';
const implementation=process.env.IMPLEMENTATION||'rust';
const out=process.env.MEMORY_OUTPUT || '/tmp/fn8-rust-memory';await mkdir(out,{recursive:true});
const browser=await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE||'/usr/bin/chromium',headless:true,args:['--no-sandbox','--ozone-platform=x11','--use-angle=gl','--enable-gpu','--ignore-gpu-blocklist','--disable-software-rasterizer']});
const result={implementation,startedAt:new Date().toISOString(),method:'CDP Runtime.getHeapUsage after explicit HeapProfiler.collectGarbage and rendered frames. Separately labelled V8/backing storage domains; not total browser/process/GPU RAM. Three retained/disposed rebuilds per subject. This is not a latency run.',subjects:[]};
try {
for(const subject of ['telperion','laurelin','comparison']) {
 const page=await browser.newPage({viewport:{width:1600,height:1000},deviceScaleFactor:1});page.setDefaultTimeout(120000);
 const url=process.env.BROWSER_URL || 'http://127.0.0.1:5185';await page.route(url+'/',r=>r.fulfill({contentType:'text/html',body:'<!doctype html><html><body style="margin:0;width:100vw;height:100vh;overflow:hidden"><canvas style="display:block;width:100%;height:100%"></canvas></body></html>'}));await page.goto(url+'/');
 await page.evaluate(async implementation=>{const{createStage}=await import('/harness/stage.ts');const {buildPreset,buildComparison}=await import('/harness/skeleton-view.ts');const{TELPERION,LAURELIN,initializeTreeCore,treeCore}=await import(implementation==='rust'?'/src/browser/core.ts':'/src/presets/two-trees.ts');if(initializeTreeCore)await initializeTreeCore();const THREE=await import('/node_modules/.vite/deps/three.js');window.rig={stage:createStage(document.querySelector('canvas')),buildPreset,buildComparison,TELPERION,LAURELIN,THREE,treeCore};},implementation);
 const cdp=await page.context().newCDPSession(page);
 const sample=async()=>{await page.evaluate(()=>new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r))));await cdp.send('HeapProfiler.collectGarbage');return await cdp.send('Runtime.getHeapUsage');};
 const row={subject,before:await sample(),builds:[]};result.subjects.push(row);
 for(let iteration=0;iteration<3;iteration++) {
  const stats=await page.evaluate(subject=>{const r=window.rig;let stats;r.stage.setTree(clay=>{const b=subject==='comparison'?r.buildComparison([r.TELPERION,r.LAURELIN],clay):r.buildPreset(subject==='telperion'?r.TELPERION:r.LAURELIN,clay);stats=b.stats;return b.tree??b.group;});r.stage.frame(subject==='laurelin'?132:148);return stats;},subject);
  const retained=await sample(); const wasmBytes=await page.evaluate(()=>window.rig.treeCore?.().memoryBytes ?? null);
  await page.evaluate(()=>{const r=window.rig;r.stage.setTree(()=>new r.THREE.Group());});
  const disposed=await sample();row.builds.push({iteration,stats,retained,disposed,wasmBytes});console.log(subject,iteration,JSON.stringify({retained,disposed}));await writeFile(out+'/results.json',JSON.stringify(result,null,2));
 }
 await page.evaluate(()=>{window.rig.stage.dispose();window.rig.treeCore?.().dispose();});row.engineDisposed=await sample();await writeFile(out+'/results.json',JSON.stringify(result,null,2));await page.close();
}
result.finishedAt=new Date().toISOString();await writeFile(out+'/results.json',JSON.stringify(result,null,2));
} finally {await browser.close();}
