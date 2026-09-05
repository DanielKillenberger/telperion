// Run from the repository root: node crates/telperion-core/tests/crown_reference.mjs
// Uses the pinned FN6 source and runs only its crown stage, never giant meshes.
import {mkdtemp,readFile,writeFile,symlink,rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {execFileSync} from 'node:child_process';
import {build} from 'esbuild';
const root=new URL('../../../',import.meta.url).pathname;
const revision='fdafb099b1495519de75a6b9a66d37f7d07e47bd';
if(execFileSync('git',['rev-parse',`${revision}^{commit}`],{cwd:root,encoding:'utf8'}).trim()!==revision)throw Error('Mismatched FN6 reference');
const scratch=await mkdtemp(join(tmpdir(),'crown-reference-'));
try {
  execFileSync('tar',['-x','-C',scratch],{input:execFileSync('git',['archive',revision],{cwd:root,maxBuffer:64*1024*1024})});
  await symlink(join(root,'node_modules'),join(scratch,'node_modules'));
  const fixtures=await Promise.all(['ordinary','telperion','laurelin','empty','capped','envelope-crossing'].map(async id=>JSON.parse(await readFile(join(root,'tests/migration/fixtures',id+'.json'),'utf8'))));
  if(fixtures.some(f=>f.revision!==revision))throw Error('Mismatched fixture provenance');
  const source=`import {writeFileSync} from 'node:fs';
import {Vector3} from 'three';
import {sampleEnvelope} from './src/envelope';
import {createRng} from './src/rng';
import {innerEnvelope,resolveGrowth} from './src/skeleton/grow';
import {colonize} from './src/skeleton/colonize';
const fixtures=${JSON.stringify(fixtures.map(f=>({id:f.id,params:f.parameters.skeleton})))};
for(const {id,params:p} of fixtures){
 const e=p.envelope,b=p.bias;
 const points=sampleEnvelope(innerEnvelope(e,p.twigs.reach),p.attractors,createRng(p.seed));
 const c=resolveGrowth(p,points.length),tree=colonize(points,new Vector3(),c);
 const lines=[id,[e.height,e.crownBase,e.spread,e.fullness,e.shoulder].join(' '),
 [p.seed,b.gravitropism,b.lean,b.writheAmplitude,b.writheWavelength,b.spiralRate].join(' '),
 [c.influenceRadius,c.killDistance,c.stepDistance,c.trunkHeight,c.maxNodes,c.maxTurnPerStep??35].join(' '),
 String(points.length),...points.map(p=>[p.x,p.y,p.z].join(' ')),String(tree.nodes.length),
 ...tree.nodes.map(n=>[n.parent,n.position.x,n.position.y,n.position.z].join(' '))];
 writeFileSync(${JSON.stringify(scratch)}+'/'+id+'.txt',lines.join('\\n'));
}`;
  await build({stdin:{contents:source,resolveDir:scratch,loader:'ts'},bundle:true,platform:'node',format:'esm',packages:'external',outfile:join(scratch,'export.mjs')});
  execFileSync(process.execPath,[join(scratch,'export.mjs')],{stdio:'inherit',timeout:600_000});
  execFileSync('cargo',['test','-p','telperion-core','--test','crown_reference','--','--ignored','--nocapture'],{cwd:root,stdio:'inherit',timeout:600_000,env:{...process.env,CROWN_REFERENCE_DIR:scratch}});
} finally {await rm(scratch,{recursive:true,force:true});}
