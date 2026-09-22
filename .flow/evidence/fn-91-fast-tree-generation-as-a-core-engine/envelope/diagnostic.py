"""Scratch-only counters; never used for the timed executables."""
import json, os, subprocess
from pathlib import Path
ev=Path(__file__).resolve().parent
scratch=Path((ev/'scratch-path.txt').read_text().strip())
p=scratch/'crates/telperion-core/src/branching/local/admission.rs'
s=p.read_text().replace('    bounds: Option<[f64; 257]>,','    bounds: Option<[f64; 257]>,\n    counts: std::cell::Cell<[usize; 4]>,')
s=s.replace('        Self { config, bounds }','        Self { config, bounds, counts: std::cell::Cell::new([0; 4]) }')
s=s.replace('    pub fn rejected(&self, p: Vec3) -> bool {','''    fn count(&self, i: usize) { let mut counts = self.counts.get(); counts[i] += 1; self.counts.set(counts); }
    pub fn rejected(&self, p: Vec3) -> bool {''')
s=s.replace('            return true;','            self.count(3);\n            return true;')
s=s.replace('''        self.classify(p)
            .unwrap_or_else(|| rejected(&self.config, p))''','''        match self.classify(p) {
            Some(false) => { self.count(0); false },
            Some(true) => { self.count(1); true },
            None => { self.count(2); rejected(&self.config, p) },
        }''')
s+='''
impl Drop for Admission {
    fn drop(&mut self) {
        eprintln!("DIAG {} {} {:?}", self.bounds.is_some(), std::mem::size_of::<Self>() - std::mem::size_of::<std::cell::Cell<[usize;4]>>(), self.counts.get());
    }
}
'''
p.write_text(s)
p=scratch/'crates/telperion-core/examples/envelope_screen.rs';p.write_text(p.read_text().replace('0..4','0..1'))
with (ev/'diagnostic-build.log').open('w') as log:
 subprocess.run(['cargo','build','--release','-p','telperion-core','--example','envelope_screen'],cwd=scratch,env=dict(os.environ,CARGO_TARGET_DIR=str(scratch/'target')),stdout=log,stderr=subprocess.STDOUT,timeout=600,check=True)
(scratch/'diagnostic-output').mkdir(exist_ok=True)
rows=[]
for species in ['oregon-white-oak','norway-spruce']:
 for seed in [1,7]:
  result=subprocess.run([str(scratch/'target/release/examples/envelope_screen'),species,str(seed),str(scratch/'diagnostic-output')],text=True,capture_output=True,timeout=120,check=True)
  (ev/f'diagnostic-{species}-{seed}.log').write_text(result.stderr)
  records=[]
  for line in result.stderr.splitlines():
   if line.startswith('DIAG '):
    _,supported,size,counts=line.split(' ',3)
    records.append({'prepared':supported=='true','context_bytes':int(size),'counts':json.loads(counts)})
  counts=[sum(r['counts'][i] for r in records) for i in range(4)]
  total=sum(counts)
  rows.append({'species':species,'seed':seed,'contexts':len(records),'prepared_contexts':sum(r['prepared'] for r in records),'context_bytes':records[0]['context_bytes'],'counts_inside_outside_fallback_vertical':counts,'query_total':total,'useful_fraction':(counts[0]+counts[1])/total,'fallback_fraction':counts[2]/total})
  assert (scratch/f'diagnostic-output/{species}-{seed}.bin').read_bytes()==(scratch/f'candidate-output/{species}-{seed}.bin').read_bytes()
(ev/'diagnostic-summary.json').write_text(json.dumps(rows,indent=2)+'\n')
print(json.dumps(rows,indent=2))
