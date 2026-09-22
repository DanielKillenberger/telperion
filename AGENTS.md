## Generator evolution (owner, 2026-09-20)

The generator is under active development. Fidelity and measured speed improvements may change seeded specimens, random sequences, topology and encodings. Prefer byte-identical output for performance improvements when practical because exact comparison simplifies verification; this preference must not block a worthwhile measured gain that passes visual and correctness checks. Use byte-equivalence checks where unchanged output is intended, including repeat runs, unaffected paths and behavior-preserving optimizations. Intentional improvements may change both the generated structure and its byte representation; historical hashes and cross-backend equality must not prevent that evolution. See STRATEGY.md, "Our approach" and "Attributability". Do not ask again for permission merely because an authorized improvement changes those outputs. Performance optimizations that change bytes are acceptable when measurements demonstrate the gain, visual comparison shows no perceptible regression at the supported views and in motion where applicable, and relevant correctness requirements still hold. Byte mismatch alone is not a failure in that case. Update affected tests and baselines with evidence of botanical validity, sound geometry, visual quality and cost; retain meaningful repeatability checks within the stated implementation and explicit correctness contracts such as spatial contacts. This policy supersedes blanket output-preservation and re-pin permission language in older specs, while retaining scoped equivalence checks where their premise still holds, but does not waive their remaining product requirements or declare an unbuilt feature complete.

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

## TypeSafe

TypeSafe's Jev model (`typesafe:typesafe-ai` skill) screens source literature for measured values; it never runs in the generator, the renderer, or any path a preset reaches. The key is `$TYPESAFE_API_KEY`, reachable only through `bash -ic`; never read or echo it. Code finds and copies every number, and each is checked against its source before it reaches a preset. Full rule in `docs/typesafe.md` and in `CLAUDE.md` under "TypeSafe".

## Pull requests (owner, 2026-09-20)

The format and the procedure are `docs/pr-format.md`. Every run of `/flow-next:make-pr`, by hand or under `flow --auto`, follows that file in place of the skill's body phases and does not read the skill's `workflow.md` or its companion files. The body is Change, Proof, Look here, Decisions and Open, 1,500 characters for a small diff and at most 4,000 for a large one, ending in the make-pr marker. The reason: bodies had reached 57 KB, and fn-72 recorded about 30k tokens of skill reading before a three-criterion fix could open its PR.

## Friction reports (owner, 2026-09-18)

Every agent on a build reports friction as it happens, in `.flow/evidence/<spec>/FRICTION.md`, one dated entry per report: what it was doing, what slowed or hindered it, what it cost in minutes, ticks or tokens, and what would have removed it (a missing flag, a hand step, a slow gate, a wait that dwarfed the work). A report is written the moment progress slows, never reconstructed at the end. When the slowness is obviously inefficient, the agent returns early with the report and `NEEDS_HUMAN` instead of pushing through; a build that burned its budget on a known inefficiency has broken this. At the end of a build, before the spec closes, the host reads every FRICTION.md entry of that spec, brings each one up in its report, and proposes a fix for it, as its own spec or as a line in an open one; the owner decides which proposals become specs, and the host writes none of them on its own (owner, 2026-09-18). A local setup problem on the owner's machine is reported, never specced, because it does not belong in the repository. The rule exists because fn-13 task 5 spent a weekly quota on captures nobody had flagged as slow and fn-34 ran 23 rounds before anyone wrote down that the loop had no finish line.

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
