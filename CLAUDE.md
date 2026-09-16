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

Owner routing (2026-09-12, Astra quota back): dispatch Codex gpt-6-astra at high effort for all implementation through a shell-out to the codex CLI from the task's workspace; the host Fable session checks each diff itself and no cross-model review backend runs. One task per spec: the owner found that splitting a spec into several tasks lowers the outcome with a strong implementer, so a spec's plan lives in its body and work mints one implicit task. Between 2026-09-08 and 2026-09-11, while the Astra quota was exhausted, Claude Opus implemented; before that (2026-09-07) gpt-6-astra at low. From 2026-09-14 12:07 until the Astra quota resets on 2026-09-19, Claude Opus implements directly in-host again (no bridge, one worker per task, returning once), starting with fn-31's open items. Grok 4.6 is allowed for tasks it can complete, reached through `cursor-agent -p --force --model cursor-grok-4.6-high-fast` from the task's workspace (never the grok build CLI). Generator parameters are primary; templates are reusable parameter presets, and supported parameter changes must require no renderer code changes.

<!-- The owner's choice (2026-09-05): Codex gpt-6-astra implements at low
     effort. Per-task code review is not wanted: run work with --review=none.
     When a code review is wanted, it is host-native on Fable (the reviewer pin
     above, a different family from the writer). Since 2026-09-08 the plan
     review and spec completion review also run host-native on Fable
     (`.flow/config.json` review.backend = none since the owner asked for no cross-model review); the Codex
     high-effort review was a major quota sink. The live QA pass
     (pipeline.qa) is off; run /flow-next:qa by hand when a spec is ready. -->
<!-- flow-next:model-routing:end -->

### Bridged worker recipe (owner, 2026-09-14)

One codex session does the whole task in one persistent context window; the worker never splits the task into phases and never implements, investigates, fetches sources or calibrates itself. After `flowctl anchor <task-id> --md` and `git rev-parse HEAD` for the base, the worker composes a pointer prompt: the task id, the spec path, the spec's `## Resolved via Research` section or the research section of the spec it was split from, `CLAUDE.md`, the instruction to re-anchor on the repo, investigate, source and checksum references, implement, test and render, and the long-task brief from `flowctl usage` (flow-next PR #436, closes the owner's issue #431): commit each completed scope unit as a checkpoint on the named branch; never push, rebase, amend, change scope, issue a verdict or spawn an agent; return only when the scope is done or blocked, with the commit list. No timebox and no "stop if you run out of room" line in the brief. It runs that prompt once, in the foreground, from the asserted repo root, with the model and effort inline, stdin from `/dev/null`, `-o` for the digest, network access on, and no sandbox so codex can commit and reach the GPU (the checkout on its own branch is the boundary; `workspace-write` keeps `.git` read-only and segfaults the renderer). Every follow-up round, gate output, review findings, an owner answer, goes back into the same session with `codex exec resume <session-id> "<prompt>"`, never a fresh `codex exec`.

The worker then reviews the range from the recorded base, runs the gates on it, and completes the task; a spec rule that needs the owner (a tolerance miss, a verdict slot) is recorded in the task file as `NEEDS_HUMAN` instead of `flowctl done`.

## Token and evidence budget (owner, 2026-09-08)

fn-13 task 5 consumed a full weekly quota on 22 full-forest GPU captures and image inspection. These rules bind every agent and the pilot loop:

- **Small before large.** Never start a 1,024-tree capture until the specific defect reproduces and is fixed on the 8-tree forest with a red/green test. At most one full-forest capture per commit.
- **Read summaries, not receipts.** Agents read `OUTCOME.json`, `INSPECTION.md` and `metrics.json`. Never open `receipt.json`, videos or frame sequences; never view more than four images per capture. Raw receipts, videos and frame directories are gitignored and stay on disk.
- **No commit budget (owner, 2026-09-15).** A task keeps going for as long as its work is achievable within its spec's boundaries. It stops with `NEEDS_HUMAN` only for a decision that belongs to the owner or a bound the spec sets. Do not rescope a task inside its own spec; open a new task.
- **Review is lean.** `review.backend` is `none`; the host session checks diffs directly. The QA pipeline stage is off. Do not raise them without the owner.

## Code rules (owner, 2026-09-08)

The mantra is "Minimalist af, efficient af and beautiful". Typed Rust and TypeScript only under `src` and `crates`; no untyped JavaScript in production. Readable line widths, functions that do one thing, files under about 400 lines. Nothing is copied from a prototype or experiment without a rewrite and a test. Presets are value tables; no species or template branch in generator or renderer.

No hardcoded caps on generation (owner, 2026-09-15). A count or limit that changes what the generator grows is a configurable parameter with a validated range that presets author, never a constant hardcoded apart from those ranges. Tests assert form, not counts: no node, leaf or population ceiling gates a result. Where a tree needs more work, improve performance instead of capping it; build cost is measured and reported, not a gate on form.

## TypeSafe (owner, 2026-09-16)

TypeSafe's Jev model is installed as the `typesafe:typesafe-ai` skill: `POST https://api.typesafe.ai/v1/systemone` with `Authorization: Bearer $TYPESAFE_API_KEY`. The key sits in `~/.bashrc` below its non-interactive guard (`[[ $- != *i* ]] && return`), so a plain shell never sees it; call through `bash -ic '...'`. Never read, echo or copy the key.

Jev never runs in generation. No model call runs in the generator, the renderer, or any path a build or a preset load reaches: pin tests, byte-identical presets and seeded streams all depend on determinism. It works offline, on evidence about trees and on the owner's notes, and a preset stays a fixed table of values.

Jev selects; it never supplies a number. It takes text and JSON only, and answers a Choice, a Noul or a Score over options code gives it. Code proposes every candidate value and owns every calculation, render and measurement. Two uses fit (owner, 2026-09-16). Screening source literature: Jev judges which candidates are a measured value at a stated age and under what growing condition, and code copies the number; every such value is checked against its source text before it reaches a preset. Tuning presets: Jev maps each of the owner's written verdict notes to the preset rows that answer it and the direction to move them, and may rank candidate values code proposed from their measured comparison; the value that ships is one code proposed and a render measured. First use: OWIC's "1 to 2 ft per year for trees 10 to 30 years old", cited as O1 behind the oak's 2.4 m ten-year height, is a site-classification criterion, not a measurement.

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
