<!-- Read-only inventory of the species runner, 2026-09-24, fn-80 branch at de00bd35; host-commissioned for the lean runner spec. -->
# Species runner inventory (worktree fn-80)

All paths below are relative to `/home/daniel/Projects/telperion/.worktrees/fn-80-the-gap-loops-first-live-run/crates/telperion-jev/src/` unless they start with `scripts/` or `data/`. The code in scope is about 34,000 lines, plus about 2,600 lines of scripts.

## 1. Stage list (one species, in execution order)

**`species-conductor step`** (`bin/species_conductor.rs` → `conductor/cli.rs:63` → `step::drive`, `conductor/step.rs:278`). Each call reads `plan::next` (`conductor/plan.rs:215`) and runs one hop. The order is fixed:

1. an existing pause;
2. an open dispatch;
3. stale stages (the build id changed);
4. search-again;
5. owner-first decisions;
6. a dependency;
7. a landed fix whose stages have not rerun;
8. a decision that blocks a stage (routine, gap loop or owner);
9. a missing stage artifact;
10. a changed stage fingerprint;
11. a tuning revision;
12. the gap check;
13. the packet, then ready.

**Stages.** The conductor spawns `species-pipeline <stage> --dir D --run-dir R` (`step.rs:55`). Stage artifacts go to `D/<stage>.json` (`pipeline/stage.rs:84`).

| # | Stage | Writes | Read by |
|---|---|---|---|
| 1 | discover | `discover.json`; files a `manifest-proposed` decision; admission can rewrite `manifest.json` sources and `resolutions.json` (`stages/discover.rs:187`) | `manifest.json` → every later stage |
| 2 | fetch | `fetch.json`, `R/cache/firecrawl/…` | extract, screen, quality, select, verify, fit, document, report |
| 3 | extract | `extract.json` | screen |
| 4 | screen | `screen.json` | quality, select |
| 5 | quality | `quality.json` | select, fit, report |
| 6 | select | `select.json`, `packet/profile.json`, `packet/references.json`, `provenance.json` | verify (`verify.rs:53`), gate, generate, document; conductor `overlay.rs:41` |
| 7 | verify | `verify.json` | report |
| 8 | fit | `fit.json` | generate (optional, `generate/mod.rs:137`), report |
| 9 | gate | `gate.json`; reads `packet/capability.json`, which the host writes | generate; conductor `finish.rs:354` |
| 10 | generate | `generate.json`, `packet/species.json`, `packet/specimens.json`, `R/stills/` | document, report, gate (specimens) |
| 11 | document | `document.json`, `catalogue/<sp>/sources/*.md`, `ARTICLE.md` (from node scripts), `R/document/<id>.json` | report, conductor packet (`packet.rs:137`) |
| 12 | report | `report.json`, `report.md` | conductor packet (`packet.rs:43`) |

**Side paths**

- **search-again** (`step.rs:73`, `pipeline/search/mod.rs:47`) writes `D/search-rounds.json`, `manifest.json` and `resolutions.json`. The conductor plan reads them (`plan.rs:245`, `step.rs:264`).
- **Every pipeline command** appends to `R/command-log.json` (`stage.rs:384`). Only swap reads it (`swap.rs:68`).
- **The gap loop.** The conductor never runs it. `Next::GapLoop` opens a routine dispatch (`step.rs:326`), and an agent runs `species-pipeline gap open|options|route|spec|review|resume`. That writes `D/gaps/<slug>/gap.json`, which `plan.rs:164` reads. `gap metrics` writes `D/metrics.json`, which report (`report.rs:67`) and packet (`packet.rs:51`) read.

**Tuning revision.** `conductor/tuning.rs:50`:

1. Checks the revision cap and asks the continuation question if the revision is 2 or later.
2. `overlay::refresh` writes `initial_overrides` into the tuning config, plus `<config>.derived.json`.
3. Spawns `tuning-loop run --config C --out conductor/tuning-N/` (`step.rs:95`).
4. `settle` (`tuning.rs:180`) reads `tuning-N/run.json` and `result.json`.

Inside `tuning::command::run_with` (`tuning/command.rs:276`):

| Phase | Where | Writes |
|---|---|---|
| lock, prepare/resume | `command.rs:289`, `:59` | `run.lock`, `run.json` |
| calibration and convergence check | `config.verify()`, `live.rs:269` (convergence proof `live.rs:376-425`) | pause in `run.json` |
| pilot authority | `command.rs:358` | pause |
| preparation charge | `command.rs:367` | `run.json` |
| baseline | `engine.rs:584` (evaluate → `matched.rs` headless + `compare-references.py`) | trial in `run.json` |
| visual pass | `engine.rs:511` `assess` → `live.rs:464` → `reference_first::assess` / `vision::Adapter::assess` | adapter ledger record (receipt) |
| receipt | `reference_first.rs:415` bind, `tidy.rs`, `repair.rs` (one repair re-call) | the same ledger |
| priority review | `engine.rs:334` `priority_gate` (checkpoint + `objectives::offer`) | `priority-review.json` (`command.rs:314`) |
| rounds | `engine.rs:616-795`: runaway check, `route_remaining`, `settle_round_boundary`, proposal batches, then bundle (`bundle/round.rs`) or single candidates, veto | `run.json`, `handoffs.json` |
| result | every save, `command.rs:334` | `result.json`, `RESULT.md`, `RESULT.html`, `finalists.json` |
| convergence (conductor side) | `finish::converged`, `conductor/finish.rs:24` | the conductor's `run.json` |

After the revision the conductor runs the **gap check** (`step.rs:165`, `gapcheck.rs:148`). A reachable gap goes back to tuning, a covered gap becomes a dependency (`handoff::attach`), and a new gap is packaged to `conductor/gaps/<id>.json` and pauses. **Dependencies** go through `dependency::advance` (`dependency.rs:300`) and open dispatches. Then comes the **packet** (`packet.rs:40`), written to `conductor/packet.json`.

## 2. Module table

Classes: a = produces an artifact; b = validates or guards another stage's output; c = asks for or checks permission; d = test or fixture support.

### Binaries

| Module | Lines | Purpose | Called by | Class |
|---|---|---|---|---|
| `bin/species_pipeline.rs` | 283 | Stage dispatcher, swap, gap, search-again; prints SEARCH_AGAIN / NEEDS_HUMAN | the conductor (spawned) | a; c at `:155` |
| `bin/species_conductor.rs` | 21 | Wrapper around `conductor::cli` | host | — |
| `bin/tuning_loop.rs` | 185 | run, preflight, inventory, freeze-replay, result, freeze, calibrate, vision-replay | the conductor (`run` only) | a; c for calibrate at `:426` |
| `bin/jev.rs` | 275 | screen, select, cite, triage, ask, cases | host only | d (cases) |

### Conductor

| Module | Lines | Purpose | Called by | Class |
|---|---|---|---|---|
| `cli` | 189 | Subcommand surface | the bin | — |
| `mod` | 204 | Config, budget caps, paths | all | c: caps validated at `mod.rs:147` |
| `plan` | 377 | Next action, with no side effects | step, cli | c: owner-first `:70`, decision ownership `:109` |
| `step` | 352 | Runs one hop; spawns binaries | cli, tuning | a; c at `:217`, `:248` |
| `state` | 353 | `conductor/run.json` | all | c: resume check `:298` |
| `handoff` | 226 | Pause file plus resume and attach | step, dependency, tuning, cli | c at `:61`, `:197` |
| `tuning` | 311 | Runs, carries and settles one revision | step, cli | c at `:64`, `:102`, `:266` |
| `finish` | 81 | Converged rule, known gaps | tuning, packet | b at `:24` |
| `gapcheck` | 262 | Reachable, covered or new, plus the gap package | step, plan | a (package); b at `:148` |
| `dependency` | 548 | Design and implementation routing for a gap spec | step, cli, adopt | c at `:300-420` |
| `dispatch` | 181 | Dispatch record; refuses obsolete or interrupted results | several | b at `:126` |
| `policy` | 311 | `data/conductor-policy.json` route table | plan, dependency, cases | c at `:115` (dispatch routing) |
| `questions` | 222 | Conductor Jev sets, continuation | dependency, gapcheck, tuning | c at `:169` |
| `adopt` | 145 | Records host-built work as landed | cli | c |
| `packet` | 193 | Readiness packet | step, cli, plan | a; b at `:40-160` |
| `report` | 303 | Run measurements, `report.json` | cli only | a |
| `overlay` | 212 | Writes the derived `initial_overrides` | tuning | a |
| `derive` | 309 | Turns profile metrics into dial values | overlay | a |
| `cases` | 185 | Offline policy case scoring | cli `cases` | d |

### Pipeline

| Module | Lines | Purpose | Called by | Class |
|---|---|---|---|---|
| `stage` | 553 | Paths, idempotence key, stop on open decision, command log | everything | b/c: open-decision stop `:211` |
| `canon`, `build_id`, `tree_digest` | 229 | Canonical JSON, build digest | everything, `build.rs` | a support |
| `decision` | 436 | `decisions.json`; stale resolutions void | stages, plan | c at `:259` |
| `consume` | 382 | Which stage consumes which option; owner stops | stage, plan, bin | c at `:73`, `:169`, `:200` |
| `manifest` | 466 | Admitted manifest | stages | c (the human boundary, `:1-6`) |
| `admission` | 389 | Self-admission of sources by rights | discover, search | c at `:107`, `:127` |
| `rights` | 391 | Rights lookup plus the Jev class | admission | b at `:243` |
| `known` | 402 | Sources the repo already cites | discover | a |
| `judge` | 120 | Jev call plus ledger index | stages, conductor | a |
| `adapter/` (4 files) | 945 | Firecrawl CLI, fixtures, table parsing, content refusal | fetch, discover, search, rights | a; b at `adapter/content.rs` |
| `search/` | 467 | Requirement search rounds | bin, plan, step | a; c (round cap, `rounds.rs:59`) |
| `requirements` | 392 | Growth-form requirement table | manifest, quality, select | b |
| `floors` | 120 | Pick floors fitted to labels | pick | c (calibration) |
| `sets`, `sets/cases` | 814 | Question sets; case scoring | stages, jev | a / d |
| `curve` | 496 | Chapman-Richards fit | fit | a |
| `render` | 262 | Measurer / headless binaries | generate, tuning evaluation | a |
| `routes` | 396 | Described and reference value routes | generate | a |
| `cost` | 64 | Credits and calls | stage, report | a |
| `swap` | 223 | Compares two runs | the bin's `swap` | d |
| `gap/` (9 files) | 2636 | Gap loop: open, options, route table, rounds, resume, metrics | bin `gap`, plan (`gap_dir`, `HALT_KINDS`), report | c: `route.rs:171`, `table.rs`, `rounds.rs:156`, `resume.rs:122`; a: `metrics.rs:218` |
| `stages/discover, fetch, extract, screen, quality, select, fit, generate/, document, report` | ≈3,700 | The 12 stages | bin | a |
| `stages/verify` | 454 | Citation, obligations, structural checks | bin | b at `:39-380` |
| `stages/gate` | 386 | Capability, registry and seed gates | bin | b at `:86` |
| `stages/capability_class` | 104 | Identity or improvement class | gate | c: the host decides the class (`:1-12`, `:90`) |
| `stages/unavailable, flagged` | 203 | Refile decisions | fetch, select | c |
| `stages/appearance, pick, points, rows` | 755 | Select and quality helpers | select, quality | a |

### Tuning

| Module | Lines | Purpose | Called by | Class |
|---|---|---|---|---|
| `command` | 391 | Lock, prepare/resume, pre-dispatch pauses, save files | the bin | c at `:59-230`, `:350`, `:358` |
| `engine` | 809 | Round loop, reserve/settle, stop | command | a; c at `:334`, `:373`, `:387`, `:462` |
| `live` | 1060 | The real services, Jev asks, config verify | command, engine | a; c at `live.rs:269` (calibrations) |
| `continuation` | 373 | Basis, pause and human-decision types; `assess` | conductor, tuning | c at `:54`, `:114` |
| `caps` | 203 | Cap changes on resume | command | c |
| `calibration` | 417 | Frozen manifests; `qualified` | live, bin | c |
| `replay` | 209 | Reference-first replay | bin | c |
| `preflight` | 270 | Read-only plan | bin `preflight` | c (estimate) |
| `priority` | 309 | Checkpoint and owner approval | engine | c at `:200-230` |
| `objectives` | 209 | Objectives offered for approval | engine | c |
| `evidence` | 40 | Which revisions' evidence can be reused | engine, command | b |
| `runaway` | 111 | Pause after N rounds that kept nothing | engine | c at `:76` |
| `round` | 220 | Round boundary, repeat filter | engine | c at `:68`, `:96` |
| `routing` | 306 | Per-priority route plus pre-dispatch risk | engine | c at `:12`, `:72` |
| `handoff` (tuning) | 236 | Grounded fn-89 handoffs | routing | a |
| `judgments`, `digest`, `facts` | 703 | Jev states and question sets | live, engine | a |
| `direction` | 189 | Direction-mass acceptance | live | b |
| `stride/` | 316 | Gap-magnitude class | bundle | a; c at `decide.rs:70` |
| `bundle/` | 1274 | Bundle round, isolate, worse, tracks | engine | a |
| `sheet/` | 851 | Contact-sheet reviewer | bundle | a; b at `sheet.rs:356-411` |
| `progress/` | 781 | Side-by-side reviewer (Selection::Visual) | engine | a |
| `veto/` | 293 | Closing review can undo an adoption | engine, bundle | b |
| `vision` | 346 | Command adapter, receipt check | live, reference_first | b at `:191-240` |
| `reference_first` | 640 | Comparison request, bind, verify convergence | live | b at `:415`, `:480` |
| `tidy`, `repair` | 365 | Receipt tidiness and one repair call | reference_first, vision | b |
| `joint` | 247 | Hash-bound evidence packet | vision | b |
| `inventory` | 164 | Stage A reference inventory | bin `inventory` | a |
| `matched`, `evaluation` | 522 | Captures and metrics | live | a |
| `result/` | 544 | EndResult, gap list, MD/HTML | command, conductor, bin | a |
| `state` | 225 | Budget, Visual, `ready()` | all | c (caps, `:177-217`) |
| `actions` | 222 | Dial and candidate bounds | engine | b |
| `unexpressed` | 120 | Traits the generator cannot draw yet | result, finish | b |

### Scripts

- `scripts/*-codex.py` (contact-sheet, progress-review, reference-first, tuning-vision) and the Claude twins `*-claude.py`, which all go through `vision_claude.py`: class a. Which one runs is set in the tuning config; the palm config uses `*-claude.py` (`.flow/evidence/fn80/tuning-date-palm.json`).
- `test-*.py`: class d.
- `catalogue-sources.mjs` and `catalogue-article.mjs`: class a, run by document (`document.rs:27-28`, `:294`).
- `catalogue-check.mjs` and `catalogue-pages.mjs`: no Rust caller.
- `catalogue-docs.test.mjs`: class d.

## 3. Pause and decision points

**Conductor pauses** (`handoff::pause` writes `conductor/handoff-<id>.json` and `.md`):

| Reason | Where |
|---|---|
| "new gap: its cause and the shape of its spec are the host's" | `step.rs:217` |
| "the owner's decisions are open: …" | `step.rs:248` |
| policy human route (`why` from a policy row) | `dependency.rs:400` |
| "tuning revision cap reached" | `tuning.rs:64` |
| continuation reason (unavailable or unjustified) | `tuning.rs:102` |
| a carried tuning pause (same id and reason) | `tuning.rs:266` |

**Conductor waits** (not pauses):

- AwaitDispatch: `step.rs:287`
- AwaitOwner: `step.rs:301`
- AwaitLanding: `step.rs:305`
- Ready ("only their verdict accepts"): `step.rs:338`

**Continuation reasons** (`tuning/continuation.rs`):

- "continuation calibration unavailable": `:61`
- "missing bounded attempt basis or unknown prior usage": `:70`
- "hard budget exhausted": `:90`
- "stale or unattributed continuation assessment": `:94`
- "next attempt unjustified: human decision required": `:97`

**Tuning pauses** (in `run.json`):

| Reason | Where |
|---|---|
| "interrupted attempt; spend retained" | `command.rs:74` |
| calibration failure from `config.verify` | `command.rs:351` |
| "magnitude live efficacy unvalidated" (pilot authority) | `command.rs:359`, `engine.rs:377` |
| "Owner gap-priority review required…" | `engine.rs:365` |
| "reviewer passed all required cells; …bootstrap; owner look required" | `engine.rs:394` |
| "interrupted attempt; reservation retained" | `engine.rs:559` |

Every other error becomes a stop with the action "reassess or diagnose" (`engine.rs:567`). The main ones:

- "unknown judgment usage": `:496`
- "judgment exceeded reservation": `:507`
- "baseline infeasible": `:591`
- visual pass or round limit reached: `:630`, `:633`
- "round preflight cannot fit": `:659`
- "no supported proposal…": `:697`, `:700`, and `bundle/round.rs:356`, `:387`, `:390`, `:393`
- "more than four proposals refused": `:705`
- "runaway: N rounds in a row kept nothing": `runaway.rs:90`
- "uncertainty pause" and "fn-89 handoff": `routing.rs:65`, `:69`
- "hard budget exhausted": `round.rs:70`
- "repeat attempt unjustified": `round.rs:113`
- "stale or failed sheet response": `sheet/adapter.rs:45`

**Pipeline decisions** (open means owner, unless the policy lists it as routine):

| Kind | Where |
|---|---|
| manifest-proposed | `discover.rs:90` |
| unavailable-source | `unavailable.rs:45` |
| coverage-gap | `fetch.rs:228` |
| data-insufficient / requirements-unmet | `quality.rs:361` |
| requirements-unmet | `flagged.rs:98`, `appearance.rs:316` |
| claim-contradicted / claim-unsupported | `verify.rs:88`, `:272` |
| structural-unmet | `verify.rs:167` |
| obligation-unmet | `verify.rs:367` |
| missing-curve | `fit.rs:279` |
| tolerance-miss | `fit.rs:304` |
| onboarding-gate | `gate.rs:369` |
| level-miss, no-reference, visual-unassessed | `generate/mod.rs:47-58` |
| article-unfilled | `document.rs:60` |
| article-claim-contradicted / -unsupported | `document.rs:104` |
| value-rounds | `gap/rounds.rs:156` |
| gap-fix | `gap/route.rs:278` |
| gap-review | `gap/resume.rs:122` |

The routine kinds are listed in `data/conductor-policy.json` under `decisions.routine`. The halt kinds that go to the gap loop are onboarding-gate and level-miss (`gap/mod.rs:33`).

## 4. External calls

**Jev from the pipeline** (through `judge.ask`):

| Question set | Where | Answer used for |
|---|---|---|
| ranking | `discover.rs:330`, search | choosing a source per field |
| rights | `rights.rs:243` | admission |
| screen | `screen.rs:50` via `screen.rs:90` | row kind and condition |
| sufficiency, mature, growth | `quality.rs:71` | level and gap, which can file a decision |
| select | `pick.rs:68` | the span that fills a value |
| described | `appearance.rs:383` | appearance level |
| cite | `verify.rs:59`, `document.rs:84` | claim decisions |
| obligation:* | `verify.rs:105`, `:219` | obligation decisions |
| transfer | `generate/mod.rs:290` | nearest template |
| gap | `gap/route.rs:171` | signals for the route table |

**Jev from the conductor** (through `Asker::ask`, `questions.rs:83`):

| Question | Where | Answer used for |
|---|---|---|
| conductor-reach | `gapcheck.rs:168` | Reachable verdict |
| conductor-cover | `gapcheck.rs:185` | Covered verdict |
| conductor-implementation | `dependency.rs:205` | policy signal |
| conductor-design | `dependency.rs:231` | policy signal |
| conductor-continuation | `questions.rs:185` | continue or pause |

**Jev from tuning** (through `Live::ask`, `live.rs:557`, tool "tuning"):

| Question | Where | Answer used for |
|---|---|---|
| proposal batches | `live.rs:945`, called at `engine.rs:674` | dial proposals |
| routes | `live.rs:1014`, called at `routing.rs:23` | per-priority route |
| risk_only | `live.rs:902`, called at `routing.rs:92` | handoff authorization |
| round evidence | `live.rs:914`, called at `round.rs:109` | repeat allowed or not |
| stride | `live.rs:759`, called at `stride/decide.rs:111` | step size |
| veto | `live.rs:926`, called at `veto/judge.rs:77` | keep or undo an adoption |

**Reviewer adapters** (spawned with `timeout`):

| Adapter | Where | Answer used for |
|---|---|---|
| vision assessment | `vision.rs:201`, `repair.rs:37` (called from `live.rs:533`/`539`) | cells, readiness, defects |
| contact sheet | `progress/adapter.rs:26` (the shared `shell`), called at `sheet/review.rs:24` | ranking the bundle variants |
| progress review | `progress.rs:288` | side-by-side choice |
| inventory | `inventory.rs:89` | reference inventory (a separate command) |

**Fetch and search**

- Firecrawl CLI: `firecrawl.rs:116` (search, research, scrape, parse) and `ureq` at `firecrawl.rs:373`.
- Europe PMC and DOAJ: `rights.rs:169`, `:177`.

**Other binaries**

- species_measure, headless: `render.rs:113`, `:144`, `:176`.
- gate checks: `gate.rs:63`, `:71`.
- headless and `compare-references.py`: `matched.rs:151`, `:189`.
- node catalogue scripts: `document.rs:294`.

## 5. Shared state files

| File | Written by | Read by |
|---|---|---|
| `conductor/run.json` | `conductor/state.rs:223` | plan, every cli command |
| `conductor/handoff-*.json` / `.md` | `handoff.rs:99` | host only |
| `conductor/gaps/*.json` | `gapcheck.rs:117` | the handoff list (`handoff.rs:105`) |
| `conductor/packet.json` | `packet.rs:174` | `plan.rs:368` |
| `conductor/report.json` | `conductor/report.rs:212` | host |
| `tuning-N/run.json` | `tuning/command.rs:311` | `conductor/tuning.rs:25` |
| `tuning-N/result.json` | `command.rs:336`, `bin/tuning_loop.rs:36` | `gapcheck.rs:39`, packet, handoff |
| `priority-review.json`, `handoffs.json`, `finalists.json`, `RESULT.*` | `command.rs:314-346` | no code; humans only |
| `D/decisions.json` | `stage.rs:361`, `decision.rs:269`, `gap/resume.rs:279` | stages, plan, packet, bin |
| `D/resolutions.json` | owner or agent, plus `admission.rs:204` (search, discover) | `decision.rs:261`, plan fingerprint (`plan.rs:88`) |
| `D/search-rounds.json` | `search/rounds.rs:51` | plan, step |
| `D/gaps/*/gap.json` | `gap/mod.rs:145` | `plan.rs:164`, gap metrics |
| `D/rounds.json`, `D/metrics.json` | `gap/rounds.rs:38`, `gap/metrics.rs:218` | report, packet |
| `R/ledger/entries`, `ledger/index.json` | caller; `judge.rs:57` | `conductor/report.rs:43`, `gap/metrics.rs:125` |
| tuning `config.ledger` and adapter ledgers | `live.rs:557`; `vision.rs:219`, `progress/adapter.rs:44`, `repair.rs` | `reference_first.rs:480` (convergence), calibration |
| `R/command-log.json` | `stage.rs:384` | `swap.rs:68` only |
| stage artifacts `D/<stage>.json` | `stage.rs:339` | later stages, `plan.rs:187`, `:193` |

## 6. Dead or duplicated

**No production caller**

- Only defined, never called: `tuning/round.rs` `attempts_here` (no callers anywhere) and `pipeline/adapter/firecrawl.rs` `scrape_pdf`.
- Reached only from tests: `curve::gould_decade_growth_cm`, `rights::set_version`, `sets::set_version`, `floors::level_floor_cases`, `policy::routine_decision`, `vision::blind_request`, `firecrawl::with_program`.
- `scripts/catalogue-check.mjs` and `scripts/catalogue-pages.mjs` have no Rust caller.
- `handoffs.json` and `finalists.json` are written and no code reads them.

**Metrics gap in the conductor flow.** `metrics.json` is required by the packet (`packet.rs:51`), but only `species-pipeline gap metrics` writes it, and the conductor never runs that command.

**Duplicated jobs**

- **Two reviewer comparisons:** `tuning/sheet/` (Bundle) and `tuning/progress/` (Visual). They share `progress::shell` and `redacted`; only one runs per selection mode. The palm run uses bundle.
- **Two visual assessment paths:** `vision::Adapter::assess` and `reference_first::assess` → `repair`, switched at `live.rs:525-539`.
- **Four result/report writers:**
  - `tuning/result.rs`, written twice: `command.rs:334` and `bin/tuning_loop.rs:34` `result`;
  - `conductor/report.rs`;
  - `pipeline/stages/report.rs`;
  - `pipeline/gap/metrics.rs`.
- **Two handoff modules:** `conductor/handoff.rs` (pause file) and `tuning/handoff.rs` (fn-89 gap handoff).
- **Two route tables:** `conductor/policy.rs` with `data/conductor-policy.json`, and `pipeline/gap/table.rs` with `data/gap-routes.json`. Same row and first-match design (`policy.rs:1-8`, `gap/table.rs:1-8`).
- **Two round counters:** `tuning/round.rs` and `tuning/runaway.rs` (tuning), and `pipeline/gap/rounds.rs` (value rounds).
- **Two continuation checks:** `conductor/questions.rs:169` and `tuning/routing.rs:72`, both over `tuning::continuation`.
- **Labelled-case scoring in three places:** `conductor/cases.rs`, `pipeline/sets/cases.rs` with `gap/questions.rs`, and `tuning/calibration.rs` with `replay.rs`.
- **Six adapter scripts, in pairs:** Codex and Claude twins, where the Claude one wraps the Codex one.

**CLI subcommands the conductor never calls**

- `species-pipeline`: `swap`, and all ten `gap` subcommands (only an agent inside a dispatch runs them).
- `tuning-loop`: `preflight`, `inventory`, `freeze-replay`, `result`, `freeze`, `calibrate`, `vision-replay`. Only `run [--resume]` is used (`step.rs:97`).
- `jev`: all six (`screen`, `select`, `cite`, `triage`, `ask`, `cases`).
- `species-conductor` `cases`, `report`, `packet`: the host runs them by hand, though `step` also calls packet assembly itself.