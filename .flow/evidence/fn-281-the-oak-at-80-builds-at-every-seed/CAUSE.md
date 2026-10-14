# fn-281 R2: why the oak at 80, seed 7, has an unsolved radius

Checked in code on `fn-281-oak-radii` at 783605a11 plus the probes of c6c2a14d5. Logs are in `raw/` (ignored): `radii_tests-1.log`, `radii_tests-2.log`, `above_tests.log`, `oak80s7.log`, `device-oak80s7.log`.

## The failure condition

`Tree::validate_solved` (`crates/telperion-core/src/tree.rs:378`) refuses a tree with "unsolved radii" when any node's radius is not above zero. The engine's tree is not radius-solved in the core: each node takes its radius from the engine's girth in the conversion (`pipeline/space.rs`), and the first stage that validates it refuses it.

## The node

The tree has 2,193,278 nodes, and one is at radius zero: node 1,849,447. It is a lateral head (`branch` is itself, `BudFate::Lateral`), a twig with radius, start and base all 0 and no codominant weight. Its parent is 4.4 mm thick. The sweep's seven unsolved-radii refusals hold ten nodes at radius zero, one to three a tree, and every one has this signature (`SWEEP.md`).

Its identity is the lineage of phytomer 22 of 26 on engine axis 419,480 (pa 9). Relay 484,593 joins that axis inside the span of that phytomer, at `t` 0.623 (`radii_tests-2.log`).

## The chain, step by step

1. **The engine's join leaves no wood above.** `girth::thicken_given` (`crates/telperion-space/src/girth.rs:283`) sets `Join::above` to `weigh(section - at)`. Here `section` is the f32 running section at the join's phytomer, with the relay's share `at` already added; secondary growth is 1. The stopped module's head above the join carries a section of 4.1e-13, its own wood with nothing lateral. The relay's share is 1.2e-5. The ratio is 3.4e-8, under f32's 2^-24 (6.0e-8). The head is absorbed in the sum, the subtraction returns exactly 0, and `above` is 0. In f64 the head's radius is about 1.8e-5 m. The head's darkest leaf term is light 0.046, not zero (`above_tests.log`).
2. **The conversion keeps the phytomer.** `space.rs:426` keeps phytomers while `p.radius >= FINEST` (2e-4 m). `p.radius` is the phytomer's radius with the relay's wood in it (2.2e-3 m here), not the wood drawn above the join.
3. **The conversion draws the head at the join's `above`.** `space.rs:443` takes `own.radius.min(j.above)`, which is 0, and the remainder of the span (1 - t of its wood) is long enough to be drawn (`SHORTEST`) and not behind the join. A node is pushed at radius 0.
4. **The relay turns the head aside.** `aside` (`space.rs:262`) makes that node a lateral run, with `start_radius = radius` = 0, and its codominance is none at weight 0.
5. **The first validating stage refuses the tree.** That is `validate_solved` in surface, field or canopy, or in `clothe_leaf_bases` for a preset that hangs bases. The probe holds the expansion's tree to it directly.

## How common it is in this tree

- **Zero joins:** 112 relay joins have `above` exactly 0.
- **Kept by the conversion:** 102 of the 112 sit on a phytomer the conversion keeps.
- **Drawn:** only one of those 102 is drawn. The others are not drawn as their own node: merged, behind the join, or on an axis the conversion never reached. Which of the three applies to each was not split.
- **Absorption:** the engine-side check finds 97 of the 102 absorbed in f32 with wood above (`above_tests.log`). None has a head whose every leaf term is zero.
- **Thin wood the module says it never draws:** the same tree draws 486 nodes above zero but thinner than `FINEST`. The module doc says such wood "is not drawn". The source of each of the 486 was not split, so how many come from this clamp is unknown.

## The device

The device builds the same tree without refusing: `every_tree_builds_on_the_device`, filtered to `oregon-white-oak 80 7`, gave 3,422,802 instances in 3.6 s (`device-oak80s7.log`). The device runs no `validate_solved`. Its dress clamps the same way, `radius = min(radius, above)` at `generation/cycle/dress.wgsl:753`. Whether the device's own girth also gives this join an `above` of 0, and so draws a zero-radius head, was not checked. So R1 has no red test on the device, because the device path does not report the refusal.
