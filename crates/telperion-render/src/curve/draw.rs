//! Drawing what the curve's passes wrote (fn-208): the wood's depth first,
//! then the bark over the nearest surface alone, tubes and ribbons through
//! the same pipelines, and both into the sun's map.
use super::{CurveGpu, VERTEX_FLOATS};
use crate::pass::{lit_pipeline, prepass_pipeline, Depth, Stage, Surface};
use crate::{device::Gpu, FrameStats};

const LAYOUT: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
    0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Float32
];

/// One lit shader's depth and shading pipelines.
struct Lit {
    depth: wgpu::RenderPipeline,
    shaded: wgpu::RenderPipeline,
}

/// The pipelines the wood is drawn through.
pub(crate) struct CurveDraw {
    plain: Lit,
    smooth: Lit,
    shadow: wgpu::RenderPipeline,
    empty: wgpu::BindGroup,
}

fn layout(attributes: &'static [wgpu::VertexAttribute]) -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: VERTEX_FLOATS * 4,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes,
    }
}

impl CurveDraw {
    pub(crate) fn new(
        gpu: &Gpu,
        scene: &wgpu::BindGroupLayout,
        shadow: &crate::shadow::Shadow,
        surface: Surface,
        modules: [&wgpu::ShaderModule; 2],
    ) -> Self {
        let empty_layout = gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("wood: no group 1"),
                entries: &[],
            });
        let groups = [Some(scene), Some(&empty_layout), Some(shadow.layout())];
        let buffers = [Some(layout(&LAYOUT))];
        let lit = |module: &wgpu::ShaderModule| {
            let stage = |fragment| Stage {
                vertex: "vertex",
                fragment,
            };
            Lit {
                depth: prepass_pipeline(
                    gpu,
                    &groups,
                    module,
                    surface,
                    &buffers,
                    stage("depth_only"),
                    "wood depth",
                ),
                shaded: lit_pipeline(
                    gpu,
                    &groups,
                    module,
                    surface,
                    &buffers,
                    Depth::Prepassed,
                    stage("fragment"),
                    "wood",
                ),
            }
        };
        Self {
            plain: lit(modules[0]),
            smooth: lit(modules[1]),
            shadow: crate::depth_pipeline(
                gpu,
                &[Some(shadow.light_layout())],
                shadow.module(),
                "wood",
                &[Some(layout(&LAYOUT[..1]))],
                "wood shadow",
            ),
            empty: gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("wood: no group 1"),
                layout: &empty_layout,
                entries: &[],
            }),
        }
    }

    /// The camera's wood: depth first, then shading over the nearest surface.
    pub(crate) fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        curve: &CurveGpu,
        smooth: bool,
    ) -> FrameStats {
        let Some(t) = &curve.camera else {
            return FrameStats::default();
        };
        let lit = if smooth { &self.smooth } else { &self.plain };
        pass.set_bind_group(1, &self.empty, &[]);
        pass.set_vertex_buffer(0, t.vertices.slice(..));
        pass.set_index_buffer(t.indices.slice(..), wgpu::IndexFormat::Uint32);
        for pipeline in [&lit.depth, &lit.shaded] {
            pass.set_pipeline(pipeline);
            pass.draw_indexed_indirect(&t.args, 0);
            pass.draw_indexed_indirect(&t.args, 20);
        }
        // The triangles are the device's count, which a frame does not wait
        // for: a still adds them from the readback (`headless::render`).
        FrameStats {
            draw_calls: 4,
            triangles: 0,
            instances: 0,
        }
    }

    /// The tubes and ribbons into the sun's map.
    pub(crate) fn draw_shadow(&self, pass: &mut wgpu::RenderPass<'_>, curve: &CurveGpu) {
        let Some(t) = &curve.sun else { return };
        pass.set_pipeline(&self.shadow);
        pass.set_vertex_buffer(0, t.vertices.slice(..));
        pass.set_index_buffer(t.indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed_indirect(&t.args, 0);
        pass.draw_indexed_indirect(&t.args, 20);
    }
}
