//! The build chain the C-ABI runs: growth, wood surface, foliage and field.
use crate::clock;
use serde_json::{json, Value};
use telperion_core::{
    branching,
    field::{Field, FieldSnapshot},
    foliage,
    math::Vec3,
    params, surface,
    tree::NodeKind,
    twigs, Error, Result,
};

#[derive(Default)]
pub(crate) struct Output {
    pub(crate) surface: Option<surface::SurfaceMesh>,
    pub(crate) element_positions: Vec<f32>,
    pub(crate) element_indices: Vec<u32>,
    pub(crate) element_coords: Vec<f32>,
    pub(crate) instances: foliage::Instances,
    pub(crate) structure: Vec<f64>,
    pub(crate) topology: Vec<u32>,
    pub(crate) field: Option<Field>,
    pub(crate) snapshot: Option<FieldSnapshot>,
}
fn bounds(min: Vec3, max: Vec3) -> Value {
    json!({"min":[min.x,min.y,min.z],"max":[max.x,max.y,max.z]})
}
fn branch_diagnostics(
    tree: &telperion_core::tree::Tree,
    params: twigs::TwigParams,
) -> Result<([usize; twigs::MAX_LEVELS + 1], usize, usize, usize)> {
    let mut counts = [0usize; twigs::MAX_LEVELS + 1];
    let mut handoffs = 0;
    let mut capped_handoffs = 0;
    let mut twig_count = 0;
    let t = params.resolved()?;
    for (i, n) in tree.nodes.iter().enumerate().skip(tree.crossover) {
        twig_count += usize::from(n.kind == NodeKind::Twig && n.branch as usize == i);
        if n.parent.is_some_and(|p| (p as usize) < tree.crossover) {
            handoffs += 1;
            let mut r = n.base_radius;
            let mut generation = 0;
            while r > t.twig.diameter / 2.0 && generation < twigs::MAX_LEVELS {
                r = twigs::child_radius(r, t.length_ratio, t.ratio_power);
                generation += 1;
            }
            counts[generation] += 1;
            capped_handoffs += usize::from(r > t.twig.diameter / 2.0);
        }
    }
    Ok((counts, handoffs, capped_handoffs, twig_count))
}
pub(crate) fn generate(v: Value) -> Result<(Output, Value)> {
    let object = v
        .as_object()
        .ok_or(Error::InvalidInput("build request object"))?;
    if object
        .keys()
        .any(|k| !["family", "outputs"].contains(&k.as_str()))
    {
        return Err(Error::InvalidInput("unknown request member"));
    }
    let f = params::parse(
        v.get("family")
            .ok_or(Error::InvalidInput("family missing"))?,
    )?;
    let flags = v
        .get("outputs")
        .and_then(Value::as_object)
        .ok_or(Error::InvalidInput("outputs object"))?;
    if flags.iter().any(|(k, v)| {
        !["surface", "foliage", "structure", "field"].contains(&k.as_str()) || !v.is_boolean()
    }) {
        return Err(Error::InvalidInput("output selection"));
    }
    let wants = |key: &str| flags.get(key).and_then(Value::as_bool).unwrap_or(false);
    let started = clock();
    let report = branching::generate(&f.skeleton, f.radii)?;
    let tree = report.tree;
    let growth_ms = clock() - started;
    let mut out = Output::default();
    let (counts, handoffs, capped_handoffs, twig_count) =
        branch_diagnostics(&tree, f.skeleton.twigs)?;
    let t = f.skeleton.twigs.resolved()?;
    let start = clock();
    if wants("surface") {
        out.surface = Some(surface::build(
            &tree,
            f.skeleton.envelope.height,
            &f.surface,
        )?);
    }
    let surface_ms = if wants("surface") {
        clock() - start
    } else {
        0.0
    };
    let start = clock();
    let mut placed_count = 0;
    let mut leaf_bounds = Value::Null;
    let needs_foliage = wants("foliage") || wants("field");
    let mut element = None;
    let mut anatomy = Value::Null;
    if needs_foliage {
        let blade = foliage::build_element(f.element)?;
        let placed = foliage::place_on_surface(
            &tree,
            f.skeleton.envelope,
            f.skeleton.seed,
            f.canopy,
            Some(foliage::TwigPlacement {
                internode_length: t.twig.internode_length,
                stations_per_internode: t.twig.stations_per_internode,
            }),
            &f.surface,
        )?;
        placed_count = placed.matrices.len();
        out.instances = foliage::cull(&placed, &blade, f.skeleton.envelope, f.shell_depth)?;
        if wants("foliage") {
            anatomy = blade.anatomy.as_ref().map_or(Value::Null, |a| json!({
                "unit": match a.unit { foliage::FoliageUnit::Leaf => "leaf", foliage::FoliageUnit::Needle => "needle" },
                "vertices": [a.vertices.start, a.vertices.end],
                "indices": [a.indices.start, a.indices.end],
                "sections": a.sections.iter().map(|r| [r.start, r.end]).collect::<Vec<_>>()
            }));
            leaf_bounds = out
                .instances
                .bounds(&blade)?
                .map_or(Value::Null, |b| bounds(b.min, b.max));
            out.element_positions = blade
                .positions
                .iter()
                .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32])
                .collect();
            out.element_indices = blade.indices.clone();
            out.element_coords = blade.coords.clone();
        }
        element = Some(blade);
    }
    let foliage_ms = if needs_foliage { clock() - start } else { 0.0 };
    let start = clock();
    if wants("field") {
        out.field = Some(Field::new(
            &tree,
            element.as_ref().map(|e| (&out.instances, e)),
        )?);
    }
    let field_ms = if wants("field") { clock() - start } else { 0.0 };
    let retained_count = out.instances.matrices.len();
    if !wants("foliage") {
        out.instances = foliage::Instances::default();
    }
    if wants("structure") {
        out.structure
            .try_reserve(tree.nodes.len() * 6)
            .map_err(|_| Error::ResourceLimit("structure transfer"))?;
        out.topology
            .try_reserve(tree.nodes.len() * 3)
            .map_err(|_| Error::ResourceLimit("topology transfer"))?;
        for n in &tree.nodes {
            out.structure.extend([
                n.position.x,
                n.position.y,
                n.position.z,
                n.radius,
                n.start_radius,
                n.base_radius,
            ]);
            out.topology.extend([
                n.parent.unwrap_or(u32::MAX),
                n.branch,
                match n.kind {
                    NodeKind::Structural => 0,
                    NodeKind::Branch => 1,
                    NodeKind::Twig => 2,
                },
            ]);
        }
    }
    let meta = json!({
        "nodes":tree.nodes.len(),"crossover":tree.crossover,"shed":report.shed,
        "capped":tree.diagnostics.node_capped,"levelCapped":tree.diagnostics.level_capped,
        "attractionCapped":tree.diagnostics.attraction_capped,"complete":tree.diagnostics.complete(),
        "handoffs":handoffs,"generationCounts":counts,"levelCappedHandoffs":capped_handoffs,"twigs":twig_count,
        "leavesPlaced":placed_count,"instances":retained_count,
        "surfaceBounds":out.surface.as_ref().and_then(|s|s.bounds).map(|b|bounds(b.min,b.max)),
        "foliageBounds":leaf_bounds,
        "foliageAnatomy":anatomy,
        "biologicalUnits": if element.as_ref().is_some_and(|e| e.anatomy.is_some()) { Some(retained_count) } else { None },
        "fieldBounds":out.field.as_ref().and_then(Field::bounds).map(|b|bounds(b.min,b.max)),
        "fieldBytes":out.field.as_ref().map_or(0,Field::storage_bytes),
        "timings":{"growthMs":growth_ms,"surfaceMs":surface_ms,"foliageMs":foliage_ms,"fieldMs":field_ms,"coreMs":clock()-started},
        "stages":{"surface":wants("surface"),"foliage":needs_foliage,"field":wants("field")}
    });
    Ok((out, meta))
}

#[cfg(test)]
mod tests {
    use super::*;
    use telperion_core::tree::{Node, Tree};
    #[test]
    fn surviving_diagnostics_match_radius_law_and_empty_state() {
        let mut tree = Tree {
            crossover: 2,
            ..Default::default()
        };
        for (i, parent) in [None, Some(0), Some(1), Some(1), Some(1), Some(1), Some(2)]
            .into_iter()
            .enumerate()
        {
            let mut node = Node::root();
            node.parent = parent;
            node.branch = i as u32;
            if i >= 2 {
                node.base_radius = [0.0025, 0.005, 0.02, 0.08, 0.0025][i - 2];
            }
            if i == 2 || i == 6 {
                node.kind = NodeKind::Twig;
            }
            tree.nodes.push(node);
        }
        let params = twigs::TwigParams {
            length_ratio: 0.5,
            ratio_power: 1.0,
            ..Default::default()
        };
        let (counts, handoffs, capped, twigs) = branch_diagnostics(&tree, params).unwrap();
        assert_eq!(handoffs, 4);
        assert_eq!(twigs, 2);
        assert_eq!(capped, 0);
        assert_eq!(counts, [1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            branch_diagnostics(
                &tree,
                twigs::TwigParams {
                    ratio_power: 0.0,
                    ..params
                }
            )
            .unwrap()
            .2,
            3
        );
        tree.nodes.truncate(2);
        assert_eq!(
            branch_diagnostics(&tree, params).unwrap(),
            ([0; 13], 0, 0, 0)
        );
    }

    /// The mesh the renderer draws and the chain this binding runs are one
    /// geometry: same counts, same bounds, family for family.
    #[test]
    fn mesh_build_matches_the_binding_chain_for_oak_and_spruce() {
        use telperion_core::mesh::{self, Detail};
        fn union_bounds(meta: &Value) -> Value {
            let corner = |key: &str, name: &str| -> [f64; 3] {
                let v = meta[key][name].as_array().expect("bounds corner");
                [0, 1, 2].map(|i| v[i].as_f64().expect("bounds component"))
            };
            let (w, f) = (
                corner("surfaceBounds", "min"),
                corner("foliageBounds", "min"),
            );
            let min = Vec3::new(w[0].min(f[0]), w[1].min(f[1]), w[2].min(f[2]));
            let (w, f) = (
                corner("surfaceBounds", "max"),
                corner("foliageBounds", "max"),
            );
            let max = Vec3::new(w[0].max(f[0]), w[1].max(f[1]), w[2].max(f[2]));
            bounds(min, max)
        }
        for id in ["oregon-white-oak", "norway-spruce"] {
            let request = json!({"family": id, "outputs": {"surface": true, "foliage": true}});
            let (out, meta) = generate(request).unwrap_or_else(|e| panic!("{id}: {e}"));
            let wood = out.surface.as_ref().expect("wood surface");
            let expected = (
                wood.positions.len() / 3,
                wood.indices.len() / 3,
                out.instances.matrices.len(),
                union_bounds(&meta),
            );
            drop(out);
            let family = params::by_identity(id).unwrap();
            let m = mesh::build(&family, Detail::Full).unwrap_or_else(|e| panic!("{id}: {e}"));
            assert_eq!(
                (m.wood_vertices(), m.wood_triangles(), m.foliage_instances()),
                (expected.0, expected.1, expected.2),
                "{id}: mesh counts differ from the binding"
            );
            assert_eq!(
                bounds(m.bounds.min, m.bounds.max),
                expected.3,
                "{id}: mesh bounds differ from the binding"
            );
        }
    }
}
