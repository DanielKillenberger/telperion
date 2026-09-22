"""One approved two-stage study; immutable inputs, checkpointed spend, no retries."""
import json,hashlib,subprocess,importlib.util
from pathlib import Path
r=Path(__file__).parent;o=r/'v3-generated';ledger=Path('.flow/ledger/fn68-v3-calibration')
def text(v):return json.dumps(v,sort_keys=True,separators=(',',':'),ensure_ascii=False)
def read(p):return json.loads(p.read_text())
def save(p,v):p.write_text(json.dumps(v,indent=2)+'\n')
m=read(o/'manifest.json');pre=read(o/'preflight.json');assert pre['total']<=11000
for name,h in pre['hashes'].items():assert hashlib.sha256((o/name).read_bytes()).hexdigest()==h
account=r/'v3-calibration-accounting.json';assert not account.exists()
a={'previous_tokens':208907,'additional_max_tokens':11000,'cumulative_max_tokens':270000,'new_actual_tokens':0,'cumulative_actual_tokens':208907,'pending':None,'ledgers':[],'runtime_untouched':True,'manifest_sha256':pre['hashes']['manifest.json']}
def call(stage,state,questions):
 reserve=len(text(state).encode())+len(text(questions).encode())+1024
 assert a['new_actual_tokens']+reserve<=11000
 for part,value in [('state',state),('questions',questions)]:
  p=o/(stage+'-'+part+'.json')
  if p.exists():assert read(p)==value
  else:p.write_text(text(value)+'\n')
 a['pending']={'stage':stage,'reserved':reserve,'state_sha256':hashlib.sha256(text(state).encode()).hexdigest(),'questions_sha256':hashlib.sha256(text(questions).encode()).hexdigest()};save(account,a)
 before=set(ledger.glob('*.json'))
 cmd=f'target/release/jev ask --state {o}/{stage}-state.json --questions {o}/{stage}-questions.json --tool fn68-v3-{stage} --ledger {ledger}'
 result=subprocess.run(['bash','-ic',cmd],capture_output=True,text=True)
 (r/'local'/('v3-'+stage+'-stdout.json')).write_text(result.stdout)
 assert result.returncode==0,result.stderr
 new=set(ledger.glob('*.json'))-before;assert len(new)==1
 p=new.pop();e=read(p)
 assert e['model']=='jev-1.13.0' and e['questions']==questions and e['state_sha256']==a['pending']['state_sha256']
 used=e['usage']['input_tokens']+e['usage']['output_tokens']
 a['new_actual_tokens']+=used;a['cumulative_actual_tokens']=208907+a['new_actual_tokens'];a['ledgers'].append(str(p));a['pending']=None;save(account,a)
 assert used<=reserve and a['new_actual_tokens']<=11000
 return e
d=call('direction',read(o/'direction-state.json'),read(o/'direction-questions.json'))
cases={};questions={}
for c in m['cases']:
 i=c['id'];answer=d['answers'][i];direction=answer['choice']
 if answer['confidence']<.5 or direction not in c['available']:continue
 vals=c['available'][direction]
 cases[i]={'effect':m['effects'][c['parameter']],'observation':c['observation'],'current':c['current'],'direction':direction,'candidates':vals}
 questions[i]={'type':'choice','instructions':f'Apply policy to cases.{i}.','criteria':{k:('Supported smaller probe' if k=='small' else 'Supported larger probe') for k in vals}}
 questions[i]['criteria']['insufficient_evidence']='No justified size'
s=call('size',{'policy':m['size_policy'],'cases':cases},questions) if cases else {'answers':{}}
spec=importlib.util.spec_from_file_location('scorer',r/'score-v3-study.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
result=module.score(m,d['answers'],s['answers']);result.update(direction_ledger=d,size_ledger=s,accounting=a,provenance=m['provenance'])
save(r/'v3-policy-result.json',result);print(json.dumps({'metrics':result['metrics'],'unsafe':result['unsafe'],'passes':result['passes'],'accounting':a}))
