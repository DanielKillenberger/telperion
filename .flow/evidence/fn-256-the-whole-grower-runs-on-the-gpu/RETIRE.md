# fn-256 task 2: retirement on the device (worker, 2026-10-11)

This builds DESIGN.md section 6, "As decided for the build". Commits f8ceda9bd (the verdicts) and d99d4e739 (letting go and the settle).

## How it runs

- **In the plan, each cycle's start, no sync.** Passes are sized by the axes bound, in phase "retirement" after the step (`plan_retire.rs`):
  - open marking from each living apex and sleeping bud;
  - closing, and each closed axis's presences assigned once;
  - the woken buds' waits and kept shares;
  - living carried up the hierarchy's block levels;
  - the verdicts;
  - the retired subtrees marked inside, level by level from the top.
- **Only a retired root's living is stored.** That is `R_ROOT_LIVED` and `R_ROOT_ALIVE`; an axis inside the subtree carries nothing up.
- **Compaction happens only at a move, and only where a build asks for it** (`Sizes.compact`, `retire_compact.rs`):
  - each old row is flagged kept or recorded;
  - two stable scans place the rows;
  - the kept rows are scattered into the new tables;
  - the recorded rows' grown columns go into a records buffer, but only where a PA keeps shed pipes or leaf girth (PA flags & 6);
  - each unit's `U_FIRST` is remapped, into the records by its high bit, or emptied where no PA keeps the rows;
  - the census's phytomers are set to the kept count.
- **One wait per compaction, plus one more when records are kept.** When records are kept, it reads back their size first. Every compaction waits for the device before its old tables are let go.
- **The settle uses the given root living.** It drops the subtree walk below a retired root, reads a record wherever a unit points into the records, and sizes its layout and sums by arena rows plus records.

## Agreement (exact)

- **Verdicts** (`every_verdict_is_the_cpus`): every root, cycle, verdict (waited, kept or retired) and living equals the CPU's on all eight young presets. That includes the waits on the woken buds: 18 on the spruce and 16 on the ordinary.
- **Settled tree with retired subtrees let go** (`young_trees_settle_as_on_the_cpu_with_retired_subtrees_let_go`): all eight young presets are grown from 512-row phytomer tables, so they move several times, and every word equals the CPU's laid tree. The oak and the ordinary read from their records.
- **Determinism** (`the_node_table_with_retirement_is_the_same_on_every_context`): oak 9 and spruce 12, plain with compaction, give the same laid node table on ten contexts and at block caps 64, 128 and 1,024 (264,642 and 1,183,746 words, none differ).
- **The cycle suite is still green** (36 tests), as are the telperion-space grow tests (21).

## Arena (rows held of the final phytomer tables)

| Build | Without letting go | With | Records |
|---|--:|--:|--:|
| european-beech 8 | 1,941 of 4,547 | 1,941 of 4,547 | 0 |
| oregon-white-oak 9 | 9,145 of 16,420 | 8,826 of 16,227 | 319 |
| norway-spruce 12 | 31,914 of 69,236 | 30,360 of 50,515 | 0 |
| date-palm 8 | 96 of 512 | 96 of 512 | 0 |
| silver-birch 8 | 4,852 of 11,592 | 4,813 of 11,423 | 0 |
| ordinary 10 | 4,761 of 9,970 | 4,573 of 7,402 | 188 |
| laurelin 8 | 1,941 of 4,547 | 1,941 of 4,547 | 0 |
| telperion 8 | 1,941 of 4,547 | 1,941 of 4,547 | 0 |

**Telperion 82, plain** (`retirement_is_measured`, `raw/retire-time-4.txt`):

| | Without | With |
|---|--:|--:|
| Arena rows | 18,239,310 | 10,130,717 |
| Final phytomer tables | 23,950,613 | 13,613,661 |
| Resident | 3,410 MB | 2,624 MB |
| Retirement, stamped | 236 ms in 3,029 dispatches | 117 ms in 3,029 |
| Longest retirement dispatch | 0.91 ms | 0.72 ms |
| Moves | 0 | 2 |
| Build wall | 1.55 s | 2.68 s |

Both settle to the same 373,607 kept axes and 3,773,761 kept phytomers.

**Caveat on the "without" column.** It reuses the table sizes a first build settled on, so it makes no moves. The "with" column starts from the least tables and moves twice. Part of the 1.1 s in wall time is those moves, not the compaction; I have not separated the two.

## Open

- **Telperion keeps no records.** No Telperion PA keeps shed pipes or leaf girth, so the 18M-row case does not exercise the records. The oak and the ordinary exercise them young; the oak at 80 has not been measured with compaction.
- **The compaction's whole-device wait** (`device.poll`, `retire_compact.rs`) was added while I chased a settle fault ("a phytomer no unit assigned"). The fault's cause turned out to be the settle's layout, which was sized by arena rows without the records. Whether the wait is needed was not checked; a test without it would settle that.
