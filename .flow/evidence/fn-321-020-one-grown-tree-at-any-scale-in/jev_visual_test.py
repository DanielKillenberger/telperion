"""Quick test: does Jev's image input agree with the owner's recorded verdicts on stills?"""
import base64, json, os, sys, time, urllib.error, urllib.request, uuid

S = "/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/649ff7cf-1483-484f-a9ef-853526f3a69b/scratchpad/jev-visual/"
OUT = sys.argv[1]
URL = "https://api.typesafe.ai/v1/systemone"

def call(model, state, questions, images):
    assets = {}
    def put(name):
        key = str(uuid.uuid4())
        data = base64.b64encode(open(S + name + ".jpg", "rb").read()).decode()
        assets[key] = {"type": "image", "format": "jpg", "data_base64": data}
        return key
    keys = {n: put(n) for n in images}
    def sub(v):
        if isinstance(v, str):
            for n, k in keys.items():
                v = v.replace("@" + n + "@", k)
            return v
        if isinstance(v, dict):
            return {a: sub(b) for a, b in v.items()}
        return v
    body = {"model": model, "state": sub(state), "questions": sub(questions), "assets": assets}
    req = urllib.request.Request(URL, json.dumps(body).encode(), {
        "Authorization": "Bearer " + os.environ["TYPESAFE_API_KEY"],
        "TypeSafe-Danger-Zone-Images": "true",
        "Content-Type": "application/json"})
    t = time.time()
    try:
        with urllib.request.urlopen(req, timeout=180) as r:
            out = json.load(r)
    except urllib.error.HTTPError as e:
        return {"error": e.code, "detail": e.read().decode()[:300]}
    out["elapsed_s"] = round(time.time() - t, 2)
    return out

DEFECTS = {
    "clip": {"type": "noul", "instructions": "Thick branches in `still` pass through one another or through the trunk, with no natural join where they meet."},
    "leaves_on_limbs": {"type": "noul", "instructions": "Leaves in `still` grow directly along thick limbs rather than from thin twigs."},
    "on_ground": {"type": "noul", "instructions": "Long thin branches in `still` lie on the ground and snake along it."},
    "wands": {"type": "noul", "instructions": "The tree in `still` has long thin low limbs that reach far beyond its crown, with abrupt hooks or zigzag bends."},
    "realism": {"type": "score", "instructions": "How much does the tree in `still` look like a real living tree?", "criteria": [
        "Plainly not a real tree: its form is broken or impossible.",
        "Tree-like, with obvious faults a casual viewer would notice.",
        "Convincing at a glance, with faults on inspection.",
        "Could pass as a real tree under close inspection."]},
    "photo": {"type": "noul", "instructions": "`still` is a photograph of a real tree, not a computer rendering."},
}
SINGLES = ["fork", "birch-ground", "oak-s7-today", "oak-s7-new", "oak-s1-today", "oak-s1-new", "f4-oak-s7", "v014-oak-s1",
           "v014-oak-s7", "photo-oak", "photo-spruce", "f4-oregon-white-oak", "f4-norway-spruce", "f4-silver-birch",
           "f4-date-palm", "f4-ordinary", "f4-telperion", "f4-laurelin", "f4-european-beech"]
PAIR_QUESTION = {"type": "choice", "instructions": "Which tree looks more like a real living tree?",
                 "criteria": {"first": "The tree in `first`.", "second": "The tree in `second`.", "no_difference": "Neither looks more real than the other."}}
REF_QUESTION = {"type": "choice", "instructions": "`reference` is a photograph of the real species. Which tree's overall crown form is closer to it?",
                "criteria": {"first": "The tree in `first`.", "second": "The tree in `second`.", "no_difference": "Neither is closer."}}
PAIRS = [("oak-s7-new", "oak-s7-today", "owner: new is better"), ("oak-s7-new", "oak-s7-new", "control: identical")]
REFS = [("v014-oak-s1", "f4-oregon-white-oak", "owner: the 0.1.4 oak is the broad closed dome of the reference"),
        ("v014-oak-s7", "f4-oak-s7", "same, seed 7")]
log = []
models = sys.argv[2:]
for model in models:
    for name in SINGLES:
        r = call(model, {"still": "@%s@" % name}, DEFECTS, [name])
        log.append({"test": "single", "model": model, "image": name, "result": r})
        print(model, name, "ok" if "answers" in r else r, flush=True)
    for runs, (a, b, why) in enumerate(PAIRS):
        for x, y in ((a, b), (b, a)):
            r = call(model, {"first": "@%s@" % x, "second": "@%s@" % (y if x != y else y)}, {"pick": PAIR_QUESTION}, [x] if x == y else [x, y])
            log.append({"test": "pair", "model": model, "first": x, "second": y, "label": why, "result": r})
            print(model, x, y, "ok" if "answers" in r else r, flush=True)
    for a, b, why in REFS:
        for x, y in ((a, b), (b, a)):
            r = call(model, {"reference": "@photo-oak@", "first": "@%s@" % x, "second": "@%s@" % y}, {"pick": REF_QUESTION}, ["photo-oak", x, y])
            log.append({"test": "reference", "model": model, "first": x, "second": y, "label": why, "result": r})
            print(model, x, y, "ok" if "answers" in r else r, flush=True)
    for i in range(2):
        r = call(model, {"still": "@oak-s7-today@"}, DEFECTS, ["oak-s7-today"])
        log.append({"test": "repeat", "model": model, "image": "oak-s7-today", "result": r})
with open(OUT, "w") as f:
    json.dump(log, f, indent=1)
print("wrote", OUT)
