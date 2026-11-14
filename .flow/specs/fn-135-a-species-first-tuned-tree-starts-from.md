# A species' first tuned tree starts from its sourced profile

## Goal

The literature filled the profile (sizes, appearance), but nothing turned it into the tree tuning starts from: the palm was tuned from registration values with the default bark, and its sourced grey rough bark and grey-green leaves reached nothing. The conductor must derive starting values from the profile before the first tuning revision, for any species.

Owner quotes (2026-09-24):
> "I thought we have a generalized tuning loop to get to a acceptable looking species based on literature"
> To a spec for the profile-to-preset step before the next tuning pass: "it is tomorrow now so yes"

## Decisions

Host design (2026-09-24):
- A species-agnostic derivation table `data/profile-to-preset.json` maps profile entries to wire paths through code-owned formulas (`midpoint`, `ratio`, `identity`); conditions read only family values, never a species name.
- Appearance ranges keyed by material fields set their midpoints; sizes map height, trunk diameter over height, crown width via `spread = crown_width / (2 * height)` (non-rosette only), frond length to rachis (rosette), leaflet/leaf sizes to the element. Unconfirmed rows are left out and recorded.
- Values clamped to the dial's validated range, written as the tuning config's `initial_overrides` plus a provenance file; a manual override wins; no preset code changes.
- A profile metric the measurer cannot read (outside `derive::MEASURED`) is `contextual`, not `gating`.
- Confirmed from code (2026-09-24): `species_measure`'s metric keys; `envelope.spread` is the widest radius over height.

## Acceptance Criteria

- **R1:** On the palm's profile, colours, roughness, hue and brightness ranges set to midpoints and mapped sizes set, clamped, with provenance — met.
- **R2:** The table names no species; a broadleaf profile skips rosette rows — met.
- **R3:** The conductor writes the derived `initial_overrides` before revision 1 and after a profile change; manual wins — met.
- **R4:** Unreadable metrics are `contextual` — met.
- **R5:** Gate green — not on one run: 1031 passed, 4 failed (core `tests/species.rs` memory ceilings under a concurrent gate; 14/14 alone; no core file touched).

## Result

Landed in `dff2cfbd` (`conductor/derive.rs`, `conductor/overlay.rs`); carried by the fn-149 rewrite (#121). Host-check calls: no `packet/profile.json` derives nothing; trunk radius uses breast-height diameter; frond length maps to the rachis alone.

## Open

- Nothing built the tuning profile from the packet profile; the step only downgraded unreadable metrics.

## Friction

- Core memory-ceiling tests fail under a concurrent gate -> recorded; same flake as fn-109's.
