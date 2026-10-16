# Vendored crates

## `wgpu-30.0.1-async`

wgpu 30.0.1 as published on crates.io, with one addition: a browser-only
`Device::create_compute_pipeline_async`. The workspace takes it through
`[patch.crates-io]` in the root `Cargo.toml`; nothing else in the copy is
changed, and natively it is the published crate.

**Why.** Chrome makes a synchronous `createComputePipeline` on its GPU
process's main thread. The chain grower's `grow` takes 13 to 17 s to
compile cold natively, and on a cold Chrome profile the GPU watchdog ended
the GPU process 19 s into the first build (fn-243 R1,
`.flow/evidence/fn-243-the-browser-compiles-the-gpu-programs/R1.md`).
`createComputePipelineAsync` compiles off that thread. wgpu 30.0.1 has
the binding (`webgpu_sys/gen_GpuDevice.rs`) but no public way to call it,
and no way to import a pipeline made outside it.

**Upstream.** gfx-rs/wgpu#3794, "Add `create_*_pipeline_async()`", open.
The patch is dropped when a wgpu release carries the method.

**What it adds.**

- `Device::create_compute_pipeline_async(&self, desc)` (`src/api/device.rs`,
  `#[cfg(webgpu)]`): asks the browser when called, not when first polled,
  and returns `Option<impl Future<Output = Result<ComputePipeline, String>>>`,
  `None` off the WebGPU backend. The error is the browser's
  `GPUPipelineError` as `"<reason>: <message>"`.
- `WebDevice::create_compute_pipeline_async` (`src/backend/webgpu.rs`):
  the descriptor mapping, moved out of `create_compute_pipeline` into
  `map_compute_pipeline_descriptor` so both creations share it, then the
  promise awaited and wrapped exactly as the synchronous pipeline is.

Telperion calls it from `crates/telperion-render/src/programs.rs`.

**Updating wgpu.** Copy the new release's crate source over this
directory, re-apply the diff below, and rename the directory and the
patch path to its version.

## The diff, against the crates.io source

```diff
diff --git i/vendor/wgpu-30.0.1-async/src/api/device.rs w/vendor/wgpu-30.0.1-async/src/api/device.rs
index 276b1c01..35fd223b 100644
--- i/vendor/wgpu-30.0.1-async/src/api/device.rs
+++ w/vendor/wgpu-30.0.1-async/src/api/device.rs
@@ -427,6 +427,23 @@ impl Device {
         self.inner.as_webgpu_opt().map(|wd| &wd.inner)
     }
 
+    /// Creates a [`ComputePipeline`] through the browser's
+    /// `createComputePipelineAsync`, which compiles it without holding the
+    /// browser's GPU process. The browser is asked when this is called, not
+    /// when the future is first polled. The error is the browser's rejection,
+    /// its reason and message. `None` off the WebGPU backend (telperion patch).
+    #[cfg(webgpu)]
+    pub fn create_compute_pipeline_async(
+        &self,
+        desc: &ComputePipelineDescriptor<'_>,
+    ) -> Option<impl Future<Output = Result<ComputePipeline, String>> + 'static> {
+        let pending = self
+            .inner
+            .as_webgpu_opt()?
+            .create_compute_pipeline_async(desc);
+        Some(async move { pending.await.map(|inner| ComputePipeline { inner }) })
+    }
+
     /// Creates a new [`ExternalTexture`].
     #[must_use]
     pub fn create_external_texture(
diff --git i/vendor/wgpu-30.0.1-async/src/backend/webgpu.rs w/vendor/wgpu-30.0.1-async/src/backend/webgpu.rs
index 870e98a4..f4024cdd 100644
--- i/vendor/wgpu-30.0.1-async/src/backend/webgpu.rs
+++ w/vendor/wgpu-30.0.1-async/src/backend/webgpu.rs
@@ -1882,6 +1882,76 @@ impl WebDevice {
     }
 }
 
+/// A compute pipeline's descriptor as the browser takes it, for its
+/// synchronous and its async creation alike (telperion patch).
+fn map_compute_pipeline_descriptor(
+    desc: &crate::ComputePipelineDescriptor<'_>,
+) -> webgpu_sys::GpuComputePipelineDescriptor {
+    let shader_module = desc.module.inner.as_webgpu();
+    let mapped_compute_stage = webgpu_sys::GpuProgrammableStage::new(&shader_module.module);
+    insert_constants_map(&mapped_compute_stage, desc.compilation_options.constants);
+    if let Some(ep) = desc.entry_point {
+        mapped_compute_stage.set_entry_point(ep);
+    }
+    let mapped_desc = match desc.layout {
+        Some(layout) => webgpu_sys::GpuComputePipelineDescriptor::new(
+            &layout.inner.as_webgpu().inner,
+            &mapped_compute_stage,
+        ),
+        None => webgpu_sys::GpuComputePipelineDescriptor::new_with_gpu_auto_layout_mode(
+            webgpu_sys::GpuAutoLayoutMode::Auto,
+            &mapped_compute_stage,
+        ),
+    };
+
+    if let Some(label) = desc.label {
+        mapped_desc.set_label(label);
+    }
+    mapped_desc
+}
+
+/// A rejected `createComputePipelineAsync` in the browser's own words: the
+/// `GPUPipelineError`'s reason and message (telperion patch).
+fn pipeline_rejection(error: wasm_bindgen::JsValue) -> String {
+    let field = |name: &str| {
+        js_sys::Reflect::get(&error, &wasm_bindgen::JsValue::from_str(name))
+            .ok()
+            .and_then(|value| value.as_string())
+    };
+    match (field("reason"), field("message")) {
+        (Some(reason), Some(message)) => format!("{reason}: {message}"),
+        (None, Some(message)) => message,
+        _ => format!("{error:?}"),
+    }
+}
+
+impl WebDevice {
+    /// `createComputePipelineAsync`, asked of the browser when this is
+    /// called; the future resolves with the pipeline or the browser's
+    /// rejection (telperion patch).
+    pub(crate) fn create_compute_pipeline_async(
+        &self,
+        desc: &crate::ComputePipelineDescriptor<'_>,
+    ) -> impl Future<Output = Result<dispatch::DispatchComputePipeline, String>> + 'static {
+        let mapped_desc = map_compute_pipeline_descriptor(desc);
+        let promise = self.inner.create_compute_pipeline_async(&mapped_desc);
+        MakeSendFuture::new(
+            wasm_bindgen_futures::JsFuture::from(promise),
+            |result: Result<webgpu_sys::GpuComputePipeline, wasm_bindgen::JsValue>| {
+                result
+                    .map(|inner| {
+                        WebComputePipeline {
+                            inner,
+                            ident: crate::cmp::Identifier::create(),
+                        }
+                        .into()
+                    })
+                    .map_err(pipeline_rejection)
+            },
+        )
+    }
+}
+
 impl dispatch::DeviceInterface for WebDevice {
     fn features(&self) -> crate::Features {
         map_wgt_features(self.inner.features())
@@ -2411,27 +2481,7 @@ impl dispatch::DeviceInterface for WebDevice {
         &self,
         desc: &crate::ComputePipelineDescriptor<'_>,
     ) -> dispatch::DispatchComputePipeline {
-        let shader_module = desc.module.inner.as_webgpu();
-        let mapped_compute_stage = webgpu_sys::GpuProgrammableStage::new(&shader_module.module);
-        insert_constants_map(&mapped_compute_stage, desc.compilation_options.constants);
-        if let Some(ep) = desc.entry_point {
-            mapped_compute_stage.set_entry_point(ep);
-        }
-        let mapped_desc = match desc.layout {
-            Some(layout) => webgpu_sys::GpuComputePipelineDescriptor::new(
-                &layout.inner.as_webgpu().inner,
-                &mapped_compute_stage,
-            ),
-            None => webgpu_sys::GpuComputePipelineDescriptor::new_with_gpu_auto_layout_mode(
-                webgpu_sys::GpuAutoLayoutMode::Auto,
-                &mapped_compute_stage,
-            ),
-        };
-
-        if let Some(label) = desc.label {
-            mapped_desc.set_label(label);
-        }
-
+        let mapped_desc = map_compute_pipeline_descriptor(desc);
         let compute_pipeline = self.inner.create_compute_pipeline(&mapped_desc);
 
         WebComputePipeline {
```
