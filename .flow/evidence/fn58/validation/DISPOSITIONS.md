# fn-58 R7 validation against fn30, dispositions

Two fixture runs, the oak and the spruce, each with one admitted manifest
under `<species>/manifest.json`, pinned sources under `<species>/fixtures/`
(the Ertragstafeln tables transcribed from fn30's `curves.py` as markdown
with decimal commas, the Silvics and Iowa State sentences), and the expected
fit under `<species>/expected/fit.json` read off fn30's `REPORT.md` as
printed. The test `crates/telperion-jev/tests/validation_fn30.rs` runs the
fetch and fit stages over these fixtures offline and compares the fit to the
expected artifact. Every difference is listed here; a person records its
disposition (`fn30 error`, `pipeline error`, `accepted alternative`), and an
unresolved difference leaves R7 unmet.

| Species | Artifact | Pointer | fn30 | Pipeline | Disposition | By |
|---|---|---|---|---|---|---|
| oak | fit | /rows/0/reference_height_m | 8.00 | 8.01 | accepted alternative (fn30 printed rounded values; the pipeline carries full precision) | Daniel Killenberger, 2026-09-18 |
| oak | fit | /rows/1/reference_height_m | 16.00 | 16.003 | accepted alternative (same) | Daniel Killenberger, 2026-09-18 |
| oak | fit | /rows/0/reference_dbh_m | 0.064 | 0.0641 | accepted alternative (same) | Daniel Killenberger, 2026-09-18 |
| oak | fit | /rows/1/reference_dbh_m | 0.112 | 0.1120 | accepted alternative (same) | Daniel Killenberger, 2026-09-18 |
| oak | fit | /rows/2/reference_dbh_m | 0.236 | 0.2357 | accepted alternative (same) | Daniel Killenberger, 2026-09-18 |
| spruce | fit | /rows/0/reference_height_m | 5.00 | 5.006 | accepted alternative (same) | Daniel Killenberger, 2026-09-18 |
| spruce | fit | /rows/1/reference_height_m | 10.00 | 10.004 | accepted alternative (same) | Daniel Killenberger, 2026-09-18 |
| spruce | fit | /rows/2/reference_height_m | 15.00 | 15.019 | accepted alternative (same) | Daniel Killenberger, 2026-09-18 |
| spruce | fit | /rows/*/reference_dbh_m | 0.106, 0.203, 0.285 | 0.1058, 0.2028, 0.2852 | accepted alternative (same) | Daniel Killenberger, 2026-09-18 |
| both | fit | /misses | fn30 judged DBH misses against measured wood at each age on the growth path | the pipeline judges `mature_dbh_m * fraction(age)` against the composed reference, as fn30's `curves.py` model columns did | accepted alternative (the owner's 2026-09-18 rule makes the direct build the product; the growth-path measurement is not run here) | Daniel Killenberger, 2026-09-18 |
| both | sources | E1 sha256 | c6c7d655... (the PDF bytes) | none (the fixture is a transcription; the manifest leaves `sha256` unset) | accepted alternative (the live run over the real PDF records the checksum) | Daniel Killenberger, 2026-09-18 |

What this validation does not cover: the screen, quality, select and verify
stages over live Jev, the gate and generate stages over the built examples,
and the report. Those run live on the runbook and their artifacts are
compared by the same table once a person has run them.

Dispositions recorded 2026-09-18 by the owner in session: the nine rounding rows are one difference (fn30 printed rounded values, the pipeline carries full precision); the misses row follows the owner's 2026-09-18 rule that the direct build is the product; the checksum row fills on the first live run over the real PDF, whose bytes hashed to c6c7d655 on 2026-09-18 (`.flow/evidence/fn58/parse-ertragstafeln.json`).
