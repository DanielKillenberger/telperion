# Bundle-search run on the beech, 2026-09-21

Production `tuning-loop` at `c508867e` (CI-profile gate EXIT=0, 796 tests). Fresh bootstrap run, 109 score-visible dials, `selection: bundle`, strengths 0.5/1/2/4 small steps, one uncalibrated contact-sheet review per round (requested `gpt-6-astra` medium through `scripts/contact-sheet-codex.py`, prompt redacted of paths and trial keys). Owner confirmed the same three priorities ("ok go"). Raw state in ignored scratch `.flow/tmp/fn68-canonical/bundle/`. Eight rounds ran without host intervention.

| Round | Bundle | Strengths that built | Reviewer, crown / hanging | Outcome |
| --- | --- | --- | --- | --- |
| 13 | 33 dials incl. hang, rise_secondary, sag, max_droop, pendulous_length, curtain_drop, lateral_pitch, lateral_orders, limb_clumping | 0.5 and 1 (2 failed a numeric gate; 4 collapsed surface triangles) | 0.5: clear / slight, nothing broken. 1: clear / clear but "long exposed limbs cross the crown… stripped and skeletal" | **0.5 adopted** |
| 14 | 32 dials, same family of directions | 0.5 and 1 | 0.5: clear / slight. 1: breaks, "sparse… antler-like branches" | **0.5 adopted** |
| 15 | 26 dials | all four | 0.5: clear / slight. 1, 2, 4: worse, crown collapsing to "a bare tapered pole" | **0.5 adopted** |
| 16–20 | 23–29 dials | almost every strength fails a numeric gate; the two that built were judged worse ("small terminal tuft above an excessively long bare trunk") | — | five bundle stalls |
| 21 | — | every strength already tried from this tree | — | paused: "bundle already tried; no new direction" |

Three consecutive adoptions, each graded clear on crown shape and slight on hanging foliage by the reviewer. After them `hang` is 0.75, `riseSecondary` −0.375, `pendulousLength` 1.75, `maxDroop` 2.6, `sag` 0.225, `lateralPitch` 58, `lateralOrders` 5, `shoulder` 1.05: the coupled hanging rows moved together, which no single-dial round could do. The telemetry score of the current tree is 0.1623 against the baseline's 0.2432, so on this tree the numbers and the reviewer agree.

Then the search hit a wall: from the third adopted tree nearly every further bundle fails the species' numeric gates before it can be rendered, and the few that build lose the crown. The closing all-view review still fails every cell. Unknown: which gate fails (the run records only "numeric gate failed"), and whether the owner's eye agrees with three "clear" grades.

Reviewer spend for this run: 9 calls (one all-view review after approval, three closing reviews, five contact sheets). Visual passes 54 of 90, evaluations 63 of 80, images 322 of 420, rounds 21 of 22.

## Owner's eye on the stills, 2026-09-21

Shown the reference, the start and the three adopted trees. Verbatim: "to me the structure looks messy. Branches too thick on the outer rim, too much chaotic branching. Bundle 3 i wont' even judge that's just garbage. Bundle 1 seems like an improvement and then it went off the rails i think."

So: bundle 1 is an improvement to the owner; bundles 2 and 3, each graded "clear" on crown shape by the contact sheet, made the tree worse, and the third is unacceptable. The one-view, two-priority sheet adopted steps whose side effects (thick outer branches, chaotic branching, the leader) the owner rejects and the all-view review also reported. The sheet's grades cannot be trusted past the first step as built. New owner observations, not yet priorities: branches too thick at the outer rim; too much chaotic branching.

Owner, a few minutes later, verbatim: "one big difference is also just the whole crown needs to be stretched vertically it's longer and taller in the rference". The telemetry numbers agree: the whole view's width over height is 0.7504 on the baseline against the photograph's 0.6965. No reviewer finding in any of today's packets named crown proportion, and bundle 1 moved `spread` up (0.36 to 0.41), the wrong way for this observation.
