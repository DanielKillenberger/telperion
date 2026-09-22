//! The slim C ABI: one owned engine per Wasm instance, one tree at a time.
//! The request is a species id and a seed; the answers are the field's
//! bounds and one batch occupancy query. No JSON crosses the boundary: an
//! error is its code and the core's message bytes.
mod grow;
use std::cell::RefCell;
use telperion_core::{field::Field, math::Vec3, Error, Result};

/// The family's own limb order, as the `limb_order` argument of `grow`.
pub const FAMILY_ORDER: u32 = u32::MAX;
const SPECIES_LIMIT: u32 = 256;

#[derive(Default)]
struct Engine {
    species: Vec<u8>,
    field: Option<Field>,
    bounds: Vec<f64>,
    error: Vec<u8>,
    queries: Vec<f64>,
    flags: Vec<u8>,
    wood_radii: Vec<f32>,
    leaves: Vec<f32>,
    limbs: Vec<u32>,
    revision: u32,
}
impl Engine {
    fn clear_answers(&mut self) {
        self.flags.clear();
        self.wood_radii.clear();
        self.leaves.clear();
        self.limbs.clear();
    }
    fn drop_tree(&mut self) {
        self.field = None;
        self.bounds.clear();
        self.queries.clear();
        self.clear_answers();
        self.revision = self.revision.wrapping_add(1);
    }
}
thread_local! { static ENGINE: RefCell<Engine> = RefCell::new(Engine::default()); }

/// Zero on success; 1 for an invalid input, 2 for a resource limit, with the
/// core's message readable at `error_ptr` until the next call.
fn status(e: &mut Engine, result: Result<()>) -> u32 {
    e.error.clear();
    match result {
        Ok(()) => 0,
        Err(err) => {
            e.error = err.to_string().into_bytes();
            match err {
                Error::InvalidInput(_) | Error::InvalidValue { .. } => 1,
                Error::ResourceLimit(_) => 2,
            }
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
pub extern "C" fn species_alloc(len: u32) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.species.clear();
        let result = if len > SPECIES_LIMIT {
            Err(Error::InvalidInput("species id exceeds 256 bytes"))
        } else {
            reserve(&mut e.species, len as usize)
        };
        status(&mut e, result)
    })
}
#[no_mangle]
pub extern "C" fn species_ptr() -> *const u8 {
    ENGINE.with(|e| e.borrow().species.as_ptr())
}
#[no_mangle]
pub extern "C" fn error_ptr() -> *const u8 {
    ENGINE.with(|e| e.borrow().error.as_ptr())
}
#[no_mangle]
pub extern "C" fn error_len() -> usize {
    ENGINE.with(|e| e.borrow().error.len())
}
/// Grows the species written at `species_ptr` at `seed`, with limbs named at
/// `limb_order` (`FAMILY_ORDER` for the family's own). Drops the previous
/// tree and advances the revision whether or not the growth succeeds.
#[no_mangle]
pub extern "C" fn grow(seed: u32, limb_order: u32) -> u32 {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.drop_tree();
        let result = (|| {
            let species =
                std::str::from_utf8(&e.species).map_err(|_| Error::InvalidInput("species id"))?;
            let order = (limb_order != FAMILY_ORDER).then_some(limb_order);
            let field = grow::field(species, seed, order)?;
            if let Some(b) = field.bounds() {
                e.bounds
                    .try_reserve(6)
                    .map_err(|_| Error::ResourceLimit("bounds"))?;
                e.bounds
                    .extend([b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z]);
            }
            e.field = Some(field);
            Ok(())
        })();
        status(&mut e, result)
    })
}
#[no_mangle]
pub extern "C" fn revision() -> u32 {
    ENGINE.with(|e| e.borrow().revision)
}
/// Six f64, min xyz then max xyz; length zero while no tree stands or the
/// field is empty.
#[no_mangle]
pub extern "C" fn bounds_ptr() -> *const f64 {
    ENGINE.with(|e| e.borrow().bounds.as_ptr())
}
#[no_mangle]
pub extern "C" fn bounds_len() -> usize {
    ENGINE.with(|e| e.borrow().bounds.len())
}
#[no_mangle]
pub extern "C" fn release() {
    ENGINE.with(|e| {
        let mut e = e.borrow_mut();
        e.drop_tree();
        e.species.clear();
    });
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
        status(&mut e, result)
    })
}
#[no_mangle]
pub extern "C" fn query_ptr() -> *const f64 {
    ENGINE.with(|e| e.borrow().queries.as_ptr())
}
/// Answers every cell written at `query_ptr` (x, y, z, half extent each) for
/// the tree of `revision`. A batch with one invalid cell answers nothing.
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
                field,
                queries,
                flags,
                wood_radii,
                leaves,
                limbs,
                ..
            } = &mut *e;
            let field = field.as_ref().ok_or(Error::InvalidInput("no tree grown"))?;
            let count = queries.len() / 4;
            for reserve in [
                flags.try_reserve(count),
                wood_radii.try_reserve(count),
                leaves.try_reserve(count),
                limbs.try_reserve(count),
            ] {
                reserve.map_err(|_| Error::ResourceLimit("query output"))?;
            }
            for q in queries.as_chunks::<4>().0 {
                let hit = field.query(Vec3::new(q[0], q[1], q[2]), q[3])?;
                flags.push(u8::from(hit.wood) | (u8::from(hit.foliage) << 1));
                wood_radii.push(hit.wood_radius as f32);
                leaves.push(hit.leaves as f32);
                limbs.push(hit.limb.unwrap_or(u32::MAX));
            }
            Ok(())
        })();
        if result.is_err() {
            e.clear_answers();
        }
        status(&mut e, result)
    })
}
/// One answer per cell of the last query: slot 0 the flags (wood 1, foliage
/// 2) as u8, 1 the thickest wood radius in metres as f32, 2 the leaf-count
/// estimate as f32, 3 the owning limb as u32 (u32::MAX for none).
#[no_mangle]
pub extern "C" fn answer_ptr(slot: u32) -> *const u8 {
    ENGINE.with(|e| {
        let e = e.borrow();
        match slot {
            0 => e.flags.as_ptr(),
            1 => e.wood_radii.as_ptr().cast(),
            2 => e.leaves.as_ptr().cast(),
            3 => e.limbs.as_ptr().cast(),
            _ => std::ptr::null(),
        }
    })
}
#[no_mangle]
pub extern "C" fn answer_len(slot: u32) -> usize {
    ENGINE.with(|e| {
        let e = e.borrow();
        match slot {
            0 => e.flags.len(),
            1 => e.wood_radii.len(),
            2 => e.leaves.len(),
            3 => e.limbs.len(),
            _ => 0,
        }
    })
}

#[cfg(test)]
mod tests;
