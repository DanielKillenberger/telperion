# fn-258 friction

## 2026-10-10, task 1

- **Two same-age rows could not be measured: the spruce at 100 and the beech at 120 ran out of device memory.** Other sessions held about 4.5 GB of the 10 GB card while the sweep ran (7.5 GB used at the time), and F4 asks 4-6 GB for a large tree before its first frame. The rows are recorded as refused, not as numbers. Cost: two probe slots and a rerun to plan. What would remove it: a GPU memory reservation beside `gpu.lock`, or the sweep refusing to start below a stated free memory.
- **The dcg hook blocked the GPU wrapper written by heredoc**, because the redirect target was a variable. Rewritten with the Write tool and literal paths. Cost: about 1 minute. Local setup, reported and not specced.
- **The baseline sweep took about 25 minutes**, mostly the same-age rows: the ordinary at 100 builds in 16.5 s and the oak at 100 in 25.4 s, four builds each. Cost: 25 minutes of wall time with the GPU lock held. What would remove it: a probe flag for one build where only memory is wanted.
