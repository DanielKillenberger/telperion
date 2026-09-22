# Evidence retention, 2026-09-22

The owner asked for the fn-68 evidence to be cut to what a reader needs, with Jev labelling every file and keep labels beside drop labels so nothing important is filtered by omission. `retention-classify.py` did it in three steps, all recorded.

1. **Code listed every tracked file** with its size, lines, first commit, an excerpt, and what cites it in code, scripts, docs or other evidence. A file cited by code or scripts, or named in the keep list (the report, the friction files, the run notes, the protocols, the result), is kept without asking: 19 files.
2. **Jev labelled the other 251** in batches of eight, one choice per file over essential record, reusable source, superseded intermediate, redundant duplicate, raw low-info, or unsure. Ledger under `.flow/ledger/fn68-retention/`. Labels: 97 essential, 104 reusable, 29 superseded, 19 raw, 2 redundant, 0 unsure.
3. **Code dropped a file only when the three drop labels' summed probability reached 0.6**, the same mass rule the loop's acceptance uses, and never a hard keep. 37 files, 233 KB, 5,831 lines: mostly reservations, preflight and resume snapshots, per-run handoff dumps and captured stderr whose outcome a run note records.

Every dropped file is in `retention-manifest.json` with its sha256, label, drop mass and ledger reference, and in the ignored `raw/retired-2026-09-22/` beside this file. Recovery: `git show <source_revision>:<path>` with the revision in the manifest. Nothing an executable test or script loads was dropped: the citation check runs before Jev is asked.
