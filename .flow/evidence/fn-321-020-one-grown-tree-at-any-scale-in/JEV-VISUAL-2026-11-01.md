# Jev's image input on tree stills, 1 Nov 2026

Daniel asked for a quick test of whether Jev's visual model helps judge visuals. It helps with narrow screening and does not stand in for his eye. This is a small sample: 19 stills, one wording per question, and labels taken from recorded verdicts where they exist. `jev-latest` and `jev-preview` both answered as jev-1.14.0, so the second run counts as a repeat, not a second model. TypeSafe calls image input "Dreamer-ready", not production-ready.

Script: `jev_visual_test.py`. Answers: ignored `raw/jev-visual-2026-11-01.json`. The shared caller cannot send images, so these calls have no ledger entry in the usual form. Crops came from fn-286's and fn-259's sheets and were viewed before sending.

## What worked

| Check | Result (jev-latest) |
|---|---|
| Photograph or render | Photos 0.94-0.95; every render 0.02-0.06 |
| Realism, photos against renders (scale 0-3) | Photos 2.66-2.69; renders 0.44-1.24 |
| Closer to the oak reference photo, 0.1.4 or F4 | 0.1.4 at both seeds, in both orders, 0.78-0.87 |
| Fixed seed 7 against the old one (owner: "definitely better") | The fixed one in both orders, but weakly: 0.46 to 0.33 and 0.50 to 0.19 |
| Identical pair (control) | No difference, 0.97-0.98 |
| Birch branches on the ground (owner's still) | 0.93 |
| Long low wands on old seed 7 (owner's complaint) | 0.75, against 0.51 once fixed |
| Repeat of one still | Within 0.02 |

Each request took a median 0.23 s and about 1,800 input tokens.

## What did not

- **Realism among renders does not follow the owner.** The birch ("the best species") scores lowest of the shipped species' stills at 0.77 and the spruce ("looks really good") 0.81. The F4 oak ("needs work") scores highest at 1.24.
- **Branch clipping does not separate renders.** Every render scores 0.55-0.78. The owner's still with clipping scores 0.73, no higher than the rest.
- **Leaves on limbs is missed.** The owner's still scores 0.41, under the palm's 0.48.
- **False alarms.** Branches on the ground reads 0.72 on the F4 oak and 0.71 on the F4 birch (the ground shadows are a possible cause, unverified). Wands read 0.74 on the beech (unverified, not looked at for this).

## 0.1.4 against the new engine, birch and spruce

Daniel: "tbf i think the .1.4 looks "better" than the new oak. Make it compare the birch or the spruce". Script: `jev_visual_compare.py`. Answers: ignored `raw/jev-visual-compare-2026-11-01.json`. `jev-latest`, fn-259's sheets at seeds 1, 7 and 42 at each view, probabilities averaged over both orders. The birch references are Oslo and Uusimaa photos from `.flow/references/silver-birch`; the spruce's is `piab977.jpg`, which shows two views side by side.

| | Looks more real, 9 views | Looks better, 9 views | Closer to the reference photo |
|---|---|---|---|
| Spruce | New 9 of 9 (0.68-0.85) | New 9 of 9 (0.76-0.96) | New 3 of 3 (0.70-0.75) |
| Birch | 0.1.4 7 of 9, 2 about level | 0.1.4 6 of 9, new 3 of 9 | New 4 of 6, 0.1.4 2 of 6, with 2 of the 6 within 0.1 |

The birch's whole view goes to 0.1.4 at every seed (0.62-0.86), and still does with the new birch at age 70 (0.65 to 0.27). Its close view at seed 1 is a poor comparison, since the 0.1.4 crop is mostly a flat trunk.

Limits: birch whole-view answers move with order, by up to 0.43, so they are less firm than the spruce's. These are Jev's answers, with no owner verdict on these pairs. Its realism score ran against the owner's verdicts in the first test.

Host's reading (inferred, from the montage and these answers): fullness may drive these preferences. 0.1.4's oak and birch have fuller crowns than the new ones, as do the reference photos, while the new spruce is the fuller and more solid of its pair.

## Eval: Jev against Claude and Codex

Daniel asked how well it judges, how fast and at what cost, against Claude 5.5 and Codex Astra 6. One run of 78 items, each model given the same question, with images under random file names. `visual_eval.py` runs it, `visual_eval_score.py` and `visual_eval_owner.py` score it, and the answers are in ignored `raw/visual-eval-2026-11-01.json`. Jev ran as `jev-1.14.0`, Claude through `claude -p` with image blocks, Astra through `codex exec` at its configured medium effort.

| | Jev 1.14 | Sonnet 5.5 | Opus 5.5 | Astra 6 |
|---|---|---|---|---|
| Owner's verdicts, 22 answers (11 pairs, both orders) | 8 | 6 | 12 | 16 |
| The same without the 4 birch "best species" pairs, 14 answers | 7 | 5 | 8 | 13 |
| Pairs right in both orders, of 11 | 2 | 2 | 5 | 7 |
| Facts, 38 answers (named defects, species, fullness, identical pairs) | 34 | 34 | 35 | 33 |
| Pairs that keep the same pick when swapped, of 16 | 12 | 14 | 14 | 14 |
| Median seconds a call | 0.24 | 2.24 | 1.84 | 4.39 |
| Cost a call | $0.00012 | $0.0081 | $0.0157 | $0.0120 |
| Cost of 1,000 calls | $0.12 | $8.11 | $15.73 | $12.01 |

Costs: Jev at $0.042 per Mtok of input. Claude as reported by the CLI, which includes the cache-write premium. Astra at list ($10 and $50 per Mtok) after subtracting the Codex harness's 21,280-token baseline; with that overhead it is about $0.10 a call. Sonnet's list price is $2 and $10 per Mtok and Opus's $4 and $20.

**Jev's probability means opposite things by question type.**

| Jev's top probability | Facts | Owner's verdicts |
|---|---|---|
| Under 0.7 | 4 of 7 | 7 of 13 |
| 0.7 to 0.9 | 7 of 8 | 1 of 7 |
| 0.9 and over | 23 of 23 | 0 of 2 |

On taste, Jev is most wrong when most sure, so escalating when it is unsure does not fix it: with Astra as the fallback at 0.9, the owner's group reaches 15 of 22 with 20 of 22 escalated, under Astra alone at 16.

Limits: one run and small groups. The 11 owner pairs are thin, and 4 of them stretch one sentence about the birch into comparisons across species. The facts group is easy: all four scored 16 of 16 on defects because the stills an earlier test gave false alarms on could not be labelled and were left out, so this does not show false alarms are gone. Fullness labels and defect negatives are the host's, from viewing the stills.

## Use

Host's recommendation, not approved. Route by question type. Jev answers factual checks (a named defect, what a still shows, fullness), trusted at 0.9 and over, with the rest going to a strong model. "Which looks better" goes straight to Astra, as a proposal to order sheets. The owner's verdict stays the only pass. `docs/typesafe.md` holds the rule (owner, 2026-11-01).
