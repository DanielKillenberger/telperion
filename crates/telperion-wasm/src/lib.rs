//! One owned engine per Wasm instance. No caller pointers enter native code.
mod params;
use serde_json::{json, Value};
use std::cell::RefCell;
use telperion_core::{
    branching, field::Field, foliage, math::Vec3, surface, tree::NodeKind, twigs, Error, Result,
};

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "env")]
extern "C" {
    fn now() -> f64;
}
fn clock() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        unsafe { now() }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0.0
    }
}
#[derive(Default)]
struct Output {
    surface: Option<surface::SurfaceMesh>,
    element_positions: Vec<f32>,
    element_indices: Vec<u32>,
    instances: foliage::Instances,
    structure: Vec<f64>,
    topology: Vec<u32>,
    field: Option<Field>,
}
#[derive(Default)]
struct Engine {
    input: Vec<u8>,
    output: Output,
    metadata: Vec<u8>,
    queries: Vec<f64>,
    occupancy: Vec<u8>,
    revision: u32,
}
thread_local! { static ENGINE: RefCell<Engine> = RefCell::new(Engine::default()); }
fn status(e: &mut Engine, result: Result<Value>) -> u32 {
    match result {
        Ok(v) => {
            e.metadata = v.to_string().into_bytes();
            0
        }
        Err(err) => {
            let (code, message) = match err {
                Error::InvalidInput(m) => (1, m),
                Error::ResourceLimit(m) => (2, m),
            };
            e.metadata = json!({"error": message, "code": code})
                .to_string()
                .into_bytes();
            code
        }
    }
}
fn reserve<T: Default + Clone>(v: &mut Vec<T>, len: usize) -> Result<()> {
    v.clear();
    v.try_reserve(len)
        .map_err(|_| Error::ResourceLimit("binding allocation"))?;
    v.resize(len, T::default());
    Ok(())
}
#[no_mangle]
pub extern "C" fn request_alloc(len: u32) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.input.clear();
        let result = if len > 65536 {
            Err(Error::InvalidInput("request exceeds 64 KiB"))
        } else {
            reserve(&mut e.input, len as usize)
        };
        status(&mut e, result.map(|_| json!(null)))
    })
}
#[no_mangle]
pub extern "C" fn request_ptr() -> *const u8 {
    ENGINE.with(|e| e.borrow().input.as_ptr())
}
#[no_mangle]
pub extern "C" fn metadata_ptr() -> *const u8 {
    ENGINE.with(|e| e.borrow().metadata.as_ptr())
}
#[no_mangle]
pub extern "C" fn metadata_len() -> usize {
    ENGINE.with(|e| e.borrow().metadata.len())
}
#[no_mangle]
pub extern "C" fn preset(id: u32) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        status(&mut e, params::preset(id).map(|f| params::metadata(&f)))
    })
}
#[no_mangle]
pub extern "C" fn release() {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.output = Output::default();
        e.input.clear();
        e.queries.clear();
        e.occupancy.clear();
        e.revision = e.revision.wrapping_add(1);
    });
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
fn generate(v: Value) -> Result<(Output, Value)> {
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
    if needs_foliage {
        let blade = foliage::build_element(f.element)?;
        let placed = foliage::place(
            &tree,
            f.skeleton.envelope,
            f.skeleton.seed,
            f.canopy,
            Some(foliage::TwigPlacement {
                internode_length: t.twig.internode_length,
                stations_per_internode: t.twig.stations_per_internode,
            }),
        )?;
        placed_count = placed.matrices.len();
        out.instances = foliage::cull(&placed, &blade, f.skeleton.envelope, f.shell_depth)?;
        if wants("foliage") {
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
        "fieldBounds":out.field.as_ref().and_then(Field::bounds).map(|b|bounds(b.min,b.max)),
        "fieldBytes":out.field.as_ref().map_or(0,Field::storage_bytes),
        "timings":{"growthMs":growth_ms,"surfaceMs":surface_ms,"foliageMs":foliage_ms,"fieldMs":field_ms,"coreMs":clock()-started},
        "stages":{"surface":wants("surface"),"foliage":needs_foliage,"field":wants("field")}
    });
    Ok((out, meta))
}
#[no_mangle]
pub extern "C" fn build() -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.output = Output::default();
        e.revision = e.revision.wrapping_add(1);
        let result = serde_json::from_slice(&e.input)
            .map_err(|_| Error::InvalidInput("malformed JSON request"))
            .and_then(generate);
        let result = result.map(|(output, mut meta)| {
            e.output = output;
            meta["revision"] = json!(e.revision);
            meta
        });
        status(&mut e, result)
    })
}
#[no_mangle]
pub extern "C" fn buffer_ptr(slot: u32) -> *const u8 {
    ENGINE.with(|e| {
        let e = e.borrow();
        let o = &e.output;
        match slot {
            0 => o
                .surface
                .as_ref()
                .map_or(std::ptr::null(), |s| s.positions.as_ptr().cast()),
            1 => o
                .surface
                .as_ref()
                .map_or(std::ptr::null(), |s| s.normals.as_ptr().cast()),
            2 => o
                .surface
                .as_ref()
                .map_or(std::ptr::null(), |s| s.indices.as_ptr().cast()),
            3 => o.element_positions.as_ptr().cast(),
            4 => o.element_indices.as_ptr().cast(),
            5 => o.instances.matrices.as_ptr().cast(),
            6 => o.structure.as_ptr().cast(),
            7 => o.topology.as_ptr().cast(),
            8 => e.occupancy.as_ptr(),
            _ => std::ptr::null(),
        }
    })
}
#[no_mangle]
pub extern "C" fn buffer_len(slot: u32) -> usize {
    ENGINE.with(|e| {
        let e = e.borrow();
        let o = &e.output;
        match slot {
            0 => o.surface.as_ref().map_or(0, |s| s.positions.len()),
            1 => o.surface.as_ref().map_or(0, |s| s.normals.len()),
            2 => o.surface.as_ref().map_or(0, |s| s.indices.len()),
            3 => o.element_positions.len(),
            4 => o.element_indices.len(),
            5 => o.instances.matrices.len() * 16,
            6 => o.structure.len(),
            7 => o.topology.len(),
            8 => e.occupancy.len(),
            _ => 0,
        }
    })
}
#[no_mangle]
pub extern "C" fn query_alloc(count: u32) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.occupancy.clear();
        e.queries.clear();
        let result = (count as usize)
            .checked_mul(4)
            .ok_or(Error::ResourceLimit("query size"))
            .and_then(|len| reserve(&mut e.queries, len));
        status(&mut e, result.map(|_| json!(null)))
    })
}
#[no_mangle]
pub extern "C" fn query_ptr() -> *const f64 {
    ENGINE.with(|e| e.borrow().queries.as_ptr())
}
#[no_mangle]
pub extern "C" fn query(revision: u32) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.occupancy.clear();
        let result = (|| {
            if e.revision != revision {
                return Err(Error::InvalidInput("stale field handle"));
            }
            let Engine {
                output,
                queries,
                occupancy,
                ..
            } = &mut *e;
            let field = output
                .field
                .as_ref()
                .ok_or(Error::InvalidInput("no field requested"))?;
            occupancy
                .try_reserve(queries.len() / 4)
                .map_err(|_| Error::ResourceLimit("query output"))?;
            for q in queries.as_chunks::<4>().0 {
                let hit = field.query(Vec3::new(q[0], q[1], q[2]), q[3])?;
                occupancy.push(u8::from(hit.wood) | (u8::from(hit.foliage) << 1));
            }
            Ok(json!(null))
        })();
        if result.is_err() {
            e.occupancy.clear();
        }
        status(&mut e, result)
    })
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
}
