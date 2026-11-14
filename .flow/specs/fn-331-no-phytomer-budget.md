# No phytomer budget

## Conversation Evidence

> owner: "remove the shoot limit everywhere.. We don't want artificial limits like that i've said that before"

## Goal & Context

Child of goal fn-321, serving R7. The generator refuses any tree past 20,000,000 phytomers grown, the shed ones included, on the CPU and on the device. fn-330 measured that the full cycle grows the oak to age 200 in 104 s and 9.1 GB once the budget is raised, so the budget, not the machine, stopped it. The owner rules out such caps. [paraphrase]

## Acceptance Criteria

- **R1 (no budget):** no entry, request or executor carries a phytomer budget or refuses a tree for the number of phytomers it grew; the refusal and its error are gone. [paraphrase]
- **R2 (real limits stay named):** where a platform truly lacks memory (a device buffer past its size, 32-bit Wasm's memory), the tree is still refused by name with what it needed, as today; no such refusal is added or loosened by this child. [inferred]
- **R3 (the old tree builds):** an ignored measurement builds the oak at age 200, seed 1, through the pipeline on the CPU, and records its time and peak memory. [inferred]
- **R4 (verification):** the gate is green and the codex review of the child's diff ends SHIP. [inferred]

## Boundaries

- No threshold of the goal lowered. Nothing merges to master, nothing is tagged or published. [paraphrase]
- Tests that asserted the budget's refusal are removed with it; tests that used a small budget only to bound their work keep what they checked. [inferred]

## Decision Context

`AGENTS.md`, "No artificial limits" (owner, 3 Nov 2026). [paraphrase]

## Open Questions

None.
