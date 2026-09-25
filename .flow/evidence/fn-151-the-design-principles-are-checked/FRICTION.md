# fn-151 friction

## 2026-09-25, labelling the runner positives

- **Doing:** labelling each confirmed positive by mechanism and span on its immutable revision, for the replay corpus.
- **Slowed by:** the survey points most runner PRs (#38, #76, #77, #83, #87, #94) at module doc comments (`:1-13`), not at code. The stop each PR added had to be found by hand in its diff, and several are design-level ("another classification layer") with no single span.
- **Cost:** about 25 minutes and roughly 40k tokens of diff reading.
- **Would have removed it:** a survey column for the stop site's `path:line` at the head revision, written when the PR is labelled.

## 2026-09-25, the shell guard blocks truncating writes

- **Doing:** writing new data files and fixture answers with shell redirects.
- **Slowed by:** the local `dcg` hook refuses `>` onto a new path whose parent directory does not exist yet, and any redirect to a shell-expanded path, and `git checkout <ref> -- <path>` in a scratch worktree. Each refusal needed a rewrite through the file tool or Python.
- **Cost:** about 5 minutes over four refusals.
- **Would have removed it:** nothing in the repository; a local setup matter, reported only.

## 2026-09-25, Jev's first confirming question barely separates

- **Doing:** the live evaluation, round one (59 calls).
- **Slowed by:** the single phase-two Noul ("breached, and no allowance covers it") answered 0.36 to 0.68 on confirmed breaches and up to 0.61 on clean pushes, so no cut separated them. Round two split it into "shown" and "covered" and re-asked phase two only (29 calls), which separates better but still misses every runner positive in the holdout.
- **Cost:** 88 Jev calls, about 322k input tokens, two rounds.
- **Would have removed it:** a small labelled set of phase-two cases per principle before the first corpus run, as `docs/typesafe.md` asks for other question sets.
