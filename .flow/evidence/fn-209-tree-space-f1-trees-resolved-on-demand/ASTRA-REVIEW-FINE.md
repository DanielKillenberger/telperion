**Verdict: accept with changes.** Moving fine growth into expansion is a sound direction and can preserve one pipeline. The draft does not yet establish topology equality, a sufficient boundary contract, or 150–300 ms retuning. I would authorize a focused probe after tightening those contracts.

I reviewed the supplied design, evidence, prior reviews, specs and relevant code. Timings below are reported measurements or explicitly labelled extrapolations; I did not rerun benchmarks or judge rendered images. No files were modified.

**1. It can keep one pipeline, but the split must change execution without creating a second botanical algorithm.**

The generator should own one expansion contract, executed by Rust or WGSL. The renderer schedules it and consumes its outputs. The CPU reference must expand the same fine-root records using the same rules, including for an exhaustive build.

That fits fn-125’s architecture. It extends its explicit boundary that growth stays on the CPU, so the dependency and contract need updating. It also requires integrating the tree-space engine into the pipeline. Growing externally and feeding the result through the current test-oriented `executor::expand` entrance would leave the integration problem identified in [GPU-PATH.md](/home/daniel/Projects/telperion/.worktrees/baseline/.flow/evidence/fn-209-tree-space-f1-trees-resolved-on-demand/GPU-PATH.md) unresolved.

There is a more consequential ambiguity. **Is N merely a work partition, or does it select different physiology and mechanics?** If crossing N changes realized descendant loading into expected loading, or individual light histories into one root history, changing the cut changes the tree. Camera independence alone does not solve that discontinuity.

Prefer making work-item boundaries independent of physiological approximation boundaries. If they must coincide, carry forward DESIGN-LIGHT’s complementary accounting and continuity requirements.

**2. Integer-keyed decisions are necessary; the determinism plan is not yet sufficient.**

An identical random draw can produce different outcomes when its threshold differs. The current code contains precisely those dependencies:

- [lineage.rs](/home/daniel/Projects/telperion/.worktrees/baseline/crates/telperion-space/src/lineage.rs) uses a 64-bit hash, 53-bit draws, floating cumulative probabilities and a Poisson recurrence.
- [advance.rs](/home/daniel/Projects/telperion/.worktrees/baseline/crates/telperion-space/src/grow/advance.rs) derives survival and abortion thresholds from powers and fractional exposure.
- [schedule.rs](/home/daniel/Projects/telperion/.worktrees/baseline/crates/telperion-space/src/schedule.rs) makes cycle-transition decisions with floating arithmetic and a `1e-12` boundary.
- P2’s shedding multiplies extinction probabilities and computes a conditional survival probability before comparing its draw.

“Exactly representable arithmetic” needs a specified representation, rounding, overflow behaviour and evaluation order. Quantizing the final result of two independently evaluated floating expressions still permits disagreement at quantization boundaries.

I would require:

- A portable key and integer-draw definition.
- Canonical probability thresholds and fractional-time decisions, either supplied identically or computed with specified deterministic arithmetic.
- Stable lineage identities and allocation-independent ordering.
- An audit of **every continuous-to-discrete dependency**, including geometry filtering, contact selection and culling.

That last item extends beyond botanical topology. The converter merges or omits wood using radius, distance and direction tests. Two geometries within 50 µm can still fall on opposite sides of such a test.

Maintain separate checks for identical botanical topology, identical output counts/identities, geometric tolerance, and attachment correctness. Fn-12 demonstrated that small numerical differences can violate contact semantics; it did not demonstrate that GPU growth itself is untenable.

**3. The boundary approach is defensible, but the record listed in the draft is incomplete.**

| Effect | What changes against whole-tree growth | Required contract |
|---|---|---|
| Sag | The bearer responds to committed descendant loading instead of the realized subtree. Mass, radius and lever-arm correlations are lost. | Committed mass **and first moment**, their frame, the finalized attachment frame, and explicit ground-support handling. |
| Shedding | Extinction-based decisions approximate correlated descendant survival. P2’s reported beech deficit remains a botanical difference. | Ancestor survival and termination history, conditional shedding context, dormant/relay history and retained-pipe transfer. |
| Light | Internal gradients and self-shading disappear when every descendant reads its root’s history. | History-dependent aggregate state and fidelity tests on the **new count-selected subtrees**. |
| Allocation | Aggregate demand cannot generally reproduce internal branching competition or nonlinear PA normalization. | Incoming vigour, internal subdivision rules, PA moments/normalizers and the accepted under-2% per-PA drift check. |
| Junction girth | A fitted committed base preserves the interface while relaxing realized pipe accounting. | Immutable attachment radius/section, exponent interpretation, retained/live partition, and a local fit that does not rescale descendants. |

Two code findings matter here.

First, [P2’s `Bud`](/home/daniel/Projects/telperion/.worktrees/probe-alone/crates/telperion-space/src/grow/alone.rs) carries birth draws, a chain root, horizon and waking state beyond lineage, PA and birth cycle. The serialized boundary must preserve the information these represent.

Second, option 4’s mechanics already include borne buds’ committed mass and moment. The draft’s “own load only” description should distinguish a complete fine subtree’s internal loads from the committed load its CPU bearer reads. [The implementation](/home/daniel/Projects/telperion/.worktrees/probe-alone/crates/telperion-space/src/bounded/work.rs) provides a more precise starting point.

Also specify whether internal chains retain option 4’s committed interfaces or use realized whole-subtree mechanics. Those produce different trees and different sensitivity to moving the cut.

**The light evidence does not transfer automatically.** A count below 1,024 establishes no spatial extent bound. The earlier single-read result at 0.5–1 m cannot establish its adequacy for this partition. DESIGN-LIGHT’s accepted history-dependent aggregates must remain part of the design.

**4. Count is a better scheduling coordinate than reach, but expected kept count is neither a work bound nor an allocation bound.**

[SPLIT.md](/home/daniel/Projects/telperion/.worktrees/baseline/.flow/evidence/fn-209-tree-space-f1-trees-resolved-on-demand/SPLIT.md) makes a persuasive case for testing N = 1,024. Reach produces heavy-tailed work; the count cut leaves a small explicit structure across all three species.

However:

- The measured cut knows the realized surviving subtree.
- The proposed predictor knows an expectation before growth.
- Growth processes organs that later die or disappear.
- SPLIT excludes empty branches, which the prospective classifier may still need to process.
- P2’s 35–78 µs measures a **chain**, whereas one proposed GPU item contains a **subtree of chains**.

Use expected kept count to choose a partition, then separately measure generated organs, chain count, cycles, temporary storage and emitted geometry. Define bounded execution and overflow handling. Splitting an oversized item into further work must preserve its botanical result; silently dropping work cannot pass.

Keep a separate spatial criterion for light approximation and conservative bounds for visibility. One scalar need not answer all three questions.

**5. The target is plausible for investigation, but unsupported for all three species. Oak is the strongest objection.**

A deliberately optimistic calculation illustrates the remaining budget. Multiply GPU-PATH’s age-80 growth time by SPLIT’s remaining node share, then add its estimated coarse preparation:

| Species | Hypothetical coarse growth | Coarse preparation | Subtotal before fine expansion and drawing |
|---|---:|---:|---:|
| Beech | ~38 ms | ~55 ms | **~93 ms** |
| Spruce | ~36 ms | ~36 ms | **~72 ms** |
| Oak | ~229 ms | ~25 ms | **~254 ms** |

**These are not predictions.** Final surviving-node share is a poor estimator of historical growth work, especially for the oak’s repeated physiology, light grids and layouts. The calculation merely shows that even an optimistic proportional model leaves oak little room inside 300 ms.

The remaining time goes into:

- Predictor/boundary preparation invalidated by a retune.
- Fine-root aggregate updates and history recording.
- CPU structure growth, settling and preparation.
- GPU topology, shedding, layout, girth, rings, stations and curves.
- Buffer sizing, scans, ordering, transfer and synchronization.
- Selection, shadows, surfacing and the completed frame.

Moving growth alone is insufficient. **Fine rings, stations and curves must also stay off the CPU**, or the measured per-node preparation cost returns through readback.

Spruce has abundant parallel work, but also about 73,000 roots, 4.64M nodes and 16.9M needles. Its measured steady frame is already 58 ms. GPU generation does not remove that draw cost. Fn-176 may help at suitable views, but its current design starts from ordered generated data and cannot be credited with avoiding fine growth without further work.

The draft also overgeneralizes the shipped comparison. The 153–174 ms measurements are oak and spruce; shipped beech takes about 3.66 seconds through its fallback.

My assessment is **beech plausible, spruce plausible but output-heavy, oak unproved and likely dependent on a successful coarse physiology implementation**. None has a demonstrated end-to-end budget yet.

**6. I would change the design before the first probe in four ways.**

1. Define the canonical fine-root record and numeric decision contract.
2. Separate work partitioning from physiological approximation, or explicitly specify and test their coupled transition.
3. State that fine preparation remains device-resident through curves, foliage and drawing.
4. Replace the inferred timing claim with an end-to-end feasibility gate that includes invalidated tables, aggregates and histories.

The first probe should have two successive parts.

**First, prove the contract on small adversarial examples.** Exercise fractional lifespans, threshold-adjacent probabilities, relay and dormant buds, ancestor shedding, changing light, retained pipes and a loaded junction. Compare canonical topology by lineage on CPU and GPU, then compare geometry and contacts. Repeat with different item order, chunking and refinement order. Test partition changes with fixed botanical inputs.

**Then test representative throughput.** Use beech, spruce and lit oak at ages 40 and 80, seeds 1 and 7. Classify prospectively, without growing the reference first to discover the cut. Report predicted versus realized sizes, rejected/empty roots, total historical work, worst-item latency, memory and every stage through a completed frame. Include growth-parameter edits that invalidate preparation.

Keep three comparisons distinct:

- CPU versus GPU under the new contract.
- The new contract versus whole-tree growth for botanical fidelity.
- Exhaustive versus on-demand execution for resolution independence.

Passing the first does not establish the other two. That separation is what would make this probe decisive.