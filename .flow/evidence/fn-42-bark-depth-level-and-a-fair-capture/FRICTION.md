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

### 2026-09-19 — GPU contention before timing extension

Preparing fn-26 baseline timing for the flat bark fixture and mature oak frame, the preflight `nvidia-smi --query-gpu=name,utilization.gpu,memory.used --format=csv,noheader` reported RTX 3080 utilization 11%, 984 MiB resident before this worker ran GPU work. Cost under one minute; no benchmark retry or expensive whole-tree build started. An idle GPU interval would remove this local session obstruction. Timing instrumentation can be implemented and compiled independently; unavailable baseline is not recorded as a valid performance sample.

### 2026-09-19 — candidate distance regression

The first combined parallax/colour/grain candidate failed the unchanged oak 4× distance bound (mean 3.304683/255). About one minute of the broad gate run reached this failure. Isolating grain from the coordinate correction before making a second candidate; keeping the measured failure and unchanged bound avoids a visual-only tuning loop. No workflow change proposed for a normal regression caught by the gate.

### 2026-09-19 — debug grazing mask cost

The existing grazing-resolution test spent over 100 seconds at one CPU core after the oak result while evaluating the spruce fixture in the debug profile. The host used the warmed debug target from capture preparation; that saves compilation but is a poor choice for the CPU raster mask. Finish this single gate invocation, without repeating green gates. An optimized warmed test profile would remove this local build-cost tradeoff; no repository feature spec proposed.

The next required checks used the existing warmed `ci` profile. Compilation took 8.38 seconds and both grazing/trunk tests completed in 11.13 seconds instead of the debug run's 112.34 seconds. The local build-profile issue is resolved for the remaining checks.

### 2026-09-19 — historical hero sweep recipe incomplete

The fn71 close-up sweeps reproduced and passed, but its hero sweep stores curves without the complete camera/capture recipe. Reconstructing it with the default hero camera failed the 0.03 bound. The report states 1350×900, so the host corrected an initial 2160×1440 assumption and ran one pre-change shader control. The corrected-size candidate and baseline both reach a 2.26 far/near band ratio at 4×, unlike the historical 1.05. About three minutes, three small numeric capture runs, no images viewed. This reconstruction cannot certify the historical hero invariant and does not establish a new shader regression. Stop the reconstruction loop here with NEEDS_HUMAN rather than guessing further cameras. Persist complete camera, dimensions, view/mask, seed, source hashes and invocation beside future sweep curves; owner may choose a line in an open evidence-tooling spec. No spec created.
