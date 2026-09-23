# fn-80 friction

## 2026-09-22 — host: the conductor attached the driver's sentence as a spec

First live tick. The gap-loop dispatch's scope says "report the spec it minted in observed", the driver wrote a sentence there because no spec had been minted, and the conductor attached the whole sentence as a dependency and planned to design it. Cost: about 15 minutes, one code fix (attach only a spec the Flow tree holds), one test on the live record, and a replay of the two steps. What would have removed it: a result field that is typed, `minted_spec: Option<String>`, instead of a free sentence doing double duty. Recorded as a defect the live run found; the fix ships on this branch.

## 2026-09-22 — host: two pauses on a routine dependency before anything was tried

Second and third ticks. After fn-108 was attached, the conductor judged the spec routine (0.99) and then paused twice on the continuation check. The first pause priced the next attempt at one token because the only known usage was the driver's 0 (fixed: a zero usage is unknown). The second came from the continuation trio answering insufficient evidence on progress, risk and tractability, which is true by construction for a first attempt: there is no history to judge. fn-68's R11 settled this for the tuning loop on 2026-09-21 (a first bounded attempt proceeds without a judgment; only a repeat on moved evidence asks) and the conductor did not carry the rule. Cost: about 25 minutes, two resumes, one Jev trio spent on an unanswerable question. What would remove it: the same first-attempt rule in the conductor, applied below.

## 2026-09-22 — host: after the landing the conductor went after a halt the landing had cleared

Fourth tick. fn-108 landed, `gap resume` expired the gate stage's key, and the conductor's next action was the gap loop on the registry gate, the other halt the old gate run had filed, instead of rerunning the gate that would clear it. The plan read open halts before stale stages. Cost: about 10 minutes, one plan-order fix (once a dependency has landed and the stage fingerprint is stale, stages rerun before any halt is acted on). No Jev call was spent.

## 2026-09-22 — host: the gate left a passed halt open, and a landed round did not loop again

Fifth tick, after the gate reran on fn-108. Two things. The registry gate had passed (`gate.json` says `registry: true`) but its decision from the earlier run stayed open with its old inputs, because a rerun only replaces a decision it files again and never retires one it no longer files; the conductor saw an open halt. And the capability halt, narrowed to five names by the landing, was treated as "looped once, the owner's now", because the plan could not tell a landed round from a round that went to the owner. Cost: about 40 minutes, two fixes with tests: the gate stage retires the open decisions a rerun with changed inputs did not file again (`superseded`, by the stage), and a halt whose gap record shows a landed round loops again. What would have removed it: fn-63's gap loop had never been run twice on one halt; the second round is where both rules were missing.

## 2026-09-22 — host: the owner's resolution did not reach the gap record until a stage ran

Round two. The owner confirmed the rosette; the resolution was appended to `resolutions.json` with the decision's exact inputs and option, but `gap spec` refused ("no owner resolution names an option") because only a stage run reconciles resolutions into `decisions.json`; the conductor's own read of open decisions did not write the reconciliation back. Running the gate stage, which was current, applied it. Cost: about 10 minutes and one wrong `gap spec`. What would remove it: `gap spec` (or the conductor's attach) reconciling before it reads, or a `species-pipeline reconcile` command the runbook names for a person's resolution.

## 2026-09-22 — host: the second option set was written in the host session, so its usage is unknown

The pipeline refuses a second agent set for one gap, so the round-two options were the host's; dispatch-3's usage is therefore null and `usage_known` is false on the run, which the continuation check will read as unavailable at the next repeat. What would remove it: the conductor dispatching the stronger set to a strong-tier agent with counted usage, as it does for design, instead of leaving it to the host.

## 2026-09-22 — host: one uncounted dispatch made every later continuation judgment unavailable

After the rosette design verified, the conductor paused: "the continuation judgment is unavailable (unknown prior usage)". The cause was dispatch-3, the host-written option set with no token count, which had set the run's usage to unknown for good. Cost: about 15 minutes and one fix with a test: a finished dispatch that could not count charges its reservation, the run's usage stays known, and the report still lists the cost as unknown; a record written before the rule heals on open. Implementation complexity was judged complex at 0.61, just over the floor, routing to the strong tier at low effort.

## 2026-09-22 — host: a judgment over unchanged evidence was re-bought and flipped; a resume was second-guessed

After the resume from pause-5 the conductor re-asked implementation complexity over the same design revision and got insufficient evidence where the step before had complex at 0.61, then asked the trio, which split on tractability, and paused again. Two rules were missing: a judgment is kept with the evidence it read and reused while that evidence is unchanged (insufficient evidence is not kept, since new evidence may settle it), and a scoped human resume authorizes the attempt it names without the trio asking again; opening the attempt clears the authorization. Cost: about 30 minutes, two Jev calls, one pause. Both rules now ship with tests.

## 2026-09-22 — host: an under-floor answer bought a second investigation of the same design

Three implementation judgments over the same design revision put 0.61, 0.68 and 0.64 on complex and never cleared the concentration floor of 0.6, so each read as insufficient evidence and the policy bought an investigation, then a second one on unchanged evidence. Cost: one cheap investigation that was still useful (it found a sixth authoring site and a wrong type name), one that would not have been, about 20 minutes. Rule added with a test: one investigation is what an under-floor answer buys; after it, the same answer is decided on the mass its side carries, the rule the owner approved for the tuning loop's direction acceptance. dispatch-6 is closed on the record as not run.

## 2026-09-22 — host: the loop's first landing turned master red on a check the gate does not run

fn-108 landed green on the cargo gate and red on CI: its spec, written by the host, asked for `catalogue/date-palm/packet/profile.json` by analogy with the beech, and the node job's catalogue check walks every folder under `catalogue/` and requires all thirteen files. The implementer ran the gate the rule names, which is cargo only, so nothing local could have caught it. Cost: master red for about an hour, one fix PR. What would remove it: the catalogue check in the local gate line or in make-pr's preflight, since a species folder is exactly what a gap spec touches; and a spec claim about a repository layout checked against the check that owns it, not against a sibling folder.

## 2026-09-22 — host: a gap could land one fix; the second round's landing was refused

fn-109 landed on master and the pipeline's `gap resume` refused it: "already resumed at b8b29448", the first round's landing. The gap record held one landing, and fn-63's loop had never run two rounds on one halt. Fixed with a test: a later round's spec lands too, the earlier landing moves into the record's history, and the rerun's idempotence key carries every fix. The same spec landing twice is still refused.

## 2026-09-22 — host: the conductor's landing and the pipeline's landing are two steps, and the gate reran between them

`species-conductor land` marks the dependency landed; the pipeline's `gap resume` records the fix on the gap and expires the halted stage's key. They are separate commands, and the conductor's rule that stages rerun after a landing fired on the first, before the second had happened, so the gate reran with the old key and reported nothing new; the rerun-after-landing record then said the stages had run for that commit. Cost: one wasted stage pass and a forced gate rerun by hand. What would remove it: `land` running the pipeline's `gap resume` itself, with the pin note passed through, so one command does both and the stage rerun follows the key change.

## 2026-09-22 — host: a non-blocking gate was sent to the gap loop

After the capability halt narrowed to the three organs, the conductor's next action was the gap loop on the seeds gate, "packet/specimens.json does not exist yet", which the generate stage writes and which blocks nothing. The plan takes any open decision after the blocking ones and routes a halt kind to the loop regardless of whether it blocks. Cost: one dispatch opened and closed unrun. What would remove it: an open decision that blocks nothing is not a halt; the stages run and it resolves itself or reaches the owner after them.

## 2026-09-22 — host: the palm's first generate measured nothing, and document could not find a bibliography

Two things from the palm's first pass through generate. The measurer builds its case as `ID:PROFILE:PRESET:SEED` and the profile came from `--profile-id`, which the conductor's stage arguments did not carry, so every measurement failed on an empty part and the specimens carry no heights; the profile's own manifest (`packet/profile.json`) is a valid profiles file and now rides on the stage arguments. Then the document stage's catalogue script reads `catalogue/<species>/sources.json`, a folder this run never had because it ran with the evidence directory as `--dir`; the bibliography is now written from the manifest's sources. Cost: one wasted generate pass and about 25 minutes. What would remove it: the pipeline reading the profile id and the profiles file off the manifest and the packet it already holds instead of two flags, and the document stage writing the bibliography from the manifest when the folder lacks one, since the sources are the manifest's.

## 2026-09-22 — host: a non-blocking decision stopped the run for the owner before any tuning

After the palm's first generate the conductor awaited the owner on `generate/visual-unassessed`, a decision that blocks nothing and that the tuning revisions and the packet answer. The plan acted on any open decision after the blocking ones. Fixed with a test: only a decision that blocks a stage is acted on before the stages and the tuning; the rest surface in the packet's unresolved list. Cost: about 10 minutes.

## 2026-09-22 — host: the Claude vision adapter failed its own first live call twice

The palm's first Opus inventory came back complete and the adapter marked it failed: the CLI answers a `--json-schema` request through its own `StructuredOutput` tool call, and the adapter counted that call as a forbidden tool. The second call came back with 17 well-formed traits and the receipt refused it: `Inventory::verify` holds at most 16, and the schema handed to the model stated no cap. Each cost one paid Opus call of about 3,300 output tokens and a fix with a test (7d469619, 8bf329e1). What would have removed it: one live smoke call of a new adapter before it is wired into a run, and a schema generated from the receipt's limits rather than written beside them.

## 2026-09-22 — host: the visual replay is bound to the reviewer's model, so a reviewer swap re-buys the calibration

Moving the palm's reviewer from Codex to Opus made the frozen birch/beech replay stale: the replay carries the model and effort, and its two case inventories were Astra's. Qualifying Opus takes two fresh inventories and two comparisons, four paid calls before a single palm render is judged. The cost is right (a reviewer is qualified per model), but nothing in the config told me before the preflight did, and the inventory configs for the two calibration species had to be assembled by hand from the frozen replay. What would have removed it: a `tuning-loop qualify --adapter` that reads the frozen replay's cases and re-runs them on the named adapter.

## 2026-09-22 — host: the tuning run's first three starts each stopped at the baseline on a wiring fault

Start one measured nothing: the config still pointed at fn-68's profile set, which has no date palm. Start two: the palm's own profile carried `dbh_m` as `unavailable`, and the measurer reads only `gating` or `contextual`, so the same field that had halted the conductor's generate stage halted the baseline; it is now `contextual` with no range, which reports the value and gates nothing, and leaves the owner's lower-bar decision open. Start three: the compare script names a cached photograph by the last segment of its record's url, and the three photographs were stored under view names. Each start costs a fresh run directory and a rewritten authority decision, about five minutes; nothing paid was re-bought, since the preparation charge is re-read from its receipt. What would have removed it: a preflight that runs the baseline's measurer and photograph lookup dry, before any authority is asked for; a profile derivation that emits only classifications the measurer reads; and the cache naming rule written where the photographs are fetched.

## 2026-09-22 — host: the shot's foliage word is `leaf-on`, and nothing at the authoring site says so

The fourth start of the tuning run stopped before its first paid look: the palm's derived shots said foliage `shown`, and the packet reads only `leaf-on`, `hidden` or nothing (`joint.rs`, `with_shots`). The references file has no schema and the reference-recording step has no check, so the word was wrong from the moment the shots were derived and surfaced three stages later. Cost: one more run directory and authority rewrite, about four minutes; no paid call. What would have removed it: the shot vocabulary validated where a reference record is written, or the packet accepting `shown` as the synonym it already maps `leaf-on` to.

## 2026-09-22 — host: a gap that minted two specs landed the wrong one

The capability gate's third round minted two organ specs, fn-110 and fn-111. `gap spec` keeps one `spec` on the record, so recording fn-111 after fn-110 overwrote it, and `gap resume --commit aa488beb` (fn-110's merge) wrote the landing against fn-111. Cost: about 15 minutes, a hand repair of the gap record (noted in it), and a fix with a test: the record keeps every minted spec in `specs`, and `gap resume --spec` names which one lands. What would have removed it: the loop's record modelled one spec per round, and the palm was the first gap to mint two in a round.

## 2026-09-22 — host: the receipt refused a paid pass over one variation row, twice in one day

The round-three visual pass on Opus came back complete (16 coverage rows, 12 findings, 6,462 output tokens) and the receipt refused it because one variation row, epiphytes on the trunk, cited the whole-tree photograph instead of the two close views it was inventoried from; the earlier beech calibration case had been refused the same way over a row citing no render. Each refusal cost the pass's reservation, a fix with a test, a rebuild and a scoped resume, about 20 minutes. What would have removed it: the receipt's row rules treating a row that cannot count as a row to drop or downgrade and record, which is now what both do, and a dry bind of the reviewer's schema against every rule before the first live call.

## 2026-09-22 — host: every owner decision in the loop must re-carry the pilot authority

The priority approval was refused on the next start because a decision that does not carry `experimental_pilot` leaves the run without authority, by design (the fn-68 test names it); the same held after the dial-table repair and the recalibration, so the authority was rewritten five times in one afternoon, once per pause. Cost: about 15 minutes of resumes. What would have removed it: the pause's `decision_requested` naming that the authority must be re-carried, or the run keeping an authority whose identity and caps still match.

## 2026-09-22 — host: a stage that wrote nothing was never current, so the conductor reran it on every step

After fn-110 landed and the stages reran, every conductor step reported `fit: ran` and its next action stayed "stages from fit": the fit stage returns `Skipped` when the manifest names no curve, and wrote no record, so `is_current` never matched. Cost: about 15 minutes and a fix with a test (the skipped fit writes its record). What would have removed it: the stage contract saying every outcome leaves a record; the palm is the first species run whose manifest names no growth curve.

## 2026-09-23 — host: the tuning run's gap list went to the owner instead of back into the loop

The palm's first tuning revision stopped at its eighth pause and wrote a gap list whose result file says it "mints nothing". The host read the list, wrote five proposals and parked the run on the owner's approval (2026-09-22 18:23 to 2026-09-23). Two of the proposals were tuning-loop design, which the dispatch rule puts with the host, not the owner, and the owner asked why the run had stopped. Cost: about a day of wall clock with the run idle, and one owner turn. What would have removed it: the tuning result's gaps entering the gap loop as decisions (reachable, range-capped, covered, new) that the table routes the way it routes a gate halt, so a loop-design fix is minted and built without a stop, and only a real owner route (a visual verdict, a new organ the owner has not asked for) pauses the run.

## 2026-09-23 — host: the conductor cannot resume a tuning run it started

The conductor's `step` ran `tuning-loop run` for the palm's second revision and the run paused at once for scoped experimental authority, as every fresh tuning run does. The conductor's `tune` (`conductor/step.rs`) passes `--config` and `--out` and never `--resume`, and `species-conductor resume` resumes the conductor's own pauses, not the tuning loop's, so the pause has no path back through the conductor; the host resumes `tuning-loop` by hand in the conductor's `tuning-1` directory. Cost: about 10 minutes, and the second revision again runs beside the conductor rather than through it. What would remove it: the conductor recording the tuning pause as its own pause and passing the decision through to `tuning-loop run --resume` on its next step.
