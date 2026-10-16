# A prepared tree meets master's memory and frame rate

## Goal

The owner's release bar (fn-237 R0, 2026-10-09) asks F4 to be at least as good as master; the parity scorecard showed memory behind master on every species (palm 1→106 MB, spruce about 400→1,450 MB) and frame rate behind for the ordinary tree and both Two Trees. This spec measured where a prepared tree's bytes and frame time go, cut what was waste, and proved where F4 stands against master. Release item for fn-237.

## Decisions

- Owner (fn-237 R0, 2026-10-09): "I would really like to ship something that is at least as good as the previous version", and "if we get way better fidelity for slightly less performance that's a trade we can live with. But as of now performance is way worse."
- Host (2026-10-10): each view's budget grows from what the view asked, per kind (cut 5).
- Host (2026-10-10): the native allocator hint `MemoryHints::MemoryUsage` is not taken (returned 0.7-1.0 GB of reserved blocks but slowed the spruce's vegetation pass by 24 %; native only).
- Host (2026-10-10): the remaining losses against master are the owner's, weighed in fn-259's qualification; fn-258 closes on code and evidence.

## Acceptance Criteria

- **R1:** per preset, resident GPU/CPU memory split by owner (drawn tree, growth scratch, pools, duplicates) with consumer and lifetime; frame time per pass at whole/mid/close — met (BYTES.md, measured before any cut).
- **R2:** growth scratch retired, pools bounded, duplicates removed, emitted detail chosen by measured error, each cut naming its consumer — met (five cuts).
- **R3:** every preset at or under master's memory and frame time at both ages, any remaining loss named for the owner; stills and motion viewed; Astra agrees; gate green — partial: losses remain and were handed to the owner in fn-259.

## Result

Five cuts, each naming its consumer: (1) growth scratch and the pool released after preparation; (2) one set of surfacing buffers shared by sun and camera; (3) coarser error scales measured only when the finest overruns; (4) the count pass reuses the measure pass's counts; (5) each view's budget grows from what it asked, per kind. Cuts 1-4 change nothing drawn (palm and oak stills byte-identical; spruce and Telperion within run-to-run leaf noise).

F4's process GPU memory at default age, seed 1: 839-4,849 MB before → 583-1,355 MB after. F4 now holds less GPU memory than master for the beech and draws faster frames than master for the five real species and the palm. Remaining losses (RESULT.md table): the ordinary 8-57 % slower at the hero view; Laurelin and Telperion 2.4-46× slower and up to 3.3× the memory; about 100 MB of compiled programs; peak CPU at master's age (oak at 100: 7.7-9.2 GB vs master 0.35 GB); the spruce at 100 and the beech at 120 refused for device memory on a shared 10 GB card. Coarser frames from cut 5: none at the hero pose, up to 9 of 30 zoom frames and 4 of 120 close-turn frames at scale 2, indistinguishable to the eye. Astra: AGREE WITH NOTES. Gate and `npm test` (155/155) green.

## Open

The remaining losses are the owner's to accept or refuse in fn-259; the CPU peak during preparation moved to fn-256 R3a.

## Friction

- Spruce at 100 and beech at 120 ran out of device memory while other sessions held the card -> GPU memory reservation / free-memory check asked in fn-259 R4.
- Same-age sweeps took 25 and 55 minutes (four builds a row) -> one-build probe mode, fn-259 R4.
- ImageMagick 7 `compare -metric AE` is intensity-weighted, understated a difference 100× -> pixel-difference helper, fn-259 R4.
- Fine-executor leaf stills not repeatable run to run -> fn-261.
- Cold-compile test red under load -> fn-237 R9.
- `cargo fmt -p telperion-render` reformatted 18 untouched files -> fmt-clean crate (reported).
- dcg hook on heredoc redirect, `/tmp` quota while linking -> local setup, not specced.

## Evidence

- `.flow/evidence/fn-258-a-prepared-tree-meets-masters-memory/BYTES.md` (R1: owners, consumers, lifetimes, bytes per preset)
- `.flow/evidence/fn-258-a-prepared-tree-meets-masters-memory/RESULT.md` (R2 cuts, R3 tables, remaining losses)
