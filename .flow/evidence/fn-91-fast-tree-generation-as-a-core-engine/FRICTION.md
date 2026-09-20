
## 2026-09-20 — baseline tooling build

The direct mature path had no reusable native per-stage timing runner. Adding a small example and compiling its release binary has cost roughly two minutes of inspection and an ongoing build wait (over ten seconds so far). Retaining the runner removes this setup from the candidate comparison; attachment preparation remains included in placement pending host direction.

## 2026-09-20 — browser baseline build and completion boundary

Fresh core and renderer Wasm builds took 14.17 and 14.99 seconds before browser measurement. The renderer exposes frame submission but not GPU completion, so the current runner cannot measure the first completed frame without an additional benchmark-only fence. The host was informed; current evidence labels submission as a lower bound. A retained benchmark completion hook would remove that gap. No captures were taken.
