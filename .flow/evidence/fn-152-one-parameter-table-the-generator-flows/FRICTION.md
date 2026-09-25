# fn-152 friction

## 2026-09-25: the read map had to be built before the task could be sized

- **Doing:** step 1 of the dispatch, mapping which stage reads each of the 249 wire rows.
- **Slowed by:** rows reach stages inside whole structs (`&SkeletonParams`, `CanopyParams`, `self.x`), and short field names collide (`length`, `spread`, `step`, `size`), so grep cannot attribute a read to a row. Tracing them took four parallel read-only agents, about 5 minutes of wall time and roughly 670k subagent tokens.
- **Cost:** the dispatch ended at the map; no code moved.
- **Would have removed it:**
  - The spec's own Unknown bullet ("whether every stage can move to views in one pass") answered while planning, so the work arrived as the staged specs in READ-MAP.md.
  - A generated row-to-reader index, which the table in stage 1 of the split would provide.
- **Hook:** the dcg hook refused a heredoc redirect into the evidence directory created in the same command (the parent was missing when it checked). Writing through the editor instead cost one retry.
