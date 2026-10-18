//! fn-257: a tree's field grown on a device of its own, for a page or a
//! worker with no canvas (`telperion/field`). The tree grows on the
//! device and its field stays there; a query crosses the binding as the
//! cells going in and their answers coming out. Without WebGPU, or on a
//! lost device, every call rejects by name.
use super::*;
use crate::generation::{DeviceField, Generator};
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

/// A shipped species' field at a seed, on the device it grew on until
/// released.
#[wasm_bindgen]
pub struct WebField {
    made: Rc<Generator>,
    field: Rc<RefCell<Option<DeviceField>>>,
    bounds: [f32; 6],
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

    /// `species` grown at `seed` on the device, its limb systems opened by
    /// laterals of `limb_order` or less (the family's own where none):
    /// resolves to the field. An id the catalogue does not serve rejects
    /// with the core's error.
    pub fn grow(species: String, seed: u32, limb_order: Option<u32>) -> js_sys::Promise {
        future_to_promise(async move {
            let mut family =
                telperion_core::presets::by_identity(&species).map_err(|e| js_error(e.into()))?;
            family.skeleton.seed = seed;
            let made = fields().await?;
            let field = made
                .grow_field(&family, limb_order)
                .await
                .map_err(js_error)?;
            let bounds = field.bounds;
            let field = Rc::new(RefCell::new(Some(field)));
            Ok(WebField {
                made,
                field,
                bounds,
            }
            .into())
        })
    }

    /// The field's box: its least corner, then its most, in metres.
    pub fn bounds(&self) -> Vec<f32> {
        self.bounds.to_vec()
    }

    /// The answers for `cells`, four numbers a cell (a closed cube's
    /// centre and half extent): resolves to four words a cell, its flags
    /// (wood 1, foliage 2), its thickest wood radius and its leaves (each
    /// a float's bits) and its limb system (`u32::MAX` where none).
    pub fn query(&self, cells: Vec<f32>) -> js_sys::Promise {
        let (made, field) = (self.made.clone(), self.field.clone());
        future_to_promise(async move {
            let held = field
                .try_borrow()
                .map_err(|_| JsError::new("the field is being released"))?;
            let field = held
                .as_ref()
                .ok_or_else(|| JsError::new("the field is released"))?;
            let answers = made.query_field(field, &cells).await.map_err(js_error)?;
            let mut words = Vec::with_capacity(answers.len() * 4);
            for a in answers {
                let flags = u32::from(a.wood) | u32::from(a.foliage) << 1;
                let limb = a.limb.unwrap_or(u32::MAX);
                words.extend([flags, a.wood_radius.to_bits(), a.leaves.to_bits(), limb]);
            }
            Ok(js_sys::Uint32Array::from(words.as_slice()).into())
        })
    }

    /// The field let go: its device memory freed, every later query
    /// refused. A field answering a query is refused by name.
    pub fn release(&self) -> std::result::Result<(), JsError> {
        let mut held = (self.field.try_borrow_mut())
            .map_err(|_| JsError::new("the field is answering a query"))?;
        held.take();
        Ok(())
    }
}
