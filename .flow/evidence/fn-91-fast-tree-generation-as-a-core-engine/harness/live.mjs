import { chromium } from 'playwright';
import { writeFile } from 'node:fs/promises';
const browser = await chromium.launch({ executablePath: '/usr/bin/chromium', headless: false,
  args: ['--no-sandbox','--enable-unsafe-webgpu','--enable-features=Vulkan','--use-angle=vulkan','--disable-vulkan-surface','--ignore-gpu-blocklist'] });
const page = await browser.newPage({viewport:{width:1440,height:1000}});
const errors=[];
page.on('pageerror',e=>errors.push(String(e)));
await page.route('**/harness/main.tsx*',async route=>{
  const response=await route.fetch();
  const prefix=`
const {WebRenderer: QaRenderer}=await import('/src/browser/render/telperion_render.js');
window.__qa={calls:[],active:0,maxActive:0,cpuCalls:0};
const qaGpu=QaRenderer.prototype.setTreeGpu;
QaRenderer.prototype.setTreeGpu=async function(family){
 const q=window.__qa, row={seed:JSON.parse(family).skeleton.seed,start:performance.now()};
 q.calls.push(row);q.active++;q.maxActive=Math.max(q.maxActive,q.active);
 try {const result=await qaGpu.call(this,family);const v=JSON.parse(result);Object.assign(row,{ms:performance.now()-row.start,backend:v.backend,gpuPositions:v.stages.gpuPositions,foliage:v.foliageInstances});return result;}
 catch(e){row.error=String(e);throw e;}finally{q.active--;}
};
const qaCpu=QaRenderer.prototype.setTree;
QaRenderer.prototype.setTree=function(...args){window.__qa.cpuCalls++;return qaCpu.apply(this,args);};
`;
  await route.fulfill({response,body:prefix+await response.text()});
});
try {
  await page.goto('http://127.0.0.1:5173/?species=oregon-white-oak&seed=1');
  if(!page.url().startsWith('http://127.0.0.1:5173/')) await page.goto('http://127.0.0.1:5173/?species=oregon-white-oak&seed=1');
  console.log('navigation',page.url());
  if(!page.url().startsWith('http://127.0.0.1:5173/')) throw Error('browser did not navigate to renderer');
  await page.waitForFunction(()=>window.__qa!==undefined,null,{timeout:10000});
  await page.waitForFunction(()=>window.__qa?.calls.some(x=>x.backend),null,{timeout:60000});
  await page.locator('#gd-seed').fill('17');
  await page.locator('#gd-seed').fill('18');
  await page.locator('#gd-seed').fill('19');
  await page.waitForFunction(()=>window.__qa?.active===0&&window.__qa.calls.at(-1)?.seed===19&&window.__qa.calls.at(-1)?.backend,null,{timeout:30000});
  await page.getByRole('button',{name:'norway spruce',exact:true}).click();
  await page.waitForFunction(()=>window.__qa?.active===0&&window.__qa.calls.at(-1)?.foliage>1000000,null,{timeout:30000});
  await page.getByRole('button',{name:'oregon white oak',exact:true}).click();
  await page.waitForFunction(()=>window.__qa?.active===0&&window.__qa.calls.at(-1)?.foliage<1000000,null,{timeout:30000});
  await page.getByRole('button',{name:'reframe',exact:true}).click();
  await page.evaluate(()=>new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r))));
  await page.screenshot({path:'.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/harness/live.png'});
  const result=await page.evaluate(()=>({qa:window.__qa,alerts:[...document.querySelectorAll('[role=alert]')].map(x=>x.textContent),liveDevices:window.telperionLiveDevices,body:document.body.innerText.slice(-1800)}));
  result.errors=errors;
  await writeFile('.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/harness/live.json',JSON.stringify(result,null,2));
  if(result.qa.maxActive!==1||result.qa.cpuCalls!==0||result.alerts.length||errors.length||result.qa.calls.some(c=>c.error)||result.liveDevices!==1)throw Error(JSON.stringify(result));
  console.log(JSON.stringify(result,null,2));
}catch(error){console.log('FAIL',String(error),errors,await page.evaluate(()=>({url:location.href,qa:window.__qa,body:document.body.innerText.slice(0,1800)})));throw error;}
finally{await browser.close();}
