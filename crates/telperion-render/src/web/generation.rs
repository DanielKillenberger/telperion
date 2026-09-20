use super::*;
use crate::generation::{
    request::{Request, State},
    Delivery, Generator,
};

pub(super) type Generation = Rc<RefCell<State>>;
#[wasm_bindgen]
impl WebRenderer {
    /// Experimental GPU foliage preparation. Unsupported parameters explicitly
    /// use the CPU fallback; the existing synchronous entry remains available.
    #[wasm_bindgen(js_name = setTreeGpu)]
    pub fn set_tree_gpu(&self, family: &str) -> js_sys::Promise {
        let parsed = serde_json::from_str(family)
            .map_err(|e| JsError::new(&format!("the parameters are not JSON: {e}")))
            .and_then(|value| params::parse(&value).map_err(|e| js_error(e.into())));
        let live = self.live.clone();
        let state = self.generation.clone();
        let begun = parsed.and_then(|family| {
            Request::begin(state.clone())
                .map(|request| (family, request))
                .map_err(JsError::new)
        });
        future_to_promise(async move {
            let (family, request) = begun?;
            request.check().map_err(JsError::new)?;
            let cached = state.borrow().generator.clone();
            let generator = if let Some(generator) = cached {
                generator
            } else {
                let construction = {
                    let live = borrow(&live)?;
                    Generator::new_async(&live.renderer)
                };
                let generator = Rc::new(construction.await.map_err(js_error)?);
                request.check().map_err(JsError::new)?;
                state.borrow_mut().generator = Some(generator.clone());
                generator
            };
            let prepared = generator
                .prepare_async(&family, Delivery::Resident)
                .await
                .map_err(js_error)?;
            request.check().map_err(JsError::new)?;
            let backend = format!("{:?}", prepared.backend);
            let metrics = &prepared.metrics;
            let stages = json!({"skeletonMs":metrics.skeleton_ms,"descriptorsMs":metrics.descriptors_ms,
                "uploadDispatchMs":metrics.upload_dispatch_ms,"placementWaitMs":metrics.placement_wait_ms,
                "compactMs":metrics.compact_ms,"massMs":metrics.mass_ms,"readbackMs":metrics.readback_ms,
                "woodMs":metrics.wood_ms,"woodPrepareMs":metrics.wood_prepare_ms,"woodUploadDispatchMs":metrics.wood_upload_dispatch_ms,"woodWaitMs":metrics.wood_wait_ms,"woodPreparedCpuBytes":metrics.wood_prepared_cpu_bytes,"woodMetadataCpuBytes":metrics.wood_metadata_cpu_bytes,"woodGpuPeakBytes":metrics.wood_gpu_peak_bytes,"woodBackend":metrics.wood_backend.map(|b| format!("{b:?}")),"woodFallback":metrics.wood_fallback,"totalMs":metrics.total_ms,
                "baseCpuBytes":metrics.base_cpu_bytes,"woodCpuBytes":metrics.wood_cpu_bytes,
                "descriptorCpuBytes":metrics.descriptor_cpu_bytes,"sharedContactCpuBytes":metrics.shared_contact_cpu_bytes,"sharedPrepareCpuBytes":metrics.shared_prepare_cpu_bytes,"sharedMetadataCpuBytes":metrics.shared_metadata_cpu_bytes,"gpuComputePeakBytes":metrics.gpu_compute_peak_bytes,
                "retainedGpuBytes":metrics.retained_gpu_bytes});
            let mut live = borrow(&live)?;
            let previous_tree_gpu_bytes = Generator::tree_buffer_bytes(&live.renderer);
            let submitted = live.renderer.submit_prepared(prepared).map_err(js_error)?;
            live.renderer.set_material(family.material);
            let mut value: serde_json::Value =
                serde_json::from_str(&submitted_json(&submitted)).expect("submitted JSON");
            value["backend"] = json!(backend);
            value["stages"] = stages;
            value["previousTreeGpuBytes"] = json!(previous_tree_gpu_bytes);
            value["treeGpuBytes"] = json!(Generator::tree_buffer_bytes(&live.renderer));
            Ok(JsValue::from_str(&value.to_string()))
        })
    }
}
