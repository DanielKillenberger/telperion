//! Drawing what the curve's passes wrote (fn-208): the tubes' depth, then
//! the ribbons' depth over the samples they cover, then each shaded over the
//! nearest surface alone, and the tubes into the sun's map.
use super::{CurveGpu, Target, VERTEX_FLOATS};
use crate::pass::{lit_pipeline, prepass_pipeline, Depth, Stage, Surface};
use crate::{device::Gpu, FrameStats};

const LAYOUT: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
    0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Float32, 4 => Float32
];

/// One lit shader's four pipelines.
struct Lit {
    tube_depth: wgpu::RenderPipeline,
    ribbon_depth: wgpu::RenderPipeline,
    tube: wgpu::RenderPipeline,
    ribbon: wgpu::RenderPipeline,
}

/// The pipelines the curve's wood is drawn through.
pub(crate) struct CurveDraw {
    plain: Lit,
    smooth: Lit,
    shadow: wgpu::RenderPipeline,
    empty: wgpu::BindGroup,
}

fn layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: VERTEX_FLOATS * 4,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &LAYOUT,
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
                label: Some("curve wood: no group 1"),
                entries: &[],
            });
        let groups = [Some(scene), Some(&empty_layout), Some(shadow.layout())];
        let buffers = [Some(layout())];
        let lit = |module: &wgpu::ShaderModule| {
            let stage = |fragment, coverage| Stage {
                vertex: "curve_vertex",
                fragment,
                coverage,
            };
            let depth = |fragment, coverage, label| {
                prepass_pipeline(
                    gpu,
                    &groups,
                    module,
                    surface,
                    &buffers,
                    stage(fragment, coverage),
                    label,
                )
            };
            let shaded = |fragment, coverage, label| {
                let s = stage(fragment, coverage);
                lit_pipeline(
                    gpu,
                    &groups,
                    module,
                    surface,
                    &buffers,
                    Depth::Prepassed,
                    s,
                    label,
                )
            };
            Lit {
                tube_depth: depth("depth_only", false, "curve tube depth"),
                ribbon_depth: depth("ribbon_depth", true, "curve ribbon depth"),
                tube: shaded("fragment", false, "curve tube"),
                ribbon: shaded("ribbon_fragment", true, "curve ribbon"),
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
                &[Some(wgpu::VertexBufferLayout {
                    array_stride: VERTEX_FLOATS * 4,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &LAYOUT[..1],
                })],
                "curve wood shadow",
            ),
            empty: gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("curve wood: no group 1"),
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
        let lit = if smooth { &self.smooth } else { &self.plain };
        let t: &Target = &curve.camera;
        pass.set_bind_group(1, &self.empty, &[]);
        pass.set_vertex_buffer(0, t.vertices.slice(..));
        pass.set_index_buffer(t.indices.slice(..), wgpu::IndexFormat::Uint32);
        for (pipeline, draw) in [
            (&lit.tube_depth, 0),
            (&lit.ribbon_depth, 1),
            (&lit.tube, 0),
            (&lit.ribbon, 1),
        ] {
            pass.set_pipeline(pipeline);
            pass.draw_indexed_indirect(&t.args, draw * 20);
        }
        FrameStats {
            draw_calls: 4,
            triangles: 0,
            instances: 0,
        }
    }

    /// The tubes and ribbons into the sun's map.
    pub(crate) fn draw_shadow(&self, pass: &mut wgpu::RenderPass<'_>, curve: &CurveGpu) {
        let t = &curve.sun;
        pass.set_pipeline(&self.shadow);
        pass.set_vertex_buffer(0, t.vertices.slice(..));
        pass.set_index_buffer(t.indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed_indirect(&t.args, 0);
        pass.draw_indexed_indirect(&t.args, 20);
    }
}
