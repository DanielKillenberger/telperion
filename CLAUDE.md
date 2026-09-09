# Telperion

A runtime tree generator: space colonization for the crown, botanical rules below the crossover, one bias field for everything supernatural. Strategy lives in `STRATEGY.md`; specs and tasks in `.flow/` via `flowctl`.

<!-- flow-next:model-routing:start -->
## Model routing

<!-- Grammar: <tier>: <model>   or   <tier>: <model> at <effort>
     Resolution at each dispatch site: an explicit instruction in the moment,
     then this block, then the agent definition's own default, then the session
     model. A model this harness cannot reach falls back to the session model
     with one note. -->

implementer: claude-opus-5
reviewer-note: host session checks diffs directly; no review backend
reviewer: claude-fable-5-1

Owner routing (2026-09-08, while the Astra quota is exhausted): dispatch Claude Opus for all implementation; the host Fable session checks each diff itself and no cross-model review backend runs. Before that (2026-09-07): gpt-6-astra at low for all implementation, including core generator/renderer logic and measurement, capture, diagnostic and other technical harness work. Do not dispatch Grok. Generator parameters are primary; templates are reusable parameter presets, and supported parameter changes must require no renderer code changes.

<!-- The owner's choice (2026-09-05): Codex gpt-6-astra implements at low
     effort. Per-task code review is not wanted: run work with --review=none.
     When a code review is wanted, it is host-native on Fable (the reviewer pin
     above, a different family from the writer). Since 2026-09-08 the plan
     review and spec completion review also run host-native on Fable
     (`.flow/config.json` review.backend = none since the owner asked for no cross-model review); the Codex
     high-effort review was a major quota sink. The live QA pass
     (pipeline.qa) is off; run /flow-next:qa by hand when a spec is ready. -->
<!-- flow-next:model-routing:end -->

## Token and evidence budget (owner, 2026-09-08)

fn-13 task 5 consumed a full weekly quota on 22 full-forest GPU captures and image inspection. These rules bind every agent and the pilot loop:

- **Small before large.** Never start a 1,024-tree capture until the specific defect reproduces and is fixed on the 8-tree forest with a red/green test. At most one full-forest capture per commit.
- **Read summaries, not receipts.** Agents read `OUTCOME.json`, `INSPECTION.md` and `metrics.json`. Never open `receipt.json`, videos or frame sequences; never view more than four images per capture. Raw receipts, videos and frame directories are gitignored and stay on disk.
- **Per-task budget.** A task gets 5 pilot ticks or 10 commits. When it is hit, stop with `NEEDS_HUMAN` and a one-paragraph blocker in the task file instead of another attempt. Do not rescope a task inside its own spec; open a new task.
- **Review is lean.** `review.backend` is `none`; the host session checks diffs directly. The QA pipeline stage is off. Do not raise them without the owner.

## Code rules (owner, 2026-09-08)

The mantra is "Minimalist af, efficient af and beautiful". Typed Rust and TypeScript only under `src` and `crates`; no untyped JavaScript in production. Readable line widths, functions that do one thing, files under about 400 lines. Nothing is copied from a prototype or experiment without a rewrite and a test. Presets are value tables; no species or template branch in generator or renderer.
