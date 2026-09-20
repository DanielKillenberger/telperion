from pathlib import Path
import sys
root=Path(sys.argv[1]); core=root/'crates/telperion-core'
(core/'src/lib.rs').write_text((core/'src/lib.rs').read_text()+'\n#[doc(hidden)] pub mod cpu_diagnostic;\n')
(core/'src/cpu_diagnostic.rs').write_text('''use std::{cell::RefCell, time::Instant};
thread_local! { static EVENTS: RefCell<Vec<(&'static str, f64)>> = const { RefCell::new(Vec::new()) }; }
pub fn mark(name: &'static str, start: Instant) { let ms=start.elapsed().as_secs_f64()*1000.; EVENTS.with(|e| e.borrow_mut().push((name,ms))); }
pub fn take() -> Vec<(&'static str,f64)> { EVENTS.with(|e| std::mem::take(&mut *e.borrow_mut())) }
''')
def stages(path, pairs):
 p=core/'src'/path; s=p.read_text()
 for old,new in pairs:
  assert old in s,(path,old)
  s=s.replace(old,new,1)
 p.write_text(s)
def start(): return '\n        let cpu_t = std::time::Instant::now();\n'
def lap(name): return f'\n        crate::cpu_diagnostic::mark("{name}", cpu_t);'+start()
stages('branching/specimen.rs',[
 ('let mut s = Self::new(params, radii)?;', start()+'let mut s = Self::new(params, radii)?;'+lap('skeleton.new')),
 ('s.step(usize::MAX, 0)?;', 's.step(usize::MAX, 0)?;'+lap('skeleton.step_structural')),
 ('s.step(0, usize::MAX)?;', 's.step(0, usize::MAX)?;'+lap('skeleton.step_local')),
 ('s.shed = finish(&mut s.tree, params, radii)?;', 's.shed = finish(&mut s.tree, params, radii)?;'+lap('skeleton.finish')),
 ('s.remap_after_shedding();', 's.remap_after_shedding();\ncrate::cpu_diagnostic::mark("skeleton.final_remap_identify",cpu_t);'),
 ('let first = self.tree.crossover;\n        let mut tail', start()+'let first = self.tree.crossover;\n        let mut tail'),
 ('let result = self.scaffold.advance(', lap('step.split_config')+'let result = self.scaffold.advance('),
 ('let added = self.tree.nodes.len() - first;', lap('step.scaffold')+'let added = self.tree.nodes.len() - first;'),
 ('radius::solve(&mut self.tree, self.params.envelope, self.radii)?;', lap('step.remap')+'radius::solve(&mut self.tree, self.params.envelope, self.radii)?;'+lap('step.radius')),
 ('self.identify();\n        self.local.seed(', 'self.identify();'+lap('step.identify_before')+'self.local.seed('),
 ('if !self.tree.diagnostics.node_capped {',lap('step.seed')+'if !self.tree.diagnostics.node_capped {'),
 ('self.identify();\n        Ok(())',''+lap('step.local_advance')+'self.identify();\ncrate::cpu_diagnostic::mark("step.identify_after",cpu_t);\n        Ok(())'),
])
stages('branching.rs',[
 ('let twigs = params.twigs.resolved()?;',start()+'let twigs = params.twigs.resolved()?;'),
 ('radius::solve(tree, params.envelope, radii)?;',lap('finish.shed')+'radius::solve(tree, params.envelope, radii)?;'+lap('finish.radius')),
 ('Ok(removed)','crate::cpu_diagnostic::mark("finish.tips",cpu_t);\n    Ok(removed)')])
stages('surface/compact.rs',[
 ('    tree.validate()?;\n    let mut edges',start()+'    tree.validate()?;'+lap('compact.wrapper_validate')+'    let mut edges'),
 ('    let surface = prepare_inner',lap('compact.edges_allocate')+'    let surface = prepare_inner'),
 ('    tree.validate()?;\n    params.validate()?;',start()+'    tree.validate()?;\n    params.validate()?;'),
 ('    let paths = paths(&tree.nodes)?;',lap('compact.validate')+'    let paths = paths(&tree.nodes)?;'+lap('compact.paths')),
 ('    let height = height.max(1e-6);',lap('compact.validate_solved')+'    let height = height.max(1e-6);'),
 ('    for path in &paths.runs {',lap('compact.distance_allocate')+'    for path in &paths.runs {'),
 ('    ordered.sort_by(',lap('compact.order_samples')+'    ordered.sort_by('),
 ('    let mut base = 0;',lap('compact.sort')+'    let mut base = 0;'),
 ('    Ok(out)','crate::cpu_diagnostic::mark("compact.sample_frame_pack_contact",cpu_t);\n    Ok(out)')])
stages('foliage/prepared.rs',[
 ('    placement::validate(tree, envelope, p, twig)?;',start()+'    placement::validate(tree, envelope, p, twig)?;'),
 ('    for nodes in placement::bearing_runs(tree, p) {',lap('stations.validation_contact')+'    for nodes in placement::bearing_runs(tree, p) {'),
 ('    Ok(Some((segments, total_count, contacts)))','crate::cpu_diagnostic::mark("stations.runs_frames_descriptors",cpu_t);\n    Ok(Some((segments, total_count, contacts)))')])
stages('radius.rs',[
 ('    tree.validate()?;',start()+'    tree.validate()?;'+lap('radius.validate_tree')),
 ('    let count = tree.crossover;',lap('radius.validate_params')+'    let count = tree.crossover;'),
 ('    tree.validate_solved()',lap('radius.solve_body')+'    let result = tree.validate_solved();\ncrate::cpu_diagnostic::mark("radius.validate_solved",cpu_t);\nresult')])
for path,expr,label in [('branching/local/advance.rs','tree.validate_range(first..tree.nodes.len(), true)','local.validation'),('branching/scaffold/frontier.rs','b.tree.validate()','scaffold.validation')]:
 stages(path,[(expr,start()+'let result = '+expr+';\ncrate::cpu_diagnostic::mark("'+label+'",cpu_t);\nresult')])
p=core/'examples/cpu_profile.rs'; s=p.read_text().replace('        let total = Instant::now();','        telperion_core::cpu_diagnostic::take();\n        let total = Instant::now();').replace('let stages: Vec<(&str, f64)> = Vec::new(); // instrumented replacement','let stages = telperion_core::cpu_diagnostic::take();'); p.write_text(s)
