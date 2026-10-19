//! One owned engine per Wasm instance. No caller pointers enter native code.
mod generate;
#[cfg(any(target_arch = "wasm32", test))]
mod live;
use generate::{generate, Output};
use serde_json::{json, Value};
use std::cell::RefCell;
use telperion_core::{params, Error, Result};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: live::Counting = live::Counting;

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
                e @ Error::WoodMemory { .. } => (2, e.to_string()),
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
    #[cfg(target_arch = "wasm32")]
    telperion_core::memory::set_left(live::left);
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
/// One output buffer by slot, as bytes and its length in elements; any
/// other slot is empty.
fn slot(o: &Output, slot: u32) -> (*const u8, usize) {
    fn of<T>(v: &[T]) -> (*const u8, usize) {
        (v.as_ptr().cast(), v.len())
    }
    match slot {
        3 => of(&o.element_positions),
        4 => of(&o.element_indices),
        5 => (
            o.instances.leaves.as_ptr().cast(),
            o.instances.leaves.len() * telperion_core::foliage::WORDS,
        ),
        6 => of(&o.structure),
        7 => of(&o.topology),
        // The field's grid, one byte a cell: wood 1, foliage 2. Slots 9
        // to 13 held the removed field snapshot.
        8 => of(&o.field.occupancy),
        // Slots 0 to 2 and 14 held the flat wood; `chunk_ptr` reads its
        // chunks now.
        15 => of(&o.element_coords),
        // Slots 16 to 18 held the removed growth path's buffers.
        // Beside the occupancy bits in slot 8: one f32 leaf-count
        // estimate and one u32 limb id (u32::MAX for none) per cell.
        19 => of(&o.field.leaves),
        20 => of(&o.field.limbs),
        // Slots 21 to 24 and 26 held the removed field snapshot's plan.
        // Beside slots 19 and 20: one f32 wood radius per cell, the
        // thickest wood sweep reaching it in metres, zero without wood.
        25 => of(&o.field.wood_radii),
        _ => (std::ptr::null(), 0),
    }
}
#[no_mangle]
pub extern "C" fn buffer_ptr(n: u32) -> *const u8 {
    ENGINE.with(|e| slot(&e.borrow().output, n).0)
}
/// Elements, not bytes, in one output buffer (`buffer_ptr`).
#[no_mangle]
pub extern "C" fn buffer_len(n: u32) -> usize {
    ENGINE.with(|e| slot(&e.borrow().output, n).1)
}
/// One field of wood chunk `chunk`, as bytes: 0 its positions, three f32 a
/// vertex; 1 its normals, two i16 a vertex, octahedral; 2 its indices, u16
/// local to the chunk. The metadata's `surfaceChunks` counts the chunks;
/// any other chunk or field is empty.
fn chunk_field(o: &Output, chunk: u32, field: u32) -> (*const u8, usize) {
    let chunk = o
        .surface
        .as_ref()
        .and_then(|s| s.chunks.get(chunk as usize));
    match (chunk, field) {
        (Some(c), 0) => (c.positions.as_ptr().cast(), c.positions.len()),
        (Some(c), 1) => (c.normals.as_ptr().cast(), c.normals.len()),
        (Some(c), 2) => (c.indices.as_ptr().cast(), c.indices.len()),
        _ => (std::ptr::null(), 0),
    }
}
#[no_mangle]
pub extern "C" fn chunk_ptr(chunk: u32, field: u32) -> *const u8 {
    ENGINE.with(|e| chunk_field(&e.borrow().output, chunk, field).0)
}
/// Elements, not bytes, in one field of one wood chunk (`chunk_ptr`).
#[no_mangle]
pub extern "C" fn chunk_len(chunk: u32, field: u32) -> usize {
    ENGINE.with(|e| chunk_field(&e.borrow().output, chunk, field).1)
}
