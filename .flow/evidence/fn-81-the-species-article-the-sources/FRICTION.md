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
