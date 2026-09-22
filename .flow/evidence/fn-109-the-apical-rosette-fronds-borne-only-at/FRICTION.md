# Friction — fn-109

## 2026-09-22, implementing the rosette

**The species test binary flaked on the process memory ceiling.** Running
`cargo test --profile ci -p telperion-core --test species` once failed all four
`fixed_*_pass_geometry_and_profile_gates` cases with "peak resident 4986331136
bytes stands over the process ceiling 3972796416 bytes"; the identical rerun,
with nothing changed, passed all fourteen. The ceiling is read off the machine
at the moment the suite starts, so another process holding memory turns the
workspace gate red for a reason no diff caused. Cost: one 50-second rerun here,
and the risk of a worker reading its own change as the culprit. What would
remove it: the budget guard reporting the reading as inconclusive and retrying
once, rather than asserting on a number it does not own.

**"The childless tip of each order-zero axis" reads two ways.** The design
handoff (`.flow/evidence/fn80/design-fn-109.md`) defines the rosette's apex
that way. Read literally as a node with no children at all, it finds no apex on
any branching tree - the birch's leader carries laterals - and the rosette
places nothing, which is how the first implementation behaved. The reading that
works is the last node of the *stem run*: a stem node with no stem child. Cost:
one build-and-test cycle, about four minutes. What would remove it: the handoff
naming the predicate rather than the picture, which is now
`Tree::stem_apices` for the next reader.

**The handoff's authoring-site count was one short.** Ten new `CanopyParams`
rows needed six sites each, not the five the handoff listed: the sixth is a
`blend.rs` walk bucket, which no coverage test catches. The investigation
(`.flow/evidence/fn80/investigation-fn-109.md`) had already caught it, so it
cost nothing here - it is logged because the omission would have shipped a
silent gap had the investigation not run.

**The tuning loop's proposal cap had 358 bytes of headroom, and ten dials do
not fit.** `judgments::proposal_state` lists every score-visible dial in full,
and `twelve_rounds_of_attempts_fold_into_a_digest_that_still_fits` asserts the
result fits `PROPOSAL_CAP` (24,576 bytes). Measured on this branch with the ten
new rows removed: 24,218 bytes. With them: 25,888. At about 160 bytes a dial
the guard admits two more geometry rows before any spec trips it, so the next
capability spec that authors rows pays this cost too, and pays it at the end of
its build after the gate has already run once. Cost here: about twenty minutes
between the first red gate and the measurement that showed it was a cliff
rather than a regression, plus the build's finish line. What would remove it:
the proposal state grouping its dial menu the way the run summary already
groups it, or scoping the menu to the priorities a round is tuning, so the cost
stops scaling with the whole table.
