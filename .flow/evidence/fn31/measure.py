"""Repeat the inherited fn30 instrument; never changes its reference curves."""
import json
import subprocess
from pathlib import Path
ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / '.flow/evidence/fn31'
for preset, ages in [('oregon-white-oak','1,10,26.7,56.1,112,200'),
                     ('norway-spruce','1,5,14.1,26.6,36.9,60'),
                     ('ordinary','173'),('telperion','173'),('laurelin','173')]:
    cmd = ['cargo','run','--release','-p','telperion-core','--example','growth_curve',
           '--','--preset',preset,'--seed','7','--ages',ages,'--envelope']
    with (OUT/'measurements'/f'{preset}.jsonl').open('w') as out, (OUT/'logs'/f'measure-{preset}.log').open('w') as log:
        log.write(' '.join(cmd)+'\n'); log.flush()
        result = subprocess.run(cmd,cwd=ROOT,stdout=out,stderr=log)
        log.write(f'\nexit={result.returncode}\n')
    print(preset,result.returncode,flush=True)
    result.check_returncode()
