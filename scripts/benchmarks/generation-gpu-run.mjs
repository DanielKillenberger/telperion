import { pathToFileURL } from 'node:url';
import { mkdir,writeFile,readFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { cpus,totalmem,platform,release } from 'node:os';
import { createHash } from 'node:crypto';
const {chromium}=await import(process.env.PLAYWRIGHT_MODULE?pathToFileURL(process.env.PLAYWRIGHT_MODULE).href:'playwright');
const out=process.env.GENERATION_OUTPUT||'/tmp/fn12-gpu',url=process.env.BROWSER_URL||'http://127.0.0.1:5188';
const args=['--enable-gpu','--use-gl=angle','--use-angle=vulkan','--enable-features=Vulkan','--disable-vulkan-surface','--enable-unsafe-webgpu','--ozone-platform=x11'];
await mkdir(out,{recursive:true});
const browser=await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE||'/usr/bin/chromium',headless:true,args});
const result={schema:1,recordedAt:new Date().toISOString(),
  method:'Exclusive GPU lease. Full presets, field only. Alternating subjects and CPU/GPU query order; one warmup and five paired measured samples. Each session uploads the indexed snapshot once; first query wall time includes query packing/allocation/upload/submission/readback; immediate second CPU/GPU pair checks deterministic reuse and reports supplemental resident time. Two actual query pairs supply a supplemental per-query amortization estimate. A separate direct caller stopwatch measures EACH cold workload from fresh snapshot extraction through device creation, one query and device disposal, with no CPU reference or hashing inside that interval; its separately paired CPU query is the cold denominator. Two-query amortization is a summed stage estimate from the resident session, including measured teardown; it is supplemental, not a direct lifecycle stopwatch. Whole-build totals also charge the unchanged CPU prerequisites. No correction for ordinary f32 boundary drift.',
  gates:{medianGain:0.20,nonoverlap:'maximum GPU end-to-end sample < minimum CPU end-to-end sample',tail:'GPU max <= CPU max',precision:'every occupancy bit equals indexed Rust/Wasm; any unexplained mismatch rejects uncorrected candidate',repeatability:'source, input, CPU and GPU hashes agree across repeated builds and repeated queries',resident:'supplemental source-resident query; not cold adoption evidence'},
  source:{commit:execFileSync('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim(),dirty:execFileSync('git',['status','--porcelain'],{encoding:'utf8'}),wasmSha256:createHash('sha256').update(await readFile(new URL('../../src/browser/telperion.wasm',import.meta.url))).digest('hex'),candidateSha256:createHash('sha256').update(await readFile(new URL('./generation-gpu.mjs',import.meta.url))).digest('hex')},
  hardware:{cpu:cpus()[0]?.model,logicalCpus:cpus().length,totalMemoryBytes:totalmem(),platform:platform(),osRelease:release(),browser:browser.version(),args},subjects:[],samples:[]};
const save=()=>writeFile(out+'/results.json',JSON.stringify(result,null,2)+'\n');
try{
  const pages=new Map();
  for(const subject of ['ordinary','telperion']){
    const page=await browser.newPage();page.setDefaultTimeout(600000);
    await page.route(url+'/',r=>r.fulfill({contentType:'text/html',body:'<!doctype html><canvas></canvas>'}));await page.goto(url);
    const glRenderer=await page.evaluate(()=>{const gl=document.querySelector('canvas').getContext('webgl2'),ext=gl?.getExtension('WEBGL_debug_renderer_info');return ext?gl.getParameter(ext.UNMASKED_RENDERER_WEBGL):null;});
    await page.waitForTimeout(500);
    const metadata=await page.evaluate(async subject=>{
      const {TreeEngine,presetById}=await import('/src/browser/core.ts');const {id,name,note,...family}=presetById(subject);
      window.engine=await TreeEngine.create();window.family=family;
      const {hashArray}=await import('/scripts/benchmarks/generation-inputs.mjs');
      return {subject,family,parameterHash:await hashArray(new TextEncoder().encode(JSON.stringify(family))),userAgent:navigator.userAgent,requestedOutputs:{field:true}};
    },subject);
    result.subjects.push({...metadata,glRenderer});pages.set(subject,page);
  }
  for(let sample=-1;sample<5;sample++)for(const subject of sample%2===0?['telperion','ordinary']:['ordinary','telperion']){
    const row=await pages.get(subject).evaluate(async({sample,subject})=>{
      const {createGpuField}=await import('/scripts/benchmarks/generation-gpu.mjs');
      const {snapshotArrays,gridCells,boundaryCells,hashArray}=await import('/scripts/benchmarks/generation-inputs.mjs');
      const output=window.engine.build(window.family,{field:true}),d=output.diagnostics;let snapshot=output.field.snapshot();
      const row={sample,warmup:sample<0,subject,diagnostics:d,snapshot:{bounds:snapshot.bounds,timings:snapshot.timings,hashes:{},woodNodes:snapshot.woodIndex.nodeCount,leafNodes:snapshot.leaves.nodeCount},queries:[],memory:{nativeFieldBytes:d.fieldBytes,wasmHighWaterBytes:window.engine.memoryBytes}};
      const gpu=await createGpuField(snapshot);
      if(!gpu.ok){row.failure=gpu;window.engine.release();return row;}
      row.adapter=gpu.metadata;row.preparation=gpu.preparation;row.memory={...row.memory,...gpu.memory,nativeSnapshotTransientBytes:gpu.memory.sourceBytes};
      try{
        for(const size of [32,64,'boundary']){
          const p=performance.now(),cells=size==='boundary'?boundaryCells(snapshot):gridCells(snapshot.bounds,size),cellsMs=performance.now()-p;
          let flags,cpuMs,first;
          const cpu=()=>{const t=performance.now();flags=output.field.query(cells);cpuMs=performance.now()-t;};
          if(sample%2===0){first=await gpu.query(cells);cpu();}else{cpu();first=await gpu.query(cells);}
          const query={size,count:cells.length/4,cellsMs,cpuMs,order:sample%2===0?'gpu-cpu':'cpu-gpu',cellsHash:await hashArray(cells),cpuHash:await hashArray(flags)};
          if(!first.ok){query.failure=first;row.queries.push(query);continue;}
          let repeat,cpuRepeatMs,cpuRepeat;
          const repeatCpu=()=>{const t=performance.now();cpuRepeat=output.field.query(cells);cpuRepeatMs=performance.now()-t;};
          if(sample%2===0){repeat=await gpu.query(cells);repeatCpu();}else{repeatCpu();repeat=await gpu.query(cells);}
          const mismatches=[];let mismatchCount=0,woodMismatch=0,leafMismatch=0,falseNegatives=0;
          for(let i=0;i<flags.length;i++)if(flags[i]!==first.flags[i]){
            mismatchCount++;woodMismatch+=Number(Boolean((flags[i]^first.flags[i])&1));leafMismatch+=Number(Boolean((flags[i]^first.flags[i])&2));falseNegatives+=Number(Boolean(flags[i]&~first.flags[i]));
            mismatches.push({index:i,cell:Array.from(cells.subarray(i*4,i*4+4)),cpu:flags[i],gpu:first.flags[i]});
          }
          const coldQueryMs=snapshot.timings.totalMs+gpu.preparation.totalMs+first.timings.totalMs;
          Object.assign(query,{gpuHash:await hashArray(first.flags),repeatHash:repeat.ok?await hashArray(repeat.flags):null,repeatFailure:repeat.ok?null:repeat,
            repeatable:repeat.ok&&repeat.flags.every((v,i)=>v===first.flags[i])&&cpuRepeat.every((v,i)=>v===flags[i]),cpuRepeatMs,mismatchCount,woodMismatch,leafMismatch,falseNegatives,mismatches,
            gpu:first.timings,residentRepeat:repeat.ok?repeat.timings:null,outsideRangeCount:first.outsideRangeCount,memory:first.memory,
            coldQueryMs,summedColdEstimateMs:coldQueryMs,coldCpuMs:cpuMs,amortizedTwoQueriesGpuMs:repeat.ok?(coldQueryMs+repeat.timings.totalMs)/2:null,amortizedTwoQueriesCpuMs:(cpuMs+cpuRepeatMs)/2,wholeCpuMs:d.timings.buildMs+cellsMs+cpuMs,wholeGpuMs:d.timings.buildMs+cellsMs+coldQueryMs});
          row.queries.push(query);
          const m=first.memory,field=gpu.memory;
          query.memory.accountedColdBufferPeakBytes=field.sourceBytes+field.packedBytes+field.validationScratchBytes+field.gpuSourceBytes+field.uploadStagingUpperBoundBytes;
          query.memory.accountedResidentBufferPeakBytes=field.sourceBytes+m.querySourceBytes+m.queryPackedBytes+flags.byteLength*4+field.gpuSourceBytes+m.gpuQueryBytes+m.readbackBytes+m.timestampBufferBytes+m.queryUploadStagingUpperBoundBytes;
          query.memory.accountedPeakBytes=Math.max(query.memory.accountedColdBufferPeakBytes,query.memory.accountedResidentBufferPeakBytes);
          query.memory.note='Logical JS array plus GPU buffer allocation upper bounds, including four copied occupancy arrays during repeat comparison; validation scratch conservatively added across both indexes. Driver query-set storage, browser/GC retention and internal writeBuffer staging beyond payload are opaque. Wasm high-water is separate and already includes native field and transient snapshot/query scratch.';
        }
        row.memory.wasmHighWaterBytes=window.engine.memoryBytes;
        for(const [name,a]of Object.entries(snapshotArrays(snapshot)))row.snapshot.hashes[name]=await hashArray(a);
      }finally{const start=performance.now();gpu.dispose();row.residentSessionDisposalMs=performance.now()-start;row.disposed=gpu.disposed;}
      snapshot=null;
      try {
        for(const query of row.queries) {
          const cells=query.size==='boundary'?null:gridCells(row.snapshot.bounds,query.size);
          // Contact cells depend on the source; reconstruct outside the timed operation.
          const probes=cells||boundaryCells(output.field.snapshot());
          let cpuFlags,cpuWallMs;
          const cpu=()=>{const start=performance.now();cpuFlags=output.field.query(probes);cpuWallMs=performance.now()-start;};
          if(sample%2!==0)cpu();
          const lifecycleStart=performance.now();
          const cold=await(async()=>{
            const start=performance.now();let fresh;
            try {
              const source=output.field.snapshot(),snapshotEnd=performance.now();
              fresh=await createGpuField(source);const sessionEnd=performance.now();
              if(!fresh.ok)return {failure:fresh,wallMs:performance.now()-start};
              const answer=await fresh.query(probes),queryEnd=performance.now();
              fresh.dispose();const end=performance.now();
              return {answer,wallMs:end-start,snapshotWallMs:snapshotEnd-start,sessionWallMs:sessionEnd-snapshotEnd,queryWallMs:queryEnd-sessionEnd,disposalMs:end-queryEnd,preparation:fresh.preparation,metadata:fresh.metadata,memory:fresh.memory};
            }finally{fresh?.dispose?.();}
          })();
          cold.wallMs=performance.now()-lifecycleStart;
          if(sample%2===0)cpu();
          if(cold.failure||!cold.answer.ok){query.failure=cold.failure||cold.answer;query.coldLifecycle={wallMs:cold.wallMs};continue;}
          const coldHash=await hashArray(cold.answer.flags),cpuHash=await hashArray(cpuFlags);
          query.repeatable=query.repeatable&&coldHash===query.gpuHash&&cpuHash===query.cpuHash;
          const {answer,...lifecycle}=cold;
          query.coldLifecycle={...lifecycle,gpuHash:coldHash,cpuHash,cpuWallMs,queryTimings:answer.timings};
          query.coldQueryMs=cold.wallMs;query.coldCpuMs=cpuWallMs;
          query.wholeCpuMs=d.timings.buildMs+query.cellsMs+cpuWallMs;
          query.wholeGpuMs=d.timings.buildMs+query.cellsMs+cold.wallMs;
          query.amortizedTwoQueriesGpuMs+=row.residentSessionDisposalMs/2;
        }
      }finally{window.engine.release();}
      return row;
    },{sample,subject});
    const prior=result.samples.find(r=>r.subject===subject);
    row.repeatedInputs=!prior||JSON.stringify(prior.snapshot.hashes)===JSON.stringify(row.snapshot.hashes)&&row.queries.every((q,i)=>q.cellsHash===prior.queries[i].cellsHash&&q.cpuHash===prior.queries[i].cpuHash&&q.gpuHash===prior.queries[i].gpuHash);
    row.complete=row.diagnostics.complete&&!row.diagnostics.capped&&row.diagnostics.timings.surfaceMs===0;
    result.samples.push(row);await save();
    console.log(subject,sample,'complete',row.complete,'queries',row.queries.map(q=>({size:q.size,cpu:q.cpuMs,gpu:q.gpu?.totalMs,cold:q.coldQueryMs,mismatches:q.mismatchCount,failure:q.failure})),row.failure||'');
    if(row.failure||!row.complete||!row.repeatedInputs)throw Error('inconclusive execution or repeated input failure; saved raw evidence');
  }
  const stats=values=>{const a=[...values].sort((a,b)=>a-b);return {median:a[Math.floor(a.length/2)],min:a[0],max:a.at(-1),p95:a[Math.ceil(.95*a.length)-1]};};
  result.summary=result.subjects.flatMap(({subject})=>[32,64,'boundary'].map(size=>{
    const rows=result.samples.filter(r=>r.subject===subject&&!r.warmup),qs=rows.map(r=>r.queries.find(q=>q.size===size));
    if(qs.some(q=>q.failure||!q.repeatable))return {subject,size,classification:'inconclusive',reason:'explicit execution failure or repeatability failure'};
    const cpu=stats(qs.map(q=>q.coldCpuMs)),cold=stats(qs.map(q=>q.coldQueryMs)),resident=stats(qs.map(q=>q.residentRepeat.totalMs));
    const residentCpu=stats(qs.map(q=>q.cpuRepeatMs)),amortizedGpu=stats(qs.map(q=>q.amortizedTwoQueriesGpuMs)),amortizedCpu=stats(qs.map(q=>q.amortizedTwoQueriesCpuMs));
    const wholeCpu=stats(qs.map(q=>q.wholeCpuMs)),wholeGpu=stats(qs.map(q=>q.wholeGpuMs)),precision=qs.every(q=>q.mismatchCount===0),gain=1-cold.median/cpu.median;
    const coldGates={precision,median:gain>=.2,nonoverlap:cold.max<cpu.min,tail:cold.max<=cpu.max};
    const residentGates={precision,median:resident.median<=residentCpu.median*.8,nonoverlap:resident.max<residentCpu.min,tail:resident.max<=residentCpu.max};
    const amortizedGates={precision,median:amortizedGpu.median<=amortizedCpu.median*.8,nonoverlap:amortizedGpu.max<amortizedCpu.min,tail:amortizedGpu.max<=amortizedCpu.max};
    return {subject,size,classification:Object.values(coldGates).every(Boolean)?'qualified':'rejected',coldGates,coldGain:gain,cpu,cold,resident,residentCpu,amortizedGpu,amortizedCpu,amortizedGates,wholeCpu,wholeGpu,residentGates,
      residentClassification:Object.values(residentGates).every(Boolean)?'qualified for measured source-resident query only':'rejected',mismatchCount:qs[0].mismatchCount,
      preparation:stats(rows.map(r=>r.preparation.totalMs)),snapshot:stats(rows.map(r=>r.snapshot.timings.totalMs)),stages:Object.fromEntries(Object.keys(rows[0].diagnostics.timings).map(k=>[k,stats(rows.map(r=>r.diagnostics.timings[k]))]))};
  }));
  result.finishedAt=new Date().toISOString();await save();
}catch(error){result.failure=String(error);await save();throw error;}
finally{await browser.close();}
