// Run from the repository root; raw reference data is reproducible and uncommitted.
import {spawnSync} from 'node:child_process';
import {readFileSync,writeFileSync} from 'node:fs';
import {resolve,join} from 'node:path';
import {tmpdir} from 'node:os';
import * as THREE from 'three';
const directory=resolve(process.env.REFERENCE_OUTPUT??join(tmpdir(),'telperion-surface-reference'));
const cases=process.argv.slice(2); if(!cases.length)cases.push('ordinary','telperion','laurelin','empty','capped');
function run(command,args,env={}) {const r=spawnSync(command,args,{stdio:'inherit',env:{...process.env,...env}});if(r.status!==0)process.exit(r.status??1);}
run('node',['scripts/export-reference.mjs',...cases],{REFERENCE_BUFFERS:'1',REFERENCE_OUTPUT:directory});
for(const id of cases) {
    const record=JSON.parse(readFileSync(join(directory,id+'.json'),'utf8'));
    const s=record.parameters.surface;
    writeFileSync(join(directory,id+'-surface-config.txt'),[record.parameters.skeleton.envelope.height,s.radialSegments,s.lobes,s.lobeDepth,s.twistRate,s.flareRadius,s.flareFalloff,s.flareDepth,s.forkSocket,s.forkSwell].join(' '));
    const typed=(suffix,Type)=>{const b=readFileSync(join(directory,id+suffix));return new Type(b.buffer.slice(b.byteOffset,b.byteOffset+b.byteLength));};
    const geometry=new THREE.BufferGeometry();
    geometry.setAttribute('position',new THREE.BufferAttribute(typed('-surface-positions.bin',Float32Array),3));
    geometry.setIndex(new THREE.BufferAttribute(typed('-surface-indices.bin',Uint32Array),1));
    geometry.computeVertexNormals();
    writeFileSync(join(directory,id+'-surface-normals.bin'),new Uint8Array(geometry.getAttribute('normal').array.buffer));
    geometry.dispose();
}
run('cargo',['test','-p','telperion-core','--test','surface_reference','--','--ignored','--nocapture'],{SURFACE_REFERENCE:directory,SURFACE_CASES:cases.join(',')});
