# Friction: fn-127

## 2026-09-23: a shell edit refused by the local command guard

- **Doing:** adding the R2 red test to `tests/requirements.rs` through a Python heredoc.
- **Hindered by:** the local `dcg` hook refused the whole command because a Rust type in the heredoc body (`Vec<(&String, &Value)>`) parsed as POSIX process substitution.
- **Cost:** about 1 minute and one retry. The same edit went through the editor tool.
- **Would remove it:** nothing in the repository. This is a local setup matter, reported and not specced.

## 2026-09-23: the verify path for appearance spans was an unforeseen design point

- **Doing:** R2, recording the chosen span's source on every appearance value.
- **Hindered by:** until now an appearance entry had no source, so `verify` skipped its citation claim and its measurement obligation as "unchecked". With a source and span recorded, both checks now run on the appearance span, and the design did not say whether they should. I left `verify` unchanged. The fixture test holds, but a live section that carries numbers could still list a claim.
- **Cost:** about 10 minutes of reading `verify` and `cite` before choosing the smallest change.
- **Would remove it:** the host confirming on the palm rerun whether appearance spans belong in the citation check and the measurement obligation.
