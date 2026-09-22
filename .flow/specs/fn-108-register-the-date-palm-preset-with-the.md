# Register the date palm preset with the values the generator already reaches

## Conversation Evidence

> gap loop, 2026-09-22, decision `date-palm/gate/onboarding-gate/capability`: route `proceed`, chosen option `register-date-palm-reachable-values` (row 10, table version 1, Jev 0.72 against none 0.16), ledger `caeac8de925958ac060bdd66`.
> user (2026-09-22): "ok go" — the first live run of the gap loop under the conductor (fn-80), on the date palm of fn-82.

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 50% [paraphrase], 30% [inferred] -->

The date palm's run halted at the gate stage on two onboarding gates: the registry gate, "preset date-palm is not registered with a bound profile", and the capability gate, "the generator does not support woody-axes, apical-rosette, pinnate-frond, acanthophyll, persistent-leaf-base, infructescence". The host's capability assessment of 2026-09-19 (`.flow/evidence/date-palm/pipeline/packet/capability.json`) reads the six against the trait space: woody-axes is reachable and missing only because the preset is not registered, the trunk's habit and radius needs are reachable values on existing dials, and the other five are unsupported anatomy. The gap loop, given four options, chose to register the preset with its reachable values first. This spec is that fix, minted by the loop and worked under the ordinary gates, never applied inside the species run. [paraphrase]

What it does not do: it gives the generator no palm anatomy. After it lands the gate reruns and halts again on the five anatomy capabilities alone, which is the loop decomposing the halt. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **Registration is three places, checked 2026-09-22.** `Preset` in `crates/telperion-core/src/presets.rs`: a `DatePalm` variant with id `date-palm` in `from_id` and `id`, and `profile_id` returning `date-palm`, since `species_measure --print-family date-palm` exiting zero is what the registry gate reads. The value table in `crates/telperion-core/src/presets/species.rs`, a block like the beech's. The catalogue list in `crates/telperion-core/src/params.rs` near line 258, which names id, display name and taxon (`Phoenix dactylifera`). [checked]
- **The values are the packet's reachable rows, nothing more:** `skeleton.habit.stems` 1, `skeleton.habit.lateral_orders` 0, `skeleton.habit.apical_dominance` near 1, `radii.length_taper` 0. `laterals_per_station` stays at its floor of 1, which the packet marks an unreachable value and which never fires with `lateral_orders` 0. Every other row keeps the family default. No frond, rosette, organ or material work. [paraphrase]
- **The profile packet** `catalogue/date-palm/packet/profile.json` is copied from the run's `packet/profile.json`, as the beech's catalogue folder holds its own; the manifest the run drafted stays where the run keeps it. [inferred]
- **Capabilities:** `geometry_benchmark --support date-palm` must then report `implemented: true` with the capabilities the value table produces, so the capability gate lists exactly the five anatomy names. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `species_measure --print-family date-palm` exits zero and prints a family whose four rows carry the values above; every other shipped preset is byte-identical in mesh and metrics. Errors: a value outside its validated range is refused by name; a changed row on any other preset is a defect.
- **R2:** `geometry_benchmark --support date-palm` reports `implemented: true`, profile id `date-palm`, and `woody-axes` among its capabilities. Errors: an unregistered profile or a missing capability list fails.
- **R3:** The palm run's gate stage, rerun through `gap resume` at the landed commit, passes the registry gate and halts on the capability gate naming only `apical-rosette`, `pinnate-frond`, `acanthophyll`, `persistent-leaf-base` and `infructescence`. Errors: a gate that still names woody-axes means the registration is incomplete; a gate that passes means a capability was claimed the generator does not have.
- **R4:** The workspace gate is green; the pin tests for the other presets are untouched.

## Boundaries
<!-- scope: business -->

- No palm anatomy: the apical rosette, the pinnate frond, the acanthophyll, the persistent leaf base and the infructescence are the gate's remaining halt and their own specs.
- No species branch in generator or renderer; the preset is a value table.
- Not the species' acceptance: fn-82 owns the palm and the owner's verdict.

## Strategy Alignment

- Serves "The catalogue": one more species as a value table over shared capabilities, with its remaining gaps named exactly. [strategy:The catalogue]
