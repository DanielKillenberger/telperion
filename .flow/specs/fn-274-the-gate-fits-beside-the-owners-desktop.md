# The gate fits beside the owner's desktop

## Goal & Context

fn-251 and fn-270 (2026-10-14, FRICTION.md): the workspace gate is killed for memory below a 13 GB cap or above 2 test threads, in `telperion-core`'s unit tests. `pipeline::space::repeat_tests::every_preset_grows_the_same_tree_again_and_on_any_cores` holds a whole preset while it grows the same preset again (Telperion's live structure is several GB), and the `suite::` whole-preset tests run beside it. The desk has about 13-15 GB available with the owner's desktop open, so a gate occupies the whole machine and admits nothing else (the owner, 2026-10-13: "doesn't completely block the machine but if there is capacity you can run multiple things"). Test-only, an obvious fix (AGENTS.md, friction).

## Requirements

- R1: the repeat test compares builds by a digest of the tree in build order, taken and the tree dropped before the next build, so it holds one tree at a time; the comparison stays to the bit in build order (not canonical order), and a deliberately perturbed build still fails it.
- R2: each whole-preset test in `telperion-core`'s unit tests measured for its peak; those over 2 GB hold one tree at a time the same way, or share one build per preset where they test the same entry.
- R3: the gate's peak at 4 test threads measured before and after; target under 8 GB, every test count unchanged.

## Boundaries

Test-only. Release housekeeping (fn-237).
