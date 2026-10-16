# fn-57 evidence

The 2026-09-16 sweep and the five live Jev pilots that sit behind
`docs/typesafe.md`.

| Path | What it is |
|---|---|
| `INVENTORY.md` | Sixteen sites, one disposition each (`jev`, `code`, `leave`) |
| `pilot/RESULTS.md` | Labelled outcomes, token and wall-time totals |
| `pilot/SOURCES.md` | Fetched source URLs, SHA-256 and byte counts |
| `pilot/jev.py` | The pilot caller. Evidence, not a shipped tool |
| `cases-live.txt` | Rewrite live run of `jev cases`: per-case rows, accuracy and confidence spread |

The pilot's labelled sets (formerly `pilot/exp*.json`) now live, rewritten, in
`crates/telperion-jev/data/cases`; the pilot build scripts are archived (see
`.flow/evidence/ARCHIVE-MANIFEST.md`). The shipped caller and tools are a
rewrite with tests in `crates/telperion-jev`. Do not invoke the Python pilot as
a product command.
