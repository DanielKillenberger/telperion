# fn-188 friction

## 2026-10-02, task 3 (the competitive-growth probe)

- **Doing:** reusing R2's measures (traced systems, twig wood on thick limbs, tone) on the probe's tree.
- **Hindered by:** R1's and R2's tools (`traced`, `twigsee`, `ablrender`, `lens`) were never committed; they survive only in the host session's scratchpad (`/tmp/claude-1000/...`), and R7's prose ("substantial divisions") does not match the tracer's code, which medians every scaffold daughter. Finding them and reading the code to learn the definitions took about 10 minutes and ~25k tokens.
- **Cost:** ~10 min, ~25k tokens.
- **What would remove it:** commit each round's measuring tool and revert it, as the generator switches were, so the definition sits in history beside the numbers.

## 2026-10-02, task 3, measuring

- **Doing:** collecting the probe's rows into `raw/probe.jsonl` and peak memory.
- **Hindered by:** the dcg hook blocks a shell redirect to a variable path (`> $R/probe.jsonl`), and `/usr/bin/time` is not installed, so peak RSS went through a Python `getrusage` wrapper.
- **Cost:** ~3 min, ~3k tokens.
- **What would remove it:** a committed measuring driver that writes its own rows and reads its own RSS (local setup for the second part; reported, not specced).
