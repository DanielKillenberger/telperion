//! Measurements of caller-classified fine wood against the authored crown.
use crate::{
    envelope::{distance_to_profile, Envelope},
    tree::Tree,
    Error, Result,
};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillMeasurement {
    Tested {
        numerator: usize,
        denominator: usize,
    },
    Untested(&'static str),
}
impl FillMeasurement {
    pub fn fraction(self) -> Option<f64> {
        match self {
            Self::Tested {
                numerator,
                denominator,
            } => Some(numerator as f64 / denominator as f64),
            Self::Untested(_) => None,
        }
    }
}
fn terminals(tree: &Tree, terminal: &[bool]) -> Result<Vec<usize>> {
    tree.validate()?;
    if terminal.len() != tree.nodes.len() {
        return Err(Error::InvalidInput("terminal mask length"));
    }
    Ok(terminal
        .iter()
        .enumerate()
        .filter_map(|(i, &selected)| selected.then_some(i))
        .collect())
}

/// Equal-volume cells, classified at their centres. Sizes are fractions of height
/// and maximum radius respectively; over two million candidate cells is an error.
pub fn shell_occupancy(
    tree: &Tree,
    terminal: &[bool],
    envelope: Envelope,
    cell_size: f64,
    shell_depth: f64,
) -> Result<FillMeasurement> {
    let selected = terminals(tree, terminal)?;
    envelope.validate()?;
    if !cell_size.is_finite() || cell_size <= 0.0 || !shell_depth.is_finite() || shell_depth < 0.0 {
        return Err(Error::InvalidInput("fill resolution or shell depth"));
    }
    if selected.is_empty() {
        return Ok(FillMeasurement::Untested("no-terminals"));
    }
    let size = cell_size * envelope.height;
    let radius = envelope.max_radius();
    let shell = shell_depth * radius;
    if !size.is_finite() || !shell.is_finite() {
        return Err(Error::InvalidInput("fill scale overflow"));
    }
    if size == 0.0 || radius == 0.0 || envelope.crown_base == 1.0 {
        return Ok(FillMeasurement::Untested("no-shell-cells"));
    }
    let base = envelope.height * envelope.crown_base;
    let lo = (-radius / size).floor();
    let hi = (radius / size).ceil();
    let bottom = (base / size).floor();
    let top = (envelope.height / size).ceil();
    let cells = (hi - lo).powi(2) * (top - bottom);
    if ![lo, hi, bottom, top, cells]
        .iter()
        .all(|x| x.is_finite() && x.abs() <= (1_u64 << 53) as f64)
        || cells > 2_000_000.0
    {
        return Err(Error::ResourceLimit("fill grid"));
    }
    let profile = envelope.profile();
    let in_shell = |x: f64, y: f64, z: f64| {
        if y < base || y > envelope.height {
            return false;
        }
        let r = x.hypot(z);
        let slack = envelope.radius_at(y) - r;
        slack >= 0.0 && (slack <= shell || distance_to_profile(&profile, r, y) <= shell)
    };
    let mut occupied = HashSet::new();
    for i in selected {
        let p = tree.nodes[i].position;
        if in_shell(p.x, p.y, p.z) {
            occupied.insert((
                (p.x / size).floor() as i64,
                (p.y / size).floor() as i64,
                (p.z / size).floor() as i64,
            ));
        }
    }
    let mut denominator = 0;
    let mut numerator = 0;
    for y in bottom as i64..top as i64 {
        for x in lo as i64..hi as i64 {
            for z in lo as i64..hi as i64 {
                if in_shell(
                    (x as f64 + 0.5) * size,
                    (y as f64 + 0.5) * size,
                    (z as f64 + 0.5) * size,
                ) {
                    denominator += 1;
                    numerator += usize::from(occupied.contains(&(x, y, z)));
                }
            }
        }
    }
    Ok(if denominator == 0 {
        FillMeasurement::Untested("no-shell-cells")
    } else {
        FillMeasurement::Tested {
            numerator,
            denominator,
        }
    })
}

/// Appended children do not remove a tip from the colonization reference set.
pub fn tip_clustering(tree: &Tree, terminal: &[bool], distance: f64) -> Result<FillMeasurement> {
    let selected = terminals(tree, terminal)?;
    if !distance.is_finite() || distance < 0.0 {
        return Err(Error::InvalidInput("clustering distance"));
    }
    if selected.is_empty() {
        return Ok(FillMeasurement::Untested("no-terminals"));
    }
    let mut children = vec![false; tree.crossover];
    for n in &tree.nodes[..tree.crossover] {
        if let Some(p) = n.parent {
            children[p as usize] = true;
        }
    }
    let tips: Vec<_> = (1..tree.crossover)
        .filter(|&i| !children[i])
        .map(|i| tree.nodes[i].position)
        .collect();
    if tips.is_empty() {
        return Ok(FillMeasurement::Untested("no-colonization-tips"));
    }
    let numerator = selected
        .iter()
        .filter(|&&i| {
            tips.iter()
                .any(|p| p.distance(tree.nodes[i].position) <= distance)
        })
        .count();
    Ok(FillMeasurement::Tested {
        numerator,
        denominator: selected.len(),
    })
}
