# fn-55 friction

## 2026-09-18, codex bridge refused on quota
Dispatching the implementation to `codex exec -m gpt-6-astra` per the routing block: the CLI printed `You've hit your usage limit ... try again at Sep 19th, 2026 4:27 PM` and exited in seconds. Cost: about two minutes and the fallback to the session model for the whole task. What would remove it: a quota probe before the bridge (one cheap `codex exec` with a trivial prompt) so the routing decision is made before the brief is composed.

## 2026-09-18, resolution bounds sat within 3% of their limits at base
The 4x distance-series mean read 2.904 against a bound of 3.0 and the birch close-up 2.987 against 3.0 before a line was changed, with nothing in the test output or the evidence saying so; the numbers only print on failure or with `--nocapture`. Finding out took two stash-and-rebuild cycles, about twelve minutes. What would remove it: each resolution test recording its measured margins to a receipt on a green run, so the next change to shading knows where it stands before it runs.

## 2026-09-18, the species runner's quick look needs the fn34 profiles by hand
`node tests/species.mjs --quick european-beech` fails with `No matched reference records` because the default profiles path is fn9's; the fn34 profiles have to be passed with `--profiles`. Cost: one failed pair of runs, about three minutes. What would remove it: the runner resolving a preset id to the profile set that carries its records.
