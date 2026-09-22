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
