# Friction reports and loop notes, fn-56 (the ash through the fn-58 pipeline)

The first cheap-driver run of the species pipeline, 2026-09-18, Grok 4.6 fast
through cursor-agent, run directory `.flow/evidence/european-ash/pipeline`.
Entries follow the friction rule in CLAUDE.md; the owner decides which become
specs.

## 2026-09-18 19:05 discovery did not find the source the repository already knew

- **Doing:** the driver ran `discover` on the seed manifest (two fields, height and trunk diameter at 20, 50 and 80 years, open-grown).
- **Hindered by:** the web and research searches returned ten hits per field and Jev ranked none first for both: the age-indexed yield table for the ash (Ertragstafeln extract, `Esche`, eleven rows from 20 to 120 years) was not among them, although fn-11 recorded that PDF's URL, fn-58 parsed it the same day and the oak and spruce fixture manifests admit it. The research index returned ozone and sycamore papers.
- **Cost:** one discover run (28 s, 3 Firecrawl credits, 2 ledger entries) that proposed no source; the host supplied the sources by hand from the repository's own evidence.
- **What would remove it:** discovery seeds its candidate list from the sources every admitted manifest under `.flow/evidence/*/manifest.json` already names, and from the URLs the specs' `Resolved via Research` sections cite, before it searches the web; and the query is written in plain words ("Fraxinus excelsior height at age, open grown"), not the field id (`height_m open_grown by age`).
- **Early return:** taken by design: the stage stopped on the manifest-proposed decision.

## 2026-09-18 19:10 one markdown table holds five species

- **Doing:** admitting the Ertragstafeln extract's ash table into the manifest with `table_index` and `expected_rows`.
- **Hindered by:** Firecrawl's parse puts the ash, alder, birch and two poplars into one markdown table (index 5, 37 age-indexed rows) with a species-name row between blocks. An admitted table is one markdown table, and the coverage guard in `stages/fetch.rs` is one-sided: it files coverage-gap only when the parse yields fewer rows than the manifest states, so `expected_rows: 11` against 37 parsed rows passes and the fit reads five species' rows as the ash's. The guard catches a flattened table and not a merged one.
- **Cost:** the first live fit on the ash cannot be trusted until the block is cut; measured on this run.
- **What would remove it:** an admitted table names its block (the species-name row that opens it) as well as its index, the row parser cuts the block at the next name row, and the coverage guard files a decision when the count differs in either direction.
- **Early return:** not applicable.

## 2026-09-18 19:25 admitting the manifest voids its own admission

- **Doing:** the driver resumed the runbook after the owner admitted the drafted manifest and the host wrote the resolution for the manifest-proposed decision.
- **Hindered by:** `discover`'s idempotence key includes the manifest checksum, so the admitted manifest made discover run again (21 s, 3 more credits), which reissued the manifest-proposed decision under a new draft checksum; the resolution, bound to the old draft, was void and fetch stopped a second time on the same decision. Every admission reissues the proposal once.
- **Cost:** one wasted driver dispatch and one extra discovery; the host re-bound the resolution by hand.
- **What would remove it:** discover keys its proposal on the seed manifest's fields (taxon, fields), not the whole manifest, or it skips proposing when the manifest already admits sources; and a resolution to a manifest-proposed decision is bound to the admitted manifest's checksum rather than the proposal's draft.
- **Early return:** taken by the stage; the host intervened.

## 2026-09-18 19:40 a mainstream page fails on TLS in the raw fetch

- **Doing:** the driver ran `fetch` over the four admitted sources.
- **Hindered by:** the Ertragstafeln PDF, the JRC atlas PDF and the OSU page fetched and cached with matching checksums; the Missouri Botanical Garden Plant Finder page stopped the stage with `tls connection init failed: invalid peer certificate: UnknownIssuer` from the adapter's plain second request (the one that records the raw bytes), while Firecrawl's scrape of the same URL would have succeeded. The page duplicates the OSU one, so it was dropped.
- **Cost:** one driver dispatch (52 s) and one decision resolved by hand.
- **What would remove it:** the raw request uses the system certificate store (ureq's native-tls or rustls-native-certs) instead of the bundled webpki roots, or an unavailable source that Firecrawl did scrape falls back to Firecrawl's raw HTML with the fidelity difference recorded, as the parked check already measured.
- **Early return:** taken by the stage.

## 2026-09-18 19:50 a resolution option nothing acts on

- **Doing:** resolving the unavailable-source decision for M1 with `drop-source`, one of the three options the decision itself offers, and resuming the driver.
- **Hindered by:** `stages/fetch.rs` never reads a resolution's option: it retries every source the manifest names, fails on M1 again and refiles the same decision. `replace-source` and `drop-source` are choices a person can record that no stage consumes; only editing the manifest changes the outcome, and that reissues the manifest proposal (entry 3).
- **Cost:** one wasted dispatch (24 s) and two more to come (drop M1 from the manifest, re-bind the admission, resume).
- **What would remove it:** fetch reads the resolutions for its own decisions before fetching: `drop-source` skips the source and records it as dropped in fetch.json, `replace-source` reads the replacement from the resolution's payload; the decision kinds table in the spec states which stage consumes each option.
- **Early return:** taken by the stage; the host intervened.

## 2026-09-18 20:05 the merged table reached the fit and produced no curve

- **Doing:** reading the fit stage's output after the driver ran discover through gate green.
- **Hindered by:** fetch recorded 31 age-indexed rows for the admitted ash table against the manifest's 11 and filed nothing (entry 2's one-sided guard, now observed); the fit read the five species' rows as one series and answered none of the three reference ages, so it filed missing-curve for height rather than a wrong curve. The safer outcome, by luck of the interpolation, not by a guard.
- **Cost:** the run has no height curve for the ash although the table with the eleven right rows is on disk.
- **What would remove it:** entry 2's block selector and a two-sided coverage guard.
- **Early return:** taken by the stage.

## 2026-09-18 20:05 one species run cost 107 Firecrawl credits

- **Doing:** reading `firecrawl credit-usage` after the run: 588 before, 481 after.
- **Hindered by:** discover ran three times (the seed, the admission, the M1 drop), each time two web searches and two research searches, and fetch scraped three sources including a 6.8 MB atlas PDF through parse. The parked check budgeted 12 credits for three probes; a real species is an order of magnitude more, most of it the repeated discovery.
- **Cost:** 107 credits, about a tenth of the monthly plan, for one species that ended with one copied value.
- **What would remove it:** entry 3's fix (discovery keyed on the seed, not the whole manifest) removes two of the three discover runs; a credit line in the report per stage makes the cost visible before the next species.
- **Early return:** not applicable.

## Loop notes, end of run

- What worked as designed: the driver ran eleven commands and read nothing; the screen kept the site-quality criterion out and kept the two mature-range sentences; the gate scored both fields proxy-only and filed data-insufficient for the diameter against its bar of partial; select copied `20-35 m` from the JRC atlas with its ledger reference and left the diameter unavailable; verify supported the claim and the measurement obligation held; the onboarding gate stopped generate on the registry, which is the ash's real gap (no preset until fn-33 delivers the compound leaf).
- The run ended with four open decisions a person owns: missing-curve for height (fix the table block), data-insufficient for dbh (admit a proxy or add a source), the registry gate and the seeds gate (the species needs its preset). This is where fn-63's loop starts: propose the fixes, route them, mint the specs.
- Six driver dispatches for one species, three of them wasted on the reissued proposal and the unconsumed resolution option. With entries 3 and 5 fixed, the run is two dispatches: one to the manifest proposal, one to the end.
- 26 ledger entries, every one cited from the artifact that consumed it.

## Loop notes (not friction, things to improve)

- The driver did exactly the runbook: two commands, stopped on the open decision, read nothing. Twenty-eight seconds. The cheap tier is enough for this shape.
- The seed manifest is written by hand before discover can run. A `species-pipeline seed --taxon ... --profile FILE` that drafts it from an fn-34-style profile would remove that step.
