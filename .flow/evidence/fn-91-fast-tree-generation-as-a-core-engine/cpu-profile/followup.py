from pathlib import Path
import sys
root=Path(sys.argv[1]); core=root/'crates/telperion-core'; mode=sys.argv[2]
if mode=='prepare':
 p=core/'examples/cpu_profile.rs'; s=p.read_text(); start=s.index('        drop(stations);'); end=s.index('\n    }\n    Ok(())',start)
 s=s[:start]+'''        drop(stations); drop(compact);
        std::hint::black_box(report);
'''+s[end:]
 # Keep the same input/first+3warm and shared preparation; omit CPU-output verification already completed.
 p.write_text(s); sys.exit()
p=core/'src/cpu_diagnostic.rs'; s=p.read_text(); s+='''
thread_local! { static CALLS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }
pub fn sampled(key: u32) -> bool {
 CALLS.with(|n| n.set(n.get()+1));
 let mut h=key; h^=h>>16; h=h.wrapping_mul(0x7feb352d); h^=h>>15; h=h.wrapping_mul(0x846ca68b); h^=h>>16;
 h & 63 == 0
}
pub fn calls() -> u64 { CALLS.with(|n| n.replace(0)) }
'''; p.write_text(s)
p=core/'src/branching/local/planner.rs'; s=p.read_text()
s=s.replace('        let Axis {','        let sampled = crate::cpu_diagnostic::sampled(axis.key);\n        let profile_start = sampled.then(std::time::Instant::now);\n        let Axis {',1)
s=s.replace('        for k in 0..count {','        if let Some(t) = profile_start { crate::cpu_diagnostic::mark("planner.setup_sample",t); }\n        let trace_start = sampled.then(std::time::Instant::now);\n        for k in 0..count {',1)
s=s.replace('                let mut low = 0.0;','                let clip_start = sampled.then(std::time::Instant::now);\n                let mut low = 0.0;',1)
s=s.replace('                if low > 1e-9 {','                if let Some(t) = clip_start { crate::cpu_diagnostic::mark("planner.clip_sample",t); }\n                if low > 1e-9 {',1)
s=s.replace('        let mut actual = *along.last().unwrap();','        if let Some(t) = trace_start { crate::cpu_diagnostic::mark("planner.trace_sample",t); }\n        let tail_start = sampled.then(std::time::Instant::now);\n        let mut actual = *along.last().unwrap();',1)
s=s.replace('            return None;','            if let Some(t) = tail_start { crate::cpu_diagnostic::mark("planner.tail_sample",t); }\n            return None;',1)
s=s.replace('        Some(Rc::new(Run {','        let result = Some(Rc::new(Run {',1)
s=s.replace('        }))\n    }','        }));\n        if let Some(t) = tail_start { crate::cpu_diagnostic::mark("planner.tail_sample",t); }\n        result\n    }',1)
p.write_text(s)
p=core/'examples/cpu_profile.rs';s=p.read_text().replace('telperion_core::cpu_diagnostic::take();\n        let total','telperion_core::cpu_diagnostic::take(); telperion_core::cpu_diagnostic::calls();\n        let total').replace('let stages = telperion_core::cpu_diagnostic::take();','let mut stages = telperion_core::cpu_diagnostic::take();\n        stages.push(("planner.calls",telperion_core::cpu_diagnostic::calls() as f64));'); p.write_text(s)
