# fn-309 task 1: the classes before and after

`crates/telperion-core/tests/walk_flying.rs`, run as
`cargo test --profile ci -p telperion-core --test walk_flying -- --ignored --nocapture --test-threads 1`
through `telperion-heavy 8 --threads 2`. Oak to palm at 40, `seat` 0.1 m, a pixel 26.8 mm (the
film's camera at 28.03 m). Columns: (a) on wood at least a pixel, (b) on wood under a pixel, (c) on
a drawn rachis at least and under a pixel, (d) a station- or cluster-borne leaf more than `seat` off
drawn wood, (e) any leaf more than `seat` off every drawn element. Raw logs, ignored:
`raw/walk_flying-base.txt`, `raw/walk_flying-change.txt`.

## Base (975a70eea): 13 of 13 frames hold flying leaves

```
from	to	age	seed	t	leaves	a	b	c>=1px	c<1px	d	e	farthest m
oregon-white-oak	date-palm	40	7	0.129	3937	209	1809	0	25	1901	1894	0.90
oregon-white-oak	date-palm	40	7	0.227	4139	177	1389	6	120	2513	2447	1.32
oregon-white-oak	date-palm	40	7	0.34	845	12	51	25	92	681	665	2.43
oregon-white-oak	date-palm	40	7	0.463	2476	118	46	160	188	2061	1964	3.04
oregon-white-oak	date-palm	40	1	0.25	3407	100	711	7	91	2538	2498	1.59
oregon-white-oak	date-palm	40	1	0.5	4936	230	178	277	244	4193	4007	3.37
oregon-white-oak	date-palm	40	1	0.75	3784	600	17	839	429	1963	1899	2.69
oregon-white-oak	date-palm	40	7	0.25	4096	147	1162	12	124	2715	2651	1.68
oregon-white-oak	date-palm	40	7	0.5	3954	190	103	283	221	3329	3157	3.12
oregon-white-oak	date-palm	40	7	0.75	3218	573	12	810	367	1483	1456	3.15
oregon-white-oak	date-palm	40	42	0.25	1072	17	185	2	65	818	803	1.76
oregon-white-oak	date-palm	40	42	0.5	4977	191	174	302	227	4275	4083	2.65
oregon-white-oak	date-palm	40	42	0.75	2704	405	7	853	445	1054	994	2.99
```

## Change: (d) and (e) are 0 on every frame

The placements' leaflets stand at their points, so the leaf count of every frame is unchanged.

```
from	to	age	seed	t	leaves	a	b	c>=1px	c<1px	d	e	farthest m
oregon-white-oak	date-palm	40	7	0.129	3937	27	3894	0	16	0	0	0.00
oregon-white-oak	date-palm	40	7	0.227	4139	64	4035	3	37	0	0	0.00
oregon-white-oak	date-palm	40	7	0.34	845	5	740	20	80	0	0	0.00
oregon-white-oak	date-palm	40	7	0.463	2476	40	2201	97	138	0	0	0.00
oregon-white-oak	date-palm	40	1	0.25	3407	13	3349	0	45	0	0	0.00
oregon-white-oak	date-palm	40	1	0.5	4936	869	3704	211	152	0	0	0.00
oregon-white-oak	date-palm	40	1	0.75	3784	1720	868	782	414	0	0	0.00
oregon-white-oak	date-palm	40	7	0.25	4096	28	4014	0	54	0	0	0.00
oregon-white-oak	date-palm	40	7	0.5	3954	644	2968	191	151	0	0	0.00
oregon-white-oak	date-palm	40	7	0.75	3218	1115	972	764	367	0	0	0.00
oregon-white-oak	date-palm	40	42	0.25	1072	3	1023	0	46	0	0	0.00
oregon-white-oak	date-palm	40	42	0.5	4977	565	4043	219	150	0	0	0.00
oregon-white-oak	date-palm	40	42	0.75	2704	883	487	901	433	0	0	0.00
```

Measured: both tables. The base's frame 526 (t 0.34, seed 7) counts 665 leaves in (e), against the
host's 687 from the probe, which measured to the nearest ring's sphere where this test measures to
the tapered capsule between rings.
