# fn-193 friction

## 2026-10-03, task 1, rendering the structure

- **Doing:** handing the space structure to the pipeline's expansion for the first stills.
- **Hindered by:** the expansion refused it twice. First, `unsolved radii`: phytomers growing in from their draw have zero radius. Second, `surface triangles collapsed in float32 = 6903`: millimetre internodes. Neither error names the node it came from.
- **Cost:** about 10 minutes and two rebuilds.
- **What would remove it:** the refusal naming the first offending node index and its radius or length.

## 2026-10-03, task 1, measuring time and memory

- **Doing:** measuring peak memory of the 80-cycle build.
- **Hindered by:** `/usr/bin/time` is not installed, and the `dcg` guard blocked a loop that redirected to variable paths.
- **Cost:** about 4 minutes. Measured through Python's `getrusage` instead.
- **What would remove it:** nothing in the repository; this is the owner's local setup.

## 2026-10-03, task 1, a stale example binary

- **Doing:** timing `examples/beech` after changing the beech's values.
- **Hindered by:** building `space_beech` (telperion-render) does not rebuild `telperion-space`'s own example. The first timing ran the old values: 171k nodes instead of 605k.
- **Cost:** about 2 minutes; caught because the node counts disagreed with the stills run.
- **What would remove it:** one example for both stills and measures, or rebuilding both in one command.
