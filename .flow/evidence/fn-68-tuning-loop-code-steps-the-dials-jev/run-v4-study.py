import json,hashlib,subprocess,importlib.util
from pathlib import Path
r=Path(__file__).parent;o=r/'v4-generated';ledger=Path('.flow/ledger/fn68-v4-calibration')
def read(p):return json.loads(p.read_text())
def canon(v):return json.dumps(v,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()
def save(p,v):p.write_text(json.dumps(v,indent=2)+'\n')
pre=read(o/'preflight.json');assert pre['reservation']<=8000
for name,h in pre['hashes'].items():assert hashlib.sha256(canon(read(o/(name+'.json')))).hexdigest()==h
p=r/'v4-calibration-accounting.json';assert not p.exists()
a={'previous_tokens':212422,'additional_max_tokens':8000,'cumulative_max_tokens':270000,'new_actual_tokens':0,'cumulative_actual_tokens':212422,'pending':{'reserved':pre['reservation']},'host_release':'one call after offline corrections','runtime_untouched':True};save(p,a)
before=set(ledger.glob('*.json'));cmd=f'target/release/jev ask --state {o}/state.json --questions {o}/questions.json --tool fn68-v4-candidates --ledger {ledger}'
result=subprocess.run(['bash','-ic',cmd],capture_output=True,text=True);(r/'local/v4-stdout.json').write_text(result.stdout);assert result.returncode==0,result.stderr
new=set(ledger.glob('*.json'))-before;assert len(new)==1
path=new.pop();e=read(path);assert e['model']=='jev-1.13.0' and e['questions']==read(o/'questions.json') and e['state_sha256']==pre['hashes']['state']
used=e['usage']['input_tokens']+e['usage']['output_tokens'];a.update(new_actual_tokens=used,cumulative_actual_tokens=212422+used,pending=None,ledger=str(path));save(p,a);assert used<=pre['reservation']
spec=importlib.util.spec_from_file_location('scorer',r/'score-v4-study.py');s=importlib.util.module_from_spec(spec);spec.loader.exec_module(s)
out=s.score(read(o/'manifest.json'),e['answers']);out.update(ledger=e,accounting=a,manifest_sha256=pre['hashes']['manifest']);save(r/'v4-policy-result.json',out);print(json.dumps({k:v for k,v in out.items() if k!='ledger'}))
