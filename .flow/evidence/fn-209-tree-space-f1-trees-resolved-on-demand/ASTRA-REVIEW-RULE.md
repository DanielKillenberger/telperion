**Verdict: reject as written.** Keep chain-by-chain resolution and investigate filtered splats, but replace the claimed one-pixel guarantee. The rule neither bounds image error nor defines how to draw large subtrees waiting for resolution.

I reviewed the requested documents and relevant curve code. I treat P3’s figures as reported evidence; I did not rerun the probes or independently judge their images. No files were modified.

1. **The one-pixel rule is a useful heuristic, but its justification is unsound.**

   The [design’s central claim](/home/daniel/Projects/telperion/.worktrees/ondemand/.flow/evidence/fn-209-tree-space-f1-trees-resolved-on-demand/DESIGN-RULE.md:22) assumes that sub-pixel structure becomes invisible except for its total coverage. Its **filtered position, visibility and colour still matter**. A small object moving across a pixel boundary changes neighbouring pixels. Collapsing it to its bud changes that signal. Correct filtering depends on the footprint and reconstruction filter, rather than a universal one-pixel cutoff. [PBRT, Image Reconstruction](https://www.pbr-book.org/4ed/Sampling_and_Reconstruction/Image_Reconstruction).

   There are several distinct failures:

   - **The envelope supplies no bound.** RMS reach can hide long, sparse extremities. The design describes RMS about the centroid, while the current [`Envelope::reach`](/home/daniel/Projects/telperion/.worktrees/ondemand/crates/telperion-space/src/closed_form/envelope.rs:53) measures length-weighted wood distance from the bud. Neither bounds all wood and leaves. A centroid-relative radius also cannot bound displacement to a splat placed at the bud without including the centroid offset.
   - **Leaf area is not visible coverage.** Projection depends on leaf orientation; coverage depends on overlap and occlusion. Expected physical area alone cannot supply it. Expected coverage also need not resemble the particular seeded subtree that eventually appears.
   - **Many small splats can produce a large error.** Consider two opaque fragments each covering half a pixel. Their combined coverage is 0.5 if they coincide, or 1 if they occupy complementary halves. Ordinary alpha-over gives 0.75 in both cases. Scalar coverage has discarded the information needed to distinguish them. Thousands of correlated fragments can therefore change crown opacity systematically.
   - **Colour blending needs depth order.** Even where accumulated alpha is order-independent, differently coloured fragments are not. Separate wood and leaf passes cannot establish the correct interleaving. Sorting or order-independent transparency addresses ordering, but cannot reconstruct missing sub-pixel overlap. [NVIDIA, Stratified Sampling for Stochastic Transparency](https://research.nvidia.com/publication/2011-06_stratified-sampling-stochastic-transparency).
   - **A single depth is insufficient near occlusion boundaries.** Writing the splat’s depth as though its whole footprint were opaque hides surfaces through its uncovered portion. Disabling depth writes loses occlusion between splats. Bud depth can also place the aggregate on the wrong side of its bearer.
   - **Lighting remains visible below one pixel.** The bearer’s normal does not represent a collection of differently oriented leaves and twigs. Directional light, backlighting and material differences can change their integrated colour.
   - **Camera size does not bound shadow error.** A subtree small in the camera can be large in the light’s projection or cast a visible shadow elsewhere. Resolution must account for shadow consumers too. Keep these rendering decisions separate from DESIGN-LIGHT’s camera-independent physiological state.
   - **The threshold can pop.** Crossing one pixel replaces expected coverage, position and shading with a different realization. Hysteresis reduces repeated switching; it does not remove that mismatch. Independent alpha fades can introduce another opacity error.
   - **Alpha does not itself provide antialiasing.** The splat needs a continuous, normalized screen-space filter footprint. A point or tiny quad can still disappear between raster samples.

   **F3 does not establish the proposed equivalence.** Its [ribbon implementation](/home/daniel/Projects/telperion/.worktrees/ondemand/crates/telperion-core/src/pipeline/surface/curve/tessellate.rs:193) retains the realized centreline, true width and varying normals. Its geometry participates in depth and shadow rendering. Also, its ribbon threshold is **one pixel in radius**, not a whole subtree fitting inside one pixel.

   Finally, the rule leaves progressive startup undefined. A pending unresolved subtree may span hundreds of pixels. Drawing it as a sub-pixel splat cannot provide the promised first-view stand-in.

2. **The 40 m view is plausibly expensive, but “content-bound, not rule-bound” is unproved.**

   The pixel-scale argument establishes that many details are potentially visible. It does not establish that most botanical elements must be generated separately. Occlusion, redundancy, chain granularity and the quality of the substitute all affect the necessary work.

   P3 tested particular rules paired with particular proxies. Their high resolved shares establish the cost of those combinations, not a lower bound for every representation. Round 3’s error remaining above the floor even when fully refined especially warrants checking the reference and rendering path before attributing cost to unavoidable content.

   I would replace the claim with:

   > The tested approaches resolve much of the tree at 40 m. Improving cost per resolved chain is justified; the minimum necessary resolved share remains unknown.

   Report **visible contribution and cost saved**, alongside chain and phytomer shares. A resolved percentage alone can conceal where the expense lies.

3. **The proposed bar and diagnostic need changes.**

   The view matrix is a useful starting point. The alternate-leaf-seed “noise floor” is **specimen variation**, not a measurement of rendering noise or perceptual acceptability. It can tolerate systematic blur or opacity bias, while providing almost no useful tolerance for bare wood. Multiplying it by 1.5 does not fix that.

   Use the same seeded specimen as the primary reference. Retain alternate seeds as context. Supplement whole-image difference with tree-region error, silhouette and gap preservation, opacity bias and shadow comparisons. P3’s reported case where drawing nothing scores better is a warning that the aggregate metric cannot carry acceptance alone.

   Add targeted checks before another large matrix:

   - Coincident and complementary sub-pixel coverage, with wood and leaves at different depths.
   - Sub-pixel camera translation and slow movement across the resolution threshold.
   - An occlusion edge and a subtree small to the camera but large to the shadow map.
   - Cold appearance and a camera cut, including the visibly incomplete frames.

   The timing bar must distinguish resolution work from total frame time. Count preparation, coupling, scheduling, surfacing, upload, splat overdraw and shadow work, consistent with DESIGN-LIGHT’s accepted end-to-end gate. Measure first-frame latency and time to acceptable quality. An average per-chain cost model cannot guarantee an 8 ms limit when one indivisible chain exceeds it. Spruce bursts are a documented failure of a universal bound, even when their cause is understood.

   **The realized-coverage diagnostic is helpful, but its binary conclusion is false.** Success would implicate the estimator or its conditioning, projection and conversion, not necessarily the committed rows. Failure could implicate the representation’s missing position, depth or overlap information, even with a perfectly implemented splat renderer.

   Use successive substitutions on a fixed set of replaced subtrees: expected versus realized area; bud versus realized projected centroid; scalar splat versus a richer realized footprint. Start with isolated unlit examples, then introduce overlap, depth, lighting and shadows. This separates causes that the proposed diagnostic combines.

4. **I would replace the rule with an explicitly measured approximation contract.**

   Keep one resolver and one pipeline, with camera-independent botanical state. Then:

   - Treat projected envelope size as a **refinement priority heuristic**, not proof of invisible error. Define whether size means radius, diameter or maximum projected span.
   - Distinguish conservative bounds, expected spatial moments and projected coverage. They serve different purposes.
   - Define the splat’s filtered footprint, projected centroid, premultiplied colour, depth handling and compositing. Accept scalar splats only where the small adversarial checks establish adequate fidelity; refine where they fail.
   - Apply the rule to every rendering consumer, including shadows, without feeding camera refinement back into physiology.
   - Specify large pending stand-ins and their permitted transient error separately. Measure convergence after cold appearance and cuts.
   - Specify a transition that avoids double-counting coverage and preserves stable lineage identity. Verify it in motion; a fade alone is not evidence.
   - Keep the owner’s screen-error budget explicit. One pixel is a convenient scale, not a free accuracy guarantee. Account for proxy error and F3 surface error together.

   This preserves the proposed engine architecture and gives splats a fair test. It removes the unsupported assertion that expected area at a bud is visually exact whenever an RMS envelope falls below one pixel.