//! One owned engine per Wasm instance. No caller pointers enter native code.
mod generate;
use generate::{generate, Output};
use serde_json::{json, Value};
use std::cell::RefCell;
use telperion_core::{params, Error, Result};

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
            // The field's grid, one byte a cell: wood 1, foliage 2. Slots 9
            // to 13 held the removed field snapshot.
            8 => o.field.occupancy.as_ptr(),
            // Surface coordinates, two floats a vertex, beside the positions
            // in slots 0 and 3. A consumer that ignores them reads as before.
            14 => o
                .surface
                .as_ref()
                .map_or(std::ptr::null(), |s| s.coords.as_ptr().cast()),
            15 => o.element_coords.as_ptr().cast(),
            // Slots 16 to 18 held the removed growth path's buffers.
            // Beside the occupancy bits in slot 8: one f32 leaf-count
            // estimate and one u32 limb id (u32::MAX for none) per cell.
            19 => o.field.leaves.as_ptr().cast(),
            20 => o.field.limbs.as_ptr().cast(),
            // Slots 21 to 24 and 26 held the removed field snapshot's plan.
            // Beside slots 19 and 20: one f32 wood radius per cell, the
            // thickest wood sweep reaching it in metres, zero without wood.
            25 => o.field.wood_radii.as_ptr().cast(),
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
            8 => o.field.occupancy.len(),
            14 => o.surface.as_ref().map_or(0, |s| s.coords.len()),
            15 => o.element_coords.len(),
            19 => o.field.leaves.len(),
            20 => o.field.limbs.len(),
            25 => o.field.wood_radii.len(),
            _ => 0,
        }
    })
}
