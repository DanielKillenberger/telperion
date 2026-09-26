# fn-61 friction

## 2026-09-27, worker: neutrality digest

- Doing: capturing the base generation digest (8 tables x seeds 1 and 7) before the edit, to prove the two rows neutral to the byte.
- Hindered: the dcg hook refused the first run because its output redirect used a shell variable for the scratchpad path; the command had to be rewritten with literal paths.
- Cost: about 1 minute and one extra tool call.
- Remedy: a `--out <file>` flag on `generation_digest` would avoid shell redirects entirely.

## 2026-09-27, worker: the digest is a GPU run of about 3 minutes

- Doing: the byte-neutrality proof R1 asks for ("every shipped table at two seeds and in the specimen views").
- Hindered: no in-tree test states that claim; `generation_digest` is the only tool that covers every table, both seeds and the growth view, and it needs the GPU and about 3 minutes per run, twice (base and change).
- Cost: about 7 minutes of wall time.
- Remedy: a CPU-only digest mode (pipeline, mesh and growth digests without the two GPU deliveries) for scaffold-only changes.
