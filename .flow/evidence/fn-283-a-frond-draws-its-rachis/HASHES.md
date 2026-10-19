# fn-283 .1: CPU mesh hashes, base 99111868b against the axes

Every shipped preset at its own age and seed, `mesh::build`, the whole TreeMesh's Debug text folded by DefaultHasher (a scratch test, not committed). Seven presets are byte-identical; the date palm's mesh moves (its axes and the wider leaf box), its 6,554 leaves kept (`suite::rachis::the_axes_move_no_leaf` holds them to the bit before quantisation).

```
before
HASH ordinary 039a4974e029fe5f leaves 13780
HASH oregon-white-oak 8eaf36ff9d3aff06 leaves 47575
HASH norway-spruce b781531635c3e96a leaves 4084402
HASH european-beech d9e51919e8bd53ba leaves 76640
HASH silver-birch 45e1d92800c63d48 leaves 137681
HASH date-palm c142176c897f61e6 leaves 6554
HASH telperion 7c2fc5114425f49a leaves 930938
HASH laurelin bacef20aa2ecfdc1 leaves 875204
after
HASH ordinary 039a4974e029fe5f leaves 13780
HASH oregon-white-oak 8eaf36ff9d3aff06 leaves 47575
HASH norway-spruce b781531635c3e96a leaves 4084402
HASH european-beech d9e51919e8bd53ba leaves 76640
HASH silver-birch 45e1d92800c63d48 leaves 137681
HASH date-palm 5846c7eb190923d7 leaves 6554
HASH telperion 7c2fc5114425f49a leaves 930938
HASH laurelin bacef20aa2ecfdc1 leaves 875204
```
