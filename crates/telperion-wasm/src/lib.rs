//! One owned engine per Wasm instance. No caller pointers enter native code.
mod generate;
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
        e.queries.clear();
        e.occupancy.clear();
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
            5 => o.instances.matrices.as_ptr().cast(),
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
            9 => o.snapshot.as_ref().map_or(0, |s| s.wood.len()),
            10 => o.snapshot.as_ref().map_or(0, |s| s.wood_index.bounds.len()),
            11 => o
                .snapshot
                .as_ref()
                .map_or(0, |s| s.wood_index.topology.len()),
            12 => o.snapshot.as_ref().map_or(0, |s| s.leaves.bounds.len()),
            13 => o.snapshot.as_ref().map_or(0, |s| s.leaves.topology.len()),
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
                "extractionMs":clock()-start});
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
