"""Compare whole-secondary terminal allocation: BEFORE AFTER OUTPUT; requires NumPy."""
import json,pathlib,sys,numpy as np
before=pathlib.Path(sys.argv[1]);after=pathlib.Path(sys.argv[2]);output=pathlib.Path(sys.argv[3])
rows=[]
for seed in [1,2,3,1982700925,281313742,2271779095,4250668600]:
 a=json.load(open(before/f'norway-spruce-{seed}.json'));b=json.load(open(after/f'norway-spruce-{seed}.json'));first=a['crossover'];n=len(a['structure']);parents=np.array([r[3] or 0 for r in a['structure']]);x=np.array([r[:3] for r in a['structure']]);y=np.array([r[:3] for r in b['structure']]);op=np.zeros(n,int);sys=np.zeros(n,int);members={};group={}
 for i in range(1,n):
  sys[i]=sys[parents[i]]
  if i<first:
   v=x[i]-x[parents[i]]
   if sys[i]==0 and v[1]/np.linalg.norm(v)<-.5:sys[i]=i
  else:
   op[i]=i if parents[i]<first else op[parents[i]];group.setdefault(int(op[i]),[]).append(i)
  if sys[i]:members.setdefault(int(sys[i]),[]).append(i)
 systems=[]
 for root,ids in members.items():
  structure=[i for i in ids if i<first];tips=np.array([i for i in ids if a['structure'][i][5]=='Twig'],int)
  if not len(tips):continue
  top=x[parents[root],1];bottom=x[structure,1].min();length=np.linalg.norm(x[tips]-x[parents[tips]],axis=1);moved=np.linalg.norm(y[tips]-x[tips],axis=1)>1e-8;single=np.array([len(group.get(int(op[i]),[]))<3 for i in tips]);systems.append(dict(root=root,socket=int(parents[root]),structural_nodes=len(structure),terminal_edges=len(tips),terminal_length_m=float(length.sum()),moved_terminal_edges=int(moved.sum()),moved_terminal_length_m=float(length[moved].sum()),single_group_terminal_edges=int(single.sum()),single_group_terminal_length_m=float(length[single].sum()),below_structural_span_before=int((x[tips,1]<bottom).sum()),below_structural_span_after=int((y[tips,1]<bottom).sum()),bands_before=np.histogram(x[tips,1],bins=np.linspace(bottom,top,5))[0].tolist(),bands_after=np.histogram(y[tips,1],bins=np.linspace(bottom,top,5))[0].tolist()))
 totals={k:sum(s[k] for s in systems) for k in ['terminal_edges','terminal_length_m','moved_terminal_edges','moved_terminal_length_m','single_group_terminal_edges','single_group_terminal_length_m','below_structural_span_before','below_structural_span_after']};rows.append(dict(seed=seed,systems=len(systems),totals=totals,per_system=systems))
 print(seed,totals,flush=True)
output.write_text(json.dumps(dict(method='Original whole secondary IDs and structural forks pinned. Measures all NodeKind::Twig edges and their centreline lengths, a guaranteed needle-bearing subset; excludes slender non-Twig bearing wood. Terminal edge counts are not needles or biological axes. Four equal-height structural-span bands; below-span endpoints reported separately. Single-group classification is before pass7.',cases=rows),indent=2)+'\n')
