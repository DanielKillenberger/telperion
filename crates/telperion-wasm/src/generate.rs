//! What the C-ABI makes of a build request: the core's pipeline runs the
//! stages, and this reads the request and hands the artifacts on as buffers
//! and metadata. A field request reads the leaf plan and places no leaf where
//! the family has a plan; only a foliage request, or a family without a plan,
//! places.
use crate::clock;
use serde_json::{json, Value};
use telperion_core::{
    field::{Field, FieldSnapshot},
    foliage,
    math::Vec3,
    params, pipeline, surface,
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
    let wants = |key: &str| flags.get(key).and_then(Value::as_bool).unwrap_or(false);
    let built = pipeline::build(
        &f,
        pipeline::Request {
            wood: wants("surface"),
            leaves: wants("foliage"),
            field,
            structure: wants("structure"),
            clock,
            ..Default::default()
        },
    )?;
    let (tree, o) = (&built.skeleton.tree, built.outputs);
    let (counts, handoffs, capped_handoffs, twig_count) =
        branch_diagnostics(tree, f.skeleton.twigs)?;
    let mut out = Output {
        surface: o.wood,
        field: o.field,
        ..Output::default()
    };
    let mut anatomy = Value::Null;
    let mut leaf_bounds = Value::Null;
    let (mut placed_count, mut retained_count) = (0, 0);
    let places = o.leaves.is_some();
    if let Some(leaves) = o.leaves {
        (placed_count, retained_count) = (leaves.placed, leaves.retained);
        leaf_bounds = leaves.bounds.map_or(Value::Null, |b| bounds(b.min, b.max));
        out.instances = leaves.instances;
    }
    if let Some(blade) = o.element.as_ref().filter(|_| wants("foliage")) {
        anatomy = blade.anatomy.as_ref().map_or(Value::Null, |a| json!({
            "unit": match a.unit { foliage::FoliageUnit::Leaf => "leaf", foliage::FoliageUnit::Needle => "needle" },
            "vertices": [a.vertices.start, a.vertices.end],
            "indices": [a.indices.start, a.indices.end],
            "sections": a.sections.iter().map(|r| [r.start, r.end]).collect::<Vec<_>>()
        }));
        out.element_positions = blade
            .positions
            .iter()
            .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32])
            .collect();
        out.element_indices = blade.indices.clone();
        out.element_coords = blade.coords.clone();
    }
    let reference = out.instances.reference;
    if let Some(s) = o.structure {
        (out.structure, out.topology) = (s.nodes, s.topology);
    }
    let t = o.stages;
    // The rings are the wood's where it is drawn, else the seated leaves'.
    let (wood_rings, leaf_rings) = if wants("surface") {
        (t.rings_ms, 0.0)
    } else {
        (0.0, t.rings_ms)
    };
    let leaf_plan = o.plan.as_ref();
    let meta = json!({
        "nodes":tree.nodes.len(),"crossover":tree.crossover,"shed":built.skeleton.shed,
        "capped":tree.diagnostics.node_capped,"levelCapped":tree.diagnostics.level_capped,
        "attractionCapped":tree.diagnostics.attraction_capped,"complete":tree.diagnostics.complete(),
        "handoffs":handoffs,"generationCounts":counts,"levelCappedHandoffs":capped_handoffs,"twigs":twig_count,
        "leavesPlaced":placed_count,"instances":retained_count,
        // Stations the plan counts before any cull; zero without a plan.
        "leavesPlanned":leaf_plan.map_or(0,|p|p.total),
        "surfaceBounds":out.surface.as_ref().and_then(|s|s.bounds).map(|b|bounds(b.min,b.max)),
        "foliageBounds":leaf_bounds,
        // The box every leaf position is quantised against. A reader of the
        // placement buffer decodes its three words with nothing else.
        "foliageReference":{
            "min":[reference.min.x, reference.min.y, reference.min.z],
            "extent":[reference.extent.x, reference.extent.y, reference.extent.z]
        },
        "foliageAnatomy":anatomy,
        "biologicalUnits": if places && o.element.as_ref().is_some_and(|e| e.anatomy.is_some()) { Some(retained_count) } else { None },
        "fieldBounds":out.field.as_ref().and_then(Field::bounds).map(|b|bounds(b.min,b.max)),
        "fieldBytes":out.field.as_ref().map_or(0,Field::storage_bytes),
        "timings":{"growthMs":t.skeleton_ms,"surfaceMs":wood_rings+t.wood_ms,"planMs":t.plan_ms,
            "foliageMs":leaf_rings+t.placement_ms+t.cull_ms,"fieldMs":t.field_ms,"coreMs":t.total_ms},
        // Which stages ran: placement and the cull run together, the contact
        // surface only under placement with surface contact, the plan for a
        // field request the family's plan can describe.
        "stages":{"surface":wants("surface"),"foliage":places,"placement":places,"cull":places,
            "contacts":places && f.canopy.surface_contact > 0.0,"plan":leaf_plan.is_some(),"field":field.is_some(),
            "fieldSource":if field.is_none() { Value::Null } else if leaf_plan.is_some() { json!("plan") } else { json!("placed") }}
    });
    Ok((out, meta))
}

#[cfg(test)]
mod tests;
