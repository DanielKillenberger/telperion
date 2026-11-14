# fn-368 friction

## 14 Nov 2026, 02:40, the measurement's CPU seconds rounded to whole seconds

- Doing: setting up the round measurement the spec names (`scratchpad/peak.py`).
- Hindered: `peak.py` prints CPU_S rounded to a whole second, and the oak at 40 grows in about one CPU second, so a 10% round on it cannot be read.
- Cost: about 3 minutes, a copy with hundredths (`peak.py` beside this file).
- Would have removed it: `peak.py` printing CPU and wall seconds to the hundredth.

## 14 Nov 2026, 02:43, the queue held the base runs for want of memory room

- Doing: the base measurement (`measure.sh r0-base`), twelve one-thread runs.
- Hindered: `telperion-heavy 8` waited at "room 0 GB" to "room 7 GB" for 8 GB under `TELPERION_HEAVY_RESERVE=8`, for minutes, while the oak at 40 needs well under 1 GB; the build before it waited too.
- Cost: about 20 minutes (02:43 to 03:02). Two other sessions' jobs (12 and 16 GB claims, a held-run oak at 300 and the fn-352 scoreboard) were admitted one after another while both of this spec's 8 GB jobs waited at "room 1 GB"; nothing in the queue orders waiters, so a large claim can wait indefinitely behind a stream of others.
- Done about it: the small trees now declare 2 GB (their peak is 0.15 GB), the oak at 120 keeps 8 (`measure.sh`'s rows). The dispatch named `telperion-heavy 8`; the admission rule itself is unchanged.
- Would have removed it: waiters admitted in order of arrival, or each run's claim taken from its measured peak.

## 14 Nov 2026, 03:10, the digest I added to `early` cost the beech 8 CPU seconds

- Doing: reading the base runs against the per-pass report.
- Hindered: the beech at 80 took 35.3 CPU s for a 27.4 s grow; the gap was `tree_digest`, which formats every phytomer's Debug text (4.5 M phytomers). Every CPU_S of the base runs taken so far carried it.
- Cost: about 10 minutes and the base runs so far, retaken.
- Removed it: `early` prints the process's CPU seconds (`/proc/self/stat`) once the tree is grown, before the digest; `summary.py` reads that.
