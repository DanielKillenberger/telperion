from pathlib import Path
import os,json,subprocess,hashlib
here=Path(__file__).resolve().parent
root=Path(json.loads((here/'paths.json').read_text())['root'])
rows=[]
for variant in ['baseline','candidate']:
 exe=root/variant/'target/release/examples/station_screen'
 for species in ['oregon-white-oak','norway-spruce']:
  for seed in [1,7]:
   prefix=root/'capacity-dump'
   r=subprocess.run([str(exe),species,str(seed)],env={**os.environ,'GENERATION_SAMPLES':'1','STATION_DUMP':str(prefix)},capture_output=True,text=True,check=True)
   (here/f'capacity-{variant}-{species}-{seed}.log').write_text(r.stdout+r.stderr)
   vals={l.split()[0]:list(map(int,l.split()[1:])) for l in r.stderr.splitlines() if l.startswith('CAP_')}
   rows.append({'variant':variant,'species':species,'seed':seed,'childBytes':vals['CAP_CHILD'][0],'runBytes':vals['CAP_CHILD'][1],'pointsAlongPeak':vals['CAP_STATIONS'][0],'outputCapacityBytes':vals['CAP_STATIONS'][1],'preparationPeakWithOutputReallocation':vals['CAP_STATIONS'][2],'binarySha256':hashlib.sha256(exe.read_bytes()).hexdigest()})
(here/'capacity-results.json').write_text(json.dumps(rows,indent=2)+'\n')
print(json.dumps(rows,indent=2))
