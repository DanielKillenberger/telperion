# fn-157 friction

## 2026-09-26, worker, fn-157.1: the recorded run cannot replay a changed literature stage

- **Doing:** reading `tests/replay.rs` and the beech tape before replacing discover, admission, screen, quality, select and search.
- **Hindered:** the tape keys every Firecrawl and Jev answer by its request. The new stages ask different searches (one broad gather) and a new Jev question (a label per span), so no answer they need is on the tape, and the recording cannot be extended offline without inventing answers. The replay test through Start stays red from the first commit that changes the literature stages until a live run re-records it, which the Firecrawl budget (22 credits) forbids in this task.
- **Cost:** about 20 minutes to confirm there is no honest offline path; the gate cannot be fully green at hand-back.
- **Would have removed it:** a replay mode that serves a recorded page by URL to any stage that asks for it (scrapes are content, not judgment), so a pipeline change needs only its new Jev answers recorded; or budgeting the re-record into the spec's first live run.
