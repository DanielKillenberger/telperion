# fn-245-a-breakthrough-in-generation-time.2 E1/E2: scale-split oracle and the scaffold's floor (CPU)

## Description
COUNCIL.md round 6, E1 and E2. E1 in FIELD's replay harness: oak 80 seed 1, scaffold = axes whose expected life passes N years (N = 3, 5, 8); sub-N buds stand in the cycle in two forms, (a) the closed form's expectation (Fable), (b) a compact state with a vigour/survival spread and internal shedding (Astra); after the cycle those buds regrow alone against the per-cycle field recorded at their tips. Report kept, axes shared, tips p50/p90, leaf area, shed Jaccard, and stills whole/mid/close beside exact for the big limb, mid branch and whole tree. Stop: whole-tree kept outside 5% or stills told apart at whole or mid. E2: time the CPU cycle with sub-N laterals skipped on Telperion (lit, fn-264's rows) and the lit oak: the scaffold floor, and the share of time the twig layer takes.

## Acceptance
SPLIT.md with E1's tables and viewed stills per N and per form, and E2's measured floors.

## Done summary
E1 and E2 ran on fn-245-field-spike with fn-264 merged at fc0257862 (merge 01beb13ce); the report is SPLIT.md on that branch (commit 923a948a9). The split holds in its regrowth half: the oak at 80, seed 1, keeps within +1.2 % (N = 3) and +1.5 % (N = 8) when its sub-N subtrees regrow alone against the exact 64³ field, and the viewed stills cannot be told apart. It breaks at the closed-form stand-in (form a): the tree keeps +17 % / +14 %, the scaffold sheds a different set of laterals, and the stills are told apart at whole and mid, so the stop condition fires. The scaffold's floor (E2): the oak's cycle takes 1.09 s at N = 3 and 0.20 s at N = 8, against 6.58 s; lit Telperion's takes 1.45 s at N = 5, against 8.64 s. The twig layer is therefore 83-97 % of the cycle.

NEEDS_HUMAN:
- Form (b)'s compact state is unspecified and was not built.
- N was read against the tables' life bound (`shed::lasting`), so N = 3 equals N = 5 for the oak, and Telperion has no sub-3 PA.
- The closed-form stand-in costs more CPU than the twigs at N = 3 (25.6 s against 8.2 s).

stage: impl-review - skipped(policy: risk - evidence-only spike; no production code changed, the patch is reversed and kept as evidence)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 01beb13ce, 923a948a9
- Tests: split e1 oregon-white-oak 80 1 <out> 0|3|8, split e2 oregon-white-oak 80 1 3 8, split e2 telperion 82 42 5 8
- PRs: