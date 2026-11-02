"""fn-326: search a preset's /space/coarse rows on its tuning seeds.

Usage: search.py <preset> <name> <cpu> <firsts> <wide> <generations> <size> [from.log...]

A first pass of <wide> candidates is uniform in log space around fn-325's values
for each first age in <firsts> (comma-separated); with logs given, it is skipped.
Then the climb: each generation draws <size> around the five best, sigma 0.15 in
log space. Score, lower is better: 30 * max(0, worst - 0.04) + ln(seconds / 0.05),
so the statistics come first. Fidelity 1.0, the preset's age, seeds 1001 to 1005,
first pass on 1001 and 1002. Held-out seeds choose nothing. Logs given are read
for the first ages in <firsts> only, so a climb can leave an age it is stuck on.
"""
import math, os, random, subprocess, sys

EV = ".flow/evidence/fn-326-the-grower-takes-a-fidelity/raw/"
BIN = os.environ["FIDELITY_BIN"]
KEYS = ["first", "factor", "leaf", "pipe", "bud", "hazard", "shedding"]
WIDE = {"factor": (1.3, 5), "leaf": (2, 80), "pipe": (1.1, 6), "bud": (1, 3), "hazard": (1, 5), "shedding": (0.05, 1)}
BOUNDS = {"factor": (1.1, 8), "leaf": (1, 200), "pipe": (1, 10), "bud": (1, 5), "hazard": (1, 8), "shedding": (0.02, 1)}


def rows(path):
    out = []
    for line in open(path):
        i = line.find("CAND ")
        if i >= 0:
            w = line[i:].split()[1:]
            out.append({w[k]: float(w[k + 1]) for k in range(0, len(w) - 1, 2)})
    return out


def score(r):
    return 30 * max(0.0, r["worst"] - 0.04) + math.log(r["seconds"] / 0.05)


def run(preset, cpu, file, log, seeds):
    env = dict(os.environ, RAYON_NUM_THREADS="1", SEARCH_PRESET=preset, SEARCH_SEEDS=seeds, SEARCH_REPEATS="1", SEARCH_FILE=file)
    with open(log, "w") as f:
        subprocess.run(["telperion-heavy", "8", "--threads", "1", "--", "taskset", "-c", cpu, BIN, "fidelity_rows", "--ignored", "--nocapture", "--test-threads=1"], env=env, stdout=f, stderr=subprocess.STDOUT, check=True)
    return rows(log)


def main():
    preset, name, cpu, firsts, wide, gens, size = sys.argv[1], sys.argv[2], sys.argv[3], [int(x) for x in sys.argv[4].split(",")], int(sys.argv[5]), int(sys.argv[6]), int(sys.argv[7])
    rng = random.Random(326)
    pool = [r for p in sys.argv[8:] for r in rows(p) if int(r["first"]) in firsts]
    if not pool:
        file = f"{EV}{name}-wide.txt"
        with open(file, "w") as f:
            for _ in range(wide):
                v = [rng.choice(firsts)]
                for k in KEYS[1:]:
                    lo, hi = WIDE[k]
                    v.append(round(math.exp(rng.uniform(math.log(lo), math.log(hi))), 3))
                f.write(" ".join(map(str, v)) + " 1\n")
        pool = run(preset, cpu, file, f"{EV}{name}-wide.log", "1001,1002")
        print(f"wide: {len(pool)} read", flush=True)
    for g in range(gens):
        best = sorted(pool, key=score)[:5]
        file = f"{EV}{name}-{g}.txt"
        with open(file, "w") as f:
            for r in best:
                f.write(" ".join(str(r[k]) for k in KEYS) + " 1\n")
            for _ in range(size):
                r = rng.choice(best)
                v = [int(r["first"])]
                for k in KEYS[1:]:
                    lo, hi = BOUNDS[k]
                    v.append(round(min(hi, max(lo, r[k] * math.exp(rng.gauss(0, 0.15)))), 3))
                f.write(" ".join(map(str, v)) + " 1\n")
        seeds = ",".join(str(1001 + k) for k in range(int(os.environ.get("TUNE_SEEDS", "5"))))
        new = run(preset, cpu, file, f"{EV}{name}-{g}.log", seeds)
        pool = new if g == 0 else pool + new
        top = sorted(pool, key=score)[0]
        print(f"gen {g}: {len(new)} read, best " + " ".join(f"{k} {top[k]:.3f}" for k in top if k != "age"), flush=True)
    print("cheapest within each departure:")
    for lim in (0.05, 0.07, 0.1, 0.15, 0.25):
        for r in sorted((r for r in pool if r["worst"] <= lim), key=lambda r: r["seconds"])[:1]:
            print(f"  worst<={lim}: " + " ".join(f"{k} {r[k]:.3f}" for k in r if k != "age"))


main()
