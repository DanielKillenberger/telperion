# fn-204: every preset's artifacts, before and after the move

`crates/telperion-core/examples/artifact_digest.rs` writes every artifact the one build makes for a preset (the skeleton tree and its shed count, the element, the wood, the leaves with their placed and retained counts, the structure records and the field snapshot) as debug text; each line below is the first 16 hex digits of its sha256. Debug text prints every float at round-trip precision, so equal digests are equal bytes.

    cargo build --profile ci -p telperion-core --example artifact_digest
    target/ci/examples/artifact_digest <preset> | sha256sum

Before, at `140ffc61` (the base, the clothing in the skeleton stage):

```
date-palm 8d7a61d2bdc8c9d2
european-beech 63bd65f435ffaaf7
laurelin ee159a8fc6378678
norway-spruce 39ec7946b872caa6
ordinary 01ab34f05816230f
oregon-white-oak af9a2c0b628ad218
silver-birch b1791e589018e1eb
telperion cc9b0ea305d3f6a9
```

After, the clothing at the start of expansion:

```
date-palm 8d7a61d2bdc8c9d2
european-beech 63bd65f435ffaaf7
laurelin ee159a8fc6378678
norway-spruce 39ec7946b872caa6
ordinary 01ab34f05816230f
oregon-white-oak af9a2c0b628ad218
silver-birch b1791e589018e1eb
telperion cc9b0ea305d3f6a9
```

Identical for all eight shipped presets, the date palm (`8d7a61d2bdc8c9d2`, 4,398,506 bytes of text) among them. Repeat runs give the same digest.

## After the P2 fix (host, 2026-10-05)

`clothe_leaf_bases` leaves a tree that already ends in exactly the bases its table hangs as it is, so a clothed tree handed back to `expand` is not clothed twice (`a_clothed_tree_is_not_clothed_again`, red first: 1,064 nodes against 552). All eight digests are again identical to the base's.
