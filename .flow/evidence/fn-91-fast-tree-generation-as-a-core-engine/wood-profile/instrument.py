"""Apply bounded observational instrumentation to a scratch checkout only."""
from pathlib import Path
import sys
root = Path(sys.argv[1])
p = root / 'crates/telperion-core/src/surface.rs'
s = p.read_text()
def replace(old, new):
    global s
    assert s.count(old) == 1, (old, s.count(old))
    s = s.replace(old, new)
replace('    tree.validate()?;', '''    let mut times = [0.0_f64; 11];
    let mut clock = std::time::Instant::now();
    tree.validate()?;''')
replace('    for path in &paths.runs {', '''    times[0] += clock.elapsed().as_secs_f64() * 1000.;
    clock = std::time::Instant::now();
    for path in &paths.runs {''')
replace('    ordered.sort_by', '''    times[1] += clock.elapsed().as_secs_f64() * 1000.;
    clock = std::time::Instant::now();
    ordered.sort_by''')
replace('    for (path, largest_radius) in ordered {', '''    times[2] += clock.elapsed().as_secs_f64() * 1000.;
    for (path, largest_radius) in ordered {
        clock = std::time::Instant::now();''')
replace('        let base = (mesh.positions.len() / 3) as u32;', '''        times[3] += clock.elapsed().as_secs_f64() * 1000.;
        clock = std::time::Instant::now();
        let base = (mesh.positions.len() / 3) as u32;''')
replace('        for i in 0..samples.len() - 1 {', '''        times[4] += clock.elapsed().as_secs_f64() * 1000.;
        clock = std::time::Instant::now();
        for i in 0..samples.len() - 1 {''')
replace('        mesh.normals.resize(mesh.positions.len(), 0.0);', '''        times[5] += clock.elapsed().as_secs_f64() * 1000.;
        clock = std::time::Instant::now();
        mesh.normals.resize(mesh.positions.len(), 0.0);
        times[6] += clock.elapsed().as_secs_f64() * 1000.;''')
replace('        mesh.dropped += normals::shade(', '        let (dropped, accumulate_ms, normalize_ms) = normals::shade(')
replace('        let end = u32::try_from(mesh.indices.len())', '''        mesh.dropped += dropped;
        times[7] += accumulate_ms;
        times[8] += normalize_ms;
        clock = std::time::Instant::now();
        let end = u32::try_from(mesh.indices.len())''')
replace('    finish(mesh, segments)', '''        times[9] += clock.elapsed().as_secs_f64() * 1000.;
    clock = std::time::Instant::now();
    let result = finish(mesh, segments);
    times[10] += clock.elapsed().as_secs_f64() * 1000.;
    PROFILE.with(|p| p.set(times));
    result''')
# Place per-run table timer inside loop, not once after it.
replace('''    }
        times[9] += clock.elapsed().as_secs_f64() * 1000.;''', '''        times[9] += clock.elapsed().as_secs_f64() * 1000.;
    }''')
s += '''\nstd::thread_local! { static PROFILE: std::cell::Cell<[f64; 11]> = const { std::cell::Cell::new([0.; 11]) }; }
pub fn diagnostic_profile() -> [f64; 11] { PROFILE.with(|p| p.get()) }
'''
p.write_text(s)
p = root / 'crates/telperion-core/src/surface/normals.rs'
s = p.read_text().replace(') -> Result<usize> {\n    let dropped', ') -> Result<(usize, f64, f64)> {\n    let clock = std::time::Instant::now();\n    let dropped',1)
s = s.replace('    normalize(&mut normals[base * 3..], facing)?;', '''    let accumulate_ms = clock.elapsed().as_secs_f64() * 1000.;
    let clock = std::time::Instant::now();
    normalize(&mut normals[base * 3..], facing)?;
    let normalize_ms = clock.elapsed().as_secs_f64() * 1000.;''',1).replace('    Ok(dropped)', '    Ok((dropped, accumulate_ms, normalize_ms))',1)
p.write_text(s)
p = root / 'crates/telperion-core/examples/wood_profile.rs'
s = p.read_text().replace('let stages: Vec<f64> = Vec::new();', 'let stages = surface::diagnostic_profile().to_vec();')
p.write_text(s)
