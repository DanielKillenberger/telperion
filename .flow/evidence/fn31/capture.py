"""One native full capture; run once after a checkpoint and small previews."""
import subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'.flow/evidence/fn31'
(OUT/'strips').mkdir(exist_ok=True)
with (OUT/'round3/logs/capture.log').open('w') as log:
 def run(cmd):
  log.write(' '.join(map(str,cmd))+'\n'); log.flush()
  r=subprocess.run(list(map(str,cmd)),cwd=ROOT,stdout=log,stderr=log)
  log.write(f'exit={r.returncode}\n'); log.flush(); r.check_returncode()
 for species,ages in [('oregon-white-oak',[1,10,26.7,56.1,112,200,432]),('norway-spruce',[1,5,14.1,26.6,36.9,60,158])]:
  frames=[]
  for age in ages:
   out=OUT/'strips'/f'{species}-age-{age}.png'; frames.append(out)
   cmd=['target/release/examples/headless','--preset',species,'--seed','7','--age',age,'--size','700x1000','--out',out]
   if age==1: cmd+=['--frame-min-y',0]
   run(cmd); print(species,age,'rendered',flush=True)
  run(['montage',*frames,'-tile','7x1','-geometry','+4+0',OUT/'strips'/f'{species}-strip.png'])
  run(['target/release/examples/headless','--preset',species,'--seed',7,'--size','1600x1000','--out',OUT/f'{species}-hero.png'])
  run(['montage',ROOT/'.flow/evidence/fn30'/f'{species}-hero.png',OUT/f'{species}-hero.png','-tile','2x1','-geometry','+4+0',OUT/f'{species}-beside-fn30.png'])
  run(['montage',ROOT/'.flow/evidence/fn30/strips'/f'{species}-strip.png',OUT/'strips'/f'{species}-strip.png','-tile','1x2','-geometry','+0+4',OUT/f'{species}-strips-beside-fn30.png'])
 run(['target/release/examples/headless','--preset','ordinary','--seed',7,'--size','1600x1000','--view','clay','--out',ROOT/'.flow/evidence/fn24/ordinary-hero.png'])
