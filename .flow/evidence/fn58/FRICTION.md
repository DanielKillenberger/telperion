# Friction reports, fn-58 session

## 2026-09-18 14:37 workspace test run is slow

- **Doing:** running `cargo test --release --workspace` once before a chore commit of three small hooks (a parameter overlay in core, a `--family` flag on two examples, an `ask` subcommand on jev), whose own unit tests had passed in 0.7 s.
- **Hindered by:** the run had taken 9 minutes at the time of writing and was still inside the core `species` suite, which generates full fixed-seed specimens of every catalogue species (oak, spruce, beech, birch) at about 10 s each; 30 suites had reported green before it. The background shell's 10-minute cap sits right at the run's length, so a green run can also be lost to the harness.
- **Cost:** 10 minutes of wall clock (14:27 to 14:37, exit 0, 30 suites green) and a blocked commit, for a change the species suite does not exercise.
- **What would remove it:** a `test:quick` command (or a cargo alias) that runs every suite except the specimen-generating ones, with the full run reserved for landing; or caching the generated specimens under a content key of the preset wire so a run that changes no preset rebuilds none; or scoping the pre-commit run to the crates a diff touches. Any of the three turns the pre-commit check into under a minute.
- **Early return:** not taken; the run is the landing gate and was already past its midpoint.

## 2026-09-18 15:20 implementer tier unreachable, codex quota exhausted

- **Doing:** resolving the implementer tier for fn-58.1 per the routing block (`implementer: gpt-6-astra at high` over the codex CLI bridge) and probing reach with a one-word `codex exec -m gpt-6-astra` call from the worktree before composing the long-task brief.
- **Hindered by:** codex-cli 0.154.0 answered `You've hit your usage limit ... try again at Sep 19th, 2026 4:27 PM` on the probe (`rc=1`, no digest written). The bridge cannot run for this task today.
- **Cost:** about 3 minutes for the probe; the task now runs on the session model (Fable 5.1) instead of the routed implementer, as the routing block's 2026-09-08 to 2026-09-11 precedent did.
- **What would remove it:** a reach probe in the conductor before dispatch (one `codex exec` with a one-word prompt, ~10 s) so the routing decision is made with the quota state known, and a routing-block note naming the fallback tier explicitly so no worker has to infer it from the history paragraph.
- **Early return:** not taken; the worker phases define the fallback (session model, stated once) and the owner's precedent matches it.

## 2026-09-18 15:52 a Choice's criteria cannot carry an order

- **Doing:** building the fn-58 ranking Choice over candidate source ids so the acceptance test could assert that the criteria preserve table order and end with the `none` no-match key, the way the described Score's level array does.
- **Hindered by:** `serde_json` is declared without `preserve_order`, so `serde_json::Map` is a `BTreeMap` and a `Value::Object` sorts its keys. A Choice's criteria are an object (that is the live-proven shape `selection_questions` uses), so no construction can put `none` last or keep candidate order. Turning the feature on is not local: it is one workspace-wide feature unification that would change every crate's JSON key order, including preset output.
- **Cost:** about 20 minutes weighing three shapes (object plus a sibling `order` array, an array of `{key, what}` entries, keeping the object), with no way to settle it by a live probe because the task forbids network calls.
- **What would remove it:** an owner line on whether a Choice's criteria may be an ordered array, or a `data/questions/README` naming the one request shape every set must use. Either removes the guess. In the meantime the ranking Choice keeps the object and the test asserts that every candidate and the no-match key are present, not their order; the described Score keeps its array and is order-checked.
- **Early return:** not taken; the object shape matches the sets already running live, so the work continued on the proven shape.

## 2026-09-18 16:50 parallel module owners broke the crate build for each other

- **Doing:** four subagents implementing disjoint modules under `crates/telperion-jev/src/pipeline/` in one working tree while the host wrote the core contracts.
- **Hindered by:** a module declared before its file existed (`adapter.rs` naming `mod firecrawl; mod tables;`) and half-written siblings kept `cargo test -p telperion-jev` and `cargo clippy --all-targets` red for about fifteen minutes per agent; each agent retried its gates two or three times and scoped them to `--lib` or `--test <name>` to get a signal, and one built a throwaway crate in the scratchpad for a red/green loop.
- **Cost:** roughly 15 minutes of retries per agent (four agents) and one agent's detour into a scratch crate; no wrong code shipped.
- **What would remove it:** one worktree per module owner with the host integrating, or a rule that a submodule declaration lands in the same write as its file and that each owner gates with `--lib`/`--test <own file>` until the host runs the crate-wide gate.
- **Early return:** not taken; the retries converged and the crate-wide gate is the host's.

## 2026-09-18 17:35 float round trips were not byte-stable

- **Doing:** the downstream stage owner testing that a rerun of the fit stage writes nothing new, comparing `decisions.json` bytes before and after.
- **Hindered by:** `serde_json` parses floats on its fast path without the `float_roundtrip` feature, so `0.23570950225444562` read back and rewritten became `0.2357095022544456`; every `Context::open` rewrites the decision list after reconciling resolutions, so a payload's numbers drifted by one ULP on the first rerun and R6's byte-for-byte comparison would have failed on a rerun with no change.
- **Cost:** one test rewritten to compare the stage's own artifact instead, and about 20 minutes to find the cause.
- **What would remove it:** the fix landed in `canon.rs` (canonical bytes are the fixed point of serialize-then-parse, with a test); turning on `float_roundtrip` workspace-wide would also remove it but unifies the feature into the core, render and wasm crates' parsing and is the owner's call.
- **Early return:** not taken; the fix is local.
