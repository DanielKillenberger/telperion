# fn-80 economics: what a species costs and what would make it cheap

The goal this serves (owner, 2026-09-18): most tree species in the world,
onboarded by swarms of cheap, fast agents running the `add-species` loop. The
unit that matters is therefore cost per species at swarm scale, not the cost of
one careful run. A loop that is correct but expensive does not reach the goal.

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

