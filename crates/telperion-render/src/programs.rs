//! The GPU programs a pass dispatches (fn-243). Natively a compute
//! pipeline is made where it is asked for. A browser compiles it in the
//! background instead: Chrome makes a synchronous pipeline on its GPU
//! process's main thread, where its watchdog ended the process 19 s into
//! the first cold build (fn-243 R1). There every program is asked of the
//! browser at once, through the patched wgpu's
//! `create_compute_pipeline_async` (vendor/README.md), and `ready`
//! settles them all before anything dispatches one.
use crate::{Gpu, RenderError, Result};
#[cfg(any(test, target_arch = "wasm32"))]
use std::future::Future;

/// One compute pipeline. Natively it is made at once; in a browser it is
/// filled when `ready` settles it.
pub(crate) struct Program {
    #[cfg(not(target_arch = "wasm32"))]
    made: wgpu::ComputePipeline,
    #[cfg(target_arch = "wasm32")]
    made: std::rc::Rc<std::cell::OnceCell<wgpu::ComputePipeline>>,
}

impl Program {
    /// The pipeline `desc` states, asked of the device now.
    pub fn make(gpu: &Gpu, desc: &wgpu::ComputePipelineDescriptor<'_>) -> Self {
        #[cfg(test)]
        MADE.with(|made| made.set(made.get() + 1));
        Self::asked(gpu, desc)
    }

    /// The pipelines `descs` state, asked of the device at once: natively
    /// each made on its own thread, as a browser compiles them side by
    /// side, so a module's cold compile is its slowest entry's, not their
    /// sum (fn-256, DESIGN.md 9.3). Each comes with what asking for it took
    /// natively; a browser compiles in the background, and has no clock
    /// `std::time` reads, so there asking takes nothing measured.
    pub fn make_many(
        gpu: &Gpu,
        descs: &[wgpu::ComputePipelineDescriptor<'_>],
    ) -> Vec<(Self, std::time::Duration)> {
        #[cfg(test)]
        MADE.with(|made| made.set(made.get() + descs.len()));
        #[cfg(not(target_arch = "wasm32"))]
        return std::thread::scope(|s| {
            let timed = |d: &wgpu::ComputePipelineDescriptor<'_>| {
                let started = std::time::Instant::now();
                let made = Self::asked(gpu, d);
                (made, started.elapsed())
            };
            let asked: Vec<_> = descs.iter().map(|d| s.spawn(move || timed(d))).collect();
            asked
                .into_iter()
                .map(|h| h.join().expect("a pipeline made"))
                .collect()
        });
        #[cfg(target_arch = "wasm32")]
        descs
            .iter()
            .map(|d| (Self::asked(gpu, d), std::time::Duration::ZERO))
            .collect()
    }

    fn asked(gpu: &Gpu, desc: &wgpu::ComputePipelineDescriptor<'_>) -> Self {
        // Natively a pipeline may be made on any thread (`make_many`), and
        // error scopes are a thread's own, so each is made under scopes of
        // its own and a refusal kept by name for `ready`.
        #[cfg(not(target_arch = "wasm32"))]
        {
            let validation = gpu.device.push_error_scope(wgpu::ErrorFilter::Validation);
            let memory = gpu.device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
            let made = gpu.device.create_compute_pipeline(desc);
            let memory = pollster::block_on(memory.pop());
            let caught = memory.or(pollster::block_on(validation.pop()));
            if let Some(error) = caught {
                let name = desc.label.unwrap_or("unnamed").to_string();
                let mut refused = gpu.refused.lock().expect("the refusals");
                refused.push((name, error.to_string()));
            }
            Self { made }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let made = std::rc::Rc::<std::cell::OnceCell<_>>::default();
            let slot = made.clone();
            let name = desc.label.unwrap_or("unnamed").to_string();
            let asked = gpu.device.create_compute_pipeline_async(desc);
            gpu.programs.borrow_mut().push((
                name,
                Box::pin(async move {
                    let pipeline = asked
                        .ok_or_else(|| "the device is not the browser's WebGPU".to_string())?
                        .await?;
                    let _ = slot.set(pipeline);
                    Ok(())
                }),
            ));
            Self { made }
        }
    }

    /// The pipeline. In a browser, only after `ready` settled it: every
    /// generation pass's owner awaits `ready` before it dispatches.
    pub fn get(&self) -> &wgpu::ComputePipeline {
        self.compiled()
            .expect("a GPU program dispatched before `programs::ready` settled it")
    }

    /// The pipeline, where it is compiled: always natively, in a browser
    /// once `ready` settled it.
    pub fn compiled(&self) -> Option<&wgpu::ComputePipeline> {
        #[cfg(not(target_arch = "wasm32"))]
        return Some(&self.made);
        #[cfg(target_arch = "wasm32")]
        self.made.get()
    }
}

/// The programs the browser is compiling, by name, each resolving once
/// its pipeline is filled in.
#[cfg(target_arch = "wasm32")]
pub(crate) type Pending = std::cell::RefCell<
    Vec<(
        String,
        std::pin::Pin<Box<dyn Future<Output = std::result::Result<(), String>>>>,
    )>,
>;

/// Every program asked of `gpu` so far, settled: natively there is nothing
/// to wait on; in a browser each compile is awaited, all of them already
/// running side by side.
pub(crate) async fn ready(gpu: &Gpu) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        let pending = std::mem::take(&mut *gpu.programs.borrow_mut());
        settle(pending, || gpu.lost()).await?;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let refused = std::mem::take(&mut *gpu.refused.lock().expect("the refusals"));
        if let Some(lost) = gpu.lost() {
            return Err(lost);
        }
        if let Some((program, reason)) = refused.into_iter().next() {
            return Err(RenderError::ProgramRefused { program, reason });
        }
    }
    Ok(())
}

/// What a preparation reports when more than one thing went wrong: a lost
/// device first, then the program the browser refused, by name, then
/// whatever the error scopes caught (a refused program's shader also
/// raises a scoped validation error, which names nothing).
pub(crate) fn first_cause(
    lost: Option<RenderError>,
    ready: Result<()>,
    scoped: Result<()>,
) -> Result<()> {
    match lost {
        Some(lost) => Err(lost),
        None => ready.and(scoped),
    }
}

/// Each compile awaited in turn. A device lost meanwhile is the error
/// reported, by name, whatever the compile said; otherwise the first
/// program the browser refused is, and nothing after it is awaited.
#[cfg(any(test, target_arch = "wasm32"))]
async fn settle<F>(pending: Vec<(String, F)>, lost: impl Fn() -> Option<RenderError>) -> Result<()>
where
    F: Future<Output = std::result::Result<(), String>>,
{
    for (program, compile) in pending {
        let compiled = compile.await;
        if let Some(lost) = lost() {
            return Err(lost);
        }
        compiled.map_err(|reason| RenderError::ProgramRefused { program, reason })?;
    }
    Ok(())
}

#[cfg(test)]
thread_local! {
    /// Programs this thread asked of a device, for the test that each is
    /// made once.
    static MADE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// How many programs this thread has asked of a device.
#[cfg(test)]
pub(crate) fn made() -> usize {
    MADE.with(|made| made.get())
}

#[cfg(test)]
mod tests;
