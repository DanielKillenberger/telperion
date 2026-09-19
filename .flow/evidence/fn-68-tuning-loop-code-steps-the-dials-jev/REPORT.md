# fn68 implementation and calibration checkpoint

Status: in progress. Deterministic implementation and focused tests are green; a live beech tuning pilot and all-cell convergence proof have not yet run. No preset was shipped. No generator or renderer behavior was changed by this continuation.

## Recovered historical identity

Two bounded historical builds and six images recovered the exact historical still bytes. `scripts/tuning-reconstruct.py` records the render argument construction; revisions are beech `4e6d903c` and birch `0f2c2f6a`. Build logs are `/tmp/fn68-reconstruction-build1.log` and `/tmp/fn68-reconstruction-build2.log`; render logs `/tmp/fn68-reconstruction-images.log` and `/tmp/fn68-reconstruction-birch.log`. Images remain under `/tmp/fn68-reconstruction-images`, not in git.

| Historical view | SHA-256 |
| --- | --- |
| beech BWHOLE | 7e69fda3e95b317fcb9d89e15e32179c54fdd316a57d1c3223d6e26639b8b490 |
| beech BBARE | 4c7373ad697e4038efba254fa08bfa7943207f57979d4e3905c37136bea2ec75 |
| beech BBASE | 6a067457b2460f166b59c25ca3a5b29e941822f7aaff37f3589f598d8629439c |
| birch SWHOLE | 2745f4c37a4410236f2adfb0d6b0c3903c7312bd8a15872ef8e58d4c35293d1e |
| birch SBARE | 246aa0c58eae50300ec5be6520aca8a854cf97506eb5c86f581c7ede7c29be68 |
| birch SBARK | 67d8000c31f2272ef242b3f78f622213522fabf6020834f6ad6ddc614248edbb |

Beech rows match round22-ships in historical fn34 rounds.tsv (retained at `e5eaa4fd`); birch matches round26-birch-crown at `471e1385`. The host independently checked the beech rows. Exact reconstruction licenses the historical labels for those images, not different current renders. Current beech authored preset rows are unchanged from `4e6d903c`, but generator/renderer versions differ; any current pilot comparison must disclose that difference.

## Frozen calibration results

`PROTOCOL.md` records hashes, budgets, prospective rubric changes and the composed-policy measurement correction. `cases/*-result.json` are persistent result indices; their immutable Jev ledgers remain locally under `.flow/ledger/fn68-calibration`. Visual ledgers remain ignored in `vision-ledger/`. A copied checkout without those ledgers/images cannot claim reverified calibration merely from this report.

| Role | Held-out result | Limitation |
| --- | --- | --- |
| direction, Jev 1.13.0 | 4/5 = 0.8 | correlated round22 owner-note dial labels |
| magnitude, Jev 1.13.0 | 7/8 = 0.875, zero unsafe | authored interpretation/bounds, not biological efficacy |
| continuation auxiliary labels | 8/12 = 0.667 | original exact-label threshold failed, preserved |
| continuation composed policy v1 | 4/4; zero false continue/pause | authored cases; unchanged frozen labels/questions |
| visual rubric v2, Sol medium | 1/1 rejected + 1/1 accepted correct | whole crown seed1 only; no bark/fresh-seed qualification |

The four original-rubric visual calls remain diagnostic, not model failure evidence: Sol and Astra both rejected the historical accepted birch under an underspecified naturalness rubric. The owner clarified recognizable species at established catalogue quality, not photorealism. V2 adds an independent shipped spruce finish anchor and separates blocking defects from optional polish. It correctly rejected beech and accepted birch. The spruce image is not a species-morphology reference or a fabricated per-view owner verdict.

Actual visual usage: 123,936 tokens across six successful passes. Jev usage: 16,000. Cumulative actual 139,936 plus conservative failed-infrastructure reservation 5,000 = 144,936 charged against the approved 200,000 ceiling. Cached-token breakdown and previous implementation/scout tokens are unknown. Two additional visual passes are authorized only for the bounded pilot, not model retries. Nothing was reset.

## Deterministic evidence

`env -u TYPESAFE_API_KEY cargo test -p telperion-jev` passed, exit 0, log `/tmp/fn68-stable-focused.log`. New focused groups: eight tuning contract tests, two engine iteration/routing tests, two CLI pause/recovery tests. Existing crate suites also ran. The host owns the final release workspace gate.

The engine mock performs repeated evaluate → assess → route cycles and proves numeric improvement cannot remove a visible defect. The baseline capability test was first red (wasted candidate evaluations) and then green after routing moved before proposal dispatch. Other checks cover bounds/integer choices, contextual numeric observations with passing gates, failed gates before rendering, stale/missing/unknown visual cells, shared calibration linkage, consumed confidence, persistent reservations and scoped changed-revision recovery.

R1–R4 and R9–R11 have deterministic implementation and focused coverage. R5 has the bounded calibration above. R8 has limited real whole-crown replay, not full readiness coverage. R7 live magnitude pilot/comparison is still unmet at this checkpoint. R6 relies on existing overlay/flag/caller tests plus current focused coverage; an explicit empty-overlay measurement golden and direct ask-command mock remain to be documented or added. This checkpoint is not task completion.

Tier: session (jev long_running 0.41), explicit IMPLEMENTER preserved. No implementation bridge or subagents were used in this continuation. stage: impl-review - skipped(config: REVIEW_MODE=none).
