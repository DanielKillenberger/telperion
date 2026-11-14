# fn-306 friction

## 2026-10-22 08:25, conductor, flow --auto start: the host-dispatched spec was not ready

- **Doing:** starting `flow --auto` on fn-306 as the host dispatched it.
- **Slowed by:** the spec was captured without `ready: true`, so the pilot snapshot read it ineligible, and its route was `work_no_plan_default` though the dispatch asked for plan and plan-review. The conductor marked it ready on the host-dispatch precedent (fn-297, fn-283, fn-291) and took the plan route on the dispatch's word.
- **Cost:** about 3 minutes and four tool calls to read the precedent.
- **Would remove it:** the host marks a spec ready (and records its plan route) in the capture commit when it dispatches the spec in the same breath.

## 2026-10-22 08:40, conductor, task .1: the spec's "Quick commands" and walk-frame parse did not exist

- **Doing:** planning R1 and R3.
- **Slowed by:** R1 named "AGENTS.md's Quick commands", a section AGENTS.md does not have, and R3 named "the same parse the film-walk tests use", which does not exist (they address frames by segment, seed and number). Both were unchecked architecture claims; the plan had to place the line and write the parse.
- **Cost:** one scout and about 5 minutes of plan rework; no review round.
- **Would remove it:** checking a spec's named sections and functions against the code before capture, as AGENTS.md "Gates and checked claims" asks.

## 2026-10-22 08:43, conductor, task .1: dcg blocked a redirect to a variable path

- **Doing:** saving a nextest listing to the evidence `raw/` directory through a `$R` shell variable.
- **Slowed by:** dcg's `redirect-truncate-dynamic-path` rule blocked `> $R/list.log` inside a subshell (the same form passed outside one minutes earlier).
- **Cost:** one retry, under a minute.
- **Would remove it:** literal paths in every redirect (local habit; nothing to spec).
