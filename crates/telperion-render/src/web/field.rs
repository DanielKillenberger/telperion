//! fn-257: a tree's field read out on a device of its own, for a page or a
//! worker with no canvas (`telperion/field`). The tree grows on the
//! device and its field is read out there to the grid the call names;
//! only the grid crosses the binding, and nothing stays on the device.
//! Without WebGPU, or on a lost device, every call rejects by name, its
//! error carrying a `code` the entry reads (fn-317 R4).
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

/// `error` as the field rejects with it: its message, and a `code` where
/// it is one a caller acts on, no WebGPU or a lost device.
fn refused(error: RenderError) -> JsValue {
    let code = match &error {
        RenderError::WebGpuUnavailable(_) => Some("webgpu-unavailable"),
        RenderError::DeviceLost { .. } => Some("device-lost"),
        _ => None,
    };
    let e = js_sys::Error::new(&error.to_string());
    if let Some(code) = code {
        // Setting a property on a fresh Error cannot fail.
        let _ = js_sys::Reflect::set(&e, &"code".into(), &code.into());
    }
    e.into()
}

/// The instance's generator, its device requested and its programs
/// compiled once, however many calls ask at once. A lost device is
/// refused by name and let go, so the next call asks for another.
async fn fields() -> std::result::Result<Rc<Generator>, JsValue> {
    if let Some(made) = FIELDS.with(|f| f.borrow().clone()) {
        if let Some(lost) = made.lost() {
            FIELDS.with(|f| f.borrow_mut().take());
            return Err(refused(lost));
        }
        return Ok(made);
    }
    let making = MAKING.with(|m| {
        m.borrow_mut()
            .get_or_insert_with(|| {
                future_to_promise(async {
                    let gpu = Gpu::request(None).await.map_err(refused)?;
                    let made = Generator::for_fields_async(gpu).await.map_err(refused)?;
                    made.prepare_programs().await.map_err(refused)?;
                    FIELDS.with(|f| *f.borrow_mut() = Some(Rc::new(made)));
                    Ok(JsValue::UNDEFINED)
                })
            })
            .clone()
    });
    let made = JsFuture::from(making).await;
    MAKING.with(|m| m.borrow_mut().take());
    made?;
    FIELDS
        .with(|f| f.borrow().clone())
        .ok_or_else(|| JsError::new("the device for fields was not made").into())
}

/// A tree's field, read out to a grid
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

    /// The tree `family` describes (its parameters as JSON, read as the
    /// CPU entry reads them) grown on the device and its field read out to
    /// `n` cells a side over its box, its limb systems opened by laterals
    /// of `limb_order` or less (the family's own where none): resolves to
    /// the grid. Parameters the core refuses, a side outside 3 to 256 and
    /// a grid past the device's limits reject with the core's or the
    /// device's error.
    pub fn grow(family: String, n: u32, limb_order: Option<u32>) -> js_sys::Promise {
        future_to_promise(async move {
            let value = serde_json::from_str(&family)
                .map_err(|e| JsError::new(&format!("the parameters are not JSON: {e}")))?;
            let family = params::parse(&value).map_err(|e| js_error(e.into()))?;
            telperion_core::field::Frame::side(n).map_err(|e| js_error(e.into()))?;
            let made = fields().await?;
            let grid = made
                .grow_field(&family, n, limb_order)
                .await
                .map_err(refused)?;
            let mut answers = Vec::with_capacity(grid.cells.len() * 5);
            for a in &grid.cells {
                let flowered = u32::from(a.flowers > 0.0) << 2;
                let flags = u32::from(a.wood) | u32::from(a.foliage) << 1 | flowered;
                let limb = a.limb.unwrap_or(u32::MAX);
                let (radius, leaves) = (a.wood_radius as f32, a.leaves as f32);
                let flowers = (a.flowers as f32).to_bits();
                answers.extend([flags, radius.to_bits(), leaves.to_bits(), limb, flowers]);
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

    /// Five words a cell, in cell order: its flags (wood 1, foliage 2,
    /// flowers 4), its thickest wood radius and its leaves (each a float's
    /// bits), its limb system (`u32::MAX` where none) and its florets (a
    /// float's bits; fn-316).
    pub fn answers(&self) -> js_sys::Uint32Array {
        js_sys::Uint32Array::from(self.answers.as_slice())
    }
}
