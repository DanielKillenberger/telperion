//! A tree growing for the page (fn-270 R6, R7; fn-353 R1): the page begins
//! a family's growth, grows it on to an age and draws it at an age within
//! its last year, through `telperion_core::pipeline::growth`, the one
//! grower, on the page's main thread. The growth state crosses the binding
//! as a handle the page holds (`WebGrowth`); a draw dresses the tree as
//! `mesh::drawn` does and puts it up through `Renderer::submit`, the tree
//! drawn before staying on the canvas until it does.
use super::*;
use std::sync::Arc;
use telperion_core::pipeline::growth::{self, GrowthState, Params};

/// A family's tree growing, held by the page between calls.
#[wasm_bindgen]
pub struct WebGrowth {
    family: telperion_core::Family,
    state: GrowthState,
}

#[wasm_bindgen]
impl WebGrowth {
    /// The whole years grown.
    pub fn committed(&self) -> u32 {
        self.state.committed()
    }

    /// The most years it grows: the family's age.
    pub fn horizon(&self) -> u32 {
        self.state.horizon()
    }
}

#[wasm_bindgen]
impl WebRenderer {
    /// The family's tree before its first year, to grow to at most its
    /// age, as a handle the page holds (`WebGrowth`). Rejects by name for a
    /// family the core cannot grow; the tree drawn now stays.
    #[wasm_bindgen(js_name = beginGrowth)]
    pub fn begin_growth(&self, family: &str) -> std::result::Result<WebGrowth, JsError> {
        let family = parsed(family)?;
        let params = Arc::new(Params::of(&family).map_err(core_error)?);
        let state = growth::begin(&params, family.skeleton.seed, family.age).map_err(core_error)?;
        Ok(WebGrowth { family, state })
    }

    /// `growth` grown on to `age` years; the tree drawn now stays on the
    /// canvas. Returns the whole years grown. An age past the horizon, a
    /// year or more back, or not finite is refused by name before anything
    /// moves; a failure while it grows spends the tree, and every later
    /// call refuses it by name.
    #[wasm_bindgen(js_name = growTo)]
    pub fn grow_to(&self, growth: &mut WebGrowth, age: f64) -> std::result::Result<u32, JsError> {
        growth::grow_to(&mut growth.state, age).map_err(core_error)?;
        Ok(growth.state.committed())
    }

    /// `growth` drawn at `at`, within its last year grown, dressed and put
    /// on the canvas in place of the tree there, with its leaves coloured
    /// as far as `at` has turned them; returns what went up as `setTree`
    /// does. A draw refused by name leaves the tree on the canvas.
    #[wasm_bindgen(js_name = drawGrowth)]
    pub fn draw_growth(&self, growth: &WebGrowth, at: f64) -> std::result::Result<String, JsError> {
        let (state, family) = (&growth.state, &growth.family);
        let colouring = growth::colouring(state, at).map_err(core_error)?;
        let mesh = growth::drawn(state, at, family).map_err(core_error)?;
        let mut live = self.borrow()?;
        let submitted = live.renderer.submit(&mesh).map_err(js_error)?;
        live.renderer.set_family(family);
        live.renderer.set_colouring(colouring);
        live.redraw.ask();
        Ok(submitted_json(&submitted))
    }
}

/// The parameters as the page sent them, parsed.
pub(super) fn parsed(family: &str) -> std::result::Result<telperion_core::Family, JsError> {
    let value = serde_json::from_str(family)
        .map_err(|e| JsError::new(&format!("the parameters are not JSON: {e}")))?;
    params::parse(&value).map_err(|e| js_error(e.into()))
}

/// The core's refusal in the renderer's words.
fn core_error(error: telperion_core::Error) -> JsError {
    js_error(error.into())
}
