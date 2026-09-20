"""Exactly one call per frozen stage; independent side accounting, no runtime activation."""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import time

ROOT=Path(__file__).resolve().parent
WORKTREE=ROOT.parents[2]
SCRIPT=WORKTREE/'scripts/reference-first-codex.py'
RUNTIME=WORKTREE/'.flow/tmp/fn68-pilot-run/run.json'
RUNTIME_HASH='d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e'
def digest(b):return hashlib.sha256(b).hexdigest()
def read(p):return json.loads(p.read_text())
def new(p,v):
    with p.open('x') as f:json.dump(v,f,indent=2)
def main():
    parser=argparse.ArgumentParser();parser.add_argument('stage',choices=['a','b']);parser.add_argument('--execute',action='store_true');args=parser.parse_args()
    envelope=read(ROOT/f'reference-first-{args.stage}-request.json')
    spec=importlib.util.spec_from_file_location('adapter',SCRIPT);adapter=importlib.util.module_from_spec(spec);spec.loader.exec_module(adapter)
    paths,schema,prompt=adapter.prepare(envelope)
    frozen={'request_sha256':envelope['request_sha256'],'request_file_sha256':digest((ROOT/f'reference-first-{args.stage}-request.json').read_bytes()),'adapter_sha256':digest(SCRIPT.read_bytes()),'prompt':prompt,'prompt_sha256':digest(prompt.encode()),'schema':schema,'schema_sha256':digest(json.dumps(schema).encode()),'images':[{'path':str(p),'sha256':digest(p.read_bytes())}for p in paths],'model':'gpt-6-astra','effort':'medium'}
    fp=ROOT/f'reference-first-{args.stage}-frozen.json'
    if fp.exists():assert read(fp)==frozen
    else:new(fp,frozen)
    reserve=25000 if args.stage=='a' else 35000
    prior=475672
    if args.stage=='b':
        previous=read(ROOT/'reference-first-a-settlement.json');assert previous['status']=='settled'
        prior=previous['cumulative_actual_tokens']
        assert (ROOT/'reference-first-inventory.json').exists()
    assert prior+reserve<=550000
    if args.stage=='a':assert prior+60000<=550000
    print(json.dumps({'stage':args.stage,'images':len(paths),'reserve':reserve,'prior':prior,'remaining_after_reservation':550000-prior-reserve,'prompt_bytes':len(prompt.encode()),'schema_bytes':len(json.dumps(schema).encode())}),flush=True)
    if not args.execute:return
    assert digest(RUNTIME.read_bytes())==RUNTIME_HASH
    new(ROOT/f'reference-first-{args.stage}-reservation.json',{'authority':'Owner answered YES to raise cumulative ceiling520000 to550000 for two-stage Astra-medium test, max60000 combined reservations, existing images, no retries','prior_actual_tokens':prior,'reserve':reserve,'cap':550000,'attempted_visual_passes':18 if args.stage=='a' else 19,'frozen_sha256':digest(fp.read_bytes()),'status':'reserved_before_dispatch','runtime_sha256':RUNTIME_HASH})
    started=time.monotonic()
    run=subprocess.run(['python3',str(SCRIPT),'--model','gpt-6-astra','--effort','medium'],input=json.dumps(envelope).encode(),capture_output=True,cwd=WORKTREE,timeout=330)
    (ROOT/f'local/reference-first-{args.stage}-stdout.json').write_bytes(run.stdout)
    (ROOT/f'local/reference-first-{args.stage}-stderr.txt').write_bytes(run.stderr)
    assert run.returncode==0,'failed call; reservation retained, no retry'
    receipt=json.loads(run.stdout)
    assert receipt['model']=='gpt-6-astra' and receipt['effort']=='medium'
    assert receipt['request_sha256']==envelope['request_sha256'] and receipt['dispatched_prompt_sha256']==frozen['prompt_sha256'] and receipt['schema_sha256']==frozen['schema_sha256']
    assert receipt['image_sha256']==[x['sha256'] for x in frozen['images']]
    events=receipt.pop('raw_events');receipt['raw_events_sha256']=digest(events.encode())
    (ROOT/f'local/reference-first-{args.stage}-events.jsonl').write_text(events)
    new(ROOT/f'reference-first-{args.stage}-receipt.json',receipt)
    assert receipt['usage'] is not None,'unknown usage: reservation retained; stop'
    used=receipt['usage']['input_tokens']+receipt['usage']['output_tokens']
    status='settled' if used<=reserve and receipt['status']=='ok' else 'failed_or_over_reservation_stop'
    settlement={'status':status,'actual_tokens':used,'cumulative_actual_tokens':prior+used,'elapsed_seconds':time.monotonic()-started,'receipt_sha256':digest((ROOT/f'reference-first-{args.stage}-receipt.json').read_bytes()),'usage':receipt['usage'],'model':receipt['model'],'model_identity_basis':'requested command argument; actual resolved identity not exposed','effort':receipt['effort']}
    new(ROOT/f'reference-first-{args.stage}-settlement.json',settlement)
    assert digest(RUNTIME.read_bytes())==RUNTIME_HASH
    print(json.dumps(settlement),flush=True)
    assert status=='settled','stop; no next stage'
if __name__=='__main__':main()
