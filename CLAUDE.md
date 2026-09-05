# Telperion

A runtime tree generator: space colonization for the crown, botanical rules below the crossover, one bias field for everything supernatural. Strategy lives in `STRATEGY.md`; specs and tasks in `.flow/` via `flowctl`.

<!-- flow-next:model-routing:start -->
## Model routing

<!-- Grammar: <tier>: <model>   or   <tier>: <model> at <effort>
     Resolution at each dispatch site: an explicit instruction in the moment,
     then this block, then the agent definition's own default, then the session
     model. A model this harness cannot reach falls back to the session model
     with one note. -->

implementer: gpt-6-astra at low
reviewer: claude-fable-5-1

<!-- The owner's choice (2026-09-05): Codex gpt-6-astra implements at low
     effort. Per-task code review is not wanted: run work with --review=none.
     When a code review is wanted, it is host-native on Fable (the reviewer pin
     above, a different family from the writer). The plan review and the
     spec completion review at the end of a spec stay on Codex at high effort
     (`.flow/config.json` review.backend = codex:gpt-6-astra:high). The live QA
     pass (/flow-next:qa, pipeline.qa on) runs on the host, because it drives
     the harness in a browser. -->
<!-- flow-next:model-routing:end -->
