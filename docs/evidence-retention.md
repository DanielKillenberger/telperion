# Evidence retention

Keep enough evidence to understand a decision and reproduce a result. A PR should not carry every run of the investigation.

Commit:
- Specs and task receipts, final result summaries, qualification gaps, decisions and friction dispositions.
- Reusable verification or benchmark sources and the small inputs they require.
- Curated numeric summaries and provenance needed to assess a claim.
- Test fixtures that executable tests actually load; these are source inputs, not disposable run output.

Write raw logs, stdout/stderr, per-iteration measurements, screenshots, videos, scratch patches and temporary diagnostic output under `.flow/evidence/<spec>/raw/`. That directory is ignored. Demo recordings belong in the ignored `demo-video/` directory. Generated output should not be force-added simply because a workflow collected it. Promote a necessary fixture or concise summary deliberately and explain its role.

Before opening a PR, inspect the staged evidence file count and byte size. Retain the final result and rejected approaches' conclusions, not every intermediate run. Check references before removing anything: archive-dependent historical replay scripts must say how their inputs are recovered, and executable test inputs must remain present.

For an existing evidence cleanup, archive and verify the original bytes before removal. Keep a compact manifest with source revision, paths and checksums, plus recovery instructions. A local archive is a convenience, not shared durable hosting. Previously committed evidence stays recoverable from its pinned Git revision; never rewrite repository history merely to remove it. For new uncommitted evidence, arrange durable storage before deleting the only copy when reproducibility needs it.

This policy does not retroactively delete other specs' evidence. The fn-91 cleanup preserves its reports, JSON summaries/provenance, replay scripts and patches. Those scripts describe historical experiments; some consume archived raw measurements or use the original machine's scratch paths. Use the documented historical checkout for replay rather than treating them as portable CI tests.
