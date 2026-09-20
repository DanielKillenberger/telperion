# Bounded tuning evidence tool

`tuning-loop` belongs to `telperion-jev`, never the generator, renderer or shipped presets. It selects authored direction/magnitude names; Rust computes bounded numbers. It does not run a sweep or ship a species.

Build with `cargo build --release -p telperion-jev --bin tuning-loop`. The command accepts:

```text
tuning-loop freeze --manifest AUTHORING.json --table DIALS.json --out FROZEN.json
tuning-loop calibrate --manifest FROZEN.json --ledger LEDGER_DIR --out JOURNAL.json --max-tokens N
tuning-loop vision-replay --manifest REPLAY.json --adapter ADAPTER.json --out JOURNAL.json --max-tokens N
tuning-loop run --config CONFIG.json --out RUN_DIR
tuning-loop run --config CONFIG.json --out RUN_DIR --resume DECISION.json
```

Live Jev calls need `TYPESAFE_API_KEY` through the documented interactive-shell environment. Offline tests require no key. Never print the key. `freeze` regenerates questions from the runtime question implementation and creates, rather than overwrites, the frozen manifest. Calibration journals reserve before each call and persist result indices and actual usage. Failed or interrupted attempts retain reservations.

The typed config is `tuning::live::Config`. It names the species/seed, partial initial overlay, versioned authored dial table, owner notes, measurement binary/profile, matched camera references, numeric reference IDs, vision adapter, reference images, independent quality anchors, required checklist/view/seed cells, frozen calibration manifests/results, resolved Jev model and cumulative budget. Numeric references can be BWHOLE/BBARE while visual requirements additionally include BBASE. Required cells must include fixed and fresh seeds. All image bytes are hash-checked. Calibration model, table, questions and visual protocol must match the runtime roles.

Each evaluation records the effective partial overlay, metrics, numeric gates, node-cap result, matched comparison distances and image identities. Numeric failure stops before rendering. The weighted score uses five relative-distance metrics; unreadable observations contribute distance one. At most four targeted proposals run per round. The best three feasible trials are retained as numeric finalists, not automatically visually ready.

Visual readiness means every required cell passes with no blocking defect and a fresh matching identity. The rubric is recognizable intended species at established catalogue quality, not photorealism. Optional detail improvements are observations, not blockers. Wrong species character, obvious construction artifacts, regression below the independent accepted-quality floor and explicit unmet requirements can block; unassessable cells are unknown. Whole-crown calibration does not imply bark or fresh-seed validation.

The baseline first inspects the first configured fixed-seed view (configure whole crown first), reserving 25,000 tokens. Every other required cell is explicitly unknown, so this shortcut cannot award readiness. A later candidate assessment includes every required view/seed and reserves 40,000 tokens. The run pauses if that pack cannot fit; it never silently omits cells or reduces image resolution to manufacture readiness.

Blocking defects route before another tuning round. Existing bounded dial work can continue; capability gaps return a structured handoff for the host/fn89, not an automatic new spec or repair. Continuation separately judges tractability, evidence/progress and risk. Code additionally requires current evidence, a bounded next-token estimate, known usage and remaining hard budget. Confidence controls uncertainty in a consumed decision; it is not probability of engineering success.

`run.json` is the durable checkpoint, protected by an OS lock and atomic replacement. Resume requires the exact pause ID, previous identity, scoped action and rationale. A repaired input revision additionally names `next_identity`; spend and old trials survive, while the current selection and visual assessment are invalidated and reevaluated. Caps cannot silently change on resume. Interrupted render reservations may be retained through explicit recovery; unknown model usage remains a reconciliation blocker. Starting another output directory is a new run, not permission to reset a project's externally tracked cumulative allowance.

An explicit `token_cap_extension: {previous, next}` authorizes only that exact increased token ceiling, tied to the same scoped pause decision. `preserve_evidence: true` is narrowly for policy-input-only continuation: normalizing the new cap back to the prior cap must reproduce the original complete config/artifact identity. The current trial identity, image hashes, reference/anchor bytes and visual model must still match. Relevant artifact changes refuse reuse. This one-step preservation does not relabel old evidence; a later second preservation after another identity change fails closed without an explicit lineage mechanism.

`round_cap_extension` and `visual_cap_extension` use the same exact old/new structure. Historical runs lacking a visual counter require an explicit `visual_reconciliation` listing every paid attempt ledger, prior cap and reconciliation reason; failed or unknown paid attempts count too. Only a proven pre-model failure may be excluded, with its evidence retained. Visual reservations increment before dispatch and survive failure/interruption.

`baseline_amendment: {previous, next}` names exact partial overlays. It validates the new nested wire through `params::overlay`, rejects evidence preservation, invalidates current/visual evidence and retains prior trials/spend. Ordinary resume after accepted tuning retains effective candidate overrides; it compares the configured starting overlay with the last authorized baseline, not the tuned candidate. Historical trials remain visible with revision tags and node/growth feasibility, but old-identity trials cannot appear as current finalists.

General unattended magnitude efficacy remains unvalidated. Calibration receipts alone cannot unlock the live command. This increment permits only an explicit scoped `experimental_pilot` in the human resume decision: nonempty purpose/reason, exact next identity and cumulative token/round/evaluation/image/visual ceilings. The decision is appended to run authorization history, and every existing calibration check remains mandatory. This is bounded experimental permission, not a qualification override. A fresh unqualified run pauses before reading the live key; an unattended efficacy-proof contract is not yet implemented.

Before any round-boundary model call, the engine checks the hard round limit and the sum of serialized-request reservations for routing, continuation and proposals plus the full-cell visual reservation. The continuation state names current authored dial values/bounds/actions, max four candidates, numeric measurement before rendering, separate visual verification and no shipping. It carries the real downstream reservation, not an arbitrary flat estimate. Pre-dispatch capability assessment only returns a host handoff; it is never authorization to execute an unknown-cost repair.

The fn68 evidence folder records the small frozen calibration and its limitations. Tests exercise deterministic engine iteration, numeric failure before rendering, stale/missing visual evidence, uncertainty, budget persistence and scoped CLI recovery. Live pilot/convergence evidence must be reported separately; green mocks do not establish a species is ready.
