import fs from 'node:fs';
import crypto from 'node:crypto';
import cp from 'node:child_process';
const root=import.meta.dirname;
const read=name=>JSON.parse(fs.readFileSync(root+'/'+name));
const result=read('owner-grading-result.json'), pre=read('owner-grading-preflight.json'), map=read('owner-grading-map.json');
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const dims=['crown','hanging','material','standard'];
function rank(answers,mapping) {
  const rows=Object.entries(mapping).map(([id,source])=>{
    const scores={},confidence={},probabilities={};
    for(const d of dims){
      const a=answers[id+'_'+d];
      if(!a||a.type!=='score'||!Number.isFinite(a.score)||a.score<0||a.score>3||!Number.isFinite(a.confidence)||a.confidence<0||a.confidence>1)throw Error('Invalid Score '+id+d);
      const p=a.probabilities;
      if(!p||Object.keys(p).join(',')!=='0,1,2,3'||Object.values(p).some(v=>!Number.isFinite(v)||v<0||v>1)||Math.abs(Object.values(p).reduce((a,b)=>a+b,0)-1)>.021)throw Error('Invalid probabilities');
      scores[d]=a.score;confidence[d]=a.confidence;probabilities[d]=p;
    }
    return {id,label:source.label,scores,confidence,probabilities,primary_mean:(scores.crown+scores.hanging)/2,secondary_mean:(scores.material+scores.standard)/2};
  });
  return rows.sort((a,b)=>b.primary_mean-a.primary_mean||b.secondary_mean-a.secondary_mean);
}
const rows=rank(result.entry.answers,map);
if(result.entry.state_sha256!==pre.state_sha256||JSON.stringify(result.entry.questions)!==JSON.stringify(read('owner-grading-questions.json'))) {
  // Object order is immaterial; compare recursively key-sorted forms below.
  const sort=x=>Array.isArray(x)?x.map(sort):x&&typeof x==='object'?Object.fromEntries(Object.keys(x).sort().map(k=>[k,sort(x[k])])):x;
  if(result.entry.state_sha256!==pre.state_sha256||JSON.stringify(sort(result.entry.questions))!==JSON.stringify(sort(read('owner-grading-questions.json'))))throw Error('Receipt identity mismatch');
}
for(const source of Object.values(map))if(hash(fs.readFileSync(root+'/'+source.source))!==source.source_sha256)throw Error('Source changed');
if(result.entry.error||result.actual_tokens!==result.entry.usage.input_tokens+result.entry.usage.output_tokens||result.cumulative_actual_tokens!==464835+result.actual_tokens)throw Error('Usage mismatch');
// Small deterministic ranking/invalid-answer regressions, not model qualification.
const mock={};for(const id of ['a','b'])for(const d of dims)mock[id+'_'+d]={type:'score',score:id==='a'?(d==='crown'||d==='hanging'?2:0):1,confidence:1,probabilities:{0:0,1:1,2:0,3:0}};
if(rank(mock,{a:{label:'a'},b:{label:'b'}})[0].id!=='a')throw Error('Secondary scores displaced primary');
let rejected=false;try{rank({}, {a:{label:'a'}});}catch{rejected=true;}if(!rejected)throw Error('Missing answer accepted');
const output={policy:pre.ranking,rows,actual_tokens:result.actual_tokens,cumulative_actual_tokens:result.cumulative_actual_tokens,model:result.entry.model,
  limitations:['Text agreement with owner on one known development case, not an image re-evaluation or general model benchmark.','Astra medium and high primary scores differ by only0.075; no statistically established winner.','All hanging Scores lie near absence/ambiguity level1, not major-gap level3.','Opus and Fable crown confidence is very low; numerical ordering is not a reliable separation.','Acceptance-standard wording scored highly even in verdicts that relaxed morphology; do not let this secondary score erase primary misses.','Prior host image-fidelity concerns remain separate; Jev saw text, not images.'],
  checks:'24 Score shapes/distributions, immutable source hashes, receipt state/questions, usage sums, primary-dominance and missing-answer mocks passed'};
const dest=root+'/owner-grading-ranking.json',content=JSON.stringify(output,null,2)+'\n';
if(fs.existsSync(dest)){if(fs.readFileSync(dest,'utf8')!==content)throw Error('Immutable ranking changed');}else cp.execFileSync('apply_patch',[],{input:'*** Begin Patch\n*** Add File: '+dest+'\n'+content.trimEnd().split('\n').map(s=>'+'+s).join('\n')+'\n*** End Patch'});
console.log(JSON.stringify(rows.map(r=>({model:r.label,primary:r.primary_mean,crown:r.scores.crown,hanging:r.scores.hanging}))));
