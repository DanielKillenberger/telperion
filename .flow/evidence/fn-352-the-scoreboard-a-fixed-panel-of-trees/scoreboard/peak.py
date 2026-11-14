"""fn-330: run one command, then print its wall seconds and peak resident memory.
# fn-352: copied from fn-330 probes/peak.py, unchanged below.

Usage: peak.py <command...>. The child's output passes through; the last line
is `PEAK seconds <s> rss_gib <g> exit <code>`.
"""
import resource, subprocess, sys, time

start = time.monotonic()
code = subprocess.run(sys.argv[1:]).returncode
seconds = time.monotonic() - start
rss = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss / 2**20
print(f"PEAK seconds {seconds:.1f} rss_gib {rss:.2f} exit {code}", flush=True)
sys.exit(code)
