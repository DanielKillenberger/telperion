//! What the C-ABI makes of a build request: the core's pipeline runs the
//! stages, and this reads the request and hands the artifacts on as buffers
//! and metadata. A field request names its grid and reads the leaf plan; it
//! places no leaf, and a family the plan cannot describe is refused by name.
use crate::clock;
use serde_json::{json, Value};
use telperion_core::{field::Grid, foliage, math::Vec3, params, pipeline, surface, Error, Result};

#[derive(Default)]
pub(crate) struct Output {
    pub(crate) surface: Option<surface::SurfaceMesh>,
    pub(crate) element_positions: Vec<f32>,
    pub(crate) element_indices: Vec<u32>,
    pub(crate) element_coords: Vec<f32>,
    pub(crate) instances: foliage::Instances,
    /// The flowers, a second foliage: their element's positions and
    /// indices and their florets, packed against their own reference; none
    /// where the tree flowers nowhere or no foliage was asked for.
    pub(crate) organ_positions: Vec<f32>,
    pub(crate) organ_indices: Vec<u32>,
    pub(crate) organ: Option<foliage::Instances>,
    pub(crate) structure: Vec<f64>,
    pub(crate) topology: Vec<u32>,
    pub(crate) field: FieldCells,
}
/// The field's grid as the binding hands it on, one entry a cell in the
/// grid's order: the flags (wood 1, foliage 2, flowers 4), the leaf
/// estimate, the limb system (`u32::MAX` for none), the thickest wood
/// radius and the florets (fn-316).
#[derive(Default)]
pub(crate) struct FieldCells {
    pub(crate) occupancy: Vec<u8>,
    pub(crate) leaves: Vec<f32>,
    pub(crate) limbs: Vec<u32>,
    pub(crate) wood_radii: Vec<f32>,
    pub(crate) flowers: Vec<f32>,
}
impl FieldCells {
    fn of(grid: &Grid) -> Result<Self> {
        let n = grid.cells.len();
        let mut out = Self::default();
        for reserve in [
            out.occupancy.try_reserve_exact(n),
            out.leaves.try_reserve_exact(n),
            out.limbs.try_reserve_exact(n),
            out.wood_radii.try_reserve_exact(n),
            out.flowers.try_reserve_exact(n),
        ] {
            reserve.map_err(|_| Error::ResourceLimit("field output"))?;
        }
        for c in &grid.cells {
            let flowers = u8::from(c.flowers > 0.0) << 2;
            out.occupancy
                .push(u8::from(c.wood) | (u8::from(c.foliage) << 1) | flowers);
            out.leaves.push(c.leaves as f32);
            out.limbs.push(c.limb.unwrap_or(u32::MAX));
            out.wood_radii.push(c.wood_radius as f32);
            out.flowers.push(c.flowers as f32);
        }
        Ok(out)
    }
}
fn bounds(min: Vec3, max: Vec3) -> Value {
    json!({"min":[min.x,min.y,min.z],"max":[max.x,max.y,max.z]})
}
/// The twig detail a build at its node budget kept, in sixteenths of a
/// generation; null where the budget took none, so a reduced tree reads apart
/// from a full one.
fn twig_detail(d: telperion_core::tree::Diagnostics) -> Option<u16> {
    d.reduced().then_some(d.twig_detail).flatten()
}

/// The field selection: absent or `false` asks for no field;
/// `{"n": n}` the grid of `n` cells a side at the family's limb order, and
/// `{"n": n, "limbOrder": k}` at order `k`.
fn field_request(v: Option<&Value>) -> Result<Option<pipeline::FieldRequest>> {
    let Some(v) = v else { return Ok(None) };
    if v.as_bool() == Some(false) {
        return Ok(None);
    }
    let refused = Error::InvalidInput("field request");
    let o = v.as_object().ok_or(refused.clone())?;
    if o.keys().any(|k| k != "n" && k != "limbOrder") {
        return Err(refused);
    }
    let whole = |key: &str| -> Result<Option<u32>> {
        o.get(key)
            .map(|v| v.as_u64().and_then(|n| u32::try_from(n).ok()))
            .map(|n| n.ok_or(refused.clone()))
            .transpose()
    };
    let n = whole("n")?.ok_or(refused.clone())?;
    Ok(Some(pipeline::FieldRequest {
        n,
        limb_order: whole("limbOrder")?,
    }))
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
    let mut out = Output {
        surface: o.wood,
        field: o
            .field
            .as_ref()
            .map(FieldCells::of)
            .transpose()?
            .unwrap_or_default(),
        ..Output::default()
    };
    let grid = o.field.as_ref().map(|g| {
        let f = g.frame;
        json!({"n": f.n, "cell": f.cell, "origin": [f.origin.x, f.origin.y, f.origin.z]})
    });
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
    // A window of no length flowers never: its organ has no floret and
    // crosses as none, not as an empty one.
    if let Some(organ) = o.organ.filter(|o| !o.instances.leaves.is_empty()) {
        out.organ_positions = organ
            .element
            .positions
            .iter()
            .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32])
            .collect();
        out.organ_indices = organ.element.indices;
        out.organ = Some(organ.instances);
    }
    let reference = out.instances.reference;
    let organ_reference = out.organ.as_ref().map(|o| {
        let r = o.reference;
        json!({"min": [r.min.x, r.min.y, r.min.z], "extent": [r.extent.x, r.extent.y, r.extent.z]})
    });
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
        "nodes":tree.nodes.len(),"crossover":tree.crossover,
        "capped":tree.diagnostics.node_capped,"levelCapped":tree.diagnostics.level_capped,
        "attractionCapped":tree.diagnostics.attraction_capped,"complete":tree.diagnostics.complete(),
        "twigDetail":twig_detail(tree.diagnostics),
        "leavesPlaced":placed_count,"instances":retained_count,
        // Stations the plan counts before any cull; zero without a plan.
        "leavesPlanned":leaf_plan.map_or(0,|p|p.total),
        "surfaceBounds":out.surface.as_ref().and_then(|s|s.bounds).map(|b|bounds(b.min,b.max)),
        // The wood's chunks, each read field by field through `chunk_ptr`.
        "surfaceChunks":out.surface.as_ref().map_or(0,|s|s.chunks.len()),
        "foliageBounds":leaf_bounds,
        // The box every leaf position is quantised against. A reader of the
        // placement buffer decodes its three words with nothing else.
        "foliageReference":{
            "min":[reference.min.x, reference.min.y, reference.min.z],
            "extent":[reference.extent.x, reference.extent.y, reference.extent.z]
        },
        "foliageAnatomy":anatomy,
        // The box the flowers' florets are quantised against; null where
        // the tree grew none.
        "organReference":organ_reference,
        "biologicalUnits": if places && o.element.as_ref().is_some_and(|e| e.anatomy.is_some()) { Some(retained_count) } else { None },
        // The field's grid: its side, its cell and its least corner.
        "field":grid,
        "timings":{"growthMs":t.skeleton_ms,"surfaceMs":wood_rings+t.wood_ms,"planMs":t.plan_ms,
            "foliageMs":leaf_rings+t.placement_ms+t.cull_ms,"fieldMs":t.field_ms,"coreMs":t.total_ms},
        // Which stages ran: placement and the cull run together, the contact
        // surface only under placement with surface contact, the plan for a
        // field request.
        "stages":{"surface":wants("surface"),"foliage":places,"placement":places,"cull":places,
            "contacts":places && f.canopy.surface_contact > 0.0,"plan":leaf_plan.is_some(),"field":field.is_some()}
    });
    Ok((out, meta))
}

#[cfg(test)]
mod tests;
