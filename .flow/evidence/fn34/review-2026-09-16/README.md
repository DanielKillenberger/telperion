# fn-34 path review, 2026-09-16

The owner asked for a review of the plan with Fable and, where it fits, Jev,
"to make sure we're going down the right path" and not "get lost in the
weeds". Fable (`claude-fable-5-1`) reviewed the specs, the report and six
images; Jev checked each of the owner's recorded notes against the planned
work (`jev-note-coverage.json`, from `coverage.py`, which is on disk under `measure/archive/review-2026-09-16/`).

## Findings

- **Done had no finish line.** fn-34 closes only on an accepting verdict,
  so each note minted a spec, and 23 rounds chased centre mean, box fill and
  an ellipse residual that round 23 showed cannot see the lumps the owner
  sees. All five beech reference records are `matching: qualitative`, and
  B-WHOLE and B-BARE are photographs of two different trees.
- **The beech's whole-tree mismatch is shape and tone first.** Primary axes
  run straight until they leave the shell (`branching/scaffold.rs:208`,
  `reach`), every lateral leaves at one pitch with no height term
  (`scaffold.rs:263`), and the result is a symmetric fan with a smooth
  shell-cut top. fn-59's leaf-load bend touches neither.
- **fn-59's unhandled risks.** Leaves on wood bent below the shell meet the
  envelope cull (`mesh.rs:89`); two builds double the identity pins; the
  bend must be applied to a copy so the growth timeline does not grow from
  bent wood; rotated subtrees can make the plaited surface intersect itself.
- **The unnamed gap.** STRATEGY.md measures coverage against the Hallé and
  Oldeman models. The beech grows on Troll's model, with plagiotropic,
  sympodial, zigzag axes. Telperion builds it as a monopodial broom bounded
  by a shell. `crookedness` is a smooth wave about an axis's intent
  (`scaffold.rs:293`), not a zigzag.
- **Jev's coverage.** Every note maps to some planned item, but the chance
  that the plan fully resolves a note is 0.16 to 0.53 for each. Jev maps "the
  whole tree doesn't match the reference at all" to the beech value round
  (0.77), not to fn-59.

## Decisions (owner, 2026-09-16)

- fn-34 closes on the birch: one colour-row round for the bark contrast and
  S-BARK's framing, then the owner's verdict.
- The beech moves to its own spec, judged on a per-view trait checklist at
  three seeds, with the numbers kept as regression readings. At most two
  value rounds per verdict before a generator spec or a stop.
- The beech's order: a bare-only value round, then a small Troll's-model
  generator spec (pitch by height, and limbs that stop short of the shell by
  varying amounts), then the whole tree judged again, then fn-59 only if the
  summer crown still sits high.
- fn-59 and fn-60 no longer gate fn-34. fn-60 runs in parallel. fn-55 and the
  mass grid's 62 ms wait until the beech's structure lands.
- Jev classifies notes as a value row, a generator gap or a tone issue, and
  picks the checklist traits a verdict touches. It no longer runs row-mapping
  trial loops where a note names a missing row.
