//! A coarse tree's stand-in answers foliage by its expected occupancy
//! (fn-214, host decision 2026-10-09): a cell inside its capsule is foliage
//! with the chance 1 − e^(−λ) that at least one of its expected leaves lands
//! there, and no cell below the ground is.
use super::Field;
use crate::math::Vec3;
use crate::pipeline::foliage::plan::{Descriptor, Plan, Standing};
use crate::tree::Tree;

const CELL: f64 = 0.1;

fn stand(a: Vec3, b: Vec3, spread: f64, count: u32) -> Field {
    let plan = Plan {
        descriptors: vec![Descriptor {
            endpoints: [a, b],
            radii: [0.0; 2],
            count,
            system: 0,
            side: Vec3::ZERO,
            stand: Some(Standing {
                spread,
                key: 0x5eed,
            }),
        }],
        total: count,
        blade: 0.0,
        seat: 1.0,
        rachis: 0.0,
    };
    Field::planned(&Tree::default(), &plan).expect("a field")
}

/// Every cell of the grid whose cube lies wholly inside the capsule about
/// the y axis from `y0` to `y1` of radius `r`, with whether it answers.
fn inside(field: &Field, y0: f64, y1: f64, r: f64) -> Vec<(Vec3, bool)> {
    let half = CELL / 2.0;
    let n = ((r + y1.abs().max(y0.abs())) / CELL).ceil() as i64 + 1;
    let mut cells = Vec::new();
    for i in -n..n {
        for j in -n..n {
            for k in -n..n {
                let c = Vec3::new(i as f64 + 0.5, j as f64 + 0.5, k as f64 + 0.5) * CELL;
                let along = c.y.clamp(y0, y1);
                let d = (c - Vec3::new(0.0, along, 0.0)).length();
                if d + half * 3_f64.sqrt() < r {
                    cells.push((c, field.query(c, half).expect("answers").foliage));
                }
            }
        }
    }
    cells
}

fn capsule(r: f64, length: f64) -> f64 {
    std::f64::consts::PI * r * r * length + 4.0 / 3.0 * std::f64::consts::PI * r.powi(3)
}

#[test]
fn a_stand_in_fills_its_expected_share_of_its_cells() {
    let (r, y0, y1) = (0.5, 1.0, 3.0);
    for lambda in [0.1, 0.5, 2.0] {
        let count = (lambda * capsule(r, y1 - y0) / CELL.powi(3)).round() as u32;
        let field = stand(Vec3::new(0.0, y0, 0.0), Vec3::new(0.0, y1, 0.0), r, count);
        let cells = inside(&field, y0, y1, r);
        let share = cells.iter().filter(|c| c.1).count() as f64 / cells.len() as f64;
        let expected = 1.0 - (-lambda).exp();
        let error = 4.0 * (expected * (1.0 - expected) / cells.len() as f64).sqrt();
        assert!(
            (share - expected).abs() < error,
            "λ {lambda}: {share:.3} of {} cells, expected {expected:.3} ± {error:.3}",
            cells.len()
        );
    }
}

#[test]
fn no_stand_in_answers_below_the_ground() {
    let (r, y0, y1) = (0.5, -0.5, 0.5);
    let count = (20.0 * capsule(r, y1 - y0) / CELL.powi(3)) as u32;
    let field = stand(Vec3::new(0.0, y0, 0.0), Vec3::new(0.0, y1, 0.0), r, count);
    let cells = inside(&field, y0, y1, r);
    assert!(
        cells.iter().all(|(c, f)| c.y > 0.0 || !f),
        "foliage below the ground"
    );
    assert!(
        cells.iter().filter(|(c, _)| c.y > 0.0).all(|(_, f)| *f),
        "λ 20 fills"
    );
}
