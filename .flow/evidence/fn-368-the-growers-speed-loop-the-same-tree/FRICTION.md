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

## 14 Nov 2026, 03:18, one-thread CPU seconds spread a quarter between runs

- Doing: the base round (`raw/r0`), three runs a tree at one thread.
- Hindered: the same binary on the same tree spread 1.00 to 1.63 CPU s (oak at 40), 19.98 to 25.51 (beech at 80), 47.3 to 52.4 (oak at 120), with no PAUSE inside: the desk was loaded by other sessions' jobs, and CPU seconds still follow the clock speed and the shared caches. A median of three cannot tell the 3 to 5% the candidates promise from that spread.
- Cost: unknown yet; every round's verdict is weaker for it.
- Would have removed it: a counted measure (instructions, from `perf stat`, not installed), or rounds measured on a quiet desk.

## 14 Nov 2026, 03:40, the preset digest pin is red on the base

- Doing: R3's tests after round 1.
- Hindered: `every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial` fails on the base (`d59085ecc`, the round reverted, `raw/r0/pin.log`) with the same read as round 1 (`raw/r1/pin.log`): every preset's digest differs from `PINNED`, so a change since the pin moved every tree without repinning it. R3's pin cannot be green; round 1 is held to it by its read being equal to the base's, entry for entry.
- Cost: about 6 minutes (a base build and run).
- Would have removed it: the pin repinned by the spec that moved it. Repinning is the host's call; not done here.
