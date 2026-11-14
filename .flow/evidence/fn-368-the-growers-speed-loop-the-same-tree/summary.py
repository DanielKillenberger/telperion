"""fn-368: one round's raw runs read back (measure.sh's raw/<label>/).

Per tree: the median CPU seconds of its runs to the grown tree, the most peak memory, the
digests (one, or every one that differs), and per-pass seconds from the
runs with no watcher PAUSE inside, where the yield log is given.

    python3 summary.py <raw/label> [yield.log]
"""
import pathlib
import re
import statistics
import sys
from datetime import datetime

raw = pathlib.Path(sys.argv[1])
log = pathlib.Path(sys.argv[2]) if len(sys.argv) > 2 else None


def stamp(text):
    return datetime.strptime(text, "%Y-%m-%d %H:%M:%S")


pauses = []
if log and log.exists():
    events = []
    for line in log.read_text().splitlines():
        m = re.match(r"(\S+ \S+) (PAUSE|RESUME)", line)
        if m:
            events.append((stamp(m.group(1)), m.group(2)))
    start = None
    for t, kind in events:
        if kind == "PAUSE" and start is None:
            start = t
        elif kind == "RESUME" and start is not None:
            pauses.append((start, t))
            start = None
    if start is not None:
        pauses.append((start, datetime.max))

spans = {}
for line in (raw / "STATUS").read_text().splitlines():
    m = re.match(r"(start|end rc=\d+) (\S+) (\d+) (\d+) (t\d+) (\d) (\S+ \S+)", line)
    if m:
        key = f"{m.group(2)}-{m.group(3)}-{m.group(4)}-{m.group(5)}-{m.group(6)}"
        spans.setdefault(key, [None, None])[0 if m.group(1) == "start" else 1] = stamp(m.group(7))


def paused(key):
    a, b = spans.get(key, [None, None])
    if a is None or b is None:
        return True
    return any(p < b and q > a for p, q in pauses)


trees = {}
for f in sorted(raw.glob("*-t*-?.txt")):
    text = f.read_text()
    # The grow's CPU seconds, read before the digest (`early`'s "cpu:");
    # the process's CPU_S where an older run has none.
    cpu = re.search(r"  cpu: ([\d.]+) s", text) or re.search(r"CPU_S ([\d.]+)", text)
    peak = re.search(r"PEAK_MIB (\d+)", text)
    digest = re.search(r"digest: (\d+)", text)
    phases = re.search(r"phases \(s\): (.*)", text)
    name = f.stem.rsplit("-", 1)[0]
    t = trees.setdefault(name, {"cpu": [], "peak": [], "digest": set(), "phases": []})
    if cpu:
        t["cpu"].append(float(cpu.group(1)))
    if peak:
        t["peak"].append(int(peak.group(1)))
    t["digest"].add(digest.group(1) if digest else "none")
    if phases and not paused(f.stem):
        t["phases"].append(phases.group(1))

for name, t in trees.items():
    cpu = statistics.median(t["cpu"]) if t["cpu"] else float("nan")
    print(f"{name}: CPU_S median {cpu:.2f} of {t['cpu']}, peak {max(t['peak'], default=0)} MiB, digest {sorted(t['digest'])}")
    if t["phases"]:
        print(f"  phases (no pause): {t['phases'][0][:200]}")
