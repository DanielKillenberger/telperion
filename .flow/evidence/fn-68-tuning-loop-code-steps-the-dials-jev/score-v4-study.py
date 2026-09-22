import json,math
from pathlib import Path
def probability(a):
    v=a.get('noul') if isinstance(a,dict) else None
    return v if isinstance(v,(int,float)) and not isinstance(v,bool) and math.isfinite(v) and 0<=v<=1 else None
def select(case,answers,threshold=.8):
    for size in ['small','substantial']:
        value=case['candidates'].get(size);p=probability(answers.get(case['id']+'_'+size))
        if isinstance(value,(int,float)) and not isinstance(value,bool) and math.isfinite(value) and case['min']<=value<=case['max'] and value!=case['current'] and (value>case['current'])==(case['direction']=='increase') and p is not None and p>=threshold:return size
    return None
def score(m,answers):
    correct=false_yes=false_no=unsafe=accepted=acceptable=covered=actionable=invalid=0;rows=[]
    for c in m['cases']:
        for size,label in c['expected'].items():
            p=probability(answers.get(c['id']+'_'+size));observed=p is not None and p>=m['noul_threshold']
            invalid+=p is None
            correct+=p is not None and observed==label;false_yes+=observed and not label;false_no+=not observed and label
        action=select(c,answers,m['noul_threshold']);wanted=any(x is not None for x in c['acceptable_actions'])
        actionable+=wanted;covered+=wanted and action is not None
        accepted+=action is not None;acceptable+=action is not None and action in c['acceptable_actions']
        unsafe+=action in c['unsafe_candidates'] if action else 0
        rows.append({'id':c['id'],'group':c['group'],'action':action,'acceptable':action in c['acceptable_actions']})
    total=sum(len(c['expected']) for c in m['cases'])
    metrics={'candidate_accuracy':correct/total,'actionable_coverage':covered/actionable,'accepted_action_accuracy':acceptable/accepted if accepted else 0}
    return {'metrics':metrics,'candidate_correct':correct,'candidate_total':total,'invalid_or_missing':invalid,'false_yes':false_yes,'false_no':false_no,'unsafe_actions':unsafe,'accepted':accepted,'acceptable':acceptable,'covered':covered,'actionable':actionable,'cases':rows,'passes':unsafe==0 and all(v>=m['gate'][k] for k,v in metrics.items())}
if __name__=='__main__':
    from copy import deepcopy
    o=Path(__file__).parent/'v4-generated';m=json.loads((o/'manifest.json').read_text());answers={c['id']+'_'+s:{'noul':.95 if yes else .05} for c in m['cases'] for s,yes in c['expected'].items()}
    assert score(m,answers)['passes'];c=m['cases'][0]
    for a,b,want in [(.9,.9,'small'),(.9,.1,'small'),(.1,.9,'substantial'),(.1,.1,None),(.5,.5,None)]:assert select(c,{c['id']+'_small':{'noul':a},c['id']+'_substantial':{'noul':b}})==want
    for bad in [None,{}, {'noul':float('nan')},{'noul':True},{'noul':2},{'confidence':1}]:assert select(c,{c['id']+'_small':bad}) is None
    x=deepcopy(c);x['candidates']['small']=100;assert select(x,{c['id']+'_small':{'noul':1}}) is None
    for value in [True,c['current']-.01]:
        x=deepcopy(c);x['candidates']['small']=value;assert select(x,{c['id']+'_small':{'noul':1}}) is None
    assert select(c,{c['id']+'_small':{'noul':.8}})=='small'
    assert select(c,{c['id']+'_small':{'noul':.799999}}) is None
    assert score(m,{})['invalid_or_missing']==10
    result={'synthetic_only':True,'checks':'all-good scoring; both/single/neither/ambiguous; missing/malformed/nonfinite/bool/out-of-range noul; out-of-bounds numeric blocked','model_calls':0};(o/'scorer-tests.json').write_text(json.dumps(result)+'\n');print(json.dumps(result))
