const {chromium}=await import(process.env.PLAYWRIGHT_MODULE||'playwright');
import {mkdir,writeFile} from 'node:fs/promises';
const out=process.env.FIELD_OUTPUT||'/tmp/fn8-field';await mkdir(out,{recursive:true});
const url=process.env.BROWSER_URL||'http://127.0.0.1:5185';
const browser=await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE||'/usr/bin/chromium',headless:true,args:['--no-sandbox']});
const results={recordedAt:new Date().toISOString(),method:'mesh-free field; 1 warmup, 5 samples; 32^3 closed cells spanning field bounds; queryMs includes packed input transfer and result copying; fieldMs only field index build, foliage required but no surface or render buffers',subjects:[]};
try {
for(const subject of ['ordinary','telperion']) {
const page=await browser.newPage();page.setDefaultTimeout(120000);await page.route(url+'/',r=>r.fulfill({contentType:'text/html',body:'<!doctype html>'}));await page.goto(url);
await page.evaluate(async subject=>{const {TreeEngine,ORDINARY,TELPERION}=await import('/src/browser/core.ts');window.engine=await TreeEngine.create();window.family=subject==='ordinary'?ORDINARY:TELPERION;},subject);
const builds=[];
for(let sample=-1;sample<5;sample++) {
const row=await page.evaluate(()=>{const output=window.engine.build(window.family,{field:true});if(output.surface||output.foliage||output.diagnostics.timings.surfaceMs!==0)throw Error('field built render outputs');const b=output.diagnostics.fieldBounds;const step=Math.max(...b.max.map((x,i)=>x-b.min[i]))/32;const cells=new Float64Array(32**3*4);let i=0;for(let x=0;x<32;x++)for(let y=0;y<32;y++)for(let z=0;z<32;z++)cells.set([b.min[0]+(x+.5)*step,b.min[1]+(y+.5)*step,b.min[2]+(z+.5)*step,step/2],i++*4);const start=performance.now();const flags=output.field.query(cells);const queryMs=performance.now()-start;const wood=flags.filter(f=>f&1).length,foliage=flags.filter(f=>f&2).length;if(!wood||!foliage)throw Error('missing field domain');const memoryBytes=window.engine.memoryBytes;window.engine.release();return{diagnostics:output.diagnostics,queryMs,cells:flags.length,wood,foliage,memoryBytes};});builds.push({warmup:sample<0,...row});console.log(subject,sample,row.diagnostics.timings.buildMs,row.queryMs);
}
results.subjects.push({subject,builds});await writeFile(out+'/results.json',JSON.stringify(results,null,2));await page.evaluate(()=>window.engine.dispose());await page.close();
}
results.finishedAt=new Date().toISOString();await writeFile(out+'/results.json',JSON.stringify(results,null,2));
} finally {await browser.close();}
