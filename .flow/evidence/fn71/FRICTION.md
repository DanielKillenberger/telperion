# fn-71 friction

## 2026-09-18, codex bridge refused on quota again
A one-word probe of `codex exec -m gpt-6-astra` before composing the brief returned the same usage-limit refusal fn-55 met (until 2026-09-19 16:27). Cost: one minute, and the whole task on the session model. What would remove it: a routing block that names the fallback when the frontier tier is out of quota, so the decision is made once a day rather than once a task.

## 2026-09-18, the resolution numbers print only on failure
Reproducing fn-55's margins meant rerunning three test files with `--nocapture` and reading the stderr by hand, and every probe of a shader change is another full run of the same. Cost: about two minutes per probe, eight probes so far. What would remove it: the receipts R2 asks for, written on every green run, which this spec adds.

## 2026-09-18, the command guard blocks compound shell blocks
`dcg` refused two blocks in a row: a `$F gate check` call through a variable-held path, and a heredoc python edit followed by a test run redirected to a variable-held log path. Each had to be split and rerun. Cost: about four minutes. What would remove it: a local setup allowlist for the flowctl path and the scratchpad log directory (the owner's machine, not the repository).

## 2026-09-18, round 2: the camera-walk sweep measured the framing, not the shading
The first sweep walked the camera to f times the distance and compared the far still's centre crop with the near still reduced; with the relief off entirely the far draw still read 7 to 13% brighter, because a crop of the same pixel size at f times the distance sees more of the lit side of a cylinder. Two rounds of shader changes were judged on that number before a relief-free control exposed it. Cost: about twenty minutes and one misdirected change. What would remove it: walking the footprint alone (the same camera drawn at a fth of the size, registered pixel for pixel), which the sweep tool now does when the walked still is absent.

## 2026-09-18, round 2: the command guard again
`dcg` refused overwriting the working-tree shaders from the round-one commit to measure it, so the baseline curve came from a scratch worktree and a fresh 58 s build; `--family` wants a file path where the usage line reads `<json>`, one failed sweep of 28 renders. Cost: about eight minutes. What would remove it: the headless usage line saying `--family <file>`; the guard is the owner's machine, not the repository.
