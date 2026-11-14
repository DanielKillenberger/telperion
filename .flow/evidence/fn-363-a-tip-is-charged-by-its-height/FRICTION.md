# fn-363 friction

## 13 Nov 2026, 18:35 (builder, Opus)

- **Doing:** the first narrow build (`balance_tests`) through `telperion-heavy 8`.
- **What slowed it:** the queue held it at "room 5 GB for 8" for about 6 minutes (18:32 to 18:38) while another worktree's probe grew the oak at 300. The wait was expected and changed nothing else.
- **Cost:** 6 minutes of wall time. No tokens were wasted, because other work went on meanwhile.
- **What would have removed it:** nothing in the repository. It is the shared machine's admission rule working as designed.

## 13 Nov 2026, 18:40 (builder, Opus)

- **Doing:** writing scripts and logs into the session scratchpad.
- **What hindered it:** the shell hook refuses every truncating redirect to a home path, even into a new file under the scratchpad (`> scratchpad/x.out`), and refuses a heredoc into `python3 -`. Three commands had to be rewritten: once as an append (`>>`), once with the file tool, once with `git diff | git apply -R`.
- **Cost:** about 3 minutes and three refused calls.
- **What would have removed it:** a line in the dispatch brief saying that appends (`>>`) and the file tool pass the hook. The brief already lists what is refused.

## 13 Nov 2026, 19:25 (builder, Opus)

- **Doing:** R5's young trees, one `telperion-heavy` job per tree.
- **What slowed it:** each tree was declared at 8 GB. With the other probe's oak chain at 300 holding memory, the queue kept finding "room 4 GB for 8". A tree that grows in under 10 s, at 0.1 to 1.6 GiB peak, waited up to 8 minutes for admission. Over 20 minutes went to 30 young trees.
- **Cost:** about 20 minutes of wall time.
- **What would have removed it:** declaring each tree near its measured peak, 4 GB for the young trees and 6 for the oak at 80. I restarted the batch that way at 19:24; finished trees are kept. A reusable runner that read its declarations from a measured table would avoid guessing.

## 13 Nov 2026, 20:07 (builder, Opus)

- **Doing:** R5's remaining trees: the young presets after, then the old oak, both runs.
- **What hindered it:** the yield watcher froze every job at 19:33:16. It was still frozen at 20:07, while the owner used the machine. About 50 trees were left, the old oak among them, which takes 1.5 to 2 hours of queue once running.
- **Cost:** 34 minutes of waiting so far, with no progress possible.
- **What I did:** the first Stop had already fired on measured numbers (Telperion at 82, leaf +41%), and R6 may not be drawn past it, so the remaining R5 rows could not change the decision. I wrote MEASURES.md with what was measured and returned to the host instead of waiting. `r5/all.sh` resumes the batch and keeps finished trees.
