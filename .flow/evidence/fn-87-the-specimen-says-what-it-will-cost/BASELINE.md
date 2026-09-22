# Before measurements

These measurements were supplied by the dispatching host, not rerun by this
worker. Base commit: `38e6c57f`. Machine: 32 cores, 32 GB. Profile: `ci`.
Workload: the whole `target/ci/deps/species-*` binary. Peak RSS was polled from
`/proc/<pid>/status`, field `VmHWM`. MB units are preserved as supplied.

| Test threads | Peak RSS (MB) | Suite wall time (s) |
| --- | --- | --- |
| 32 | 5,695 | 49.12 |
| 4 | 5,602 | 47.76 |
| 2 | 3,486 | 74.45 |
| 1 | 2,589 | 104.03 |

R5's permitted 15 percent regression uses 49.12 s: the upper limit is 56.488 s.
The pre-fn-86 figures in the spec's Decision Context are historical and are not
the acceptance baseline.

No after measurements exist: implementation stopped for the admission-policy
decision recorded in FRICTION.md.
