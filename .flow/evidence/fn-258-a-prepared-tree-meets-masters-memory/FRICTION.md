# fn-258 friction

## 2026-10-10, task 1

- **Two same-age rows could not be measured: the spruce at 100 and the beech at 120 ran out of device memory.** Other sessions held about 4.5 GB of the 10 GB card while the sweep ran (7.5 GB used at the time), and F4 asks 4-6 GB for a large tree before its first frame. The rows are recorded as refused, not as numbers. Cost: two probe slots and a rerun to plan. What would remove it: a GPU memory reservation beside `gpu.lock`, or the sweep refusing to start below a stated free memory.
- **The dcg hook blocked the GPU wrapper written by heredoc**, because the redirect target was a variable. Rewritten with the Write tool and literal paths. Cost: about 1 minute. Local setup, reported and not specced.
- **The baseline sweep took about 25 minutes**, mostly the same-age rows: the ordinary at 100 builds in 16.5 s and the oak at 100 in 25.4 s, four builds each. Cost: 25 minutes of wall time with the GPU lock held. What would remove it: a probe flag for one build where only memory is wanted.
- **`cargo fmt -p telperion-render` reformatted 18 files nobody had touched**, because the crate is not fmt-clean; they went into a commit and had to be taken out again with a soft reset and a tagged stash. Cost: about 4 minutes. What would remove it: a fmt-clean crate, or CI checking fmt so it stays clean.
- **Leaf stills of the fine-executor trees are not repeatable run to run** (spruce, Telperion: 0.08-0.16 % of pixels differ between two runs of the same binary), so a cut that should change nothing could not be proved byte-identical on them and was held to the repeat noise instead. Cost: about 6 minutes and two extra GPU runs. What would remove it: a repeatable fine executor, or a stated tolerance for its stills.
- **The scratch `/tmp` hit its quota while linking the probe** (`Disk quota exceeded`, linker SIGBUS); a retry linked. Cost: 2 minutes. Local setup, reported and not specced.
- **The R3 sweep took about 55 minutes for 48 rows**, again mostly the same-age builds (oak at 100: 25-31 s a build, four builds a row). What would remove it: the one-build flag named above.

## 2026-10-11, task 1, after the host's decisions

- **The cold-compile test was red alone too at a load average of 6** (`grow` 20.6 s against 20 s), where an earlier lone run at 3.4 passed. The bound sits within a few percent of what this machine takes under other sessions' load. Cost: one extra locked GPU run, about 1 minute. What would remove it: fn-237 R9.
- **ImageMagick 7's `compare -metric AE` reported an intensity-weighted figure, not a count of differing pixels**, so RESULT.md first stated a pixel difference 100× too small; Astra caught it. Cost: about 5 minutes and one correction pass. What would remove it: a pixel-difference helper in the evidence tools that states its metric.
