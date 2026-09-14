//! The guards the hill climb may not break, measured in the process that is
//! climbing rather than by spawning a test run per candidate.
//!
//! `bark_distance`, `bark_resolution` and `look` are the three the task names.
//! Each of their assertions is a number this file computes at the same poses,
//! against the same masks and the same bounds: the trunk pose and the two
//! distance poses for the oak, the grazing pose for both species, and the
//! redraw equality every one of them asserts. `look` draws the ordinary
//! family, whose plate rows are all zero, so no row this climb moves can
//! reach it; the shipped test binaries are still run on the values that are
//! kept, which is where that guard is actually honoured.
use telperion_core::{material::MaterialParams, math::Vec3, mesh::TreeMesh, surface::SurfaceMesh};
use telperion_render::{
    crop_mean, measure_frame, render, Camera, Renderer, Still, Structure, View,
};

/// One pose, and what is asked of it. An empty mask asks only that the draw
/// repeat itself; the scoring pose is the one the still is taken at.
pub struct Case {
    pub name: &'static str,
    pub camera: Camera,
    pub mask: Vec<(usize, usize)>,
    pub scores: bool,
}

pub struct Check {
    pub name: &'static str,
    pub mean: f64,
    pub p95: f64,
    pub redrew: bool,
}

pub struct Reading {
    pub structure: Structure,
    /// The scoring crop's mean colour, in code values.
    pub colour: [f64; 3],
    pub checks: Vec<Check>,
}

impl Reading {
    /// fn-29's colour is not this spec's to redo, and a structure score taken
    /// on the grey of the crop would happily buy a darker furrow with a
    /// bluer trunk. A candidate may not move any channel of the crop's mean
    /// by more than six code values from where the shipped rows stand, and
    /// may not reorder the channels.
    pub fn keeps_colour(&self, start: [f64; 3]) -> bool {
        let order = |c: [f64; 3]| {
            let mut index = [0, 1, 2];
            index.sort_by(|&a, &b| c[b].total_cmp(&c[a]));
            index
        };
        order(self.colour) == order(start)
            && self
                .colour
                .iter()
                .zip(&start)
                .all(|(now, then)| (now - then).abs() <= 6.0)
    }

    /// The bounds the shipped tests assert, unchanged: three code values of
    /// mean error against an exact box reduction, twelve at the 95th, and a
    /// redraw that is byte for byte the frame before it.
    pub fn holds(&self) -> bool {
        self.checks
            .iter()
            .all(|c| c.redrew && c.mean <= 3.0 && c.p95 <= 12.0)
    }
}

pub struct Guard {
    renderer: Renderer,
    cases: Vec<Case>,
}

impl Guard {
    pub fn new(renderer: Renderer, cases: Vec<Case>) -> Self {
        Self { renderer, cases }
    }

    /// The still's structure, and - when the guards are asked for - what the
    /// three shipped tests would measure at these rows. A candidate the score
    /// has already rejected never pays for the guards.
    pub fn read(&mut self, material: MaterialParams, guarded: bool) -> Reading {
        self.renderer.set_material(material);
        self.renderer.set_view(View::Bare);
        let mut structure = None;
        let mut colour = [0.0; 3];
        let mut checks = Vec::new();
        for case in &self.cases {
            if !guarded && !case.scores {
                continue;
            }
            let high = render(&mut self.renderer, &case.camera, 1600, 1000).expect("a frame");
            if case.scores {
                structure = Some(
                    measure_frame(&high.rgba, 1600, 1000).expect("the frame carries the crop"),
                );
                colour = crop_mean(&high.rgba, 1600, 1000).expect("the frame carries the crop");
            }
            if !guarded {
                continue;
            }
            let low = render(&mut self.renderer, &case.camera, 800, 500).expect("a frame");
            let again = render(&mut self.renderer, &case.camera, 800, 500).expect("a frame");
            let (mean, p95) = if case.mask.is_empty() {
                (0.0, 0.0)
            } else {
                agreement(&high, &low, &case.mask)
            };
            checks.push(Check {
                name: case.name,
                mean,
                p95,
                redrew: low.rgba == again.rgba,
            });
        }
        Reading {
            structure: structure.expect("one case scores"),
            colour,
            checks,
        }
    }
}

/// `tests/common/resolution.rs`, computing the same number: the mean and 95th
/// percentile difference between a half-size draw and an exact 2x2 box
/// reduction of the full-size one, over a fixed material-independent mask.
fn agreement(high: &Still, low: &Still, mask: &[(usize, usize)]) -> (f64, f64) {
    let mut errors = Vec::with_capacity(mask.len() * 3);
    for &(x, y) in mask {
        for c in 0..3 {
            let sum: u32 = (0..2)
                .flat_map(|dy| (0..2).map(move |dx| (dx, dy)))
                .map(|(dx, dy)| u32::from(high.rgba[((y * 2 + dy) * 1600 + x * 2 + dx) * 4 + c]))
                .sum();
            errors.push((f64::from(sum) / 4.0 - f64::from(low.rgba[(y * 800 + x) * 4 + c])).abs());
        }
    }
    let mean = errors.iter().sum::<f64>() / errors.len() as f64;
    errors.sort_by(f64::total_cmp);
    (mean, errors[errors.len() * 95 / 100])
}

pub fn camera(position: Vec3, target: Vec3, field_of_view: f64) -> Camera {
    Camera {
        position,
        target,
        field_of_view,
        near: 0.01,
        far: 1000.0,
    }
}

/// The oak's cases: the trunk pose `bark_resolution` pins, which is also the
/// pose the still is taken at, and the two distance poses `bark_distance`
/// walks back to.
pub fn oak_cases(tree: &TreeMesh) -> Vec<Case> {
    let target = Vec3::new(0.0, 2.0, 0.0);
    let offset = Vec3::new(1.7307636095778745, 0.2967023330704928, -1.7307636095778745);
    let mut cases = vec![Case {
        name: "trunk",
        camera: camera(target + offset, target, 38.0),
        mask: (8..492)
            .flat_map(|y| (280..520).map(move |x| (x, y)))
            .collect(),
        scores: true,
    }];
    for distance in [2usize, 4] {
        cases.push(Case {
            name: if distance == 2 {
                "distance-2x"
            } else {
                "distance-4x"
            },
            camera: camera(target + offset * distance as f64, target, 38.0),
            mask: (200..300)
                .flat_map(|y| (400 - 100 / distance..400 + 100 / distance).map(move |x| (x, y)))
                .collect(),
            scores: false,
        });
    }
    cases.push(grazing(tree, 2.0));
    cases
}

/// The spruce's cases: the trunk pose its still is taken at, and the grazing
/// pose `bark_resolution` pins for it.
pub fn spruce_cases(tree: &TreeMesh) -> Vec<Case> {
    let target = Vec3::new(0.0, 0.65, 0.0);
    let offset = Vec3::new(1.7307636095778745, 0.2967023330704928, -1.7307636095778745);
    vec![
        Case {
            name: "trunk",
            camera: camera(target + offset.normalized() * 1.8, target, 38.0),
            mask: Vec::new(),
            scores: true,
        },
        grazing(tree, 0.9),
    ]
}

fn grazing(tree: &TreeMesh, height: f64) -> Case {
    let camera = camera(
        Vec3::new(
            1.7307636095778745,
            height + 0.2967023330704928,
            -1.7307636095778745,
        ),
        Vec3::new(-0.24, height, -0.24),
        20.0,
    );
    let mask = edge_mask(&tree.wood, &camera);
    assert!(mask.len() > 1000, "grazing fixture has too little wood");
    Case {
        name: "grazing",
        camera,
        mask,
        scores: false,
    }
}

/// `bark_resolution`'s own mask, copied because a test's private helper is
/// not a public interface: a CPU raster of the unchanged wood that selects
/// grazing trunk pixels by incidence, never by colour, so the mask is the
/// same for every candidate the climb draws.
fn edge_mask(wood: &SurfaceMesh, camera: &Camera) -> Vec<(usize, usize)> {
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
