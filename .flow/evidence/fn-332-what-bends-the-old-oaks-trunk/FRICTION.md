# fn-332 friction

## 3 Nov 2026: a gate beside a worker's builds overran its budget

Doing: the gate on fn-332's one example file, while fn-333's worker built and tested in its own worktree.

Slowed by: the two shared the CPUs (load average 18); the test run took 1,945 s against the 1,560 s budget and the gate failed on time alone. About 32 minutes lost, and the gate must run again.

Removed by: running a gate only when no other build or test suite runs; the host waits for a worker's narrow runs to finish before starting one.
