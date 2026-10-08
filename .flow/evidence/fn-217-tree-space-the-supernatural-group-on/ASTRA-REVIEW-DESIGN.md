**The design needs revision before implementation.** X2 does not currently specify an ordinary-beech “off” state, and S1’s proposed edit points are unsafe. The selected mechanisms are buildable, but several claimed outcomes need different acceptance checks.

Reviewed against branch `7a7324f6`, including §6’s release split. This was a read-only code review; I ran no builds or captures and modified no files.

1. **P1 — X2 leaves legendary flowers and Laurelin’s coloured margin enabled.**  
   [DESIGN.md:77](/home/daniel/Projects/telperion/.worktrees/supernatural/.flow/evidence/fn-217-tree-space-the-supernatural-group-on/DESIGN.md:77), [DESIGN.md:379](/home/daniel/Projects/telperion/.worktrees/supernatural/.flow/evidence/fn-217-tree-space-the-supernatural-group-on/DESIGN.md:379)

   X2 gates reflectance, tint, scale and the future light terms. Flowering remains in the species, flower geometry in `/bloom`, and flower colours in the material. [space.rs:431](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-core/src/pipeline/space.rs:431) creates blooms from the PA’s flowering share, and [input.rs:158](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-core/src/pipeline/input.rs:158) passes `family.bloom` through unchanged. Turning X2 off therefore retains the white blossoms or golden racemes.

   E2 also uses the **natural margin colour** for its gold tint. That colour already contributes diffuse yellow independently of reflectance at [foliage.wgsl:166](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-render/src/shaders/foliage.wgsl:166). Disabling the glint leaves the yellow rim. The darker Telperion front is explicitly retained too.

   **Required correction:** define which legendary flower and colour contributions return to natural values when disabled. X2 can implement that through applied stage inputs without preset inheritance or species branches, but its present row list cannot deliver §6’s off-sheet.

2. **P1 — S1 cannot safely be implemented by scaling the two `up()` helpers.**  
   [DESIGN.md:135](/home/daniel/Projects/telperion/.worktrees/supernatural/.flow/evidence/fn-217-tree-space-the-supernatural-group-on/DESIGN.md:135)

   CPU `up()` transforms positions **and heading/side vectors** at [space.rs:427](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-core/src/pipeline/space.rs:427). GPU `up()` does the same at [dress.wgsl:372](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-render/src/generation/fine/dress.wgsl:372). Scaling either helper changes frame lengths as well as geometry. Neither helper scales radii.

   The device handoff also combines CPU attachment positions, radii and distances with an engine-space root origin at [stubs.rs:47](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-render/src/generation/fine/stubs.rs:47). The design must specify those units, including previous-wave records, rather than treating conversion as two isolated multiplications.

   **Required correction:** preserve natural-unit topology decisions, scale position and length quantities exactly once, and leave orientation vectors unchanged. Include `radius`, `start_radius`, `base_radius`, grown lengths and device distances. Finish before whole-tree folds and expansion. This remains a term inside the existing pipeline and fits the sanctioned CPU/GPU executor structure.

3. **P1 — S1 preserves branchwise leaf spacing, not crown coverage; increasing `florets` does not fix spatial flower coverage.**  
   [DESIGN.md:140](/home/daniel/Projects/telperion/.worktrees/supernatural/.flow/evidence/fn-217-tree-space-the-supernatural-group-on/DESIGN.md:140), [DESIGN.md:382](/home/daniel/Projects/telperion/.worktrees/supernatural/.flow/evidence/fn-217-tree-space-the-supernatural-group-on/DESIGN.md:382)

   With unchanged leaf size and approximately `s` times as many leaves, total leaf area grows approximately `s`, while crown projected area grows `s²`. At scale 5, leaf area per crown area is approximately one fifth of the natural tree’s, before overlap and culling. “Leaves keep their density” needs that qualification. The count mechanism is explicit at [plan.rs:150](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-core/src/pipeline/foliage/plan.rs:150).

   More florets fill the **same local inflorescence**. Their positions use `t = (i + 0.5) / florets` along an unchanged rachis and pedicel at [bloom.rs:227](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-core/src/pipeline/foliage/bloom.rs:227). Increasing the count can make clusters overlap while enlarged gaps between flowering nodes remain. The row also stops at 64 florets at [bloom_params.rs:13](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-core/src/pipeline/foliage/bloom_params.rs:13).

   **Required correction:** judge crown coverage at the actual legendary scales, including organ-only coverage and visible clustering. Treat `florets × s²` as a hypothesis, not a sufficient preset recipe.

4. **P2 — M0’s stated measurement is unavailable and its ratio cannot diagnose rasterisation loss.**  
   [DESIGN.md:301](/home/daniel/Projects/telperion/.worktrees/supernatural/.flow/evidence/fn-217-tree-space-the-supernatural-group-on/DESIGN.md:301)

   [FrameStats at lib.rs:70](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-render/src/lib.rs:70) contains draw calls, triangles and instances. It supplies neither organ-covered pixels nor projected area. Summed projected floret area also includes overlapping petals, overlapping flowers and flowers hidden behind leaves or wood. Visible coverage can be far below that sum with correct rendering.

   **Required correction:** compare an isolated organ mask against a higher-resolution reference resolved to the target size, with matching visibility, and separate finest-geometry coverage from selected-LOD coverage. M0 is buildable with this instrumentation. A single ordinary coloured still and current counts cannot make its proposed decision.

   M2 likewise needs verification: four-sample alpha-to-coverage is quantised, so “expected coverage equals true area” is not guaranteed for every tiny floret merely by outputting the area ratio.

5. **P2 — F1–F3 do not establish Laurelin’s horn-shaped flowers.**  
   [DESIGN.md:282](/home/daniel/Projects/telperion/.worktrees/supernatural/.flow/evidence/fn-217-tree-space-the-supernatural-group-on/DESIGN.md:282)

   F1 adds colour, F2 filaments, and F3 changes lighting normals. The existing element still rotates separate, fixed-profile leaf blades around an axis at [bloom.rs:92](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-core/src/pipeline/foliage/bloom.rs:92). Narrow opening and overlapping petals might suggest a horn, but the design supplies no checked example demonstrating its throat, flared mouth or silhouette.

   **Required correction:** make a horn-form capability check part of the release work, using existing values first. A successful five-petal blossom sheet does not answer Laurelin’s requirement. Pendant placement exists; the axis and stalk are used to place florets rather than emitted as connecting geometry, so also check that the racemes read as hanging clusters rather than disconnected flowers.

6. **P2 — E2’s neutral arithmetic does not establish full identity, particularly for the existing sheen.**  
   [DESIGN.md:171](/home/daniel/Projects/telperion/.worktrees/supernatural/.flow/evidence/fn-217-tree-space-the-supernatural-group-on/DESIGN.md:171)

   The back already receives sheen through `u.canopy.w` at [foliage.wgsl:227](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-render/src/shaders/foliage.wgsl:227). Giving that existing term the new back tint can change pixels even when the **new reflectance** is zero. Its tint must resolve to the existing behaviour at neutral. The organ block copies leaf uniforms at [frame.rs:221](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-render/src/scene/frame.rs:221), so it also needs an explicit policy preventing leaf-only silver terms from reaching petals.

   E2 is buildable, but define how its reflection shares energy with the existing diffuse/front reflection and how roughness is selected. The current `highlight()` returns both reflected share and directional intensity, rather than a colour to add blindly.

   **Identity conclusion:** passed-species geometry can remain byte-identical, and neutral rendering can preserve pixels. X2 does not itself guarantee either. Separate geometry/field identity from rendered-pixel identity. The claim at design line 138 that float multiplication by 1 inherently breaks identity is wrong for the finite stored values here; changing precision or operation order is the relevant risk.

7. **P2 — The deferred light design incorrectly claims an ambient change reaches the sky.**  
   [DESIGN.md:196](/home/daniel/Projects/telperion/.worktrees/supernatural/.flow/evidence/fn-217-tree-space-the-supernatural-group-on/DESIGN.md:196)

   [sky.wgsl:29](/home/daniel/Projects/telperion/.worktrees/supernatural/crates/telperion-render/src/shaders/sky.wgsl:29) directly tone-maps the sky gradient and never calls `ambient()`. Moreover, the existing ambient function lacks the world position needed for a spatial emitter. L2 requires lighting-interface changes; G1 needs its own sky integration. This does not block §6’s release subset, but it is a wrong code claim.

The remaining buildability and downstream assessment is:

| Area | Assessment |
|---|---|
| **Envelope and folds** | Compatible with S1 when every participating CPU/GPU position is scaled before folding. CPU envelope derivation already reads the converted tree; resident execution combines CPU and device folds. Mixing units would corrupt the result. |
| **Field** | No intrinsic blocker. Wood occupancy reads converted positions/radii; planned foliage reads converted segments and station counts. Apply scale through the common pipeline input so the field package receives it too. |
| **LOD and packing** | Existing selection follows projected geometry. Fixed-size leaves and florets become smaller at a whole-tree camera distance. Position quantisation also becomes coarser as the reference box grows. Check contact precision and motion at supported scales, including the 0.1 and 100 endpoints. |
| **F1** | Buildable through petal coordinates and organ uniforms, with zero eye share preserving existing colour. |
| **F2** | Buildable, but the design still needs filament width, fractional-count behaviour and LOD treatment. Existing organ levels replicate petal indices/deviations; they cannot automatically describe newly added filaments. Measure their visible benefit before accepting the estimated triangle cost. |
| **F3** | Buildable as the stated organ-uniform change. It removes canopy-normal bending, not all inherited leaf transmission, reflectance or crown shading. Its visual benefit remains untested. |

Under §6, light, dew and counterphase are deliberate post-release omissions. **Wind remains a separate unowned visual gap**, so silver revealed by turning leaves and swinging racemes will not appear dynamically. For the release trees, the unresolved requirements are the ordinary-beech off-state, enlarged-crown coverage, Laurelin’s horn silhouette, and silver/gold readability under the intended scene lighting. Those need named visual checks before fn-218 and fn-219 can rely on the two release specs.