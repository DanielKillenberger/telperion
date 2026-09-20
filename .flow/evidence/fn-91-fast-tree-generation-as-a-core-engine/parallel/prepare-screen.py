from pathlib import Path
import shutil
root=Path.cwd();ev=root/'.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/parallel';s=Path((ev/'scratch-path.txt').read_text().strip())
for name in ['surface.rs','surface/parallel.rs','surface/normals.rs']:
 shutil.copy2(root/'crates/telperion-core/src'/name,s/'crates/telperion-core/src'/name)
p=s/'crates/telperion-core/src/surface.rs';t=p.read_text();t += '''
static PARALLEL_ATTEMPTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static PARALLEL_FALLBACKS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub fn diagnostic_parallel() -> (usize,usize) {use std::sync::atomic::Ordering; (PARALLEL_ATTEMPTS.load(Ordering::Relaxed),PARALLEL_FALLBACKS.load(Ordering::Relaxed))}
'''
t=t.replace('            return match parallel::build(', '            PARALLEL_ATTEMPTS.fetch_add(1,std::sync::atomic::Ordering::Relaxed);\n            return match parallel::build(').replace('Err(_) => build_mode(tree, height, params, None, None, false),','Err(_) => {PARALLEL_FALLBACKS.fetch_add(1,std::sync::atomic::Ordering::Relaxed);build_mode(tree, height, params, None, None, false)},')
p.write_text(t)
p=s/'crates/telperion-core/examples/radius_order.rs';t=p.read_text();t=t.replace('"contact_bytes":compact.contact_bytes()', '"contact_bytes":compact.contact_bytes(),"parallel":surface::diagnostic_parallel()');p.write_text(t)
