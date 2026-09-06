import assert from 'node:assert/strict';
import { prepareSnapshot, packQueries, checkLimits, createGpuField, guarded } from '../../scripts/benchmarks/generation-gpu.mjs';
const empty = () => ({schema:1,wood:new Float64Array(),woodIndex:{nodeCount:0,bounds:new Float64Array(),topology:new Uint32Array()},leaves:{nodeCount:0,bounds:new Float64Array(),topology:new Uint32Array()}});
const s = empty();
const packed = prepareSnapshot(s);
assert.equal(packed.sourceBytes,0);
assert.equal(packQueries(new Float64Array()).count,0);
for (const cells of [[],new Float64Array(1),new Float64Array([0,0,0,-1]),new Float64Array([NaN,0,0,0]),new Float64Array([Infinity,0,0,0]),new Float64Array([1e200,0,0,0])]) assert.throws(()=>packQueries(cells),/input|overflow/);
assert.throws(()=>packQueries(new Float64Array([1e100,0,0,0])),/precision-range/);
assert.throws(()=>prepareSnapshot({...s,schema:2}),/snapshot/);
assert.throws(()=>prepareSnapshot({...s,wood:new Float64Array([0,0,0,0,0,0,-1,0])}),/snapshot/);
assert.throws(()=>prepareSnapshot({...s,leaves:{nodeCount:1,bounds:new Float64Array(6),topology:new Uint32Array([0,0,0,0])}}),/snapshot/);
assert.throws(()=>checkLimits({maxBufferSize:16,maxStorageBufferBindingSize:16,maxComputeWorkgroupsPerDimension:1,maxComputeInvocationsPerWorkgroup:64,maxComputeWorkgroupSizeX:64,maxStorageBuffersPerShaderStage:8},[32],1),/limit/);
assert.throws(()=>checkLimits({maxBufferSize:268435456,maxStorageBufferBindingSize:134217728,maxComputeWorkgroupsPerDimension:1,maxComputeInvocationsPerWorkgroup:64,maxComputeWorkgroupSizeX:64,maxStorageBuffersPerShaderStage:8},[16],65),/limit/);
assert.throws(()=>packQueries(new Float64Array([0,0,0,1e100]),[[-1,-1,-1,1,1,1]]),/precision-range/);
assert.equal((await createGpuField(s,{gpu:null})).error.code,'unavailable-adapter');
assert.equal((await createGpuField(s,{gpu:{requestAdapter:async()=>null}})).error.code,'unavailable-adapter');
let cleaned=0;
await assert.rejects(guarded(()=>new Promise(()=>{}),{timeoutMs:5,onFailure:()=>cleaned++}),/timeout/);
await assert.rejects(guarded(()=>Promise.reject(Error('readback test')), {onFailure:()=>cleaned++}),/readback test/);
assert.equal(cleaned,2);
console.log('generation GPU helper tests passed');

// Controlled doubles exercise API failure plumbing, not hardware reliability.
function fakeGpu(failure) {
  const counts={created:0,destroyed:0,deviceDestroyed:0};let lose;
  const limits={maxBufferSize:268435456,maxStorageBufferBindingSize:134217728,maxComputeWorkgroupsPerDimension:65535,maxComputeInvocationsPerWorkgroup:256,maxComputeWorkgroupSizeX:256,maxStorageBuffersPerShaderStage:8};
  const device={limits,features:new Set(),lost:new Promise(resolve=>lose=resolve),addEventListener(){},
    destroy(){counts.deviceDestroyed++;lose({message:'controlled destruction'});},pushErrorScope(){},async popErrorScope(){return null;},
    createShaderModule(){return {async getCompilationInfo(){return {messages:failure==='shader'?[{type:'error',message:'controlled shader failure'}]:[]};}};},
    async createComputePipelineAsync(){if(failure==='pipeline')throw Error('controlled pipeline failure');return {getBindGroupLayout(){return {};}};},
    createBuffer({size}){if(failure==='allocation')throw Error('controlled allocation failure');counts.created++;let destroyed=false;return {destroy(){if(!destroyed){destroyed=true;counts.destroyed++;}},
      async mapAsync(){if(failure==='readback')throw Error('controlled readback failure');if(failure==='loss'){lose({message:'controlled loss'});await new Promise(()=>{});}if(failure==='timeout')await new Promise(()=>{});},getMappedRange(){const a=new ArrayBuffer(size);if(failure==='traversal')new Uint32Array(a).fill(4);return a;},unmap(){}};},
    createBindGroup(){return {};},createCommandEncoder(){return {beginComputePass(){return {setPipeline(){},setBindGroup(){},dispatchWorkgroups(){},end(){}};},copyBufferToBuffer(){},finish(){return {};}};},
    queue:{writeBuffer(){},async onSubmittedWorkDone(){},submit(){if(failure==='submission')throw Error('controlled submission failure');}}
  };
  return {counts,gpu:{async requestAdapter(){return {limits,features:new Set(),info:{vendor:'controlled double'},async requestDevice(){if(failure==='device')throw Error('controlled device failure');return device;}};}}};
}
globalThis.GPUBufferUsage={STORAGE:1,COPY_DST:2,COPY_SRC:4,MAP_READ:8,UNIFORM:16};globalThis.GPUMapMode={READ:1};
const failures=[];
for(const kind of ['device','shader','pipeline','allocation','submission','readback','loss','timeout','traversal']){
  const mock=fakeGpu(kind),session=await createGpuField(empty(),{gpu:mock.gpu,timeoutMs:20});
  const result=session.ok?await session.query(new Float64Array([0,0,0,0])):session;
  assert.equal(result.ok,false,kind);assert.equal('flags' in result,false,kind);
  assert.equal(result.error.code,({device:'device-creation',pipeline:'shader',allocation:'upload',loss:'device-lost'})[kind]||kind,kind+' preserves failure cause');
  if(session.ok){assert.equal(session.disposed,true);assert.equal((await session.query(new Float64Array())).ok,false);}
  assert.equal(mock.counts.created,mock.counts.destroyed,kind+' cleanup');
  if(kind!=='device')assert.equal(mock.counts.deviceDestroyed,1,kind+' device cleanup');
  failures.push({kind,result,counts:mock.counts,observation:'controlled API double'});
}
const healthy=fakeGpu(),session=await createGpuField(empty(),{gpu:healthy.gpu});
assert.equal(session.ok,true);assert.match(session.metadata.timestampStatus,/unavailable/);
assert.equal((await session.query(new Float64Array([NaN,0,0,0]))).error.code,'invalid-input');
assert.equal((await session.query(new Float64Array([0,0,0,0]))).ok,true,'invalid input leaves session reusable');
session.dispose();session.dispose();assert.equal(healthy.counts.deviceDestroyed,1);assert.equal(healthy.counts.created,healthy.counts.destroyed);
assert.equal((await session.query(new Float64Array())).error.code,'disposed');
const late=fakeGpu();const request=late.gpu.requestAdapter;
late.gpu.requestAdapter=async()=>{const adapter=await request();const create=adapter.requestDevice;adapter.requestDevice=async()=>{await new Promise(resolve=>setTimeout(resolve,20));return create();};return adapter;};
assert.equal((await createGpuField(empty(),{gpu:late.gpu,timeoutMs:5})).error.code,'timeout');
await new Promise(resolve=>setTimeout(resolve,30));assert.equal(late.counts.deviceDestroyed,1,'late device creation is destroyed after timeout');
console.log(JSON.stringify({controlledFailureTests:failures}));

if(process.argv.includes('--hardware')) {
  const {pathToFileURL}=await import('node:url');const {writeFile}=await import('node:fs/promises');
  const {chromium}=await import(process.env.PLAYWRIGHT_MODULE?pathToFileURL(process.env.PLAYWRIGHT_MODULE).href:'playwright');
  const args=['--enable-gpu','--use-gl=angle','--use-angle=vulkan','--enable-features=Vulkan','--disable-vulkan-surface','--enable-unsafe-webgpu','--ozone-platform=x11'];
  const browser=await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE||'/usr/bin/chromium',headless:true,args});
  try {
    const page=await browser.newPage(),url=process.env.BROWSER_URL||'http://127.0.0.1:5188';
    await page.route(url+'/',r=>r.fulfill({contentType:'text/html',body:'<!doctype html><canvas></canvas>'}));await page.goto(url);
    await page.evaluate(()=>document.querySelector('canvas').getContext('webgl2'));await page.waitForTimeout(500);
    const actual=await page.evaluate(async()=>{
      const {createGpuField}=await import('/scripts/benchmarks/generation-gpu.mjs');
      const check=(value,message)=>{if(!value)throw Error(message);};
      const empty=()=>({schema:1,wood:new Float64Array(),woodIndex:{nodeCount:0,bounds:new Float64Array(),topology:new Uint32Array()},leaves:{nodeCount:0,bounds:new Float64Array(),topology:new Uint32Array()}});
      const s=empty();
      s.wood=new Float64Array([0,0,0,0,2,0,1,.5]);
      s.woodIndex={nodeCount:1,bounds:new Float64Array([-1,-1,-1,1,2.5,1,-1,-1,-1,1,2.5,1]),topology:new Uint32Array([0,1,0xffffffff,0xffffffff,0])};
      s.leaves={nodeCount:1,bounds:new Float64Array([3,0,0,4,1,1,3,0,0,4,1,1]),topology:new Uint32Array([0,1,0xffffffff,0xffffffff,0])};
      const gpu=await createGpuField(s,{timestamps:false});check(gpu.ok,JSON.stringify(gpu));
      const cells=new Float64Array([0,0,0,0,1,0,0,0,3,0,0,0,4,1,1,0,5,5,5,0,1e100,1e100,1e100,0]);
      const first=await gpu.query(cells),repeat=await gpu.query(cells);
      check(first.ok&&repeat.ok,JSON.stringify(first));check(first.flags.join(',')==='1,1,2,2,0,0','wood/foliage, point, contact and outside outputs');
      check(first.flags.join(',')===repeat.flags.join(','),'deterministic repeat');check(first.outsideRangeCount===1,'large coordinate prefilter');
      check(first.timings.gpuMs===null,'optional timestamps disabled');
      const emptyBatch=await gpu.query(new Float64Array());check(emptyBatch.ok&&emptyBatch.flags.length===0,'empty batch');
      const invalid=await gpu.query(new Float64Array([0,0,0,-1]));check(!invalid.ok&&!('flags'in invalid),'invalid input whole failure');
      check((await gpu.query(cells)).ok,'reusable after input error');
      gpu.dispose();gpu.dispose();const disposed=await gpu.query(cells);check(!disposed.ok&&disposed.error.code==='disposed','disposed complete failure');
      const emptyField=await createGpuField(empty());check(emptyField.ok,JSON.stringify(emptyField));
      const emptyFlags=await emptyField.query(cells);check(emptyFlags.ok&&emptyFlags.flags.every(v=>v===0),'empty field occupancy');emptyField.dispose();
      let externalDevice;
      const lossGpu={async requestAdapter(){const adapter=await navigator.gpu.requestAdapter();if(!adapter)return null;return {limits:adapter.limits,features:adapter.features,info:adapter.info,async requestDevice(options){externalDevice=await adapter.requestDevice(options);return externalDevice;}};}};
      const doomed=await createGpuField(empty(),{gpu:lossGpu});check(doomed.ok,JSON.stringify(doomed));
      externalDevice.destroy();await externalDevice.lost;await new Promise(resolve=>setTimeout(resolve,0));
      const lost=await doomed.query(cells);check(!lost.ok&&lost.error.code==='device-lost'&&doomed.disposed,'actual device destruction loss');
      return {passed:true,adapter:gpu.metadata,flags:Array.from(first.flags),outsideRangeCount:first.outsideRangeCount,invalid,disposed,lost,observation:'actual WebGPU shader execution, optional timestamps disabled, actual explicit device.destroy loss; unsupported adapter and other failures are controlled doubles'};
    });
    const receipt={recordedAt:new Date().toISOString(),browser:browser.version(),args,helperTests:{passed:true,coverage:['missing navigator.gpu','null adapter','unavailable timestamp feature (controlled)','empty and malformed snapshots','invalid/nonfinite/negative/overflow cells','overlapping f32-unsafe cells','buffer and dispatch limits','late device allocation timeout cleanup','idempotent disposal and post-input-failure reuse']},controlledFailures:failures,actual};
    await writeFile(process.env.GENERATION_TEST_OUTPUT||'/tmp/fn12-gpu-tests.json',JSON.stringify(receipt,null,2)+'\n');console.log('hardware GPU tests passed');
  }finally{await browser.close();}
}
