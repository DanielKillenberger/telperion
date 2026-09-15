# Telperion

A runtime tree generator: space colonization for the crown, botanical rules below the crossover, one bias field for everything supernatural. Strategy lives in `STRATEGY.md`; specs and tasks in `.flow/` via `flowctl`.

<!-- flow-next:model-routing:start -->
## Model routing

<!-- Grammar: <tier>: <model>   or   <tier>: <model> at <effort>
     Resolution at each dispatch site: an explicit instruction in the moment,
     then this block, then the agent definition's own default, then the session
     model. A model this harness cannot reach falls back to the session model
     with one note. -->

implementer: gpt-6-astra at high
reviewer-note: host session checks diffs directly; no review backend
reviewer: claude-fable-5-1

Owner routing (2026-09-12, Astra quota back): dispatch Codex gpt-6-astra at high effort for all implementation through a shell-out to the codex CLI from the task's workspace; the host Fable session checks each diff itself and no cross-model review backend runs. One task per spec: the owner found that splitting a spec into several tasks lowers the outcome with a strong implementer, so a spec's plan lives in its body and work mints one implicit task. Between 2026-09-08 and 2026-09-11, while the Astra quota was exhausted, Claude Opus implemented; before that (2026-09-07) gpt-6-astra at low. Species specs (rendered from `templates/species-spec.md`, value tables over a supported form) route to the value tier: cursor-agent with `cursor-grok-4.6-high-fast`, one run on the spec's own branch with the long-task brief, the host Fable session checking the diff and running the gates; fn-34 (2026-09-14) was the first and took 36 minutes for two species. Generator gap specs, and everything else, stay on the frontier tier above; Grok is not dispatched outside species specs. Generator parameters are primary; templates are reusable parameter presets, and supported parameter changes must require no renderer code changes.

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

<!-- BEGIN FLOW-NEXT -->
<!-- flow-next:snippet:v2 -->
## Flow-Next

This project uses Flow-Next for ALL task tracking. `flowctl` comes from the flow-next plugin install — every flow-next skill resolves it itself, and on Claude Code it is also on PATH. Do NOT create markdown TODOs or use TodoWrite. Cold session: `flowctl brief` first — one bounded call (specs, ready tasks, memory); go deeper with `show`/`cat`/`anchor <task-id>`.

- Lifecycle: `flowctl list` / `show fn-N.M` / `start fn-N.M` / `done fn-N.M --summary-file s.md --evidence-json e.json` (e.json: `{"commits": ["<sha>"], "tests": ["<cmd>"], "prs": []}`)
- BEFORE any other flowctl operation, or when unsure of a flag: run `flowctl usage` (CLI cheatsheet + orchestration recipes) or `flowctl --help`.
- BEFORE bridging work to another model/CLI (`codex exec`, `cursor-agent`, `claude -p`, `grok`) or picking an implementation/review model: run `flowctl usage` and follow "Orchestration & model steering" exactly.
- Creating a spec: write it directly — `/flow-next:plan` is task breakdown only. `flowctl spec create --title "Short title" --plan-file plan.md --json`, then `/flow-next:plan <spec-id>`. Scaffold cascade (first match wins): `SPEC.md` -> `spec.md` -> bundled template.
- Substantial replies (reports, reviews, multi-section answers): invoke `/flow-next:prose` BEFORE drafting — the artifact prose contract applies to chat replies too. Short conversational turns skip it.
- If `flowctl` is not found: your shell lacks the plugin's `scripts/` dir on PATH (only Claude Code injects it). Resolve it the way the skills do - the plugin install's `scripts/flowctl` (Claude/Droid: plugin-root env var; Codex: `${CODEX_HOME:-$HOME/.codex}/scripts/flowctl`; Cursor/Grok: two levels above any flow-next SKILL.md) - or update/reinstall the flow-next plugin. A repo with no `.flow/` yet: run `/flow-next:setup`.
<!-- END FLOW-NEXT -->
