# fn-34 round 23 trials: the beech against the owner's round-22 notes

The owner's round-22 beech verdict was "not yet", in four notes. Jev
(TypeSafe, `jev-latest`) chose, for each note and each of fourteen beech
rows, whether raising or lowering the row answers the note, or neither
(`jev-note-mapping.json`, from `map_notes.py`). It picked from options code
gave it and wrote no value. Every move it put at 0.85 or above went into the
trials.

`trial.py` sets beech rows on the integration branch, renders the matched
stills, measures them with `scripts/compare-references.py`, and restores the
preset. `trial-base.json` reproduces round 22 exactly.

| Trial | Box filled (B-WHOLE, photo 0.457) | Centre (photo 79.7) | Outline (photo 0.227) | Shoulder band |
|---|---|---|---|---|
| base (round 22) | 0.532 | 56.4 | 0.170 | 0.912 |
| A: every move, outline at lobe scale 0.7 | 0.511 | 64.7 | 0.164 | 0.911 |
| B: outline at 0.45 / 0.35, full thinning | 0.474 | 64.4 | 0.171 | 0.818 |
| C: B's outline, milder thinning | 0.494 | 56.8 | 0.161 | 0.845 |

The outline metric barely moves, but B's and C's silhouettes are visibly
irregular where A's is a smooth dome: the fitted-ellipse residual does not
see small side lumps. The photograph's per-band density cannot be read with
the compare script's dark-pixel rule (sunlit leaves read as empty, dark
buildings behind B-BARE as tree), so the bands are the render's own.

B and C each pass all forty-eight protocol cases, none capped; the heaviest
beech seed is 138,621 nodes (B) and 182,859 (C), and seed 1's DBH proxy rises
from 0.889 m to 1.010 m. Both are on the judging page as round 23 A (C) and
B (B); the preset changes once the owner picks.
