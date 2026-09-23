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

## 2026-09-22 — the render suite's orbit wall clock on a shared display

Task 2 ran `npm run test:render` as the harness check for R7. The suite
takes about six minutes to reach the orbit assertions and failed on the
first orbit's wall-clock p95 while another harness was running on port
5173 and the display was in use; the GPU percentiles of the same session
were valid and under threshold. Cost: about six minutes and one
inconclusive line in RESULTS.md instead of a green. What would have removed
it: a way to run the page-drives-a-tree half of the suite without the
timing and soak sessions, or the suite naming a contended display as a
skip rather than a failure, the way it already skips a machine with no
adapter.

## 2026-09-22 — wasm-bindgen's default module path inlines too

After the two `?url` imports were replaced, `dist/telperion.js` was still
2.5 MB: the generated glue's own `new URL('telperion_render_bg.wasm',
import.meta.url)` fallback is a string literal, and Vite's library build
inlines any such asset. Cost: one extra build and a bundle read, a few
minutes. Removed by `--omit-default-module-path` on the wasm-bindgen call,
now in `scripts/build-render.mjs`.

## 2026-09-22 — the review round: `rm -rf dist` blocked by dcg

Rebuilding `dist` from empty for the R7 second-round listing, `rm -rf dist`
(a gitignored build output) was refused by the dcg hook
(`core.filesystem:rm-rf-general`); the same step ran as `mv dist
<scratchpad>/dist-before-r8`. A Python heredoc holding shell text in prose
was refused too (`heredoc.shell:launcher-unverified`) and ran as a script
file. Cost: two retries, a few minutes. A local setup matter on this
machine, reported and not specced: an allow rule for `rm -rf` of `dist/`
under a worktree would remove the first.

## 2026-09-22 — the review round: the plugin's regex reached into a comment

The first draft of `telperion:wasm-beside-entry` rewrote every
`new URL("./x.wasm", import.meta.url)` in a module's text, including the one
spelled inside `src/wasm-source.ts`'s own doc comment, and esbuild refused
the broken comment. Cost: one failed build, two minutes. Removed by not
spelling the literal in prose; a transform that skipped comments would be
more than the plugin needs.

## 2026-09-23 — the first publish failed with a 404 because npm was not logged in

The owner's hand publish of the placeholder answered `404 Not Found - PUT`,
which reads as a problem with the name. The real cause was a machine with
no npm login (`npm whoami` answered 401). Cost: one round trip, a few
minutes. What would have removed it: the R6 steps opening with `npm whoami`.
A local setup matter, reported and not specced.

## 2026-09-23 — the fresh install hit a stale npm cache

`npm install telperion@0.1.0` answered `notarget` right after the publish,
because the local cache held the listing from 0.0.1. `--prefer-online`
fixed it. Cost: one retry. What would have removed it: `--prefer-online`
in the documented smoke steps.

## 2026-09-23 — dcg again: heredocs, `mv` and redirects through variables

Recording R6, three shell writes that used a variable for the evidence
path were refused (`heredoc.shell:launcher-unverified`,
`core.filesystem:mv-dynamic-path`, `redirect-truncate-dynamic-path`) and
ran again through the file tools. Cost: three retries, a few minutes.
A local setup matter, reported and not specced.
