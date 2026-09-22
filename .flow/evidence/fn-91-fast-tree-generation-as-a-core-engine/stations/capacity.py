from pathlib import Path
import json
here=Path(__file__).resolve().parent
root=Path(json.loads((here/'paths.json').read_text())['root'])
helper='''
thread_local! {
    static FN91_PREFIX: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static FN91_PEAK: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}
pub fn fn91_prefix(n: usize) { FN91_PREFIX.with(|p| p.set(n)); }
pub fn fn91_observe(n: usize) { FN91_PREFIX.with(|p| FN91_PEAK.with(|m| m.set(m.get().max(p.get()+n)))); }
pub fn fn91_reset() { FN91_PREFIX.with(|p| p.set(0)); FN91_PEAK.with(|p| p.set(0)); }
pub fn fn91_peak() -> usize { FN91_PEAK.with(|p| p.get()) }
'''
for variant in ['baseline','candidate']:
 d=root/variant
 p=d/'crates/telperion-core/src/lib.rs'; p.write_text(p.read_text()+helper)
 p=d/'crates/telperion-core/src/foliage/placement.rs'; s=p.read_text(); idx=s.index('pub(super) fn bearing_runs'); a,b=s[:idx],s[idx:]
 child='children.capacity()*std::mem::size_of_val(&children[0])'
 if variant=='baseline': child+=' + children.iter().map(|v| v.capacity()*8).sum::<usize>()'
 b=b.replace('    runs\n}',f'''    let run_bytes = runs.capacity()*24 + runs.iter().map(|v| v.capacity()*8).sum::<usize>();
    let child_bytes = {child};
    crate::fn91_observe(child_bytes + run_bytes);
    eprintln!("CAP_CHILD {{}} {{}}", child_bytes, run_bytes);
    runs
}}'''); p.write_text(a+b)
 p=d/'crates/telperion-core/src/foliage/prepared.rs'; s=p.read_text(); s=s.replace('    for nodes in placement::bearing_runs(tree, p) {','''    let diag_runs = placement::bearing_runs(tree, p);
    let diag_outer = diag_runs.capacity()*24;
    let mut diag_remaining = diag_runs.iter().map(|v| v.capacity()*8).sum::<usize>();
    let mut diag_point_peak = 0usize;
    let mut diag_output_peak = 0usize;
    for nodes in diag_runs {
        let diag_run_bytes = diag_outer + diag_remaining;
        diag_remaining -= nodes.capacity()*8;''')
 s=s.replace('        let length = *along.last().unwrap();','''        let diag_points = points.capacity()*24 + along.capacity()*8;
        diag_point_peak = diag_point_peak.max(diag_points);
        crate::fn91_prefix(diag_run_bytes + diag_points + segments.capacity()*std::mem::size_of::<StationSegment>());
        crate::fn91_observe(0);
        let length = *along.last().unwrap();''')
 s=s.replace('        // The original reverse search', '        let diag_frames_bytes = '+ ('frames.capacity()*72;' if variant=='baseline' else '0;')+'\n        // The original reverse search')
 s=s.replace('                segments.push(StationSegment {','''                let diag_old_output = segments.capacity()*std::mem::size_of::<StationSegment>();
                segments.push(StationSegment {''')
 s=s.replace('                tile_first += tile_count;','''                let diag_output = segments.capacity()*std::mem::size_of::<StationSegment>();
                diag_output_peak = diag_output_peak.max(diag_output);
                crate::fn91_prefix(diag_run_bytes + diag_points + diag_output + diag_frames_bytes);
                crate::fn91_observe(if diag_output > diag_old_output { diag_old_output } else { 0 });
                tile_first += tile_count;''')
 s=s.replace('    Ok(Some((segments, total_count, contacts)))','''    eprintln!("CAP_STATIONS {} {} {}", diag_point_peak, diag_output_peak, crate::fn91_peak());
    Ok(Some((segments, total_count, contacts)))'''); p.write_text(s)
 if variant=='baseline':
  p=d/'crates/telperion-core/src/foliage/station.rs'; s=p.read_text().replace('    result\n}', '''    crate::fn91_observe(segments.capacity()*24 + tangents.capacity()*24 + result.capacity()*72);
    result
}'''); p.write_text(s)
 p=d/'crates/telperion-core/examples/station_screen.rs'; s=p.read_text().replace('        let stations = foliage::prepared::prepare_compact_stations(', '        telperion_core::fn91_reset();\n        let stations = foliage::prepared::prepare_compact_stations('); p.write_text(s)
