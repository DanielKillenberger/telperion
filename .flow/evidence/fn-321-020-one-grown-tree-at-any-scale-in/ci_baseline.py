#!/usr/bin/env python3
"""Telperion's tests workflow, last 30 completed runs: wall time, failure rate, and which jobs and steps cost and fail."""
import collections, datetime as dt, json, statistics as st, subprocess

def gh(path):
    return json.loads(subprocess.run(["gh", "api", path], capture_output=True, text=True, check=True).stdout)

def ts(s):
    return dt.datetime.fromisoformat(s.replace("Z", "+00:00"))

REPO = "DanielKillenberger/telperion"
runs = gh(f"repos/{REPO}/actions/workflows/tests.yml/runs?per_page=30&status=completed")["workflow_runs"]
print("runs", len(runs), collections.Counter(r["conclusion"] for r in runs), collections.Counter(r["event"] for r in runs))
print("head branches", collections.Counter(r["head_branch"] for r in runs).most_common(6))
green = [(ts(r["updated_at"]) - ts(r["created_at"])).total_seconds() / 60 for r in runs if r["conclusion"] == "success"]
if green:
    g = sorted(green)
    print(f"green wall min median {st.median(g):.1f} p90 {g[int(len(g) * .9)]:.1f} max {g[-1]:.1f} n {len(g)}")
jobs, fails, steps = collections.defaultdict(list), collections.Counter(), collections.defaultdict(list)
fail_steps = collections.Counter()
for r in runs:
    for j in gh(f"repos/{REPO}/actions/runs/{r['id']}/jobs?per_page=100")["jobs"]:
        if j["conclusion"] == "failure":
            fails[j["name"]] += 1
            for s in j["steps"]:
                if s["conclusion"] == "failure":
                    fail_steps[f"{j['name']} / {s['name']}"] += 1
        if j["conclusion"] == "success" and j.get("started_at") and j.get("completed_at"):
            jobs[j["name"]].append((ts(j["completed_at"]) - ts(j["started_at"])).total_seconds() / 60)
            for s in j["steps"]:
                if s.get("started_at") and s.get("completed_at"):
                    steps[f"{j['name']} / {s['name']}"].append((ts(s["completed_at"]) - ts(s["started_at"])).total_seconds() / 60)
print("\njobs (success): median min, n; failures")
for k, v in sorted(jobs.items(), key=lambda x: -st.median(x[1])):
    print(f"  {st.median(v):6.1f} {len(v):3d} fail {fails[k]:3d}  {k}")
print("\nslowest steps (median min)")
for k, v in sorted(steps.items(), key=lambda x: -st.median(x[1]))[:10]:
    print(f"  {st.median(v):6.1f}  {k}")
print("\nfailing steps", fail_steps.most_common(10))
print("\nlatest 12:", [(r["conclusion"][:4], r["head_branch"][:18], r["created_at"][5:10]) for r in runs[:12]])
