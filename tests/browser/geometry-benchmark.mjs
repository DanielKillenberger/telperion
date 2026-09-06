import { readFile, writeFile, mkdir, realpath, copyFile, appendFile, rename } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { execFileSync, spawn } from 'node:child_process';
import { resolve, dirname, relative, isAbsolute, basename, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { platform, release } from 'node:os';
import { isDeepStrictEqual } from 'node:util';
import { analyzeGaps, convergence, rasterizeHull } from './geometry-benchmark-diagnostics.mjs';
const root=resolve(dirname(fileURLToPath(import.meta.url)),'../..');
export const sha=bytes=>createHash('sha256').update(bytes).digest('hex');
const encoded=value=>JSON.stringify(value,null,2)+'\n';
export function validateCaptureRules(protocol,frozen) {
  for(const key of ['render','required_views'])if(!isDeepStrictEqual(protocol[key],frozen[key]))throw Error('invalid-manifest: unsupported capture rules');
}
const json=async path=>JSON.parse(await readFile(path,'utf8'));
const save=async(path,value)=>{const temp=path+'.writing';await writeFile(temp,encoded(value),{flush:true});await rename(temp,path);};
export async function contained(base,name) {
  if(isAbsolute(name)||name.split(/[\\/]/).includes('..'))throw Error('path-escape');
  const baseReal=await realpath(base),path=resolve(baseReal,name);
  let existing=path;while(true){try{existing=await realpath(existing);break;}catch(e){if(e.code!=='ENOENT')throw e;const p=dirname(existing);if(p===existing)throw e;existing=p;}}
  if(relative(baseReal,existing).startsWith('..')||isAbsolute(relative(baseReal,existing)))throw Error('path-escape');
  return path;
}
export async function artifact(base,name,media_type) {
  const bytes=await readFile(await contained(base,name));if(!bytes.length)throw Error('missing-asset: empty '+name);
  return {path:name,bytes:bytes.length,sha256:sha(bytes),media_type};
}
export async function verifyArtifacts(base,artifacts) {
  if(!artifacts.length)throw Error('missing-asset: empty artifacts');
  for(const a of artifacts){const actual=await artifact(base,a.path,a.media_type);if(actual.bytes!==a.bytes||actual.sha256!==a.sha256)throw Error('missing-asset: hash/size mismatch '+a.path);}
}
export async function verifyCaptureArtifacts(base,record) {
  for(const channel of ['beauty','coverage'])for(const sample of ['native','64','128'])for(const extension of ['png','f32']) {
    const suffix='/'+channel+'-'+sample+'.'+extension;
    if(record.artifacts.filter(a=>a.path.endsWith(suffix)).length!==1)throw Error('missing-asset: required '+suffix);
  }
  if(!record.camera||!record.selected_target||!record.geometry_sha256||!record.conditions_sha256)throw Error('source-mismatch: missing capture identity');
  await verifyArtifacts(base,record.artifacts);
}
export function verifyConditionsIdentity(expected,provenance,conditions) {
  for(const key of ['protocol_sha256','references_sha256','manifest_sha256','conditions_sha256'])if(!expected[key]||expected[key]!==provenance[key])throw Error('roi-mismatch: '+key);
  if(!conditions.source_sha256||provenance.source?.sha256!==conditions.source_sha256)throw Error('source-mismatch: baseline conditions');
}
const git=(...args)=>execFileSync('git',args,{cwd:root,encoding:'utf8'}).trim();
async function identity(paths) {
  const files=[];for(const path of [...new Set(paths)].sort())files.push({path,sha256:sha(await readFile(await contained(root,path)))});
  return {commit:git('rev-parse','HEAD'),dirty:!!git('status','--porcelain','--',...paths),files,sha256:sha(files.map(f=>f.path+'\0'+f.sha256+'\n').join(''))};
}
export function terminalProcessFailure(result) {
  if(result.expired)return {capture_status:'fail',reason:'timeout'};
  const detail=result.error??result.text?.slice(-1000)??'missing subprocess result';
  const hardware=/browserType.launch|Failed to launch|Executable.*exist|hardware-unavailable|WebGL context/.test(detail);
  return {capture_status:hardware?'unavailable':'fail',reason:(hardware?'hardware-unavailable: ':'interrupted: ')+detail};
}
function runChild(argv,timeout=300000) {
  return new Promise(done=>{
    const child=spawn(process.execPath,argv,{cwd:root,stdio:['ignore','pipe','pipe'],detached:true});let text='',expired=false,cleanupTimer;
    child.stdout.on('data',s=>{text+=s;process.stdout.write(s);});child.stderr.on('data',s=>{text+=s;});
    const timer=setTimeout(()=>{expired=true;child.kill('SIGTERM');cleanupTimer=setTimeout(()=>{try{process.kill(-child.pid,'SIGKILL');}catch{}},5000);},timeout);
    child.on('error',e=>{clearTimeout(timer);clearTimeout(cleanupTimer);done({code:-1,expired,error:String(e),text});});child.on('close',code=>{clearTimeout(timer);clearTimeout(cleanupTimer);done({code,expired,text});});
  });
}
async function preflight(protocolPath,referencesPath) {
  const script=`import importlib.util,json,sys\np='crates/telperion-core/examples/geometry_benchmark/runner.py'\ns=importlib.util.spec_from_file_location('fn19native',p);m=importlib.util.module_from_spec(s);s.loader.exec_module(m)\np=m.read(sys.argv[1]);r=m.read(sys.argv[2]);m.manifest(p,r)\nprint(json.dumps([m.admission(x,p,r,sys.argv[3]) for x in p['species']]))`;
  return JSON.parse(execFileSync('python3',['-c',script,protocolPath,referencesPath,join(root,'target/release/examples/geometry_benchmark')],{cwd:root,encoding:'utf8',timeout:60000,env:{...process.env,PYTHONDONTWRITEBYTECODE:'1'}}));
}
function captureRecord(run,c,view,azimuth) {
  return {schema_version:1,run_id:run.run_id,case_id:c.id,view,azimuth_deg:azimuth,conditions_sha256:run.conditions_sha256,capture_status:'pending',visual_status:'unassessed',assessment:'unassessed',expert_status:'unassessed',owner_feedback:null,camera:null,selected_target:null,geometry_sha256:null,artifacts:[],convergence:{status:'unavailable',samples:[],beauty_rmse:null,coverage_tile_max:null,reason:'pending'},reason:'pending'};
}
async function storeArray(page,out,prefix,data,channels) {
  const bytes=Buffer.alloc(data.length*4);for(let i=0;i<data.length;i++)bytes.writeFloatLE(data[i],i*4);
  await writeFile(await contained(out,prefix+'.f32'),bytes);
  const rgba=Buffer.alloc(1600*1000*4);
  for(let i=0;i<1600*1000;i++){
    for(let c=0;c<3;c++){const x=data[i*channels+(channels===1?0:c)],s=x<=.0031308?12.92*x:1.055*x**(1/2.4)-.055;rgba[i*4+c]=Math.round(Math.max(0,Math.min(1,s))*255);}rgba[i*4+3]=255;
  }
  const png=await page.evaluate(b=>{const a=Uint8ClampedArray.from(atob(b),c=>c.charCodeAt(0)),canvas=document.createElement('canvas');canvas.width=1600;canvas.height=1000;canvas.getContext('2d').putImageData(new ImageData(a,1600,1000),0,0);return canvas.toDataURL('image/png').split(',')[1];},rgba.toString('base64'));
  const pngBytes=Buffer.from(png,'base64');await writeFile(await contained(out,prefix+'.png'),pngBytes);
  // The browser encodes and decodes the preview; header-only files cannot pass.
  const dimensions=await page.evaluate(async b=>{const image=new Image();image.src='data:image/png;base64,'+b;await image.decode();return [image.naturalWidth,image.naturalHeight];},png);
  if(dimensions[0]!==1600||dimensions[1]!==1000)throw Error('missing-asset: decoded PNG dimensions');
  return [await artifact(out,prefix+'.f32','application/octet-stream'),await artifact(out,prefix+'.png','image/png')];
}
async function worker(jobPath) {
  const job=await json(jobPath),{chromium}=await import(process.env.PLAYWRIGHT_MODULE?pathToFileURL(process.env.PLAYWRIGHT_MODULE).href:'playwright');
  const browser=await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE,headless:true,args:['--no-sandbox']});
  const terminate=()=>browser.close().finally(()=>process.exit(1));process.once('SIGTERM',terminate);
  const out=job.out,records=job.records,conditions=[];
  const checkpoint=()=>save(job.result,{records,conditions});
  try {
    const page=await browser.newPage({viewport:{width:1600,height:1000},deviceScaleFactor:1});page.setDefaultTimeout(300000);
    const url=process.env.BROWSER_URL??'http://127.0.0.1:5199';
    await page.route(url+'/',r=>r.fulfill({contentType:'text/html',body:'<!doctype html><body style="margin:0"><canvas></canvas>'}));await page.goto(url+'/');
    const meta=await page.evaluate(async job=>{
      for(const file of job.served_files){const url='/'+file.path+'?raw',response=await (await fetch(url)).text();const source=response.startsWith('export default ')?(await import(url)).default:response;const hash=[...new Uint8Array(await crypto.subtle.digest('SHA-256',new TextEncoder().encode(source)))].map(x=>x.toString(16).padStart(2,'0')).join('');if(hash!==file.sha256)throw Error('source-mismatch: served '+file.path);}
      const {createRig}=await import('/tests/browser/geometry-benchmark-rig.mjs');window.rig=await createRig(job.parameters,job.species);return window.rig.metadata;},job);
    if(meta.wasm_sha256!==job.wasm_sha256)throw Error('source-mismatch: served Wasm differs from recorded binary');
    const geometry=sha(encoded(meta.hashes));
    await save(join(out,job.case_dir,'geometry.json'),{...meta,browser:browser.version(),parameters:job.parameters});
    const geometryArtifact=await artifact(out,job.case_dir+'/geometry.json','application/json');
    for(const record of records){
      record.capture_status='started';record.reason='capture in progress';await checkpoint();
      try {
        const fixed=job.fixed?.entries.find(e=>e.case_id===record.case_id&&e.view===record.view&&e.azimuth_deg===record.azimuth_deg);
        if(!job.prepare&&!fixed)throw Error('roi-mismatch: missing frozen conditions');
        const prepared=await page.evaluate(({view,azimuth,fixed})=>window.rig.prepare(view,azimuth,fixed),{view:record.view,azimuth:record.azimuth_deg,fixed});
        if(fixed&&!isDeepStrictEqual(prepared.camera,fixed.camera))throw Error('roi-mismatch: changed capture camera');
        const dir=job.case_dir+'/'+record.view+'-'+record.azimuth_deg;await mkdir(await contained(out,dir),{recursive:true});
        record.camera=prepared.camera;record.selected_target=prepared.target;record.geometry_sha256=geometry;record.artifacts=[geometryArtifact];
        let roi=null;
        if(record.view==='whole'){
          const projected=rasterizeHull(prepared.polygon,1600,1000);
          if(fixed){
            const rasterPath=await contained(job.conditions_root,fixed.roi.raster.path);const bytes=await readFile(rasterPath);
            if(bytes.length!==1600000||sha(bytes)!==fixed.roi.raster.sha256)throw Error('roi-mismatch: baseline raster');
            roi={raster:new Uint8Array(bytes),polygon:fixed.roi.polygon_px};
          }else roi=projected;
          await writeFile(await contained(out,dir+'/roi.u8'),roi.raster);
        }
        const entry={case_id:record.case_id,view:record.view,azimuth_deg:record.azimuth_deg,camera:prepared.camera,target:prepared.target,roi:roi?{polygon_px:roi.polygon,raster:await artifact(out,dir+'/roi.u8','application/octet-stream'),origin:meta.normalization.root_m}:null,metres_per_pixel:prepared.metres_per_pixel,leaf_state:prepared.leaf_state,environment:prepared.environment,baseline_geometry_sha256:fixed?.baseline_geometry_sha256??geometry};
        conditions.push(entry);
        await save(await contained(out,dir+'/anatomy.json'),prepared.anatomyEvidence);record.artifacts.push(await artifact(out,dir+'/anatomy.json','application/json'));
        if(job.prepare){record.capture_status='unavailable';record.reason='conditions prepared; image capture not requested';await checkpoint();continue;}
        const data={};
        for(const channel of ['beauty','coverage']){
          const channels=channel==='beauty'?3:1;
          const nativeEncoded=await page.evaluate(({entry,channel})=>{
            const a=window.rig.raw(entry,channel),zero=window.rig.raw(entry,channel,[0,0]);if(a.some((x,i)=>x!==zero[i]))throw Error('zero jitter differs from native');
            const bytes=new Uint8Array(a.buffer);let s='';for(let i=0;i<bytes.length;i+=32768)s+=String.fromCharCode(...bytes.subarray(i,i+32768));return btoa(s);
          },{entry,channel});
          const decode=s=>{const b=Buffer.from(s,'base64'),a=new Float32Array(b.length/4);for(let i=0;i<a.length;i++)a[i]=b.readFloatLE(i*4);return a;};
          const native=decode(nativeEncoded);record.artifacts.push(...await storeArray(page,out,dir+'/'+channel+'-native',native,channels));await checkpoint();
          await page.evaluate(()=>{window.integration={sum:null,count:0};});
          for(let stop=16;stop<=128;stop+=16){
            await page.evaluate(({entry,channel,stop})=>{
              const halton=(i,b)=>{let f=1,v=0;while(i>0){f/=b;v+=f*(i%b);i=Math.floor(i/b);}return v;};
              const s=window.integration;while(s.count<stop){const i=++s.count,a=window.rig.raw(entry,channel,[halton(i,2)-.5,halton(i,3)-.5]);s.sum??=new Float64Array(a.length);for(let j=0;j<a.length;j++)s.sum[j]+=a[j];}
            },{entry,channel,stop});
            if(stop===64||stop===128){
              const raw=await page.evaluate(()=>{const s=window.integration,a=Float32Array.from(s.sum,x=>x/s.count),b=new Uint8Array(a.buffer);let text='';for(let i=0;i<b.length;i+=32768)text+=String.fromCharCode(...b.subarray(i,i+32768));return btoa(text);});
              data[channel+stop]=decode(raw);record.artifacts.push(...await storeArray(page,out,dir+'/'+channel+'-'+stop,data[channel+stop],channels));await checkpoint();
            }
            console.log(record.case_id,record.view,record.azimuth_deg,channel,stop+'/128');
          }
        }
        record.convergence=convergence(data.beauty64,data.beauty128,data.coverage64,data.coverage128,1600,1000);
        await save(await contained(out,dir+'/sampling.json'),{native_status:'complete',reference64_status:'complete',reference128_status:'complete',zero_jitter_equivalence:true,convergence:record.convergence,visual_status:'unassessed',biological_assessment:'unassessed'});
        record.artifacts.push(await artifact(out,dir+'/sampling.json','application/json'));
        if(record.convergence.status!=='measured')throw Error('unconverged');
        if(roi){
          const gaps=analyzeGaps({coverage:data.coverage128,roi:roi.raster,width:1600,height:1000,metresPerPixel:entry.metres_per_pixel});
          await save(await contained(out,dir+'/gaps.json'),gaps);record.artifacts.push(await artifact(out,dir+'/gaps.json','application/json'),entry.roi.raster);
        }
        await verifyCaptureArtifacts(out,record);record.capture_status='pass';record.reason=record.selected_target.reason;
      }catch(error){record.capture_status=String(error).includes('hardware-unavailable')?'unavailable':'fail';record.reason=String(error);}
      await checkpoint();
    }
  }finally{await checkpoint();await browser.close();process.removeListener('SIGTERM',terminate);}
}
async function main(args) {
  if(args.includes('--help')){console.log(`Frozen botanical geometry captures (run from repository root).
--protocol FILE      Default .flow/evidence/fn19/protocol.json
--references FILE    Default .flow/evidence/fn19/references.json
--output NEW_DIR     Required; never overwrite an existing run
--prepare            Prepare baseline conditions and ROI without image capture
--conditions FILE    Replay exact prepared conditions; required unless --prepare
--case ID            Partial case selection (all unselected cases remain pending)
--view VIEW          Partial view selection (unselected views remain pending)
--worker JOB         Internal bounded case subprocess
Build: npm ci && npm run wasm:build && cargo build --release -p telperion-core --example geometry_benchmark
Start isolated Vite: npx vite --host 127.0.0.1 --port 5199
BROWSER_URL defaults to http://127.0.0.1:5199; CHROMIUM_EXECUTABLE and PLAYWRIGHT_MODULE supported.
Native and 64/128-sample images are always 1600x1000 DPR1; no reduced fidelity mode.
Prepare on the baseline first, then share conditions.json and its hashed ROI artifacts.
Source/tool/Wasm hashes and every requested case/view are preserved. Exit 1 means
partial, failed or unavailable required capture. Visual/botanical judgments stay unassessed.`);return;}
  const values=new Set(['--protocol','--references','--output','--conditions','--case','--view','--worker']);
  for(let i=0;i<args.length;i++){if(values.has(args[i])){if(!args[++i]||args[i].startsWith('--'))throw Error('missing option');}else if(args[i]!=='--prepare')throw Error('unknown option '+args[i]);}
  const option=name=>{const i=args.indexOf(name);return i<0?null:args[i+1];};
  if(option('--worker'))return worker(option('--worker'));
  if(!option('--output')||(!args.includes('--prepare')&&!option('--conditions'))||args.includes('--prepare')&&option('--conditions'))throw Error('require new output and either --prepare or --conditions');
  const protocolPath=resolve(option('--protocol')??'.flow/evidence/fn19/protocol.json'),referencePath=resolve(option('--references')??'.flow/evidence/fn19/references.json');
  const protocol=await json(protocolPath),admissions=await preflight(protocolPath,referencePath);
  const frozen=await json(join(root,'.flow/evidence/fn19/protocol.json'));
  validateCaptureRules(protocol,frozen);
  if(option('--case')&&!protocol.cases.some(c=>c.id===option('--case')))throw Error('invalid-manifest: unknown case');
  if(option('--view')&&!protocol.required_views.some(v=>v.id===option('--view')))throw Error('invalid-manifest: unknown view');
  const sourcePaths=git('ls-files','--cached','--others','--exclude-standard','--','crates/telperion-core/src','crates/telperion-wasm/src','crates/telperion-core/Cargo.toml','crates/telperion-wasm/Cargo.toml','Cargo.toml','Cargo.lock','src/browser/core.ts','src/browser/presets.generated.ts','src/browser/three.ts').split('\n');
  const toolPaths=git('ls-files','--cached','--others','--exclude-standard','--','tests/browser/geometry-benchmark*.mjs','harness/stage.ts','package-lock.json','vite.config.ts','crates/telperion-core/examples/geometry_benchmark/runner.py','crates/telperion-core/examples/geometry_benchmark.rs','crates/telperion-core/examples/geometry_benchmark/params.rs').split('\n');
  const source=await identity(sourcePaths),tool=await identity(toolPaths);
  if(args.includes('--prepare'))for(const file of protocol.baseline_source.files){const current=source.files.find(f=>f.path===file.path);if(current&&current.sha256!==file.sha256)throw Error('source-mismatch: prepare requires baseline generator '+file.path);}
  const output=resolve(option('--output'));await mkdir(dirname(output),{recursive:true});const out=join(await realpath(dirname(output)),basename(output));await mkdir(out);
  const cases=protocol.cases,parameters={};for(const c of cases){const p=structuredClone(protocol.species.find(s=>s.id===c.species_id).parameters);p.skeleton.seed=c.seed;parameters[c.id]=p;}
  await save(join(out,'manifest.json'),cases);await copyFile(protocolPath,join(out,'protocol.json'));await copyFile(referencePath,join(out,'references.json'));await save(join(out,'admission.json'),admissions);
  await copyFile(join(root,'src/browser/telperion.wasm'),join(out,'generator.wasm'));
  await copyFile(join(root,'target/release/examples/geometry_benchmark'),join(out,'generator-support'));
  let fixed=null,conditionsHash=null,conditionsRoot=null;
  if(option('--conditions')){
    const path=resolve(option('--conditions'));fixed=await json(path);conditionsRoot=dirname(path);conditionsHash=sha(await readFile(path));
    if(fixed.schema_version!==1||fixed.benchmark_id!==protocol.benchmark_id||!fixed.entries?.length)throw Error('roi-mismatch: conditions identity');
    const provenance=await json(join(conditionsRoot,'run.json'));
    const keys=fixed.entries.map(e=>[e.case_id,e.view,e.azimuth_deg].join('/'));if(new Set(keys).size!==keys.length)throw Error('roi-mismatch: duplicate conditions');
    const validateScript="import importlib.util,json,sys; s=importlib.util.spec_from_file_location('r','crates/telperion-core/examples/geometry_benchmark/runner.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m);p=m.read(sys.argv[1]);c=m.read(sys.argv[2]);[m.check('conditions',e,p) for e in c['entries']]";
    execFileSync('python3',['-c',validateScript,protocolPath,path],{cwd:root,env:{...process.env,PYTHONDONTWRITEBYTECODE:'1'}});
    verifyConditionsIdentity({protocol_sha256:sha(await readFile(protocolPath)),references_sha256:sha(await readFile(referencePath)),manifest_sha256:sha(await readFile(join(out,'manifest.json'))),conditions_sha256:conditionsHash},provenance,fixed);
    await copyFile(path,join(out,'conditions.json'));
    for(const e of fixed.entries){
      if(!cases.some(c=>c.id===e.case_id)||!protocol.required_views.some(v=>v.id===e.view&&v.azimuth_deg.includes(e.azimuth_deg)))throw Error('roi-mismatch: unknown condition');
      if(e.roi){await verifyArtifacts(conditionsRoot,[e.roi.raster]);const dest=await contained(out,e.roi.raster.path);await mkdir(dirname(dest),{recursive:true});await copyFile(await contained(conditionsRoot,e.roi.raster.path),dest);}
    }
  }
  const run={schema_version:1,benchmark_id:protocol.benchmark_id,run_id:basename(out),created_at:new Date().toISOString(),protocol_sha256:sha(await readFile(protocolPath)),references_sha256:sha(await readFile(referencePath)),source,tool,binaries:[await artifact(out,'generator.wasm','application/wasm'),await artifact(out,'generator-support','application/octet-stream')],manifest_sha256:sha(await readFile(join(out,'manifest.json'))),conditions_sha256:conditionsHash,parameters,cases,machine:{node:process.version,os:platform()+' '+release(),lifecycle:'cold browser per case; readback included',exclusive_window:false},status:'started',partial:true,reason:'pending capture; incomplete cases remain interrupted'};
  const captures=cases.flatMap(c=>protocol.required_views.flatMap(v=>v.azimuth_deg.map(a=>captureRecord(run,c,v.id,a))));
  await save(join(out,'run.json'),run);await save(join(out,'run-started.json'),run);await save(join(out,'captures.json'),captures);
  const event=(c,event,reason=null,costs=[])=>appendFile(join(out,'measurements.jsonl'),JSON.stringify({schema_version:1,run_id:run.run_id,case_id:c.id,event,numeric_status:'unassessed',metrics:{},costs,artifacts:[],reason})+'\n',{flush:true});
  for(const c of cases)await event(c,'pending','visual collection pending; structural metrics collected separately');
  const conditionEntries=[];
  for(let index=0;index<cases.length;index++){
    const c=cases[index];if(option('--case')&&c.id!==option('--case'))continue;
    await event(c,'started','visual case started');
    const records=captures.filter(r=>r.case_id===c.id&&(!option('--view')||r.view===option('--view')));
    const admitted=admissions.find(a=>a.species_id===c.species_id);
    if(admitted.status!=='admitted'){for(const r of records){r.capture_status='unavailable';r.reason=admitted.status+': '+admitted.reasons.join('; ');}await save(join(out,'captures.json'),captures);await event(c,'unavailable',admitted.status);continue;}
    const caseDir='cases/'+index;await mkdir(join(out,caseDir),{recursive:true});
    const servedFiles=[...source.files,...tool.files].filter(f=>['src/browser/core.ts','src/browser/presets.generated.ts','src/browser/three.ts','harness/stage.ts','tests/browser/geometry-benchmark-rig.mjs','tests/browser/geometry-benchmark-diagnostics.mjs'].includes(f.path));
    const job={out,served_files:servedFiles,wasm_sha256:run.binaries[0].sha256,case_dir:caseDir,parameters:parameters[c.id],species:protocol.species.find(s=>s.id===c.species_id),records,prepare:args.includes('--prepare'),fixed,conditions_root:conditionsRoot,result:join(out,caseDir,'checkpoint.json')};
    const jobPath=join(out,caseDir,'job.json');await save(jobPath,job);
    for(const r of records){r.capture_status='started';r.reason='bounded case subprocess started';}await save(join(out,'captures.json'),captures);
    const processResult=await runChild([fileURLToPath(import.meta.url),'--worker',jobPath],protocol.render.timeout_ms_per_case);await save(join(out,caseDir,'process.json'),processResult);
    let checkpoint;try{checkpoint=await json(job.result);}catch{}
    for(const r of records){const updated=checkpoint?.records.find(x=>x.view===r.view&&x.azimuth_deg===r.azimuth_deg);if(updated)Object.assign(r,updated);if(['pending','started'].includes(r.capture_status)){Object.assign(r,terminalProcessFailure(processResult));}}
    if(checkpoint)conditionEntries.push(...checkpoint.conditions);
    let meta;try{meta=await json(join(out,caseDir,'geometry.json'));}catch{}
    const costs=['generation','capture-preparation','cpu-rss','wasm-capacity','gpu-allocation','gpu-time'].map(domain=>{
      const value=({'generation':meta?.generation_ms,'capture-preparation':meta?.preparation_ms,'wasm-capacity':meta?.wasm_bytes})[domain]??null;
      return {domain,status:value===null?'unavailable':'measured',value,unit:['generation','capture-preparation','gpu-time'].includes(domain)?'ms':'bytes',conditions:'cold Wasm/browser; nonexclusive resource window; capture includes synchronized readback',reason:value===null?'not measured in this capture domain':null};
    });
    await event(c,records.every(r=>r.capture_status==='pass')?'completed':'unavailable',records.every(r=>r.capture_status==='pass')?null:'partial or failed visual collection; inspect captures.json',costs);
    await save(join(out,'captures.json'),captures);
    if(job.prepare)await save(join(out,'conditions.json'),{schema_version:1,benchmark_id:protocol.benchmark_id,source_sha256:source.sha256,entries:conditionEntries});
  }
  if(args.includes('--prepare')){
    const path=join(out,'conditions.json');if(conditionEntries.length){run.conditions_sha256=sha(await readFile(path));for(const c of captures)c.conditions_sha256=run.conditions_sha256;}
    run.reason='conditions preparation only; no image collection or visual pass';
  }else{
    for(const c of captures)if(c.capture_status==='pass')try{await verifyCaptureArtifacts(out,c);}catch(error){c.capture_status='fail';c.reason=String(error);}
    const complete=captures.every(c=>c.capture_status==='pass');run.status=complete?'complete':'failed';run.partial=!complete;run.reason=complete?null:'required capture missing, failed or unavailable';
  }
  if(args.includes('--prepare')){run.status='failed';run.partial=true;}
  await save(join(out,'captures.json'),captures);await save(join(out,'run.json'),run);console.log(out);
  process.exitCode=args.includes('--prepare')?conditionEntries.length===captures.length?0:1:run.status==='complete'?0:1;
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url))main(process.argv.slice(2)).catch(error=>{console.error(error);process.exitCode=1;});
