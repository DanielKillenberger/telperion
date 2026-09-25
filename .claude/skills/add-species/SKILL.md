---
name: add-species
description: Onboard one real species end to end - seed its manifest and tuning config, run the species runner, settle the claims it stops on, hand identity gaps to the host, and hand the owner the tree to look at. Use when asked to "add species A", onboard a taxon, or resume a halted species run. Triggers - "add <species>", "onboard <species>", "resume the <species> run", "what is blocking the <species> run".
---

# Add a species

One instruction, "add species A". This file restates no method: the runbook
is [`docs/species-runner.md`](../../../docs/species-runner.md), and the
literature stages it runs are in
[`docs/species-pipeline.md`](../../../docs/species-pipeline.md).

One species per spec (AGENTS.md, owner 2026-09-16). A generator gap the
species needs is its own spec, never a patch inside this run.

## The loop

1. **Spec.** `node scripts/new-species-spec.mjs --id <species> ...` mints the
   species spec from `templates/species-spec.md`. Refine it, mark it ready,
   and start the run's stack from its branch (`docs/species-onboarding.md`,
   "The stack").
2. **Seed.** Write the seed manifest to `.flow/evidence/<species>/pipeline/manifest.json`
   and the tuning config to `.flow/evidence/<species>/tuning.json`
   (`docs/species-runner.md`, "The tuning config").
3. **Capability assessment.** `packet/capability.json` is the host's, not
   yours: reasoning and system design escalate to the host. Hand it up and
   wait for it before the Capability stage.
4. **Run.** `bash -ic 'target/release/species <species>'` from the repository
   root. Run it again after anything changes; it reruns only what changed.
5. **A stop.** The run prints `STOPPED:` with one of three reasons.
   - **Claims.** Read each named decision in `decisions.json` and the source
     text it cites. A resolution that drops or replaces a value is the
     runbook's; one that keeps a flagged value is the owner's.
   - **Identity gaps.** Stop and hand `runner/gaps.md` to the host. The host
     writes the spec; the species spec depends on it; the run continues once
     it lands on the stack.
   - **The owner's look.** Hand the owner the tree and the checklist. Only
     they run `species <species> --accept`.

## What is never yours

- Jev writes no value and no fix. It answers the questions the tools ask.
- No generator or renderer change inside a species run.
- No visual verdict and no acceptance. The owner looks and accepts.
- No merge. The stack merges once, by the owner, after the checklist.
- No reasoning about the system: the capability assessment, the class of a
  gap and the shape of a spec are the host's. Report and stop; escalation is
  not failure and costs nothing.
- No full-forest capture (AGENTS.md's token and evidence budget).

## Friction

Report friction in `.flow/evidence/<spec>/FRICTION.md` as it happens, one
dated entry. When a path is obviously inefficient, return early with
`NEEDS_HUMAN` and the entry rather than spending the budget on it.
