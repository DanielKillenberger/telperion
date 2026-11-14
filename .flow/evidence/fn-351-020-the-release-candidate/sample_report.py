"""fn-351 speed round 2: sample.py's samples read back: the share of the
samples by the physical function sampled (the outermost frame at the
address: what a symbol-level profile shows), by the innermost inlined
frame (where the instruction came from), and by every frame on the inline
chain (a function's own code wherever it was inlined).

    python3 sample_report.py <samples.tsv> [rows] [without]

`without` is a pattern: a sample with a frame matching it is left out and
counted apart (the digest's formatting, where a run was sampled whole).
"""
import collections
import re
import sys

rows = int(sys.argv[2]) if len(sys.argv) > 2 else 30
without = re.compile(sys.argv[3]) if len(sys.argv) > 3 else None
left_out = 0
outer, inner, anywhere = (collections.Counter() for _ in range(3))
total = 0


def short(name):
    """A demangled Rust path without its hash and generic arguments."""
    name = re.sub(r"::h[0-9a-f]{16}$", "", name)
    name = re.sub(r" \(\.llvm\.\d+\)$", "", name)
    return name if len(name) <= 96 else name[:93] + "..."


for line in open(sys.argv[1]):
    parts = line.rstrip("\n").split("\t")
    n, frames = int(parts[0]), parts[1:]
    if without and any(without.search(f) for f in frames):
        left_out += n
        continue
    total += n
    if len(frames) == 1:
        frames = [frames[0], frames[0]]
    names, where = [short(f) for f in frames[:-1]], frames[-1]
    names = [f"?? {where}" if f == "??" else f for f in names]
    outer[names[-1]] += n
    inner[names[0]] += n
    for name in set(names):
        anywhere[name] += n

if without:
    print(f"left out: {left_out} samples matching {without.pattern}; kept {total}")
for title, table in (
    ("by the physical function sampled", outer),
    ("by the innermost inlined frame", inner),
    ("by any frame on the inline chain", anywhere),
):
    print(f"\n{title} ({total} samples)")
    for name, n in table.most_common(rows):
        print(f"  {100 * n / total:5.1f}%  {name}")
