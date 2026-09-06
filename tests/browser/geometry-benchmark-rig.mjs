import { convexHull, targetSupport } from './geometry-benchmark-diagnostics.mjs';

// Browser-only adapter. Imports the installed generator and materializer without
// importing fn13's executable runner or altering the production stage.
export async function createRig(parameters,species) {
  const THREE=await import('/node_modules/.vite/deps/three.js');
  const {TreeEngine}=await import('/src/browser/core.ts');
  const {CATALOGUE}=await import('/src/browser/presets.generated.ts');
  const {materializeTree}=await import('/src/browser/three.ts');
  const {measureSubject}=await import('/harness/stage.ts');
  const registry=CATALOGUE.map(e=>({id:e.id,profile_id:e.id,capabilities:[
    'woody-axes',...({lobedBlade:['lobed-blade'],fourSidedNeedle:['four-sided-needle']}[e.family.element.anatomy]??[]),
    ...({alternate:['alternate-petiole'],radialNeedles:['radial-peg']}[e.family.canopy.attachment]??[]),
    ...(e.family.skeleton.habit.kind==='tiered'?['tiered-secondary']:[])]}));
  const support=targetSupport(species,registry);if(support.status!=='supported')throw Error('unsupported-anatomy: '+JSON.stringify(support));
  const response=await fetch('/src/browser/telperion.wasm');if(!response.ok)throw Error('hardware-unavailable: missing Wasm');
  const wasm=await response.arrayBuffer(),wasmHash=[...new Uint8Array(await crypto.subtle.digest('SHA-256',wasm))].map(x=>x.toString(16).padStart(2,'0')).join('');
  const started=performance.now(),engine=await TreeEngine.create(wasm);
  let output,wasmBytes;
  try {output=engine.build(parameters,{surface:true,foliage:true,structure:true});wasmBytes=engine.memoryBytes;}finally{engine.dispose();}
  const generationMs=performance.now()-started;
  if(!output.diagnostics.complete||output.diagnostics.capped||output.diagnostics.levelCapped||output.diagnostics.attractionCapped)throw Error('resource-cap: incomplete geometry');
  const arrays={wood:output.surface.positions,wood_normals:output.surface.normals,wood_indices:output.surface.indices,foliage:output.foliage.positions,foliage_indices:output.foliage.indices,matrices:output.foliage.matrices,structure:output.structure.values,topology:output.structure.topology};
  const hashes={};
  for(const [name,a]of Object.entries(arrays)){
    for(const x of a)if(!Number.isFinite(x))throw Error('nonfinite: '+name);
    hashes[name]=[...new Uint8Array(await crypto.subtle.digest('SHA-256',a))].map(x=>x.toString(16).padStart(2,'0')).join('');
  }
  const f=output.foliage,anatomy=f.anatomy;if(!anatomy||!f.matrices.length)throw Error('empty-crown: biological subset unavailable');
  const material=new THREE.MeshStandardMaterial({color:0x9d968c,roughness:.92});
  const leafMaterial=new THREE.MeshStandardMaterial({color:0x9d968c,roughness:.92,side:THREE.DoubleSide,flatShading:true,alphaTest:.5});
  const tree=materializeTree(output,{surface:material,element:leafMaterial});
  const bounds=measureSubject(tree);if(bounds.isEmpty())throw Error('nonfinite: empty bounds');
  const canopy=tree.getObjectByName('grower-canopy'),wood=tree.getObjectByName('grower-trunk');
  const scene=new THREE.Scene();scene.background=new THREE.Color(0xc6ced5);scene.add(tree,new THREE.HemisphereLight(0xffffff,0x6a6966,3.1));
  const renderer=new THREE.WebGLRenderer({canvas:document.querySelector('canvas'),antialias:true,preserveDrawingBuffer:true});
  renderer.setSize(1600,1000);renderer.setPixelRatio(1);renderer.toneMapping=THREE.NoToneMapping;renderer.outputColorSpace=THREE.LinearSRGBColorSpace;
  const gl=renderer.getContext(),debug=gl.getExtension('WEBGL_debug_renderer_info');
  const backend=gl.getParameter(debug?debug.UNMASKED_RENDERER_WEBGL:gl.RENDERER);
  const matrix=new THREE.Matrix4(),vertex=new THREE.Vector3();
  const eachBiological=visit=>{for(let i=0;i<f.matrices.length/16;i++){
    matrix.fromArray(f.matrices,i*16);
    for(let j=anatomy.vertices[0];j<anatomy.vertices[1];j++)visit(vertex.fromArray(f.positions,j*3).applyMatrix4(matrix),i);
  }};
  const values=output.structure.values,topology=output.structure.topology,n=topology.length/3;
  const point=i=>new THREE.Vector3().fromArray(values,i*6),root=point(0);
  const children=Array.from({length:n},()=>[]),order=new Uint32Array(n);
  for(let i=1;i<n;i++){const p=topology[i*3];if(p>=i)throw Error('invalid-topology');children[p].push(i);}
  for(let i=0;i<n;i++){
    children[i].sort((a,b)=>values[b*6+4]-values[a*6+4]||a-b);
    for(let j=0;j<children[i].length;j++)order[children[i][j]]=order[i]+(j?1:0);
  }
  const crown=new THREE.Box3();let radius=0,maxUnitLength=0;
  eachBiological(p=>{crown.expandByPoint(p);radius=Math.max(radius,Math.hypot(p.x-root.x,p.z-root.z));});
  const pairs=[];
  for(let a=anatomy.vertices[0];a<anatomy.vertices[1];a++)for(let b=a+1;b<anatomy.vertices[1];b++)pairs.push(new THREE.Vector3().fromArray(f.positions,a*3).sub(new THREE.Vector3().fromArray(f.positions,b*3)));
  const longest=Math.max(...pairs.map(v=>v.length()));
  for(let i=0;i<f.matrices.length;i+=16){
    matrix.fromArray(f.matrices,i);const e=matrix.elements,axes=[new THREE.Vector3(e[0],e[1],e[2]),new THREE.Vector3(e[4],e[5],e[6]),new THREE.Vector3(e[8],e[9],e[10])],scales=axes.map(v=>v.length());
    const uniform=Math.max(...scales)-Math.min(...scales)<1e-6 && Math.abs(axes[0].dot(axes[1]))+Math.abs(axes[0].dot(axes[2]))+Math.abs(axes[1].dot(axes[2]))<1e-6;
    if(uniform)maxUnitLength=Math.max(maxUnitLength,longest*Math.max(...scales));
    else {const linear=new THREE.Matrix3().setFromMatrix4(matrix);for(const pair of pairs)maxUnitLength=Math.max(maxUnitLength,pair.clone().applyMatrix3(linear).length());}
  }
  const normalization={root_m:root.toArray(),crown_min_m:crown.min.toArray(),crown_max_m:crown.max.toArray(),radius_m:radius};
  const corners=[];for(const x of [bounds.min.x,bounds.max.x])for(const y of [bounds.min.y,bounds.max.y])for(const z of [bounds.min.z,bounds.max.z])corners.push(new THREE.Vector3(x,y,z));
  // Spatial buckets index unchanged unit origins for terminal attachment queries.
  const buckets=new Map(),cell=.5;
  const key=(x,y,z)=>`${x},${y},${z}`;
  for(let i=0;i<f.matrices.length/16;i++){const k=i*16,x=Math.floor(f.matrices[k+12]/cell),y=Math.floor(f.matrices[k+13]/cell),z=Math.floor(f.matrices[k+14]/cell),s=key(x,y,z);if(!buckets.has(s))buckets.set(s,[]);buckets.get(s).push(i);}
  function attached(node) {
    const base=point(topology[node*3]),tip=point(node),axis=tip.clone().sub(base),l2=axis.lengthSq();if(l2<1e-18)return [];
    const pad=Math.max(values[node*6+3],values[node*6+4])+.003;
    const lo=base.clone().min(tip).addScalar(-pad),hi=base.clone().max(tip).addScalar(pad),found=[];
    for(let x=Math.floor(lo.x/cell);x<=Math.floor(hi.x/cell);x++)for(let y=Math.floor(lo.y/cell);y<=Math.floor(hi.y/cell);y++)for(let z=Math.floor(lo.z/cell);z<=Math.floor(hi.z/cell);z++)for(const i of buckets.get(key(x,y,z))??[]){
      const p=new THREE.Vector3().fromArray(f.matrices,i*16+12),t=p.clone().sub(base).dot(axis)/l2;
      if(t<0||t>1)continue;const r=values[node*6+4]*(1-t)+values[node*6+3]*t;
      if(p.distanceTo(base.clone().addScaledVector(axis,t))<=r+Math.max(.002,r*.75))found.push(i);
    }
    return found;
  }
  function select(view,norm=normalization) {
    const query=view==='fork'?[.25,.35,.25]:view==='attached-shoot'?[.7,.5,.7]:[0,0,0];
    const candidates=[];
    if(view==='fork')for(let p=1;p<n;p++)if(children[p].length>=2&&children[p].some(i=>order[i]===1&&order[i]!==order[p])){
      const incoming=point(p).distanceTo(point(topology[p*3]));if(incoming<=1e-9||children[p].some(i=>point(i).distanceTo(point(p))<=1e-9))continue;
      candidates.push({p,node:p,nodes:[topology[p*3],p,...children[p]],span:Math.max(1,12*values[p*6+3]*2),units:[]});
    }
    if(view==='attached-shoot')for(let i=1;i<n;i++)if(!children[i].length&&topology[i*3+2]===2&&topology[i*3]>0){
      const units=attached(i);if(units.length)candidates.push({p:topology[i*3],node:i,nodes:[topology[topology[i*3]*3],topology[i*3],i],span:Math.max(.25,2*point(i).distanceTo(point(topology[i*3]))+2*maxUnitLength),units});
    }
    const normalize=p=>[(p.x-norm.root_m[0])/norm.radius_m,(p.y-norm.crown_min_m[1])/(norm.crown_max_m[1]-norm.crown_min_m[1]),(p.z-norm.root_m[2])/norm.radius_m];
    for(const c of candidates){c.position=point(c.p);c.distance=Math.hypot(...normalize(c.position).map((v,i)=>v-query[i]));}
    candidates.sort((a,b)=>a.distance-b.distance||a.position.y-b.position.y||a.position.x-b.position.x||a.position.z-b.position.z||a.node-b.node);
    let c=candidates[0];
    if(view==='base')c={position:root.clone().add(new THREE.Vector3(0,.5,0)),nodes:[0,children[0][0]],span:3,units:[]};
    if(view==='whole'||view==='bare')c={position:bounds.getCenter(new THREE.Vector3()),nodes:[],span:null,units:[]};
    if(!c)throw Error('unresolvable-target: '+view);
    return {target:{rule:view,query_normalized:query,selected_position_m:c.position.toArray(),node_indices:c.nodes,axis_order:view==='fork'?1:order[c.node??0],resolution:'resolved',reason:null},span:c.span,units:c.units};
  }
  function cameraFrom(c) {
    const camera=new THREE.OrthographicCamera(c.left_m,c.right_m,c.top_m,c.bottom_m,c.near_m,c.far_m);
    camera.position.fromArray(c.position_m);camera.up.fromArray(c.up);camera.lookAt(new THREE.Vector3(...c.target_m));camera.updateMatrixWorld(true);return camera;
  }
  function prepare(view,azimuth,fixed) {
    canopy.instanceMatrix=originalInstances;canopy.count=f.matrices.length/16;canopy.frustumCulled=true;
    wood.visible=true;canopy.visible=view!=='bare';canopy.material=leafMaterial;canopy.geometry.setIndex(fullIndex);
    const norm=fixed?.environment?.normalization??normalization,{target,span,units}=select(view,norm);
    let c=fixed?.camera;
    if(!c){
      const direction=new THREE.Vector3(.62,.28,1).normalize().applyAxisAngle(new THREE.Vector3(0,1,0),azimuth*Math.PI/180);
      const right=new THREE.Vector3(0,1,0).cross(direction).normalize(),up=direction.clone().cross(right).normalize();
      const centre=new THREE.Vector3(...target.selected_position_m),projected=corners.map(p=>p.clone().sub(centre));
      const xs=projected.map(p=>p.dot(right)),ys=projected.map(p=>p.dot(up)),zs=projected.map(p=>p.dot(direction));
      const vertical=span??1.3*Math.max(Math.max(...ys)-Math.min(...ys),(Math.max(...xs)-Math.min(...xs))/1.6);
      const distance=Math.max(...zs)+2;
      c={projection:'orthographic',position_m:centre.clone().addScaledVector(direction,distance).toArray(),target_m:centre.toArray(),up:[0,1,0],left_m:-vertical*.8,right_m:vertical*.8,top_m:vertical/2,bottom_m:-vertical/2,near_m:1,far_m:distance-Math.min(...zs)+1,width_px:1600,height_px:1000,dpr:1};
    }
    const camera=cameraFrom(c),project=p=>p.clone().project(camera),inside=p=>{const q=project(p);return Math.abs(q.x)<1&&Math.abs(q.y)<1&&Math.abs(q.z)<1;};
    let framePoints;
    if(view==='whole'||view==='bare')framePoints=corners;
    else if(view==='base')framePoints=[root,new THREE.Vector3(...target.selected_position_m)];
    else framePoints=target.node_indices.map(point);
    if(!framePoints.every(inside))throw Error('clipped-subject: named attachment or whole bounds');
    if(view==='base')for(let i=0;i<output.surface.positions.length;i+=3){vertex.fromArray(output.surface.positions,i);if(vertex.y<=root.y+.5&&!inside(vertex))throw Error('clipped-subject: drawn trunk base');}
    for(const unit of units){matrix.fromArray(f.matrices,unit*16);for(let j=anatomy.vertices[0];j<anatomy.vertices[1];j++)if(!inside(vertex.fromArray(f.positions,j*3).applyMatrix4(matrix)))throw Error('clipped-subject: attached biological unit');}
    let hull=[];
    if(view==='whole'){
      let chunk=[];eachBiological(p=>{const q=project(p);if(Math.abs(q.x)>=1||Math.abs(q.y)>=1||Math.abs(q.z)>=1)throw Error('clipped-subject: projected biological vertex');chunk.push([(q.x+1)*800,(1-q.y)*500]);if(chunk.length>=4096){hull=convexHull([...hull,...chunk]);chunk=[];}});hull=convexHull([...hull,...chunk]);
    }
    const anatomyEvidence={connected_nodes:target.node_indices,retained_unit_indices:units,attachment_in_frame:true,occlusion:'not-applicable'};
    if(view==='fork'||view==='attached-shoot'||view==='base'){
      tree.updateMatrixWorld(true);const focus=new THREE.Vector3(...target.selected_position_m),projectedFocus=focus.clone().project(camera);
      const ray=new THREE.Raycaster();ray.setFromCamera(new THREE.Vector2(projectedFocus.x,projectedFocus.y),camera);ray.far=c.far_m;const old=wood.material.side;wood.material.side=THREE.DoubleSide;
      const hit=ray.intersectObjects([wood,canopy],false)[0];wood.material.side=old;
      const radiusAt=target.node_indices.length?Math.max(...target.node_indices.map(i=>values[i*6+3])):0;
      anatomyEvidence.first_surface_hit_m=hit?.point.toArray()??null;
      anatomyEvidence.first_surface=hit?.object.name??null;
      anatomyEvidence.occlusion=hit?.object===wood&&hit.point.distanceTo(focus)<=Math.max(.02,radiusAt*3)?'selected-neighbourhood-visible':'occluded-target';
      if(anatomyEvidence.occlusion==='occluded-target'){target.resolution='occluded';target.reason='occluded-target: first drawn surface does not expose selected woody neighbourhood';}
    }
    return {camera:c,target,polygon:hull,anatomyEvidence,metres_per_pixel:(c.top_m-c.bottom_m)/1000,environment:{width:1600,height:1000,dpr:1,neutral:true,ground:false,shadows:false,wind:false,LOD:false,normalization:norm,coverage:'biological index subset; original matrices; linear MSAA'},leaf_state:view==='bare'?'hidden':'leaf-on'};
  }
  const white=new THREE.MeshBasicMaterial({color:0xffffff,side:THREE.DoubleSide,toneMapped:false});
  const fullIndex=canopy.geometry.index,biologicalIndex=new THREE.BufferAttribute(f.indices.slice(...anatomy.indices),1);
  const originalInstances=canopy.instanceMatrix;
  function diagnosticInstances(units) {
    const selected=new Float32Array(units.length*16);
    units.forEach((unit,i)=>selected.set(f.matrices.subarray(unit*16,unit*16+16),i*16));
    canopy.instanceMatrix=new THREE.InstancedBufferAttribute(selected,16);canopy.count=units.length;
    canopy.frustumCulled=false;
  }
  function prepareVisibility(view) {
    if(!['fork','attached-shoot'].includes(view))throw Error('unsupported visibility view');
    const selected=select(view),target=selected.target,nodes=target.node_indices;
    const units=view==='attached-shoot'?selected.units:[];
    diagnosticInstances(units);wood.visible=true;canopy.visible=units.length>0;
    canopy.material=leafMaterial;canopy.geometry.setIndex(fullIndex);tree.updateMatrixWorld(true);
    const junction=point(nodes[1]),probes=[],frame=[];
    if(view==='fork') {
      const radius=values[nodes[1]*6+3];
      for(const node of [nodes[0],...nodes.slice(2)]){
        const delta=point(node).sub(junction),length=Math.min(delta.length(),Math.max(.15,radius*5));
        frame.push(junction.clone().addScaledVector(delta.normalize(),length));
        probes.push(junction.clone().addScaledVector(delta,length*.55));
      }
      frame.push(junction);probes.push(junction);
    }else {
      const base=point(nodes[1]),tip=point(nodes[2]),incoming=point(nodes[0]).sub(base);
      frame.push(base,tip,base.clone().addScaledVector(incoming.normalize(),Math.min(.08,base.distanceTo(tip)*.5)));
      for(const t of [.15,.5,.85])probes.push(base.clone().lerp(tip,t));
      for(const unit of units){matrix.fromArray(f.matrices,unit*16);for(let j=0;j<f.positions.length/3;j++)frame.push(new THREE.Vector3().fromArray(f.positions,j*3).applyMatrix4(matrix));}
    }
    const areaNormals=[];
    for(const unit of units){matrix.fromArray(f.matrices,unit*16);for(let j=anatomy.indices[0];j<anatomy.indices[1];j+=3){
      const a=new THREE.Vector3().fromArray(f.positions,f.indices[j]*3).applyMatrix4(matrix),b=new THREE.Vector3().fromArray(f.positions,f.indices[j+1]*3).applyMatrix4(matrix),c=new THREE.Vector3().fromArray(f.positions,f.indices[j+2]*3).applyMatrix4(matrix);
      areaNormals.push(b.sub(a).cross(c.sub(a)));
    }}
    const box=new THREE.Box3().setFromPoints(frame),centre=box.getCenter(new THREE.Vector3());
    const radiusAt=Math.max(...nodes.map(i=>values[i*6+3]));
    const ray=new THREE.Raycaster(),oldSide=wood.material.side;wood.material.side=THREE.DoubleSide;
    const directions=[];
    for(const elevation of [-.5,0,.5])for(let azimuth=0;azimuth<16;azimuth++)directions.push(new THREE.Vector3(Math.cos(azimuth*Math.PI/8),elevation,Math.sin(azimuth*Math.PI/8)).normalize());
    const candidates=directions.map((direction,index)=>{
      const right=new THREE.Vector3(0,1,0).cross(direction).normalize(),up=direction.clone().cross(right).normalize();
      const local=frame.map(p=>p.clone().sub(centre));
      const extent=Math.max(...local.map(p=>Math.abs(p.dot(up))*2),...local.map(p=>Math.abs(p.dot(right))*2/1.6));
      const vertical=Math.max(.1,extent*1.35),zs=corners.map(p=>p.clone().sub(centre).dot(direction)),distance=Math.max(...zs)+2;
      const camera={projection:'orthographic',position_m:centre.clone().addScaledVector(direction,distance).toArray(),target_m:centre.toArray(),up:[0,1,0],left_m:-vertical*.8,right_m:vertical*.8,top_m:vertical/2,bottom_m:-vertical/2,near_m:1,far_m:distance-Math.min(...zs)+1,width_px:1600,height_px:1000,dpr:1};
      const c=cameraFrom(camera),hits=probes.map(p=>{
        const projected=p.clone().project(c);ray.setFromCamera(new THREE.Vector2(projected.x,projected.y),c);ray.far=camera.far_m;
        const hit=ray.intersectObjects(units.length?[wood,canopy]:[wood],false)[0];
        return {point_m:p.toArray(),first_surface:hit?.object.name??null,first_hit_m:hit?.point.toArray()??null,visible:hit?.object===wood&&hit.point.distanceTo(p)<=Math.max(.008,radiusAt*2)};
      });
      const visible=hits.filter(h=>h.visible).length;
      const axis=(view==='attached-shoot'?point(nodes[2]):frame[0].clone()).sub(junction).normalize();
      const separation=1-Math.abs(axis.dot(direction));
      const projected_area=areaNormals.reduce((sum,n)=>sum+Math.abs(n.dot(direction)),0);
      return {index,camera,hits,visible,projected_area,separation};
    });
    wood.material.side=oldSide;
    candidates.sort((a,b)=>b.visible-a.visible||b.projected_area-a.projected_area||b.separation-a.separation||a.index-b.index);
    const chosen=candidates[0];
    if(!chosen.visible)throw Error('unassessed: no unobstructed woody probe in deterministic camera search');
    return {camera:chosen.camera,target,leaf_state:units.length?'selected-attachment-only':'hidden',diagnostic:{version:'fn19-visibility-v2',view,geometry_hashes:hashes,wood:'full-connected-original',retained_unit_indices:units,original_unit_count:f.matrices.length/16,attachment_mapping:'original matrix origins within tapered terminal segment; same geometric attachment rule as v1; no generator ownership ID available',foliage_filter:view==='fork'?'all foliage hidden':'only selected terminal-segment attachment matrices retained, including original connectors',target_rule:'unchanged v1 normalized semantic target and tie order',camera_rule:'48 fixed spherical directions; most visible woody probes, then greatest projected retained biological area, then least axial foreshortening, then direction index',camera_direction_index:chosen.index,projected_nodes_px:nodes.map(i=>{const p=point(i).project(cameraFrom(chosen.camera));return {node:i,x:(p.x+1)*800,y:(1-p.y)*500};}),visible_probe_count:chosen.visible,probe_count:probes.length,probes:chosen.hits,candidate_scores:candidates.map(c=>({index:c.index,visible:c.visible,projected_area:c.projected_area,separation:c.separation})),frame_margin:1.35,assessment:'unassessed pending direct image inspection',density_use:'prohibited: diagnostic filtering is not natural foliage density'}};
  }
  function raw(condition,channel,offset=null) {
    if(condition.diagnostic){if(canopy.count!==condition.diagnostic.retained_unit_indices.length)diagnosticInstances(condition.diagnostic.retained_unit_indices);}else{canopy.instanceMatrix=originalInstances;canopy.count=f.matrices.length/16;canopy.frustumCulled=true;}
    const c=condition.camera,camera=cameraFrom(c);if(offset)camera.setViewOffset(1600,1000,offset[0],offset[1],1600,1000);
    const mask=channel==='coverage';wood.visible=!mask;canopy.visible=condition.leaf_state!=='hidden';canopy.material=mask?white:leafMaterial;canopy.geometry.setIndex(mask?biologicalIndex:fullIndex);scene.background=new THREE.Color(mask?0x000000:0xc6ced5);
    renderer.render(scene,camera);gl.finish();if(gl.isContextLost()||gl.getError()!==gl.NO_ERROR||(!renderer.info.render.triangles&&!(mask&&condition.leaf_state==='hidden')))throw Error('hardware-unavailable: lost context or empty draw');
    const pixels=new Uint8Array(1600*1000*4);gl.readPixels(0,0,1600,1000,gl.RGBA,gl.UNSIGNED_BYTE,pixels);
    const channels=mask?1:3,data=new Float32Array(1600*1000*channels);
    for(let y=0;y<1000;y++)for(let x=0;x<1600;x++)for(let k=0;k<channels;k++)data[(y*1600+x)*channels+k]=pixels[((999-y)*1600+x)*4+k]/255;
    return data;
  }
  return {prepare,prepareVisibility,raw,metadata:{hashes,three_revision:THREE.REVISION,wasm_sha256:wasmHash,generation_ms:generationMs,preparation_ms:performance.now()-started-generationMs,wasm_bytes:wasmBytes,backend,diagnostics:output.diagnostics,normalization,source_array_bytes:Object.values(arrays).reduce((n,a)=>n+a.byteLength,0),samples:gl.getParameter(gl.SAMPLES),encoding:'linear unsigned8 framebuffer MSAA; Float64 averaging; float32 little endian; sRGB preview',exclusive_window:false}};
}
