# Leaf-weighted droop of the outer crown

## Conversation Evidence

> user (2026-09-22): "let's mint them specs but check how things are correlated. Maybe there's an overarching issue that fixes things more elegantly"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 40% [paraphrase], 30% [inferred] -->

The fn-68 round-42 bundle moved all ten pendulous twig rows together; the reviewer graded hanging "slight" better and the owner said the tree "looks better". What remains, in the reviewer's words: "dominant sprays remain ascending or straight, providing insufficient leaf-weight droop". Droop today is a twig property switched on where wood is thinner than `pendulousRadius` times the root radius (`branching/local/pendant.rs:77`), so with an even fork split it fires as one shell; fn-103 changes where it fires. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **Step one is the same measurement as fn-104**, on the same still: the sheet's grade on "hanging outer foliage" for fn-103's tree against the round-42 tree, recorded by code. [host design]
- **If droop gathers on the laterals**, the pendulous rows keep their range and this closes on the evidence. [inferred]
- **If not**, the remaining gap is that droop never bends the branch that carries the leaves, only the twigs. The fix then is leaf load in the bias field for the outer structural orders: a downward term proportional to the foliage a branch carries, one row, default zero, byte-identical. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The measurement is run and recorded (stills, seed, overlay, grade) before any parameter is added.
- **R2:** If a parameter is added: byte-identical default on every shipped preset, range-checked, a dial-table row, and the growth path unchanged at the default.
- **R3:** The owner's eye decides on the whole-crown still.

## Boundaries
<!-- scope: business -->

- Depends on fn-103. Twig hanging rows are not changed. No species branch.

## Strategy Alignment

- Serves "Growth and botanical fidelity". [strategy:Growth and botanical fidelity]
