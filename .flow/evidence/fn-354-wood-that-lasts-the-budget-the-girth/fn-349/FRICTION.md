# fn-349 friction

## 11 Nov 2026, 06:05, builder: a lattice just under its limit costs more than the desk has

- **Doing:** the space crate's lib tests after the corrected balance.
- **Slowed by:** `paused::tests::a_growth_that_failed_is_spent` and `light::tests::a_lattice_past_its_nodes_is_refused_by_name` were killed by `telperion-heavy` at 8, 12, 16 and then 20 GB.
  - **Cause:** both tests grow the oak with 5 m internodes to age 8 and expect the light lattice to pass 2^28 nodes and be refused. Under the new balance, the tree's lattice at age 8 comes to about 233 M nodes, just under the limit. Each core then allocates its own 1.87 GB lattice buffer (measured: `memory allocation of 1869362208 bytes failed` under `ulimit -v 8000000`).
  - **Why it did not fail fast:** a lattice under the limit is admitted whatever it costs, so the test does not fail by name. It runs out of memory instead.
- **Cost:** about 15 minutes and six killed runs.
- **What would have removed it:** a refusal that is checked against the bytes the sweep will take, not only the node count. Or a test tree built to overflow by a margin, not one that sits near the limit.
