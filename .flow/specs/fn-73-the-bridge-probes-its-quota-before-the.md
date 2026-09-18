# The bridge probes its quota before the brief

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 60% [paraphrase], 20% [inferred] -->

The routing block sends implementation to `codex exec -m gpt-6-astra`. On 2026-09-18 the fn-55 worker composed the long-task brief, dispatched, and the CLI answered `You've hit your usage limit ... try again at Sep 19th, 2026 4:27 PM` within seconds; the whole task then ran on the session model. The brief was composed for nothing, the routing decision was made after the fact, and the run's summary is the only place the fallback is recorded. Filed as friction by that build. [paraphrase]

This spec makes the quota answer known before a brief exists: one cheap probe of the bridge, its answer recorded where the run and the owner can read it, and the fallback a decision rather than an accident. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **A probe is a trivial `codex exec`.** One short prompt with `--skip-git-repo-check` from a scratch directory, a bounded timeout, and the exit code and first line captured. A usage-limit answer carries the retry time; the probe parses it out. [inferred]
- **The answer is written before dispatch.** A line under the spec's evidence, `BRIDGE: gpt-6-astra <available|limited until <time>>`, and the same line in the task's summary, so the model that actually implemented is never inferred from the summary alone. [inferred]
- **Fallback is the routing block's word.** When the probe says limited, the worker takes the session model as it does today and says so in the same line; no retry loop, no waiting. [paraphrase]
- **Where it lives.** A script under `scripts/` the worker calls per the project instructions, never a change to the flow-next plugin. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A probe script answers available or limited with the retry time in under ten seconds, on the live CLI and on a recorded usage-limit reply. [paraphrase] Errors: an unparseable reply is reported as unknown, never as available.
- **R2:** The project instructions name the probe as the step before any bridged brief, and a run's summary carries the `BRIDGE:` line. [user] Errors: none beyond the instruction.
- **R3:** The probe never reaches the workspace: it runs from a scratch directory with no repo access. [inferred] Errors: a probe that ran inside the checkout fails.

## Boundaries
<!-- scope: business -->

- No change to the routing block's tiers or models. [paraphrase]
- No change to the flow-next plugin; the probe is the project's. [inferred]
