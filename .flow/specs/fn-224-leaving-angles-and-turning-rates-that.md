# Leaving angles and turning rates that vary axis to axis

## Goal

The silver birch (fn-213) had Astra's "comb" named on sheets 8 and 9: strands with "repeated hooked starts and straight, parallel drops", in ranks. Cause, checked in code: every lateral of a physiological age left its bearer at one constant `insertion` and turned at one constant `form.tropism`; only roll was drawn per axis. Host decision (2026-10-07): a capability, the sibling of fn-222's internode spread, not more values. The engine's layout only; the birch's values are fn-213's.

## Decisions

- Both spreads uniform (bounded, mean-preserving, closed-form moments), as fn-222's; a correlated or log-normal draw is a later design choice.
- The insertion bound is pi/2 (worker's choice for "bounded"; a negative or past-pi angle turns continuously, so nothing jumps). A tighter bound is a value change in `validate.rs`.
- The tropism draw is per axis, continuations included, so the rate can step at a PA boundary as the PA's own tropism already can.

## Acceptance Criteria

- **R1:** `insertion_spread` and `tropism_spread` per PA, drawn from new lineage keys, mean-preserving, bounded, zero with no draw, walk-tested within 30 — met.
- **R2:** Both executors draw them identically under the integer contract, probe extended, GPU lay applies them — met.
- **R3:** The closed form correct in expectation (or error stated); predictor counts unchanged — met, exact in expectation.
- **R4:** Every passed species byte-identical; npm test and the gate green — met.

## Result

- Insertion = insertion + s(2u - 1) from `Key(axis.lineage).child(INSERTION)`; tropism x (1 + s(2u - 1)) from `TROPISM`; at 0 bit-identical with no draw. GPU PA table 24 -> 25 slots.
- R1: beech and oak at 20 keep counts, axes, keys and scales; every lateral's angle matches its draw within 1e-15; offset sd 0.571-0.575 (uniform 0.577).
- R2: probe selector 17 over 262,144 keys bit-equal; beech 25 tips within 6.1e-6 m (screen 50 um); disabling either GPU term puts tips 0.86 m / 0.32 m off.
- R3: the closed form reads E cos = cos a sin s / s and E cos^2; over 400 seeds within 1.2 standard errors (constant reading 10.5-21 se off); predictor counts, log lengths and reach bounds bit-identical at both bounds.
- R4: `space_hash` 16/16 identical to base `5869f7e1`; gate green; npm test 143 passed. Catalogue digests moved only by Debug fields.

## Open

None.

## Friction

- Base hashes raced the first edits (built a half-edited tree) -> proposed `space_hash` taking a git revision or `--base`.
- The shell guard blocks heredoc edits and variable-path redirects -> proposed an AGENTS.md note: edit scripts as scratch files, literal redirect paths.
- The `walks` binary is 431 s of the space suite's 8 minutes -> proposed a `WALK=<setting>` filter.
