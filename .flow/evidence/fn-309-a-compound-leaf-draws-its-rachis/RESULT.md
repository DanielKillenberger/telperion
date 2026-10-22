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
