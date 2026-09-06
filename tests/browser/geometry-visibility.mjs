import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {execFileSync} from 'node:child_process';
import {resolve,dirname,join} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {isDeepStrictEqual} from 'node:util';
import {sha,artifact} from './geometry-benchmark.mjs';
const root=resolve(dirname(fileURLToPath(import.meta.url)),'../..');
const json=async p=>JSON.parse(await readFile(p,'utf8'));
const save=(p,v)=>writeFile(p,JSON.stringify(v,null,2)+'\n');
export function validateVisibilityMapping(record,original) {
  if(record.version!=='fn19-visibility-v2'||!['fork','attached-shoot'].includes(record.view))throw Error('unsupported diagnostic');
  if(!isDeepStrictEqual(record.geometry_hashes,original.hashes)||record.wood!=='full-connected-original')throw Error('source-mismatch: geometry');
  const units=record.retained_unit_indices;
  if(!Array.isArray(units)||new Set(units).size!==units.length||units.some(i=>!Number.isInteger(i)||i<0||i>=record.original_unit_count))throw Error('invalid attachment mapping');
  if(record.view==='attached-shoot'&&!units.length||record.view==='fork'&&units.length)throw Error('invalid diagnostic filter');
  if(!(record.visible_probe_count>0&&record.visible_probe_count<=record.probe_count))throw Error('unassessed: hidden woody anatomy');
}
async function main() {
  const [baselineArg,outputArg,caseArg]=process.argv.slice(2);
  if(!baselineArg||!outputArg)throw Error('usage: node tests/browser/geometry-visibility.mjs V1_VISUAL_RUN NEW_OUTPUT [CASE_INDEX]');
  const baseline=resolve(baselineArg),out=resolve(outputArg),original=await json(join(baseline,'run.json'));
  const protocol=await json(join(root,'.flow/evidence/fn19/protocol.json'));
  if(sha(await readFile(join(root,'.flow/evidence/fn19/protocol.json')))!==original.protocol_sha256)throw Error('protocol mismatch');
  const pinned=await json(join(root,'.flow/evidence/fn19/final/visual.json'));
  if(!isDeepStrictEqual(original,pinned.run))throw Error('source-mismatch: baseline differs from pinned mature receipt');
  await mkdir(out,{recursive:false});
  const toolPaths=[...original.tool.files.map(f=>f.path),'tests/browser/geometry-visibility.mjs','.flow/evidence/fn19/visibility-v2/protocol.json'];
  const files=[];for(const path of [...new Set([...original.source.files.map(f=>f.path),...toolPaths])].sort())files.push({path,sha256:sha(await readFile(join(root,path)))});
  for(const expected of original.source.files)if(files.find(f=>f.path===expected.path)?.sha256!==expected.sha256)throw Error('source-mismatch: '+expected.path);
  const run={version:'fn19-visibility-v2',created_at:new Date().toISOString(),baseline:{path:baseline,run_sha256:sha(await readFile(join(baseline,'run.json'))),protocol_sha256:original.protocol_sha256,source:original.source},tool:{commit:execFileSync('git',['rev-parse','HEAD'],{cwd:root,encoding:'utf8'}).trim(),dirty:!!execFileSync('git',['status','--porcelain','--',...files.map(f=>f.path)],{cwd:root,encoding:'utf8'}).trim(),files,sha256:sha(JSON.stringify(files))},render:{width:1600,height:1000,samples:[64,128],channels:['neutral-beauty'],density_comparison:false},records:[],status:'started'};
  const cases=original.cases.map((c,index)=>({...c,index})).filter(c=>caseArg===undefined||c.index===Number(caseArg));
  if(!cases.length)throw Error('empty case selection');
  for(const c of cases)for(const view of ['fork','attached-shoot'])run.records.push({case_id:c.id,case_index:c.index,view,status:'interrupted',assessment:'unassessed',reason:'pending',artifacts:[]});
  await save(join(out,'run.json'),run);
  const {chromium}=await import(process.env.PLAYWRIGHT_MODULE?pathToFileURL(process.env.PLAYWRIGHT_MODULE).href:'playwright');
  for(const c of cases){
    let browser,caseTimer;
    const records=run.records.filter(r=>r.case_id===c.id);
    try {
      browser=await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE,headless:true,args:['--no-sandbox']});
      caseTimer=setTimeout(()=>{for(const r of records)if(r.status!=='captured'){r.status='fail';r.reason='timeout: five-minute case cap';}browser.close().catch(()=>{});},300000);
      const page=await browser.newPage({viewport:{width:1600,height:1000},deviceScaleFactor:1});page.setDefaultTimeout(300000);
      const url=process.env.BROWSER_URL??'http://127.0.0.1:5199';
      await page.route(url+'/',r=>r.fulfill({contentType:'text/html',body:'<!doctype html><body style="margin:0"><canvas></canvas>'}));await page.goto(url+'/');
      const meta=await page.evaluate(async ({parameters,species,files})=>{
        for(const f of files.filter(f=>['src/browser/core.ts','src/browser/three.ts','src/browser/presets.generated.ts','tests/browser/geometry-benchmark-rig.mjs','tests/browser/geometry-benchmark-diagnostics.mjs','harness/stage.ts'].includes(f.path))){const url='/'+f.path+'?raw',response=await (await fetch(url)).text(),source=response.startsWith('export default ')?(await import(url)).default:response;const hash=[...new Uint8Array(await crypto.subtle.digest('SHA-256',new TextEncoder().encode(source)))].map(x=>x.toString(16).padStart(2,'0')).join('');if(hash!==f.sha256)throw Error('served source mismatch '+f.path);}
        const {createRig}=await import('/tests/browser/geometry-benchmark-rig.mjs');window.rig=await createRig(parameters,species);return window.rig.metadata;
      },{parameters:original.parameters[c.id],species:protocol.species.find(s=>s.id===c.species_id),files});
      const priorPath=join(baseline,'cases',String(c.index),'geometry.json'),prior=await json(priorPath);
      const priorArtifact=pinned.views.find(v=>v.case_id===c.id).artifacts.find(a=>a.path.endsWith('/geometry.json'));
      if(!priorArtifact||sha(await readFile(priorPath))!==priorArtifact.sha256)throw Error('source-mismatch: original geometry receipt');
      if(!isDeepStrictEqual(meta.hashes,prior.hashes)||meta.wasm_sha256!==prior.wasm_sha256)throw Error('source-mismatch: original generated arrays/Wasm');
      for(const record of records){
        try {
          const condition=await page.evaluate(view=>{window.condition=window.rig.prepareVisibility(view);return window.condition;},record.view);
          validateVisibilityMapping(condition.diagnostic,prior);
          record.condition=condition;record.geometry=meta;record.browser=browser.version();record.parameters=original.parameters[c.id];
          const prefix=c.index+'-'+record.view;
          await page.evaluate(()=>{window.sum=new Float64Array(1600*1000*3);window.samples=0;});
          for(let stop=8;stop<=128;stop+=8){
            await page.evaluate(stop=>{function radical(i,b){let f=1,r=0;for(;i;i=Math.floor(i/b)){f/=b;r+=f*(i%b);}return r;}for(;window.samples<stop;window.samples++){const index=window.samples+1,a=window.rig.raw(window.condition,'beauty',[radical(index,2)-.5,radical(index,3)-.5]);for(let j=0;j<a.length;j++)window.sum[j]+=a[j];}},stop);
            if(stop===64)await page.evaluate(()=>{window.mean64=Float32Array.from(window.sum,x=>x/64);});
            if(stop===128)record.beauty_rmse_64_128=await page.evaluate(()=>{let sum=0;for(let i=0;i<window.sum.length;i++)sum+=(window.sum[i]/128-window.mean64[i])**2;return Math.sqrt(sum/window.sum.length);});
            if(stop===64||stop===128){
              const png=await page.evaluate(()=>{const canvas=document.createElement('canvas');canvas.width=1600;canvas.height=1000;const pixels=new Uint8ClampedArray(1600*1000*4);for(let i=0;i<1600*1000;i++){for(let k=0;k<3;k++){const x=window.sum[i*3+k]/window.samples,s=x<=.0031308?12.92*x:1.055*x**(1/2.4)-.055;pixels[i*4+k]=Math.round(Math.max(0,Math.min(1,s))*255);}pixels[i*4+3]=255;}canvas.getContext('2d').putImageData(new ImageData(pixels,1600,1000),0,0);return canvas.toDataURL('image/png').split(',')[1];});
              const name=prefix+'-'+stop+'.png';await writeFile(join(out,name),Buffer.from(png,'base64'));record.artifacts.push(await artifact(out,name,'image/png'));
            }
          }
          if(!Number.isFinite(record.beauty_rmse_64_128)||record.beauty_rmse_64_128>.005)throw Error('unassessed: beauty convergence exceeds 0.005');
          record.status='captured';record.reason='diagnostic capture complete; direct anatomy inspection pending';
        }catch(e){record.status='fail';record.reason=String(e);}
        await save(join(out,'run.json'),run);console.log(c.id,record.view,record.status,record.condition?.diagnostic.visible_probe_count);
      }
    }catch(e){for(const r of records){r.status='fail';r.reason=String(e);}}
    finally{clearTimeout(caseTimer);await browser?.close();await save(join(out,'run.json'),run);}
  }
  for(const f of files)if(sha(await readFile(join(root,f.path)))!==f.sha256)throw Error('source-mismatch: tool changed during capture '+f.path);
  run.status=run.records.every(r=>r.status==='captured')?'captured':'incomplete';await save(join(out,'run.json'),run);
  process.exitCode=run.status==='captured'?0:1;
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url))main().catch(e=>{console.error(e);process.exitCode=1;});
