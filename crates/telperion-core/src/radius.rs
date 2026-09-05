//! Structural fork solve and the separate branch-local taper contract.
use crate::{envelope::Envelope, tree::Tree, Error, Result};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadiusParams {
    pub trunk_radius: f64,
    pub fork_exponent: f64,
    pub length_taper: f64,
}
impl Default for RadiusParams {
    fn default() -> Self {
        Self {
            trunk_radius: 0.02,
            fork_exponent: 2.0,
            length_taper: 0.6,
        }
    }
}
impl RadiusParams {
    pub fn resolved(self) -> Result<Self> {
        if ![self.trunk_radius, self.fork_exponent, self.length_taper]
            .iter()
            .all(|v| v.is_finite())
        {
            return Err(Error::InvalidInput("radius parameters"));
        }
        Ok(Self {
            trunk_radius: self.trunk_radius.max(4e-6),
            fork_exponent: self.fork_exponent.clamp(1.0, 8.0),
            length_taper: self.length_taper.max(0.0),
        })
    }
}
pub fn solve(tree: &mut Tree, envelope: Envelope, params: RadiusParams) -> Result<()> {
    tree.validate()?;
    envelope.validate()?;
    let p = params.resolved()?;
    if tree.nodes.is_empty() {
        return Ok(());
    }
    let count = tree.crossover;
    if count == 0 {
        return Err(Error::InvalidInput("structural crossover"));
    }
    let height = envelope.height.max(1e-6);
    let trunk = p.trunk_radius * height;
    let mut shed = vec![0.0; count];
    let mut carried = vec![0.0_f64; count];
    for i in 1..count {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        shed[i] = (shed[parent]
            + p.length_taper * tree.nodes[parent].position.distance(tree.nodes[i].position)
                / height)
            .min(12.0);
    }
    for i in (0..count).rev() {
        let r = if carried[i] > 0.0 {
            carried[i].powf(1.0 / p.fork_exponent)
        } else {
            1.0
        };
        let start = if let Some(parent) = tree.nodes[i].parent {
            let parent = parent as usize;
            let start = r * (shed[i] - shed[parent]).exp();
            carried[parent] += start.powf(p.fork_exponent);
            start
        } else {
            r
        };
        tree.nodes[i].radius = r;
        tree.nodes[i].start_radius = start;
    }
    let scale = trunk / tree.nodes[0].radius;
    for n in &mut tree.nodes[..count] {
        n.radius *= scale;
        n.start_radius *= scale;
    }
    for i in count..tree.nodes.len() {
        tree.nodes[i].start_radius = if tree.nodes[i].branch as usize == i {
            tree.nodes[i].base_radius
        } else {
            tree.nodes[tree.nodes[i].parent.unwrap() as usize].radius
        };
    }
    tree.validate_solved()
}
