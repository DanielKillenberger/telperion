# The apical rosette: fronds borne only at the apex of an unbranched stem

## Goal

The date palm's gate halted on five capabilities; two are its crown: `apical-rosette` (fronds only as a rosette at the apex of a thick unbranched stem, no laterals, no twig clothing) and `pinnate-frond` (one frond as leaflets along a rachis in one placement). The canopy layer bears foliage on wood below a radius threshold and the element layer draws one blade, so nothing placed a whorl of large compound leaves at one point. This spec adds that crown, off by default, as fn-82's dependency — the first gap-loop spec that changes the generator (gap loop round two, winner `apical-rosette-of-pinnate-fronds`, Jev 0.42 vs none 0.34, route owner).

Owner (2026-09-22): "ok it's really hard for me to judge if the palm needs a apical rosette but if confidence is high let's go for it?" — taken as confirmation.

## Decisions

Host decisions on the strong-tier design handoff (2026-09-22):
- A frond is one placement expanded into leaflets, never a compound element (fn-33's grouping-at-the-placement decision; element stays single, renderer unchanged).
- The pinnate rows live on `canopy`, authored once here; fn-33 adopts them for its leaflet half.
- `pinnate-compound` moves into the expressed list with the two new names; the ash's gate (fn-56) then halts on its own values, a value round, not a gap.
- An apex that bears a rosette bears no twig.
- Left open (not the host's): what `species_measure` reports for a frond (fn-82), and whether a leaning palm's rosette follows its axis (default) or the sky.
- Blocker resolution (host, 2026-09-22): the tuning loop's proposal-state cap (24 KiB, 358 bytes of headroom, set without a measured limit) was raised to 32 KiB because fn-68's ledger shows Jev answering 34,000-token batches; scoping the proposal menu to a round's priorities is the structural fix, proposed in fn-80's friction.

## Acceptance Criteria

- **R1:** The palm gate names `apical-rosette` and `pinnate-frond` as expressed and halts on the other three — vocabulary and derivation met (`tests/capability.rs`, `stages_downstream.rs`); gate rerun the host's after landing.
- **R2:** Every shipped preset byte-identical with the rosette absent — met (species digests and catalogue pins unchanged, with a positive control).
- **R3:** The palm at seed 1 builds a rosette at the apex, no foliage or twig wood below it — met (eight tests in `tests/rosette.rs`, one still); the owner's look belongs to fn-82.
- **R4:** New rows in the dial table; gate green — met after the cap raise (894 passed).

## Result

`foliage/rosette.rs` is a third placement source at every stem apex (`Tree::stem_apices`, the stem run's last node) bearing N fronds on a phyllotactic spiral; `fan` is the one expansion of a placement into leaflets, shared by the station walk, short shoots and the rosette; `terminal_leaflet` blends the last leaflet's pitch. Ten canopy rows with ranges, wire lines, blend buckets and dial rows; `clear_apical_twigs` and `mesh::grow` keep rosette apices twig-free. Two corrections to the handoff: the apex is the stem run's last node, not a childless tip; the terminal leaflet is a blend, not an extra instance. Commits `90b61031`, `9c08aa55`, `31f1f763`.

## Open

None here; leaflet size and trunk habit are fn-82's tuning.

## Friction

- The species test flaked on a process memory ceiling read off the machine at suite start (one rerun) -> the budget guard should report inconclusive and retry once.
- "Childless tip of each order-zero axis" read two ways (~4 min) -> handoffs name the predicate (`Tree::stem_apices`).
- Handoff listed five authoring sites per row, not six (the `blend.rs` bucket); caught by the investigation -> logged only.
- Proposal cap had 358 bytes of headroom (~20 min) -> raised to 32 KiB; grouping/scoping the menu proposed.
