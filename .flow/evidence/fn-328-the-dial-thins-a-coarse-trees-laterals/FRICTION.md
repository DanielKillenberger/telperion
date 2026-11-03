# fn-328 friction

## 2 Nov 2026: three of four climbs never started

Doing: oak round 1, four climbs meant to run at once on four cores.

Slowed by: `search.py` claimed 8 GB of `telperion-heavy` per climb. The desk had room for one, so climbs 7, 8 and 9 sat in the queue for 1 h 40 min with no CPU, while I reported all four as running. I never checked after launch. A climb's scope peaked at 0.8 GB.

Cost: 1 h 40 min of wall time for three climbs. The first climb's own run was not affected.

Removed by: a claim of 2 GB in `search.py`, and reading the queue's output straight after starting parallel work.

## 3 Nov 2026: one slow climb held the whole round for five hours

Doing: spruce and birch round 1, four climbs started at 03:20 under one script that reports only when all four end.

Slowed by: the birch climbs ended at 04:05 and 04:21 and the spruce's first age 8 at 06:58, but the spruce's first age 9 ran to about 10:00, at 50 to 75 minutes a generation. From its first generation it cost 1.5 s against first age 8's 0.7 s, the same pattern the oak's round 2 had already shown for later first ages, so it could not have won. Nothing acted on the finished climbs meanwhile, and the birch's second round and the spruce's fifty-seed read waited.

Cost: about 5.5 hours of wall time for the birch, 3 hours for the spruce, and about 3 hours of one core on a climb with no chance.

Removed by: reporting each climb as it ends rather than the round as a whole, and dropping a climb whose cheapest row within 5% already costs more than a finished climb's.
