# fn-182 friction

## 2026-10-02: no list of the sites a new row must touch

- **Doing:** adding `twigShell` and `twigShellSoftness` to the twig rows.
- **What slowed it:** the recipe (catalogue counts, sweep held rows, snapshot schema, identity digests, limits inventory, generated docs and browser metadata, frozen geometry protocol, the Jev dial counts, the conformance jitter set) lives only in fn-177's diff. Four of those sites (`geometry_benchmark`, two `tuning_engine` counts, `conformance`) were found by the full gate, not before it.
- **Cost:** about 15 minutes, including a 314 s gate run that ended with 4 failures and three targeted re-runs.
- **What would remove it:** a short "adding a row" checklist in `docs/parameters.md`'s header, or one catalogue test that names each site a new row reaches.

## 2026-10-02: the query profiler takes a preset, not a family

- **Doing:** counting the shell's crown queries on the beech and plane candidates.
- **What slowed it:** `growth_profile --features query-count` reads a preset id and a seed only, so a candidate's overlay or a row value cannot be passed. A scratch example was written to overlay rows and print the counts.
- **Cost:** about 10 minutes.
- **What would remove it:** an optional `--family FILE` overlay on `growth_profile`, as the headless renderer has.

## 2026-10-02: the shell guard blocks redirects to variable paths

- **Doing:** writing digest and still output under `$S/...` and `$E/...`.
- **What slowed it:** the dcg hook refuses `> $VAR/file`. Each command had to be rewritten with literal paths or moved into a script.
- **Cost:** about 3 minutes over two commands.
- **What would remove it:** this is local setup, reported rather than specced.
