use super::*;
use crate::generation::{
    request::{Request, State},
    Admitted, Delivery, Generator, BROWSER,
};

pub(super) type Generation = Rc<RefCell<State>>;
/// Whether `family` fits the page's budget beside the drawn tree, only
/// once it is let go, or not at all.
fn admitted(
    live: &Rc<RefCell<Option<Live>>>,
    state: &Generation,
    family: &telperion_core::presets::Family,
) -> std::result::Result<Admitted, JsError> {
    let generator = state.borrow().generator.clone();
    let renderer = &borrow(live)?.renderer;
    Generator::admit(generator.as_deref(), family, renderer, BROWSER).map_err(js_error)
}

#[wasm_bindgen]
impl WebRenderer {
    /// What a build of these parameters would do under the page's budget,
    /// asked before it starts so the page can say so while it builds:
    /// `"beside"` the drawn tree, or `"makeRoom"`, the drawn tree let go
    /// first and the screen empty until it is built. A tree that does not
    /// fit even alone rejects by name (fn-235).
    pub fn admission(&self, family: &str) -> std::result::Result<String, JsError> {
        let value = serde_json::from_str(family)
            .map_err(|e| JsError::new(&format!("the parameters are not JSON: {e}")))?;
        let family = params::parse(&value).map_err(|e| js_error(e.into()))?;
        Ok(match admitted(&self.live, &self.generation, &family)? {
            Admitted::Beside => "beside",
            Admitted::MakeRoom => "makeRoom",
        }
        .into())
    }

    /// Compiles every GPU program a build dispatches, as soon as the
    /// device exists: the browser compiles them in the background, and a
    /// build waits on this rather than compiling again (fn-243 R3). A
    /// program the browser refuses, or a device lost meanwhile, rejects by
    /// name, and no build starts after it.
    #[wasm_bindgen(js_name = preparePrograms)]
    pub fn prepare_programs(&self) -> js_sys::Promise {
        let live = self.live.clone();
        let state = self.generation.clone();
        let begun = Request::begin(state.clone()).map_err(JsError::new);
        future_to_promise(async move {
            let request = begun?;
            if state.borrow().generator.is_some() {
                return Ok(JsValue::UNDEFINED);
            }
            let construction = Generator::new_async(&borrow(&live)?.renderer);
            let generator = construction.await.map_err(js_error)?;
            generator.prepare_programs().await.map_err(js_error)?;
            request.check().map_err(JsError::new)?;
            state.borrow_mut().generator = Some(Rc::new(generator));
            Ok(JsValue::UNDEFINED)
        })
    }

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
            // A build never compiles: it runs on the programs
            // `preparePrograms` compiled.
            let generator = state.borrow().generator.clone().ok_or_else(|| {
                JsError::new("the GPU programs are not prepared; await preparePrograms first")
            })?;
            // The budget's one rule: beside the drawn tree, the drawn tree
            // let go first, or refused by name with it kept.
            let made_room = admitted(&live, &state, &family)? == Admitted::MakeRoom;
            if made_room {
                borrow(&live)?.renderer.release_tree();
            }
            let prepared = generator
                .prepare_async(&family, Delivery::Resident)
                .await
                .map_err(js_error)?;
            request.check().map_err(JsError::new)?;
            let backend = format!("{:?}", prepared.backend);
            let metrics = &prepared.metrics;
            let stages = json!({"positionPrepareMs":metrics.position_prepare_ms,"positionUploadMs":metrics.position_upload_ms,"positionWaitMs":metrics.position_wait_ms,"positionCpuBytes":metrics.position_cpu_bytes,"positionGpuPeakBytes":metrics.position_gpu_peak_bytes,"positionRetainedMetadataBytes":metrics.position_retained_metadata_bytes,"positionFallback":metrics.position_fallback,"gpuPositions":metrics.gpu_positions,"skeletonMs":metrics.skeleton_ms,"descriptorsMs":metrics.descriptors_ms,
                "uploadDispatchMs":metrics.upload_dispatch_ms,"placementWaitMs":metrics.placement_wait_ms,
                "massMs":metrics.mass_ms,"readbackMs":metrics.readback_ms,
                "woodMs":metrics.wood_ms,"woodPrepareMs":metrics.wood_prepare_ms,"woodUploadDispatchMs":metrics.wood_upload_dispatch_ms,"woodWaitMs":metrics.wood_wait_ms,"woodPreparedCpuBytes":metrics.wood_prepared_cpu_bytes,"woodMetadataCpuBytes":metrics.wood_metadata_cpu_bytes,"woodGpuPeakBytes":metrics.wood_gpu_peak_bytes,"woodBackend":metrics.wood_backend.map(|b| format!("{b:?}")),"woodFallback":metrics.wood_fallback,"totalMs":metrics.total_ms,
                "baseCpuBytes":metrics.base_cpu_bytes,"woodCpuBytes":metrics.wood_cpu_bytes,
                "descriptorCpuBytes":metrics.descriptor_cpu_bytes,"sharedContactCpuBytes":metrics.shared_contact_cpu_bytes,"sharedPrepareCpuBytes":metrics.shared_prepare_cpu_bytes,"sharedMetadataCpuBytes":metrics.shared_metadata_cpu_bytes,"gpuComputePeakBytes":metrics.gpu_compute_peak_bytes,
                "retainedGpuBytes":metrics.retained_gpu_bytes});
            let mut live = borrow(&live)?;
            let previous_tree_gpu_bytes = Generator::tree_buffer_bytes(&live.renderer);
            let submitted = live.renderer.submit_prepared(prepared).map_err(js_error)?;
            live.renderer.set_family(&family);
            let mut value: serde_json::Value =
                serde_json::from_str(&submitted_json(&submitted)).expect("submitted JSON");
            value["backend"] = json!(backend);
            value["stages"] = stages;
            value["previousTreeGpuBytes"] = json!(previous_tree_gpu_bytes);
            value["madeRoom"] = json!(made_room);
            value["treeGpuBytes"] = json!(Generator::tree_buffer_bytes(&live.renderer));
            Ok(JsValue::from_str(&value.to_string()))
        })
    }
}
