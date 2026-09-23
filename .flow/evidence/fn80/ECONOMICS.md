# fn-80 economics: what a species costs and what would make it cheap

The goal this serves (owner, 2026-09-18): most tree species in the world,
onboarded by swarms of cheap, fast agents running the `add-species` loop. The
unit that matters is therefore cost per species at swarm scale, not the cost of
one careful run. A loop that is correct but expensive does not reach the goal.

**The axis is model cost, not scraping credits** (owner, 2026-09-18, correcting
this file's first two entries): "i mean cheap models. Not cheap scraping
credits. I might have to get those sorted separately." So the question every
entry answers first is which model a step needs, whether a cheap fast one can
do it, and how often the step escalates to a frontier model. Scraping is still worth
being efficient about, and its credits stay recorded because they are the one
number the artifacts already carry; the owner is sorting that supply
separately, so it is the second question, not the first. An entry that
optimises credits while leaving a frontier model in the loop has measured the
wrong thing; an entry that ignores an easy scraping cut has left money on the
table.

This file is the judgment the numbers do not carry. The loop already records
gaps by route, rounds to acceptance, reversals, tokens, wall clock, Jev calls,
Firecrawl credits and captures (`gap metrics`, fn-63 R5). What it cannot record
is which step dominated, what a cheaper model would have done instead, and what
could be cached, batched or skipped across species. That goes here, as it
happens, one dated entry per observation, never reconstructed at the end.

Distinct from `FRICTION.md`, which records what slowed or hindered an agent.
This records what a step cost and what would make the next hundred species
cheaper. A step can be fast and still be the wrong place to spend.

## What every entry answers

- **Step.** Which stage or command, and what it was doing.
- **Cost.** The measured number: credits, Jev calls, tokens, wall clock,
  captures, driver dispatches. A number, not an adjective.
- **Dominates?** Whether this is a large share of the run's total, and of what.
- **Cheaper next time.** The concrete change: a cheaper model for this step, a
  cache across species, a batch across species, a step skipped when a condition
  holds, or an owner stop answered once for many species rather than per
  species.
- **Swarm reading.** What this costs when multiplied by a thousand species, and
  whether that number is acceptable.

## Anchors from before this run

| Run | Cost | Note |
|---|---|---|
| European ash pipeline, 2026-09-18 | 107 Firecrawl credits, 6 driver dispatches for a 2-dispatch path | the four defects behind the extra four became fn-75 |
| fn-34, three species, 2026-09-14 | 23 rounds, 15 capability dependencies | the reason for one species per spec |
| fn-13 task 5 | a full weekly quota, 22 full-forest captures | the reason for small before large |

## Entries

## 2026-09-18 — the capability question is answerable before the literature spend

- **Step.** The `add-species` skill's order: mint the spec, run `discover`,
  admit the manifest, then walk the stages; the capability gate that files an
  `onboarding-gate` decision sits at `gate`, near the end.
- **Cost.** The literature stages are where Firecrawl credits go: the European
  ash run spent 107 of them before reaching its gate. For the date palm, the
  architectural model is Corner, known from the taxon alone and needing no
  source at all, and fn-35 already records that no architectural-model coverage
  file exists and that the 23-model list becomes its own spec the first time a
  species names an unsupported model. So at least one of this species' gaps was
  knowable for zero credits, before the run started.
- **Dominates?** Yes for a species whose form is unsupported. The whole
  literature spend precedes the finding that the species cannot be drawn yet,
  and is then repeated when the species resumes after the gap specs land, since
  the run's own fixes expire the stages from the halted one down.
- **Cheaper next time.** A capability pre-flight before `discover`: take the
  architectural model and the organ list off the spec, check them against the
  coverage file, and file the `onboarding-gate` decision then, when it costs
  nothing. The literature stages run only for a species whose form the field
  already draws, or after its gap specs land. This reorders the skill; it adds
  no new machinery, because the gap loop already takes an `onboarding-gate`
  decision from wherever it is filed.
- **Swarm reading.** At a thousand species the sort matters more than the
  saving on any one. Most of the world's trees are forms the field does not yet
  draw, so a swarm that spends the literature budget before the capability
  check spends most of its budget on species it then parks. The pre-flight
  turns the catalogue into a cheap sort into "drawable now" and "waiting on a
  named gap", and only the first group costs credits.

## 2026-09-18 — date-palm discover: 6 estimated Firecrawl credits, 2 Jev calls, 10.6 s

- **Step.** `species-pipeline discover` on the date-palm seed (two fields, `height_m` and `dbh_m`, both `open_grown`, ages 20/50/80). One plain-word web search and one research-index search per field, then one ranking judgment per field.
- **Cost.** `discover.json` records 6 Firecrawl credits (method: estimated, one credit per unpriced CLI call), 2 Jev calls (ledger `9f3928213de63590113735f2`, `f6d62e66ff4d519ab72091c0`), 1 run, about 10.6 s wall clock. Two fields should be 4 CLI calls; the extra 2 unpriced calls are not named in the stage. No scrape. Credits used: 6 of the run's 40-cap.
- **Dominates?** No of this run so far. It is the whole Firecrawl spend because the run stops before fetch. It does not dominate a finished species: the ash run spent 107 credits and most of those were later stages.
- **Cheaper next time.** Three cuts, in order of swarm leverage. (1) Taxon-filter the known list so oak, spruce and fn-11 URLs never enter a palm ranking; 11 of 21 hits per field were those, and both Jev calls paid to reject them. (2) Price `research search-papers` (and search when the CLI omits `creditsUsed`) so the 6 is a real number, not an estimate. (3) Drop or cache the research index for a first-pass discover when the web hits already contain an extension page; both research lists here were off-topic or same-taxon morphology, and none ranked first.
- **Swarm reading.** A thousand species at this seed is 6,000 estimated Firecrawl credits and 2,000 Jev calls, plus whatever fetch then spends. That discover slice is cheap enough to swarm if the later stages stay parked for unsupported forms (see the entry above). It is not cheap if every species also fetches: the ash 107 is the number that multiplies. The known-list flood is paid on every species until it is filtered, and it grows as more manifests land.


## 2026-09-18 — the axis correction, and what the discover leg showed about models

- **Step.** The whole of skill step 2, run by `cursor-agent` on
  `cursor-grok-4.6-high-fast`: write the seed, run `discover`, read a 42-hit
  proposal across two fields, drop Wikipedia and a wrong-taxon candidate, draft
  the manifest, and write both notes.
- **Cost.** Four minutes of wall clock, one bridge call, zero escalations to a
  frontier model. The host spent nothing on the step beyond dispatching it and
  reading the result. Token cost: **not measurable** — see below.
- **Dominates?** No. This is the encouraging datapoint: the cheapest tier in
  the routing block did a judgment-heavy step, rejecting a forbidden citation
  and a sister species, and wrote two notes a person can use. Nothing here
  needed a frontier model.
- **Cheaper next time.** Nothing to cut on this leg. The open question is how
  far up the stages that holds: the later stages read tables and copy measured
  values, which is more mechanical than this was, so the cheap tier should hold
  at least to the gate. The place to expect escalation is the gap loop's option
  writing, where the route table itself can send a set to a stronger model.
- **Swarm reading.** A thousand species at this rate is a thousand four-minute
  cheap-tier calls for the manifest step, which is the shape the goal needs.
  The threat to it is not this step; it is any step that silently needs a
  frontier model, and the escalation share is the number to watch.
- **Measurement gap, and it blocks the goal.** The bridge reports no token
  count, so the driver's own model cost is invisible: `cursor-agent` returns
  text, and the pipeline's `cost.rs` counts runs, Jev calls and Firecrawl
  credits but has no field for the driver's tokens. Cost per species in the
  unit that matters therefore cannot be computed today, only estimated from
  wall clock. A swarm plan needs that number per run, per model, so a field for
  it beside the existing three, filled by the host from whatever the bridge
  reports, is the first thing to add.

## 2026-09-18 — date-palm fetch: 3 estimated Firecrawl credits, 0 Jev calls, 14.9 s

- **Step.** `species-pipeline fetch` on the three admitted sources (F1 IFAS HTML, A1 UA arboretum HTML, M1 BMC DOI resolving to Springer HTML). No tables in the manifest, so no row parse and no coverage-gap. The admit resolution was consumed; no new decision.
- **Cost.** `fetch.json` records 3 Firecrawl credits (method: estimated, one credit per unpriced adapter call), 0 Jev calls, 1 run, 14.930 s wall clock. Driver thinking: none. The command printed `ran` and I read three source records. No escalation.
- **Dominates?** No of model cost: a cheap fast model only has to invoke the command and notice `ran` versus a named decision. Yes of this leg's Firecrawl so far (3 of 3), but that is three scrapes, not a literature hunt. Credits this leg: 3 of the 120-cap.
- **Cheaper next time.** Price the scrape so the 3 is `creditsUsed` rather than an estimate. Cache by final URL across species so a later palm that admits the same IFAS page spends zero. The DOI resolved to HTML, not a PDF, which kept the cost at one call; a PDF scrape is the credit risk on this stage, not the HTML pages.
- **Swarm reading.** A thousand species at three HTML sources is 3,000 estimated credits and zero Jev. The model cost is one cheap-tier invoke per species. This is the shape the swarm wants: the driver does not read the pages. The only step that needs a person is an `unavailable-source` or `coverage-gap` decision, and this run had neither.

## 2026-09-18 — date-palm extract: 0 credits, 0 Jev calls, 0.015 s, 33 candidates

- **Step.** `species-pipeline extract` over the three cached markdowns. Code pulls every sentence with a length, age or rate unit. 22 from A1, 5 from F1, 6 from M1.
- **Cost.** `extract.json` records 0 Firecrawl credits, 0 Jev calls, 1 run, 0.015 s wall clock. Driver thinking: none. I ran the command and counted the rows.
- **Dominates?** No of this stage. It dominates the *next* stage: most of the 33 sentences are fruit length, spadix strands, frond width, offset weight, rain, or citation-list debris, not height or trunk diameter. Screen paid one Jev call per sentence.
- **Cheaper next time.** Tighten the candidate regex to the field's unit family (height/dbh vs organ and fruit), or drop sentences whose only number is a page/table id. That is a code cut, not a model cut. The driver step itself is already the cheapest possible: invoke and skip.
- **Swarm reading.** A thousand species at 15 ms and zero calls is free. The swarm cost is the noise this stage hands screen. If every species' extract is this broad, screen Jev calls scale with page length, not with the two growth fields.

## 2026-09-18 — date-palm screen: 0 credits, 33 Jev calls, 20.9 s

- **Step.** `species-pipeline screen` over the 33 extract sentences. One Jev call per sentence. Kinds: 21 `not_about_tree_size`, 4 `mature_size_range`, 4 `sprout_cultivar_or_nursery`, 2 `measured_size_at_age`, 2 `typical_growth_rate`. One sentence was `anchor_usable`. One was `open_grown` (F1's "heights up to 80 feet").
- **Cost.** `screen.json` records 0 Firecrawl credits, 33 Jev calls, 1 run, 20.912 s wall clock (~0.63 s per call). Driver thinking: none. I ran the command and summarised the kinds. No escalation.
- **Dominates?** Yes of Jev so far this leg (33 of 33). No of model cost for the driver: a cheap fast model invokes and reads the kind counts. The judgment is Jev's, not the driver's.
- **Cheaper next time.** The 21 `not_about_tree_size` calls are the waste extract predicted. A pre-filter on unit family, or one Jev call per source instead of per sentence, would cut this to the 12 sentences that were about size. Batching the 33 into one request would also drop wall clock, but the call count is the swarm number.
- **Swarm reading.** A thousand species at this extract noise is 33,000 Jev calls for screen alone. That is the first number that starts to matter. If extract were tight enough to hand screen ~8 size sentences, it would be 8,000. The cheap-tier driver still does not need to read any sentence.

## 2026-09-18 — date-palm quality: 0 credits, 2 Jev calls, 1.6 s, one owner decision

- **Step.** `species-pipeline quality` over the two fields. Code counted measured points (none) and uncovered ages (20/50/80 both fields). Jev scored sufficiency: `height_m` level `proxy_only` against bar `proxy_only` (passed); `dbh_m` level `proxy_only` against bar `partial` (failed). Dominant gap both times: `no_age_indexed_points`. Filed `date-palm/quality/data-insufficient/dbh_m` with options `admit-proxy` / `add-sources` / `lower-bar`. I did not resolve it.
- **Cost.** `quality.json` records 0 Firecrawl credits, 2 Jev calls (ledger `cb82aa6f28f81a2a2e2c2bc7`, `bb529938c84ad20a9997412b`), 1 run, 1.638 s wall clock. Driver thinking: light. I had to read the decision kind and the runbook table, then leave it. That is not hard thinking; it is following a stop rule. No escalation.
- **Dominates?** No of Jev or credits. The owner halt is the cost: select/fit/generate stay blocked on `dbh_m` until a person acts. That is the right cost, not a model cost.
- **Cheaper next time.** For a palm, `dbh_m` at bar `partial` will fail on almost every species whose trunk does not thicken with age. An owner stop answered once for the growth-form `palm` ("dbh is a poor fit; bar is `proxy_only` or the field is dropped") would save a thousand identical decisions. The cheap-tier driver already handled the halt without a frontier model.
- **Swarm reading.** Two Jev calls per species is 2,000 at a thousand species, cheap. The swarm risk is a thousand owner tickets for the same palm-trunk fact. One catalogue rule for `growth_form: palm` removes that queue.

## 2026-09-18 — date-palm select: 0 credits, 1 Jev call, 0.7 s

- **Step.** `species-pipeline select`. `dbh_m` recorded unavailable (`below the data-quality bar`) because of the open `data-insufficient` decision. Jev picked a span for `height_m` from A1; code copied `35 m` into `/profiles/0/metrics/height_m` (range 35–35, source A1, route `copied`). Packet `profile.json` and `references.json` written. No described traits in the manifest.
- **Cost.** `select.json` records 0 Firecrawl credits, 1 Jev call (ledger `34be9eefc6354c51b45d5718`), 1 run, 0.714 s wall clock. Driver thinking: none. I did not choose the 35 m; I read the filled pointer after the command printed `ran`.
- **Dominates?** No. One selection call is cheaper than screen by more than an order of magnitude.
- **Cheaper next time.** Nothing on the driver side. The span is a mature-size ceiling ("up to 35 m"), not a height at a required age; quality already passed `height_m` at `proxy_only`, so this is the bar working as designed. A cheap model can run this unaided.
- **Swarm reading.** One Jev call per passed field, a thousand times, is 1,000–2,000 calls. Acceptable. The driver still does not read the source.

## 2026-09-18 — date-palm verify: 0 credits, 2 Jev calls, 2.0 s, one owner decision

- **Step.** `species-pipeline verify` on the filled `height_m: 35 m` claim. Citation check: relation `supports`, confidence 0.28 below 0.8, so it listed the claim and filed `date-palm/verify/claim-unsupported/A1` (options `accept` / `replace-source` / `drop-value`, blocks generate). The section it attached is the A1 fact box "Height: 50 - 100 feet", not the "up to 35 m" sentence. Obligation `measurement_not_invention` held. I did not resolve the decision.
- **Cost.** `verify.json` records 0 Firecrawl credits, 2 Jev calls (ledger `c2f143d9816445fb40b150f7`, `e7c599f8468c9b723ebd2139`), 1 run, 1.985 s wall clock. Driver thinking: light. I read the decision kind, saw it is not in the four kinds a driver consumes, and left it. No escalation.
- **Dominates?** No of Jev or credits. The owner ticket is the cost, and it is a real one: the check agreed the source supports the claim and then failed the confidence cut, pointing at a different span.
- **Cheaper next time.** Point the cite check at the copied span ("35 m") rather than a neighbouring fact box in feet. That is a code cut. For the swarm, a cheap model can leave `claim-unsupported` for the owner; it must not "accept" to keep the run moving. The 0.28-on-supports pattern will fire on every source whose markdown is a mashed fact sheet.
- **Swarm reading.** Two Jev calls per species is cheap. A thousand `claim-unsupported` tickets on the same confidence-cut-plus-wrong-section pattern is not. Fix the section binding once, or the owner drowns in listed claims that the check already called supporting.

## 2026-09-18 — date-palm fit: 0 credits, 0 Jev calls, 0.009 s, no artifact

- **Step.** `species-pipeline fit`. The admitted manifest has no `curves` entry, so the stage returned `Skipped` (the CLI still prints `ran`; it writes no `fit.json`). No `missing-curve` decision, because that path is only for a named curve whose table is absent.
- **Cost.** No artifact, so no cost header. Wall clock 0.009 s. Driver thinking: none, after I looked at `fit.rs` to see why there was no file. A cheap model that only reads `ran` would look for `fit.json` and stall. That is a CLI lie, not a model-tier problem.
- **Dominates?** No.
- **Cheaper next time.** Print `skipped` when the manifest names no curve, so a cheap driver does not search for an artifact. For palms, an owner rule that a `growth_form: palm` manifest ships without curves would keep this stage a no-op on every such species.
- **Swarm reading.** Free. The only swarm cost is a cheap agent that treats `ran` as "there is a fit.json" and then improvises.

## 2026-09-18 — date-palm gate: 0 credits, 0 Jev calls, 0.020 s, two onboarding-gates, no capability line

- **Step.** Built the missing examples (`species_measure`, `geometry_benchmark`, 22.5 s), then `species-pipeline gate --dir DIR --example`. Required capabilities were empty (no `packet/species.json`, no `engineering.required_capabilities`). `--support date-palm` returned `implemented: false` and `capabilities: []`, so `missing` was also empty and the capability gate passed. Filed `date-palm/gate/onboarding-gate/registry` (`preset date-palm is not registered with a bound profile`) and `date-palm/gate/onboarding-gate/seeds` (`packet/specimens.json does not exist yet`). I did not start the gap loop.
- **Cost.** `gate.json` records 0 Firecrawl credits, 0 Jev calls, 1 run, 0.020 s wall clock. The 22.5 s example build is outside the stage. Driver thinking: medium. I had to read `gate.rs` to see why Corner, frond, and no-secondary-thickening were not named. A cheap model that only reads the artifact would report "halted on registry and seeds" and miss that the form gap was never asked. I wanted a stronger model for that reading, or a gate that prints the missing form in the artifact.
- **Dominates?** No of credits or Jev. Yes of the run's meaning. The literature stages spent 3 Firecrawl credits and 38 Jev calls to reach a halt that does not name the palm form. The first entry in this file said a capability pre-flight would save that spend. This gate would not have fired that pre-flight either, because the check only compares a required list the manifest never wrote.
- **Cheaper next time.** Put the architectural model and the organ list on the seed (the spec already has Corner and frond). Have the gate, or a pre-flight before discover, compare those names to the coverage file. Then a cheap model reads `onboarding-gate/capability` with a line such as `architectural-model:Corner` and stops. Registry-not-registered and specimens-not-yet-written are true and also uninformative: every new species fails both on a first pass, and seeds cannot exist until generate, which gate blocks.
- **Swarm reading.** Gate itself is free (0 Jev, 20 ms). Multiplied by a thousand it stays free. The expensive part is every species whose unsupported form is invisible to the gate, so the swarm spends screen's 33 Jev calls and then parks on `registry`. That is the wrong ticket for the host to route. A form check on the spec's model and organs, before fetch, is the cut that makes a cheap swarm possible for palms and every other unsupported architecture.

## 2026-09-18 — this leg's model-cost reading, fetch through gate

- **Step.** The whole of skill step 3 on `cursor-grok-4.6-high-fast`: fetch, extract, screen, quality, select, verify, fit, gate. No generate, no report, no gap loop.
- **Cost.** 3 estimated Firecrawl credits (all fetch), 38 Jev calls (33 screen, 2 quality, 1 select, 2 verify), about 40 s of stage wall clock plus 22.5 s to build the two gate examples. Driver thinking was mechanical on fetch, extract, screen, select, and fit. Light on quality and verify (read a decision kind, leave it). Medium only at gate, to notice the capability list was empty. Zero escalations. Token count still unmeasurable.
- **Dominates?** Screen dominates Jev (33 of 38). Fetch dominates credits (3 of 3). Gate dominates the outcome (the form gap was not filed). No stage needed a frontier model to *run*. The one place a stronger model would have helped is reading why the expected palm capability was absent from `gate.json`.
- **Cheaper next time.** The cheap tier held. Keep it here. The cuts that matter are extract noise (21 of 33 screen calls were `not_about_tree_size`), a palm-form rule so `dbh_m` is not a per-species owner ticket, cite-check section binding so `claim-unsupported` is not a thousand listed-but-supporting claims, and a form check that files `onboarding-gate/capability` from the spec's model before any scrape.
- **Swarm reading.** A thousand species at this leg is 3,000 fetch credits, 38,000 Jev calls, and a thousand cheap-tier driver runs of about one minute. That is acceptable if the drawable species then generate. It is waste if most species are unsupported forms and the gate still cannot name the form. The cheap model can do this leg unaided. What it cannot do is invent the missing capability line. That has to be a code check, not a smarter driver.

## 2026-09-18 — a tertiary article is a map, not a citation, and discovery treats it as neither

- **Step.** `discover` ranking. For `height_m` Jev ranked a Wikipedia page
  first; the driver dropped it by hand, because the species spec's rule is that
  a tertiary figure is a lead to a primary source and never a citation.
- **Cost.** The rejection itself was free, but the lead was thrown away with
  it. Discovery paid to find the one page whose reference list indexes most of
  the literature it wanted, judged it unusable, and went back to searching. The
  driver then spent its own reading on a research index that returned avocado
  biochar, an insecticide paper, olive radiocarbon and a sister species.
- **Dominates?** Not in credits. It dominates the part of discovery that is
  luck: whether a plain-word web search happens to surface a silvics table.
  Every species pays that lottery independently.
- **Cheaper next time.** Treat a tertiary species article as a **map**: never a
  candidate to cite, always an index to expand. Fetch its reference list only,
  not its prose, and put the primary sources it names into the candidate list
  where Jev ranks them like any other. The owner's rule is unchanged, since
  nothing tertiary is ever cited; what changes is that the lead is followed
  instead of discarded. A species article's references are dense in exactly
  what the pipeline wants, floras, forestry measurement tables and silvics
  literature, and they are already curated by taxon, which is the filter the
  known-source list currently lacks.
- **Swarm reading.** This is the difference between discovery that searches and
  discovery that looks things up. Nearly every tree species on earth has such
  an article with a reference list; almost none has a reliable plain-word
  search path to its measurement tables. At a thousand species, a map per
  species is a thousand curated bibliographies for one cheap fetch each, and it
  makes the first-pass research-index search, which returned nothing usable
  here, droppable. It also degrades safely: a species with no article, or one
  with no references, falls back to today's search.
- **Status.** A proposal, not a spec. The owner agreed on 2026-09-18 that a
  tertiary article is worth having as a map; whether it becomes its own spec
  against the discover stage is theirs to decide.

## 2026-09-18 — the capability gate is vacuous on a first run, and the loop is circular there

- **Step.** `gate`, confirmed by the host in `stages/gate.rs`. Its comment states
  the rule: "The capabilities the packet requires: the generate stage's
  `species.json` when it exists, else the manifest's engineering entry, else
  none." On a new species neither exists. `generate` has not run, because gate
  blocks it; the manifest's `engineering` entry is hand-written and nothing
  derives it from the species spec's architectural model and organs. So
  `required` is empty, `missing` is empty, and the capability check passes for
  every species on its first pass.
- **Cost.** This run: the entire literature chain, 9 estimated Firecrawl
  credits and 40 Jev calls across both legs, to arrive at a gate that never
  asked the question the species was chosen to ask. The two gates that did fire
  are bookkeeping, an unregistered preset and a specimens file that only
  `generate` writes.
- **Dominates?** It decides whether the loop can sort at all. Everything in the
  entry above about a capability pre-flight assumed the gate would answer the
  question once reached. It does not answer it on the pass where it matters.
- **Cheaper next time.** The required list has to come from the species spec,
  which already names the architectural model and the organs, and be written
  into the manifest's `engineering.required_capabilities` at seed time, before
  `discover`. Then a palm files `onboarding-gate/capability` with a named line,
  `architectural-model:Corner` or `frond` or `no-secondary-thickening`, and the
  gap loop has something to route. The check also has to be reachable before
  the literature spend, not only at the stage that sits behind it.
- **Swarm reading.** This is the difference between a swarm that sorts the
  world's species cheaply and one that buys literature for all of them. As
  built, every unsupported species costs a full literature chain and then halts
  on bookkeeping that says nothing about why it cannot be drawn. A thousand
  species is a thousand chains bought before anyone learns which of them the
  field can draw.
- **Consequence for fn-80.** R1 asks for a halt whose gap record carries the
  capability line the halt names. Neither open gate carries one, so this run
  does not satisfy it and the palm cannot reach the loop's routing step until
  the required list exists. Proposal, for the owner: one spec against the gate
  and the seed, making the spec's model and organs the required list. The owner
  decides whether it is written.

## 2026-09-19 — capability assessment: cheap model, if it reads the code that uses the ranges

- **Step.** The CAPABILITY ASSESSMENT stage of `docs/species-onboarding.md`,
  which is not a pipeline command. Read `traits.rs`, `scaffold.rs`,
  `element.rs`, `placement.rs`, `radius.rs`, `twigs.rs`, and
  `capabilities()` in `geometry_benchmark.rs`. Wrote
  `engineering.required_capabilities` and `packet/capability.json`. Reran
  `gate --example` once. Did not run the gap loop and did not mint a spec.
- **Cost.** 0 Firecrawl, 0 Jev, 1 gate run (`gate.json` cost.runs is now 2,
  0.256 s this invoke). Driver thinking: heavy. This is the first step on
  this run that is a judgment over source, not a command. Token count still
  unmeasurable. No escalation.
- **Dominates?** Yes of this leg, and of whether the gap loop has a true
  halt. The literature chain already spent 9 Firecrawl credits and 40 Jev
  calls to reach a vacuous gate. This leg spent none of those and filed
  `date-palm/gate/onboarding-gate/capability`.
- **Was a cheap model enough?** Yes, unaided, provided it reads the
  *use* of each trait and not only `validate()`. I am that cheap tier
  (`cursor-grok-4.6-high-fast`). I was not guessing on the ranges or on
  `lateral_orders=0` and `length_taper=0`. I was guessing if I had stopped
  at the `laterals_per_station` rail (1..=12), which looks like "cannot have
  zero laterals" until `scaffold.rs` shows that 0 orders never stations one.
  Persistent leaf base (organ versus bark plate) and whether a date spadix
  fits fn-33's one organ element are the two places I marked unsure rather
  than deciding. Guessing either as a new gap, or guessing the unbranched
  stem as unreachable, would have been worse than useless.
- **Cheaper next time.** Keep this on the cheap tier. Do not send it to a
  frontier model. What a thousand species cannot afford is each agent
  rediscovering that the assessment is not a command and rereading six
  source files. Put the spec's model and organs on the seed, and have a
  mechanical check emit the required list before discover. A cheap driver
  then only reads `capability.json` for the two unsure rows.
- **Swarm reading.** One cheap-tier invoke that reads the trait space, 0
  credits, 0 Jev, then one 20 ms gate. Acceptable at a thousand if the
  required list is derived from the spec. Unacceptable if every species
  pays a 20-file source read to write six strings the spec already named.

## 2026-09-19 — the capability assessment, and what it says about the escalation line

- **Step.** The onboarding method's capability assessment, the stage the skill
  never conducts, run on the value tier before the owner ruled that reasoning
  and system design escalate to the host. Then verified by the host.
- **Cost.** Under four minutes, one bridge call, no credits. The host's
  verification was three source reads and one generation trial.
- **What it produced.** Six required capabilities on the manifest, and a
  per-trait record separating what the trait space reaches from what no trait
  expresses. `gate`, rerun, filed
  `date-palm/gate/onboarding-gate/capability` naming the missing list. That is
  the first real capability halt the pipeline has ever filed, and it is what
  the gap loop needs to route.
- **Quality, checked not assumed.** The host verified the load-bearing claims
  against the source rather than accepting them. `lateral_orders = 0` does
  yield an unbranched axis: `scaffold.rs` stations a lateral only while
  `axis.order < habit.lateral_orders`, and the validity check admits 0. The
  twig rail is 1 to 6, so twigs cannot be switched off, which is why an apical
  rosette is unmet. A generation trial over the oak's family with
  `lateralOrders: 0`, `lengthTaper: 0` and an 18 m envelope produced a single
  unbranched stem measuring 17.99 m tall and 0.648 m in constant diameter. So
  the palm's TRUNK is reachable today as a value table; what is missing is the
  organs and their placement, not the form of the stem. The assessment was
  right and the host's earlier reading, that no combination of values produces
  a palm's trunk, was wrong.
- **The escalation reading, which is the point.** The value tier did this well:
  it read the rails from source, refused to call a reachable value a gap,
  found that two of the six needs are already covered by open fn-33, and marked
  two judgments unsure rather than deciding them. It also caught a defect in
  the gate itself, that `woody-axes` shows as missing only because `--support`
  returns an empty list for an unregistered preset. That is a strong showing
  for a cheap model on a reasoning step.
  It does not overturn the owner's rule. The rule holds because the failure
  mode is asymmetric and invisible: a wrong reading is caught by the next
  stage, a wrong capability judgment is written into an artifact and then
  believed. This run happens to be a case where the cheap tier was right, and
  nobody could have known that without the host checking. The check is the
  cost of the rule, and it is far cheaper than the assessment itself.
- **Swarm reading.** The shape that works is a cheap tier that drafts and a
  host that verifies, not a cheap tier that decides. Drafting cost four
  minutes; verifying cost three reads and one trial. A thousand species can
  afford both. What a thousand species cannot afford is a wrong form judgment
  written into a manifest and discovered after the generator work is built.

## 2026-09-23 — owner: the run's token cap is 6 M

The conductor's cap was 2 M, set by the host on 2026-09-22; the palm had spent 1.61 M, most of it the first tuning revision (about 1.64 M over seven rounds on Opus). The owner chose 6 M total so the run continues without stopping to ask, matching the tuning run's own ceiling. The conductor has no cap-extension command, so `budget.max_tokens` was raised in both its config and its run record, and `max_tuning_revisions` from 2 to 4 so tokens are the binding limit. Cheaper next time: a `species-conductor resume` decision carrying a cap extension, as `tuning-loop` already has.

## 2026-09-23 — host: a convergence guard on the literature step

The literature step took three rounds of fixes from three live reruns on the palm (fn-118, fn-127, fn-128), each finding smaller defects than the last; the palm's sizes went from one of six filled to five of six. Converging, but it is patching; the root cause was a host spec (fn-118) written without tracing a new field through every stage. Guard: if the next rerun finds a fourth defect in the same step, the host stops patching and reviews the step's design whole before another spec. Worker spend for the day's fourteen dispatches is about 3 M tokens (150 to 290 k each), beside the palm's 9.35 M of tuning.
