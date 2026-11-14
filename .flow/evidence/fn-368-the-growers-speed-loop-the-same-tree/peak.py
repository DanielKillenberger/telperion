"""fn-368: a command's peak memory, wall and CPU seconds (user plus system).

The scratchpad's peak.py, with CPU seconds to the hundredth: the oak at 40
grows in about a second, which whole seconds cannot tell apart.
"""
import resource
import subprocess
import sys
import time

t = time.time()
r = subprocess.run(sys.argv[1:])
u = resource.getrusage(resource.RUSAGE_CHILDREN)
cpu = u.ru_utime + u.ru_stime
print(f"PEAK_MIB {u.ru_maxrss // 1024} WALL_S {time.time() - t:.2f} CPU_S {cpu:.2f} EXIT {r.returncode}")
sys.exit(r.returncode)
