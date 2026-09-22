#!/usr/bin/env python3
"""Evidence retention for fn-68: code lists every tracked evidence file with
its facts, Jev labels each one (keep labels beside drop labels), and code
applies only the drops the rules allow. Usage:
  retention-classify.py extract  > candidates.json
  retention-classify.py ask candidates.json LEDGER_DIR > labels.json
  retention-classify.py apply candidates.json labels.json   (moves to raw/, writes manifest)
"""
import json, os, re, subprocess, sys, hashlib
E = ".flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev"
KEEP_NAMES = {"REPORT.md", "FRICTION.md", "FRICTION-REVIEW.md", "final-run-RESULT.md", "PROTOCOL.md",
              "contact-sheet-protocol.md", "progress-review-protocol.md", "reference-first-protocol.md",
              "retention-classify.py", "RETENTION.md", "retention-manifest.json"}
CHOICES = {
    "essential_record": "a final result, an owner decision or verdict, a run note or a report that later reading depends on",
    "reusable_source": "a script, harness, fixture, manifest or receipt that a rerun or a replay needs",
    "superseded_intermediate": "an earlier version of a later artifact, or a resume, preflight, config or draft snapshot whose outcome a retained file records",
    "redundant_duplicate": "the same content as another retained file in this set",
    "raw_low_info": "run output, per-call journals, generated candidates or bookkeeping that a summary already captures",
    "unsure": "cannot tell from what is shown",
}
DROP = {"superseded_intermediate", "redundant_duplicate", "raw_low_info"}
THRESHOLD = 0.6

def sh(cmd):
    return subprocess.run(cmd, shell=True, capture_output=True, text=True).stdout

def tracked():
    return [p for p in sh(f"git ls-files {E}").split("\n") if p and not p.startswith(f"{E}/raw/")]

def refs(name, files):
    hits = sh(f"grep -rl --exclude-dir=raw --exclude-dir=target --exclude-dir=node_modules -F '{name}' crates scripts docs {E} 2>/dev/null").split("\n")
    hits = [h for h in hits if h and os.path.basename(h) != name]
    code = [h for h in hits if not h.startswith(E)]
    return code, [h for h in hits if h.startswith(E)]

def excerpt(path, n=900):
    try:
        with open(path, "rb") as f:
            b = f.read(4096)
        if b.startswith(b"\x89PNG") or path.endswith((".png", ".jpg")):
            return "<binary image>"
        return b[:n].decode("utf-8", "replace")
    except OSError:
        return "<unreadable>"

def extract():
    files = tracked()
    out = []
    for p in files:
        name = os.path.basename(p)
        code, ev = refs(name, files)
        added = sh(f"git log --diff-filter=A --format=%as -1 -- '{p}'").strip()
        out.append({"path": p, "name": name, "bytes": os.path.getsize(p),
                    "lines": sum(1 for _ in open(p, "rb")) if not p.endswith((".png", ".jpg")) else 0,
                    "added": added, "cited_by_code": code, "cited_by_evidence": [os.path.relpath(x, E) for x in ev],
                    "hard_keep": name in KEEP_NAMES or bool(code), "excerpt": excerpt(p)})
    print(json.dumps(out, indent=1))

def ask(cands_path, ledger):
    cands = json.load(open(cands_path))
    todo = [c for c in cands if not c["hard_keep"]]
    labels = {}
    for i in range(0, len(todo), 8):
        batch = todo[i:i + 8]
        state = {"policy": "Keep enough evidence to understand a decision and reproduce a result. Keep final summaries, decisions, owner verdicts and reusable verification sources; raw run output, per-iteration output, snapshots and drafts belong in ignored storage.",
                 "files": [{"id": f"f{j}", "path": os.path.relpath(c["path"], E), "bytes": c["bytes"], "lines": c["lines"],
                            "added": c["added"], "cited_by_other_evidence": c["cited_by_evidence"], "excerpt": c["excerpt"]}
                           for j, c in enumerate(batch)]}
        questions = {f"f{j}": {"type": "choice", "instructions": f"For file f{j}, which disposition fits under the policy? A file cited by other retained evidence leans essential.",
                               "criteria": CHOICES} for j in range(len(batch))}
        json.dump(state, open("/tmp/claude-1000/jev-state.json", "w")); json.dump(questions, open("/tmp/claude-1000/jev-q.json", "w"))
        r = subprocess.run(["bash", "-ic", f"./target/ci/jev ask --state /tmp/claude-1000/jev-state.json --questions /tmp/claude-1000/jev-q.json --ledger {ledger} --tool retention"],
                           capture_output=True, text=True)
        if r.returncode != 0:
            sys.stderr.write(r.stderr[-800:]); sys.exit(1)
        out = json.loads(r.stdout)
        for j, c in enumerate(batch):
            a = out["answers"][f"f{j}"]
            labels[c["path"]] = {"choice": a.get("choice"), "confidence": a.get("confidence"), "distribution": a.get("distribution") or a.get("probabilities"), "ledger": out.get("reference")}
        sys.stderr.write(f"{i + len(batch)}/{len(todo)}\n")
    print(json.dumps(labels, indent=1))

def apply(cands_path, labels_path):
    cands = json.load(open(cands_path)); labels = json.load(open(labels_path))
    rev = sh("git rev-parse HEAD").strip()
    dropped, kept = [], []
    for c in cands:
        l = labels.get(c["path"])
        # The drop labels' summed probability decides, as the loop's own
        # acceptance rule reads mass rather than the argmax.
        mass = sum((l or {}).get("distribution", {}).get(k, 0) for k in DROP) if l else 0
        drop = (not c["hard_keep"]) and mass >= THRESHOLD
        entry = {"path": os.path.relpath(c["path"], E), "sha256": hashlib.sha256(open(c["path"], "rb").read()).hexdigest(),
                 "bytes": c["bytes"], "label": (l or {}).get("choice", "hard_keep"), "confidence": (l or {}).get("confidence"), "drop_mass": round(mass, 2), "ledger": (l or {}).get("ledger")}
        (dropped if drop else kept).append(entry)
        if drop:
            dest = os.path.join(E, "raw", "retired-2026-09-22", entry["path"])
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            os.replace(c["path"], dest)
            subprocess.run(["git", "rm", "-q", "--cached", c["path"]], check=True)
    json.dump({"source_revision": rev, "threshold": THRESHOLD, "recover": f"git show {rev}:{E}/<path>", "dropped": dropped, "kept": kept},
              open(os.path.join(E, "retention-manifest.json"), "w"), indent=1)
    print(f"dropped {len(dropped)} kept {len(kept)}")

cmd = sys.argv[1]
{"extract": lambda: extract(), "ask": lambda: ask(sys.argv[2], sys.argv[3]), "apply": lambda: apply(sys.argv[2], sys.argv[3])}[cmd]()
