"""One authorized camera-only repair, at most four still/twins; no measurements/models."""
import hashlib,json,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parent;WORKTREE=ROOT.parents[2]
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
fit=read(ROOT/'framefit-result.json');family=WORKTREE/fit['candidate_family'];binary=WORKTREE/'target/release/examples/headless'
runtime=WORKTREE/'.flow/tmp/fn68-pilot-run/run.json';original=sha(runtime)
assert original=='d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e'
directory=ROOT/'local/reframed';directory.mkdir(exist_ok=False)
refs=read(ROOT/'assets/beech-references-4e6d903c.json');commands=[]
for shot in fit['shots']:
    assert shot['new_max_corner_ndc']<=.95
    record=next(r for r in refs['references'] if r['id']==shot['id']);record['shot']['camera']=shot['camera']
    for twin in [False,True]:
        dim=1-.8*shot['light']['overcast'];overcast=shot['light']['overcast']
        scene={'sunAzimuth':(shot['camera']['azimuth']+180)%360 if twin else shot['light']['sunAzimuth'],'sunElevation':5 if twin else shot['light']['sunElevation'],'sunRed':3*dim,'sunGreen':2.85*dim,'sunBlue':2.6*dim,'skyZenithRed':.18+.37*overcast,'skyZenithGreen':.30+.36*overcast,'skyZenithBlue':.62+.18*overcast}
        out=directory/(shot['id']+('-twin' if twin else'')+'.png')
        commands.append([str(binary),'--preset','european-beech','--seed','1','--view','bare' if shot['id']=='B-BARE' else'whole','--size','x'.join(map(str,shot['size'])),'--out',str(out),'--family',str(family),'--camera',json.dumps(shot['camera']),'--scene',json.dumps(scene),'--no-figure'])
metadata=ROOT/'reframed-shots.json'
with metadata.open('x')as f:json.dump(refs,f,indent=2)
journal={'authority':'Owner YES to570000 cumulative, atmost4camera-repair captures and ONE corrected comparison; no retries','prior_actual_tokens':524517,'token_cap':570000,'evaluations':6,'prior_image_reservations':24,'image_reservations':28,'prior_actual_images':20,'actual_images':20,'attempted_captures':0,'runtime_sha256':original,'family_sha256':sha(family),'headless_sha256':sha(binary),'framefit_sha256':sha(ROOT/'framefit-result.json'),'commands':commands,'captures':[],'status':'reserved_before_capture','numeric_comparison':'diagnostic newcamera; not directly comparable to old pixel metrics'}
jp=ROOT/'reframed-capture-journal.json'
with jp.open('x')as f:json.dump(journal,f,indent=2)
for command in commands:
    assert sha(family)==journal['family_sha256'] and sha(binary)==journal['headless_sha256']
    journal['attempted_captures']+=1;jp.write_text(json.dumps(journal,indent=2))
    started=time.monotonic();result=subprocess.run(command,cwd=WORKTREE,capture_output=True,text=True,timeout=300)
    out=Path(command[command.index('--out')+1]);(directory/(out.stem+'.log')).write_text(result.stdout+result.stderr)
    assert result.returncode==0,'capture failed; no retry'
    journal['captures'].append({'path':str(out),'sha256':sha(out),'seconds':time.monotonic()-started});journal['actual_images']+=1;jp.write_text(json.dumps(journal,indent=2))
assert sha(runtime)==original
journal['status']='four_captures_complete';jp.write_text(json.dumps(journal,indent=2));print(json.dumps(journal['captures']))
