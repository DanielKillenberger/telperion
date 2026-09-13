//! Timeline operations use the same native specimen presentation as other hosts.
use super::*;
use telperion_core::specimen::SpecimenView;

#[wasm_bindgen]
impl WebRenderer {
    #[wasm_bindgen(js_name = buildSpecimen)]
    pub fn build_specimen(&self, family: &str, age: f64) -> std::result::Result<String, JsError> {
        let value = serde_json::from_str(family).map_err(|e| JsError::new(&e.to_string()))?;
        let mut family = params::parse(&value).map_err(|e| js_error(e.into()))?;
        family.age = age;
        let view = SpecimenView::build(&family).map_err(|e| js_error(e.into()))?;
        let mesh = view.mesh().map_err(|e| js_error(e.into()))?;
        let mut live = self.borrow()?;
        let submitted = live.renderer.submit(&mesh).map_err(js_error)?;
        live.renderer.set_material(view.material());
        let result = growth_json(&view, &submitted);
        live.growth = Some(view);
        live.growth_submitted = Some(submitted);
        Ok(result)
    }
    #[wasm_bindgen(js_name = seekSpecimen)]
    pub fn seek_specimen(&self, age: f64) -> std::result::Result<String, JsError> {
        let mut live = self.borrow()?;
        let view = live
            .growth
            .as_mut()
            .ok_or_else(|| JsError::new("invalid specimen handle"))?;
        let previous = view.age();
        view.seek(age).map_err(|e| js_error(e.into()))?;
        // A fractional remainder with no yearly slice changes no geometry.
        let changed = previous.floor() != view.age().floor();
        if changed || live.growth_submitted.is_none() {
            live.growth_submitted = None;
            let mesh = live
                .growth
                .as_ref()
                .unwrap()
                .mesh()
                .map_err(|e| js_error(e.into()))?;
            live.growth_submitted = Some(live.renderer.submit(&mesh).map_err(js_error)?);
        }
        Ok(growth_json(
            live.growth.as_ref().unwrap(),
            &live.growth_submitted.unwrap(),
        ))
    }
}
fn growth_json(view: &SpecimenView, submitted: &Submitted) -> String {
    let mut value: serde_json::Value = serde_json::from_str(&submitted_json(submitted)).unwrap();
    value["age"] = json!(view.age());
    value["frontier"] = json!(view.frontier());
    value.to_string()
}
