import json,sys,collections
# argv: jsonl files; prints best-of-3 per (wasm, species, seed, cold/warm, n): grow, end, hash
rows=collections.defaultdict(list); hashes=collections.defaultdict(set); loads=[]
for f in sys.argv[1:]:
    for l in open(f):
        d=json.loads(l); r=d['run']; loads.append(d['load'])
        state='cold' if r['order']==0 else 'warm'
        for n,x in r['per_n'].items():
            rows[(r['wasm'].split('/')[-1],r['species'],r['seed'],state,int(n))].append((r['grow_ms'],x['end_ms']))
            hashes[(r['species'],r['seed'],int(n))].add((r['wasm'].split('/')[-1],x['hash']))
for k in sorted(rows):
    v=rows[k]; print(*k, 'grow %.0f end %.0f'%(min(a for a,_ in v),min(b for _,b in v)), 'reps',len(v))
print('load', min(loads), max(loads))
for k in sorted(hashes):
    hs={h for _,h in hashes[k]}
    print('HASH', *k, 'IDENTICAL' if len(hs)==1 else 'DIFFER '+str(sorted(hashes[k])))
