"""Conserved-path bound under the candidate group floor, not a universal impossibility proof."""
import json,pathlib,sys,numpy as np
source=pathlib.Path(sys.argv[1]) if len(sys.argv)>1 else pathlib.Path("/tmp/fn99-pass6-final-audit")
D=json.load(open(source/'oregon-white-oak-2.json'));n=D['structure'];x=np.array([a[:3] for a in n]);first=D['crossover'];minimum=np.full(len(n),np.inf);minimum[:first]=x[:first,1]
for i in range(len(n)-1,0,-1):minimum[n[i][3]]=min(minimum[n[i][3]],minimum[i])
owner=np.zeros(len(n),int);groups={}
for i in range(1,len(n)):
 owner[i]=owner[n[i][3]]
 if owner[i]==0 and i<first and minimum[i]>=D['upper_m']:owner[i]=i
 if owner[i]:groups.setdefault(int(owner[i]),[]).append(i)
c=json.load(open('.flow/evidence/fn9/pass6-window-rays.json'))['cases'][0];eye=np.array(c['camera']['position']);f=np.array(c['camera']['target'])-eye;f/=np.linalg.norm(f);r=np.cross(f,[0,1,0]);r/=np.linalg.norm(r);u=np.cross(r,f);px,py=c['pixel'];ray=f+np.tan(np.deg2rad(19))*(r*(px/960*2-1)*4/3+u*(1-py/720*2));ray/=np.linalg.norm(ray);rows=[]
for root,ids in groups.items():
 socket=n[root][3];base=x[socket];floor=min(D['upper_m'],x[ids,1].min());t=float((base-eye)@ray);uncon=eye+ray*t;t=min(t,(floor-eye[1])/ray[1]);closest=eye+ray*t;along={socket:0.}
 for i in ids:along[i]=along[n[i][3]]+float(np.linalg.norm(x[i]-x[n[i][3]]))
 longest=max((along[i],i) for i in ids);distance=float(np.linalg.norm(closest-base));rows.append(dict(root=root,socket=socket,original_group_floor_m=float(floor),unconstrained_closest_ray_point=uncon.tolist(),height_constrained_closest_ray_point=closest.tolist(),distance_to_height_constrained_ray_m=distance,maximum_path_length_m=longest[0],longest_path_endpoint=longest[1],optimistic_articulated_shortfall_m=distance-longest[0]))
rows.sort(key=lambda r:r['optimistic_articulated_shortfall_m']);pathlib.Path('.flow/evidence/fn9/pass7-articulation-bound.json').write_text(json.dumps(dict(method='Seed2 original verified pass6 ray. Original maximal upper-support subtree sockets and group minimum-height floor pinned. Conserved root-to-node path length is an optimistic fully articulated bound, allowing all joints to straighten independently; not a feasible whole-group solution or leaf-area result. Ray is clipped to the existing group floor. Leaves excluded.',optimistically_reachable_groups=sum(r['optimistic_articulated_shortfall_m']<=0 for r in rows),groups=rows),indent=2)+'\n');print(rows[:3])
