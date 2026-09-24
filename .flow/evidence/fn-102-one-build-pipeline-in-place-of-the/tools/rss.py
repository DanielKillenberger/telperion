"""Run a command, pass its stdout through, then print its peak RSS as a JSON line.
usage: rss.py <preset> <cmd...>"""
import json
import resource
import subprocess
import sys

done = subprocess.run(sys.argv[2:], stdout=sys.stdout)
peak = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
print(json.dumps({"event": "rss", "preset": sys.argv[1], "kb": peak}), flush=True)
sys.exit(done.returncode)
