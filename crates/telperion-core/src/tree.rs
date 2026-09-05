use crate::{math::Vec3, Error, Result};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    #[default]
    Structural,
    Branch,
    Twig,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub position: Vec3,
    /// None only at the root; otherwise strictly earlier than this node.
    pub parent: Option<u32>,
    /// Solved distal and proximal edge radii; zero before radius solving.
    pub radius: f64,
    pub start_radius: f64,
    /// Allocation at the origin of this entire branch run, not this edge.
    pub base_radius: f64,
    /// First node of the branch run; structural nodes use their own index.
    pub branch: u32,
    pub kind: NodeKind,
}
impl Node {
    pub fn root() -> Self {
        Self {
            position: Vec3::ZERO,
            parent: None,
            radius: 0.0,
            start_radius: 0.0,
            base_radius: 0.0,
            branch: 0,
            kind: NodeKind::Structural,
        }
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Diagnostics {
    pub node_capped: bool,
    pub level_capped: bool,
    pub attraction_capped: bool,
}
impl Diagnostics {
    pub fn complete(self) -> bool {
        !(self.node_capped || self.level_capped || self.attraction_capped)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tree {
    pub nodes: Vec<Node>,
    pub crossover: usize,
    pub diagnostics: Diagnostics,
}
impl Tree {
    pub fn validate(&self) -> Result<()> {
        if self.nodes.len() > u32::MAX as usize || self.crossover > self.nodes.len() {
            return Err(Error::InvalidInput("tree length"));
        }
        for (i, n) in self.nodes.iter().enumerate() {
            if !n.position.is_finite()
                || ![n.radius, n.start_radius, n.base_radius]
                    .iter()
                    .all(|r| r.is_finite() && *r >= 0.0)
                || n.start_radius < n.radius
            {
                return Err(Error::InvalidInput("node position or radius"));
            }
            if (i == 0 && n.parent.is_some())
                || (i > 0 && !n.parent.is_some_and(|p| (p as usize) < i))
            {
                return Err(Error::InvalidInput("parent order"));
            }
            if n.branch as usize > i {
                return Err(Error::InvalidInput("branch index"));
            }
        }
        Ok(())
    }
    pub fn validate_solved(&self) -> Result<()> {
        self.validate()?;
        if self.nodes.iter().any(|n| n.radius <= 0.0) {
            return Err(Error::InvalidInput("unsolved radii"));
        }
        Ok(())
    }
}
