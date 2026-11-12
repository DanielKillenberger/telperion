# A subtree fades once

Child of fn-351 (P1's old trees; Phase 3). Host-written, 12 Nov 2026. Builds on the phase's top branch (`fn-357-a-retired-root-keeps-only-what-is-read`).

## Problem

Measured (`.flow/evidence/fn-351-020-the-release-candidate/A5-DIAGNOSIS.md`, the Oregon white oak on seed 1001): at 300 the grower holds 4,334 living tips on open wood up to 17.8 m and the settled tree draws every one at scale 0; the drawn tree is a bare 7.8 m post. At 250, 260 and 280, 55%, 62% and 91% of the living tips the grower holds above scale 0.1 are drawn under 0.01; at 120 to 240 on three seeds, 2.4 to 5.6%.

The cause, from runs: the settle's shed fade (`crates/telperion-space/src/shed.rs`, `fades`, `shed_past`). An axis whose subtree has been idle past its delay fades to `alive`, the presence of its most present living apex relative to itself (`contract/presence.rs`, `fade`). `shed_past` multiplies each lateral's and each relay's vigour by its own fade, and `geometry::scale` multiplies a relay's vigour into the base scale it inherits from its parent, which already carries the parent's fade. On a limb line of one lateral and 18 relays whose most present living apex stands at 0.132, every axis of the line fades to 0.132 and the scale runs 0.104, 0.0137, 0.0018, 0.0002, 0 down the relays.

The module's own comment states the intent the product breaks: "A living subtree is never drawn smaller than its living apex".

## The design (host)

A subtree fades once. The fade an axis applies is what its own fade asks beyond what its bearers have already applied:

- Walking parents first, each kept axis has an applied fade `A`: the seed's is 1; a continuation's is its parent's; a lateral's or relay's is the lesser of its parent's `A` and its own fade.
- The factor multiplied into its vigour is `A / A(parent)` (1 where its own fade is no less than what its bearers applied), in the contract's f32 arithmetic and in the canonical order.
- What is shed, and when, does not change: an axis is lost where its own fade is none, as now.

On paper, the line above: the lateral born 14 keeps 0.7888 x 0.132 = 0.104 and each relay after it multiplies by 1, so the line's living wood is drawn at 0.104 of its grown size, not at 0. Where every presence is whole every fade is 1 and nothing changes.

Not checked by the host, to be read by the builder before changing anything: every other reader of `Shed::fade` and of a faded `vigour` (the retirement's `Stand`, `subtree_living`, the grower's own fades in `grow/cycle/`, the closed form's shed bound in `closed_form/kept/`), and whether any of them assumes the product form. If one does, stop and return with the site.

## Acceptance Criteria

- **R1.** The reading above, in the evidence, before any product change.
- **R2.** A test, red before and green after: a hand-built line of a lateral and three relays, whose only living apex stands at a presence of one eighth relative to each, idle past the delay; the last relay's base scale equals the lateral's (one eighth of its unfaded scale), not its fourth power. A second test: where every presence is whole, the fades and the tree are unchanged to the bit.
- **R3.** Continuity kept: the existing fade, shed and by-degree tests pass unchanged, or a test that changes is named with the reason it checked the product form. The paused, year, repeat and let-go equalities hold.
- **R4.** Measured before and after on the oak, seed 1001, at 120, 200, 250, 280, 300, and seeds 1007 and 1012 at 120: living tips the grower holds above scale 0.1 that are drawn under 0.01 (the diagnosis's reader), drawn height and drawn leaf, phytomers drawn, one-thread seconds. Stills, whole and leafless, of seed 1001 at 250 and 300 before and after, on one sheet.
- **R5.** Every digest pin that moves is listed with the tree it pins; none is expected to move where no fade is below one, and that is checked.
- **R6.** `docs/tree-space.md` or the module doc says a subtree fades once.

## Stops

- Any reader that needs the product form; any equality between two ways of growing (paused, retired, year) breaking: return with the site and numbers.
- A tree on the narrow suites drawn larger than its grower's rough layout.

## Boundaries

No change to the fade's law (`contract/presence.rs`, `fade`, `carry`), to what is shed, or to any rule of growth. No new value. Whether an old limb with only small living wood should fade at all is not decided here.

## Rigor

Product code, Opus; narrow tests while building; the gate and review at the phase's landing.
