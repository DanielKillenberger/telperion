//! Device acquisition and the renderer's typed failures. Every condition that
//! can stop a frame before it is drawn is named here, never inferred by the caller.
use std::sync::{Arc, Mutex};

/// Why the renderer could not do what was asked. Each variant names the
/// condition and carries what a reader needs to act on it.
#[derive(Debug)]
pub enum RenderError {
    /// No WebGPU at all: no instance, or no adapter of any kind.
    WebGpuUnavailable(String),
    /// Only a software adapter was offered; drawing a tree on it is not honest.
    FallbackOnly { adapter: String },
    /// The device request was refused, naming the limit or feature that failed.
    DeviceRefused { requirement: String, detail: String },
    /// The device went away mid-render.
    DeviceLost { reason: String },
    /// A buffer this tree needs is larger than the device granted.
    Oversize {
        buffer: &'static str,
        bytes: u64,
        limit: u64,
    },
    /// The generator rejected the parameters.
    Generation(telperion_core::Error),
    /// The still could not be written where it was asked for.
    Output { path: String, message: String },
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WebGpuUnavailable(detail) => write!(f, "WebGPU is unavailable: {detail}"),
            Self::FallbackOnly { adapter } => write!(
                f,
                "no hardware GPU adapter; the only one offered was the software fallback \"{adapter}\""
            ),
            Self::DeviceRefused {
                requirement,
                detail,
            } => write!(f, "the GPU device was refused on {requirement}: {detail}"),
            Self::DeviceLost { reason } => write!(f, "the GPU device was lost: {reason}"),
            Self::Oversize {
                buffer,
                bytes,
                limit,
            } => write!(
                f,
                "the {buffer} buffer needs {bytes} bytes, above the device's limit of {limit}"
            ),
            Self::Generation(error) => write!(f, "the generator rejected the tree: {error}"),
            Self::Output { path, message } => write!(f, "cannot write {path}: {message}"),
        }
    }
}

impl std::error::Error for RenderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Generation(error) => Some(error),
            _ => None,
        }
    }
}

impl From<telperion_core::Error> for RenderError {
    fn from(error: telperion_core::Error) -> Self {
        Self::Generation(error)
    }
}

pub type Result<T> = std::result::Result<T, RenderError>;

/// A live GPU device with the adapter it came from. One per canvas, one per
/// headless render; it owns nothing about trees.
pub struct Gpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::AdapterInfo,
    /// Whether the adapter granted timestamp queries; the timing session reads it.
    pub timestamps: bool,
    lost: Arc<Mutex<Option<String>>>,
}

impl Gpu {
    /// Asks the platform for a hardware device, with the adapter's own maximum
    /// buffer size so a full-detail tree fits, and timestamps when offered.
    pub async fn request(compatible_surface: Option<&wgpu::Surface<'_>>) -> Result<Self> {
        let instance =
            wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface,
                force_fallback_adapter: false,
                ..Default::default()
            })
            .await
            .map_err(|error| RenderError::WebGpuUnavailable(error.to_string()))?;

        let info = adapter.get_info();
        if info.device_type == wgpu::DeviceType::Cpu {
            return Err(RenderError::FallbackOnly { adapter: info.name });
        }

        let available = adapter.limits();
        let limits = wgpu::Limits {
            max_buffer_size: available.max_buffer_size,
            ..wgpu::Limits::default()
        };
        // Ask before requesting, so a refusal names the limit rather than
        // arriving as a panic out of the validation layer.
        let mut short: Option<String> = None;
        limits.check_limits_with_fail_fn(&available, false, |name, wanted, allowed| {
            short.get_or_insert(format!("{name}: {wanted} wanted, {allowed} available"));
        });
        if let Some(detail) = short {
            return Err(RenderError::DeviceRefused {
                requirement: "limits".into(),
                detail,
            });
        }

        let timestamps = adapter.features().contains(wgpu::Features::TIMESTAMP_QUERY);
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("telperion"),
                required_features: if timestamps {
                    wgpu::Features::TIMESTAMP_QUERY
                } else {
                    wgpu::Features::empty()
                },
                required_limits: limits,
                ..Default::default()
            })
            .await
            .map_err(|error| RenderError::DeviceRefused {
                requirement: "the device request".into(),
                detail: error.to_string(),
            })?;

        let lost = Arc::new(Mutex::new(None));
        let sink = Arc::clone(&lost);
        device.set_device_lost_callback(move |reason, message| {
            if let Ok(mut slot) = sink.lock() {
                slot.get_or_insert(format!("{reason:?}: {message}"));
            }
        });

        Ok(Self {
            device,
            queue,
            adapter: info,
            timestamps,
            lost,
        })
    }

    /// The device-lost error if the device has gone away, checked after a poll.
    pub fn lost(&self) -> Option<RenderError> {
        let reason = self.lost.lock().ok()?.clone()?;
        Some(RenderError::DeviceLost { reason })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_condition_names_itself() {
        let messages = [
            RenderError::WebGpuUnavailable("no adapter".into()).to_string(),
            RenderError::FallbackOnly {
                adapter: "llvmpipe".into(),
            }
            .to_string(),
            RenderError::DeviceRefused {
                requirement: "limits".into(),
                detail: "max_buffer_size: 9 wanted, 4 available".into(),
            }
            .to_string(),
            RenderError::DeviceLost {
                reason: "Destroyed: dropped".into(),
            }
            .to_string(),
            RenderError::Oversize {
                buffer: "foliage instances",
                bytes: 507_000_000,
                limit: 268_435_456,
            }
            .to_string(),
            RenderError::Output {
                path: "/nope/x.png".into(),
                message: "permission denied".into(),
            }
            .to_string(),
        ];
        assert!(messages[0].contains("WebGPU is unavailable"));
        assert!(messages[1].contains("llvmpipe") && messages[1].contains("fallback"));
        assert!(messages[2].contains("max_buffer_size") && messages[2].contains("refused"));
        assert!(messages[3].contains("lost") && messages[3].contains("Destroyed"));
        assert!(
            messages[4].contains("foliage instances")
                && messages[4].contains("507000000")
                && messages[4].contains("268435456"),
            "an oversize buffer must name itself and both sizes: {}",
            messages[4]
        );
        assert!(messages[5].contains("/nope/x.png") && messages[5].contains("permission denied"));
    }

    #[test]
    fn a_generator_rejection_keeps_its_own_words() {
        let error = RenderError::from(telperion_core::Error::InvalidInput("surface parameters"));
        assert!(error.to_string().contains("surface parameters"));
        assert!(std::error::Error::source(&error).is_some());
    }
}
