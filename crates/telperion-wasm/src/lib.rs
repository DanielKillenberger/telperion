//! One owned engine per Wasm instance. No caller pointers enter native code.
mod generate;
mod specimen;
use generate::{generate, Output};
use serde_json::{json, Value};
use std::cell::RefCell;
use telperion_core::{math::Vec3, params, Error, Result};

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "env")]
extern "C" {
    fn now() -> f64;
}
pub(crate) fn clock() -> f64 {
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
struct Engine {
    specimen: telperion_core::specimen::SpecimenStore,
    specimen_bytes: Vec<u8>,
    specimen_record: Vec<u8>,
    specimen_ids: Vec<u8>,
    input: Vec<u8>,
    output: Output,
    metadata: Vec<u8>,
    queries: Vec<f64>,
    occupancy: Vec<u8>,
    leaves: Vec<f32>,
    limbs: Vec<u32>,
    wood_radii: Vec<f32>,
    revision: u32,
}
impl Engine {
    /// Drops every per-cell answer of the last batch query.
    fn clear_answers(&mut self) {
        self.occupancy.clear();
        self.leaves.clear();
        self.limbs.clear();
        self.wood_radii.clear();
    }
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
                Error::InvalidInput(m) => (1, m.to_string()),
                Error::InvalidValue { field, value } => (1, format!("{field} = {value}")),
                Error::ResourceLimit(m) => (2, m.to_string()),
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
/// Generated catalogue carries explicit identities and stable numeric ABI IDs.
#[no_mangle]
pub extern "C" fn catalogue() -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        let entries: Vec<_> = params::CATALOGUE
            .iter()
            .map(|&(abi_id, id, name, note)| {
                json!({"abiId": abi_id, "id": id, "name": name, "note": note,
                "family": params::metadata(&params::preset(abi_id).expect("catalogue identity"))})
            })
            .collect();
        status(&mut e, Ok(json!(entries)))
    })
}
#[no_mangle]
pub extern "C" fn release() {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.output = Output::default();
        e.input.clear();
        e.specimen.release();
        e.specimen_bytes = Vec::new();
        e.specimen_record = Vec::new();
        e.specimen_ids = Vec::new();
        e.queries.clear();
        e.clear_answers();
        e.revision = e.revision.wrapping_add(1);
    });
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
            5 => o.instances.leaves.as_ptr().cast(),
            6 => o.structure.as_ptr().cast(),
            7 => o.topology.as_ptr().cast(),
            8 => e.occupancy.as_ptr(),
            9 => o
                .snapshot
                .as_ref()
                .map_or(std::ptr::null(), |s| s.wood.as_ptr().cast()),
            10 => o
                .snapshot
                .as_ref()
                .map_or(std::ptr::null(), |s| s.wood_index.bounds.as_ptr().cast()),
            11 => o
                .snapshot
                .as_ref()
                .map_or(std::ptr::null(), |s| s.wood_index.topology.as_ptr().cast()),
            12 => o
                .snapshot
                .as_ref()
                .map_or(std::ptr::null(), |s| s.leaves.bounds.as_ptr().cast()),
            13 => o
                .snapshot
                .as_ref()
                .map_or(std::ptr::null(), |s| s.leaves.topology.as_ptr().cast()),
            // Surface coordinates, two floats a vertex, beside the positions
            // in slots 0 and 3. A consumer that ignores them reads as before.
            14 => o
                .surface
                .as_ref()
                .map_or(std::ptr::null(), |s| s.coords.as_ptr().cast()),
            15 => o.element_coords.as_ptr().cast(),
            16 => e.specimen_bytes.as_ptr(),
            17 => e.specimen_record.as_ptr(),
            18 => e.specimen_ids.as_ptr(),
            // Beside the occupancy bits in slot 8: one f32 leaf-count
            // estimate and one u32 limb id (u32::MAX for none) per cell.
            19 => e.leaves.as_ptr().cast(),
            20 => e.limbs.as_ptr().cast(),
            // The plan behind a planned field's snapshot: seven f64 a sweep
            // (endpoints, reach), two u32 (count, system), and its index.
            21 => o
                .snapshot
                .as_ref()
                .map_or(std::ptr::null(), |s| s.plan.as_ptr().cast()),
            22 => o
                .snapshot
                .as_ref()
                .map_or(std::ptr::null(), |s| s.plan_stations.as_ptr().cast()),
            23 => o
                .snapshot
                .as_ref()
                .map_or(std::ptr::null(), |s| s.plan_index.bounds.as_ptr().cast()),
            24 => o
                .snapshot
                .as_ref()
                .map_or(std::ptr::null(), |s| s.plan_index.topology.as_ptr().cast()),
            // Beside slots 19 and 20: one f32 wood radius per cell, the
            // thickest wood sweep reaching it in metres, zero without wood.
            25 => e.wood_radii.as_ptr().cast(),
            // Beside the plan in slot 21: three f64 a sweep, a ribbon's
            // half-width vector (zero for a capsule); empty where the plan
            // holds no ribbon.
            26 => o
                .snapshot
                .as_ref()
                .map_or(std::ptr::null(), |s| s.plan_sides.as_ptr().cast()),
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
            5 => o.instances.leaves.len() * telperion_core::foliage::WORDS,
            6 => o.structure.len(),
            7 => o.topology.len(),
            8 => e.occupancy.len(),
            9 => o.snapshot.as_ref().map_or(0, |s| s.wood.len()),
            10 => o.snapshot.as_ref().map_or(0, |s| s.wood_index.bounds.len()),
            11 => o
                .snapshot
                .as_ref()
                .map_or(0, |s| s.wood_index.topology.len()),
            12 => o.snapshot.as_ref().map_or(0, |s| s.leaves.bounds.len()),
            13 => o.snapshot.as_ref().map_or(0, |s| s.leaves.topology.len()),
            14 => o.surface.as_ref().map_or(0, |s| s.coords.len()),
            15 => o.element_coords.len(),
            16 => e.specimen_bytes.len(),
            17 => e.specimen_record.len(),
            18 => e.specimen_ids.len(),
            19 => e.leaves.len(),
            20 => e.limbs.len(),
            21 => o.snapshot.as_ref().map_or(0, |s| s.plan.len()),
            22 => o.snapshot.as_ref().map_or(0, |s| s.plan_stations.len()),
            23 => o.snapshot.as_ref().map_or(0, |s| s.plan_index.bounds.len()),
            24 => o
                .snapshot
                .as_ref()
                .map_or(0, |s| s.plan_index.topology.len()),
            25 => e.wood_radii.len(),
            26 => o.snapshot.as_ref().map_or(0, |s| s.plan_sides.len()),
            _ => 0,
        }
    })
}
/// Explicit experiment export; ordinary builds and queries never allocate snapshots.
#[no_mangle]
pub extern "C" fn field_snapshot(revision: u32) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.output.snapshot = None;
        let start = clock();
        let result = (|| {
            if e.revision != revision { return Err(Error::InvalidInput("stale field handle")); }
            let snapshot = e.output.field.as_ref().ok_or(Error::InvalidInput("no field requested"))?.snapshot()?;
            let meta = json!({"woodNodes":snapshot.wood_index.node_count,"leafNodes":snapshot.leaves.node_count,
                "planNodes":snapshot.plan_index.node_count,"extractionMs":clock()-start});
            e.output.snapshot = Some(snapshot);
            Ok(meta)
        })();
        status(&mut e, result)
    })
}
#[no_mangle]
pub extern "C" fn field_snapshot_release() {
    ENGINE.with(|e| e.borrow_mut().output.snapshot = None);
}
#[no_mangle]
pub extern "C" fn query_alloc(count: u32) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.clear_answers();
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
        e.clear_answers();
        let result = (|| {
            if e.revision != revision {
                return Err(Error::InvalidInput("stale field handle"));
            }
            let Engine {
                output,
                queries,
                occupancy,
                leaves,
                limbs,
                wood_radii,
                ..
            } = &mut *e;
            let field = output
                .field
                .as_ref()
                .ok_or(Error::InvalidInput("no field requested"))?;
            let count = queries.len() / 4;
            for reserve in [
                occupancy.try_reserve(count),
                leaves.try_reserve(count),
                limbs.try_reserve(count),
                wood_radii.try_reserve(count),
            ] {
                reserve.map_err(|_| Error::ResourceLimit("query output"))?;
            }
            for q in queries.as_chunks::<4>().0 {
                let hit = field.query(Vec3::new(q[0], q[1], q[2]), q[3])?;
                occupancy.push(u8::from(hit.wood) | (u8::from(hit.foliage) << 1));
                leaves.push(hit.leaves as f32);
                limbs.push(hit.limb.unwrap_or(u32::MAX));
                wood_radii.push(hit.wood_radius as f32);
            }
            Ok(json!(null))
        })();
        if result.is_err() {
            e.clear_answers();
        }
        status(&mut e, result)
    })
}
