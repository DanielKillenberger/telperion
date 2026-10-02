import json, statistics as st, collections
C={}; S={}; L={}; T=collections.defaultdict(list)
for l in open('counts.jsonl'):
    d=json.loads(l); C[(d['preset'],d['seed'],d['rung'])]=d
for l in open('stats.txt'):
    p,s,r,tag,js=l.split(' ',4); k=(p,int(s),int(r))
    (S if tag=='LADDER_STATS' else L)[k]=json.loads(js.replace('None','null').replace('Some(','').replace(')',''))
for l in open('times.jsonl'):
    d=json.loads(l)
    if d['sample']>0: T[(d['preset'],d['seed'],d['rung'])].append(d['ms'])
abbr={'oregon-white-oak':'oak','european-beech':'beech','silver-birch':'birch','norway-spruce':'spruce','telperion':'telperion'}
names=['R0 today','R1 none','R2 allowance','R3a radial','R3b probe']
print('| Preset | Rung | Queries | vs R0 | Twig queries | Planned axes | q/axis | ms (median) | vs R0 | Nodes | Leaves | Nodes/m3 | Leaves/m3 |')
print('|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|')
for p in abbr:
  for s in (1,7):
    b=C[(p,s,0)]; bt=st.median(T[(p,s,0)])
    for r in range(5):
      c=C[(p,s,r)]; bp=c['by_purpose']; st_=S[(p,s,r)]; lv=L[(p,s,r)]
      twig=sum(bp[k] for k in ('twig_stride','twig_bisection','terminal_admission','curtain_band','twig_room'))
      t=st.median(T[(p,s,r)]); v=st_['crown_m3']
      print(f"| {abbr[p]} {s} | {names[r]} | {c['radius_queries']:,} | {c['radius_queries']/b['radius_queries']-1:+.0%} | {twig:,} | {c['planned_axes']:,} | {twig/max(c['planned_axes'],1):.1f} | {t:.1f} | {t/bt-1:+.0%} | {c['nodes']:,} | {lv['retained']:,} | {c['nodes']/v:.1f} | {lv['retained']/v:.0f} |")
print()
print('| Preset | Rung | Axes | Outside | Share | Median | p95 | Max | Worst above/below base/beside | Band nodes | Band lowest (share of h) |')
print('|---|---|--:|--:|--:|--:|--:|--:|---|--:|--:|')
for p in abbr:
  for s in (1,7):
    for r in range(5):
      d=S[(p,s,r)]
      print(f"| {abbr[p]} {s} | {names[r]} | {d['axes']:,} | {d['axes_outside']:,} | {d['share_outside']:.2%} | {d['median']:.3f} | {d['p95']:.3f} | {d['max']:.3f} | {'/'.join(map(str,d['worst_above_below_beside']))} | {d['band_nodes']:,} | {d['band_lowest']:.3f} |")
print()
print('purposes R0 seed1/7:')
for p in abbr:
  for s in (1,7):
    print(abbr[p],s,C[(p,s,0)]['by_purpose'])
print('detail levels:', {k:v['detail'] for k,v in S.items() if v['detail'] is not None})
print('timing spread (min-max warm ms) R0:', {f"{abbr[k[0]]}{k[1]}":(round(min(v),1),round(max(v),1)) for k,v in T.items() if k[2]==0})
print('stem axes:', {f"{abbr[k[0]]}{k[1]}r{k[2]}":v['stem_axes_out_max'] for k,v in S.items() if v['stem_axes_out_max'][0]>0})
