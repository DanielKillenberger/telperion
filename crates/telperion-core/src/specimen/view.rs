//! Presentation keeps identity buffers and applies interval records. Wood is
//! swept anew per presentation; leaf transforms come from the record, and
//! short shoots from the wood on screen.
use super::*;
use crate::{
    branching::SpecimenBuffers,
    foliage,
    mesh::{self, TreeMesh},
    surface,
    tree::{Node, NodeKind, Tree},
};
use std::collections::BTreeMap;

pub struct SpecimenView {
    specimen: Specimen,
    family: Family,
    buffers: SpecimenBuffers,
    age: f64,
}
impl SpecimenView {
    pub fn build(family: &Family) -> Result<Self> {
        let specimen = Specimen::build(family)?;
        let buffers = specimen.buffers()?;
        let age = specimen.age();
        Ok(Self {
            specimen,
            family: family.clone(),
            buffers,
            age,
        })
    }
    pub fn age(&self) -> f64 {
        self.age
    }
    pub fn frontier(&self) -> f64 {
        self.specimen.age()
    }
    pub fn material(&self) -> crate::material::MaterialParams {
        self.family.material
    }
    pub fn seek(&mut self, age: f64) -> Result<()> {
        // Validate before any advance. Backward seeks use exactly the same
        // interval reader; growth continues from the frontier, never the dial.
        let age = crate::growth::Age::from_years(age)?.years();
        if age > self.frontier() {
            let from = self.frontier();
            // Capture the return to the frontier before an advance can compact
            // the age currently on screen out of the retained history window.
            let return_to_frontier = (self.age != from)
                .then(|| self.specimen.changes_between(self.age, from))
                .transpose()?;
            let record = self.specimen.advance(age - from)?;
            if let Some(record) = return_to_frontier {
                record.apply(&mut self.buffers)?;
            }
            record.apply(&mut self.buffers)?;
        } else {
            self.specimen
                .changes_between(self.age, age)?
                .apply(&mut self.buffers)?;
        }
        self.age = age.min(self.frontier());
        Ok(())
    }
    pub fn mesh(&self) -> Result<TreeMesh> {
        let tree = self.tree()?;
        let wood = surface::build(&tree, self.specimen.surface_height(), &self.family.surface)?;
        let element = foliage::build_element(self.family.element)?;
        let placements = self.buffers.placements.values();
        let mut instances = foliage::Instances {
            matrices: placements.clone().map(|p| p.transform).collect(),
        };
        let envelope = self.specimen.envelope_at_age(self.age)?;
        // Short shoots are the wood's, not the record's: drawn from the wood
        // on screen by its identity, as a one-shot build of it would draw them.
        let f = &self.family;
        let seed = f.skeleton.seed;
        if f.canopy.limb_clumping > 0.0 {
            // Each recorded leaf is borne by the shoot its identity names.
            let index: BTreeMap<_, _> = (0..tree.nodes.len())
                .map(|i| (tree.nodes[i].identity, i as u32))
                .collect();
            let owners = placements
                .map(|p| index.get(&p.identity.shoot).copied().unwrap_or(0))
                .collect();
            foliage::place_short_shoots_clumped(
                &tree,
                envelope,
                seed,
                f.canopy,
                owners,
                &mut instances,
            )?;
        } else {
            foliage::place_short_shoots(&tree, envelope, seed, f.canopy, &mut instances)?;
        }
        let instances = foliage::cull(&instances, &element, envelope, f.shell_depth)?;
        let bounds = mesh::union(wood.bounds, instances.bounds(&element)?.map(Into::into))
            .unwrap_or(surface::Bounds {
                min: crate::math::Vec3::ZERO,
                max: crate::math::Vec3::Y * 0.01,
            });
        Ok(TreeMesh {
            wood,
            foliage: mesh::Foliage { element, instances },
            bounds,
        })
    }
    fn tree(&self) -> Result<Tree> {
        let mut nodes: Vec<_> = self
            .buffers
            .runs
            .values()
            .flat_map(|run| run.nodes.iter().map(move |n| (run.identity, n)))
            .collect();
        nodes.sort_by_key(|(_, n)| (n.kind != NodeKind::Structural, n.identity));
        let indices: BTreeMap<_, _> = nodes
            .iter()
            .enumerate()
            .map(|(i, (_, n))| (n.identity, i as u32))
            .collect();
        let index = |id| {
            indices
                .get(&id)
                .copied()
                .ok_or(Error::InvalidInput("specimen view run identity"))
        };
        let crossover = nodes
            .iter()
            .take_while(|(_, n)| n.kind == NodeKind::Structural)
            .count();
        let nodes = nodes
            .into_iter()
            .map(|(run, n)| {
                Ok(Node {
                    identity: n.identity,
                    parent: n.parent.map(index).transpose()?,
                    branch: index(run)?,
                    position: n.position,
                    radius: n.radii[0],
                    start_radius: n.radii[1],
                    base_radius: n.radii[2],
                    kind: n.kind,
                    stem: n.stem,
                    ..Node::root()
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Tree {
            nodes,
            crossover,
            ..Tree::default()
        })
    }
}
