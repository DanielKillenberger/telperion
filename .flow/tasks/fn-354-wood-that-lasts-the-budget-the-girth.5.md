# fn-354.5 A lateral's kind follows its light (R7)

## Description

Build `.flow/evidence/fn-354-wood-that-lasts-the-budget-the-girth/R7-DESIGN.md`, section "The design", as written by the host: the row `shade_short` (the PA a shaded bud becomes, and an exponent; neutral 0), a lateral keeping its kind with probability light^e; the beech's e = 0.38 on SHOOT and BRANCH and its full-light tables.

## Acceptance

- [ ] The rule is in the grower for every species, dormant at 0, and adds no draw where it is 0 (every other species' trees are byte-equal before and after; checked).
- [ ] The beech's rows are the design's, each with its source line in the preset's comment (Lemoine et al. 2002, Table II; the power law named as an estimate in form).
- [ ] A walk over `shade_short` holds its bound; a test shows the long share of a branch's laterals at light 1 and at 0.138 within sampling error of 85% and 40%.
- [ ] Measured on seeds 1001, 1007, 1012, label `fn354-beech`: the beech at 40, Telperion at 82, Laurelin at 80; sheets beside base and task 4's; leaf, leaf area index, trunk, shoots, long share of last year's laterals, cost.
- [ ] Narrow tests, guards, fmt, tsc, catalogue check, vitest green; digests repinned with before and after; docs updated; TASK5-DONE.md written.

## Done summary
R7 built: `shade_short`, a per-PA vector; a lateral keeps its kind with probability light^e and a turn does not rekey; the beech's full-light tables (Lemoine 2002). Beech at 40: new shoots about doubled, trunk 14 to 18 cm (9 to 12 before). The birch's `sapwood` 4.6 as its own commit. Narrow suites 765 of 766: `a_node_keeps_its_identity_across_ages` is red, because the tree still depends on the requested age (fn-351 P4) and a kind that follows light now shows it in the phytomers; owned by the P4 child, which lands before Phase 2 does. See TASK5-DONE.md.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 995134622, efd26ace8, 56fb2d366, 10b13127e
- Tests: narrow space+core suites 765 of 766; a_node_keeps_its_identity_across_ages red, owned by the P4 child
- PRs: