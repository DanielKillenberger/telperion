//! fn-257: a tree's field read out on a device of its own, for a page or a
//! worker with no canvas (`telperion/field`). The tree grows on the
//! device and its field is read out there to the grid the call names;
//! only the grid crosses the binding, and nothing stays on the device.
//! Without WebGPU, or on a lost device, every call rejects by name.
use super::*;
use crate::generation::Generator;
use wasm_bindgen_futures::JsFuture;

thread_local! {
    /// The device and the programs every field of this instance grows on.
    static FIELDS: RefCell<Option<Rc<Generator>>> = const { RefCell::new(None) };
    /// Their making while it is under way: every call that asks for them
    /// meanwhile waits on this one, so one instance makes one device.
    static MAKING: RefCell<Option<js_sys::Promise>> = const { RefCell::new(None) };
}

/// The instance's generator, its device requested and its programs
/// compiled once, however many calls ask at once. A lost device is
/// refused by name and let go, so the next call asks for another.
async fn fields() -> std::result::Result<Rc<Generator>, JsError> {
    if let Some(made) = FIELDS.with(|f| f.borrow().clone()) {
        if let Some(lost) = made.lost() {
            FIELDS.with(|f| f.borrow_mut().take());
            return Err(js_error(lost));
        }
        return Ok(made);
    }
    let making = MAKING.with(|m| {
        m.borrow_mut()
            .get_or_insert_with(|| {
                future_to_promise(async {
                    let gpu = Gpu::request(None).await.map_err(js_error)?;
                    let made = Generator::for_fields_async(gpu).await.map_err(js_error)?;
                    made.prepare_programs().await.map_err(js_error)?;
                    FIELDS.with(|f| *f.borrow_mut() = Some(Rc::new(made)));
                    Ok(JsValue::UNDEFINED)
                })
            })
            .clone()
    });
    let made = JsFuture::from(making).await;
    MAKING.with(|m| m.borrow_mut().take());
    made.map_err(|e| {
        let message = e
            .dyn_ref::<js_sys::Error>()
            .map(|e| String::from(e.message()));
        JsError::new(&message.unwrap_or_else(|| "the device for fields was not made".into()))
    })?;
    FIELDS
        .with(|f| f.borrow().clone())
        .ok_or_else(|| JsError::new("the device for fields was not made"))
}

/// A shipped species' field at a seed, read out to a grid
/// (`telperion_core::field::Frame`): `n` cells a side of `cell` metres
/// from `origin`, cell `i + n * (j + n * k)` centred at `origin + (i +
/// 0.5, j + 0.5, k + 0.5) * cell`.
#[wasm_bindgen]
pub struct WebField {
    n: u32,
    cell: f64,
    origin: [f64; 3],
    answers: Vec<u32>,
}

#[wasm_bindgen]
impl WebField {
    /// The device and its programs made ready, so the first field does not
    /// wait on them: a page calls it as soon as it loads (fn-257 R3-0).
    pub fn prepare() -> js_sys::Promise {
        future_to_promise(async move {
            fields().await?;
            Ok(JsValue::UNDEFINED)
        })
    }

    /// `species` grown at `seed` on the device and its field read out to
    /// `n` cells a side over its box, its limb systems opened by laterals
    /// of `limb_order` or less (the family's own where none): resolves to
    /// the grid. An id the catalogue does not serve, a side outside 3 to
    /// 256 and a grid past the device's limits reject with the core's or
    /// the device's error.
    pub fn grow(species: String, seed: u32, n: u32, limb_order: Option<u32>) -> js_sys::Promise {
        future_to_promise(async move {
            let mut family =
                telperion_core::presets::by_identity(&species).map_err(|e| js_error(e.into()))?;
            family.skeleton.seed = seed;
            let made = fields().await?;
            let grid = made
                .grow_field(&family, n, limb_order)
                .await
                .map_err(js_error)?;
            let mut answers = Vec::with_capacity(grid.cells.len() * 4);
            for a in &grid.cells {
                let flags = u32::from(a.wood) | u32::from(a.foliage) << 1;
                let limb = a.limb.unwrap_or(u32::MAX);
                let (radius, leaves) = (a.wood_radius as f32, a.leaves as f32);
                answers.extend([flags, radius.to_bits(), leaves.to_bits(), limb]);
            }
            let f = grid.frame;
            Ok(WebField {
                n: f.n,
                cell: f.cell,
                origin: [f.origin.x, f.origin.y, f.origin.z],
                answers,
            }
            .into())
        })
    }

    /// The grid's cells a side.
    pub fn n(&self) -> u32 {
        self.n
    }

    /// A cell's side, in metres.
    pub fn cell(&self) -> f64 {
        self.cell
    }

    /// The grid's least corner, in metres.
    pub fn origin(&self) -> Vec<f64> {
        self.origin.to_vec()
    }

    /// Four words a cell, in cell order: its flags (wood 1, foliage 2), its
    /// thickest wood radius and its leaves (each a float's bits) and its
    /// limb system (`u32::MAX` where none).
    pub fn answers(&self) -> js_sys::Uint32Array {
        js_sys::Uint32Array::from(self.answers.as_slice())
    }
}
