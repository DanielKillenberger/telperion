# fn-192 friction

## 2026-10-03, task 1, comparing two grow-in widths

- **Doing:** running the walk test at two values of `GROW_IN` in one shell loop, with the log path built from the loop variable.
- **Hindered by:** the local `dcg` guard blocks any truncating redirect to a path built from a variable, and any heredoc with process substitution.
- **Cost:** about 2 min and two retries, rewritten with literal paths and the file edit tool.
- **What would remove it:** nothing in the repository. This is the owner's local setup, so it is reported, not specced.

## 2026-10-03, task 1, walk ranges that keep wood above the ground

- **Doing:** walking every setting of the test tree over its full range.
- **Hindered by:** wood below the ground is an error by design, and twig internodes, twig insertion, divergence and limb reiteration each reached the ground somewhere in their range. Each case was found by one 30 s walk run.
- **Cost:** about 6 min over four runs.
- **What would remove it:** each failure naming its setting and value, which the walk helper now does, so a range is fixed in one run.
