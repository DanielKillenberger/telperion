---
satisfies: [R3a, R4]
---
# fn-256-the-whole-grower-runs-on-the-gpu.4 Proof: speed, memory, stills, Chrome

## Description
fn-256 R3a and R4:
- **Warm build per preset** at or under F4's (beech 0.8 s, Telperion 1.2 s), including preparation and readback.
- **Cold compile** against fn-254's 11.7 s.
- **CPU peak during preparation** at or under master's (oak at 100: 9.0 GB today).
- **Stills** of every preset, whole, mid and close, beside F4's, viewed.
- **The owner's probe page**, extended to a whole tree and run by the owner (or by the host through the Chrome extension on the owner's word), with no device loss.
- Astra's agreement; codex impl-review; the owner's verdict.

Carried over from task 2 (EXECUTOR.md):
1. **Wall time from small tables.** Grown from the least sizes, with retirement letting go at moves, Telperion 82 takes 2.16 s unstamped (5 restarts, 61 moves), against 1.05 s of stamped build and 0.06 s of settle at its own sizes. Restarts and moves must come out of the build's wall time, through better first sizes or cheaper moves.
2. **Dispatches.** Telperion's build is 765 dispatches a cycle, about 0.44 s at Chrome's 7 µs, against DESIGN.md 7a's target of about 340. The units record every round each cycle (about 424 dispatches); skipping the rounds no job needs would roughly halve that.
3. **Cold compile** passes task 2's bound natively: 0.98 s all at once, 1.67 s by module. The owner's probe confirms it in Chrome.
## Acceptance
fn-256 R3a and R4 met or each miss named with its measurement for the owner.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
