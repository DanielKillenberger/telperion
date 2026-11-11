//! The GPU programs a pass dispatches (fn-243). Natively a compute
//! pipeline is made where it is asked for. A browser compiles it in the
//! background instead: Chrome makes a synchronous pipeline on its GPU
//! process's main thread, where its watchdog ended the process 19 s into
//! the first cold build (fn-243 R1). There every program is asked of the
//! browser at once, through the forked wgpu's
//! `create_compute_pipeline_async` (docs/wgpu-fork.md), and `ready`
//! settles them all before anything dispatches one.
use crate::Gpu;
#[cfg(any(test, target_arch = "wasm32"))]
use crate::{RenderError, Result};
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
        Self::asked(gpu, desc)
    }

    fn asked(gpu: &Gpu, desc: &wgpu::ComputePipelineDescriptor<'_>) -> Self {
        // Natively a refusal is caught by scopes of its own, not by wgpu's
        // uncaptured-error handler, which panics; the frame that
        // dispatches the refused pipeline fails by its own scopes.
        #[cfg(not(target_arch = "wasm32"))]
        {
            let validation = gpu.device.push_error_scope(wgpu::ErrorFilter::Validation);
            let memory = gpu.device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
            let made = gpu.device.create_compute_pipeline(desc);
            drop(pollster::block_on(memory.pop()));
            drop(pollster::block_on(validation.pop()));
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
                    let pipeline = asked.await.map_err(rejection)?;
                    let _ = slot.set(pipeline);
                    Ok(())
                }),
            ));
            Self { made }
        }
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

/// A refused compile as `ProgramRefused` states it: the browser's reason
/// and its message, `validation: …` or `internal: …`.
#[cfg(any(test, target_arch = "wasm32"))]
fn rejection(error: wgpu::Error) -> String {
    match error {
        wgpu::Error::Validation { description, .. } => format!("validation: {description}"),
        wgpu::Error::Internal { description, .. } => format!("internal: {description}"),
        wgpu::Error::OutOfMemory { .. } => error.to_string(),
    }
}

/// Every program the browser was asked for so far, settled: each compile
/// awaited, all of them already running side by side.
#[cfg(target_arch = "wasm32")]
pub(crate) async fn ready(gpu: &Gpu) -> Result<()> {
    let pending = std::mem::take(&mut *gpu.programs.borrow_mut());
    settle(pending, || gpu.lost()).await
}

/// What a preparation reports when more than one thing went wrong: a lost
/// device first, then the program the browser refused, by name, then
/// whatever the error scopes caught (a refused program's shader also
/// raises a scoped validation error, which names nothing).
#[cfg(any(test, target_arch = "wasm32"))]
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
mod tests;
