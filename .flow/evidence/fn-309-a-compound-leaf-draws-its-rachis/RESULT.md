# fn-309 task 1: the classes before and after

`crates/telperion-core/tests/walk_flying.rs`, run as
`cargo test --profile ci -p telperion-core --test walk_flying -- --ignored --nocapture --test-threads 1`
through `telperion-heavy 8 --threads 2`. Oak to palm at 40, `seat` 0.1 m, a pixel 26.8 mm (the
film's camera at 28.03 m). Columns: (a) on wood at least a pixel wide, (b) on wood under a pixel, (c)
on a drawn rachis at least and under a pixel wide, (d) a station- or cluster-borne leaf more than `seat` off
drawn wood, (e) any leaf more than `seat` off every drawn element. The base was measured in a scratch
worktree of 975a70eea with its own target and this test copied in. Raw logs, ignored:
`raw/walk_flying-base.txt`, `raw/walk_flying-change.txt`.

## Base (975a70eea): 13 of 13 frames hold flying leaves

```
from	to	age	seed	t	leaves	a	b	c>=1px	c<1px	d	e	farthest m
oregon-white-oak	date-palm	40	7	0.129	3937	339	1679	0	25	1901	1894	0.90
oregon-white-oak	date-palm	40	7	0.227	4139	301	1265	63	63	2513	2447	1.32
oregon-white-oak	date-palm	40	7	0.34	845	16	47	71	46	681	665	2.43
oregon-white-oak	date-palm	40	7	0.463	2476	121	43	255	93	2061	1964	3.04
oregon-white-oak	date-palm	40	1	0.25	3407	173	638	45	53	2538	2498	1.59
oregon-white-oak	date-palm	40	1	0.5	4936	249	159	411	110	4193	4007	3.37
oregon-white-oak	date-palm	40	1	0.75	3784	600	17	1047	221	1963	1899	2.69
oregon-white-oak	date-palm	40	7	0.25	4096	251	1058	78	58	2715	2651	1.68
oregon-white-oak	date-palm	40	7	0.5	3954	190	103	402	102	3329	3157	3.12
oregon-white-oak	date-palm	40	7	0.75	3218	573	12	979	198	1483	1456	3.15
oregon-white-oak	date-palm	40	42	0.25	1072	28	174	31	36	818	803	1.76
oregon-white-oak	date-palm	40	42	0.5	4977	197	169	427	102	4274	4082	2.65
oregon-white-oak	date-palm	40	42	0.75	2704	405	7	1062	236	1054	994	2.99
```

## Change: (d) and (e) are 0 on every frame

The placements' leaflets stand at their points, so the leaf count of every frame is unchanged.

```
from	to	age	seed	t	leaves	a	b	c>=1px	c<1px	d	e	farthest m
oregon-white-oak	date-palm	40	7	0.129	3937	39	3882	0	16	0	0	0.00
oregon-white-oak	date-palm	40	7	0.227	4139	70	4029	16	24	0	0	0.00
oregon-white-oak	date-palm	40	7	0.34	845	5	740	60	40	0	0	0.00
oregon-white-oak	date-palm	40	7	0.463	2476	40	2201	158	77	0	0	0.00
oregon-white-oak	date-palm	40	1	0.25	3407	13	3349	16	29	0	0	0.00
oregon-white-oak	date-palm	40	1	0.5	4936	869	3704	277	86	0	0	0.00
oregon-white-oak	date-palm	40	1	0.75	3784	1720	868	976	220	0	0	0.00
oregon-white-oak	date-palm	40	7	0.25	4096	31	4011	24	30	0	0	0.00
oregon-white-oak	date-palm	40	7	0.5	3954	644	2968	257	85	0	0	0.00
oregon-white-oak	date-palm	40	7	0.75	3218	1115	972	933	198	0	0	0.00
oregon-white-oak	date-palm	40	42	0.25	1072	6	1020	17	29	0	0	0.00
oregon-white-oak	date-palm	40	42	0.5	4977	565	4043	285	84	0	0	0.00
oregon-white-oak	date-palm	40	42	0.75	2704	883	487	1104	230	0	0	0.00
```

Measured: both tables. The base's frame 526 (t 0.34, seed 7) counts 665 leaves in (e), against the
host's 687 from the probe, which measured to the nearest ring's sphere only, where this test also
measures to the tapered capsule between rings; its pixel split is by width (twice the radius), where
the probe's was by radius.

## Task 2: the device (2026-10-23)

`pack.wgsl`'s station path (station and cluster placements) now places each leaflet at the
station point, keeping `on_rachis`'s axis, tangent, normal and share. It is the device's only
call site that places leaflets on wood; the frond call sites (`pack.wgsl` `leaflet`,
`resident/field.wgsl` `blade`) are unchanged.

**Agreement** (`organ_tests::the_device_draws_counts_of_organs_as_the_cpu_does`, which now also
matches each wood leaf to a CPU leaf by place within `APART` of the height). Measured, base
129e363df (CPU moved by task 1, device not), the test panicked on its second family:

```
oak, 1.3 leaflets: wood leaves 4280 / 4280, 2006 off every CPU leaf, turned 0.0 degrees
```

After the change:

```
birch, twelve fronds: wood leaves 6529 / 6529, 0 off every CPU leaf, turned 0.0 degrees
oak, 1.3 leaflets: wood leaves 4280 / 4280, 0 off every CPU leaf, turned 4.1 degrees
oak to palm at 0.01: wood leaves 3006 / 3006, 0 off every CPU leaf, turned 0.8 degrees
oak to palm at 0.3: wood leaves 4 / 4, 0 off every CPU leaf, turned 13.6 degrees
oak to palm at 0.9: wood leaves 0 / 0, 0 off every CPU leaf, turned -0.0 degrees
oak to palm at 0.34, 40, seed 7: wood leaves 740 / 740, 5 off every CPU leaf, turned 11.1 degrees
```

The five off on the film's frame are one placement at the stem's top (y 9.52 m), 0.44 m from
every CPU leaf. With `leafletCount` 1 and `rachisLength` 0 on the same frame its one leaf stands
0.44 m off too (measured with a temporary probe, not committed), so the disagreement predates
fn-309; it sits inside the test's one-in-a-hundred allowance and is reported, not fixed here.

**Device digests** (`digest_tests::digest`, ignored, every shipped preset at seeds 1, 7 and 42,
24 lines): base and change identical, sha256 of the extracted lines
`abe5137697e37bccd31ddf8f3209cc3432fd586c83bf8cafa766daf50863a59b` on both. Raw logs in
`raw/digest-base.log`, `raw/digest-change.log`, `raw/organ-base.log`, `raw/organ-change2.log`
(ignored).

```
ordinary 1: leaves bc2c3f70b592dea6 points 93976 5a697b766a94750a clusters 8283 698bc8f6391c3458 sections cbf29ce484222325 demand d2bfbca7a34afe5b
ordinary 7: leaves fd1d748be946d8db points 141692 b54d206a988a3e08 clusters 12374 788f71d16a3eff05 sections cbf29ce484222325 demand 92d5bb8a3889c741
ordinary 42: leaves c1037be404440f8d points 165579 dbc3e91b76924064 clusters 14523 d5fa9086a76b7639 sections cbf29ce484222325 demand 1b503d79f974b1e8
oregon-white-oak 1: leaves a8a220be198e5055 points 301764 71a115d139de903b clusters 27157 356b9ced3bae551a sections cbf29ce484222325 demand de26a584d957921b
oregon-white-oak 7: leaves f288f6bc25fbc0ad points 421025 91ad7b0f55c6aaa6 clusters 38410 fb8aaac9924952a3 sections cbf29ce484222325 demand d813f5e502d96e66
oregon-white-oak 42: leaves 6234d8ae7812e73d points 312369 f97806d1e79efed2 clusters 27879 068aa0d763cd7e15 sections cbf29ce484222325 demand ecf51be3f8bfb02a
norway-spruce 1: leaves 36814ac4856202ce points 1195455 fdfe12f95d811480 clusters 206416 06110305c581fdc5 sections cbf29ce484222325 demand 3c674435e8ac4430
norway-spruce 7: leaves bbcb8a9ae8600383 points 1157978 db63cb3f201b6ddc clusters 197653 93dcbb06aae3583d sections cbf29ce484222325 demand b3b163261305d1c2
norway-spruce 42: leaves be72896aabd0b906 points 1206281 c6ab8b7e8a19ce66 clusters 206593 015d59f64ec0967c sections cbf29ce484222325 demand 5c8610f3d3b4d693
european-beech 1: leaves 30975665f63fce9a points 124065 a4ccc19879ab8e1a clusters 9945 efa0ae4b95cf40f9 sections cbf29ce484222325 demand f58be76e254fec17
european-beech 7: leaves 8ac35e4fbb5fa69c points 222705 a7dc329a66dbc3d0 clusters 18256 a4ea5964589bed96 sections cbf29ce484222325 demand 8ce9644a0f8635bc
european-beech 42: leaves 4087114f33a04ee0 points 272892 1b052d69513e232b clusters 22416 c4547b16531d0fc0 sections cbf29ce484222325 demand e762474b157d7661
silver-birch 1: leaves 4bb0c70db0745935 points 267523 d2edef987b96b609 clusters 26389 799b584745790318 sections cbf29ce484222325 demand 7040b664820f41a4
silver-birch 7: leaves 5096d87ed3a633a2 points 392259 43d11484e4354c53 clusters 39012 884d7907475f866b sections cbf29ce484222325 demand 6598d7ffc5f7b128
silver-birch 42: leaves 57730e69a59faa49 points 332297 6771f59a81cd572d clusters 33155 af3bb2b69d71bf74 sections cbf29ce484222325 demand 8ede4db94a364731
date-palm 1: leaves 90852933cbba045d points 7919 1f736d1315f4d15f clusters 504 d4efe567fa446e0f sections 04a151c295a78e32 demand 9d73367fcd7187d5
date-palm 7: leaves f54a4cf9aff1ca9e points 7919 b0239ce5758461d8 clusters 504 6ce5b45254f92448 sections 3fe465e3fc855f83 demand 74c035aee7290e8a
date-palm 42: leaves efc2c2b49a9940cb points 7919 b18c1b917acefb2b clusters 504 2306b7ca22ce2e28 sections 22758f0f44678fd2 demand b902e24365ef5003
telperion 1: leaves 074c08034dace011 points 2131692 0f9ce362361b41ff clusters 158540 592c924c08d1a56f sections cbf29ce484222325 demand f3ccabccd11c3940
telperion 7: leaves d81ce724d3dbb8fa points 3606433 1b4750d730bf650a clusters 268391 1842d2159303d955 sections cbf29ce484222325 demand 439dc25b32d94526
telperion 42: leaves 2255857e3f92bfe4 points 3606864 c3d13b80f2af1b45 clusters 268771 eb6a0ad2f2308dfa sections cbf29ce484222325 demand d4724af53ac70807
laurelin 1: leaves f9c0b856a247fd0d points 1996298 0868c11df070a12c clusters 148507 296648cac2d1917f sections cbf29ce484222325 demand afcf34beaa7eee10
laurelin 7: leaves 9760e8df26d9e5dd points 3358374 1dc9f42411a0b566 clusters 250258 2149897a4ffe4b90 sections cbf29ce484222325 demand d11700368b5348b8
laurelin 42: leaves b7c5b48b097e6c4b points 3375803 68bac691226cb55d clusters 252436 835b2bff0e47e9b4 sections cbf29ce484222325 demand 6ae9b815ff94e983
```

## Owner verdict on frame 526 (2026-10-23)

Frame 526 (oak to palm at 40, seed 7, t = 0.34), hero pose over one fixed box, base e2c035e51 beside change 129e363df: `stills-526.jpg` (committed; the PNG is `raw/stills/f526-before-after.png`, gitignored; built by the scratch example kept as `raw/stills/f526_still.rs.txt`). The still builds its tree on the CPU. On the base, leaves fly above and around the crown and the bounds reach 13.38 m; on the change none fly, the bounds top is 11.98 m, and the oak's twig leaves gather as tufts at the twig ends. Owner, as relayed by the host: "better".

## Task 3: cost (2026-10-23)

`build_time` (device, warm), `BUILD_PRESETS=date-palm BUILD_FRAME=oregon-white-oak,date-palm,0.34 BUILD_AGE=40 BUILD_SEED=7`, three runs each, one scratch checkout with its own target (base e2c035e51, change 77a2727dd), through `telperion-heavy 8 --gpu 3 --threads 2`:

| Build | Base warm ms (runs) | Change warm ms (runs) | Instances | Device peak MB |
|---|--:|--:|--:|--:|
| oak to palm t 0.34, 40, seed 7 | 138 (140, 138, 138) | 139 (137, 139, 142) | 845 | 87 |
| date palm, 40, seed 7 | 79 (78, 79, 79) | 78 (76, 78, 78) | 6,554 | 34 |

The medians are within a millisecond of each other, inside the run-to-run spread. The tuft costs nothing measurable: the same leaves are placed, at one point instead of along a line.

`npm ci && npm test` (inside `telperion-heavy 8 --threads 2`, after task 2's gate): 22 files, 177 tests passed.

