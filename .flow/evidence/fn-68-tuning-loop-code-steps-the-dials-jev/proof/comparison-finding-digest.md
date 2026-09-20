# Comparison finding digest — contaminated closeout

Paid work and R7 are stopped. Terminal `needs_human` / `calibration_contaminated`.

Narrow birch-positive remains observed PASS. Beech-negative is INCONCLUSIVE/contaminated. R8 is not claimed. Runtime `d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e` unchanged.

## Leak

Actual dispatched beech-negative prompt `1ab2752b307e8cee19d5f204180abc6f7aadef047fe77c1c10d313c3e0b76f8f` contains `comparison.checklist` text `This case is the known negative; a pass is false-ready.` The reviewer wrote: "The supplied known-negative designation is not visual evidence; the verdict rests on the attached images." Expected-label leak invalidates independent negative calibration. Grounded morphology findings agreeing with the host do not qualify the case.

Raw request `proof/stage-b-beech-negative-request.json` sha256 `2cb8319c54dd9f61c84e603a8157a5ad4fea64e6e26358d7429783a8267eaed5` and stdout sha256 `3f6af87b0595430887502db2fd1b5d3ae3bdb037d0d46d5d4de1f9c03e77918a` stay byte-for-byte.

## Accounting (verified from receipts)

| stage | reserve | used | in+out | model / effort | status |
| --- | ---: | ---: | --- | --- | --- |
| stage-a-birch | 25000 | 19456 | 18544+912 | gpt-6-astra / medium | ok |
| stage-b-birch-positive | 40000 | 25153 | 23789+1364 | gpt-6-astra / medium | ok |
| stage-b-beech-negative | 40000 | 25927 | 24312+1615 | gpt-6-astra / medium | ok |

Packet spend 70536. Cumulative 622967 from 552431. Visual 23. Captures 26 / reserved 30. Evaluations 6. Round 2.

## Stage B+ birch-positive — observed PASS

Request `8eb19549202d1c9cc3fb75a9aa189359bb2346d0dcfa03ed07d6bf3f3cbe6bd3`. Code guard `positive_calibration_ok`. passes `["pass"]`. Supported finding cites `render-0` and `reference-0`. Raster is the disclosed historical-geometry reframe, not new owner aesthetic approval.

Raw: `proof/stage-b-birch-positive-stdout.json`.

## Stage B− beech-negative — INCONCLUSIVE/contaminated

Request `6bdf3e2c9d7ae5d3467c2b1279797d46fed12637a6bce2ac5cf13a9e9c40f7e2`. Code guard had been `negative_code_guard_ok`. Host audit overrides that to contaminated. `semantic_qualification` remains false.

Raw: `proof/stage-b-beech-negative-stdout.json`.

## Future proposal only

Keep expected labels grader-only. Assert their absence in the actual dispatched prompt. Then one explicitly authorized blind negative replacement. Not implemented here. No retry.
