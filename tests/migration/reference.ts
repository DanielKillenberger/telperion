import {writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {createHash} from 'node:crypto';
import {performance} from 'node:perf_hooks';
import {DEFAULT_ENVELOPE,DEFAULT_RADII,DEFAULT_SURFACE,DEFAULT_CANOPY,DEFAULT_BIAS,DEFAULT_TWIGS,DEFAULT_ELEMENT,DEFAULT_CULL,TELPERION,LAURELIN,createRng,sampleEnvelope,growReport,solveRadii,buildSurface,buildCanopy,buildElement,cullCanopy,envelopeContains} from './src/index';

const output=process.argv[2];
const selected=process.argv.slice(3);
const ordinary={id:'ordinary',skeleton:{envelope:DEFAULT_ENVELOPE,seed:42,attractors:500,step:0.02,bias:DEFAULT_BIAS,twigs:DEFAULT_TWIGS},radii:DEFAULT_RADII,surface:DEFAULT_SURFACE,canopy:DEFAULT_CANOPY};
const cases=[ordinary,TELPERION,LAURELIN,...[
  ['empty',{attractors:0}],
  ['degenerate',{envelope:{...DEFAULT_ENVELOPE,spread:0},attractors:0}],
  ['envelope-crossing',{envelope:{...DEFAULT_ENVELOPE,crownBase:0.8},attractors:64}],
  ['capped',{growth:{maxNodes:20},attractors:500}],
].map(([id,override])=>({...ordinary,id,skeleton:{...ordinary.skeleton,...override}}))];
const hash=(a:ArrayBufferView)=>createHash('sha256').update(new Uint8Array(a.buffer,a.byteOffset,a.byteLength)).digest('hex');
const buffer=(name:string,a:Float64Array|Float32Array|Uint32Array|Int32Array|Uint8Array)=>{
  if(process.env.REFERENCE_BUFFERS==='1')writeFileSync(join(output,name+'.bin'),new Uint8Array(a.buffer,a.byteOffset,a.byteLength));
  return {values:a.length,bytes:a.byteLength,sha256:hash(a)};
};
const manifest=[];
for(const fixture of cases) {
  if(selected.length&&!selected.includes(fixture.id))continue;
  const {skeleton:params}=fixture;const {envelope,seed}=params;
  const cloud=sampleEnvelope(envelope,Math.min(params.attractors,64),createRng(seed));
  const start=performance.now();
  const report=growReport(params,fixture.radii);const skeleton=report.skeleton;
  const radii=solveRadii(skeleton,envelope,fixture.radii);
  const structureMs=performance.now()-start;
  const surfaceStart=performance.now();const mesh=buildSurface(skeleton,radii,envelope,fixture.surface);const surfaceMs=performance.now()-surfaceStart;
  const foliageStart=performance.now();
  const placed=buildCanopy(skeleton,radii,envelope,seed,fixture.canopy,params.twigs?.twig);
  const canopy=cullCanopy(placed,buildElement(DEFAULT_ELEMENT),envelope,DEFAULT_CULL);const foliageMs=performance.now()-foliageStart;
  const positions=Float64Array.from(skeleton.nodes.flatMap(n=>[n.position.x,n.position.y,n.position.z]));
  const parents=Int32Array.from(skeleton.nodes.map(n=>n.parent));
  const bounds=[Infinity,Infinity,Infinity,-Infinity,-Infinity,-Infinity];
  let outside=0;for(const n of skeleton.nodes){const p=n.position;for(const [i,v] of [p.x,p.y,p.z].entries()){bounds[i]=Math.min(bounds[i],v);bounds[i+3]=Math.max(bounds[i+3],v);}if(!envelopeContains(envelope,p,1e-6))outside++;}
  const record={id:fixture.id,revision:process.env.REFERENCE_REVISION,parameters:fixture,camera:{position:[envelope.height*1.5,envelope.height*0.7,envelope.height*1.5],target:[0,envelope.height*0.5,0],verticalFov:40,viewport:[1000,1000]},
    diagnostics:{capped:report.capped,levelCapped:report.levelCapped,shed:report.shed,outside,nodes:skeleton.nodes.length,crossover:skeleton.crossover,bounds},
    invariants:{finitePositions:positions.every(Number.isFinite),parentOrder:parents.every((p,i)=>i===0?p===-1:p>=0&&p<i),positiveRadii:radii.radius.every(r=>Number.isFinite(r)&&r>0),taper:radii.startRadius.every((r,i)=>Number.isFinite(r)&&r>=radii.radius[i]),validIndices:mesh.indices.every(i=>i<mesh.positions.length/3),finiteSurface:mesh.positions.every(Number.isFinite),finiteFoliage:canopy.matrices.every(Number.isFinite)},
    timings:{structureMs,surfaceMs,foliageMs,totalMs:performance.now()-start},
    samples:{attractors:cloud.map(p=>[p.x,p.y,p.z]),nodes:skeleton.nodes.filter((_,i)=>i%Math.max(1,Math.floor(skeleton.nodes.length/32))===0).map(n=>({position:[n.position.x,n.position.y,n.position.z],parent:n.parent}))},
    stages:{positions:buffer(fixture.id+'-positions',positions),parents:buffer(fixture.id+'-parents',parents),branchId:buffer(fixture.id+'-branch-id',skeleton.branchId),baseRadius:buffer(fixture.id+'-base-radius',skeleton.baseRadius),endRadius:buffer(fixture.id+'-end-radius',skeleton.endRadius),twig:buffer(fixture.id+'-twig',skeleton.twig),radius:buffer(fixture.id+'-radius',radii.radius),startRadius:buffer(fixture.id+'-start-radius',radii.startRadius),surfacePositions:buffer(fixture.id+'-surface-positions',mesh.positions),surfaceIndices:buffer(fixture.id+'-surface-indices',mesh.indices),foliage:buffer(fixture.id+'-foliage',canopy.matrices)}};
  writeFileSync(join(output,fixture.id+'.json'),JSON.stringify(record,null,2)+'\n');manifest.push({id:fixture.id,revision:record.revision});console.log(fixture.id,record.diagnostics.nodes,record.timings);
}
if(!manifest.length)throw Error('No reference cases selected');
writeFileSync(join(output,'manifest.json'),JSON.stringify(manifest,null,2)+'\n');
