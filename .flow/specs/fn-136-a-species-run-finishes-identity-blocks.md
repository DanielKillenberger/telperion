# A species run finishes: identity blocks, improvements are captured

## Goal

A species run needs a finish line: literature passes, the generator expresses every capability the species needs to be recognisable, tuning has converged on what the generator can draw, and the owner ticks the checklist. A missing capability that only adds realism is captured as a backlog spec and listed as a known gap; it never blocks the run.

Owner quotes (2026-09-24):
> "i think we need to have a end state where if the generator is missing a feature/attribute that applies to many tree species even ones added already to get this tree to be "realistic" then we capture it but don't expect it to be done for this run. Otherwise we just run towards perfect trees on every single run and never end? Or how do you see the finish condition?"
> The host classifies at capability assessment ("Host at capability assessment (Recommended)"); the palm's date clusters are backlog ("Backlog (Recommended)").

## Decisions

- Each missing capability in the assessment carries `class: identity | improvement`, a reason, and for an improvement its backlog spec. The classification is the host's, reversible by the owner; no Jev question decides it.
- The gate blocks only on a missing identity (or unclassified) capability; an improvement is a known gap. Traits tied to an improvement are `unexpressed` (fn-116) and left out of readiness and core coverage.
- A converged revision (every drawable objective passing, or fn-117's no-progress guard) goes to the owner's packet, whose checklist lists the known gaps.
- The palm: `infructescence` is an improvement captured by fn-33 and fn-111.
- Host, during the build (2026-09-24): reviewer findings and defects carry a typed `trait_id`; the comparison request names known gaps; code drops blockers tied to them from `ready()` (no model text parsed); the comparison protocol becomes `reference-first-comparison-v2`, invalidating the frozen reviewer qualification. A converged bootstrap revision reaches the packet with machine readiness `unqualified reviewer` and qualification listed as a known gap; the owner's verdict is the acceptance.

## Acceptance Criteria

- **R1:** A missing improvement passes the gate as a known gap; a missing identity blocks — met.
- **R2:** Traits tied to an improvement leave readiness and core coverage; the palm's `fruit-clusters-pendent` no longer blocks — met.
- **R3:** A converged run leads to the packet with every known gap listed — met.
- **R4:** The palm's assessment records `infructescence` as improvement (fn-33, fn-111) — met in the fixture; the fn-80 record is the host's.
- **R5:** Gate green — met (1038 passed).
- **R6:** On the palm's recorded state with a fixture `trait_id: fruit-clusters-pendent`, the date cluster no longer blocks (the trunk still does) — met.
- **R7:** The palm's bootstrap revision-3 shape reaches the packet as `unqualified reviewer` — met.

## Result

Landed in `88c77fe5`, `a3a0d97e` (`pipeline/stages/capability_class.rs`, `conductor/finish.rs`); carried by the fn-149 rewrite (#121).

## Open

At close, requalification the next live run had to pay: mirror the v2 schema into the Claude reviewer adapter, rebuild replay cases as v2, rerun Stage B replay (one paid comparison per case).
