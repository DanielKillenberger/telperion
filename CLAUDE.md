# Telperion

A runtime tree generator: space colonization for the crown, botanical rules below the crossover, one bias field for everything supernatural. Strategy lives in `STRATEGY.md`; specs and tasks in `.flow/` via `flowctl`.

## Dispatch and escalation (owner)

One task per spec: the owner found that splitting a spec into several tasks lowers the outcome with a strong implementer, so a spec's plan lives in its body and work mints one implicit task. Generator parameters are primary; templates are reusable parameter presets, and supported parameter changes must require no renderer code changes.

Reasoning and system design escalate to the host (owner, 2026-09-19). A dispatched agent executes; it does not reason about the system. Such an agent carries the mechanical work of a species run: the seed and manifest draft, the literature stages, the readings the pipeline's question sets judge, the packet and the report. Anything that takes reasoning or system design stops and goes up to the host session conducting the run: the capability assessment that decides what the generator cannot express, gap analysis and the candidate fixes written for a gap, the shape of any spec a gap mints, and any judgment about how a part of the system should work. The host has the context and the authority; a dispatched agent has neither, and its guess is indistinguishable from an answer once it is written into an artifact. The reason for the line sitting here: a wrong reading costs one source and the next stage catches it, while a wrong design judgment sends the swarm to build the wrong thing, or parks a species the generator could already draw. Escalation is not failure and costs nothing; a cheap driver that reaches such a step reports it and stops.

## Token and evidence budget (owner, 2026-09-08)

fn-13 task 5 consumed a full weekly quota on 22 full-forest GPU captures and image inspection. These rules bind every agent and the pilot loop:

- **Small before large.** Never start a 1,024-tree capture until the specific defect reproduces and is fixed on the 8-tree forest with a red/green test. At most one full-forest capture per commit.
- **Read summaries, not receipts.** Agents read `OUTCOME.json`, `INSPECTION.md` and `metrics.json`. Never open `receipt.json`, videos or frame sequences; never view more than four images per capture. Raw receipts, videos and frame directories are gitignored and stay on disk.
- **Per-task budget.** A task gets 5 pilot ticks or 10 commits. When it is hit, stop with `NEEDS_HUMAN` and a one-paragraph blocker in the task file instead of another attempt. Do not rescope a task inside its own spec; open a new task.
- **Review is lean.** `review.backend` is `none`; the host session checks diffs directly. Do not raise it without the owner. The QA pipeline stage is on `auto` (owner, 2026-09-19): `flow --auto` puts the spec's acceptance to Jev's `qa-gate` preset and drives the harness only when the answer is UI-observable and code resolves a startable target, so a generator, CLI or preset spec still skips it. A QA pass drives `npm run dev`, never a full-forest capture; the capture budget above is unchanged.

## Code rules (owner, 2026-09-08)

The mantra is "Minimalist af, efficient af and beautiful". Typed Rust and TypeScript only under `src` and `crates`; no untyped JavaScript in production. Readable line widths, functions that do one thing, files under about 400 lines. Nothing is copied from a prototype or experiment without a rewrite and a test. Presets are value tables; no species or template branch in generator or renderer.

## Mature trees are the product (owner, 2026-09-18)

The direct build (`mesh::build`, `branching::generate`) is the product. The harness, the headless stills, `species:qa`, the numeric protocol and every owner verdict draw and judge that tree. The growth path (fn-11, fn-30, the specimen grown to an age) is a hidden feature: reachable in the harness only behind `?growth=1`, kept buildable and pinned, never a default and never a gate on species work. A spec that routes production through growth needs the owner's word first; the fn-31 branch's "route production through growth" predates this rule and is superseded for master. The reason: on 2026-09-17 the birch at seed 1 was 73,337 nodes on the direct build and 590,410 on the growth path, and every fn-34 verdict had been taken on the path the harness did not show.

One species per spec (owner, 2026-09-16). A new real species is onboarded by its own spec, never bundled with another. fn-34 took beech, ash and birch together, grew fifteen capability dependencies and ran 23 rounds before the ash moved to fn-56 and the beech to fn-62. A species spec is one authored manifest, one run of the onboarding method, its decisions resolved, and a checklist the owner ticks, as fn-62 does; the method itself lives in `docs/species-onboarding.md` and, once fn-58 lands, in the pipeline, never restated per species. A generator gap a species needs is its own spec that the species spec depends on.

## Friction reports (owner, 2026-09-18)

Every agent on a build reports friction as it happens, in `.flow/evidence/<spec>/FRICTION.md`, one dated entry per report: what it was doing, what slowed or hindered it, what it cost in minutes, ticks or tokens, and what would have removed it (a missing flag, a hand step, a slow gate, a wait that dwarfed the work). A report is written the moment progress slows, never reconstructed at the end. When the slowness is obviously inefficient, the agent returns early with the report and `NEEDS_HUMAN` instead of pushing through; a build that burned its budget on a known inefficiency has broken this. At the end of a build, before the spec closes, the host reads every FRICTION.md entry of that spec, brings each one up in its report, and proposes a fix for it, as its own spec or as a line in an open one; the owner decides which proposals become specs, and the host writes none of them on its own (owner, 2026-09-18). A local setup problem on the owner's machine is reported, never specced, because it does not belong in the repository. The rule exists because fn-13 task 5 spent a weekly quota on captures nobody had flagged as slow and fn-34 ran 23 rounds before anyone wrote down that the loop had no finish line.

## TypeSafe (owner, 2026-09-16)

The usage guideline is `docs/typesafe.md`. Tools live in `crates/telperion-jev`. Jev may run in evidence tooling, research checks, QA triage and report assembly. It never runs in generation, rendering, presets, the Wasm bindings, the browser source, or any test on the workspace test commands.

TypeSafe's Jev model is installed as the `typesafe:typesafe-ai` skill: `POST https://api.typesafe.ai/v1/systemone` with `Authorization: Bearer $TYPESAFE_API_KEY`. The key sits in `~/.bashrc` below its non-interactive guard (`[[ $- != *i* ]] && return`), so a plain shell never sees it; call through `bash -ic '...'`. Never read, echo or copy the key. Every call goes through the shared caller and leaves a ledger entry.

Jev selects; it never supplies a number. Code extracts every candidate and owns every calculation, render and measurement. Every question offers a no-match answer. Candidate coverage is tested before a selection is trusted. Thresholds come from the labelled set in `crates/telperion-jev/data`. A judgment proposes and never writes a memory entry, a receipt, a finding or an owner verdict.

Two uses fit (owner, 2026-09-16). Screening source literature: Jev judges which candidates are a measured value at a stated age and under what growing condition, and code copies the number; every such value is checked against its source text before it reaches a preset. Tuning presets: Jev maps each of the owner's written verdict notes to the preset rows that answer it and the direction to move them, and may rank candidate values code proposed from their measured comparison; the value that ships is one code proposed and a render measured. First use: OWIC's "1 to 2 ft per year for trees 10 to 30 years old", cited as O1 behind the oak's 2.4 m ten-year height, is a site-classification criterion, not a measurement.

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
