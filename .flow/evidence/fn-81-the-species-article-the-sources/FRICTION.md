# fn-81 friction

## 2026-09-19 — no fetch cache survives a pipeline run, so every source is re-fetched

Doing: planning the source copies, looking for text the repository could
already reach so the spec's budget rule ("prefer a source the repository can
already reach over a fresh fetch") could be honoured.

What slowed it: nothing on disk holds the fetched text of any catalogue
source. `.gitignore` ignores `.flow/evidence/*/pipeline/cache/` and
`.firecrawl/`, and neither directory exists in this checkout or the main one.
`.flow/evidence/european-ash/` holds only a `README.md` that says the run's
cache "stays under `pipeline/`" — a directory that is gone. The only markdown
under `.flow/evidence` is fn-58's validation fixtures, which are 84-byte
synthetic stand-ins, not source text. So the catalogue records seven distinct
rights statements, 22 sources and a `verified` date per source, and can reach
none of the text those dates were taken against.

Cost: about 20 minutes of searching, and 22 fresh Firecrawl fetches this task
would not otherwise have needed — the whole of the spec's network budget.

What would have removed it: the fetch stage's cache is the pipeline's only
copy of a source, it is ignored by git, and nothing promotes it. A run that
ends deletes the evidence its own `verified` dates rest on. Either the cache
should live somewhere durable per species (a machine-local store keyed by the
source's sha256, outside the run directory), or `document` should run inside
the same run as `fetch` so the copy is written while the cache is still warm.
This task's `document` stage reads the cache, so the second is nearly true
already — the gap is only that the five species in the catalogue predate it.

## 2026-09-19 — a `verified` date can point at a source no run can reach

Doing: writing the silver birch's source copies from the fetch cache.

What slowed it: `TSO-BIRCH`'s recorded URL returns the site's own 404 and
`ATKINSON-BIRCH` is a JSTOR DOI behind a log-in wall, yet both carry
`verified: 2026-09-14` in `sources.json` with `sha256: null`. Nothing in the
record distinguishes "a person read this text" from "this URL once resolved".
`TSO-BIRCH` is the sole citation behind the birch's `crown_width_height_ratio`
and a co-citation behind both gating blade dimensions, so a gating metric now
cites text no later run can reach. The European ash has the same shape from the
other side: `KEW-ASH` fetches cleanly, but the POWO taxon page carries name,
distribution and synonyms only — neither the height nor the leaflet count the
catalogue cites it for appears anywhere on it.

Cost: no lost time on this run, because the fetch pass found both before the
writing started. On a run that had not, it is the whole budget: an agent would
retry a dead URL, then hunt a mirror, then guess.

What would have removed it: `verified` requiring a stored checksum, so a null
`sha256` means unverified by construction; and the citation check the `document`
stage now runs being run against the source at admission time, so a source that
does not carry what it is cited for is caught when it is admitted rather than
two specs later.

## 2026-09-19 — the shared tooling moved under four parallel writers

Doing: backfilling five species in parallel while still finishing the writer
and the check they run against.

What slowed it: `scripts/catalogue-sources.mjs` gained a `source_sha256` front
matter field and `catalogue-article.mjs` tightened its number rule while the
four per-species agents were mid-run. Copies already written went stale against
the new check, and three of the four agents had to re-run the backfill and the
article command. A writer has no signal that the contract it is writing against
has moved; it only sees a check that used to pass and now does not.

Cost: five to ten minutes across the four agents, plus one wholesale rewrite of
all twenty-two source copies at the end.

What would have removed it: landing the tooling before dispatching the writers,
rather than alongside them. The parallelism was still the right call — the
reading is the expensive part and it genuinely fanned out — but the fan-out
should start from a frozen contract, not a moving one.
