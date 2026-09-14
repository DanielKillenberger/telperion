"""State Round 3 convergence before the owner's additionally authorized re-pin."""
import json
from pathlib import Path
P = Path(__file__).resolve().parent
names = ['oregon-white-oak', 'norway-spruce', 'ordinary', 'telperion', 'laurelin']
def mature(path):
    return max((json.loads(x) for x in path.read_text().splitlines() if json.loads(x)['kind']=='growth'), key=lambda x:x['age'])
def bounds(row):
    return ' / '.join('(' + ', '.join(f'{x:.5f}' for x in row['bounds'][side]) + ')' for side in ['min','max'])
s = ['# Round 3 convergence before the authorized re-pin\n',
     'The owner authorized one additional identity, audit and look re-pin in Round 3, only after geometry convergence was recorded. This document is written before that move. The preceding comparison is preserved in `round3/CONVERGENCE-round2.md`.\n',
     'All rows use seed 7 and production growth at the derived mature age. Bounds are node bounds in metres. The fn30 column is the task-base measurement at ccb44eaf. Round 2 is the rejected state at bf92380.\n',
     '| Preset / state | Age | Nodes | Crossover | Placements | Min / max bounds |',
     '|---|---:|---:|---:|---:|---|']
for name in names:
    for label, folder in [('fn30','prechange'), ('round2','round3/before'), ('round3','measurements')]:
        row = mature(P/folder/f'{name}.jsonl')
        s.append(f'| {name} / {label} | {row["age"]:g} | {row["nodes"]:,} | {row["crossover"]:,} | {row["placements"]:,} | {bounds(row)} |')
s += ['\n## Population bounds\n', '| Preset | Node change from fn30 | Crossover change | Leaf ratio to fn30 | Node band |', '|---|---:|---:|---:|---|']
for name in names:
    a=mature(P/'prechange'/f'{name}.jsonl'); b=mature(P/'measurements'/f'{name}.jsonl')
    delta=(b['nodes']/a['nodes']-1)*100
    band=('inside' if abs(delta)<=15 else 'OUTSIDE') if name in names[:2] else 'reported; authored shedding restored'
    s.append(f'| {name} | {b["nodes"]-a["nodes"]:+,} ({delta:+.2f}%) | {b["crossover"]-a["crossover"]:+,} | {b["placements"]/a["placements"]:.3f}× | {band} |')
s += ['''
## Cause of the geometry change

Secondary thickening had shortened primary shoots permanently at birth. Primary planning now uses the pipe allocation before the annual secondary scale; new wood records physical radii with that scale. The fork split and monotone radius history are unchanged. A future crown base is capped at the branch's birth station, admitting current growth without granting permanent room below the attachment. Juvenile woody laterals develop through the existing local rule; scaffold stations retain their authored spacing.

The structural leader retains its terminal bud. Other axis tips retain the local terminal recruitment that filled fn30's crown. One existing lateral bud can form a short leafy shoot on an eligible new extension; it is never allocated twice. Annual twigs fit available room. Needle cohorts interleave their sites along the shoot, including its distal part, while stable station prefixes preserve cohort identities. The annual scheduling and the numeric preset values remain unchanged from Round 2.

These changes explain production skeleton, placements, mesh counts and bounds moving. The legacy envelope generator must retain its audit hashes; element meshes and material parameters remain unchanged. No renderer camera, shader, assertion or tolerance is part of the re-pin. The Ordinary clay look reference moves with its grown geometry. The exact HELD trait inventory remains unchanged in this round.
''']
(P/'CONVERGENCE.md').write_text('\n'.join(s))
