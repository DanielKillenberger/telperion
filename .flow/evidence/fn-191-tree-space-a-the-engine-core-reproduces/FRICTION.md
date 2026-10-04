# fn-191 friction

## 2026-10-03, task 1, writing the test helpers

- **Doing:** creating `crates/telperion-space/tests/common/mod.rs` with a shell heredoc in the same command as the `mkdir` for its directory.
- **Hindered by:** the local `dcg` command guard blocks a truncating redirect whose parent directory does not exist yet. It reports the rule but not that the missing parent was the trigger.
- **Cost:** about 1 min, one retry through the file tool.
- **What would remove it:** creating directories in a separate command first. This is the owner's local setup, so it is reported, not specced.

## 2026-10-03, task 1, the oracle's variance comparison

- **Doing:** comparing the spread of counts against the oracle.
- **Hindered by:** for a count that takes two values with equal odds (the trunk's one or two nodes), the large-sample standard error of the variance, (m4 - var^2)/n, is exactly zero. The first comparison therefore demanded identical variances. The corrected error, (m4 - var^2 (n-3)/(n-1))/n, keeps a spread.
- **Cost:** about 3 min.
- **What would remove it:** nothing in the repository. It is a statistics detail, written down here so the next oracle comparison starts from the corrected form.
