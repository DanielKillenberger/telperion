//! What a driver compiles for one entry point, measured on the CPU: the
//! naga expressions of the entry and of every function it calls, each
//! callee counted again at every call site, since the driver inlines it
//! there (fn-235 REPRO.md: `grow`'s 528 inlined copies of `mul32`; fn-256
//! DESIGN.md section 9: the units entry's 43.7 s was the volume inlined
//! into one body). A load-free stand-in for the cold compile the release
//! check times (fn-237 R9).
//!
//! It tracks the cost within the shipped shaders (Spearman 0.92 over 196
//! entries, against 0.60 for the module's bytes), not across every shape:
//! fn-256's old units entry took 39.8 s at 95,761 expressions and its old
//! layout frame 2.4 s at 117,267. So it is a ceiling on growth, and the
//! release check's timing stays the authority (RELEASE-CHECKS.md).
use naga::{Block, Function, Handle, Module, Statement};
use std::collections::HashMap;

/// One entry point's inlined size: its expressions and its call sites,
/// each callee's counted once a site.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Inlined {
    pub expressions: usize,
    pub calls: usize,
}

impl std::ops::Add for Inlined {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self {
            expressions: self.expressions + o.expressions,
            calls: self.calls + o.calls,
        }
    }
}

/// Every entry point of `source` by name, with its inlined size.
pub(crate) fn entries(source: &str) -> Vec<(String, Inlined)> {
    let module = naga::front::wgsl::parse_str(source).expect("a module naga parses");
    let mut sizes = HashMap::new();
    (module.entry_points.iter())
        .map(|e| (e.name.clone(), of(&module, &e.function, &mut sizes)))
        .collect()
}

fn of(module: &Module, f: &Function, sizes: &mut HashMap<Handle<Function>, Inlined>) -> Inlined {
    let own = Inlined {
        expressions: f.expressions.len(),
        calls: 0,
    };
    own + block(module, &f.body, sizes)
}

fn callee(
    module: &Module,
    h: Handle<Function>,
    sizes: &mut HashMap<Handle<Function>, Inlined>,
) -> Inlined {
    if let Some(&s) = sizes.get(&h) {
        return s;
    }
    let s = of(module, &module.functions[h], sizes);
    sizes.insert(h, s);
    s
}

fn block(module: &Module, b: &Block, sizes: &mut HashMap<Handle<Function>, Inlined>) -> Inlined {
    let mut sum = Inlined::default();
    for s in b.iter() {
        sum = sum
            + match s {
                Statement::Call { function, .. } => {
                    Inlined {
                        expressions: 0,
                        calls: 1,
                    } + callee(module, *function, sizes)
                }
                Statement::Block(b) => block(module, b, sizes),
                Statement::If { accept, reject, .. } => {
                    block(module, accept, sizes) + block(module, reject, sizes)
                }
                Statement::Switch { cases, .. } => (cases.iter())
                    .fold(Inlined::default(), |a, c| a + block(module, &c.body, sizes)),
                Statement::Loop {
                    body, continuing, ..
                } => block(module, body, sizes) + block(module, continuing, sizes),
                _ => Inlined::default(),
            };
    }
    sum
}

/// The most expressions one entry may inline. Measured 2026-10-14 on the
/// RTX 3080 (driver 610.57.04, its disk cache off, each entry made alone
/// by `every_entry_made_cold_alone`): every shipped entry inlines at most
/// 43,525 and is made in under 0.8 s; fn-256's old entries, at 95,761 to
/// 117,267, took 1.3 to 39.8 s, and nothing between was measured. The
/// ceiling sits 15 % over the largest shipped entry; an entry past it is
/// timed by the release check before the ceiling moves.
pub(crate) const CEILING: usize = 50_000;

/// Each entry of `source` that inlines past `CEILING`, one line each.
pub(crate) fn over_ceiling(name: &str, source: &str) -> Vec<String> {
    (entries(source).into_iter())
        .filter(|(_, s)| s.expressions > CEILING)
        .map(|(e, s)| {
            format!(
                "{name} {e}: {} expressions inlined, the ceiling is {CEILING}",
                s.expressions
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{entries, over_ceiling, CEILING};

    /// fn-237 R9: no shipped entry inlines past the ceiling, and an entry
    /// that does is named.
    #[test]
    fn every_entry_inlines_under_the_ceiling() {
        let over: Vec<String> = (crate::shader_tests::shipped().iter())
            .flat_map(|(name, source)| over_ceiling(name, source))
            .collect();
        assert!(over.is_empty(), "{}", over.join("\n"));
        let module = |calls: usize| {
            let body = "a = g(a);".repeat(calls);
            format!(
                "fn g(x: f32) -> f32 {{ return x * x + 1.0; }}
                @compute @workgroup_size(1) fn main() {{ var a = 1.0; {body} }}"
            )
        };
        let per = entries(&module(2))[0].1.expressions - entries(&module(1))[0].1.expressions;
        let past = CEILING / per + 1;
        assert_eq!(over_ceiling("past it", &module(past)).len(), 1);
        assert!(over_ceiling("under it", &module(past / 2)).is_empty());
    }

    /// Every shipped compute entry made alone, one after another, each
    /// timed beside its inlined size: what the guard's ceiling is read
    /// from. Run with the driver's disk cache off and nothing else on the
    /// machine: `__GL_SHADER_DISK_CACHE=0 cargo test --profile ci -p
    /// telperion-render every_entry_made_cold_alone -- --ignored
    /// --nocapture`.
    #[test]
    #[ignore = "a measurement"]
    fn every_entry_made_cold_alone() {
        let _held = crate::whole_tree::whole_tree();
        let Ok(gpu) = pollster::block_on(crate::Gpu::request(None)) else {
            return println!("skipped: no device");
        };
        for (name, source) in crate::shader_tests::shipped() {
            let module = naga::front::wgsl::parse_str(&source).expect("parsed");
            let compute: Vec<String> = (module.entry_points.iter())
                .filter(|e| e.stage == naga::ShaderStage::Compute)
                .map(|e| e.name.clone())
                .collect();
            let sizes = entries(&source);
            let started = std::time::Instant::now();
            let shader = gpu
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(source.as_str().into()),
                });
            gpu.device
                .poll(wgpu::PollType::wait_indefinitely())
                .expect("made");
            let ms = started.elapsed().as_secs_f64() * 1e3;
            println!("MODULE\t{name}\t{ms:.0}\t{}", source.len());
            for entry in compute {
                let size = sizes.iter().find(|s| s.0 == entry).expect("sized").1;
                let started = std::time::Instant::now();
                let scope = gpu.device.push_error_scope(wgpu::ErrorFilter::Validation);
                let made = gpu
                    .device
                    .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                        label: None,
                        layout: None,
                        module: &shader,
                        entry_point: Some(&entry),
                        compilation_options: Default::default(),
                        cache: None,
                    });
                let refused = pollster::block_on(scope.pop());
                gpu.device
                    .poll(wgpu::PollType::wait_indefinitely())
                    .expect("made");
                let ms = started.elapsed().as_secs_f64() * 1e3;
                let state = refused.map_or("made".to_string(), |e| format!("refused: {e}"));
                println!(
                    "COLD\t{name}\t{entry}\t{ms:.0}\t{}\t{}\t{state}",
                    size.expressions, size.calls
                );
                drop(made);
            }
        }
    }

    /// A callee is counted at each of its call sites, and through a
    /// callee's own calls.
    #[test]
    fn a_callee_counts_once_a_call_site() {
        let one = "fn g(x: f32) -> f32 { return x * x + 1.0; }
            @compute @workgroup_size(1) fn main() { var a = g(1.0); }";
        let two = "fn g(x: f32) -> f32 { return x * x + 1.0; }
            @compute @workgroup_size(1) fn main() { var a = g(1.0); a = g(a); }";
        let deep = "fn g(x: f32) -> f32 { return x * x + 1.0; }
            fn h(x: f32) -> f32 { return g(x) + g(x); }
            @compute @workgroup_size(1) fn main() { var a = h(1.0); }";
        let [one, two, deep] = [one, two, deep].map(|s| entries(s)[0].1);
        let g = two.expressions - one.expressions - 1;
        assert!(g >= 4, "g's expressions: {g}");
        assert_eq!((one.calls, two.calls, deep.calls), (1, 2, 3));
        assert!(deep.expressions > 2 * g, "{deep:?}");
    }
}
