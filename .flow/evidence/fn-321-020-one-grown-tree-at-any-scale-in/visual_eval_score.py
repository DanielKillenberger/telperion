#!/usr/bin/env python3
"""Score visual_eval.py's answers: accuracy by group, order agreement, time, cost, agreement with the strong models, and an escalation cascade."""
import json, statistics as st, sys

L = json.load(open(sys.argv[1]))
ITEMS = L["items"]
R = L["results"]
PRICE = {"opus-5.5": (4, 20), "sonnet-5.5": (2, 10), "astra-6": (10, 50), "jev": (0.042, 0)}

def right(i, a):
    return ITEMS[i]["want"] is not None and a.get("answer") == ITEMS[i]["want"]

def acc(m, group):
    idx = [i for i, it in enumerate(ITEMS) if it["group"] == group]
    return sum(right(i, R[m]["answers"][i]) for i in idx), len(idx)

def resolved(item, ans):
    """Map a pair answer to the image it names, so two orders can be compared."""
    if item["kind"] != "pair":
        return ans
    if ans == "first":
        return item["images"][0]
    if ans == "second":
        return item["images"][1]
    return ans

def order_agreement(m):
    by = {}
    for i, it in enumerate(ITEMS):
        if it["kind"] == "pair" and it["group"] in ("owner", "fullness"):
            by.setdefault((it["group"], it["key"]), []).append(resolved(it, R[m]["answers"][i].get("answer")))
    same = sum(len(set(v)) == 1 for v in by.values())
    return same, len(by)

def cost(m, i_or_all=None):
    usd = []
    base = R[m].get("baseline") or {}
    for a in R[m]["answers"]:
        if "error" in a:
            continue
        if m == "jev":
            usd.append(a["input"] * PRICE[m][0] / 1e6)
        elif m == "astra-6":
            net = max(a["input"] - base.get("input", 0), 0)
            usd.append(net * PRICE[m][0] / 1e6 + a["output"] * PRICE[m][1] / 1e6)
        else:
            usd.append(a.get("harness_usd") or 0)
    return usd

groups = ["owner", "defect", "species", "fullness", "control"]
print(f"{'model':11}" + "".join(f"{g:>10}" for g in groups) + f"{'labelled':>10}{'order':>8}{'errs':>6}{'med s':>7}{'p95 s':>7}{'$/call':>10}{'$ total':>9}")
for m in R:
    row, ok, n = "", 0, 0
    for g in groups:
        a, b = acc(m, g)
        ok, n = ok + a, n + b
        row += f"{a:>7}/{b:<2}"
    o = order_agreement(m)
    s = [a["s"] for a in R[m]["answers"] if "s" in a]
    u = cost(m)
    errs = sum("error" in a for a in R[m]["answers"])
    print(f"{m:11}{row}{ok:>6}/{n:<3}{o[0]:>4}/{o[1]:<3}{errs:>6}{st.median(s):>7.2f}{sorted(s)[int(len(s)*.95)]:>7.2f}{st.mean(u):>10.5f}{sum(u):>9.3f}")

print("\nbaseline (no image):", {m: {k: R[m]["baseline"].get(k) for k in ("input", "output", "harness_usd")} for m in R if R[m].get("baseline")})
print("astra raw $/call incl. harness at list:", round(st.mean((a["input"] - a.get("cached", 0)) * 10 / 1e6 + a.get("cached", 0) * 1 / 1e6 + a["output"] * 50 / 1e6 for a in R["astra-6"]["answers"] if "s" in a), 4))
print("wall s:", {m: round(R[m]["wall_s"]) for m in R})

print("\nagreement on the unlabelled 0.1.4-vs-new pairs (18), by the image chosen:")
idx = [i for i, it in enumerate(ITEMS) if it["group"] == "agreement"]
models = list(R)
for a in models:
    print(f"  {a:11}" + " ".join(f"{b}:{sum(resolved(ITEMS[i], R[a]['answers'][i].get('answer')) == resolved(ITEMS[i], R[b]['answers'][i].get('answer')) for i in idx)}" for b in models if b != a))
print("\nowner-verdict items each model missed:")
for m in R:
    missed = {}
    for i, it in enumerate(ITEMS):
        if it["group"] == "owner" and not right(i, R[m]["answers"][i]):
            missed[it["key"]] = missed.get(it["key"], 0) + 1
    print(" ", m, missed)
print("\ndefect items missed:")
for m in R:
    print(" ", m, [ITEMS[i]["images"][0] + ":" + ITEMS[i]["q"][:22] + "=>" + str(R[m]["answers"][i].get("answer")) for i, it in enumerate(ITEMS) if it["group"] == "defect" and not right(i, R[m]["answers"][i])])
print("\nspecies missed:")
for m in R:
    print(" ", m, [ITEMS[i]["images"][0] + "=>" + str(R[m]["answers"][i].get("answer")) for i, it in enumerate(ITEMS) if it["group"] == "species" and not right(i, R[m]["answers"][i])])

labelled = [i for i, it in enumerate(ITEMS) if it["want"] is not None]
print("\ncascade: Jev answers when its top probability >= t, else the strong model answers")
for strong in ("opus-5.5", "sonnet-5.5", "astra-6"):
    su = cost(strong)
    ju = cost("jev")
    print(" ", strong, f"alone {sum(right(i, R[strong]['answers'][i]) for i in labelled)}/{len(labelled)}")
    for t in (0.5, 0.6, 0.7, 0.8, 0.9):
        ok = esc = 0
        c = 0.0
        for i in labelled:
            j = R["jev"]["answers"][i]
            c += ju[i] if i < len(ju) else 0
            if j.get("p", 0) >= t:
                ok += right(i, j)
            else:
                esc += 1
                ok += right(i, R[strong]["answers"][i])
                c += su[i] if i < len(su) else 0
        print(f"    t={t}: {ok}/{len(labelled)} escalated {esc} (${c:.3f} vs ${sum(su[i] for i in labelled if i < len(su)):.3f} strong alone)")

print("\nJev calibration: answers above and below each probability")
for lo, hi in ((0, .5), (.5, .6), (.6, .7), (.7, .8), (.8, .9), (.9, 1.01)):
    b = [i for i in labelled if lo <= R["jev"]["answers"][i].get("p", 0) < hi]
    if b:
        print(f"  {lo:.1f}-{hi:.1f}: {sum(right(i, R['jev']['answers'][i]) for i in b)}/{len(b)}")
