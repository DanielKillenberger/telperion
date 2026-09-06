#!/usr/bin/env python3
"""Compare actual tiny native replays plus explicitly mutated receipt fixtures."""
import argparse, copy, hashlib, json, pathlib, shutil, subprocess
p=argparse.ArgumentParser();p.add_argument('--native-controls',required=True,type=pathlib.Path);p.add_argument('--output',required=True,type=pathlib.Path);a=p.parse_args();a.output.mkdir()
root=pathlib.Path.cwd();base=a.native_controls/'baseline';candidate=a.native_controls/'candidate';protocol=a.native_controls/'protocol.json';references=root/'.flow/evidence/fn19/references.json'
def read(p):return json.loads(p.read_text())
def save(p,v):p.write_text(json.dumps(v,indent=2)+'\n')
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
results=[]
def compare(name,path,expected,proto=protocol,refs=references):
 out=a.output/(name+'-comparison');cmd=['node','scripts/benchmarks/geometry-compare.mjs','--protocol',str(proto),'--references',str(refs),'--baseline',str(base),'--candidate',str(path),'--output',str(out)]
 r=subprocess.run(cmd,capture_output=True,text=True);result=read(out/'comparison.json');assert r.returncode==expected,(name,r.stderr,result)
 results.append({'control':name,'exit_code':r.returncode,'status':result['status'],'reasons':result['reasons'],'changed_cases':[k for k,v in result.get('metrics',{}).items() if v['changed']],'comparison_path':str(out/'comparison.json'),'sha256':digest(out/'comparison.json')})
 return result
compare('unchanged-independent-native-replay',candidate,0)
for name in ['metric-drift','changed-source-metric','failed','missing','duplicate','binary','source-digest','interrupted']:
 d=a.output/name;shutil.copytree(candidate,d);run=read(d/'run.json');rows=[json.loads(x) for x in (d/'measurements.jsonl').read_text().splitlines()];term=next(x for x in rows if x['event']=='completed')
 if name in ['metric-drift','changed-source-metric']:
  native=d/term['artifacts'][0]['path'];data=read(native);data['metrics']['axes']['value']['raw_axis_count']+=1;save(native,data);term['metrics']=data['metrics'];term['artifacts'][0].update(sha256=digest(native),bytes=native.stat().st_size)
  if name=='changed-source-metric':
   run['source']['files'][0]['sha256']='0'*64;run['source']['sha256']=hashlib.sha256(''.join(f['path']+'\0'+f['sha256']+'\n' for f in run['source']['files']).encode()).hexdigest();run['source']['dirty']=True
 elif name=='failed':term.update(event='failed',numeric_status='fail',reason='deliberate failure control',metrics={},artifacts=[]);run.update(status='failed',partial=True,reason='deliberate failure control')
 elif name=='missing':rows=[r for r in rows if r['case_id']!=term['case_id']]
 elif name=='duplicate':rows.append(copy.deepcopy(term))
 elif name=='binary':(d/'native-binary').write_bytes(b'deliberately stale binary')
 elif name=='source-digest':run['source']['sha256']='0'*64
 save(d/'run.json',run);(d/'measurements.jsonl').write_text(''.join(json.dumps(x)+'\n' for x in rows) if name!='interrupted' else json.dumps(term))
 result=compare(name,d,0 if name=='changed-source-metric' else 1)
 if name=='changed-source-metric':assert any(v['changed'] for v in result['metrics'].values())
compare('later-cohort-cannot-relabel-old-runs',candidate,1,root/'.flow/evidence/fn19/onboarding-examples/later-protocol.json',root/'.flow/evidence/fn19/onboarding-examples/later-references.json')
save(a.output/'controls.json',{'schema_version':1,'purpose':'Actual unchanged independent 2m native replay; changed/failed controls are synthetic receipt mutations, never candidate production geometry. Pine is not generated.','results':results,'third_species':'Existing onboarding verify.py independently checks actual unsupported-anatomy admission; comparison rejects mixing its later cohort with these runs.'})
print(a.output/'controls.json')
