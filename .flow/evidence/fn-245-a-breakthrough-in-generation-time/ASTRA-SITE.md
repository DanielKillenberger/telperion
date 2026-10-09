**The release-sized changes should largely fix warm-cache birch and bring spruce near 0.1.4. They will probably leave oak around 0.7–1.1 seconds. I would not promise the owner’s speed target from these changes alone.**

Savings below are engineering estimates from the supplied profile, not measured improvements. Ranked by expected gain versus risk; paths are under `crates/telperion-space/src/`.

| Rank | Concrete change | Estimated saving per tree | Risk / release fit |
|---|---|---:|---|
| **1** | **Cache immutable closed-form `Rows` across grows.** `fine/coarse.rs:82` rebuilds them; `closed_form/boundary.rs:76` constructs them from species and age. `Kept` already memoizes within one construction (`closed_form/kept.rs:332`). Retain `Arc<Rows>` inside the worker’s Wasm session, keyed by the **complete species parameters, age and row mode**, with bounded retention. | Birch **630–650 ms**; oak **45–55 ms**; spruce **45–60 ms**, on cache hits. | **Low; ship.** Byte-identical. First request for each key still pays the construction cost; each worker needs its own cache unless explicitly shared. |
| **2** | **Replace bitwise division with exact word-wise division.** `allocation.rs:40` reaches `fixed/feedback/wide.rs:102`, which always walks 256 bits. Use base-2³² limbs, normalization and quotient correction, retaining signed truncation, zero errors and narrowing behavior. Zero numerators can return immediately. | Oak **150–230 ms**. No substantial birch/spruce saving established. | **Medium; ship with differential tests.** Algorithm D fits a few days including boundary tests. Preserve Contract A exactly; do not round intermediate products. |
| **3** | **Remove unused placeholder walk state, then eliminate duplicate framing where consumers permit.** `geometry/place.rs:110` prepares every axis; even an empty axis constructs a `Layer` and draws tropism (`geometry/lay.rs:398`). Empty, childless placeholders can use a frame-only operation while retaining validation and observable fields. | Tentatively oak **70–160 ms**, birch **20–70 ms**, spruce **50–110 ms**. | **Medium; release candidate.** Placeholder count is not time share: these axes already execute zero phytomer steps. The proposed savings need an isolated measurement. |
| **4** | **Remove allocation overhead from exact lattice deposits.** `fixed/feedback/lattice.rs:132` allocates two vectors for every corner calculation, including every deposit and sample. Replace them with fixed arrays; reuse gathering buffers in `grow/stepped/feedback.rs:81`. | Oak **10–30 ms**, low confidence. | **Low; ship if Wasm measurement confirms benefit.** Same deposits and sweep, hence identical output. |
| **5** | **Optimize wide arithmetic without reducing precision.** Q32 values already occupy i64; `fixed.rs:472` needs a wide product before flooring and saturation. Use exact limb implementations or narrower operations only where bounds prove equivalence. | **Unquantified.** The stage’s 3× native/Wasm ratio does not establish multiplication’s share. | A single measured primitive may fit; **a general narrowing project does not**. Even Q32 `1 × 1` needs a product of 2⁶⁴. |

**The placeholder passes cannot simply become one whole-tree pass.** The unbent layout supplies root mass moments (`fine/coarse/expected.rs:245`); those feed levers before the bent layout (`fine/lay.rs:60`, `:67`, `:75`). However, expected roots reconstruct their frames from their **bearers**, and final stands do so again (`fine/coarse.rs:135`). That makes duplicate placeholder framing a credible deletion target. Preserve parent geometry, supplied radii/loads, error behavior and any exported placeholder fields.

**Defer changing oak’s light model or cycle frequency.** Its rough layout already resumes from the previous endpoint (`grow/draw.rs:118`), rather than redrawing all existing wood. The seven-point stencils (`grow/stepped/stands.rs:187`) feed light back into later growth. Replacing them with centroids or skipping cycles can change topology. The measured **125–175 ms layout** and **122–167 ms shade** are cost ceilings, not removable savings. Exact deposit aggregation is possible, but must preserve each leaf’s rounding before summing (`fixed/feedback/lattice.rs:69`).

Assuming ranks 1–4 achieve their estimated savings, including the tentative placeholder improvement:

| Species | Warm-cache end-to-end | First tree per cache key | 0.1.4 |
|---|---:|---:|---:|
| Birch | **0.15–0.36 s** | **0.80–1.00 s** | 0.35–0.36 s |
| Oak | **0.69–1.07 s** | **0.74–1.12 s** | 0.28–0.32 s |
| Spruce | **0.20–0.29 s** | **0.26–0.34 s** | 0.24 s |

These estimates apply to the profiled **36³ grid**, not yet the full 24–48 range. Cache savings amortize over repeated species/age requests; they do not fix cold birch latency. Oak can still exceed **3×** its old time.

Following `docs/principles.md:32`, first measure the placeholder deletion, then optimize the remaining work. Keep this release set byte-identical, verify CPU/device arithmetic equivalence, and remeasure cold and warm site requests. **The set is worthwhile, but insufficient evidence to approve the release against the owner’s requirement.**