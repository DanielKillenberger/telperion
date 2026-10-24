# Evidence retention

Keep enough evidence to understand a decision and reproduce a result. A PR should not carry every run of the investigation.

Commit:
- Specs and task receipts, final result summaries, qualification gaps, decisions and friction dispositions.
- Reusable verification or benchmark sources and the small inputs they require.
- Curated numeric summaries and provenance needed to assess a claim.
- For a branch that changes trees, its `space_hash` lines (`crates/telperion-render/examples/space_hash.rs`) in its RESULT.md, so a later merge can compare against them without rebuilding the branch.
- Test fixtures that executable tests actually load; these are source inputs, not disposable run output.
- Each still sheet an owner verdict rests on, as one compressed JPEG beside the spec's evidence (outside `raw/`), so a later spec can compare against the approved look without rebuilding it in another worktree.

Write raw logs, stdout/stderr, per-iteration measurements, screenshots, videos, scratch patches and temporary diagnostic output under `.flow/evidence/<spec>/raw/`. That directory is ignored. Demo recordings belong in the ignored `demo-video/` directory. Generated output should not be force-added simply because a workflow collected it. Promote a necessary fixture or concise summary deliberately and explain its role.

Before opening a PR, inspect the staged evidence file count and byte size. Retain the final result and rejected approaches' conclusions, not every intermediate run. Check references before removing anything: archive-dependent historical replay scripts must say how their inputs are recovered, and executable test inputs must remain present.

For an existing evidence cleanup, archive and verify the original bytes before removal. Keep a compact manifest with source revision, paths and checksums, plus recovery instructions. A local archive is a convenience, not shared durable hosting. Previously committed evidence stays recoverable from its pinned Git revision; never rewrite repository history merely to remove it. For new uncommitted evidence, arrange durable storage before deleting the only copy when reproducibility needs it.

The fn-91 cleanup preserved its reports, JSON summaries/provenance, replay scripts and patches; the 2026-10-16 trim archived most of them and keeps only the reports README and `scripts/benchmarks/generation.md` cite (recoverable through `.flow/evidence/ARCHIVE-MANIFEST.md` at `8dab10634`). Those scripts describe historical experiments; some consume archived raw measurements or use the original machine's scratch paths. Use the documented historical checkout for replay rather than treating them as portable CI tests.

## Closed specs

The owner asked on 2026-10-24 to cut a release diff that still added 250k lines ("it's still 250k loc added which is a bit crazy. can we do another cleanup?") and chose to prune closed specs. A spec that is done, or closed, keeps its spec `.md` and `.json`, its summary, and the compressed sheets the owner's verdicts rest on. The summary is the first top-level `RESULT.md`, `RESULTS.md`, `REPORT.md` or `README.md`, else a top-level `*RESULT*.md`; a spec with none keeps its spec as the record. Everything else goes: the remaining evidence (logs, tables, scripts, patches, raw notes, FRICTION.md) and its task `.md` and `.json`, which flowctl needs together or not at all.

An open spec keeps everything, and so does every spec fn-237 depends on directly until fn-237 closes. A closed spec also keeps what is still read: every file that a tracked file outside `.flow/`, or an open or fn-237-dependency spec's record, names by path or by file name on a line that names its spec; a named task with the tasks it depends on, transitively; the directories code reads whole; the inputs of a kept script or JSON file, to a fixpoint; its `.gitignore` files; a PNG still its own spec cites; and a FRICTION.md the host has not disposed of. A closed spec's FRICTION.md goes once its spec records each entry's disposition. The 2026-10-24 prune is recoverable through `.flow/evidence/ARCHIVE-MANIFEST.md`.
