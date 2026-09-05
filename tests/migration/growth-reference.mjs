// Export finished FN6 in a disposable archive, then compare complete native solved trees.
import {spawnSync} from 'node:child_process';
import {readFileSync,writeFileSync} from 'node:fs';
import {resolve,join} from 'node:path';
import {tmpdir} from 'node:os';
const directory=resolve(process.env.REFERENCE_OUTPUT??join(tmpdir(),'telperion-growth-reference'));
const cases=['ordinary','telperion','laurelin','empty','capped','degenerate','envelope-crossing'];
function run(command,args,env={}){const r=spawnSync(command,args,{stdio:'inherit',env:{...process.env,...env}});if(r.status!==0)process.exit(r.status??1);}
run('node',['scripts/export-reference.mjs',...cases],{REFERENCE_BUFFERS:'1',REFERENCE_OUTPUT:directory});
const revision='fdafb099b1495519de75a6b9a66d37f7d07e47bd';
for(const id of cases){const r=JSON.parse(readFileSync(join(directory,id+'.json'),'utf8'));if(r.revision!==revision)throw Error(`Mismatched ${id} reference`);const d=r.diagnostics;writeFileSync(join(directory,id+'-growth-report.txt'),[revision,d.nodes,d.crossover,d.shed,Number(d.capped),Number(d.levelCapped)].join(' '));}
run('cargo',['test','-p','telperion-core','--test','growth_reference','--','--ignored','--nocapture'],{GROWTH_REFERENCE_DIR:directory});
