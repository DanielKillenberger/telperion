# fn-157 friction

## 2026-09-26, worker, fn-157.1: the recorded run cannot replay a changed literature stage

- **Doing:** reading `tests/replay.rs` and the beech tape before replacing discover, admission, screen, quality, select and search.
- **Hindered:** the tape keys every Firecrawl and Jev answer by its request. The new stages ask different searches (one broad gather) and a new Jev question (a label per span), so no answer they need is on the tape, and the recording cannot be extended offline without inventing answers. The replay test through Start stays red from the first commit that changes the literature stages until a live run re-records it, which the Firecrawl budget (22 credits) forbids in this task.
- **Cost:** about 20 minutes to confirm there is no honest offline path; the gate cannot be fully green at hand-back.
- **Would have removed it:** a replay mode that serves a recorded page by URL to any stage that asks for it (scrapes are content, not judgment), so a pipeline change needs only its new Jev answers recorded; or budgeting the re-record into the spec's first live run.

## 2026-09-26, worker, fn-157.1: Firecrawl refuses past the plan's credits

- **Doing:** the beech's first live run from a bare seed (`--until profile`, recorded).
- **Hindered:** the host reported overage billing accepted, but Firecrawl answered "Insufficient credits to perform this request" once the 22 credits were spent: 10 of 16 gathered documents fetched, two dropped for credits (iNaturalist, a hedge nursery). Nothing Firecrawl-backed can run again (the shipped species of R5, a re-fetch after a fetch fix).
- **Cost:** two documents lost from the beech's evidence; R5 blocked.
- **Would have removed it:** raising the plan or enabling auto-recharge on the Firecrawl account before the run; `--status` could fail when the credits left are below the Sources estimate.

## 2026-09-26, worker, fn-157.1: a failed later inner stage reruns the paid earlier one

- **Doing:** rerunning Profile after Jev answered HTTP 520 in aggregate.
- **Hindered:** `literature::refresh` drops every inner record of a stage when any of its outputs is missing, so the rerun repeated read's 82 recorded label calls although read.json was complete. The caller does not retry a 5xx other than 529, so one Cloudflare 520 failed the stage.
- **Cost:** 82 Jev calls repeated, about two minutes.
- **Would have removed it:** refresh dropping only the records whose own outputs are gone; the caller retrying 500, 502, 503, 504 and 520 like 529.

## 2026-09-26, worker, fn-157.1: an archived PDF read as text

- **Doing:** reading the live beech's documents.
- **Hindered:** web.archive.org served the EUFORGEN PDF as `text/html`, so fetch took the raw bytes' lossy text for the page; read found 31 "spans" in the binary (9M, 65Mm) and Jev labelled each `none`.
- **Cost:** 31 Jev calls and the EUFORGEN guideline's 30-35 m lost as evidence.
- **Would have removed it:** fetch taking a body that starts `%PDF-` as a PDF whatever its content type. Not changed in this task: a fetch change reruns fetch, and a replay of this recording then asks for a parse the tape lacks while Firecrawl has no credits to record one.

## 2026-09-26, worker, fn-157.1: Tune from a name found no profile manifest

- **Doing:** the beech's first live Tune revision (R8) after a clean Start.
- **Hindered:** the tuning config names a measurer profile manifest (`profiles`) that a run from a name never has; Tune failed at once with "No such file or directory", naming no file.
- **Cost:** about ten minutes to find which of the config's paths was missing.
- **Would have removed it:** Start writing the derived profile there when none exists (done in this task), and the revision's error naming the missing path.
