The oregon-white-oak and norway-spruce species records moved to `catalogue/oregon-white-oak/` and `catalogue/norway-spruce/`.

`references.json` and `protocol.json` stay here: they are the fn19 benchmark's frozen
inputs, pinned by `protocol_sha256` and `references_sha256` in every run record and read
by `examples/geometry_benchmark/runner.py`, and `references.json` also carries the
competitor records, which belong to no species.
