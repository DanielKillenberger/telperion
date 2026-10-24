//! The ramp: a horizontal cone along x at `AXIS`, its radius rising linearly
//! from the thin end to the thick one, seen side on, so a picture's column
//! stands for one radius. Shared by the tests of colour blended by radius.
use telperion_core::{
    foliage::{Element, Instances},
    material::MaterialParams,
    math::Vec3,
    mesh::{Foliage, TreeMesh},
    surface::{Bounds, Curve, CurvePoint, CurveRun, SurfaceMesh},
};
use telperion_render::{render, Camera, Renderer, Still};

pub const WIDTH: u32 = 320;
pub const HEIGHT: u32 = 160;
const LENGTH: f64 = 3.0;
const THIN: f64 = 0.04;
const THICK: f64 = 0.28;
const AXIS: f64 = 2.0;

/// A straight run along x thickening from `THIN` to `THICK`, as its curve.
pub fn ramp() -> TreeMesh {
    let rings = 151_u32;
    let points = (0..rings)
        .map(|ring| {
            let along = LENGTH * f64::from(ring) / f64::from(rings - 1);
            CurvePoint {
                centre: Vec3::new(along - LENGTH / 2.0, AXIS, 0.0),
                radius: THIN + (THICK - THIN) * along / LENGTH,
                along,
                normal: Vec3::new(0.0, 1.0, 0.0),
                binormal: Vec3::new(0.0, 0.0, 1.0),
            }
        })
        .collect();
    let run = CurveRun {
        first: 0,
        count: rings,
        trunk: false,
        section: None,
        largest_radius: THICK,
    };
    TreeMesh {
        wood: SurfaceMesh::default(),
        curve: Curve::of_runs(points, vec![run]),
        foliage: Foliage {
            element: Element::default(),
            instances: Instances::default(),
        },
        organ: None,
        bounds: Bounds {
            min: Vec3::new(-LENGTH / 2.0, AXIS - THICK, -THICK),
            max: Vec3::new(LENGTH / 2.0, AXIS + THICK, THICK),
        },
    }
}

pub fn side() -> Camera {
    Camera {
        position: Vec3::new(0.0, AXIS, 3.2),
        target: Vec3::new(0.0, AXIS, 0.0),
        field_of_view: 30.0,
        near: 0.01,
        far: 100.0,
    }
}

pub fn draw(renderer: &mut Renderer, camera: &Camera, row: MaterialParams) -> Still {
    renderer.set_material(row);
    render(renderer, camera, WIDTH, HEIGHT).unwrap()
}

/// The picture's column at a world x on the cone's axis, and the radius the
/// wood there carries. Only columns clear of both ends are read.
pub fn columns() -> impl Iterator<Item = (usize, f64)> {
    let camera = side();
    let half = camera.position.z * (camera.field_of_view.to_radians() / 2.0).tan();
    let per_metre = f64::from(HEIGHT) / (2.0 * half);
    let span = (LENGTH / 2.0 - 0.1) * per_metre;
    let centre = f64::from(WIDTH) / 2.0;
    ((centre - span) as usize..(centre + span) as usize).map(move |column| {
        let x = (column as f64 + 0.5 - centre) / per_metre;
        (column, THIN + (THICK - THIN) * (x + LENGTH / 2.0) / LENGTH)
    })
}

pub fn pixel(still: &Still, column: usize) -> [i32; 3] {
    let at = ((HEIGHT as usize / 2) * WIDTH as usize + column) * 4;
    [0, 1, 2].map(|c| i32::from(still.rgba[at + c]))
}
