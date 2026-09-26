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

## 2026-09-26, worker, fn-157.1: the measurer refused Start's profile twice

- **Doing:** the beech's live Tune revision 1 (R8) after Start wrote the profile manifest.
- **Hindered:** species_measure takes only a frozen `ready` manifest with a `ready` profile, then only `gating` or `contextual` metrics; a derived profile is `draft` and its unsourced fields `unsourced`. Each refusal surfaced as Tune's "baseline infeasible" or a bare measurer error, one per revision.
- **Cost:** two failed revisions (two visual passes, about 3,200 Jev tokens) and about 25 minutes.
- **Would have removed it:** Start writing the measurer's copy frozen, ready and gating-or-contextual (done in this task, 87c38b48), and Tune naming the measurer's reason in its stop line.

## 2026-09-26, worker, fn-157.1: a one-source value gates as a point

- **Doing:** Tune revision 1 on the native-range beech.
- **Hindered:** trunk diameter rests on one nursery page (1.5 m, `thin`); its range is the point [1.5, 1.5] and it gates, so the baseline tree's 1.49 m fails the numeric gate and Tune stops "baseline infeasible" before any round.
- **Cost:** one revision (1,579 Jev tokens, one visual pass); R8 stops here until the host decides.
- **Would have removed it:** a rule for how a thin value gates (contextual, or a tolerance around a point). Left to the host: the spec does not say.

## 2026-09-26, worker, fn-157.1: a photograph found from a name has no matched shot

- **Doing:** the beech's Tune revision 1 after the agreement rule made its numeric gate pass (height 39.97 m and crown 13.4 m gating and passing; trunk diameter contextual).
- **Hindered:** the baseline is still infeasible, now "no matched shots or height": `tuning/matched.rs` compares only against references that carry a `shot` (camera azimuth, elevation, field of view, crop), and the photograph the Profile stage keeps carries none. The tuning config's `numeric_references` and required cells name the shipped beech's hand-matched `B-WHOLE`, which a run from a name never has. No code derives a shot.
- **Cost:** one more revision (one visual pass); R8 cannot reach a round from a name.
- **Would have removed it:** a rule for who writes a found photograph's shot and which reference Tune's numeric comparison names on a run from a name. Left to the host: a system-design choice.

## 2026-09-26, worker, fn-157.1: tape_trim --check exits 0 on a finding

- **Doing:** trimming the beech and oak recordings for the fixtures.
- **Hindered:** `tape_trim --check` prints each page that keeps too much but exits 0, so a script chaining it with `&&` goes on; the replay test catches it later, after a full replay.
- **Cost:** one replay (about two minutes) before the finding was noticed in the output.
- **Would have removed it:** `--check` exiting nonzero when it lists anything.

## 2026-09-26, worker, fn-157.1: Tune asked Jev about 227 dials in one call

- **Doing:** the beech's live Tune revision 1, after its shot was chosen and the baseline judged.
- **Hindered:** the tuning config names no dial, so a revision offers every row of the dial table (227, the palm's fronds and spines among them), and with no `max_questions_per_call` the first targeted proposal asked all 227 in one request (302 KB); the service refused it with a bare 400. The revision stopped after its paid baseline look.
- **Cost:** one interrupted revision (the shot looks and the baseline visual pass, replayed on the rerun) and about 20 minutes.
- **Would have removed it:** a config that names the dials a species tunes, or a default cap on questions per call; `--init-tuning` (fn-165) is where either belongs. This run sets `max_questions_per_call` 32 in the recorded configs.

## 2026-09-26, worker, fn-157.1: Tune spent 2 million Jev tokens on rounds that could keep nothing

- **Doing:** the beech's Tune revision 1 through Gaps (R8).
- **Hindered:** from round 3 every bundle widened the crown past the gating crown width (10.7-18.3 m, two American extension pages) while the photograph shows a crown about as wide as the tree is tall; every candidate failed the numeric gate, and the revision ran five such rounds before the runaway stop. Every round asked Jev about all 227 dials in eight calls.
- **Cost:** about 2.04 million Jev tokens over 63 calls, 33 evaluations, 70 images and 7 visual passes for one kept round.
- **Would have removed it:** a stop after the first round whose every bundle fails the same gate, naming the gate; and a dial set sized to the species. Whether a crown range from landscape pages should gate against a photograph that disagrees is the host's call.

## 2026-09-26, worker, fn-157.1: gaps.md is 306 KB

- **Doing:** reading the beech's Gaps result.
- **Hindered:** every reachable gap repeats the full list of render links for every dial it names; five gaps make a 306 KB file a person cannot read.
- **Cost:** a few minutes and a truncated read.
- **Would have removed it:** one link per render pair, or the links in `gaps.json` only.

## 2026-09-26, worker, fn-157.1: a Tune recording does not replay from another directory

- **Doing:** trimming and replaying the beech's recording through Tune's first revision for the gate (R8).
- **Hindered:** four things in the requests named the run, not the question, so a replay from a scratch directory asked something the recording lacked: a render's geometry group (the run identity), the shot source's hash (a file holding the run's paths and dates), the inventory's full hash in the coverage finding and evidence Jev reads, and the sheet's render order (a hash of trial keys, which hash the run identity). Each was fixed with a test first; the recording's Jev requests were migrated for the inventory and refiled (`tape-adapter.py rekey:`, `tape_trim`), but the first round's sheet was recorded in the old order and its answer cannot be moved.
- **Cost:** five replays of about two minutes each and about two hours; the gate's Tune replay stays red until the revision is recorded again.
- **Would have removed it:** a replay of a fresh recording from a second directory as part of recording Tune, which would have named all four at once.

## 2026-09-26, worker, fn-157.1: the capped rerun, gaps.md at 1.3 MB

- **Doing:** rerunning the beech's Tune revision with the tier gating rule and a three-round cap.
- **Hindered:** the same two costs the host asked to have proposed at close: every round again asked about all 227 dials (about 0.86 million Jev tokens for three rounds), and `gaps.md` grew to 1.3 MB, one render-link list per dial per gap.
- **Cost:** 26 Jev calls, 7 visual passes, 17 evaluations; an unreadable gaps file.
- **Would have removed it:** the fixes named in the two entries above.

## 2026-09-26, worker, fn-157.1: the harness cannot show the tree the owner is asked to look at

- **Doing:** telling the host how the owner looks at the beech's kept tree (the next stop).
- **Hindered:** `docs/species-runner.md` and `runner/accept.rs` say the owner looks at the tuned tree in the harness, but the harness loads a species by `?species=<id>&seed=<n>` (the preset as shipped) and has no way to load the overlay in `runner/tuning/result.json`; the kept tree reaches a preset only with `--accept`, the decision the look is for. The only views are stills: the revision's matched renders under `run/matched/<key>-*/` and a headless still rendered with the overlay as `--family`.
- **Cost:** about 15 minutes to establish that no URL exists.
- **Would have removed it:** a harness parameter that loads a run's kept overlay (or a runner command that serves it). A harness change is the host's to spec.
