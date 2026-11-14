#!/usr/bin/env python3
"""Visual judging eval: Jev, Claude Opus 5.5, Claude Sonnet 5.5 and Codex GPT-6 Astra on one blind item set.

Usage: visual_eval.py <images-dir> <out.json> [--smoke]
Images are copied to random names, so a file name cannot leak an answer.
"""
import base64, concurrent.futures as cf, json, os, random, re, shutil, subprocess, sys, tempfile, time, urllib.request, uuid

SRC, OUT = sys.argv[1], sys.argv[2]
SMOKE = "--smoke" in sys.argv
BLIND = tempfile.mkdtemp(prefix="visual-eval-")
random.seed(321)
NAMES = {}

def blind(name):
    if name not in NAMES:
        NAMES[name] = uuid.uuid4().hex[:12] + ".jpg"
        shutil.copy(f"{SRC}/{name}.jpg", f"{BLIND}/{NAMES[name]}")
    return NAMES[name]

LOOKS = "Which tree looks better to a person who cares about realistic trees: more natural and more convincing?"
FULL = "Which tree has the fuller, denser crown?"
SPECIES = ["oregon_white_oak", "norway_spruce", "silver_birch", "date_palm", "european_beech", "none_of_these"]
ITEMS = []

def pair(group, q, better, worse, why):
    for first, second, want in ((better, worse, "first"), (worse, better, "second")):
        ITEMS.append({"group": group, "kind": "pair", "q": q, "images": [first, second], "want": want, "why": why, "key": f"{better}|{worse}"})

def single(group, q, image, want, why=""):
    ITEMS.append({"group": group, "kind": "yesno", "q": q, "images": [image], "want": want, "why": why, "key": image + q[:12]})

# Owner's recorded words.
pair("owner", LOOKS, "oak-s7-new", "oak-s7-today", "owner: 'definitely better' (fn-236, fn-286)")
for s, new in (("s1", "f4-oregon-white-oak"), ("s7", "f4-oak-s7")):
    pair("owner", LOOKS, f"v014-oak-{s}", new, "owner 1 Nov 2026: 0.1.4 'looks better' than the new oak")
pair("owner", LOOKS, "f526-change", "f526-base", "owner: 'better' (fn-309)")
pair("owner", LOOKS, "f4-laurelin", "f4-telperion", "owner: 'Looks better than telperion' (2026-10-15)")
for other in ("f4-oregon-white-oak", "f4-european-beech", "f4-telperion", "f4-laurelin"):
    pair("owner", LOOKS, "f4-silver-birch", other, "owner on the birch: 'it's the best species' (2026-10-15)")
for approved in ("f4-date-palm", "f4-ordinary"):
    pair("owner", LOOKS, approved, "f4-oak-s7", "owner approved this; oak seed 7 'doesn't look like a real tree' (2026-10-15)")

# Defects the owner named, with negatives the host viewed.
Q_CLIP = "Do thick branches pass through one another or through the trunk, with no natural join?"
Q_GROUND = "Do long thin branches lie on the ground and snake along it?"
Q_WANDS = "Does the tree have long thin low limbs reaching far beyond its crown, with abrupt hooks or zigzag bends?"
Q_LEAVES = "Do leaves grow directly along thick limbs rather than from thin twigs?"
single("defect", Q_CLIP, "fork", "yes", "owner: 'branches clipping in each other'")
single("defect", Q_LEAVES, "fork", "yes", "owner: 'leaves growing ... on branches'")
single("defect", Q_GROUND, "birch-ground", "yes", "owner: 'ground branches'")
single("defect", Q_WANDS, "oak-s7-today", "yes", "owner: 'Lower branches that grow to long and zigzag weirdly'")
for image in ("photo-oak", "photo-spruce"):
    single("defect", Q_CLIP, image, "no", "photo")
    single("defect", Q_LEAVES, image, "no", "photo")
for image in ("f4-date-palm", "f4-telperion", "f4-norway-spruce", "photo-oak"):
    single("defect", Q_GROUND, image, "no", "host viewed")
for image in ("f4-date-palm", "f4-norway-spruce", "v014-oak-s7", "photo-oak"):
    single("defect", Q_WANDS, image, "no", "host viewed")

# Known facts.
for image, want in (("f4-oregon-white-oak", "oregon_white_oak"), ("v014-oak-s1", "oregon_white_oak"), ("photo-oak", "oregon_white_oak"),
                    ("f4-norway-spruce", "norway_spruce"), ("photo-spruce", "norway_spruce"), ("f4-silver-birch", "silver_birch"),
                    ("photo-birch-oslo", "silver_birch"), ("photo-birch-uusimaa", "silver_birch"), ("f4-date-palm", "date_palm"),
                    ("f4-european-beech", "european_beech")):
    ITEMS.append({"group": "species", "kind": "species", "q": "Which species does this tree look most like?", "images": [image], "want": want, "why": "known", "key": image})

# Clear differences, host viewed.
for fuller, thinner in (("v014-oak-s1", "f4-oregon-white-oak"), ("v014-oak-s7", "f4-oak-s7"), ("photo-oak", "f4-oregon-white-oak"),
                        ("photo-birch-oslo", "f4-silver-birch"), ("norway-spruce-s1-whole-f4", "norway-spruce-s1-whole-v014")):
    pair("fullness", FULL, fuller, thinner, "host viewed")

# Controls: identical images.
for image in ("oak-s7-new", "f4-silver-birch"):
    ITEMS.append({"group": "control", "kind": "pair", "q": LOOKS, "images": [image, image], "want": "no_difference", "why": "identical", "key": image})

# No truth: agreement between models, 0.1.4 against new, one random order each.
for sp in ("silver-birch", "norway-spruce"):
    for seed in ("s1", "s7", "s42"):
        for view in ("whole", "mid", "close"):
            a, b = f"{sp}-{seed}-{view}-v014", f"{sp}-{seed}-{view}-f4"
            if random.random() < 0.5:
                a, b = b, a
            ITEMS.append({"group": "agreement", "kind": "pair", "q": LOOKS, "images": [a, b], "want": None, "why": "no truth", "key": f"{sp}-{seed}-{view}"})

if SMOKE:
    random.shuffle(ITEMS)
    ITEMS = ITEMS[:4]

def options(item):
    if item["kind"] == "pair":
        return {"first": "The tree in the first image.", "second": "The tree in the second image.", "no_difference": "Neither is better; they are the same or level."}
    if item["kind"] == "yesno":
        return {"yes": "Yes.", "no": "No."}
    return {s: s.replace("_", " ") for s in SPECIES}

def text_prompt(item):
    opts = ", ".join(options(item))
    return (f"{item['q']} Judge from the image{'s' if len(item['images']) > 1 else ''} only. "
            f"Answer with exactly one of: {opts}. Reply with only JSON: "
            '{"answer": "<option>", "confidence": <probability from 0 to 1 that your answer is right>}')

def parse(text):
    m = re.search(r"\{.*\}", text or "", re.S)
    d = json.loads(m.group(0))
    return d["answer"], float(d["confidence"])

def jev(item):
    assets, state = {}, {}
    fields = ["first", "second"] if len(item["images"]) == 2 else ["still"]
    for field, name in zip(fields, item["images"]):
        key = str(uuid.uuid4())
        data = base64.b64encode(open(f"{BLIND}/{blind(name)}", "rb").read()).decode()
        assets[key] = {"type": "image", "format": "jpg", "data_base64": data}
        state[field] = key
    if len(set(item["images"])) == 1 and len(item["images"]) == 2:
        pass
    question = {"type": "choice", "instructions": item["q"], "criteria": options(item)}
    body = {"model": "jev-latest", "state": state, "questions": {"pick": question}, "assets": assets}
    req = urllib.request.Request("https://api.typesafe.ai/v1/systemone", json.dumps(body).encode(), {
        "Authorization": "Bearer " + os.environ["TYPESAFE_API_KEY"],
        "TypeSafe-Danger-Zone-Images": "true", "Content-Type": "application/json"})
    t = time.time()
    with urllib.request.urlopen(req, timeout=180) as r:
        out = json.load(r)
    a = out["answers"]["pick"]
    return {"answer": a["choice"], "p": a["probabilities"][a["choice"]], "s": time.time() - t,
            "input": out["usage"]["input_tokens"], "output": 0, "cached": 0}

def claude(model):
    def run(item):
        content = []
        for n in item["images"]:
            data = base64.b64encode(open(f"{BLIND}/{blind(n)}", "rb").read()).decode()
            content.append({"type": "image", "source": {"type": "base64", "media_type": "image/jpeg", "data": data}})
        content.append({"type": "text", "text": text_prompt(item)})
        msg = {"type": "user", "message": {"role": "user", "content": content}}
        t = time.time()
        r = subprocess.run(["claude", "-p", "--model", model, "--input-format", "stream-json", "--output-format", "stream-json", "--verbose",
                            "--no-session-persistence", "--setting-sources", "", "--system-prompt", "You judge images. Follow the output format exactly.",
                            "--tools", ""], input=json.dumps(msg) + "\n", capture_output=True, text=True, timeout=300, cwd=BLIND)
        d = [e for e in map(json.loads, r.stdout.splitlines()) if e.get("type") == "result"][0]
        u = d["usage"]
        ans, p = parse(d["result"])
        return {"answer": ans, "p": p, "s": time.time() - t, "harness_usd": d.get("total_cost_usd"),
                "input": u["input_tokens"] + u.get("cache_read_input_tokens", 0) + u.get("cache_creation_input_tokens", 0),
                "output": u["output_tokens"], "cached": u.get("cache_read_input_tokens", 0)}
    return run

def codex(model):
    def run(item):
        cmd = ["codex", "exec", "-s", "read-only", "--skip-git-repo-check", "--ephemeral", "-m", model, "--json", text_prompt(item)]
        for n in item["images"]:
            cmd += ["-i", f"{BLIND}/{blind(n)}"]
        t = time.time()
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=300, stdin=subprocess.DEVNULL, cwd=BLIND)
        text, usage = None, {}
        for line in r.stdout.splitlines():
            try:
                e = json.loads(line)
            except ValueError:
                continue
            if e.get("type") == "item.completed" and e["item"].get("type") == "agent_message":
                text = e["item"]["text"]
            if e.get("type") == "turn.completed":
                usage = e["usage"]
        ans, p = parse(text)
        return {"answer": ans, "p": p, "s": time.time() - t, "input": usage.get("input_tokens", 0),
                "output": usage.get("output_tokens", 0), "cached": usage.get("cached_input_tokens", 0)}
    return run

BASELINE = {"group": "baseline", "kind": "yesno", "q": "Is the sky usually blue on a clear day?", "images": [], "want": None, "why": "no image: harness overhead", "key": "baseline"}
RUNNERS = {"jev": (jev, 8), "opus-5.5": (claude("claude-opus-5-5"), 4), "sonnet-5.5": (claude("claude-sonnet-5-5"), 4), "astra-6": (codex("gpt-6-astra"), 4)}
log = {"items": ITEMS, "names": NAMES, "results": {}}

def guarded(fn, item):
    for attempt in range(2):
        try:
            return fn(item)
        except Exception as e:
            err = repr(e)[:300]
    return {"error": err}

for item in ITEMS:
    for n in item["images"]:
        blind(n)
for name, (fn, width) in RUNNERS.items():
    t = time.time()
    with cf.ThreadPoolExecutor(width) as pool:
        res = list(pool.map(lambda it: guarded(fn, it), ITEMS))
    base = guarded(fn, BASELINE) if name != "jev" else None
    log["results"][name] = {"answers": res, "baseline": base, "wall_s": time.time() - t}
    print(name, "done", round(time.time() - t), "s; errors", sum("error" in r for r in res), flush=True)
    with open(OUT, "w") as f:
        json.dump(log, f, indent=1)
shutil.rmtree(BLIND, ignore_errors=True)
print("wrote", OUT, len(ITEMS), "items")
