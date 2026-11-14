"""fn-301 .5: compare the R1-row digests, base against change, and judge
each row against the host's list of rows expected to change."""
import sys

def load(path):
    rows = {}
    for line in open(path):
        parts = line.rstrip("\n").split("\t")
        if len(parts) < 9:
            continue
        key = (parts[0], float(parts[1]), int(parts[2]))
        rows[key] = dict(p.split(" ", 1) for p in parts[3:])
    return rows

def expected(preset, age, seed):
    if preset == "date-palm":
        return True
    if preset == "silver-birch":
        return True
    if preset == "norway-spruce" and age == 40:
        return True
    if preset in ("ordinary", "oregon-white-oak") and age == 40 and seed == 42:
        return True
    return False

base, change = load(sys.argv[1]), load(sys.argv[2])
assert base.keys() == change.keys(), (len(base), len(change))
unexpected, held_bad = [], []
print("| Preset | Age | Seed | Expected | Changed artifacts |")
print("|---|--:|--:|---|---|")
for key in base:
    b, c = base[key], change[key]
    moved = [k for k in b if b[k] != c[k]]
    exp = expected(*key)
    print(f"| {key[0]} | {key[1]:g} | {key[2]} | {'change' if exp else 'hold'} | {', '.join(moved) or 'identical'} |")
    if moved and not exp:
        unexpected.append((key, moved))
print()
print(f"rows {len(base)}; unexpected changes {len(unexpected)}")
for key, moved in unexpected:
    print("UNEXPECTED", key, moved, base[key], change[key])
