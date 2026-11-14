# Register the date palm preset with the values the generator already reaches

## Goal

The date palm's run (fn-82) halted at the gate on two onboarding gates: the preset was not registered with a bound profile, and the generator lacked woody-axes, apical-rosette, pinnate-frond, acanthophyll, persistent-leaf-base and infructescence. The host's capability assessment (2026-09-19) read woody-axes as reachable and missing only because the preset was unregistered; the gap loop (decision `date-palm/gate/onboarding-gate/capability`, route `proceed`, option `register-date-palm-reachable-values`, Jev 0.72 vs none 0.16) chose to register the preset first. Owner (2026-09-22), starting the first live gap-loop run under the conductor (fn-80): "ok go".

## Decisions

- Register in three places: `Preset::DatePalm` (`presets.rs`, profile id `date-palm`), a value table in `presets/species.rs`, the catalogue list in `params.rs` (`Phoenix dactylifera`), held in `params::IN_WORK` (abi id 7, unlisted).
- Only the packet's reachable rows: `stems` 1, `lateral_orders` 0, `apical_dominance` 1.0, `length_taper` 0.0; everything else family default. No palm anatomy — the five remaining capabilities are their own specs.

## Acceptance Criteria

- **R1:** `species_measure --print-family date-palm` exits 0 with the four values; other presets byte-identical — met.
- **R2:** `geometry_benchmark --support date-palm` reports `implemented: true` with `woody-axes` — met.
- **R3:** The palm gate rerun passes the registry gate and halts on the five anatomy capabilities only — left to the host after landing.
- **R4:** Workspace gate green, other pins untouched — met (118 suites).

## Result

Landed in `03644999`. The palm's gate now halts on anatomy alone (rosette and frond: fn-109; trunk organs: fn-110; infructescence: fn-111).

## Open

None.
