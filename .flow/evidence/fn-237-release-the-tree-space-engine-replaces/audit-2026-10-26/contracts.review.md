I’ll trace the release diff, the field API, and the local site consumer, focusing on contract mismatches and avoidable work. I’ll use read-only inspection and lightweight checks.

The site now uses one persistent worker, and its existing notes report GPU memory exhaustion with several workers. I also found a field contract mismatch: flowering trees read every floret back to the CPU, despite the documented “only the grid” readback promise. I’m tracing that path and checking the public entries for similar mismatches.

The retained timings are from earlier commits: fn-257 reports native warm costs, and fn-249 measures a CPU Wasm field path that has since been removed. Neither establishes browser `growField` performance at b2261e472. I also confirmed that field startup compiles wood-surface and leaf-mass programs the field path never dispatches.

I found a bounded validation defect: `growField` does not enforce `canopy.maxInstances`, while the CPU field does. It also bypasses canopy range checks such as negative `scatter`, contradicting fn-317’s validation-parity requirement. The direct TypeScript check failed against generated renderer bindings from before fn-317, so I’m treating that as a stale local artifact, not a source finding.

Three findings at b2261e472. The bounded release fix is validation parity; the larger readback change needs the host’s explicit deferral if it moves past 0.2.0.

1. **Severity:** P2
   **Confidence:** 75
   **Classification:** introduced
   **File:Line:** `crates/telperion-render/src/generation/resident/field/make.rs:95`
   **R-IDs:** [fn-317:R1]
   **Problem:** The device field bypasses validation the CPU field enforces. For `presetById("ordinary")` with `canopy.maxInstances = 1`, the CPU plan rejects with `"foliage instance budget"`, while the device field never checks that limit. It also bypasses canopy range validation—for example, `canopy.scatter = -1`. Sharing `params::parse` does not provide parity: the CPU checks these later in `canopy::validate` and `plan::runs`; the device’s `Stationer` neither receives nor checks `maxInstances`.
   **Suggestion:** Before 0.2.0, reuse the applicable canopy checks and enforce the counted station budget before returning the field. The renderer already has the bounded `build::instances` check. Cover these two inputs against both entries.

2. **Severity:** P2
   **Confidence:** 100
   **Classification:** introduced
   **File:Line:** `crates/telperion-render/src/generation/resident.rs:51`
   **R-IDs:** []
   **Problem:** A field-only visitor compiles rendering programs it never uses. `WebField` creates a generator through `for_fields_async` and calls `prepare_programs`, which executes:
   ```rust
   super::io::pair(gpu, || Kit::new(gpu), || drop(crate::curve::passes(gpu))).0
   ```
   This compiles eleven wood-surfacing pipelines. The common constructor also creates leaf-mass and position programs, and `Tools::new` creates sweep/ring programs, although the field consumes drawn records without those stages. This is unnecessary startup work and **Speculative Generality** in the shared initialization path.
   **Suggestion:** Remove these unused program constructions from field initialization, retaining them for renderer initialization. This is a bounded simplification candidate before release; its browser timing benefit remains unmeasured.

3. **Severity:** P2
   **Confidence:** 100
   **Classification:** introduced
   **File:Line:** `crates/telperion-render/src/generation/resident/florets.rs:146`
   **R-IDs:** [fn-257:R8]
   **Problem:** Flowering fields violate the “only the grid is read back” contract. `Organ::read` downloads every packed floret, copies the words into CPU `Instances`, and `field/make.rs:234` counts them into cells on the CPU. Using the retained Laurelin seed-1 count, this downloads **7,904,880 bytes of floret records** before the grid answer; that number is calculated from existing evidence, not a new measurement. The public docs still promise grid-only readback.
   **Suggestion:** Remove the per-floret CPU handoff by reducing seat bounds and cell counts on the device. This is the larger post-release optimization candidate: the retained fn-316 measurements show only about **+1% native warm time** for Laurelin. Deferring it requires the host to explicitly defer the R8 constraint; it is currently documented as fulfilled.

The existing performance evidence does **not** establish browser `growField` performance at this revision. The accepted fn-257 result places spruce around 432–440 ms versus roughly 240 ms for 0.1.4; the owner accepted that gap. Fn-249 measures a subsequently removed CPU Wasm path. The local site now pins `af1d320`, uses one worker, and records memory exhaustion with multiple devices. None of those observations is a new b2261e472 benchmark.

Lightweight verification: `tsc --noEmit` failed against an ignored, stale four-argument `WebField.grow` declaration predating fn-317. That artifact mismatch is excluded from the findings. No builds, suites, browser captures, or timing runs were performed.

## Requirements coverage

Coverage here concerns the field/API contracts traced in this audit.

| R-ID | Status | Evidence |
|---|---|---|
| fn-317:R1 | partial | Parameters and seed propagate; canopy validation and instance-budget parity fail as described above. |
| fn-317:R2 | met | `TreeEngine.build` takes `Family`; `wire` and Rust parsing reject identity strings. |
| fn-317:R3 | met | Separate presets entry and package export; main entry re-exports it. |
| fn-317:R4 | met | Exported `FieldError` and both stable codes. |
| fn-317:R4b | met | Coded Wasm errors are mapped; adapter-conditioned retry is implemented and mocked coverage exists. |
| fn-317:R4c | met | Both bindings call `Grid::name_limbs`; amended tolerances and voxelizer remainder are recorded. |
| fn-317:R5 | met | Parameter migration, CPU field alternative, presets entry and error codes are documented. |
| fn-317:R6 | partial | Preset-equivalence and binding tests exist; they do not cover the validation-parity failure. Historical gates were not rerun. |
| fn-257:R8 | partial | Direct record-to-grid field exists; flowering trees additionally read back every floret. |

Unaddressed R-IDs: [fn-317:R1, fn-257:R8]

Classification counts: 3 introduced, 0 pre_existing.

The verdict follows the unmet stated requirements; the optional startup simplification alone would not block shipping.

```json
{"classification_counts":{"introduced":3,"pre_existing":0},"unaddressed":["fn-317:R1","fn-257:R8"]}
```

<verdict>NEEDS_WORK</verdict>