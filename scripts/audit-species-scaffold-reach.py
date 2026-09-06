import json,pathlib,numpy as np
rows=[]
for case in json.load(open('.flow/evidence/fn9/pass6-window-rays.json'))['cases']:
 s=case['seed'];d=json.load(open(f'/tmp/fn99-pass6-final-audit/oregon-white-oak-{s}.json'));nodes=d['structure'];first=d['crossover'];xyz=np.array([n[:3] for n in nodes]);minimum=np.full(len(nodes),np.inf);minimum[:first]=xyz[:first,1]
 for i in range(len(nodes)-1,0,-1):minimum[nodes[i][3]]=min(minimum[nodes[i][3]],minimum[i])
 owner=np.zeros(len(nodes),int);groups={}
 for i in range(1,len(nodes)):
  owner[i]=owner[nodes[i][3]]
  if owner[i]==0 and i<first and minimum[i]>=d['upper_m']:owner[i]=i
  if owner[i]:groups.setdefault(int(owner[i]),[]).append(i)
 eye=np.array(case['camera']['position']);f=np.array(case['camera']['target'])-eye;f/=np.linalg.norm(f);right=np.cross(f,[0,1,0]);right/=np.linalg.norm(right);up=np.cross(right,f);px,py=case['pixel'];ray=f+np.tan(np.deg2rad(19))*(right*(px/960*2-1)*4/3+up*(1-py/720*2));ray/=np.linalg.norm(ray)
 rr=[]
 for root,ids in groups.items():
  base=xyz[nodes[root][3]];radius=np.linalg.norm(xyz[ids]-base,axis=1).max();v=base-eye;dist=np.linalg.norm(v-ray*(v@ray));rr.append(dict(root=root,socket=nodes[root][3],nodes=len(ids),reach_m=float(radius),ray_distance_m=float(dist),shortfall_m=float(dist-radius)))
 rr.sort(key=lambda r:r['shortfall_m']);rows.append(dict(seed=s,ray=case,eligible_groups=len(groups),optimistically_reachable=sum(r['shortfall_m']<=0 for r in rr),closest=rr[:8]))
pathlib.Path('.flow/evidence/fn9/pass7-scaffold-reach.json').write_text(json.dumps(dict(method='Maximal structural subtrees whose structural support descendants are all above original 75% structural-height boundary. Original lower supports excluded. Radius includes structural and retained local descendants around the unchanged socket. Optimistic sphere only; not whole-group or visual acceptance.',cases=rows),indent=2)+'\n')
print([(r['seed'],r['optimistically_reachable'],r['closest'][0]) for r in rows])
