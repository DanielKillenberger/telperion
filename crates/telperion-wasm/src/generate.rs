//! The build chain the C-ABI runs: growth, wood surface, foliage and field.
//! A field request reads the leaf plan and places no leaf where the family
//! has a plan; only a foliage request, or a family without a plan, places.
use crate::clock;
use serde_json::{json, Value};
use telperion_core::{
    branching,
    field::{Field, FieldSnapshot},
    foliage::{self, plan},
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
) -> Result<(Vec<usize>, usize, usize, usize)> {
    let t = params.resolved()?;
    let mut counts = vec![0usize; t.generations as usize + 1];
    let mut handoffs = 0;
    let mut capped_handoffs = 0;
    let mut twig_count = 0;
    for (i, n) in tree.nodes.iter().enumerate().skip(tree.crossover) {
        twig_count += usize::from(n.kind == NodeKind::Twig && n.branch as usize == i);
        if n.parent.is_some_and(|p| (p as usize) < tree.crossover) {
            handoffs += 1;
            let mut r = n.base_radius;
            let mut generation = 0;
            while r > t.twig.diameter / 2.0 && generation < t.generations as usize {
                r = twigs::child_radius(r, t.length_ratio, t.ratio_power);
                generation += 1;
            }
            counts[generation] += 1;
            capped_handoffs += usize::from(r > t.twig.diameter / 2.0);
        }
    }
    Ok((counts, handoffs, capped_handoffs, twig_count))
}
/// The field selection: absent or `false` asks for no field, `true` for the
/// field at the family's limb order, `{"limbOrder": n}` at order `n`.
fn field_request(v: Option<&Value>) -> Result<Option<Option<u32>>> {
    let Some(v) = v else { return Ok(None) };
    if let Some(flag) = v.as_bool() {
        return Ok(flag.then_some(None));
    }
    let request = v
        .as_object()
        .filter(|o| o.len() == 1)
        .and_then(|o| o.get("limbOrder"))
        .and_then(Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or(Error::InvalidInput("field limb order"))?;
    Ok(Some(Some(request)))
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
    for (k, v) in flags {
        match k.as_str() {
            "surface" | "foliage" | "structure" if v.is_boolean() => {}
            "field" => {}
            _ => return Err(Error::InvalidInput("output selection")),
        }
    }
    let field = field_request(flags.get("field"))?;
    let wants = |key: &str| match key {
        "field" => field.is_some(),
        _ => flags.get(key).and_then(Value::as_bool).unwrap_or(false),
    };
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
    let twig = foliage::TwigPlacement {
        internode_length: t.twig.internode_length,
        stations_per_internode: t.twig.stations_per_internode,
    };
    let mut element = None;
    if wants("foliage") || wants("field") {
        element = Some(foliage::build_element(f.element)?);
    }
    let start = clock();
    let mut leaf_plan = None;
    if wants("field") {
        leaf_plan = plan::plan(
            &tree,
            f.skeleton.envelope,
            f.canopy,
            Some(twig),
            &f.surface,
            element.as_ref().unwrap(),
            field.flatten(),
        )?;
    }
    let plan_ms = if wants("field") { clock() - start } else { 0.0 };
    let start = clock();
    let mut placed_count = 0;
    let mut leaf_bounds = Value::Null;
    let needs_placement = wants("foliage") || (wants("field") && leaf_plan.is_none());
    let mut anatomy = Value::Null;
    if needs_placement {
        let blade = element.as_ref().unwrap();
        let placed = foliage::place_on_surface(
            &tree,
            f.skeleton.envelope,
            f.skeleton.seed,
            f.canopy,
            Some(twig),
            &f.surface,
            foliage::Reference::of(&f)?,
        )?;
        placed_count = placed.len();
        out.instances = foliage::cull(placed, blade, f.skeleton.envelope, f.shell_depth)?;
        if wants("foliage") {
            anatomy = blade.anatomy.as_ref().map_or(Value::Null, |a| json!({
                "unit": match a.unit { foliage::FoliageUnit::Leaf => "leaf", foliage::FoliageUnit::Needle => "needle" },
                "vertices": [a.vertices.start, a.vertices.end],
                "indices": [a.indices.start, a.indices.end],
                "sections": a.sections.iter().map(|r| [r.start, r.end]).collect::<Vec<_>>()
            }));
            leaf_bounds = out
                .instances
                .bounds(blade)?
                .map_or(Value::Null, |b| bounds(b.min, b.max));
            out.element_positions = blade
                .positions
                .iter()
                .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32])
                .collect();
            out.element_indices = blade.indices.clone();
            out.element_coords = blade.coords.clone();
        }
    }
    let foliage_ms = if needs_placement {
        clock() - start
    } else {
        0.0
    };
    let start = clock();
    if wants("field") {
        out.field = Some(match &leaf_plan {
            Some(leaf_plan) => Field::planned(&tree, leaf_plan)?,
            None => Field::new(&tree, Some((&out.instances, element.as_ref().unwrap())))?,
        });
    }
    let field_ms = if wants("field") { clock() - start } else { 0.0 };
    let retained_count = out.instances.len();
    let reference = out.instances.reference;
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
        // Stations the plan counts before any cull; zero without a plan.
        "leavesPlanned":leaf_plan.as_ref().map_or(0,|p|p.total),
        "surfaceBounds":out.surface.as_ref().and_then(|s|s.bounds).map(|b|bounds(b.min,b.max)),
        "foliageBounds":leaf_bounds,
        // The box every leaf position is quantised against. A reader of the
        // placement buffer decodes its three words with nothing else.
        "foliageReference":{
            "min":[reference.min.x, reference.min.y, reference.min.z],
            "extent":[reference.extent.x, reference.extent.y, reference.extent.z]
        },
        "foliageAnatomy":anatomy,
        "biologicalUnits": if needs_placement && element.as_ref().is_some_and(|e| e.anatomy.is_some()) { Some(retained_count) } else { None },
        "fieldBounds":out.field.as_ref().and_then(Field::bounds).map(|b|bounds(b.min,b.max)),
        "fieldBytes":out.field.as_ref().map_or(0,Field::storage_bytes),
        "timings":{"growthMs":growth_ms,"surfaceMs":surface_ms,"planMs":plan_ms,"foliageMs":foliage_ms,"fieldMs":field_ms,"coreMs":clock()-started},
        // Which stages ran: placement and the cull run together, the contact
        // surface only under placement with surface contact, the plan for a
        // field request the family's plan can describe.
        "stages":{"surface":wants("surface"),"foliage":needs_placement,"placement":needs_placement,"cull":needs_placement,
            "contacts":needs_placement && f.canopy.surface_contact > 0.0,"plan":leaf_plan.is_some(),"field":wants("field"),
            "fieldSource":if !wants("field") { Value::Null } else if leaf_plan.is_some() { json!("plan") } else { json!("placed") }}
    });
    Ok((out, meta))
}

#[cfg(test)]
mod tests;
