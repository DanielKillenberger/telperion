# Telperion 0.2.0 release review

The five changes worth most are to stop initializing four unused renderer pipelines, finish fn-186's falsely configured no-geometry test job, retire the scaffold's obsolete test workload, use one measured gate resource declaration, and close the superseded CPU/fine-root programme. The first two have concrete code causes. The scaffold needs a consumer-by-consumer retirement, not a directory deletion. None requires changing the approved trees.

Reviewed 2026-10-16 at F4 `8dab10634`, in a detached review worktree. PR #159 is open and draft against master. fn-257 is treated as landing with device-held fields, sizing, dispatch fusion, compact records, ceiling changes and scaffold-pin removal. Its inspected worktree was at `959c8859a` with ongoing edits; final implementation and combined savings are unknown. `LAYOUT.md` did not yet exist. The separate flow-trim job owns evidence archival.

Evidence is source inspection, repository-wide reference searches, `flowctl brief/show`, package-scoped `cargo tree --offline -p telperion-core -e features -i telperion-core --no-default-features`, and existing measurement summaries. No compilation, full gate, capture or browser WebGPU was run. Historical timings below retain their source revision; proposed savings are not benchmarks. Paths are relative to this worktree unless prefixed `devfield`, meaning the sibling worktree. Inventory, dead-code and process scouts supplied facts; architectural recommendations are the host's judgment. A separate Claude-family read-only verification challenged the findings; its corrections to test consumers, metadata counts, coverage scope and sequencing are incorporated.

## Cut before 0.2.0

### 1. Stop constructing the obsolete position pipelines

**What and evidence.** `crates/telperion-render/src/generation/preparation.rs:14` sends every resident request to `prepare_resident`; the remaining production call to `prepare_whole` has CPU delivery. Its resident-only branches at lines 41–140 are reached by five explicit calls in `generation/resident/route_tests.rs`, not production. Nevertheless, `generation.rs:206` constructs the position programme with four entry points, `emit`, `admit`, `radii`, `finish`. `generation/io.rs:167` creates one pipeline per entry. `generation/positions.rs` is another 277 lines used by the old branch and tests.

**Cut.** Keep the old comparison route under `cfg(test)` where it still supplies an oracle; stop creating its four pipelines in production. Do not remove CPU delivery or the shared leaf-placement programme.

**Saving.** One shader-module creation and four compute-pipeline creations per resident-capable generator initialization, plus retained pipeline objects. CPU-only generators already skip them. Startup milliseconds and package bytes are unknown. The 277 lines need not disappear to obtain this saving.

**Risk and proof.** Low to moderate, because the old and current paths share helpers. Compile both production and tests, check all `positions` and `prepare_whole` references, retain the five comparison tests and direct `emit_positions` tests at `generation/lobed_tests.rs:27` and `generation/tests.rs:436`, and confirm production initialization contains no position-program creation. Run the ordinary required checks once when implementing.

### 2. Delete the isolated expected-envelope experiment

**What and evidence.** `crates/telperion-space/src/closed_form/envelope.rs:1` contains a private expected-envelope algorithm, 152 lines; its tests add 254. `rg 'StandIns|closed_form::envelope|super::envelope' crates src` finds only the implementation and its own tests. `closed_form.rs:19` suppresses its dead-code warning. Neither type escapes the private module. The same state exists in devfield.

**Saving.** 406 lines plus the module declaration and tests that maintain a result no consumer reads. Binary and test-time savings are unknown; dead-code elimination may already remove its shipped bytes.

**Risk and proof.** Low. Pair this cut with retiring fn-209’s superseded stand-in design. Remove the module and self-tests, repeat the reference search, and compile/check the space crate. Preserve shared event/count machinery and `along.rs`, which still has consumers. The separate core crown-envelope module is unaffected. In particular, `kept/path.rs:286` still reads `Count::wood`; deleting all moment-related fields would exceed this finding.

## Simplify before 0.2.0, if cheap

### 3. Finish fn-186 so the no-geometry job tests its named configuration

**What and evidence.** `crates/telperion-core/Cargo.toml:42` has a self dev-dependency with default features enabled. `.github/workflows/tests.yml:165` asks for no default features. The package-scoped feature-tree command run during this review still shows `geometry` enabled by that dev-dependency. fn-186 already records the defect and the owner's authorization.

**Saving.** Would remove an unintended second geometry test workload and restore no-geometry unit tests. CI already compiles the no-geometry configuration in its build step. fn-185 measured this job at 333–454 seconds on the October 2 code; today's saving is unknown.

**Risk and proof.** Moderate. Disabling the dev-dependency's defaults exposes test code that depended accidentally on geometry. fn-186's old error counts and test counts need refreshing. Set `default-features = false`, gate only genuinely geometry-dependent tests, verify feature trees and test lists for both configurations, and retain the no-geometry job. Do not simply delete its tests or promise the historical time saving.

### 4. Correct metadata that describes the replaced product

**What and evidence.** `docs/pipeline.md` still specifies fixed-point decision arithmetic and a CPU coarse-tree route awaiting fn-257. `crates/telperion-space/src/lib.rs:10` says nothing is wired into the pipeline, contradicted by `pipeline/space.rs:79`. `package.json:4` still describes the old silhouette/attractor generator. `catalogue.rs:279` says flower controls are dormant on every shipped species, while `telperion.values:718` and `laurelin.values:699` activate flowering and `pipeline/space.rs:569` emits blooms. The shared `FLOWERS` metadata serves 30 bloom/material rows (10 material and 20 bloom rows, counted by `rg` on `dial: .*FLOWERS`).

**Saving.** Removes contradictory guidance and a false tuning exclusion rationale across 30 rows. No runtime saving is claimed.

**Risk and proof.** Low for prose; changing dial eligibility is a separate behavior change. Describe the actual cycle and floating-point agreement; update the coarse-route description with or after fn-257, since it remains true before that change. Describe the final field contract; state the real flowering dormancy condition. Regenerate derived parameter docs through the existing catalogue tooling and check their diff. Preserve the owner's deferred flower redesign.

### 5. Give gate callers one measured resource declaration

**What and evidence.** fn-259 `FRICTION.md` records a 30-minute wait requesting 20 GB. fn-274 had already demonstrated a passing gate at 9 GiB/four test threads; fn-259 ultimately passed at 12 GB/two threads. fn-274 `measurements.md:46` records 1,551 seconds and 8,860 MiB anonymous peak. Its 7,541 MiB standalone-binary result did not establish an 8 GiB whole-gate budget.

**Saving.** Removes the stale declaration that caused that recorded wait. Future waiting time is workload-dependent and unknown.

**Risk and proof.** Low for consolidating the invocation; moderate for shrinking a limit. Replace the stale 20 GB declaration in fn-271 R4 with one current invocation and its measured revision, concurrency and scope; have hosts/reviewers use it. The 9 GiB run had only about 3% anonymous-memory headroom, so it is evidence of a pass, not a universal safe cap. Keep `telperion-heavy` admission and headroom for cargo, kernel and cache charges. Verify the command against the existing accepted run; this review does not request another gate merely to remeasure it.

## After 0.2.0

1. **Retire the scaffold test generator and migrate useful assertions.** `pipeline.rs:189` exposes the old scaffold only to tests; lines 241–263 state that production no longer reaches it. The old branching, colonization, twig, radius and bias modules total 6,731 lines including shared contracts and tests. That is the audit surface, not a deletion estimate. `pipeline/foliage.rs:286` contains a particularly clear 64-line test-only shell culler. fn-198 R0 already called for retiring the old generator. Separate tests of obsolete generation from tests of today's surface, foliage and serialization; delete the former and feed engine specimens to the latter. Preserve `clothe_leaf_bases`, validation, parameter types and shared helpers. Exit with current entry/geometry coverage intact and measured gate time before/after. This is the largest checked maintenance opportunity, but not a safe last-minute directory cut.

2. **Use fn-285 for one measured kernel programme.** After fn-257, reprofile before choosing work. Its pre-change `GROWTH-PROFILE.md` assigns spruce 82–86 ms stamped to units/wake and 62 ms to balance/allotment; timestamp instrumentation inflated total growth, so these are ranking evidence, not additive wall-time savings. Investigate redundant work and kernel dependency depth while preserving coupled yearly growth and measured look. Merge overlapping performance obligations into that programme only where their consumers match. Do not restart rejected aggregate, hierarchy-light or grown-at-birth designs without a new bounded oracle. COUNCIL.md records why they failed.

3. **Re-scope fn-242 around the artifacts fn-257 actually ships.** Its current plan profiles a single-thread coarse-field Wasm engine and depends on superseded fn-227. The owner explicitly retained download reduction after the release. Measure the final package's reachable modules and transfer sizes, then remove unreachable dependencies or unnecessary entry coupling. The old 616 KB engine attribution is not a forecast for a device-held field. Keep the three published Wasm entries until their actual consumers and migration contract justify any packaging change.

4. **Replace key-only cross-executor agreement with drawn-geometry agreement.** The owner already deferred this. fn-259 `RELEASE.md` records 185 relabelled oak nodes at matching positions/radii and one threshold-adjacent twig. Preserve exact repeatability within an executor, discrete identity contracts where consumers need them, attachment checks and finite geometry. Compare the delivered wood and foliage with measured tolerances so labels alone do not veto an accepted tree. This reduces misleading failures; implementation cost and test-time savings remain unknown.

5. **Make species evidence reference a generator revision explicitly at the comparison boundary.** `SPECIES-AUDIT.md` records birch round 14, Telperion round 14 and Laurelin round 8, with engine changes invalidating earlier evidence. Bind future comparison inputs and verdicts to engine revision, backend, seed and value files; reuse approved sheets when those inputs still apply. First inspect existing runner support and add only the missing check. Do not reopen this release's accepted species or build another conductor merely to wrap that check.

## Specs to close or merge

These are recommendations, not state changes. `flowctl brief` still listed the following legacy specs open in the inspected snapshot; prose and tracker state can disagree.

- **fn-214 and fn-227:** reconcile state with their explicit “Closed (host, 2026-10-13)” sections. fn-257 supersedes their CPU coarse-field route and coarse-only export split. Do not execute their old requirements.
- **fn-209 and fn-211:** recommendation for the host, not an already recorded closure. Close the old CPU-above-cut/GPU-fine-root designs as superseded by fn-255/256's complete cycle. Preserve their measurements and carry any still-live performance obligation into fn-285; do not mark obsolete requirements implemented.
- **fn-225:** recommendation for the host. Close as superseded. It routes selected species through the old fine-root GPU executor and retains oak/palm on CPU. Current `prepare_async` selects by delivery, and fn-256 supplies the whole resident grower.
- **fn-242:** retain and rewrite against post-fn-257 artifacts; remove the obsolete fn-227 dependency.
- **fn-245:** finish its umbrella accounting into fn-237 and fn-285 once fn-257 qualifies. Keep the council's decisions and failed-experiment conclusions, not an independent competing speed backlog.
- **fn-186 and fn-240:** keep. The first has a reproduced feature-configuration defect; the second's predictor still serves live sizing/blending consumers. Refresh old measurements before execution.

## What we do stupidly

1. **Compile an oracle on every production renderer startup.** The four position pipelines above exist for a route only tests request. Compile them only for that oracle.
2. **Call a geometry-enabled test run “without geometry.”** The feature-tree reproduction establishes the mismatch. Finish fn-186.
3. **Maintain tests of replaced generation as though every assertion still protects the product.** The scaffold remains test-only. Classify assertions by current consumer before optimizing them.
4. **Run review fan-out as unbudgeted extra gates.** fn-274 `FRICTION.md:17` records two reviewers running a 3.75 GB repeat test outside admission while the gate ran, taking 176–325 seconds. Give reviewers existing evidence and require a named unresolved question before a focused, admitted rerun. Whether later briefs fixed this is unknown.
5. **Publish standalone memory as a whole-gate budget.** fn-274's 8 GiB attempt was killed after 320 seconds; the successful scope needed 9 GiB. Record scope and concurrency beside every number.
6. **Time competing sessions and treat differences as optimization evidence.** fn-271 records load 10–47, briefly 154. Use a quiet measurement window and paired runs with load reported; CPU time can supplement wall time, not replace user latency.
7. **Keep superseded plans actionable.** fn-214/227 explicitly say closed but still appear open. Reconcile decisions with task state so the next driver cannot resurrect the rejected route.
8. **Confuse optimization inventory with savings.** fn-257 measures oak/spruce census-induced GPU idle below 1 ms, while host census waits are much larger. Removing those waits is not a 100–174 ms saving. Likewise, 6,731 scaffold-related lines are not 6,731 deletable lines.

## What not to touch

- **The cycle, lattice and CPU reference.** Growth is the product. COUNCIL.md rejected the hierarchy-light experiment at 476 seconds against 0.47 seconds and found coarse feedback faithful only near full expansion. CPU/GPU executors are the sanctioned exception and serve different consumers.
- **Predictors just because the cut goes.** `pipeline/space.rs:239`, `blend/size.rs:160`, `generation/cycle/build_arena.rs:85` and `resident/budget.rs:98` retain count/sizing consumers. fn-256 R3 itself conditions predictor deletion on no remaining consumer; its introductory “can go” is insufficient. Devfield `SIZED.md` attributes the birch fix to restoring that prediction, measuring 23→0 moves and 383→210 ms at seed 1.
- **Per-entry whole-preset coverage and fine walk sampling.** fn-274 explicitly retained four expensive mesh checks because they prove separate entries. fn-271 found that 50/100-step walks missed deliberately full-size births which 200 steps caught. Keep the defect catches.
- **Deprecated wire rows without a migration decision.** Catalogue tests deliberately preserve parsing, overlays and serialization of spacing/clump/clumpSpan. Unread output behavior does not make removing accepted inputs automatically safe.
- **fn-257's agreed work, owner verdicts, accepted timing exceptions and post-release deferrals.** Compression remains in release. Birch ground branches, flowers, larger mature trees and geometry agreement remain where the owner placed them. The neural-field experiment fn-268 is explicitly owner-requested and bounded, not an accidental production dependency to delete.

A minimal 0.2.0 still contains the eight approved species, one botanical cycle, two executors, renderer and field consumers, and proof at their package boundaries. Its removable excess is the superseded experiments and routes around that core, plus repeated process work that tests the wrong configuration or ignores already measured resource limits.
