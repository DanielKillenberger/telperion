"""Run every requested gate in full, preserving every exit and unedited log."""
import json
import subprocess
from pathlib import Path
root=Path(__file__).resolve().parents[3]
out=root/'.flow/evidence/fn31'
rows=[]
for command,name in [
    ('cargo fmt --all -- --check','fmt-final'),
    ('cargo clippy --workspace --all-targets -- -D warnings','clippy-final'),
    ('cargo test --release --workspace','workspace-final'),
    ('npm test','npm-final'),
    ('npm run typecheck','typecheck-final')]:
    log=out/'logs'/f'{name}.log'
    print('RUN',command,flush=True)
    with log.open('w') as stream:
        r=subprocess.run(command,shell=True,cwd=root,stdout=stream,stderr=stream)
    rows.append({'command':command,'exit':r.returncode,'log':str(log)})
    (out/'gates-final.json').write_text(json.dumps(rows,indent=2)+'\n')
    print('EXIT',r.returncode,str(log),flush=True)
