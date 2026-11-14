# fn-362 friction

## 13 Nov 2026, 03:50: the base beech at 80 killed at the probe's 9 GB scope

Doing: R4's measurement, a scratch probe that builds the tree and then digests its wood. What slowed it: the digest flattened the wood (`SurfaceMesh::flattened`), which on the base's 101.8 M vertices adds about 5.6 GB to the build's 2.1 GB, and the scope killed it after the build had already been read (`KILLED (exit 137)`). Cost: one base run (33 s) and a rerun of every base tree. What would have removed it: a digest over the chunks as they are, which the probe now takes; `flattened()` is "for tests and evidence tooling" and says nothing about what it costs at a shipped tree's size.
