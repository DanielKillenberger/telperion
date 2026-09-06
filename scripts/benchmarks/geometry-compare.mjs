import { readFile, mkdir, writeFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { sha, verifyArtifacts, verifyCaptureArtifacts } from '../../tests/browser/geometry-benchmark.mjs';
const root=resolve(dirname(fileURLToPath(import.meta.url)),'../..');
const canonical=x=>JSON.stringify(x&&typeof x==='object'?Array.isArray(x)?x.map(v=>JSON.parse(canonical(v))):Object.fromEntries(Object.keys(x).sort().map(k=>[k,JSON.parse(canonical(x[k]))])):x);
const equal=(a,b)=>canonical(a)===canonical(b);
const json=async p=>JSON.parse(await readFile(p,'utf8'));
const key=c=>[c.case_id,c.view,c.azimuth_deg].join('/');
export function sharedSource(native,visual){
  if(!native.files.length)throw Error('source: empty native identity');
  for(const f of native.files)if(visual.files.find(v=>v.path===f.path)?.sha256!==f.sha256)throw Error('source: native/browser shared file mismatch '+f.path);
}
export function compareRecords(a,b){
  const reasons=[],metrics={},views=[];
  for(const k of ['protocol_sha256','references_sha256','manifest_sha256','cases','parameters','conditions_sha256'])if(!equal(a.run[k],b.run[k]))reasons.push('condition mismatch: '+k);
  if(a.run.tool.sha256!==b.run.tool.sha256)reasons.push('measurement tool mismatch');
  if(a.run.source.sha256===b.run.source.sha256&&!equal(a.run.binaries?.map(x=>x.sha256),b.run.binaries?.map(x=>x.sha256)))reasons.push('unchanged-source binary mismatch');
  for(const side of [a,b]){
    const ids=side.run.cases.map(c=>c.id);
    if(!ids.length||new Set(ids).size!==ids.length)reasons.push('missing/duplicate case manifest');
    if(side.run.partial||side.run.status!=='complete')reasons.push('incomplete run');
  }
  for(const cid of new Set([...a.run.cases,...b.run.cases].map(c=>c.id))){
    const x=a.rows[cid],y=b.rows[cid];
    metrics[cid]={baseline:x??null,candidate:y??null,changed:!!x&&!!y&&(!equal(x.metrics,y.metrics)||x.metric_content_sha256!==y.metric_content_sha256)};
    if(!x||!y)reasons.push('missing case: '+cid);
    else if([x,y].some(r=>r.event!=='completed'||r.numeric_status!=='pass'))reasons.push('failed/unavailable/interrupted case: '+cid);
    if(metrics[cid].changed&&a.run.source.sha256===b.run.source.sha256)reasons.push('unchanged-source measurement drift: '+cid);
  }
  if(a.captures||b.captures){
    if(!a.captures||!b.captures)reasons.push('missing visual domain');
    else {
      const am=new Map(a.captures.map(c=>[key(c),c])),bm=new Map(b.captures.map(c=>[key(c),c]));
      if(am.size!==a.captures.length||bm.size!==b.captures.length)reasons.push('duplicate view');
      for(const id of new Set([...am.keys(),...bm.keys()])){
        const x=am.get(id),y=bm.get(id);
        const image=c=>c?.artifacts.filter(v=>/\.(png|f32)$/.test(v.path)).map(v=>[v.path.split('/').at(-1),v.sha256]).sort();
        views.push({id,baseline_status:x?.capture_status??'missing',candidate_status:y?.capture_status??'missing',baseline_reason:x?.reason??null,candidate_reason:y?.reason??null,image_changed:!!x&&!!y&&!equal(image(x),image(y)),geometry_changed:!!x&&!!y&&x.geometry_sha256!==y.geometry_sha256,baseline_gaps:x?.projected_gaps??null,candidate_gaps:y?.projected_gaps??null,gaps_changed:!!x&&!!y&&!equal(x.projected_gaps,y.projected_gaps)});
        if(!x||!y||x.capture_status!=='pass'||y.capture_status!=='pass')reasons.push('failed/missing view: '+id);
        else if(!equal(x.camera,y.camera)||x.conditions_sha256!==y.conditions_sha256||x.backend!==y.backend||x.browser!==y.browser)reasons.push('camera/conditions mismatch: '+id);
      }
    }
  }
  return {schema_version:1,scope:a.captures||b.captures?'numeric-and-visual':'numeric-only',status:reasons.length?'inconclusive':'comparable',reasons:[...new Set(reasons)],baseline_run_id:a.run.run_id,candidate_run_id:b.run.run_id,metrics,views,costs:{status:'inconclusive',reason:'Native generation is skeleton-branching-only, not total generation; browser generation/capture and memory domains remain distinct. No coordinated exclusive measurement receipt is available.',baseline_machine:a.run.machine,candidate_machine:b.run.machine},independent_assessment:'unassessed',biological_superiority:'unestablished'};
}
async function identity(i){
  if(!i?.files?.length||!i.commit)throw Error('source/tool: missing identity');
  const names=i.files.map(f=>f.path);
  if(!equal(names,[...new Set(names)].sort())||i.sha256!==sha(i.files.map(f=>f.path+'\0'+f.sha256+'\n').join('')))throw Error('source/tool digest mismatch');
}
export async function loadNumeric(path,protocolPath,referencesPath){
  const script=`import importlib.util,json,sys\ns=importlib.util.spec_from_file_location('r','crates/telperion-core/examples/geometry_benchmark/runner.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m)\np=m.read(sys.argv[2]);r=m.read(sys.argv[3]);m.manifest(p,r)\na,rows=m.load_run(sys.argv[1],p,m.sha(open(sys.argv[2],'rb').read()))\nfor row in rows.values():\n h=m.hashlib.sha256()\n for chunk in json.JSONEncoder(sort_keys=True,separators=(',',':'),allow_nan=False).iterencode(row['metrics']): h.update(chunk.encode())\n row['metric_content_sha256']=h.hexdigest()\n axes=row['metrics'].get('axes',{}).get('value')\n if isinstance(axes,dict): axes.pop('samples',None)\nprint(json.dumps({'run':a,'rows':rows}))`;
  const result=JSON.parse(execFileSync('python3',['-B','-c',script,path,protocolPath,referencesPath],{cwd:root,maxBuffer:256*1024*1024,timeout:600000,encoding:'utf8',stdio:['ignore','pipe','pipe']}));
  const p=await json(protocolPath);
  for(const d of ['source','tool'])await identity(result.run[d]);
  if(result.run.binaries.length!==1||result.run.binaries[0].path!=='native-binary')throw Error('missing native binary identity');
  if(result.run.benchmark_id!==p.benchmark_id)throw Error('benchmark identity mismatch');
  if(result.run.references_sha256!==sha(await readFile(referencesPath)))throw Error('reference identity mismatch');
  for(const c of p.cases){const params=structuredClone(p.species.find(s=>s.id===c.species_id).parameters);params.skeleton.seed=c.seed;if(!equal(params,result.run.parameters[c.id]))throw Error('parameter identity mismatch');}
  for(const row of Object.values(result.rows))if(row.event==='completed'&&row.numeric_status==='pass'&&(!['axes','foliage_bins'].every(k=>['measured','estimated'].includes(row.metrics[k]?.status))||!row.artifacts.some(a=>a.path.endsWith('/native.json'))))throw Error('missing numeric evidence');
  result.captures=null;return result;
}
export async function addVisual(numeric,path,protocolPath){
  const run=await json(resolve(path,'run.json')),p=await json(protocolPath);
  for(const d of ['source','tool'])await identity(run[d]);
  await verifyArtifacts(path,run.binaries);
  for(const k of ['protocol_sha256','references_sha256','cases','parameters'])if(!equal(run[k],numeric.run[k]))throw Error('numeric/visual identity mismatch: '+k);
  sharedSource(numeric.run.source,run.source);
  const support=run.binaries.find(b=>b.path==='generator-support');
  if(support?.sha256!==numeric.run.binaries.find(b=>b.path==='native-binary')?.sha256)throw Error('native/support binary mismatch');
  if(sha(await readFile(resolve(path,'manifest.json')))!==run.manifest_sha256||!equal(await json(resolve(path,'manifest.json')),run.cases))throw Error('visual manifest mismatch');
  const captures=await json(resolve(path,'captures.json'));
  const expected=p.cases.flatMap(c=>p.required_views.flatMap(v=>v.azimuth_deg.map(azimuth_deg=>key({case_id:c.id,view:v.id,azimuth_deg}))));
  if(!equal(captures.map(key).sort(),expected.sort()))throw Error('missing/duplicate visual cases/views');
  const conditions=await json(resolve(path,'conditions.json'));
  if(sha(await readFile(resolve(path,'conditions.json')))!==run.conditions_sha256)throw Error('conditions digest mismatch');
  for(const c of captures){
    if(c.run_id!==run.run_id||c.conditions_sha256!==run.conditions_sha256)throw Error('capture identity mismatch');
    if(c.artifacts.length)await verifyArtifacts(path,c.artifacts);
    const gapArtifact=c.artifacts.find(a=>a.path.endsWith('/gaps.json'));
    c.projected_gaps=gapArtifact?await json(resolve(path,gapArtifact.path)):null;
    const geometryArtifact=c.artifacts.find(a=>a.path.endsWith('/geometry.json'));
    if(geometryArtifact){
      const geometry=await json(resolve(path,geometryArtifact.path));
      if(geometry.wasm_sha256!==run.binaries.find(b=>b.path==='generator.wasm')?.sha256||sha(JSON.stringify(geometry.hashes,null,2)+'\n')!==c.geometry_sha256)throw Error('stale geometry/binary identity');
      c.backend=geometry.backend;c.browser=geometry.browser;
    }
    if(c.capture_status==='pass'){
      await verifyCaptureArtifacts(path,c);
      const fixed=conditions.entries.find(e=>key(e)===key(c));
      if(!fixed||!equal(fixed.camera,c.camera))throw Error('camera differs from frozen condition');
      if(fixed.roi)await verifyArtifacts(path,[fixed.roi.raster]);
      if(c.convergence.status!=='measured')throw Error('unconverged capture marked pass');
    }
  }
  numeric.captures=captures;numeric.visualRun=run;
}
export async function comparePaths(options){
  const {protocol,references,baseline,candidate,baselineVisual,candidateVisual}=options;
  const a=await loadNumeric(baseline,protocol,references),b=await loadNumeric(candidate,protocol,references);
  if(!!baselineVisual!==!!candidateVisual)throw Error('both visual runs required');
  if(baselineVisual){await addVisual(a,baselineVisual,protocol);await addVisual(b,candidateVisual,protocol);}
  const result=compareRecords(a,b);
  result.inputs={protocol,references,baseline,candidate,baseline_visual:baselineVisual??null,candidate_visual:candidateVisual??null};
  result.run_receipts={baseline:a.run,candidate:b.run,baseline_visual:a.visualRun??null,candidate_visual:b.visualRun??null};
  if(a.visualRun){
    for(const k of ['conditions_sha256'])if(a.visualRun[k]!==b.visualRun[k])result.reasons.push('visual condition mismatch: '+k);
    if(a.visualRun.tool.sha256!==b.visualRun.tool.sha256)result.reasons.push('visual tool mismatch');
    for(const s of [a,b])if(s.visualRun.status!=='complete'||s.visualRun.partial)result.reasons.push('incomplete visual run');
    result.status=result.reasons.length?'inconclusive':'comparable';
  }
  return result;
}
async function main(args){
  if(args.includes('--help')){console.log('node scripts/benchmarks/geometry-compare.mjs --protocol FILE --references FILE --baseline NUMERIC_DIR --candidate NUMERIC_DIR [--baseline-visual DIR --candidate-visual DIR] --output NEW_DIR\nExit 0: comparable evidence (not botanical approval); 1: inconclusive including identity, failed or interrupted evidence. Costs stay a separate unqualified domain. New cohorts require their own protocol and both sides.');return;}
  const names={'--protocol':'protocol','--references':'references','--baseline':'baseline','--candidate':'candidate','--baseline-visual':'baselineVisual','--candidate-visual':'candidateVisual','--output':'output'},o={};
  for(let i=0;i<args.length;i++){const name=names[args[i]];if(!name||o[name]||!args[i+1]||args[i+1].startsWith('--'))throw Error('invalid arguments');o[name]=resolve(args[++i]);}
  for(const name of ['protocol','references','baseline','candidate','output'])if(!o[name])throw Error('missing '+name);
  await mkdir(o.output,{recursive:false});
  let result;try{result=await comparePaths(o);}catch(error){result={schema_version:1,status:'inconclusive',reasons:[String(error)],inputs:o,independent_assessment:'unassessed',biological_superiority:'unestablished'};}
  await writeFile(resolve(o.output,'comparison.json'),JSON.stringify(result,null,2)+'\n');
  console.log(result.status+': '+o.output);process.exitCode=result.status==='comparable'?0:1;
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url))main(process.argv.slice(2)).catch(e=>{console.error(e);process.exitCode=2;});
