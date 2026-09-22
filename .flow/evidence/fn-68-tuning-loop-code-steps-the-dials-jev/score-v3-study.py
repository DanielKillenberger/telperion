"""Offline development-study scorer; never dispatches or activates runtime policy."""
import json,math
from pathlib import Path

def score(m,directions,sizes):
    counts={k:[0,0] for k in m['gates']}; unsafe=[]; rows=[]
    def judged(answer,allowed):
        if not isinstance(answer,dict):return None
        c=answer.get('confidence');v=answer.get('choice')
        return v if isinstance(c,(int,float)) and not isinstance(c,bool) and math.isfinite(c) and m['confidence']<=c<=1 and v in allowed else None
    def add(k,ok):counts[k][0]+=int(ok);counts[k][1]+=1
    for case in m['cases']:
        i=case['id'];expected=case['expected'];available=case['available']
        d=judged(directions.get(i),[*available,'hold','insufficient_evidence'])
        eligible=d in available
        s=judged(sizes.get(i),[*available[d],'insufficient_evidence']) if eligible else None
        action=(d,s) if eligible and s in available[d] else None
        want_action=expected['size'] in ['small','substantial']
        direction_ok=d==expected['direction'];size_ok=s==expected['size']
        add('direction',direction_ok)
        if expected['size'] is not None:
            add('unconditional_expected_size',direction_ok and size_ok)
            if direction_ok and eligible:add('conditional_size',size_ok)
        strict=direction_ok and (expected['size'] is None or size_ok)
        add('strict_stage',strict)
        # Missing/invalid answers are not accepted evidence of intentional abstention.
        valid_disposition=(d in ['hold','insufficient_evidence']) or (eligible and s is not None)
        composed=action==(expected['direction'],expected['size']) if want_action else action is None and valid_disposition
        add('composed',composed)
        if want_action:add('actionable_coverage',action is not None and direction_ok)
        if action and (not want_action or not direction_ok or s in m['unsafe_candidates'].get(i,[])):
            unsafe.append(i)
        rows.append({'id':i,'direction':d,'size':s,'action':action,'strict':strict,'composed':composed})
    metrics={k:{'correct':v[0],'total':v[1],'accuracy':v[0]/v[1] if v[1] else None} for k,v in counts.items()}
    return {'metrics':metrics,'unsafe':unsafe,'cases':rows,'passes':not unsafe and all(v['accuracy'] is not None and v['accuracy']>=m['gates'][k] for k,v in metrics.items())}

def tests(m):
    from copy import deepcopy
    answer=lambda v:{'choice':v,'confidence':.9}
    d={c['id']:answer(c['expected']['direction']) for c in m['cases']}
    s={c['id']:answer(c['expected']['size']) for c in m['cases'] if c['expected']['size'] is not None}
    checks=[]
    assert score(m,d,s)['passes'];checks.append('all expected answers pass')
    x=deepcopy(d);x['harmful_steps']=answer('insufficient_evidence');a=score(m,x,s)
    assert a['metrics']['direction']['correct']==7 and a['metrics']['strict_stage']['correct']==7
    assert a['metrics']['unconditional_expected_size']['correct']==4 and a['metrics']['composed']['correct']==8
    checks.append('premature harmful abstention loses direction/strict/size fidelity, not composed no-action')
    x=deepcopy(s);x['gap_probe']=answer('substantial');assert 'gap_probe' in score(m,d,x)['unsafe'];checks.append('known harmful larger gap probe unsafe')
    x=deepcopy(d);x['lobes_probe']=answer('decrease');assert 'lobes_probe' in score(m,x,s)['unsafe'];checks.append('opposite direction unsafe')
    for bad in [None,{},answer('invalid'),{'choice':'increase','confidence':.49},{'choice':'increase','confidence':float('nan')},answer('insufficient_evidence')]:
        x=deepcopy(d);x['lobes_probe']=bad;a=score(m,x,s)
        assert a['cases'][0]['action'] is None and not a['cases'][0]['strict']
    checks.append('missing/invalid/unknown/low/nonfinite direction blocks action')
    for bad in [None,{},answer('invalid'),{'choice':'small','confidence':.49},answer('insufficient_evidence')]:
        x=deepcopy(s);x['lobes_probe']=bad;assert score(m,d,x)['cases'][0]['action'] is None
    checks.append('missing/invalid/unknown/low size blocks action')
    return checks

if __name__=='__main__':
    root=Path(__file__).parent/'v3-generated';m=json.loads((root/'manifest.json').read_text())
    result={'synthetic_only':True,'checks':tests(m),'model_calls':0}
    (root/'scorer-tests.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
