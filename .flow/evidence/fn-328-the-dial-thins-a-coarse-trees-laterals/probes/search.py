"""fn-328: search a preset's /space/coarse rows with the laterals free to thin.

Usage: search.py <preset> <name> <cpu> <first> <wide> <generations> <size>, env FIDELITY_BIN.

A wide pass of <wide> candidates, uniform in log space inside WIDE, at one first
age, on seeds 1001 to 1004. Then the climb on seeds 1001 to 1012: each generation
draws <size> around the five best, sigma 0.15 in log space, inside BOUNDS. Score,
lower is better: 30 * max(0, worst - 0.04) + ln(seconds / 0.05). Fidelity 1, the
preset's age. Held-out seeds and the goal's R4 seeds choose nothing.
"""
import math, os, random, subprocess, sys

EV = ".flow/evidence/fn-328-the-dial-thins-a-coarse-trees-laterals/raw/"
BIN = os.environ["FIDELITY_BIN"]
KEYS = ["first", "factor", "leaf", "pipe", "bud", "hazard", "shedding"]
WIDE = {"factor": (1.1, 5), "leaf": (2, 300), "pipe": (1, 6), "bud": (0.1, 1.2), "hazard": (1, 5), "shedding": (0.05, 1)}
BOUNDS = {"factor": (1.0, 8), "leaf": (1, 600), "pipe": (1, 12), "bud": (0.03, 3), "hazard": (1, 8), "shedding": (0.02, 1)}


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
        subprocess.run(["telperion-heavy", os.environ.get("SEARCH_CLAIM", "2"), "--threads", "1", "--", "taskset", "-c", cpu, BIN, "fidelity_rows", "--ignored", "--nocapture", "--test-threads=1"], env=env, stdout=f, stderr=subprocess.STDOUT, check=True)
    return rows(log)


def main():
    preset, name, cpu, first, wide, gens, size = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4]), int(sys.argv[5]), int(sys.argv[6]), int(sys.argv[7])
    rng = random.Random(328 + first + 1000 * int(os.environ.get("SEARCH_ROUND", "0")))
    start = os.environ.get("SEARCH_START")
    if start:  # An earlier round's logs stand in for the wide pass, moved to this first age.
        pool = [dict(r, first=float(first)) for p in start.split(",") for r in rows(p)]
    else:
        file = f"{EV}{name}-wide.txt"
        with open(file, "w") as f:
            for _ in range(wide):
                v = [first] + [round(math.exp(rng.uniform(math.log(WIDE[k][0]), math.log(WIDE[k][1]))), 3) for k in KEYS[1:]]
                f.write(" ".join(map(str, v)) + " 1\n")
        pool = run(preset, cpu, file, f"{EV}{name}-wide.log", "1001,1002,1003,1004")
    print(f"start: {len(pool)} rows", flush=True)
    seeds = ",".join(str(1001 + k) for k in range(12))
    for g in range(gens):
        best = sorted(pool, key=score)[:5]
        file = f"{EV}{name}-{g}.txt"
        with open(file, "w") as f:
            for r in best:
                f.write(" ".join(str(r[k]) for k in KEYS) + " 1\n")
            for _ in range(size):
                r = rng.choice(best)
                v = [first]
                for k in KEYS[1:]:
                    lo, hi = BOUNDS[k]
                    v.append(round(min(hi, max(lo, r[k] * math.exp(rng.gauss(0, 0.15)))), 3))
                f.write(" ".join(map(str, v)) + " 1\n")
        new = run(preset, cpu, file, f"{EV}{name}-{g}.log", seeds)
        pool = new if g == 0 else pool + new
        top = sorted(pool, key=score)[0]
        print(f"gen {g}: {len(new)} read, best " + " ".join(f"{k} {top[k]:.3f}" for k in top if k != "age"), flush=True)
    print("cheapest within each departure:")
    for lim in (0.05, 0.07, 0.1, 0.15, 0.25):
        for r in sorted((r for r in pool if r["worst"] <= lim), key=lambda r: r["seconds"])[:1]:
            print(f"  worst<={lim}: " + " ".join(f"{k} {r[k]:.3f}" for k in r if k != "age"))


main()
