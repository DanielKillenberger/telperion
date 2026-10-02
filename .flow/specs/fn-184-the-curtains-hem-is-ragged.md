# The curtain's hem is ragged

## Conversation Evidence

> user (2026-10-02, on the birch's hanging curtain while fn-183 removed the crown's wall): "i also thought it could sag more and looks less regular in its shape so that seems like an improvement.. We don't need any of those artifical borders?"
> user (2026-10-02, on keeping the curtain floor so `curtainClearance` works): "the dial should work but atm having the bottom be a straight edge doesn't look good. I'd like that to be shaped more natural. Do we need another spec for this?", then "yes".

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 40% [user], 30% [checked], 30% [inferred] -->

A weeping crown's lower edge is ragged: its strands end at many heights, and only the longest sweep down toward the ground. The generator's curtain ends on a straight line, the owner's verdict on the birch: "having the bottom be a straight edge doesn't look good". This spec gives the curtain a natural hem while `curtainClearance` keeps its meaning, the metres above the ground no hanging twig reaches. [user, paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-10-02 on f1ec68be.** [checked]
- A curtain's floor is per station: `Curtain::new` (`pipeline/branching/local/pendant.rs`) walks it from the height its first descending ancestor's tip ends at toward `bottom` by `sag · hang`, and `bottom` is walked from the crown's base toward `curtainClearance` by the drop row. `bottom` is one height for the whole tree, so as `sag`, `drop` and `hang` rise, every floor converges on it.
- A strand grows `pendulousLength` before it stops (`twigs.rs`, `pendulous_length`), shortened by `Curtain::clear` near the floor; `below` stops terminal twigs under it. fn-183 holds planned curtain strides at the floor by a closed-form cut at its height.

**Unknown until measured (R1).** [unknown]
- Why the hem is straight: how many strands end at the floor because they were cut there, against how many end short of it by their own length, on the birch at seeds 1 and 7 and on the forced test curtain (`suite/drop.rs`).
- Why the drop row barely moves the curtain's lowest tenth once fn-183 removes the band (3.647 to 3.564 m over its whole walk on the birch): the same question as the hem, from the other side.
- Whether existing rows (`pendulousLength`, the curtain's variation row, `sag`, `drop`) already vary strand ends enough when set differently, so no new row is needed.

**Direction (host, 2026-10-02; settled by R2 after R1).** Before adding anything, ask whether the cut can be the rare case: if each strand's planned length ends it short of the floor by its own amount, most strands stop on their own and only the longest reach the clearance, and the hem is ragged with no new border (`docs/principles.md`, "Before work is made faster": remove before adding). A new row, if R1 shows one is needed, is one continuous law, dormant at today's look, refused by name off its rails. [inferred]

## Edge Cases & Constraints

- `curtainClearance` holds: no strand ends below it, at any setting. [user]
- A crown with no curtain (hang 0) is unchanged, and the hem fades in with `hang` from zero (fn-183's continuity fix). [principles]
- Identity is not required; the owner's verdict on the birch decides the look. [AGENTS.md]
- No full-forest capture. [AGENTS.md]

## Acceptance Criteria

- **R1:** For the birch at seeds 1 and 7 and the forced test curtain, a report of strand end heights: how many end at the floor by the cut, how many short of it, and their distribution, today and with each existing row walked across its window; with the workspace suite run on any exploratory build. [inferred]
- **R2:** The host's decision is recorded here before code: the mechanism, and any row with its bounds, neutral value and dial window. [inferred]
- **R3:** On every catalogue preset with a curtain, at seeds 1 and 7, no strand ends below `curtainClearance`, strand end heights spread over a band rather than one height (the band's width stated in R2), and the tree moves continuously as any new row moves. [inferred]
- **R4:** The owner's verdict on the birch at seeds 1 and 7, today beside the new hem, whole and bare. [user]
- **R5:** `cargo test --profile ci --workspace --no-fail-fast` and `npm test` are green; growth time, peak memory and shipped artifact sizes are reported. [AGENTS.md]

## Boundaries

- The crown's wall on twigs is fn-183's and is gone; this spec adds no outline query. [inferred]
- Where twigs are borne is fn-182's. [inferred]

## Decision Context

- **Split from fn-183 (owner, 2026-10-02).** fn-183 keeps the floor exactly as today so the clearance dial works; the straight hem predates it. [user]
- **Depends on fn-183,** which rewrites the same curtain code. [inferred]

## Strategy Alignment

- Serves STRATEGY.md "Our approach": botanical rules below the crossover; measured cost and look. [strategy:Our approach]
