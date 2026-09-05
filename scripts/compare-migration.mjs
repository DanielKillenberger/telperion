import {readFile} from 'node:fs/promises';
import {join} from 'node:path';
const [reference,candidate]=process.argv.slice(2);
if(!reference||!candidate)throw Error('Usage: compare-migration.mjs REFERENCE_DIRECTORY CANDIDATE_DIRECTORY');
const read=async p=>JSON.parse(await readFile(p,'utf8'));
const manifest=await read(join(reference,'manifest.json'));
if(!manifest.length)throw Error('Empty reference manifest');
const reports=[];
const invariantKeys=['finitePositions','parentOrder','positiveRadii','taper','validIndices','finiteSurface','finiteFoliage'];
for(const {id} of manifest) {
  const [ref,next]=await Promise.all([read(join(reference,id+'.json')),read(join(candidate,id+'.json'))]);
  if(ref.revision!=='fdafb099b1495519de75a6b9a66d37f7d07e47bd')throw Error(id+': mismatched final FN6 reference');
  if(next.id!==id)throw Error(id+': mismatched candidate');
  for(const key of invariantKeys)if(next.invariants?.[key]!==true)throw Error(id+': invariant failed or absent: '+key);
  const ratio=(a,b)=>b===0?(a===0?1:null):a/b;
  reports.push({id,nodeRatio:ratio(next.diagnostics.nodes,ref.diagnostics.nodes),boundsDelta:next.diagnostics.bounds.map((x,i)=>x-ref.diagnostics.bounds[i]),caps:{reference:ref.diagnostics.capped,candidate:next.diagnostics.capped},stages:Object.fromEntries(Object.entries(ref.stages).map(([key,value])=>{
    const actual=next.stages[key];if(!actual)throw Error(id+': missing stage '+key);
    return [key,{countRatio:ratio(actual.values,value.values),hashEqual:actual.sha256===value.sha256}];
  }))});
}
console.log(JSON.stringify(reports,null,2));
