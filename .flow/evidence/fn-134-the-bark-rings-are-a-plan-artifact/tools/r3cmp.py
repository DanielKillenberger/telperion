import json,sys
b=[json.loads(l) for l in open('r3-base.jsonl')]
c=[json.loads(l) for l in open(sys.argv[1] if len(sys.argv)>1 else 'r3-cand.jsonl')]
key=lambda r:(r['id'],r['seed'],r['outputs'])
B={key(r):r for r in b}; C={key(r):r for r in c}
same=diff=0; higher=[]; lower=0; eq=0
for k in C:
    if B[k]['hash']==C[k]['hash']: same+=1
    else: diff+=1; print('HASH',k)
    d=C[k]['memory']-B[k]['memory']
    if d>0: higher.append((k,B[k]['memory']>>20,C[k]['memory']>>20,d/2**20))
    elif d<0: lower+=1
    else: eq+=1
print('builds',len(C),'same',same,'diff',diff,'mem equal',eq,'lower',lower,'higher',len(higher))
for h in higher: print(h)
