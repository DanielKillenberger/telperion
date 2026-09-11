//! The native offscreen target: one frame into a texture, read back with the
//! row padding the copy demands, out as a PNG. No window, no surface.
use std::path::Path;

use crate::{
    device::{RenderError, Result},
    pass::{attachment, Target},
    scene::DEPTH_FORMAT,
    Camera, FrameStats, Renderer,
};

/// The colour format the still is rendered and read back in. Reading an sRGB
/// target back gives exactly the bytes a PNG wants.
pub const STILL_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
const CHANNELS: usize = 4;

/// One rendered frame on the host, tightly packed, top row first.
#[derive(Debug)]
pub struct Still {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub stats: FrameStats,
}

impl Still {
    /// Whether anything at all was drawn over the cleared background. A still
    /// that is entirely one colour is a failed render, not a picture.
    pub fn has_subject(&self) -> bool {
        let (pixels, _) = self.rgba.as_chunks::<CHANNELS>();
        let Some(background) = pixels.first() else {
            return false;
        };
        pixels.iter().any(|pixel| pixel != background)
    }
}

/// An offscreen frame of this size: the colour and depth the passes write at
/// the renderer's own sample count, and, where that is more than one sample,
/// the single-sample picture they resolve into. The timing session draws into
/// the same textures the still is taken from, so what is measured is the frame
/// that was judged.
pub struct Frame {
    /// The texture a reader copies out of: the resolve where the frame is
    /// multisampled, and the colour attachment itself where it is not.
    picture: wgpu::Texture,
    colour: wgpu::TextureView,
    depth: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
}

impl Frame {
    /// The textures for one frame of this renderer, at this size.
    pub fn new(renderer: &Renderer, label: &str, size: (u32, u32)) -> Self {
        let (gpu, samples) = (renderer.gpu(), renderer.samples());
        let format = renderer.colour_format();
        let colour = attachment(gpu, label, format, size, samples);
        let depth = attachment(gpu, &format!("{label} depth"), DEPTH_FORMAT, size, samples);
        let resolved =
            (samples > 1).then(|| attachment(gpu, &format!("{label} resolve"), format, size, 1));
        let view = |texture: &wgpu::Texture| texture.create_view(&Default::default());
        Self {
            colour: view(&colour),
            depth: view(&depth),
            resolve: resolved.as_ref().map(view),
            picture: resolved.unwrap_or(colour),
        }
    }

    /// Where a frame is drawn, as the renderer takes it.
    pub fn target(&self) -> Target<'_> {
        Target {
            colour: &self.colour,
            depth: &self.depth,
            resolve: self.resolve.as_ref(),
        }
    }
}

/// Renders one frame at this size and brings it back to the host.
pub fn render(renderer: &mut Renderer, camera: &Camera, width: u32, height: u32) -> Result<Still> {
    let frame = Frame::new(renderer, "still", (width, height));
    let stats = renderer.draw(camera, (width, height), frame.target());

    // Rows land in the readback buffer padded to the copy alignment; the
    // padding is stripped on the way into the still.
    let tight = width * CHANNELS as u32;
    let padded =
        tight.div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT) * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let gpu = renderer.gpu();
    let readback = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("still readback"),
        size: u64::from(padded) * u64::from(height),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("readback"),
        });
    encoder.copy_texture_to_buffer(
        frame.picture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    gpu.queue.submit([encoder.finish()]);

    let (sender, receiver) = std::sync::mpsc::channel();
    readback
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .map_err(|error| {
            gpu.lost().unwrap_or(RenderError::DeviceLost {
                reason: error.to_string(),
            })
        })?;
    if let Some(lost) = gpu.lost() {
        return Err(lost);
    }
    receiver
        .recv()
        .map_err(|_| RenderError::DeviceLost {
            reason: "the readback never completed".into(),
        })?
        .map_err(|error| RenderError::DeviceLost {
            reason: error.to_string(),
        })?;

    let view = readback
        .slice(..)
        .get_mapped_range()
        .map_err(|error| RenderError::DeviceLost {
            reason: error.to_string(),
        })?;
    let mut rgba = Vec::with_capacity((tight * height) as usize);
    for row in 0..height as usize {
        let start = row * padded as usize;
        rgba.extend_from_slice(&view[start..start + tight as usize]);
    }
    drop(view);
    readback.unmap();

    Ok(Still {
        width,
        height,
        rgba,
        stats,
    })
}

/// Writes the still where it was asked for, naming the path when it cannot.
pub fn write_png(path: &Path, still: &Still) -> Result<()> {
    let fail = |error: std::io::Error| RenderError::Output {
        path: path.display().to_string(),
        message: error.to_string(),
    };
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(fail)?;
    }
    let file = std::fs::File::create(path).map_err(fail)?;
    let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), still.width, still.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .map_err(|error| RenderError::Output {
            path: path.display().to_string(),
            message: error.to_string(),
        })?;
    writer
        .write_image_data(&still.rgba)
        .map_err(|error| RenderError::Output {
            path: path.display().to_string(),
            message: error.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn still(rgba: Vec<u8>) -> Still {
        Still {
            width: 2,
            height: 1,
            rgba,
            stats: FrameStats::default(),
        }
    }

    #[test]
    fn a_still_of_one_colour_has_no_subject() {
        assert!(!still(vec![9, 8, 7, 255, 9, 8, 7, 255]).has_subject());
        assert!(still(vec![9, 8, 7, 255, 1, 2, 3, 255]).has_subject());
        assert!(!still(Vec::new()).has_subject());
    }

    #[test]
    fn an_unwritable_path_names_itself() {
        let error = write_png(Path::new("/proc/1/telperion.png"), &still(vec![0; 8]))
            .expect_err("a write into /proc should not have succeeded");
        assert!(error.to_string().contains("telperion.png"), "{error}");
    }
}
