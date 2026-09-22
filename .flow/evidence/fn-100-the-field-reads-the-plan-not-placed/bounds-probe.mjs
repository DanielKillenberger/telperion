import { readFileSync } from 'node:fs';
const root = new URL('file:///home/daniel/Projects/telperion/');
const { instance } = await WebAssembly.instantiate(readFileSync(new URL('src/browser/telperion.wasm', root)), { env: { now: () => performance.now() } });
const x = instance.exports;
const meta = () => JSON.parse(Buffer.from(x.memory.buffer, x.metadata_ptr(), x.metadata_len()).toString());
for (const id of ['oregon-white-oak','silver-birch','norway-spruce']) {
  const request = Buffer.from(JSON.stringify({ family: id, outputs: { structure: true } }));
  if (x.request_alloc(request.length)) throw Error(JSON.stringify(meta()));
  new Uint8Array(x.memory.buffer, x.request_ptr(), request.length).set(request);
  if (x.build()) throw Error(JSON.stringify(meta()));
  const v = new Float64Array(x.memory.buffer, x.buffer_ptr(6), x.buffer_len(6));
  const t = new Uint32Array(x.memory.buffer, x.buffer_ptr(7), x.buffer_len(7));
  const n = t.length/3;
  const min=[1e9,1e9,1e9], max=[-1e9,-1e9,-1e9], argmin=[0,0,0], argmax=[0,0,0];
  for (let i=0;i<n;i++) for (let a=0;a<3;a++){ const p=v[i*6+a]; if(p<min[a]){min[a]=p;argmin[a]=i;} if(p>max[a]){max[a]=p;argmax[a]=i;} }
  const kinds = {}; for (let i=0;i<n;i++) kinds[t[i*3+2]]=(kinds[t[i*3+2]]||0)+1;
  console.log(id, 'nodes', n, 'kinds', JSON.stringify(kinds));
  console.log('  min', min.map(f=>f.toFixed(2)), 'max', max.map(f=>f.toFixed(2)), 'extent', max.map((m,a)=>(m-min[a]).toFixed(2)));
  for (const a of [0,1,2]) for (const i of [argmin[a],argmax[a]]) console.log(`  axis${a} node ${i} pos`, [0,1,2].map(b=>v[i*6+b].toFixed(2)), 'r', v[i*6+3].toFixed(4), v[i*6+4].toFixed(4), 'kind', t[i*3+2], 'branch', t[i*3+1], 'parent', t[i*3]);
  // height histogram of node count by y decile
  const h=new Array(10).fill(0); for(let i=0;i<n;i++){h[Math.min(9,Math.floor((v[i*6+1]-min[1])/(max[1]-min[1])*10))]++;} console.log('  y-decile counts', h.join(' '));
  console.log('  meta', JSON.stringify(meta()).slice(0,600));
}
