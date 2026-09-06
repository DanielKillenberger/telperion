// Explicit camera-admission aid: choose a recorded direction or widen exact depth planes.
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {resolve,join} from 'node:path';
import {pathToFileURL,fileURLToPath} from 'node:url';
import {isDeepStrictEqual} from 'node:util';
const hash=b=>createHash('sha256').update(b).digest('hex'),json=async p=>JSON.parse(await readFile(p,'utf8'));
const [proposalArg,indexArg,view,outputArg,direction='same',depth='1']=process.argv.slice(2),root=process.cwd(),out=resolve(outputArg),proposalBytes=await readFile(proposalArg),proposal=JSON.parse(proposalBytes),index=Number(indexArg);
await mkdir(out,{recursive:false});await writeFile(join(out,'proposal-input.json'),proposalBytes);
const original=proposal.records.find(r=>r.case_index===index&&r.view===view);
if(!original?.condition)throw Error('missing proposed shoot');
const condition=structuredClone(original.condition);
if(direction==='same'){condition.camera.near_m-=Number(depth);condition.camera.far_m+=Number(depth);}else{condition.diagnostic.camera_direction_index=Number(direction);condition.camera={};}
const {chromium}=await import(process.env.PLAYWRIGHT_MODULE?pathToFileURL(process.env.PLAYWRIGHT_MODULE).href:'playwright');
let browser;
try {
 for(const file of proposal.tool.files)if(hash(await readFile(join(root,file.path)))!==file.sha256)throw Error('proposal source changed '+file.path);
 browser=await chromium.launch({executablePath:process.env.CHROMIUM_EXECUTABLE,headless:true,args:['--no-sandbox']});
 const page=await browser.newPage({viewport:{width:1600,height:1000},deviceScaleFactor:1});
 const url=process.env.BROWSER_URL??'http://127.0.0.1:5199';await page.route(url+'/',r=>r.fulfill({contentType:'text/html',body:'<!doctype html><canvas></canvas>'}));await page.goto(url+'/');
 const protocol=await json('.flow/evidence/fn19/protocol.json'),baseline=await json('/tmp/fn19-mature-visual-20260906/run.json'),c=baseline.cases[index];
 const result=await page.evaluate(async ({parameters,species,fixed,files,view})=>{
  for(const f of files.filter(f=>['src/browser/core.ts','src/browser/three.ts','src/browser/presets.generated.ts','tests/browser/geometry-benchmark-rig.mjs','tests/browser/geometry-benchmark-diagnostics.mjs','harness/stage.ts'].includes(f.path))){const url='/'+f.path+'?raw',response=await(await fetch(url)).text(),source=response.startsWith('export default ')?(await import(url)).default:response;const h=[...new Uint8Array(await crypto.subtle.digest('SHA-256',new TextEncoder().encode(source)))].map(x=>x.toString(16).padStart(2,'0')).join('');if(h!==f.sha256)throw Error('served source mismatch');}
  const {createRig}=await import('/tests/browser/geometry-benchmark-rig.mjs'),rig=await createRig(parameters,species),condition=rig.prepareVisibility(view,fixed),data=rig.raw(condition,'beauty');
  const canvas=document.createElement('canvas');canvas.width=1600;canvas.height=1000;const pixels=new Uint8ClampedArray(1600*1000*4);
  for(let i=0;i<1600*1000;i++){for(let k=0;k<3;k++){const x=data[i*3+k],s=x<=.0031308?12.92*x:1.055*x**(1/2.4)-.055;pixels[i*4+k]=Math.round(Math.max(0,Math.min(1,s))*255);}pixels[i*4+3]=255;}
  canvas.getContext('2d').putImageData(new ImageData(pixels,1600,1000),0,0);
  return {condition,geometry:rig.metadata,png:canvas.toDataURL('image/png').split(',')[1]};
 },{parameters:baseline.parameters[c.id],species:protocol.species.find(s=>s.id===c.species_id),fixed:condition,files:proposal.tool.files,view});
 if(!isDeepStrictEqual(result.geometry.hashes,original.geometry.hashes)||result.geometry.wasm_sha256!==original.geometry.wasm_sha256)throw Error('original geometry changed');
 const bytes=Buffer.from(result.png,'base64');delete result.png;await writeFile(join(out,'preview.png'),bytes);
 result.condition.diagnostic.depth_crop=direction==='same'?`Explicit admission preview: original proposal near/far expanded by ${depth} metre per side; other camera values unchanged.`:`Explicit admission preview: recorded direction ${direction}, same semantic target and instance mapping; local depth volume recomputed.`;
 const receipt={version:'fn19-visibility-v2.2',case_id:original.case_id,case_index:index,view,status:'proposed',visibility:'unassessed',proposal_path:join(out,'proposal-input.json'),proposal_sha256:hash(proposalBytes),proposal_tool:proposal.tool,helper:{path:fileURLToPath(import.meta.url),sha256:hash(await readFile(fileURLToPath(import.meta.url)))},browser:browser.version(),preview:{path:'preview.png',bytes:bytes.length,sha256:hash(bytes)},...result};
 await writeFile(join(out,'preview.json'),JSON.stringify(receipt,null,2)+'\n');
 for(const f of proposal.tool.files)if(hash(await readFile(join(root,f.path)))!==f.sha256)throw Error('source changed during preview');
 console.log(original.case_id,'camera adjustment preview complete');
}finally{await browser?.close();}
