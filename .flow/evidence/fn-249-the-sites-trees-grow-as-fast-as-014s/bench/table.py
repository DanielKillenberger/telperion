import json,sys,collections
# argv: base.jsonl final.jsonl v014.jsonl -> markdown: end-to-end ms best of 3
def load(f):
    out=collections.defaultdict(list)
    for l in open(f):
        r=json.loads(l)['run']; st='cold' if r['order']==0 else 'warm'
        for n,x in r['per_n'].items():
            out[(r['species'],r['seed'],int(n),st)].append((r['grow_ms'],x['end_ms']))
    return {k:(min(a for a,_ in v),min(b for _,b in v)) for k,v in out.items()}
B,F,V=(load(f) for f in sys.argv[1:4])
short={'oregon-white-oak':'oak','silver-birch':'birch','norway-spruce':'spruce'}
print('| Tree | n | F4 before, cold | now, cold | now, warm | 0.1.4, cold | 0.1.4, warm | now ÷ 0.1.4, cold / warm |')
print('|---|--:|--:|--:|--:|--:|--:|--:|')
for sp in ['oregon-white-oak','silver-birch','norway-spruce']:
  for seed in [1234,5678]:
    for n in [24,30,36,48]:
      b=B[(sp,seed,n,'cold')][1]; fc=F[(sp,seed,n,'cold')][1]; fw=F[(sp,seed,n,'warm')][1]
      vc=V[(sp,seed,n,'cold')][1]; vw=V[(sp,seed,n,'warm')][1]
      print(f'| {short[sp]} {seed} | {n} | {b:,.0f} | {fc:,.0f} | {fw:,.0f} | {vc:,.0f} | {vw:,.0f} | {fc/vc:.1f}× / {fw/vw:.1f}× |')
print()
print('| Tree | F4 before grow, cold / warm | now grow, cold / warm | 0.1.4 grow, cold / warm |')
print('|---|--:|--:|--:|')
for sp in ['oregon-white-oak','silver-birch','norway-spruce']:
  for seed in [1234,5678]:
    k=lambda D,s: D[(sp,seed,36,s)][0]
    print(f'| {short[sp]} {seed} | {k(B,"cold"):,.0f} / {k(B,"warm"):,.0f} | {k(F,"cold"):,.0f} / {k(F,"warm"):,.0f} | {k(V,"cold"):,.0f} / {k(V,"warm"):,.0f} |')
