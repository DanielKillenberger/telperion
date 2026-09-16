# fn-57 evidence

The 2026-09-16 sweep and the five live Jev pilots that sit behind
`docs/typesafe.md`.

| Path | What it is |
|---|---|
| `INVENTORY.md` | Sixteen sites, one disposition each (`jev`, `code`, `leave`) |
| `pilot/RESULTS.md` | Labelled outcomes, token and wall-time totals |
| `pilot/SOURCES.md` | Fetched source URLs, SHA-256 and byte counts |
| `pilot/exp1_screen.json` | Twelve labelled sentences and the screen questions |
| `pilot/exp2_cite.json` | Nine labelled claims |
| `pilot/exp4_triage.json` | Eight routes, seven duplicate pairs, four severity labels |
| `pilot/exp5_sections.json` | Report-section selection over fn-30 `REPORT.md` |
| `pilot/jev.py`, `build_exp.py`, `build_exp2.py` | Pilot scripts. They are evidence, not shipped tools |

The shipped caller and tools are a rewrite with tests in `crates/telperion-jev`.
The labelled cases the tools load live in `crates/telperion-jev/data/cases`.
Do not invoke the Python pilots as product commands.
