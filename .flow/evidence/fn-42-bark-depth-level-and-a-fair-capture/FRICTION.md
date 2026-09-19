# Friction

## 2026-09-19 - stale rendering design

While re-anchoring fn-42 before implementation, the host found that its near/far threshold proposal predates the owner's accepted fn-71 requirement that relief disappear only through footprint averaging. This cost three inspection calls before implementation. Reconciled the existing spec with the later owner decision; re-anchoring older rendering specs against subsequent accepted rendering contracts removes this ambiguity. No new spec proposed.

## 2026-09-19 - original reference photographs unavailable

The host checked the repository and sibling worktree reference caches for the owner-supplied white-oak and Norway-spruce bark photographs named by fn-42. Only historical measurements and rendered stills remain; the original images have no source URL in the fn-32 catalogue. This cost two filesystem searches and one catalogue inspection. R1 and R3 need the actual source crops and a documented scale basis; historical RGB means cannot replace them. A persistent local reference cache with verified paths would remove this setup blocker. Continue only the independent small capture instrument, then return NEEDS_HUMAN for the source images rather than run a calibration loop without its target. This is a local setup issue, not a new repository spec.

### 2026-09-19 — bark-only evidence radius contract

While constructing a flat square-on calibration patch, discovered the production uploader derives material radius only from complete geometry rings; a flat patch gets zero radius and no representative bark. About 4 minutes of API inspection reached this design boundary. Escalated to host before selecting a workaround. A narrow renderer-internal evidence fixture with an explicit radius would remove the obstruction without inventing a false scale or duplicating the shader pipeline.

### 2026-09-19 — capture compilation

The calibration fixture reused the root target directory to avoid a new dependency build, but Cargo still rebuilt GPU dependencies for this worktree. At least 30 seconds compilation rather than capture work. Shared compiled artifacts did not remove this initial rebuild; a stable warmed evidence build would reduce it. One foreground compile remains running; no duplicate test run was started.

### 2026-09-19 — flat fixture lighting orientation

The first explicit capture completed in 7.71 seconds, but numerical inspection showed very dark material and dark fractions above 0.97. The default sun at azimuth 135° faces negative Z while the initial square-on patch normal faces positive Z. Escalated the evidence lighting choice to the host before a second capture. About one minute; a fixture orientation contract stated beside the camera/light rows would have removed this mismatch.

## 2026-09-19 - reference search continuation

After rejecting low-resolution or growth-covered photographs, the host had ended the turn instead of continuing retrieval; the owner had to prompt continuation. Cost: one extra user turn. Keep the accepted search objective active through source rejection; return with selected inspected candidates or a concrete bounded blocker. A third Wikimedia original returned HTTP 429 after two originals downloaded; stopped that request without a retry loop. The two downloaded originals are sufficient to assess the spruce structural reference.

### 2026-09-19 — oak source metadata versus useful pixels

The bounded oak reference search reached three source families. SelecTree's labelled original bark file is only 650 × 567; three high-resolution Commons originals proved to be whole-tree or leaf subjects. Burke's organ-tag search returned none and its taxon listing omits organ descriptions. Approximately 7 minutes, four image inspections. Stopped without calling the poor-resolution source success. A catalogue linking original-resolution bark photographs with subject labels and physical scale would remove this retrieval friction. Python image/helper modules were absent; standard JPEG identification and text parsing avoided installing tools.
