# The heavy tests fit beside each other, and the gate reaches ten minutes

## Goal & Context

Owner (2026-10-21) asked for the gate to be fixed now: "test suite takes 35 min?!". fn-304 cut the gate to about 1,124–1,241 s, from about 2,494 s, under nextest with groups at 4 threads inside `telperion-heavy 8`. That missed its ten-minute R5. The arithmetic is measured (fn-304 RESULT.md, host, 2026-10-22):

- Past 4 threads, more threads buy nothing. Tests over 1.5 GiB take one turn between them.
- At 4 threads, the ten tests over 3 GiB sum to 661 s, and the twenty between 1.5 and 3 GiB sum to 421 s. That is about 1,080 s serialised by memory alone.
- One test sets the floor: the merged walks test, at 223–230 s whatever the width.
- Letting tests share the 8 GB by measured size (one token per GiB, 7 per run) reached 891–1,072 s. But nextest left 57–239 device tests unscheduled without saying so, so no such run passed.

## Unknown, check before the plan

- Why a whole-preset test holds 1.5 to 7.7 GiB. Is it the tree itself, kept artifacts, the device's arenas, or intermediate buffers kept alive past their use?
- Why nextest skipped tests under the token scheme. Is it a configuration error, a `threads-required` above the run's width, or a nextest defect? Reproduce it on a two-test fixture.

## Requirements

- **R1 (measure):** the peak memory of each test over 1.5 GiB, split by what holds it.
- **R2 (question, delete, then optimise):** for each holder, try dropping it first, then try shrinking it. No assertion is weakened. No generator or renderer change, unless a test-observed buffer is kept alive past its use in production code too. That change is its own measured fix.
- **R3 (schedule):** tests share the 8 GB by measured size, with every test scheduled. The run fails if any test was not run.
- **R4:** the gate under the declared invocation runs in under ten minutes, with fn-304's budgets re-set from the new figures. AGENTS.md's gate line is re-measured. The gate is green, `npm test` is green, and each task gets a codex review.

## Boundaries

- Not a re-litigation of fn-304's subset rulings.
- Post-release unless R1 finds a production buffer leak.
