# fn-208 friction

## 2026-10-05, design research: timing today's wood path

- **Doing:** measuring the wood pass's GPU time for the 80-year engine spruce and oak, and today's presets, at the hero view, the limb close-up and a 5 cm twig close-up.
- **Slowed by:** no runner times a chosen tree at chosen views.
  - `telperion_render::measure` exists and is sound: it gives timestamp pairs around the vegetation, selection and shadow passes, and a contention verdict.
  - But nothing calls it on an engine tree, or at a camera other than the hero pose. The stills runner renders and does not time.
- **Cost:** about 10 minutes and one release build. I wrote a scratch example (`f3_frame`, not committed; a copy is in `f3-frame-probe.rs`).
- **Would remove it:** a `--time` flag on the stills runner (`examples/space/still.rs`) that runs `measure` at each shot it renders, `View::Bare` and `View::Whole`.

## 2026-10-05, design research: shell guard on a loop of logs

- **Doing:** running the four timing runs in a loop, each to its own log.
- **Slowed by:** the dcg hook refuses a `>` redirect to a path built from a loop variable.
- **Cost:** about 1 minute, rewritten as one appended log.
- **Would remove it:** nothing in the repository; it is a local hook setting (fn-198 FRICTION.md records the same hook).

## 2026-10-05, step 1: the gate's beech memory ceiling and the limit inventory

- **Doing:** running the workspace gate once after steps 1 and 2.
- **Slowed by:** two failures.
  - `fixed_beeches_pass_geometry_and_profile_gates_with_repeatable_varied_specimens` read a peak resident of 5.91 GB against its 5.76 GB process ceiling. It passed when rerun alone, in 22 s. fn-206 recorded the same flake.
  - `generation_limit_guard` flagged the curve's nine `min`, `max`, `clamp` and constant sites, which were added to `docs/generation-limits-inventory.json` with reasons. That one was mine and expected, but nothing ahead of the 8-minute gate would have told me.
- **Cost:** about 10 minutes: one gate run and two reruns.
- **Would remove it:** run `generation_limit_guard` from the crate's own test command, cheap and early. Make the beech's ceiling test refuse to run beside other heavy tests (a nextest test group), rather than fail under load.
