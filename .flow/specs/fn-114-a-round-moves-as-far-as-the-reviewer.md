# A round moves as far as the reviewer says the tree is off

## Conversation Evidence

> host, fn-80 `GAPS.md` proposal 2, 2026-09-22: "Tuning loop: strides scaled by the reviewer's magnitude words, or a magnitude the owner's priority carries."
> user (2026-09-23): "i approve 1 & 2"

## Goal & Context
<!-- scope: business -->

In the date palm's first tuning revision the reviewer said every round that the fronds were two to three times too short, and the frond length moved 3.5 to 3.75 m over seven rounds. A round's move is a fixed ladder of multiples of each dial's small step, whatever the reviewer's words say, so a tree that is far off takes more rounds, renders and paid looks than the budget allows to get there. [paraphrase]

This spec lets the size of the gap the reviewer or the owner names choose how far a round moves, so a far-off tree crosses the distance in a few rounds and a near one is still refined in small steps. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23 on the fn-80 branch.** A bundle is drawn at each of `bundle_strengths` (default `[0.5, 1, 2, 4]`, at most four, `tuning/live.rs:236`); a dial moves `strength × small` (`tuning/bundle.rs:84`). The palm's run set `[0.25, 0.5, 1, 2]`, so `rachis_length` (`small` 0.1) moved at most 0.2 m a round. Jev's direction question selects a direction and never a magnitude (`tuning/actions.rs:75`); the separate adjustment question selects among small and substantial options code built. A config reaching magnitude is refused without scoped experimental authority: "magnitude live efficacy unvalidated" (`tuning/command.rs:457`). [checked]
- **Shape, bound by the repo's Jev rule.** Jev selects; it never supplies a number. Code offers a magnitude class set for each owner priority or reviewer finding (for example near, clearly off, far off, with a no-match answer), Jev selects one from the words, and code maps the class to a strength ladder it owns. An owner priority may carry the class directly and then no Jev call is made. The ladder's top is bounded by the dial's range and by the existing overshoot and rollback rules. [inferred]
- **The design (host, 2026-09-23).** [host design]
  - *One question per track per round, before the draw.* Code asks Jev a choice over the track's owner priority words and the latest reviewer finding on that priority: `near`, `clearly_off`, `far_off`, or `no_match`. Code maps the class to a multiplier on the configured `bundle_strengths` (near 1, clearly off 2, far off 4); `no_match`, a confidence below the calibrated threshold, or no finding yet keeps the ladder as configured (multiplier 1). The clamp to the dial's range still applies.
  - *The owner's word wins and costs nothing.* An owner priority may carry `magnitude: near|clearly_off|far_off` in the tuning config; then no Jev call is made for that track.
  - *Overshoot steps down.* When a round's bundle at a raised class is rolled back as worse, or a reviewer finding on that priority flips direction, the track's class is capped one level lower for the following rounds until a bundle on it is adopted; the cap is recorded. This is the R4 guard against oscillation.
  - *Record.* Each round's route note names the class, its source (`owner`, `jev` with its confidence, or `default` with the reason) and the multiplier used.
  - *Calibration home.* A new calibration kind `gap_magnitude` beside `magnitude`, `direction` and `continuation` in `tuning/calibration.rs` (`questions`), with labelled cases in `crates/telperion-jev/data` written before the question is trusted: the palm's fn-80 reviewer and owner sentences ("far too thin and short", "two to three times longer") among them, plus near-miss and no-match cases. The threshold comes from that set, per `docs/typesafe.md`.
  - *Authority.* Until the `gap_magnitude` set passes, a Jev-chosen class above `near` is refused without the scoped experimental authority the existing magnitude rule requires (`tuning/command.rs:457`); an owner-carried class needs none.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A finding or owner priority that names a large gap draws the round's bundle at a larger stride than one that names a small gap, from a class code offered and Jev or the owner selected; the number is code's. Errors: when no class matches, the stride falls back to today's ladder and the record says so. [inferred]
- **R2:** Class coverage is tested against labelled cases before a selection is trusted, as `docs/typesafe.md` requires; the palm's "far too short" reviewer words are among them. [paraphrase]
- **R3:** A replay of the palm's fn-80 rounds, with no new render and no paid look, shows the frond-length dial reaching its target distance in fewer than seven rounds, or states why it cannot. [inferred]
- **R4:** A too-large stride is caught by the existing overshoot and rollback rules; a test shows an overshoot steps back rather than oscillating. [inferred]
- **R5:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- No change to dial ranges; that is the range spec approved beside this one, and without it a wide stride on a capped row still stops at the cap. [paraphrase]
- No live tuning run or capture here; fn-80's next revision exercises it. [inferred]
- Jev never produces the stride's number. [paraphrase]

## Decision Context

- The owner approved this from fn-80's gap list on 2026-09-23, with the range proposal. [user]

## Open Questions

- None. The calibration home and the class design were settled by the host on 2026-09-23 (see Architecture).
