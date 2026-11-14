"""0.1.4 against the new engine, birch and spruce: which reads more real, and which is closer to a reference photo."""
import base64, json, os, sys, urllib.request, uuid

S = "/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/649ff7cf-1483-484f-a9ef-853526f3a69b/scratchpad/jev-visual/"
OUT = sys.argv[1]

def ask(state, question):
    assets, st = {}, {}
    for field, name in state.items():
        key = str(uuid.uuid4())
        data = base64.b64encode(open(S + name + ".jpg", "rb").read()).decode()
        assets[key] = {"type": "image", "format": "jpg", "data_base64": data}
        st[field] = key
    body = {"model": "jev-latest", "state": st, "questions": {"pick": question}, "assets": assets}
    req = urllib.request.Request("https://api.typesafe.ai/v1/systemone", json.dumps(body).encode(), {
        "Authorization": "Bearer " + os.environ["TYPESAFE_API_KEY"],
        "TypeSafe-Danger-Zone-Images": "true", "Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=180) as r:
        return json.load(r)["answers"]["pick"]["probabilities"]

CRIT = {"first": "The tree in `first`.", "second": "The tree in `second`.", "no_difference": "Neither."}
REAL = {"type": "choice", "instructions": "Which tree looks more like a real living tree?", "criteria": CRIT}
BETTER = {"type": "choice", "instructions": "Which tree looks better: more natural and more pleasing to a person looking at it?", "criteria": CRIT}
REF = {"type": "choice", "instructions": "`reference` is a photograph of the real species. Which tree's overall crown form and branching is closer to it?", "criteria": CRIT}

def both(question, a, b, extra=None):
    """Probability a wins, b wins, averaged over both orders, plus the order gap."""
    extra = extra or {}
    p = ask({**extra, "first": a, "second": b}, question)
    q = ask({**extra, "first": b, "second": a}, question)
    return {"v014": round((p["first"] + q["second"]) / 2, 3), "new": round((p["second"] + q["first"]) / 2, 3),
            "same": round((p["no_difference"] + q["no_difference"]) / 2, 3), "order_gap": round(abs(p["first"] - q["second"]), 3)}

log = []
for sp in ["silver-birch", "norway-spruce"]:
    for seed in ["s1", "s7", "s42"]:
        for view in ["whole", "mid", "close"]:
            a, b = f"{sp}-{seed}-{view}-v014", f"{sp}-{seed}-{view}-f4"
            for label, q in (("real", REAL), ("better", BETTER)):
                r = both(q, a, b)
                log.append({"species": sp, "seed": seed, "view": view, "q": label, **r})
                print(sp, seed, view, label, r, flush=True)
        a, b = f"{sp}-{seed}-whole-v014", f"{sp}-{seed}-whole-f4"
        refs = ["photo-birch-oslo", "photo-birch-uusimaa"] if sp == "silver-birch" else ["photo-spruce"]
        for ref in refs:
            r = both(REF, a, b, {"reference": ref})
            log.append({"species": sp, "seed": seed, "view": "whole", "q": "ref:" + ref, **r})
            print(sp, seed, "ref", ref, r, flush=True)
r = both(REAL, "silver-birch-s1-whole-v014", "silver-birch-s1-whole-f4age70")
log.append({"species": "silver-birch", "seed": "s1", "view": "whole-age70", "q": "real", **r})
print("birch age70", r)
with open(OUT, "w") as f:
    json.dump(log, f, indent=1)
print("wrote", OUT)
