import test from 'node:test';
import assert from 'node:assert/strict';
import { analyzeGaps, convergence, rasterizeHull, validateMatch, targetSupport } from './geometry-benchmark-diagnostics.mjs';
const identity = { protocol_sha256: 'p', source_sha256: 's', camera_sha256: 'c', roi_sha256: 'r', width: 9, height: 9 };
function fixture() {
  const roi = new Uint8Array(81), coverage = new Float32Array(81);
  for (let y=1;y<8;y++) for(let x=1;x<8;x++) roi[y*9+x]=coverage[y*9+x]=1;
  return { width:9,height:9,roi,coverage,metresPerPixel:.1 };
}
test('R2/R3: projected holes, openings and excluded background retain pixel and square-metre units',()=>{
  const f=fixture(); f.coverage[40]=0; f.coverage[9+4]=0;
  const r=analyzeGaps(f).primary;
  assert.equal(r.roi_pixels,49); assert.equal(r.enclosed_pixels,1);assert.equal(r.exterior_pixels,1);
  assert.equal(r.hole_count,1); assert.equal(r.largest_hole_m2,.01); assert.equal(r.outside_roi_pixels,0);
  assert.equal(r.height_bands.length,4);
});
test('R2: diagonal gap connectivity and threshold closure are diagnostic, never biological approval',()=>{
 const f=fixture(); f.coverage[40]=0;f.coverage[30]=0;f.coverage[31]=.4;
 const r=analyzeGaps(f); assert.equal(r.sensitivity[0].hole_count,2);assert.equal(r.primary.hole_count,1);
 assert.equal(r.biological_assessment,'unassessed');assert.equal(r.favorable_direction,null);
});
test('R2/R4: empty, clipped, nonfinite and malformed masks fail without a favorable zero',()=>{
 for(const mutate of [f=>f.coverage.fill(0),f=>f.roi[0]=1,f=>f.coverage[40]=NaN,f=>f.coverage=new Float32Array(1),f=>f.roi.fill(0)]) {
  const f=fixture();mutate(f);assert.throws(()=>analyzeGaps(f));
 }
 assert.throws(()=>rasterizeHull([[0,2],[3,2],[3,5]],9,9),/clipped/);
 assert.throws(()=>rasterizeHull([[2,2],[3,3],[4,4]],9,9),/empty/);
 const roi=rasterizeHull([[1,1],[8,1],[8,8],[1,8],[4,4]],9,9);assert.equal(roi.raster.reduce((a,b)=>a+b),49);
});
test('R4: convergence uses complete arrays and partial tiles, rejecting missing/nonfinite samples',()=>{
 const b=new Float32Array(17*17*3),c=new Float32Array(17*17);assert.equal(convergence(b,b,c,c,17,17).status,'measured');
 const changed=c.slice();changed[288]=.1;assert.equal(convergence(b,b,c,changed,17,17).status,'failed');
 assert.throws(()=>convergence(b,b,c,new Float32Array(0),17,17));b[0]=NaN;assert.throws(()=>convergence(b,b,c,c,17,17));
});
test('R4: protocol/source/camera/ROI/crop identities and required artifacts fail closed',()=>{
 assert.doesNotThrow(()=>validateMatch(identity,identity));
 for(const key of Object.keys(identity))assert.throws(()=>validateMatch(identity,{...identity,[key]:'changed'}));
 assert.throws(()=>validateMatch(identity,{...identity,camera_sha256:null}));
});
test('R5: inventory is extensible but a third taxon cannot inherit another generator',()=>{
 const species={id:'new-fir',preset:'new-fir',profile_id:'new-fir',required_capabilities:['woody-axes','fork']};
 assert.equal(targetSupport(species,[]).status,'unsupported-anatomy');
 assert.equal(targetSupport(species,[{id:'oak',profile_id:'oak',capabilities:['woody-axes','fork']}]).status,'unsupported-anatomy');
 assert.equal(targetSupport(species,[{id:'new-fir',profile_id:'new-fir',capabilities:['woody-axes','fork']}]).status,'supported');
});

import { mkdtemp, writeFile, mkdir, symlink } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { artifact, contained, verifyArtifacts, verifyCaptureArtifacts } from './geometry-benchmark.mjs';
test('R4: missing masks, incomplete artifacts, changed bytes and path escapes cannot pass',async()=>{
 const root=await mkdtemp(join(tmpdir(),'fn19-artifact-control-'));
 await writeFile(join(root,'mask.f32'),Buffer.from([1,2,3,4]));const receipt=await artifact(root,'mask.f32','application/octet-stream');
 await verifyArtifacts(root,[receipt]);await writeFile(join(root,'mask.f32'),Buffer.from([4,3,2,1]));
 await assert.rejects(verifyArtifacts(root,[receipt]),/mismatch/);await assert.rejects(verifyArtifacts(root,[]),/missing/);
 await assert.rejects(verifyCaptureArtifacts(root,{artifacts:[]}),/required/);
 await mkdir(join(root,'sub'));await symlink(tmpdir(),join(root,'sub','escape'));
 for(const path of ['../escape','/tmp/escape','sub/escape/no-file'])await assert.rejects(contained(root,path),/path-escape/);
 const all=['beauty','coverage'].flatMap(channel=>['native','64','128'].flatMap(sample=>['png','f32'].map(ext=>({path:'case/'+channel+'-'+sample+'.'+ext}))));
 for(const suffix of ['coverage-128.f32','beauty-native.png'])await assert.rejects(verifyCaptureArtifacts(root,{artifacts:all.filter(a=>!a.path.endsWith(suffix))}),/required/);
});

import { verifyConditionsIdentity } from './geometry-benchmark.mjs';
test('R4: the actual replay preflight rejects a changed camera, source, protocol or reference receipt',()=>{
 const expected={protocol_sha256:'p',references_sha256:'r',manifest_sha256:'m',conditions_sha256:'c'};
 const provenance={...expected,source:{sha256:'s'}};const conditions={source_sha256:'s'};
 assert.doesNotThrow(()=>verifyConditionsIdentity(expected,provenance,conditions));
 for(const field of Object.keys(expected))assert.throws(()=>verifyConditionsIdentity({...expected,[field]:'changed'},provenance,conditions),/mismatch/);
 assert.throws(()=>verifyConditionsIdentity(expected,provenance,{source_sha256:'other'}),/source-mismatch/);
});

import { terminalProcessFailure } from './geometry-benchmark.mjs';
test('R4: timeout, missing hardware and interrupted subprocesses retain distinct nonpassing dispositions',()=>{
 assert.deepEqual(terminalProcessFailure({expired:true,text:''}),{capture_status:'fail',reason:'timeout'});
 assert.equal(terminalProcessFailure({expired:false,text:'browserType.launch: Executable does not exist'}).capture_status,'unavailable');
 assert.match(terminalProcessFailure({expired:false,text:'source-mismatch: served rig'}).reason,/source-mismatch/);
 assert.equal(terminalProcessFailure({expired:false,text:'crashed'}).capture_status,'fail');
});

import { validateCaptureRules } from './geometry-benchmark.mjs';
test('R4 capture rules compare object values independent of key order but preserve arrays and changed values',()=>{
 const frozen={render:{width_px:1600,nested:{a:1,b:2},samples:[64,128]},required_views:[{id:'whole',azimuth_deg:[0,90]}]};
 const reordered={render:{samples:[64,128],nested:{b:2,a:1},width_px:1600},required_views:[{azimuth_deg:[0,90],id:'whole'}]};
 assert.doesNotThrow(()=>validateCaptureRules(reordered,frozen));
 for(const mutate of [p=>p.render.width_px=800,p=>p.render.samples.reverse(),p=>p.required_views[0].azimuth_deg.reverse(),p=>delete p.render.nested]){const changed=structuredClone(reordered);mutate(changed);assert.throws(()=>validateCaptureRules(changed,frozen),/unsupported capture rules/);}
});
