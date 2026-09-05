import {mkdtemp,readFile,writeFile,mkdir,symlink,rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import {join,resolve} from 'node:path';
import {execFileSync} from 'node:child_process';
import {build} from 'esbuild';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {endianness} from 'node:os';

const revision='fdafb099b1495519de75a6b9a66d37f7d07e47bd';
const root=fileURLToPath(new URL('..',import.meta.url));
const observed=execFileSync('git',['rev-parse',`${revision}^{commit}`],{cwd:root,encoding:'utf8'}).trim();
if(observed!==revision) throw Error('Final FN6 reference mismatch');
const scratch=await mkdtemp(join(tmpdir(),'telperion-reference-'));
const output=resolve(process.env.REFERENCE_OUTPUT??join(root,'tests/migration/generated'));
await mkdir(output,{recursive:true});
try {
  const archive=execFileSync('git',['archive',revision],{cwd:root,maxBuffer:64*1024*1024});
  execFileSync('tar',['-x','-C',scratch],{input:archive});
  await symlink(join(root,'node_modules'),join(scratch,'node_modules'),'dir');
  const source=await readFile(new URL('../tests/migration/reference.ts',import.meta.url),'utf8');
  await build({stdin:{contents:source,resolveDir:scratch,loader:'ts'},bundle:true,platform:'node',format:'esm',packages:'external',outfile:join(scratch,'export.mjs')});
  execFileSync(process.execPath,[join(scratch,'export.mjs'),output,...process.argv.slice(2)],{stdio:'inherit',timeout:600_000,env:{...process.env,REFERENCE_REVISION:revision}});
  await writeFile(join(output,'provenance.json'),JSON.stringify({revision,archive:'git archive; disposable directory; no production reference import',node:process.version,architecture:process.arch,endianness:endianness(),dependencyLockSha256:createHash('sha256').update(await readFile(join(root,'package-lock.json'))).digest('hex')},null,2)+'\n');
} finally {await rm(scratch,{recursive:true,force:true});}
