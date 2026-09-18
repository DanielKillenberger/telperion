---
name: add-species
description: Onboard one real species end to end - draft its manifest, run the species pipeline, handle every gap through the gap loop, run species QA, and hand the owner a checklist with the run's three numbers. Use when asked to "add species A", onboard a taxon, or resume a halted species run. Triggers - "add <species>", "onboard <species>", "resume the <species> run", "what is blocking the <species> run".
---

# Add a species

One instruction, "add species A", to a species the generator draws and the
owner has ticked. This file is the conductor; it restates no method. The
method lives in [`docs/species-onboarding.md`](../../../docs/species-onboarding.md)
and the runbook in [`docs/species-pipeline.md`](../../../docs/species-pipeline.md),
whose **The gap loop** section is the one every halt below goes through.

One species per spec (CLAUDE.md, owner 2026-09-16). A generator gap the
species needs is its own spec that the species spec depends on, never a patch
inside this run.

## The loop

`DIR` is `.flow/evidence/<species>/pipeline`. Build once, then walk the
runbook's stages in order. Every command is a `species-pipeline` command from
the repository root, with the key available to an interactive shell.

1. **Spec.** `node scripts/new-species-spec.mjs --id <species> ...` mints the
   species spec from `templates/species-spec.md`. Refine it, mark it ready.
2. **Manifest.** Run `discover`, read its proposal, draft `DIR/manifest.json`,
   and ask the owner to admit it. Nothing after discover runs until they do.
3. **Stages.** Run the runbook's stages in order. A stage that prints
   `current` did nothing and is right to skip.
4. **A stage that stops.** Read the decision it names.
   - An `onboarding-gate` or a `level-miss` is a **gap**: go to *At a gap*.
   - Any other kind is a decision inside the run: resolve it the runbook's
     way, or hand it to the owner when it is theirs.
5. **Verdicts.** A verdict that is not accepting takes at most two value
   rounds (`gap round`), then names a gap or goes to the owner. The third
   round is refused by the tool, not by judgment.
6. **QA and handoff.** Run the species QA pass, write `metrics.json`
   (`gap metrics`) beside the report, and hand the owner the checklist. The
   stills are theirs; no route here decides a visual verdict.

## At a gap

```sh
P=target/release/species-pipeline
$P gap open    --dir DIR --decision <halt id>
$P gap options --dir DIR --decision <halt id> --author agent --model <you> --options options.json
$P gap route   --dir DIR --decision <halt id> --verdicts verdicts.json
```

Write two to four candidate fixes in the fixed shape (the runbook's **The gap
loop** section carries the fields and the option file's shape). Then read the
route the table gave:

| Route | What you do |
|---|---|
| `proceed` | Mint the fix as its own spec, record it (`gap spec`), work it under the repo's review, record each verdict (`gap review`), and on landing run `gap resume --commit <sha>`. Then rerun the stages it names. |
| `stronger` | Hand the same gap to the stronger reasoning model named in CLAUDE.md's routing block. It writes the set; record it with `--author stronger`, and route again. |
| `owner` | Stop. The decision the route filed is the owner's; say what it is and wait. Never resolve it yourself. |

An empty set from you goes to the stronger model; an empty set from both is
the owner's. A gap spec that comes back needs-work twice is the owner's. A fix
that moves a pin lands only with a `--pin-note` under fn-53's rule.

## What is never yours

- Jev writes no option, no fix and no number. It answers the questions the
  tool asks over options you wrote.
- No generator or renderer change inside a species run. Every fix is its own
  reviewed spec, and the run resumes only once it lands.
- No visual verdict. The owner ticks the checklist.
- No threshold edited to reach a route you wanted. `data/gap-routes.json` is
  the owner's dial; a route you disagree with is reported, not routed around.
- No full-forest capture, and no capture at all before the defect reproduces
  small (CLAUDE.md's token and evidence budget).

## Friction

Report friction in `.flow/evidence/<spec>/FRICTION.md` as it happens, one
dated entry. When a path is obviously inefficient, return early with
`NEEDS_HUMAN` and the entry rather than spending the budget on it.
