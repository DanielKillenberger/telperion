import hashlib, json, os, platform, shutil, subprocess, tempfile
from pathlib import Path
root=Path.cwd(); ev=Path(__file__).resolve().parent
base='6caa08ebbc2387ae55f8a823cf6a36613c27db58'
scratch=Path(tempfile.mkdtemp(prefix='telperion-bias-'))
subprocess.run(['bash','-c','git archive "$1" | tar -x -C "$2"','bash',base,str(scratch)],check=True)
(ev/'scratch-path.txt').write_text(str(scratch)+'\n')
shutil.copytree(root/'src/browser/render',scratch/'baseline-render')
shutil.copy2(ev/'bias_screen.rs',scratch/'crates/telperion-core/examples/bias_screen.rs')
provenance={'base':base,'scratch':str(scratch),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'platform':platform.platform(),'wasm_sha256':hashlib.sha256((scratch/'baseline-render/telperion_render_bg.wasm').read_bytes()).hexdigest(),'build':'cargo build --release -p telperion-core --example bias_screen','profile':'release opt-level 3 lto true codegen-units 1','baseline':'green via .8 handoff; .9 evidence only; .10 exactly reverted','protocol':'serial baseline then candidate per fixture; first+3 warm; bincode entire Tree and report.shed outside timing; no leaf expansion'}
(ev/'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')
env=dict(os.environ,CARGO_TARGET_DIR=str(scratch/'target'))
for mode in ['baseline','candidate']:
    if mode=='candidate': subprocess.run(['git','apply',str(ev/'candidate.patch')],cwd=scratch,check=True)
    with (ev/f'{mode}-build.log').open('w') as log:
        subprocess.run(['cargo','build','--release','-p','telperion-core','--example','bias_screen'],cwd=scratch,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=600,check=True)
    shutil.copy2(scratch/'target/release/examples/bias_screen',scratch/mode)
print(scratch)
