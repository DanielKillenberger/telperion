# fn-218 friction

## 2026-10-09: the command guard refused two shell idioms

- **Doing:** task 1, rendering the age-80 sheet and diffing the new value file against the beech's.
- **Hindrance:** dcg refused a redirect into a path built from a shell variable (`redirect-truncate-dynamic-path`, as fn-234 recorded) and a `diff <(...) <(...)` process substitution (`heredoc.posix:process-substitution`).
- **Cost:** about 2 minutes and two re-issued commands.
- **Remedy:** none in the repository; a worker spells scratchpad paths literally and uses the edit tools rather than shell idioms the guard cannot verify.

## 2026-10-09: the device route takes over two minutes for any beech

- **Doing:** R4, the cost of Laurelin's build on the harness's native route (`examples/prepare_timing`, `Generator::prepare` with resident delivery, the path the harness's `setTreeGpu` takes).
- **Hindrance:** every run takes about 140 s whatever the tree: Laurelin at 80, seed 42, 139.5 s; the shipped beech at 80, 144.5 s; the shipped beech at 40 (240k leaves), 136.9 s. Peak memory is 5.45 GB in all three, and the stage metrics print 0. So a fixed cost of the route swamps the tree's own, and Laurelin's marginal cost cannot be read from it. fn-234's FRICTION.md saw the same on `generation_gpu` (the beech at 219 s).
- **Cost:** about 8 minutes of GPU-lock time for three numbers that measure the route, not the tree.
- **Remedy:** a host check of where the device prepare spends its time (a first-run pipeline compile, a fixed allocation, or a real cost) and stage metrics that are filled on the resident route. Not this spec's to fix; reported for the host.
