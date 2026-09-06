import test from 'node:test';
import assert from 'node:assert/strict';
import { compareRecords, sharedSource } from '../../scripts/benchmarks/geometry-compare.mjs';
const source={files:[{path:'core',sha256:'a'}],sha256:'native'};
function fixture(){const run={protocol_sha256:'p',references_sha256:'r',cases:[{id:'oak'}],parameters:{oak:{}},conditions_sha256:null,source,tool:{sha256:'t'},status:'complete',partial:false,machine:{exclusive_window:false}};return {run,rows:{oak:{event:'completed',numeric_status:'pass',metrics:{length:1},costs:[]}},captures:null};}
test('R4 unchanged replay compares measured values, not metadata alone',()=>{const a=fixture(),b=fixture();assert.equal(compareRecords(a,b).status,'comparable');b.rows.oak.metrics.length=2;const r=compareRecords(a,b);assert.equal(r.status,'inconclusive');assert.equal(r.metrics.oak.changed,true);b.run.source={...source,sha256:'candidate'};assert.equal(compareRecords(a,b).status,'comparable');});
test('R4 mismatched protocol, reference, cohort, tool, conditions and interrupted or failed cases never pass',()=>{for(const mutate of [b=>b.run.protocol_sha256='q',b=>b.run.references_sha256='q',b=>b.run.cases=[],b=>b.run.cases.push({id:'oak'}),b=>b.run.tool={sha256:'q'},b=>b.run.conditions_sha256='q',b=>delete b.rows.oak,b=>b.rows.oak.event='started',b=>b.rows.oak.numeric_status='fail',b=>b.run.partial=true]){const a=fixture(),b=fixture();mutate(b);assert.equal(compareRecords(a,b).status,'inconclusive');}});
test('R4 native and browser source domains match shared files without false aggregate mismatch',()=>{assert.doesNotThrow(()=>sharedSource(source,{files:[...source.files,{path:'binding',sha256:'b'}],sha256:'visual'}));assert.throws(()=>sharedSource(source,{files:[{path:'core',sha256:'changed'}]}),/source/);assert.throws(()=>sharedSource(source,{files:[{path:'binding',sha256:'b'}]}),/source/);});
test('R4 image changes surface while camera changes and failed views reject comparison; costs remain separate',()=>{const a=fixture(),b=fixture();const c={case_id:'oak',view:'whole',azimuth_deg:0,capture_status:'pass',camera:{x:1},conditions_sha256:'c',artifacts:[{path:'beauty-128.png',sha256:'image'}]};a.captures=[structuredClone(c)];b.captures=[structuredClone(c)];assert.equal(compareRecords(a,b).costs.status,'inconclusive');b.captures[0].artifacts[0].sha256='changed';assert.equal(compareRecords(a,b).views[0].image_changed,true);b.captures[0].projected_gaps={status:'measured',primary:{occupied_ratio:0.4}};assert.equal(compareRecords(a,b).views[0].gaps_changed,true);assert.deepEqual(compareRecords(a,b).views[0].candidate_gaps,b.captures[0].projected_gaps);b.captures[0].camera.x=2;assert.equal(compareRecords(a,b).status,'inconclusive');b.captures[0].camera.x=1;b.captures[0].capture_status='fail';assert.equal(compareRecords(a,b).status,'inconclusive');});

import { mkdtemp, writeFile, readFile, mkdir, cp, rm } from 'node:fs/promises';
import { join, resolve } from 'node:path';
import { tmpdir } from 'node:os';
import { sha, artifact } from './geometry-benchmark.mjs';
import { comparePaths } from '../../scripts/benchmarks/geometry-compare.mjs';
import { loadProjectedGaps } from '../../scripts/benchmarks/geometry-compare.mjs';
import { analyzeGaps } from './geometry-benchmark-diagnostics.mjs';
test('R4 passing whole views require one hashed, successful and complete gap artifact',async()=>{
 const dir=await mkdtemp(join(tmpdir(),'fn19-gap-receipt-'));
 try {
  await mkdir(join(dir,'view'));
  const coverage=new Float32Array(25),roi=new Uint8Array(25);
  for(let y=1;y<4;y++)for(let x=1;x<4;x++){roi[y*5+x]=1;coverage[y*5+x]=1;}
  coverage[12]=0;
  const valid=analyzeGaps({coverage,roi,width:5,height:5,metresPerPixel:.1});
  const record={view:'whole',capture_status:'pass',artifacts:[]};
  await assert.rejects(loadProjectedGaps(dir,record),/gap/);
  const saveGap=async value=>{await writeFile(join(dir,'view/gaps.json'),JSON.stringify(value));record.artifacts=[await artifact(dir,'view/gaps.json','application/json')];};
  await saveGap(valid);assert.deepEqual(await loadProjectedGaps(dir,record),valid);
  record.artifacts.push(record.artifacts[0]);await assert.rejects(loadProjectedGaps(dir,record),/gap/);
  for(const mutate of [g=>g.status='failed',g=>delete g.units,g=>delete g.primary,g=>g.primary.occupied_ratio=null,g=>g.primary.occupied_pixels++,g=>g.sensitivity=[],g=>g.primary.height_bands.pop()]){
   const bad=structuredClone(valid);mutate(bad);await saveGap(bad);await assert.rejects(loadProjectedGaps(dir,record),/gap/);
  }
  await saveGap(valid);await writeFile(join(dir,'view/gaps.json'),'{}');await assert.rejects(loadProjectedGaps(dir,record),/hash\/size/);
  assert.equal(await loadProjectedGaps(dir,{view:'fork',capture_status:'pass',artifacts:[]}),null);
  assert.equal(await loadProjectedGaps(dir,{view:'whole',capture_status:'fail',artifacts:[]}),null);
 } finally {await rm(dir,{recursive:true,force:true});}
});
test('R4 receipt loader rejects stale binaries/source, interrupted JSONL and tampered metric artifacts',async()=>{
 const dir=await mkdtemp(join(tmpdir(),'fn19-combiner-test-')),base=join(dir,'base');await mkdir(base);
 const protocol=resolve('.flow/evidence/fn19/protocol.json'),references=resolve('.flow/evidence/fn19/references.json'),p=JSON.parse(await readFile(protocol));
 const save=(path,value)=>writeFile(path,JSON.stringify(value,null,2)+'\n');
 await save(join(base,'manifest.json'),p.cases);await writeFile(join(base,'native-binary'),'synthetic test binary; never generated geometry');
 const files=[{path:'synthetic-test-source',sha256:sha('test')}],id={commit:'a'.repeat(40),dirty:false,files,sha256:sha(files.map(f=>f.path+'\0'+f.sha256+'\n').join(''))};
 const parameters=Object.fromEntries(p.cases.map(c=>{const v=structuredClone(p.species.find(s=>s.id===c.species_id).parameters);v.skeleton.seed=c.seed;return [c.id,v];}));
 const run={schema_version:1,benchmark_id:p.benchmark_id,run_id:'synthetic',created_at:'test',protocol_sha256:sha(await readFile(protocol)),references_sha256:sha(await readFile(references)),source:id,tool:id,binaries:[await artifact(base,'native-binary','application/octet-stream')],manifest_sha256:sha(await readFile(join(base,'manifest.json'))),conditions_sha256:null,parameters,cases:p.cases,machine:{exclusive_window:false},status:'complete',partial:false,reason:null};
 const rows=[];
 for(let i=0;i<p.cases.length;i++){
   await mkdir(join(base,'cases',String(i)),{recursive:true});const metric={status:'measured',value:1,unit:'fixture',definition:'Synthetic receipt control, not botanical measurement',evidence_ids:[],reason:null,support:{included:1,excluded:0,flags:[]}};const metrics={axes:metric,foliage_bins:metric};
   await save(join(base,'cases',String(i),'native.json'),{metrics});rows.push({schema_version:1,run_id:run.run_id,case_id:p.cases[i].id,event:'completed',numeric_status:'pass',metrics,costs:[],artifacts:[await artifact(base,`cases/${i}/native.json`,'application/json')],reason:null});
 }
 await save(join(base,'run.json'),run);await writeFile(join(base,'measurements.jsonl'),rows.map(r=>JSON.stringify(r)+'\n').join(''));
 const opts={protocol,references,baseline:base,candidate:base};assert.equal((await comparePaths(opts)).status,'comparable');
 const variants=[
  ['binary',async d=>writeFile(join(d,'native-binary'),'stale')],
  ['source',async d=>{const r=structuredClone(run);r.source.sha256='0'.repeat(64);await save(join(d,'run.json'),r);}],
  ['artifact',async d=>writeFile(join(d,'cases/0/native.json'),'{}')],
  ['duplicate',async d=>writeFile(join(d,'measurements.jsonl'),[...rows,rows[0]].map(r=>JSON.stringify(r)+'\n').join(''))],
 ];
 for(const [name,mutate] of variants){const d=join(dir,name);await cp(base,d,{recursive:true});await mutate(d);await assert.rejects(comparePaths({...opts,candidate:d}),undefined,name);}
 const interrupted=join(dir,'interrupted');await cp(base,interrupted,{recursive:true});await writeFile(join(interrupted,'measurements.jsonl'),JSON.stringify(rows[0]));assert.equal((await comparePaths({...opts,candidate:interrupted})).status,'inconclusive');
 await rm(dir,{recursive:true,force:true});
});
test('R4 compact reports still detect raw axis-sample changes through full metric digests',()=>{const a=fixture(),b=fixture();a.rows.oak.metric_content_sha256='a';b.rows.oak.metric_content_sha256='b';const r=compareRecords(a,b);assert.equal(r.metrics.oak.changed,true);assert.equal(r.status,'inconclusive');});
