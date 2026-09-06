// Experiment only: f32 traversal is approximate until every reference flag agrees.
const NIL = 0xffffffff, WORKGROUP = 64, STACK = 64;
const F32_SAFE = Math.sqrt(3.4028234663852886e38 / 256);
const F64_SAFE = Math.sqrt(Number.MAX_VALUE / 64);
const now = () => performance.now();
function fail(code,message=code) { const error = Error(message); error.code=code; throw error; }
const outcome = (error,stage) => ({ok:false,error:{code:error.code || stage,message:String(error.message || error)},stage});
const safeFloat = v => Number.isFinite(v) && Math.abs(v)<=F32_SAFE;

export function prepareSnapshot(s) {
  const start=now();
  if(s?.schema!==1 || !(s.wood instanceof Float64Array) || s.wood.length%8) fail('invalid-snapshot');
  for(let i=0;i<s.wood.length;i++) if(!safeFloat(s.wood[i]) || (i%8>=6 && s.wood[i]<0)) fail('invalid-snapshot','invalid snapshot wood or precision range');
  let validationScratchBytes=0;
  for(const [index,isWood] of [[s.woodIndex,true],[s.leaves,false]]) {
    if(!index || !(index.bounds instanceof Float64Array) || !(index.topology instanceof Uint32Array)) fail('invalid-snapshot');
    const {bounds:b,topology:t,nodeCount:n}=index, items=b.length/6-n;
    if(!Number.isInteger(n)||n<0||!Number.isInteger(items)||items<0||t.length!==n*4+items||(n===0)!==(items===0)||(isWood&&items!==s.wood.length/8)) fail('invalid-snapshot');
    for(let i=0;i<b.length;i++) if(!safeFloat(b[i]) || (i%6<3&&b[i]>b[i+3])) fail('invalid-snapshot','invalid snapshot bounds or precision range');
    const depths=new Uint8Array(n), parents=new Uint8Array(n); validationScratchBytes+=2*n;
    for(let node=0;node<n;node++) {
      const [lo,hi,left,right]=t.subarray(node*4,node*4+4);
      if(lo>=hi||hi>items||(node===0&&(lo!==0||hi!==items))||(node>0&&parents[node]!==1)) fail('invalid-snapshot','invalid snapshot range or topology');
      if(left===NIL&&right===NIL) continue;
      if(left<=node||right<=node||left>=n||right>=n||left===right||depths[node]>=STACK-2) fail('invalid-snapshot','invalid snapshot cycle, children or depth');
      if(t[left*4]!==lo||t[left*4+1]!==t[right*4]||t[right*4+1]!==hi) fail('invalid-snapshot','invalid snapshot partition');
      for(const child of [left,right]) { if(++parents[child]!==1) fail('invalid-snapshot'); depths[child]=depths[node]+1; }
    }
    if(isWood) for(let i=n*4;i<t.length;i++) if(t[i]>=items) fail('invalid-snapshot','invalid snapshot primitive');
  }
  const validationMs=now()-start, packingStart=now();
  const arrays=[new Float32Array(s.wood),new Float32Array(s.woodIndex.bounds),s.woodIndex.topology.slice(),new Float32Array(s.leaves.bounds),s.leaves.topology.slice()];
  return {arrays,woodNodes:s.woodIndex.nodeCount,leafNodes:s.leaves.nodeCount,
    roots:[s.woodIndex,s.leaves].filter(i=>i.nodeCount).map(i=>Array.from(i.bounds.subarray(0,6))),
    sourceBytes:s.wood.byteLength+s.woodIndex.bounds.byteLength+s.woodIndex.topology.byteLength+s.leaves.bounds.byteLength+s.leaves.topology.byteLength,
    packedBytes:arrays.reduce((n,a)=>n+a.byteLength,0),validationScratchBytes,validationMs,packingMs:now()-packingStart,totalMs:now()-start};
}
export function packQueries(cells,roots) {
  if(!(cells instanceof Float64Array)||cells.length%4) fail('invalid-input','invalid packed input');
  const packed=new Float32Array(cells.length); let outsideRangeCount=0;
  for(let i=0;i<cells.length;i+=4) {
    const x=cells[i],y=cells[i+1],z=cells[i+2],h=cells[i+3],r=h*Math.sqrt(3);
    if(![x,y,z,h].every(Number.isFinite)||h<0) fail('invalid-input','invalid query input');
    if([x,y,z].some(v=>Math.abs(v-r)>F64_SAFE||Math.abs(v+r)>F64_SAFE)) fail('coordinate-overflow');
    if(![x,y,z,r].every(safeFloat)) {
      const p=[x,y,z];
      if(!roots || roots.some(b=>p.every((v,a)=>b[a]<=v+r&&b[a+3]>=v-r))) fail('precision-range','precision-range: overlapping query exceeds safe f32 arithmetic');
      packed[i+3]=-1; outsideRangeCount++; continue;
    }
    packed[i]=x;packed[i+1]=y;packed[i+2]=z;packed[i+3]=h;
  }
  return {packed,count:cells.length/4,outsideRangeCount};
}
export function checkLimits(limits,sizes,count) {
  if(sizes.some(n=>!Number.isSafeInteger(n)||n<0||Math.max(4,n)>limits.maxBufferSize||Math.max(4,n)>limits.maxStorageBufferBindingSize) || Math.ceil(count/WORKGROUP)>limits.maxComputeWorkgroupsPerDimension || limits.maxComputeInvocationsPerWorkgroup<WORKGROUP || limits.maxComputeWorkgroupSizeX<WORKGROUP || limits.maxStorageBuffersPerShaderStage<7) fail('exceeded-limits','GPU buffer or dispatch limit exceeded');
}
export async function guarded(operation,{timeoutMs=30000,lost,onFailure=()=>{}}={}) {
  let timer;
  try {
    return await Promise.race([Promise.resolve().then(operation),new Promise((_,reject)=>{timer=setTimeout(()=>{const e=Error('GPU timeout');e.code='timeout';reject(e);},timeoutMs);}),...(lost?[lost.then(info=>{const e=Error('GPU device lost: '+info.message);e.code='device-lost';throw e;})]:[])]);
  } catch(error) { onFailure(); throw error; }
  finally { clearTimeout(timer); }
}

export const shader = `
struct Params { count:u32, woodNodes:u32, leafNodes:u32, pad:u32 }
@group(0) @binding(0) var<storage,read> wood:array<f32>;
@group(0) @binding(1) var<storage,read> wb:array<f32>;
@group(0) @binding(2) var<storage,read> wt:array<u32>;
@group(0) @binding(3) var<storage,read> lb:array<f32>;
@group(0) @binding(4) var<storage,read> lt:array<u32>;
@group(0) @binding(5) var<storage,read> cells:array<vec4f>;
@group(0) @binding(6) var<storage,read_write> flags:array<u32>;
@group(0) @binding(7) var<uniform> params:Params;
fn boundValue(isWood:bool,i:u32)->f32 { if(isWood){return wb[i];} return lb[i]; }
fn topo(isWood:bool,i:u32)->u32 { if(isWood){return wt[i];} return lt[i]; }
fn overlap(isWood:bool,n:u32,p:vec3f,r:f32)->bool {
  for(var a=0u;a<3u;a++) { if(boundValue(isWood,n*6u+a)>p[a]+r || boundValue(isWood,n*6u+a+3u)<p[a]-r){return false;} }
  return true;
}
fn contains(id:u32,p:vec3f,inflation:f32)->bool {
  let i=id*8u; let a0=vec3f(wood[i],wood[i+1u],wood[i+2u]);
  let d=vec3f(wood[i+3u],wood[i+4u],wood[i+5u])-a0; let q=p-a0;
  let r=wood[i+6u]+inflation; let dr=wood[i+7u]-wood[i+6u];
  let a=(d.x*d.x+d.y*d.y+d.z*d.z)-dr*dr;
  let b=(q.x*d.x+q.y*d.y+q.z*d.z)+r*dr;
  var t=0.0; if(a>0.0){t=clamp(b/a,0.0,1.0);}else if(b>a*0.5){t=1.0;}
  let e=q-d*t; let radius=r+dr*t;
  return e.x*e.x+e.y*e.y+e.z*e.z<=radius*radius;
}
fn query(isWood:bool,p:vec3f,r:f32,nodes:u32)->u32 {
  if(nodes==0u){return 0u;}
  var stack:array<u32,64>; stack[0]=0u; var size=1u; var visits=0u;
  while(size>0u){
    size--; let n=stack[size]; visits++;
    if(n>=nodes||visits>nodes){return 4u;}
    if(!overlap(isWood,n,p,r)){continue;}
    let left=topo(isWood,n*4u+2u);
    if(left!=0xffffffffu){
      if(size+2u>64u){return 4u;}
      stack[size]=topo(isWood,n*4u+3u);stack[size+1u]=left;size+=2u;continue;
    }
    let end=topo(isWood,n*4u+1u);
    for(var item=topo(isWood,n*4u);item<end;item++){
      if(overlap(isWood,nodes+item,p,r)){
        if(!isWood){return 1u;}
        if(contains(topo(true,nodes*4u+item),p,r)){return 1u;}
      }
    }
  }
  return 0u;
}
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id:vec3u){
  if(id.x>=params.count){return;}
  let cell=cells[id.x]; if(cell.w<0.0){flags[id.x]=0u;return;}
  let w=query(true,cell.xyz,cell.w*sqrt(3.0),params.woodNodes);
  let l=query(false,cell.xyz,cell.w,params.leafNodes);
  flags[id.x]=w|(l<<1u);
}`;

// A session owns its device and buffers; failures destroy the entire session.
export async function createGpuField(snapshot,{gpu=globalThis.navigator?.gpu,timestamps=true,timeoutMs=30000}={}) {
  const start=now(), resources=[]; let device,disposed=false,stage='preparation';
  const dispose=()=>{if(disposed)return;disposed=true;for(const r of resources)r.destroy();device?.destroy();};
  try {
    const prepared=prepareSnapshot(snapshot);
    stage='unavailable-adapter';
    const adapter=await guarded(()=>gpu?.requestAdapter({powerPreference:'high-performance'}),{timeoutMs});
    if(!adapter) fail('unavailable-adapter');
    checkLimits(adapter.limits,prepared.arrays.map(a=>a.byteLength),0);
    const hasTimestamps=timestamps&&adapter.features.has('timestamp-query');
    const requiredLimits={maxStorageBufferBindingSize:Math.max(134217728,...prepared.arrays.map(a=>a.byteLength)),maxBufferSize:Math.max(268435456,...prepared.arrays.map(a=>a.byteLength))};
    stage='device-creation';
    device=await guarded(async()=>{const created=await adapter.requestDevice({requiredFeatures:hasTimestamps?['timestamp-query']:[],requiredLimits});if(disposed)created.destroy();return created;},{timeoutMs});
    let lostInfo=null,uncaptured=null;
    device.lost.then(info=>{if(!disposed){lostInfo=info;dispose();}});
    device.addEventListener('uncapturederror',event=>{uncaptured=event.error;dispose();});
    const wait=fn=>guarded(fn,{timeoutMs,lost:device.lost,onFailure:dispose});
    const scoped=async fn=>{
      device.pushErrorScope('out-of-memory');device.pushErrorScope('internal');device.pushErrorScope('validation');
      let value,problem;
      try{value=await fn();}catch(e){problem=e;}
      for(let i=0;i<3;i++){try{const e=await wait(()=>device.popErrorScope());if(e&&!problem)problem=e;}catch(e){if(!problem)problem=e;}}
      if(problem)throw problem;if(uncaptured)throw uncaptured;return value;
    };
    const makeBuffer=(size,usage,label)=>{const b=device.createBuffer({size:Math.max(4,size),usage,label});resources.push(b);return b;};
    stage='shader';const setupStart=now();
    const pipeline=await scoped(async()=>{
      const module=device.createShaderModule({code:shader});
      const info=await wait(()=>module.getCompilationInfo());
      if(info.messages.some(m=>m.type==='error'))fail('shader',info.messages.map(m=>m.message).join('\n'));
      return wait(()=>device.createComputePipelineAsync({layout:'auto',compute:{module,entryPoint:'main'}}));
    });
    const setupMs=now()-setupStart,uploadStart=now();stage='upload';
    const source=await scoped(async()=>{
      const buffers=prepared.arrays.map((a,i)=>{const b=makeBuffer(a.byteLength,GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_DST,'field-'+i);if(a.byteLength)device.queue.writeBuffer(b,0,a);return b;});
      await wait(()=>device.queue.onSubmittedWorkDone());return buffers;
    });
    const gpuSourceBytes=prepared.arrays.reduce((n,a)=>n+Math.max(4,a.byteLength),0);
    const timestampStatus=hasTimestamps?'available':timestamps?'unavailable: adapter lacks timestamp-query':'disabled for capability test';
    const memory={sourceBytes:prepared.sourceBytes,packedBytes:prepared.packedBytes,validationScratchBytes:prepared.validationScratchBytes,gpuSourceBytes,
      uploadStagingUpperBoundBytes:prepared.packedBytes,note:'writeBuffer internal staging is opaque; upper bound is payload bytes, not measured driver allocation. Native field/Wasm and browser overhead reported by runner.'};
    const preparation={validationMs:prepared.validationMs,packingMs:prepared.packingMs,adapterDeviceMs:setupStart-start-prepared.totalMs,setupMs,uploadMs:now()-uploadStart,totalMs:now()-start};
    const metadata={info:Object.fromEntries(['vendor','architecture','device','description','isFallbackAdapter'].map(k=>[k,adapter.info[k]])),features:[...adapter.features],requiredLimits,
      adapterLimits:Object.fromEntries(['maxBufferSize','maxStorageBufferBindingSize','maxComputeWorkgroupsPerDimension','maxComputeInvocationsPerWorkgroup','maxComputeWorkgroupSizeX','maxStorageBuffersPerShaderStage'].map(k=>[k,adapter.limits[k]])),timestampStatus};
    prepared.arrays.length=0;
    let busy=false;
    return {ok:true,metadata,memory,preparation,dispose,get disposed(){return disposed;},async query(cells){
      if(disposed)return outcome({code:lostInfo?'device-lost':'disposed',message:lostInfo?.message||'GPU field disposed'},'lifecycle');
      if(busy)return outcome({code:'busy',message:'GPU field query already in progress'},'lifecycle');
      busy=true;const queryStart=now(),transient=[];let queryStage='input',reply;
      const buffer=(size,usage,label)=>{const b=makeBuffer(size,usage,label);transient.push(b);return b;};
      try{
        const input=packQueries(cells,prepared.roots),packingMs=now()-queryStart;
        checkLimits(device.limits,[input.packed.byteLength,input.count*4],input.count);
        if(input.count===0)return reply={ok:true,flags:new Uint8Array(),timings:{packingMs,totalMs:now()-queryStart,submitToCopiedMs:0,gpuMs:null},outsideRangeCount:0,memory:{querySourceBytes:0,queryPackedBytes:0,gpuQueryBytes:0,readbackBytes:0,resultBytes:0}};
        queryStage='allocation';let submitToCopiedMs,gpuMs=null;
        const flags=await scoped(async()=>{
          const cb=buffer(input.packed.byteLength,GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_DST,'cells');
          const result=buffer(input.count*4,GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_SRC,'results');
          const read=buffer(input.count*4,GPUBufferUsage.MAP_READ|GPUBufferUsage.COPY_DST,'readback');
          const uniform=buffer(16,GPUBufferUsage.UNIFORM|GPUBufferUsage.COPY_DST,'params');
          let queries,resolve,timingRead;
          if(hasTimestamps){queries=device.createQuerySet({type:'timestamp',count:2});resources.push(queries);transient.push(queries);resolve=buffer(16,GPUBufferUsage.QUERY_RESOLVE|GPUBufferUsage.COPY_SRC,'timestamp-resolve');timingRead=buffer(16,GPUBufferUsage.MAP_READ|GPUBufferUsage.COPY_DST,'timestamp-read');}
          const bind=device.createBindGroup({layout:pipeline.getBindGroupLayout(0),entries:[...source,cb,result,uniform].map((b,binding)=>({binding,resource:{buffer:b}}))});
          const submitStart=now();queryStage='submission';
          device.queue.writeBuffer(cb,0,input.packed);device.queue.writeBuffer(uniform,0,new Uint32Array([input.count,prepared.woodNodes,prepared.leafNodes,0]));
          const encoder=device.createCommandEncoder();
          const pass=encoder.beginComputePass(hasTimestamps?{timestampWrites:{querySet:queries,beginningOfPassWriteIndex:0,endOfPassWriteIndex:1}}:{});
          pass.setPipeline(pipeline);pass.setBindGroup(0,bind);pass.dispatchWorkgroups(Math.ceil(input.count/WORKGROUP));pass.end();
          encoder.copyBufferToBuffer(result,0,read,0,input.count*4);
          if(hasTimestamps){encoder.resolveQuerySet(queries,0,2,resolve,0);encoder.copyBufferToBuffer(resolve,0,timingRead,0,16);}
          device.queue.submit([encoder.finish()]);queryStage='readback';
          await wait(()=>Promise.all([read.mapAsync(GPUMapMode.READ),...(hasTimestamps?[timingRead.mapAsync(GPUMapMode.READ)]:[])]));
          const values=new Uint32Array(read.getMappedRange()),copy=new Uint8Array(input.count);
          for(let i=0;i<values.length;i++){if(values[i]>3)fail('traversal','GPU traversal failed; no partial result');copy[i]=values[i];}
          read.unmap();
          if(hasTimestamps){const t=new BigUint64Array(timingRead.getMappedRange());if(t[1]<t[0])fail('timestamp','nonmonotonic GPU timestamp');gpuMs=Number(t[1]-t[0])/1e6;timingRead.unmap();}
          submitToCopiedMs=now()-submitStart;return copy;
        });
        if(disposed)fail(lostInfo?'device-lost':'disposed');
        return reply={ok:true,flags,outsideRangeCount:input.outsideRangeCount,timings:{packingMs,submitToCopiedMs,gpuMs,totalMs:now()-queryStart},
          memory:{querySourceBytes:cells.byteLength,queryPackedBytes:input.packed.byteLength,gpuQueryBytes:input.packed.byteLength+input.count*4+16,readbackBytes:input.count*4,timestampBufferBytes:hasTimestamps?32:0,timestampQuerySlots:hasTimestamps?2:0,resultBytes:flags.byteLength,queryUploadStagingUpperBoundBytes:input.packed.byteLength+16}};
      }catch(error){if(queryStage!=='input')dispose();return outcome(error,queryStage);}
      finally{for(const r of transient){r.destroy();const i=resources.indexOf(r);if(i>=0)resources.splice(i,1);}busy=false;if(reply)reply.timings.totalMs=now()-queryStart;}
    }};
  }catch(error){dispose();return outcome(error,stage);}
}
