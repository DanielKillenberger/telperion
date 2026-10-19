# Friction: fn-286

## 2026-10-15, the take-over's measures (fork)

- **Doing:** measuring the relays' turns with fn-236's `oak_lines.rs` after adding the row.
- **Slowed by:** the probe measured a relay's turn to the axis's frame heading, which a take-over does not move (it bends the drawn phytomers), so the first value runs showed no change; found by reading the probe, rewritten to the first drawn phytomer and the base re-run.
- **Cost:** about 5 minutes and one 1-minute run.
- **Would remove it:** a probe that states which geometry it reads (frame or drawn) in its output.

## 2026-10-15, today's value files for the stills (fork)

- **Doing:** writing F4's value files to the scratchpad with `git show ... > $S/$n-f4.values` in a loop.
- **Slowed by:** the dcg hook blocks a redirect to a dynamic path; rewrote as three literal-path commands.
- **Cost:** one round trip, about 1 minute.
- **Would remove it:** probes that read a value file at a revision (`<rev>:<path>`) themselves.

## 2026-10-15, the gate (fork)

- **Doing:** the workspace gate after the take-over and the oak's sourced segment.
- **Slowed by:** `telperion-heavy 16` waited on a 13 GB room for minutes before it was stopped and re-asked at 11; and the device's "Given" refusal (oak at 9) read at first like a take-over bug in the device walk, so the device changes were reverted one at a time (five rebuilds) before one probe showed the oak's 0.031 m segment alone triggers it.
- **Cost:** about 40 minutes.
- **Would remove it:** the refusal naming which sized dispatch was given too few (its index and expression), and `telperion-heavy --status`.
