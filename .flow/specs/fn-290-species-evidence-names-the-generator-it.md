# Species evidence names the generator it was taken on

## Goal & Context

Astra's release review (ASTRA-RELEASE-REVIEW.md, "After 0.2.0" 5): SPECIES-AUDIT.md records the birch at round 14, Telperion at 14 and Laurelin at 8, engine changes invalidating earlier evidence; the review's "what we do stupidly" adds that measurements are published without their scope (fn-274's standalone 8 GiB read as a whole-gate budget), timed while other sessions load the machine (fn-271: load 10-47), and that reviewers rerun heavy tests outside admission (fn-274 FRICTION.md:17).

## Requirements

- **R1:** a comparison input and an owner verdict carry the engine revision, executor, seed and value files they were taken on; a sheet is reused while those still apply and flagged when they do not. Existing runner support is checked first and only the missing check added.
- **R2:** a measurement in evidence states its scope, concurrency and the machine's load; timing comparisons are paired runs in a quiet window; a reviewer reruns a heavy test only through `telperion-heavy` and for a named unresolved question. Where this is a rule in AGENTS.md, it is proposed to the owner, not written by an agent.

## Boundaries

After the release (fn-237). Accepted species are not reopened.
