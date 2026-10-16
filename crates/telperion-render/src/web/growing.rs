//! A tree growing on the page's device (fn-270 R6, R7): the page begins
//! a family's growth, grows it on to an age and draws it at an age within
//! its last year, the tree drawn before staying on the canvas while the
//! next stretch grows; a draw that fits only alone lets it go first, as a
//! build does (`Generator::draw_admission`). The growth state crosses the
//! binding as a handle the page holds (`WebGrowth`); the tree itself never
//! leaves the device.
use super::*;
use crate::generation::{request::Request, Admitted, Generator, GrowingTree, BROWSER};

/// A family's tree growing on the page's device, held by the page between
/// calls. One call at a time works on it; a second while the first is
/// out is refused by name.
#[wasm_bindgen]
pub struct WebGrowth {
    tree: Rc<RefCell<Option<GrowingTree>>>,
}

#[wasm_bindgen]
impl WebGrowth {
    /// The whole years grown, or none once a failure spent the tree or
    /// while a call is working on it.
    pub fn committed(&self) -> Option<u32> {
        self.tree.try_borrow().ok()?.as_ref()?.committed()
    }

    /// The most years it grows: the family's age.
    pub fn horizon(&self) -> Option<u32> {
        Some(self.tree.try_borrow().ok()?.as_ref()?.horizon())
    }
}

/// The growing tree taken out of `cell` for one call, put back as the
/// call ends whatever it returns.
async fn holding<T>(
    cell: &RefCell<Option<GrowingTree>>,
    work: impl AsyncFnOnce(&mut GrowingTree) -> std::result::Result<T, JsError>,
) -> std::result::Result<T, JsError> {
    let mut tree = cell.borrow_mut().take().ok_or_else(busy)?;
    let out = work(&mut tree).await;
    *cell.borrow_mut() = Some(tree);
    out
}

/// The parameters as the page sent them, parsed.
fn family_of(family: &str) -> std::result::Result<telperion_core::Family, JsError> {
    let value = serde_json::from_str(family)
        .map_err(|e| JsError::new(&format!("the parameters are not JSON: {e}")))?;
    params::parse(&value).map_err(|e| js_error(e.into()))
}

/// The prepared generator, or the reason there is none yet.
fn generator(
    state: &generation::Generation,
) -> std::result::Result<Rc<crate::generation::Generator>, JsError> {
    state.borrow().generator.clone().ok_or_else(|| {
        JsError::new("the GPU programs are not prepared; await preparePrograms first")
    })
}

/// What drawing the tree in `cell` does under the page's budget beside the
/// tree on the canvas: beside it, or it let go first; refused by name.
fn draw_admitted(
    live: &Rc<RefCell<Option<Live>>>,
    cell: &RefCell<Option<GrowingTree>>,
) -> std::result::Result<Admitted, JsError> {
    let held = cell.try_borrow().map_err(|_| busy())?;
    let tree = held.as_ref().ok_or_else(busy)?;
    Generator::draw_admission(tree, &borrow(live)?.renderer, BROWSER).map_err(js_error)
}

/// The refusal of a second call while one works on the tree.
fn busy() -> JsError {
    JsError::new("growth.busy: a call is already working on this tree")
}

#[wasm_bindgen]
impl WebRenderer {
    /// What the next draw of `growth` does under the page's budget, asked
    /// before it so the page can say so while it draws: `"beside"` the
    /// tree on the canvas, or `"makeRoom"`, that tree let go first and the
    /// screen empty until the draw is up. A draw that does not fit even
    /// alone rejects by name, and the tree on the canvas stays (fn-235's
    /// rule; host, 2026-10-14).
    #[wasm_bindgen(js_name = growthAdmission)]
    pub fn growth_admission(&self, growth: &WebGrowth) -> std::result::Result<String, JsError> {
        Ok(match draw_admitted(&self.live, &growth.tree)? {
            Admitted::Beside => "beside",
            Admitted::MakeRoom => "makeRoom",
        }
        .into())
    }

    /// The family's tree before its first year, to grow to at most its
    /// age, as a handle the page holds (`WebGrowth`). Rejects by name for
    /// a family the device cannot grow or whose least tables the page's
    /// budget cannot hold; the tree drawn now stays.
    #[wasm_bindgen(js_name = beginGrowth)]
    pub fn begin_growth(&self, family: &str) -> js_sys::Promise {
        let parsed = family_of(family);
        let state = self.generation.clone();
        future_to_promise(async move {
            let family = parsed?;
            let request = Request::begin(state.clone()).map_err(JsError::new)?;
            let tree = generator(&state)?
                .begin_growth(&family)
                .await
                .map_err(js_error)?;
            request.check().map_err(JsError::new)?;
            let tree = Rc::new(RefCell::new(Some(tree)));
            Ok(JsValue::from(WebGrowth { tree }))
        })
    }

    /// `growth` grown on to `age` years, asynchronously on the device; the
    /// tree drawn now stays on the canvas while it grows. Resolves to the
    /// whole years grown. An age past the horizon, a year or more back, or
    /// not finite rejects by name before anything moves; a failure while
    /// it grows spends the tree, and every later call rejects it by name.
    #[wasm_bindgen(js_name = growTo)]
    pub fn grow_to(&self, growth: &WebGrowth, age: f64) -> js_sys::Promise {
        let (state, cell) = (self.generation.clone(), growth.tree.clone());
        future_to_promise(async move {
            let request = Request::begin(state.clone()).map_err(JsError::new)?;
            let generator = generator(&state)?;
            let committed = holding(&cell, async |tree| {
                generator.grow_to(tree, age).await.map_err(js_error)?;
                Ok(tree.committed())
            })
            .await?;
            request.check().map_err(JsError::new)?;
            Ok(committed.map_or(JsValue::NULL, JsValue::from))
        })
    }

    /// `growth` drawn at `at`, within its last year grown, and put on the
    /// canvas in place of the tree there, returning what went up as
    /// `setTreeGpu` does, with whether room was made. Admitted as
    /// `growthAdmission` says: the tree on the canvas let go first where
    /// the draw fits only alone; a draw that fits not even alone rejects by
    /// name, and that tree stays.
    #[wasm_bindgen(js_name = drawGrowth)]
    pub fn draw_growth(&self, growth: &WebGrowth, at: f64) -> js_sys::Promise {
        let (state, cell, live) = (
            self.generation.clone(),
            growth.tree.clone(),
            self.live.clone(),
        );
        future_to_promise(async move {
            let request = Request::begin(state.clone()).map_err(JsError::new)?;
            let generator = generator(&state)?;
            // A draw refused by name is refused before the canvas is let go.
            let held = cell.try_borrow().map_err(|_| busy())?;
            generator
                .drawable(held.as_ref().ok_or_else(busy)?, at)
                .map_err(js_error)?;
            drop(held);
            let made_room = draw_admitted(&live, &cell)? == Admitted::MakeRoom;
            if made_room {
                let mut canvas = borrow(&live)?;
                canvas.renderer.release_tree();
                canvas.redraw.ask();
            }
            let (prepared, family) = holding(&cell, async |tree| {
                let prepared = generator.settle(tree, at).await.map_err(js_error)?;
                Ok((prepared, tree.family().clone()))
            })
            .await?;
            request.check().map_err(JsError::new)?;
            let mut live = borrow(&live)?;
            let submitted = live.renderer.submit_prepared(prepared).map_err(js_error)?;
            live.renderer.set_family(&family);
            live.redraw.ask();
            let mut value: serde_json::Value =
                serde_json::from_str(&submitted_json(&submitted)).expect("submitted JSON");
            value["madeRoom"] = serde_json::json!(made_room);
            Ok(JsValue::from_str(&value.to_string()))
        })
    }
}
