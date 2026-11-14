#!/usr/bin/env python3
"""The owner-verdict and factual groups split by Jev's probability, and a cascade per group."""
import json, sys

L = json.load(open(sys.argv[1]))
I, R = L["items"], L["results"]
right = lambda i, a: a.get("answer") == I[i]["want"]
J = R["jev"]["answers"]
for name, groups in (("owner", {"owner"}), ("factual", {"defect", "species", "fullness", "control"})):
    idx = [i for i, it in enumerate(I) if it["group"] in groups]
    print(name, "n", len(idx), {m: sum(right(i, R[m]["answers"][i]) for i in idx) for m in R})
    for lo, hi in ((0, .7), (.7, .9), (.9, 1.01)):
        b = [i for i in idx if lo <= J[i]["p"] < hi]
        print(f"  jev p {lo}-{hi}: {sum(right(i, J[i]) for i in b)}/{len(b)}")
    for t in (.7, .8, .9):
        ok = sum(right(i, J[i]) if J[i]["p"] >= t else right(i, R["astra-6"]["answers"][i]) for i in idx)
        print(f"  cascade to astra t={t}: {ok}/{len(idx)}, escalated {sum(J[i]['p'] < t for i in idx)}")
print("owner items Jev got wrong at p >= 0.8:")
for i, it in enumerate(I):
    if it["group"] == "owner" and J[i]["p"] >= .8 and not right(i, J[i]):
        print(" ", it["images"], "want", it["want"], "got", J[i]["answer"], round(J[i]["p"], 2))
# Binomial sign test by pair, both orders right counts one.
pairs = {}
for i, it in enumerate(I):
    if it["group"] == "owner":
        pairs.setdefault(it["key"], []).append(i)
for m in R:
    print(m, "pairs right in both orders:", sum(all(right(i, R[m]["answers"][i]) for i in v) for v in pairs.values()), "of", len(pairs),
          "; answered no_difference:", sum(R[m]["answers"][i].get("answer") == "no_difference" for v in pairs.values() for i in v))
