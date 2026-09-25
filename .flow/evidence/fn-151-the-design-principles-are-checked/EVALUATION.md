# fn-151 evaluation, 2026-09-25

The corpus is `crates/telperion-jev/data/principles/corpus.json`: 17 positives and 22 clean pull requests, labelled by the owner on 2026-09-25, each positive by one mechanism and span on its immutable revision. Lineages decide the groups: calibration holds the pipeline copies, the habit enum, render and memory work (5 positives, 16 clean); holdout holds the species runner and the palm stack (12 positives, 6 clean). `jev principles report` reproduces every number below from the frozen extractions and recorded answers.

## Replay per labelled PR (R4)

| PR | Group | Owner | Verdict |
|---|---|---|---|
| #6 | calibration | boundary guard | caught: `cargo test` whole-tree boundary, `telperion-wasm/src/generate.rs:78` (a move, so the push-mode check alone passes it) |
| #58 | calibration | boundary guard | caught: push-mode boundary, `telperion-field/src/grow.rs:14` |
| #55 | calibration | Jev | caught by the boundary guard at `telperion-wasm/src/generate.rs:187`; Jev's candidate showed 0.76 and was cut by `P-NO-SWITCH`'s 0.85 |
| #115 | holdout | entry guard | caught: the slim leg is red on `39348def` for `date-palm` |
| #3 | calibration | Jev | caught: `P-NO-SWITCH` at `branching.rs:269` (shown 0.92, covered 0.47) |
| #13 | calibration | Jev | caught: `P-CONSUMER-READS` at `wood.wgsl:18` (shown 0.83, covered 0.27) |
| #59 | holdout | Jev | missed: the labelled span showed 0.62; a sibling `rosette::bearing` switch at `placement.rs:385` fired (0.87, 0.38) and counts as a false finding under the pre-registered span |
| #53, #61, #38, #75, #76, #77, #83, #87 | holdout | Jev | missed: candidates reached each span; phase one named a switch, never `redundant_stop`, and phase two did not confirm |
| #52, #94 | holdout | Jev | unsupported: #52's twin adapter fell past the 12-candidate cap (76 dropped); #94's classification layer has no candidate class |
| 22 clean PRs | both | | no finding; #17, #50 and #9 report incomplete (candidates over the cap) |

## Live evaluation (R5)

| Group | Jev recall (end to end) | Finding precision | Clean pushes flagged |
|---|---|---|---|
| calibration | 2/3 [21%, 94%] | 2/3 [21%, 94%] | 0/16 [0%, 19%] |
| holdout | 0/11 [0%, 26%], 2 unsupported | 0/1 [0%, 79%] | 0/6 [0%, 39%] |

Intervals are 95% Wilson. The advisory targets (at least 90% recall, 95% precision, at most 1% clean-push false flags) are not met; every principle stays in shadow. At zero false flags, the clean set bounds the false-flag rate only below 19% (calibration) and 39% (holdout).

Cuts were chosen on the calibration group alone: `P-NO-SWITCH` needs `shown` at 0.85 or above, every other principle 0.70, and every principle `covered` at 0.50 or below. The uncalibrated principles (`P-ONE-PIPELINE`, `P-NO-FALLBACK`, `P-BLOCKING-STEP`, no calibration positive) keep the default and stay in shadow.

Two rounds ran. Round one asked one conjunctive Noul ("breached, and no allowance covers it"); it gave 0.36 to 0.68 on confirmed breaches and up to 0.61 on clean pushes, so no cut separated them. Round two split it into `shown` and `covered` and re-asked phase two only. Round one's answers are kept in `raw/round1/`.

## Spec mode (R7)

- fn-150's final design: three current proposals (the ribbon rounds are left out as superseded); no finding.
- The labelled duplicate-path spec (`specs/duplicate-path.md`): phase one names `surviving_duplicate` at 0.95 and phase two finds it shown at 0.84, but `covered` is 0.64, over the 0.50 cut, so no warning. R7's "flags" is not met.
- A proposal with no decision context abstains without a call.

## Cost

| Run | Jev calls | Input tokens | Output tokens |
|---|---|---|---|
| Round one, corpus | 59 | 229,307 | 24,543 |
| Round two, phase two only | 29 | 87,189 | 4,713 |
| Spec mode, three recordings | 9 | 26,565 | |
| Latency, three measured passes | 36 | 85,758 | |
| Probe | 2 | 8,378 | |
| Total | 135 | 437,197 | |

## Hook latency (R6)

`jev principles push` on the 14 newest master commits, this desk, load average 10 to 14 from other sessions: uncached median 1.28 s, p95 2.58 s, max 2.66 s (12 Jev calls, 29,261 input tokens); cached median 18 ms, p95 19 ms. An earlier uncached pass under heavier load reached a p95 of 5.56 s and a maximum of 10.8 s; the extraction was then made parallel and cached. One uncached run used 8,313 input tokens, and the hook's first push of this branch spent 8,592 in phase one alone, both over the 8,000 bound. Extractor `extract-2` now bounds phase one on its whole request at three bytes a token (5,000), and phase two confirms the surest selections that fit the rest; the re-run of the same push spent 4,422 in phase one. The frozen corpus stays on `extract-1`.
