# Build friction from fn-71: a test that pins a mechanism, and a cost with no term on it

## Goal & Context
<!-- scope: business -->

The fn-71 build (bark relief at distance, 2026-09-18) ran four rounds and recorded eight friction entries. Six of them need nothing here: two were fixed inside fn-71 itself (the resolution receipts its R2 added, and the sweep tool's footprint walk after the camera walk measured framing instead of shading), two are the command guard on the owner's machine rather than the repository, one is the codex quota probe fn-70's R4 already owns, and one was a careless read with no repository cause. The two below are repository work, and each cost the build real time at the moment it could least afford it. [paraphrase]

Neither is about the tree. Both are about how an agent learns what its change did. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The resolution and filtering tests state their contract as a convergence, not as a mechanism: that the far reads average to the near mean within a stated tolerance, never that a term fades to a constant at a named footprint. Removing every amplitude fade in fn-71's round three broke `bark_filter`, `grain` and `smooth_means` although the contract they exist to defend held better after the change than before, and each had to be re-pinned by hand. [paraphrase] Errors: a test that cannot be phrased as a convergence says so in a comment naming what mechanism it pins and why.
- **R2:** A frame-cost capture attributes its cost by term without a hand-run sweep of `--family` rows. fn-71's first fade-free build cost twenty times the frame (the beech 11 ms to 236 ms) and four timing rounds with rows switched off by hand were needed to find which term held it; the shipped round measured the same table by hand again. [paraphrase] Errors: a per-term number the capture cannot measure is absent rather than estimated, and the tool says which terms it could not separate.

## Boundaries
<!-- scope: business -->

- No change to the generator, the renderer, the presets, or any bound fn-71 landed. [inferred]
- The command guard and the model quota stay off the repository: the first is the owner's machine, the second is fn-70's R4. [paraphrase]
- No new capture rig; R2 extends the timing receipt the headless renderer already writes. [inferred]

## Decision Context
<!-- scope: both -->

Two fixes in one spec because they share a cause: a change to shading at distance is judged by numbers the build has to work to obtain, so the build pays for the measurement twice, once to learn what broke and once to learn what it cost. fn-71's FRICTION.md entries of 2026-09-18 are the evidence for both criteria. The same shape as fn-70, which took fn-69's shop-floor friction the same way. [paraphrase]

## Strategy Alignment

- Serves "Surface and rendering at scale": the frame metric is judged per change, and a cost that names its term is what keeps a rendering technique inside the budget rather than discovering it later. [strategy:Surface and rendering at scale]
