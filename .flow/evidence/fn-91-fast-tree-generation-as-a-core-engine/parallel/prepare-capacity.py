from pathlib import Path
import shutil
root=Path.cwd();ev=root/'.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/parallel';s=Path((ev/'scratch-path.txt').read_text().strip())
for name in ['surface.rs','surface/parallel.rs','surface/normals.rs']:
 shutil.copy2(root/'crates/telperion-core/src'/name,s/'crates/telperion-core/src'/name)
p=s/'crates/telperion-core/src/surface/parallel.rs';t=p.read_text();t=t.replace('    thread::scope(|scope| -> Result<()> {', '''    eprintln!("PHASE_A positions={} coords={} paths={} distance={} angular={} descriptors={} longest={} workers={}",positions.capacity()*4,coords.capacity()*4,paths.nodes.capacity()*8+paths.runs.capacity()*size_of::<paths::Run>()+paths.forks.capacity(),distance.capacity()*8,angular.capacity()*size_of::<angular::Angular>(),runs.capacity()*size_of::<Run>(),longest,count);
    thread::scope(|scope| -> Result<()> {''',1)
t=t.replace('    for run in &runs {', '''    eprintln!("PHASE_B positions={} coords={} normals={} indices={} descriptors={} run_table={} workers={}",positions.capacity()*4,coords.capacity()*4,normals.capacity()*4,indices.capacity()*4,runs.capacity()*size_of::<Run>(),run_table.capacity()*size_of::<SurfaceRun>(),count);
    for run in &runs {''',1)
p.write_text(t)
