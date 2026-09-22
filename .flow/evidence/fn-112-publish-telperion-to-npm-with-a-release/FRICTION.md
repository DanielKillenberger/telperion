# fn-112 friction

## 2026-09-22 — a redirect into the spec's raw/ was blocked by dcg

Writing `npm pack --dry-run --json` into `raw/pack-dry-run.json` through a
shell variable holding the evidence path was refused by the dcg hook
(`redirect-truncate-dynamic-path`); the same redirect with the path spelled
out ran. Cost: one retry, about a minute. A local setup matter on this
machine, reported and not specced: an allow rule for redirects under
`.flow/evidence/*/raw/` would remove it.

## 2026-09-22 — R6 needs a page the registry does not have yet

The npm trusted-publisher form sits on a package's settings page; the
`telperion` name is unpublished, so whether the form is reachable before a
first publish is unknown from the docs and only the owner's account can
tell. Cost: one docs read and a written unknown in RESULTS.md rather than a
step. What would have removed it: the spec's Parked unknowns naming that the
owner may need one hand publish before the workflow's first tag.
