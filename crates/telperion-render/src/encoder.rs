//! A command encoder that knows which buffers wait on it. A buffer its
//! owner lets go while encoders are open is destroyed once each of them
//! is submitted or dropped, never by another submission on the device
//! (fn-212 task 11, Astra's finding 1: a frame submitted on the shared GPU
//! destroyed a generation encoder's configuration before that encoder
//! was submitted).
use std::sync::Arc;

use crate::buffer::Ledger;

/// Holds an encoder open in its ledger until it is dropped.
pub(crate) struct Open {
    id: u64,
    ledger: Arc<Ledger>,
}

impl Drop for Open {
    fn drop(&mut self) {
        self.ledger.close(self.id);
    }
}

/// An open encoder; `Gpu::encoder` makes one.
pub struct Encoder {
    inner: wgpu::CommandEncoder,
    open: Open,
}

/// An encoder's commands, held open until they are submitted.
pub struct Recorded {
    commands: wgpu::CommandBuffer,
    _open: Open,
}

impl Encoder {
    pub(crate) fn new(inner: wgpu::CommandEncoder, ledger: &Arc<Ledger>) -> Self {
        let open = Open {
            id: ledger.open(),
            ledger: Arc::clone(ledger),
        };
        Self { inner, open }
    }

    pub fn finish(self) -> Recorded {
        Recorded {
            commands: self.inner.finish(),
            _open: self.open,
        }
    }
}

impl Recorded {
    /// The commands to submit, and what closes the encoder after.
    pub(crate) fn split(self) -> (wgpu::CommandBuffer, Open) {
        (self.commands, self._open)
    }
}

impl std::ops::Deref for Encoder {
    type Target = wgpu::CommandEncoder;
    fn deref(&self) -> &wgpu::CommandEncoder {
        &self.inner
    }
}

impl std::ops::DerefMut for Encoder {
    fn deref_mut(&mut self) -> &mut wgpu::CommandEncoder {
        &mut self.inner
    }
}
