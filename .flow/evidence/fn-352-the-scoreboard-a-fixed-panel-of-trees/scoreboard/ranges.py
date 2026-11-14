"""fn-352: the published ranges the scoreboard sets beside its measures.

Only values fn-339 (`R2-REAL-TREES.md`) or fn-341 (`R1-MEASURES.md`) checked
against their sources' own text, copied as those files give them, each with
its source; a value they mark unchecked says so. Keys are (preset, measure,
age), age None for every age. Nothing here is a target: a stand value is not
an open-grown tree's, and the notes say which is which.
"""
F339 = "fn-339 R2-REAL-TREES.md"
F341 = "fn-341 R1-MEASURES.md"
OAK, BEECH, SPRUCE = "oregon-white-oak", "european-beech", "norway-spruce"

WHITE = "Q. robur open-grown {} m (White 1998, Table 1a, checked; " + F341 + ")"
GOULD = "Q. garryana ceiling {} m at the most open-grown growth (Gould et al. 2008, checked; " + F341 + ")"
PRETZSCH = ("open-grown Q. robur crown {} m for White's DBH at that age (Pretzsch et al. 2015 "
            "95% quantile, checked; spread inferred; " + F341 + "; compare with the two axes' mean)")

PUBLISHED = {
    (OAK, "dbh", 40): [WHITE.format("0.28 to 0.40"), GOULD.format("0.26"),
                       "Q. garryana, Corvallis: 0.23 +- 0.01 m at 10 m and 40 years "
                       "(Phillips et al. 2003, Table 1, checked; growing condition not read; " + F341 + ")"],
    (OAK, "dbh", 80): [WHITE.format("0.56 to 0.79"), GOULD.format("0.52")],
    (OAK, "dbh", 120): [WHITE.format("0.83 to 1.09"), GOULD.format("0.77")],
    (OAK, "dbh", 300): [WHITE.format("1.56 to 1.92"), GOULD.format("1.94"),
                        "Q. garryana, Corvallis: 0.56 +- 0.05 m at 25 m and 250 years "
                        "(Phillips et al. 2003, checked; " + F341 + ")"],
    (OAK, "height", None): ["Q. garryana, forest: 14 m at 47 to 70 years, 24 m at 95 to 135 years; "
                            "mature on favourable sites 15 to 27 m; record 36.6 m; potential maximum "
                            "on a good site 34.8 m (Stein 1990 Silvics; Gould et al. 2011; checked; "
                            + F341 + ")"],
    (OAK, "spread_x", 40): [PRETZSCH.format("7.2 to 9.6")],
    (OAK, "spread_x", 80): [PRETZSCH.format("12.7 to 16.8")],
    (OAK, "spread_x", 120): [PRETZSCH.format("17.4 to 21.8")],
    (OAK, "spread_x", 300): [PRETZSCH.format("29.3 to 34.6")],
    (OAK, "crown_ratio", None): ["open Q. garryana woodland, mean crown ratio 0.80, a stand mean "
                                 "(Gould et al. 2008, checked; " + F341 + ")"],
    (OAK, "lai", None): ["Q. robur, one Belgian stand, 2.4 to 3.3 (Bequet et al. 2011 via "
                         "Packham et al. 2012, UNCHECKED; " + F339 + "); Q. robur with ash and "
                         "lime, stand LAI 5 (Cermak 1998, abstract; a mixed stand; " + F339 + ")"],
    (BEECH, "dbh", 40): ["F. sylvatica open-grown 0.32 to 0.48 m (White 1998, checked; " + F341 + ")"],
    (BEECH, "dbh", 80): ["F. sylvatica open-grown 0.63 to 0.96 m (White 1998, checked; " + F341 + ")"],
    (BEECH, "lai", None): ["closed mature stands, one-sided, 7.4 (5.6 to 9.5) (Leuschner et al. 2006, "
                           "abstract; " + F339 + "); optical 7.1 (2.8 to 11.0) primeval, 7.4 (4.3 to "
                           "11.2) production forests (Glatthorn et al. 2017; " + F339 + ")"],
    (SPRUCE, "new_shoots", None): ["closed planted stand, about 36 years, 15.5 m: 691 to 2,372 new "
                                   "shoots a tree a year, mean 1,707 (Bellan et al. 2019, full text; "
                                   + F339 + ")"],
    (SPRUCE, "lai", None): ["28-year plantation, projected LAI 10.5 (Bolstad and Gower 1990, "
                            "abstract; " + F339 + "); the scoreboard's needle area is each needle's "
                            "whole surface, not projected"],
}
