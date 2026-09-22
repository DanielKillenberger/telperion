from pathlib import Path
import subprocess,json,os,hashlib,statistics,struct
here=Path(__file__).resolve().parent
root=Path(json.loads((here/'paths.json').read_text())['root'])
bins={v: root/v/'target/release/examples/station_screen' for v in ['baseline','candidate']}
hashes={v:hashlib.sha256(b.read_bytes()).hexdigest() for v,b in bins.items()}
assert hashes['baseline'] != hashes['candidate']
(here/'binary-hashes.json').write_text(json.dumps(hashes,indent=2)+'\n')
records=[]
for preset in ['oregon-white-oak','norway-spruce']:
 for seed in [1,7]:
  rows={}
  dumps={}
  for variant,binary in bins.items():
   prefix=root/f'{variant}-{preset}-{seed}'
   result=subprocess.run([str(binary),preset,str(seed)],env={**os.environ,'STATION_DUMP':str(prefix)},text=True,capture_output=True,check=True)
   (here/f'{variant}-{preset}-{seed}.jsonl').write_text(result.stdout)
   rows[variant]=[json.loads(l) for l in result.stdout.splitlines() if json.loads(l)['event']=='preparation']
   dumps[variant]=Path(f'{prefix}-0.bin').read_bytes()
   for sample in [1,2,3]:
    f=Path(f'{prefix}-{sample}.bin'); assert f.read_bytes()==dumps[variant],(variant,preset,seed,'repeat',sample); f.unlink()
  a,b=dumps['baseline'],dumps['candidate']; assert len(a)==len(b) and a[:4]==b[:4]
  size=12+12*8+16+9*8
  assert (len(a)-4)%size==0
  maxerr=0.; changed=0
  for offset in range(4,len(a),size):
   frame=offset+size-72
   assert a[offset:frame]==b[offset:frame],(preset,seed,'non-frame',offset)
   fa=struct.unpack_from('<9d',a,frame); fb=struct.unpack_from('<9d',b,frame)
   assert fa[:3]==fb[:3],(preset,seed,'tangent')
   err=max(abs(x-y) for x,y in zip(fa,fb)); maxerr=max(maxerr,err); changed+=a[frame:frame+72]!=b[frame:frame+72]
  assert maxerr<1e-10,(preset,seed,maxerr)
  med={v:statistics.median(x['stations_ms'] for x in rows[v][1:]) for v in bins}
  r={'preset':preset,'seed':seed,'warm_ms':med,'speedup':med['baseline']/med['candidate'],'saving_ms':med['baseline']-med['candidate'],'frame_max_component_error':maxerr,'changed_frames':changed,'records':(len(a)-4)//size,'repeatability':'all samples exact','nonframe_fields':'exact'}
  records.append(r); print(json.dumps(r),flush=True)
(here/'screen-results.json').write_text(json.dumps(records,indent=2)+'\n')
