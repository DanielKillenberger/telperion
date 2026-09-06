import { pathToFileURL } from 'node:url';
import { mkdir, writeFile, readFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { cpus, totalmem, platform, release } from 'node:os';
import { createHash } from 'node:crypto';
const { chromium } = await import(process.env.PLAYWRIGHT_MODULE ? pathToFileURL(process.env.PLAYWRIGHT_MODULE).href : 'playwright');
const out = process.env.GENERATION_OUTPUT || '/tmp/fn12-generation';
const url = process.env.BROWSER_URL || 'http://127.0.0.1:5188';
await mkdir(out,{recursive:true});
const browser = await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE || '/usr/bin/chromium',headless:true,args:['--no-sandbox','--disable-gpu']});
const result = {
  recordedAt:new Date().toISOString(), schema:1,
  method:'CPU only, mesh-free full presets; alternating paired Ordinary/Telperion, one warmup and five measured samples each. Indexed Rust/Wasm query wall time includes cell transfer and copied flags. JS snapshot traversal is correctness-only. Grids 32^3 and 64^3 plus fixed point/outside/contact matrix. Packing is f64-to-f32 conversion plus topology copies, not a qualified GPU layout. No GPU timings.',
  source:{commit:execFileSync('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim(),dirty:execFileSync('git',['status','--porcelain'],{encoding:'utf8'}),wasmSha256:createHash('sha256').update(await readFile(new URL('../../src/browser/telperion.wasm',import.meta.url))).digest('hex')},
  hardware:{cpu:cpus()[0]?.model,logicalCpus:cpus().length,totalMemoryBytes:totalmem(),platform:platform(),osRelease:release(),browser:browser.version(),gpu:'disabled; adapter not requested'},
  subjects:[], samples:[],
};
const save = () => writeFile(out+'/results.json',JSON.stringify(result,null,2));
const pages = new Map();
try {
  for (const subject of ['ordinary','telperion']) {
    const page = await browser.newPage({acceptDownloads:true}); page.setDefaultTimeout(600000);
    await page.route(url+'/',r=>r.fulfill({contentType:'text/html',body:'<!doctype html>'})); await page.goto(url);
    const metadata = await page.evaluate(async subject => {
      const {TreeEngine,presetById} = await import('/src/browser/core.ts');
      const {id,name,note,...family} = presetById(subject);
      window.engine = await TreeEngine.create(); window.family = family;
      const {hashArray} = await import('/scripts/benchmarks/generation-inputs.mjs');
      return {subject,family,parameterHash:await hashArray(new TextEncoder().encode(JSON.stringify(family))),userAgent:navigator.userAgent,requestedOutputs:{field:true}};
    },subject);
    result.subjects.push(metadata); pages.set(subject,page);
  }
  for (let sample=-1;sample<5;sample++) for(const subject of (sample%2===0?['telperion','ordinary']:['ordinary','telperion'])) {
    const page = pages.get(subject);
    const row = await page.evaluate(async ({sample,subject}) => {
      const {snapshotArrays,querySnapshot,gridCells,boundaryCells,hashArray} = await import('/scripts/benchmarks/generation-inputs.mjs');
      const output = window.engine.build(window.family,{field:true}), d = output.diagnostics;
      if(output.surface || output.foliage || d.timings.surfaceMs!==0) throw Error('unexpected render generation');
      const wasmBeforeSnapshot = window.engine.memoryBytes, snapshot = output.field.snapshot();
      const arrays = snapshotArrays(snapshot), sourceBytes = Object.values(arrays).reduce((n,a)=>n+a.byteLength,0);
      const packStart = performance.now();
      const packed = Object.fromEntries(Object.entries(arrays).map(([name,a])=>[name,a instanceof Float64Array?new Float32Array(a):a.slice()]));
      const packingMs = performance.now()-packStart, packedBytes = Object.values(packed).reduce((n,a)=>n+a.byteLength,0);
      const nonfinitePacked = Object.values(packed).some(a=>a.some(v=>!Number.isFinite(v)));
      const hashStart = performance.now(), hashes = {};
      for(const [name,array] of Object.entries(arrays)) hashes[name] = await hashArray(array);
      const hashingMs = performance.now()-hashStart;
      const queries = [], artifacts = {...arrays};
      for(const size of [32,64,'boundary']) {
        const prep = performance.now(), cells = size==='boundary'?boundaryCells(snapshot):gridCells(snapshot.bounds,size);
        const cellsMs = performance.now()-prep, start = performance.now(), flags = output.field.query(cells), queryMs = performance.now()-start;
        let verificationMs = null, mismatchCount = null;
        if(sample===-1 || size==='boundary') {
          const start = performance.now(), reference = querySnapshot(snapshot,cells);
          verificationMs = performance.now()-start;
          mismatchCount = flags.reduce((n,v,i)=>n+Number(v!==reference[i]),0);
        }
        const hashStart = performance.now(), cellsHash = await hashArray(cells), flagsHash = await hashArray(flags), hashMs = performance.now()-hashStart;
        queries.push({size,cells:flags.length,cellsMs,queryMs,wood:flags.reduce((n,v)=>n+Number(Boolean(v&1)),0),foliage:flags.reduce((n,v)=>n+Number(Boolean(v&2)),0),cellsHash,flagsHash,hashMs,verificationMs,mismatchCount,
          memory:{cellsBytes:cells.byteLength,resultBytes:flags.byteLength},
          coldCpuMs:d.timings.buildMs+cellsMs+queryMs,
          candidateSharedPreparationMs:snapshot.timings.totalMs+packingMs});
        artifacts['cells-'+size]=cells; artifacts['flags-'+size]=flags;
      }
      window.artifacts = sample===0?artifacts:null;
      const row = {sample,warmup:sample<0,subject,diagnostics:d,snapshot:{schema:snapshot.schema,bounds:snapshot.bounds,woodNodes:snapshot.woodIndex.nodeCount,leafNodes:snapshot.leaves.nodeCount,
        woodItems:snapshot.wood.length/8,leafItems:snapshot.leaves.bounds.length/6-snapshot.leaves.nodeCount,hashes,timings:snapshot.timings},
        packingMs,hashingMs,nonfinitePacked,queries,
        memory:{nativeFieldBytes:d.fieldBytes,wasmBeforeSnapshotBytes:wasmBeforeSnapshot,wasmHighWaterBytes:window.engine.memoryBytes,nativeSnapshotTransientBytes:sourceBytes,jsOwnedSnapshotBytes:sourceBytes,jsPackedBytes:packedBytes,
          note:'Wasm high-water includes allocator capacity and staging: do not add nativeField/transient bytes to it. JS source and packed arrays coexist; Wasm query scratch duplicates the largest cells/result buffers. Browser/GC/hash/download overhead is not estimated.'}};
      window.engine.release();
      return row;
    },{sample,subject});
    const previous = result.samples.find(r=>r.subject===subject);
    row.repeatable = !previous || (JSON.stringify(previous.snapshot.hashes)===JSON.stringify(row.snapshot.hashes) && row.queries.every((q,i)=>q.cellsHash===previous.queries[i].cellsHash && q.flagsHash===previous.queries[i].flagsHash));
    row.valid = row.repeatable && row.diagnostics.complete && row.snapshot.woodItems===row.diagnostics.nodes && row.snapshot.leafItems===row.diagnostics.instances && row.queries.every(q=>q.mismatchCount===null||q.mismatchCount===0);
    result.samples.push(row); await save();
    console.log(subject,sample,'build',row.diagnostics.timings.buildMs.toFixed(1),'query32/64',row.queries.slice(0,2).map(q=>q.queryMs.toFixed(1)).join('/'),'complete',row.diagnostics.complete,'valid',row.valid);
    if(sample===0) {
      const start = performance.now();
      const manifest = await page.evaluate(()=>Object.entries(window.artifacts).map(([name,a])=>({name,type:a.constructor.name,length:a.length,bytes:a.byteLength})));
      const downloadPromise = page.waitForEvent('download', {timeout:60000});
      await page.evaluate(filename=> {
        const a=document.createElement('a'), blob=new Blob(Object.values(window.artifacts)), url=URL.createObjectURL(blob);
        a.href=url;a.download=filename;a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);
      },subject+'-inputs.bin');
      const download = await downloadPromise; await download.saveAs(out+'/'+subject+'-inputs.bin');
      let offset = 0;
      for(const entry of manifest) { entry.offset = offset; offset += entry.bytes; }
      await page.evaluate(()=>window.artifacts=null);
      row.artifacts={byteOrder:'little-endian',path:subject+'-inputs.bin',arrays:manifest,saveMs:performance.now()-start}; await save();
    }
    if(!row.valid) throw Error(subject+' invalid sample; see preserved evidence');
  }
  const summary = values => {const a=[...values].sort((a,b)=>a-b);return {median:a[Math.floor(a.length/2)],p95:a[Math.ceil(.95*a.length)-1],min:a[0],max:a.at(-1)};};
  result.summary = result.subjects.map(({subject})=>{
    const rows=result.samples.filter(r=>r.subject===subject&&!r.warmup);
    return {subject,stages:Object.fromEntries(Object.keys(rows[0].diagnostics.timings).map(k=>[k,summary(rows.map(r=>r.diagnostics.timings[k]))])),
      snapshotMs:summary(rows.map(r=>r.snapshot.timings.totalMs)),packingMs:summary(rows.map(r=>r.packingMs)),queries:rows[0].queries.map((q,i)=>({size:q.size,queryMs:summary(rows.map(r=>r.queries[i].queryMs))}))};
  });
  result.finishedAt = new Date().toISOString(); await save();
} catch(error) { result.failure=String(error);await save();throw error; }
finally { await browser.close(); }
