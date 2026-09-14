use telperion_core::{branching::{self, Specimen}, math::Vec3, presets::Preset, surface};
use telperion_render::Camera;
fn main() {
 for preset in [Preset::OregonWhiteOak, Preset::NorwaySpruce] {
  let mut f=preset.parameters(); f.skeleton.seed=7;
  let height=if preset==Preset::OregonWhiteOak {2.0} else {0.9};
  let camera=Camera {position:Vec3::new(1.7307636095778745,height+0.2967023330704928,-1.7307636095778745),target:Vec3::new(-0.24,height,-0.24),field_of_view:20.0,near:0.01,far:1000.0};
  let s=Specimen::build(&f).unwrap();
  let old=branching::generate(&f.skeleton,f.radii).unwrap();
  for (kind,tree) in [("growth",s.tree()),("envelope",&old.tree)] {
   let wood=surface::build(tree,f.skeleton.envelope.height,&f.surface).unwrap();
   println!("{preset:?} {kind} mask={} root_radius={}",edge_mask(&wood,&camera).len(),tree.nodes[0].radius);
  }
 }
}
fn edge_mask(wood: &telperion_core::surface::SurfaceMesh, camera: &Camera) -> Vec<(usize, usize)> {
    let matrix = camera.view_projection(1.6);
    let vector = |values: &[f32], i: usize| {
        Vec3::new(
            f64::from(values[i * 3]),
            f64::from(values[i * 3 + 1]),
            f64::from(values[i * 3 + 2]),
        )
    };
    let mut depth = vec![f64::INFINITY; 800 * 500];
    let mut incidence = vec![1.0; 800 * 500];
    let run = &wood.run_table[0];
    for (triangle_index, triangle) in wood.indices.as_chunks::<3>().0.iter().enumerate() {
        let positions = triangle
            .iter()
            .map(|&i| vector(&wood.positions, i as usize))
            .collect::<Vec<_>>();
        let trunk = (run.first_index..run.first_index + run.index_count)
            .contains(&(triangle_index as u32 * 3))
            && positions.iter().all(|p| p.y >= 0.5 && p.y <= 3.5);
        let projected = positions
            .iter()
            .map(|p| {
                let v = [p.x, p.y, p.z, 1.0];
                let clip: [f64; 4] = std::array::from_fn(|r| {
                    (0..4).map(|c| f64::from(matrix[c * 4 + r]) * v[c]).sum()
                });
                [
                    (clip[0] / clip[3] + 1.0) * 400.0,
                    (1.0 - clip[1] / clip[3]) * 250.0,
                    clip[3],
                ]
            })
            .collect::<Vec<_>>();
        if projected.iter().any(|p| p[2] <= camera.near) {
            continue;
        }
        let min = |axis: usize, bound: f64| {
            projected
                .iter()
                .map(|p| p[axis])
                .fold(f64::INFINITY, f64::min)
                .floor()
                .clamp(0.0, bound) as usize
        };
        let max = |axis: usize, bound: f64| {
            projected
                .iter()
                .map(|p| p[axis])
                .fold(f64::NEG_INFINITY, f64::max)
                .ceil()
                .clamp(0.0, bound) as usize
        };
        let [a, b, c] = [projected[0], projected[1], projected[2]];
        let det = (b[1] - c[1]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[1] - c[1]);
        if det.abs() < 1e-8 {
            continue;
        }
        for y in min(1, 500.0)..max(1, 500.0) {
            for x in min(0, 800.0)..max(0, 800.0) {
                let px = x as f64 + 0.5;
                let py = y as f64 + 0.5;
                let wa = ((b[1] - c[1]) * (px - c[0]) + (c[0] - b[0]) * (py - c[1])) / det;
                let wb = ((c[1] - a[1]) * (px - c[0]) + (a[0] - c[0]) * (py - c[1])) / det;
                let wc = 1.0 - wa - wb;
                if wa.min(wb).min(wc) < 0.0 {
                    continue;
                }
                let w = [wa / a[2], wb / b[2], wc / c[2]];
                let z = 1.0 / w.iter().sum::<f64>();
                let index = y * 800 + x;
                if z >= depth[index] {
                    continue;
                }
                depth[index] = z;
                let p = (positions[0] * w[0] + positions[1] * w[1] + positions[2] * w[2]) * z;
                let n = (vector(&wood.normals, triangle[0] as usize) * w[0]
                    + vector(&wood.normals, triangle[1] as usize) * w[1]
                    + vector(&wood.normals, triangle[2] as usize) * w[2])
                    .normalized();
                incidence[index] = if trunk {
                    n.dot((camera.position - p).normalized())
                } else {
                    1.0
                };
            }
        }
    }
    eprintln!("CPU raster: visible={} grazing={}", depth.iter().filter(|d|d.is_finite()).count(), incidence.iter().filter(|n|(0.12..0.35).contains(*n)).count());
    (8..492)
        .flat_map(|y| (8..792).map(move |x| (x, y)))
        .filter(|&(x, y)| {
            (0.12..0.35).contains(&incidence[y * 800 + x])
                && (y - 2..=y + 2).all(|sy| {
                    (x - 2..=x + 2).all(|sx| {
                        depth[sy * 800 + sx].is_finite() && incidence[sy * 800 + sx] < 1.0
                    })
                })
        })
        .collect()
}
