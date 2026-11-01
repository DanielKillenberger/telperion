"""fn-325: a hill-climb on the tuning seeds. Usage: climb.py <round> <generations> <size> <sigma> <seed> <from.log>...

Each generation draws `size` candidates around the five best so far, Gaussian in
log space, and grows them through the probe. Score, lower is better:
10 * max(0, worst - 0.04) + ln(seconds / 0.05), so the statistics come first and
cost second, and past 4% only cost counts.
"""
import math, os, random, re, subprocess, sys

EV = ".flow/evidence/fn-325-tune-the-oaks-fine-wood-dial-to-hold/raw/"
BIN = "target/ci/deps/telperion_core-74dc39aa8e476c54"
KEYS = ["first", "s", "leaf", "pipe", "bud", "hazard", "s2", "x"]
BOUNDS = {"s": (1.2, 8), "leaf": (1, 48), "pipe": (1, 8), "bud": (1, 4), "hazard": (1, 8), "s2": (1, 3), "x": (0.03, 1)}


def rows(path):
    out = []
    for line in open(path):
        i = line.find("CAND ")
        if i >= 0:
            w = line[i:].split()[1:]
            out.append({w[k]: float(w[k + 1]) for k in range(0, len(w) - 1, 2)})
    return out


def score(r):
    return float(os.environ.get("CLIMB_WEIGHT", "10")) * max(0.0, r["worst"] - 0.04) + math.log(r["seconds"] / 0.05)


def main():
    name, gens, size, sigma, seed = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), float(sys.argv[4]), int(sys.argv[5])
    rng = random.Random(seed)
    pool = [r for p in sys.argv[6:] for r in rows(p) if "s2" in r]
    seeds = os.environ.get("CLIMB_SEEDS", "1001,1002,1003,1004,1005")
    for g in range(gens):
        best = sorted(pool, key=score)[:5]
        file = f"{EV}{name}-{g}.txt"
        with open(file, "w") as f:
            for r in best:
                f.write(" ".join(str(r[k]) for k in KEYS) + "\n")
            for _ in range(size):
                r = rng.choice(best)
                v = [rng.choice([7, 8, 9]) if os.environ.get("CLIMB_FIRST") and rng.random() < 0.3 else int(r["first"])]
                for k in KEYS[1:]:
                    lo, hi = BOUNDS[k]
                    v.append(round(min(hi, max(lo, r[k] * math.exp(rng.gauss(0, sigma)))), 3))
                f.write(" ".join(map(str, v)) + "\n")
        log = f"{EV}{name}-{g}.log"
        env = dict(os.environ, RAYON_NUM_THREADS="1", SEARCH_AGE="40", SEARCH_SEEDS=seeds, SEARCH_REPEATS="1", SEARCH_FILE=file)
        with open(log, "w") as f:
            subprocess.run(["telperion-heavy", "8", "--threads", "1", "--", "taskset", "-c", "2", BIN, "coarse_list", "--ignored", "--nocapture", "--test-threads=1"], env=env, stdout=f, stderr=subprocess.STDOUT, check=True)
        new = rows(log)
        # The seeds changed from earlier rounds: the pool restarts on this round's own readings.
        pool = new if g == 0 else pool + new
        top = sorted(pool, key=score)[0]
        print(f"gen {g}: {len(new)} grown, best " + " ".join(f"{k} {top[k]:.3f}" for k in KEYS[1:] + ["height", "spread", "trunk", "limbs", "shed", "area", "worst", "seconds"]), flush=True)
    print("cheapest within each departure:")
    for lim in (0.05, 0.07, 0.1, 0.15):
        fit = sorted((r for r in pool if r["worst"] <= lim), key=lambda r: r["seconds"])[:2]
        for r in fit:
            print(f"  worst<={lim}: " + " ".join(f"{k} {r[k]:.3f}" for k in KEYS[1:] + ["height", "spread", "trunk", "limbs", "shed", "area", "worst", "seconds"]))


main()
