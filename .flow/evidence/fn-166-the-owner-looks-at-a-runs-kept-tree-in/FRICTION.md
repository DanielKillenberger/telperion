# fn-166 friction

## 2026-09-27, task 1 (worker)

- **Doing:** the manual check of `species european-beech --look` against fn-157's live beech run (`beech-live2`).
  **Hindered by:** its kept overlay sets `/skeleton/habit/stemDivergence` and `stemForkHeight`, which master retired (fn-170), so core refuses the overlay and no look of that run can open on master. The refusal is R3 working, but the only real run could not be looked at.
  **Cost:** about 5 minutes; the fixture drops the two rows and the manual check ran on it.
  **Would remove it:** the beech's next Tune revision on master (fn-62), or a mapping of retired rows onto their replacements, which is the host's decision and not built here.
- **Doing:** checking that the dev server serves a missing look as an error.
  **Hindered by:** Vite's SPA fallback answers `/harness/looks/<missing>.json` with `200 text/html` (the page), so a missing look would have read as "not JSON".
  **Cost:** about 5 minutes. The fetch now treats a non-JSON answer as missing and names the file.
  **Would remove it:** nothing further; noted for the next harness fetch.
- **Doing:** the R1 test in `npm test`.
  **Hindered by:** it builds `species` in release (`cargo build --release -p telperion-jev --bin species`), about 80 s cold in a fresh worktree target, cached after.
  **Cost:** 80 s once per cold target.
  **Would remove it:** nothing needed while the release build is cached; `--tools`-style reuse is not worth a flag.
