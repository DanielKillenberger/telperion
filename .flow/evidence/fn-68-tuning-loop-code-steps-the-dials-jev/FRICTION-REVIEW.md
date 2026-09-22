# fn-68 friction review, 2026-09-21

The host read all seventy entries in `FRICTION.md` before proposing anything. They are grouped below by the defect behind them, every entry is accounted for by its heading date and subject, and each group carries one proposal. **The owner decides which proposals become specs; the host has written none of them.** Groups E, F and J are marked no-new-spec because fn-68 already fixed them in code.

## A. The product admitted formats it could not produce, so every run went around it

Entries: the invalid side-round shortcut, the `Config.verify` visual-replay seam, reference-first receipts not replayable through the production types, no dry-run mode, a passing `Config::verify` with no fixture (twice), the reference-first path admitted but not prepared by anything shipped, Stage A written from scratch, reference-first unusable on the beech config, and the qualification gate assuming a positive the pipeline has not produced.

Nine entries, one cause. `Config::verify` accepted a `reference-first-replay-v1` and `charge_preparation` consumed a Stage A inventory, and no shipped command emitted either; the first worker's response was to bypass the production path, which is the entry that opens the file. fn-68 has since shipped `tuning-loop inventory`, a reference-first branch in `vision-replay`, `tuning-loop preflight`, `verifying_fixture()` and the bootstrap mode, so the immediate blockage is gone.

**Proposal, one spec:** a repository rule with a test that enforces it — a format a gate admits must be emitted by a shipped command, and any gate a run can pause on ships with an offline fixture that reaches it. It should also settle the one open contradiction this group leaves: whether a reference-first inventory's `target_species` is the preset id or the factual species name, since the two sides still disagree and a paid Stage A was bought against the wrong one.

## B. Reservations are byte-sized, so caps cannot be set from expected spend

Entries: the extra allowance that omitted conservative bounds, the v2 and v3 study preflights that exceeded their allowance (four entries), the v3 precision that added 128, the owner-agreement grading reservation, the reference-first whole-attempt bound, the direct-reference budget boundary, the round preflight that reserved 222,188 tokens, the image cap that only fits one dial, and the null dial that priced a round at nothing.

`judgments::allowance` reserves one token per serialized byte plus 1,024; measured Jev calls use roughly one token per three to five bytes. The bound is safe and about four times the likely spend, which is why this session raised the token cap four times without the spend ever approaching it. The null-dial entry is the same arithmetic failing the other way: a swallowed estimator error under-reserved a round four-fold and reported a plan that fits.

**Proposal, one spec:** a measured bytes-per-token bound with a stated margin, taken from the ledger entries already on disk, and a rule that a reservation estimator returns its error instead of a default.

## C. The one accounting path with hash pinning does not cover the spend that matters

Entries: vision spend not importable through `external_usage`, and the capture actuals written as 20 instead of 24.

`ExternalUsage` verifies ledger hashes and prevents double imports, and it requires a question set, which a vision dispatch does not have. So the largest spend in the run is the one the host hand-edits into an opening balance, and the one hand-edited number in the file was wrong by four captures.

**Proposal, a line in an open spec:** vision dispatches emit an attested record of the shape `ExternalUsage` accepts, or that contract accepts a second attested shape for spend with no questions.

## D. One identity names both the configuration and the evidence

Entries: the cap-only resume that made the run forget what it had preserved, and the priority-approval resume that re-bought the baseline.

`Config::identity()` covers the budget caps, so every cap raise renames the run while its trials keep the name they were measured under. Four separate filters then dropped the preserved trials silently: the run did not fail, it behaved as though it had never measured anything. fn-68 added an evidence-identity chain, which works, but the conflation is still there for the next reader to trip over.

**Proposal, one spec:** decide once whether an identity names the configuration or the evidence; if it names both, give the second one its own name rather than leaving each reader to compare a string.

## E. The judgment contracts were shaped wrong — fixed, no new spec

Entries: the round gate buying a judgment code already had, the acceptance rule conflating direction with magnitude, the proposal judgment shown the whole run, the refused repeat consuming the round's only slot, the continuation lacking a bounded plan, the continuation omitting changed feasibility evidence, the corrected continuation whose action selection abstained, the final attempt pausing on uncertain progress, the comparative request omitting the density direction and its correction, and the owner structural target lost in the acceptance projection.

Eleven entries, all closed inside fn-68: the round boundary is code-first, acceptance reads the probability mass rather than the argmax, each judgment has its own projection, the bound moved to the end of the pipeline, and a trial now records what it moved, what it moved from and what evidence it acted on. The lesson worth keeping is in the entries themselves — a loop that cannot see its own history has to buy an opinion about it.

## F. Prompts leaked what the answer was — fixed, no new spec

Entries: the expected-label leak in the dispatched negative checklist, the resolved model identity guard, the comparison cardinality, the framing repair, the false complete-framing claim, and the pale-sky heuristic.

fn-68 now blinds the replay cases, redacts file paths and trial keys from both review prompts, pins schema cardinality, and treats a clipped render as unassessable rather than as evidence. The two framing entries are the same mistake twice, and both were caught by the owner looking at the raster rather than by any check.

**Proposal, a line in an open spec:** assert in a test that no dispatched prompt contains an expected label or an identifying path. Two of the three adapter scripts now do this; the third is pinned by a frozen replay and cannot be edited without invalidating it.

## G. Local and orchestration overhead — mostly not repository work

Entries: resume instruction overhead, repeated partial preflight handoffs, Cursor packet preparation, overnight scratch evidence lost at restart, historical calibration images unrecoverable, historical build cache freshness, the self-starved workspace gate, the parallel run-lock test failures, the core suite with no finish line, the rustc rlib listing, the reused checkpoint branch, the three passes to find doc comments, and the birch shots record that was in the catalogue all along.

Per the owner's rule a local setup problem is reported and never specced, and most of these are that. Two are repository-level and cost real time today: a gate log written to `/tmp` was destroyed by a restart, and `cargo test -p telperion-core` in the default profile ran past twenty minutes with no finish line while the gate rule already names the ci profile.

**Proposal, two lines in `CLAUDE.md`:** a gate log is written under the worktree's ignored scratch, never `/tmp`; and the ci profile is the only way to run a crate suite locally, so no agent reaches for the default profile to check one crate.

## H. The dial table had to be reconstructed from outside the core

Entries: the table not authorable from the sources that exist, the three passes to answer which rows carry a doc comment, the leaf-word families colliding across groups, and the null dial on the wire.

Seventy-five of 222 wire paths had neither a doc comment nor a validated range. fn-68 wrote 68 doc comments and a 200-row authored table with citations, which is the durable half; the fragile half is that the table lives in the tuner and is paired to the generator by a regex over the `fields!` macro from outside.

**Proposal, one spec:** a single authored parameter table in `telperion-core` that the generator, the presets and the tuner all read, with the meaning, range and step beside the field rather than reconstructed downstream.

## I. CI cannot run the offline end-to-end

Entry: the offline end-to-end needs `uv`, which this project's CI does not install.

One test is `#[ignore]`d with its reason and run explicitly; everything before the evaluation step runs in the gate.

**Proposal, a line in an open spec:** add `uv` to the CI image, or give `Matched` a compare seam so the numeric comparison can be supplied in-process.

## J. Calibration and historical evidence — superseded, no new spec

Entries: calibration prerequisites absent at the feasibility check, historical calibration images unrecoverable, independent admissibility abstaining on development replay, the v3 low-confidence size, the missing beech-positive fixture and the host correction that followed it, the blind joint review missing a gap dimension, the experiment priority changed after dispatch, the joint context persistence gap, the measurement harness inheriting the visual guard, the wider candidate exceeding the fixed frame, the Grok image transport preflight, the Cursor token preference, the direct-reference scope, and the resume integration bookkeeping.

Fourteen entries circling one thing: the loop could not be qualified because no owner-accepted render existed. The bootstrap mode and reviewer-judged selection supersede all of them, and the one durable item — obtaining that first accepted render — is the fn-68 close-out decision itself, not a friction fix.

## What this review does not propose

Nothing here proposes relaxing a gate, a threshold or an acceptance rule. Group A's rule would make gates harder to ship, not easier to pass.
