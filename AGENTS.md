# Telperion

A runtime tree generator: space colonization for the crown, botanical rules below the crossover, one bias field for everything supernatural. Strategy lives in `STRATEGY.md`; specs and tasks in `.flow/` via `flowctl`.

## Dispatch and escalation (owner)

One task per spec: the owner found that splitting a spec into several tasks lowers the outcome with a strong implementer, so a spec's plan lives in its body and work mints one implicit task. Generator parameters are primary; templates are reusable parameter presets, and supported parameter changes must require no renderer code changes.

Reasoning and system design escalate to the host (owner, 2026-09-19). A dispatched agent executes; it does not reason about the system. Such an agent carries the mechanical work of a species run: the seed and manifest draft, the literature stages, the readings the pipeline's question sets judge, the packet and the report. Anything that takes reasoning or system design stops and goes up to the host session conducting the run: the capability assessment that decides what the generator cannot express, gap analysis and the candidate fixes written for a gap, the shape of any spec a gap mints, and any judgment about how a part of the system should work. The host has the context and the authority; a dispatched agent has neither, and its guess is indistinguishable from an answer once it is written into an artifact. The reason for the line sitting here: a wrong reading costs one source and the next stage catches it, while a wrong design judgment sends the swarm to build the wrong thing, or parks a species the generator could already draw. Escalation is not failure and costs nothing; a cheap driver that reaches such a step reports it and stops.

## Token and evidence budget (owner, 2026-09-08)

fn-13 task 5 consumed a full weekly quota on 22 full-forest GPU captures and image inspection. These rules bind every agent and the pilot loop:

- **Small before large.** Never start a 1,024-tree capture until the specific defect reproduces and is fixed on the 8-tree forest with a red/green test. At most one full-forest capture per commit.
- **Read summaries, not receipts.** Agents read `OUTCOME.json`, `INSPECTION.md` and `metrics.json`. Never open `receipt.json`, videos or frame sequences; never view more than four images per capture. Raw receipts, videos and frame directories are gitignored and stay on disk.
- **Review is lean.** `review.backend` is `none`; the host session checks diffs directly. Do not raise it without the owner. The QA pipeline stage is on `auto` (owner, 2026-09-19): `flow --auto` puts the spec's acceptance to Jev's `qa-gate` preset and drives the harness only when the answer is UI-observable and code resolves a startable target, so a generator, CLI or preset spec still skips it. A QA pass drives `npm run dev`, never a full-forest capture; the capture budget above is unchanged.

## Generator evolution (owner, 2026-09-20)

The generator is under active development. Fidelity and measured speed improvements may change seeded specimens, random sequences, topology and encodings. Prefer byte-identical output for performance improvements when practical because exact comparison simplifies verification; this preference must not block a worthwhile measured gain that passes visual and correctness checks. Use byte-equivalence checks where unchanged output is intended, including repeat runs, unaffected paths and behavior-preserving optimizations. Intentional improvements may change both the generated structure and its byte representation; historical hashes and cross-backend equality must not prevent that evolution. See STRATEGY.md, "Our approach" and "Attributability". Do not ask again for permission merely because an authorized improvement changes those outputs. Performance optimizations that change bytes are acceptable when measurements demonstrate the gain, visual comparison shows no perceptible regression at the supported views and in motion where applicable, and relevant correctness requirements still hold. Byte mismatch alone is not a failure in that case. Update affected tests and baselines with evidence of botanical validity, sound geometry, visual quality and cost; retain meaningful repeatability checks within the stated implementation and explicit correctness contracts such as spatial contacts. This policy supersedes blanket output-preservation and re-pin permission language in older specs, while retaining scoped equivalence checks where their premise still holds, but does not waive their remaining product requirements or declare an unbuilt feature complete.

## Code rules (owner, 2026-09-08)

The mantra is "Minimalist af, efficient af and beautiful". Typed Rust and TypeScript only under `src` and `crates`; no untyped JavaScript in production. Readable line widths, functions that do one thing, files under about 400 lines. Nothing is copied from a prototype or experiment without a rewrite and a test. Presets are value tables; no species or template branch in generator or renderer.

## Mature trees are the product (owner, 2026-09-18)

The direct build (`mesh::build`, `branching::generate`) is the product. The harness, the headless stills, `species:qa`, the numeric protocol and every owner verdict draw and judge that tree. The growth path (fn-11, fn-30, the specimen grown to an age) is a hidden feature: reachable in the harness only behind `?growth=1`, kept buildable and pinned, never a default and never a gate on species work. A spec that routes production through growth needs the owner's word first; the fn-31 branch's "route production through growth" predates this rule and is superseded for master. The reason: on 2026-09-17 the birch at seed 1 was 73,337 nodes on the direct build and 590,410 on the growth path, and every fn-34 verdict had been taken on the path the harness did not show.

One species per spec (owner, 2026-09-16). A new real species is onboarded by its own spec, never bundled with another. fn-34 took beech, ash and birch together, grew fifteen capability dependencies and ran 23 rounds before the ash moved to fn-56 and the beech to fn-62. A species spec is one authored manifest, one run of the onboarding method, its decisions resolved, and a checklist the owner ticks, as fn-62 does; the method itself lives in `docs/species-onboarding.md` and, once fn-58 lands, in the pipeline, never restated per species. A generator gap a species needs is its own spec that the species spec depends on.

## Friction reports (owner, 2026-09-18)

Every agent on a build reports friction as it happens, in `.flow/evidence/<spec>/FRICTION.md`, one dated entry per report: what it was doing, what slowed or hindered it, what it cost in minutes, ticks or tokens, and what would have removed it (a missing flag, a hand step, a slow gate, a wait that dwarfed the work). A report is written the moment progress slows, never reconstructed at the end. When the slowness is obviously inefficient, the agent returns early with the report and `NEEDS_HUMAN` instead of pushing through; a build that burned its budget on a known inefficiency has broken this. At the end of a build, before the spec closes, the host reads every FRICTION.md entry of that spec, brings each one up in its report, and proposes a fix for it, as its own spec or as a line in an open one; the host specs and builds an obvious fix itself, one whose cause the entry names and whose remedy changes no product behaviour or owner decision (owner, 2026-09-23), and brings the rest to the owner, who decides which become specs (owner, 2026-09-18). A local setup problem on the owner's machine is reported, never specced, because it does not belong in the repository. The rule exists because fn-13 task 5 spent a weekly quota on captures nobody had flagged as slow and fn-34 ran 23 rounds before anyone wrote down that the loop had no finish line.

## Gates and checked claims (owner, 2026-09-20)

The local gate is `cargo test --profile ci --workspace --no-fail-fast`, the profile CI runs under nextest. It is run once, at the end of a task. `npm run rust:test` and any other `--release` suite are not the gate and are never run as a second proof: the two profiles differ only in link-time optimisation, and fn-87 spent ten minutes re-proving a green result under fat LTO.

A spec rests only on claims somebody ran. A defect spec's repro is run as a standalone test before an acceptance criterion is written around it, and an architecture claim about existing code is checked against that code before the spec is marked ready; a claim that was not checked is written as unknown. fn-92's first R1 asked a new test to be red on the base, and it crashed 0 of 65 runs while `bark_plates` crashed 5 of 12, which cost 35 of the task's 60 minutes. An unchecked architecture claim in fn-87 cost two bridge dispatches.

Two checkouts of the same crate never share a `target/` directory. Test binary names do not depend on the checkout path, so cargo reuses the other checkout's binary without rebuilding: fn-92 counted four crashes against the fix that most likely came from the base binary.

## Design principles (owner, 2026-09-25)

STRATEGY.md's "Our approach" is enforced by structure, not policing; `docs/principles.md` is the guide, and Jev review of specs and designs is planned in fn-156. Four questions steer every decision:

- Does every tree still pass through the one pipeline, with no copy of a stage outside it?
- Does an input the pipeline cannot draw fail with an error, never take a fallback path?
- Does every parameter change the tree by degree, dormant where its structure is absent, never a switch between ways of building?
- Is each new output read by a consumer, each cost measured, and each new stop one that catches what its neighbours cannot?

Three things hold them: the one pipeline, whose stages become private to it in fn-152 so a second chain does not compile; an ordinary test that builds every shipped preset's every artifact through the pipeline and through each package entry; and CI's size budget on every shipped artifact. A sanctioned trade-off names one of the exceptions `docs/principles.md` lists; a claim of approval without one is none.

## Pull requests (owner, 2026-09-20)

The format and the procedure are `docs/pr-format.md`. Every run of `/flow-next:make-pr`, by hand or under `flow --auto`, follows that file in place of the skill's body phases and does not read the skill's `workflow.md` or its companion files. The body is Change, Proof, Look here, Decisions and Open, 1,500 characters for a small diff and at most 4,000 for a large one, ending in the make-pr marker. The reason: bodies had reached 57 KB, and fn-72 recorded about 30k tokens of skill reading before a three-criterion fix could open its PR.

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

## Evidence retention

Follow `docs/evidence-retention.md` when collecting or staging evidence. Keep final summaries and reusable verification sources; put raw run output in ignored `.flow/evidence/<spec>/raw/`. Inspect evidence count and size before a PR. Never remove executable test fixtures as if they were generated output.

<!-- flow-next:model-routing:start -->
## Model routing

<!-- Scaffolded by /flow-next:setup as an EXAMPLE to edit. Every routing line
     below is commented out, so nothing is routed until you uncomment one.
     These are your preferences to fill in - never detected facts. flow-next
     does not know which models your account serves and never writes one here. -->

<!-- Grammar: <tier>: <model>   or   <tier>: <model> at <effort>
     Name the model ids YOUR harness and account actually serve - ask the
     harness for its list, then invoke one; ids change and vary per account. -->

<!-- reviewer: <model>                  - anything grading work someone else
     produced. Prefer a different family than the writer: a same-family review
     is not an independent verdict. Advice, not enforcement. -->
<!-- implementer: <model> at <effort>   - work handed to another harness (plan
     here, implement cheaper or faster there). Absent = the session model
     implements. -->
<!-- fast scout: <model>                - mechanical inventory scanning, where
     the cheapest tier is the correct one. -->
<!-- thinking scout: <model>            - analysis that degrades badly on a
     fast tier. -->

<!-- Unset is the default and the doctrine: planning, capture, interview,
     requirement analysis, every verdict, and the worker run on the session
     model. Effort strings pass through to the host untranslated. -->

<!-- Resolution at each dispatch site: an explicit instruction in the moment,
     then this block, then the agent definition's own default, then the session
     model. A model this harness cannot reach falls back to the session model
     with one note - routing never fails closed, and nothing here is validated. -->
<!-- flow-next:model-routing:end -->
