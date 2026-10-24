"""fn-281 R3: every tree whose bytes the fix changes, from the base and
fixed sweeps' digests (`raw/base/`, `raw/fix/`), run from the worktree root:
    python3 -I .flow/evidence/fn-281-the-oak-at-80-builds-at-every-seed/compare.py
Prints one line a changed tree with the nodes removed, then the counts."""
import pathlib
import re

RAW = pathlib.Path(".flow/evidence/fn-281-the-oak-at-80-builds-at-every-seed/raw")
LINE = re.compile(r"^(\S+) age (\d+) seed (\d+): (ok|REFUSED) (.*)$")


def read(side):
    out = {}
    for log in sorted((RAW / side).glob("*.log")):
        for line in log.read_text().splitlines():
            m = LINE.match(line)
            if not m:
                continue
            key = (m[1], int(m[2]), int(m[3]))
            rest = m[5]
            nodes = re.search(r"(\d+) nodes", rest)
            digest = re.search(r"digest (\w+)", rest)
            fine = re.search(r"(\d+) under FINEST", rest)
            out[key] = (m[4], nodes and int(nodes[1]), digest and digest[1], rest, fine and int(fine[1]))
    return out


base, fix = read("base"), read("fix")
same = changed = other = 0
for key in sorted(base.keys() & fix.keys()):
    b, f = base[key], fix[key]
    if b[0] == f[0] == "ok" and b[2] == f[2]:
        same += 1
        continue
    if b[0] == f[0] == "REFUSED" and "budget" in b[3] and "budget" in f[3]:
        same += 1
        continue
    changed += 1
    removed = b[1] - f[1] if b[1] and f[1] else None
    # A change only where the base drew wood thinner than FINEST: the fix
    # removes exactly the nodes by which that count falls (a refused base
    # printed no count).
    expected = b[4] is None or (f[4] is not None and b[4] - f[4] == removed)
    other += 0 if expected else 1
    note = "" if expected else f" UNEXPECTED (base {b[4]} under FINEST, fix {f[4]})"
    left = f" ({f[4]} under FINEST left)" if f[4] else ""
    print(f"{key[0]} {key[1]} seed {key[2]}: base {b[0]} {b[1]}, fix {f[0]} {f[1]}, removed {removed}{left}{note}")
missing = len(base.keys() ^ fix.keys())
print(f"{same} trees unchanged, {changed} changed ({other} unexpected), {missing} in one sweep only")
