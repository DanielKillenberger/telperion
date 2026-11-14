# fn-339 R3: the full tables of the read that counts

Printed by `probes/r3.py` from `raw/r3b-*.log` (ignored), the corrected probe in `probes/replay.patch`; quoted by `R3-RESULT.md`.

## oregon-white-oak, age 40, seeds 1001 to 1012 (12 trees)

Cohort sets: 0.25 m [14]; 0.5 m [13, 14]; 1 m [13, 14]
Lattice check: 5447473 apex reads, largest difference 0.0e+00.

Shares of the cycle's units inside the cohorts (median over seeds, min to max):

| tree age | at 0.25 m | at 0.5 m | at 1.0 m |
|---|---|---|---|
| 10 | 0.755 (0.731 to 0.776) | 0.952 (0.944 to 0.959) | 0.952 (0.944 to 0.959) |
| 20 | 0.779 (0.768 to 0.802) | 0.948 (0.940 to 0.958) | 0.948 (0.940 to 0.958) |
| 30 | 0.727 (0.688 to 0.776) | 0.931 (0.915 to 0.947) | 0.931 (0.915 to 0.947) |
| 40 | 0.694 (0.628 to 0.750) | 0.925 (0.900 to 0.944) | 0.925 (0.900 to 0.944) |

Light bins holding at least 5% of bearing units (median over seeds): ['0.0-0.2', '0.2-0.4', '0.4-0.6', '0.6-0.8'].

Law over cycle, per tree: median over seeds (min to max); then each counted light bin's median; sorted by PROTOCOL.md's rule.

| quantity | law reading | per tree | light bins | sorted |
|---|---|---|---|---|
| grew | root-1 | 0.995 (0.967 to 1.000) | 0.98, 0.97, 1.04, 1.06 | agrees |
| grew | subtree-1 | 1.006 (0.984 to 1.017) | 1.00, 0.98, 1.05, 1.07 | agrees |
| grew | subtree-size | 1.006 (0.984 to 1.017) | 1.00, 0.98, 1.05, 1.07 | agrees |
| grew | root-size | 0.995 (0.967 to 1.000) | 0.98, 0.97, 1.04, 1.06 | agrees |
| grew | subtree-1-drawn | 0.979 (0.961 to 0.997) | 0.98, 0.96, 1.00, 1.02 | agrees |
| living | root-1 | 0.735 (0.663 to 0.800) | 0.62, 0.79, 0.89, 0.80 | differs |
| living | subtree-1 | 0.735 (0.663 to 0.800) | 0.62, 0.79, 0.89, 0.80 | differs |
| living | subtree-size | 0.735 (0.663 to 0.800) | 0.62, 0.79, 0.89, 0.80 | differs |
| living | root-size | 0.735 (0.663 to 0.800) | 0.62, 0.79, 0.89, 0.80 | differs |
| living | subtree-1-drawn | 0.718 (0.647 to 0.788) | 0.61, 0.78, 0.85, 0.79 | differs |
| standing | root-1 | 0.808 (0.726 to 0.886) | 0.74, 0.92, 0.94, 0.82 | differs |
| standing | subtree-1 | 0.839 (0.751 to 0.922) | 0.77, 0.95, 0.98, 0.83 | differs |
| standing | subtree-size | 0.839 (0.751 to 0.922) | 0.77, 0.95, 0.98, 0.83 | differs |
| standing | root-size | 0.808 (0.726 to 0.886) | 0.74, 0.92, 0.94, 0.82 | differs |
| standing | subtree-1-drawn | 0.825 (0.736 to 0.901) | 0.76, 0.93, 0.95, 0.81 | differs |
| leaf_cur | root-1 | 0.896 (0.854 to 0.924) | 1.58, 0.79, 0.59, 0.51 | differs |
| leaf_cur | subtree-1 | 0.908 (0.864 to 0.943) | 1.60, 0.80, 0.59, 0.52 | differs |
| leaf_cur | subtree-size | 1.143 (1.053 to 1.230) | 1.20, 1.14, 1.08, 1.11 | differs |
| leaf_cur | root-size | 1.126 (1.033 to 1.214) | 1.18, 1.12, 1.07, 1.09 | differs |
| leaf_cur | subtree-1-drawn | 0.881 (0.841 to 0.934) | 1.60, 0.79, 0.56, 0.50 | differs |
| leaf_drawn | root-1 | 0.896 (0.854 to 0.924) | 1.58, 0.79, 0.59, 0.51 | differs |
| leaf_drawn | subtree-1 | 0.908 (0.864 to 0.943) | 1.60, 0.80, 0.59, 0.52 | differs |
| leaf_drawn | subtree-size | 1.143 (1.053 to 1.230) | 1.20, 1.14, 1.08, 1.11 | differs |
| leaf_drawn | root-size | 1.126 (1.033 to 1.214) | 1.18, 1.12, 1.07, 1.09 | differs |
| leaf_drawn | subtree-1-drawn | 0.881 (0.841 to 0.934) | 1.60, 0.79, 0.56, 0.50 | differs |
| wood | root-1 | 0.724 (0.703 to 0.750) | 0.98, 0.64, 0.49, 0.42 | differs |
| wood | subtree-1 | 0.824 (0.775 to 0.870) | 1.12, 0.71, 0.54, 0.49 | differs |
| wood | subtree-size | 0.991 (0.948 to 1.053) | 0.98, 1.01, 0.98, 0.95 | agrees |
| wood | root-size | 0.888 (0.810 to 0.940) | 0.81, 0.93, 0.92, 0.84 | differs |
| wood | subtree-1-drawn | 0.802 (0.762 to 0.859) | 1.10, 0.68, 0.52, 0.46 | differs |
| section | root-1 | 0.404 (0.340 to 0.487) | 0.79, 0.36, 0.21, 0.17 | differs |
| section | subtree-1 | 0.457 (0.377 to 0.572) | 0.91, 0.40, 0.23, 0.18 | differs |
| section | subtree-size | 0.912 (0.828 to 1.032) | 0.99, 0.95, 0.82, 0.77 | differs |
| section | root-size | 0.820 (0.716 to 0.922) | 0.79, 0.86, 0.77, 0.67 | differs |
| section | subtree-1-drawn | 0.505 (0.420 to 0.640) | 1.02, 0.44, 0.25, 0.20 | differs |
| join | root-1 | 0.403 (0.339 to 0.487) | 0.79, 0.36, 0.21, 0.16 | differs |
| join | subtree-1 | 0.456 (0.377 to 0.571) | 0.91, 0.40, 0.23, 0.18 | differs |
| join | subtree-size | 0.914 (0.828 to 1.034) | 0.99, 0.95, 0.82, 0.77 | differs |
| join | root-size | 0.822 (0.717 to 0.924) | 0.79, 0.86, 0.78, 0.68 | differs |
| join | subtree-1-drawn | 0.504 (0.420 to 0.640) | 1.01, 0.44, 0.25, 0.19 | differs |

Roots the balance hazard alone sheds (law, subtree-1), per tree: median 0, most 0.338.

Size trace, per cohort age, summed over seeds: mean size of the last cycle's units weighted by one, by the survival's presence, by the pencil's presence, and by the settled scale (which holds the size):

| age | units | by one | by survival | by pencil | by settled scale |
|---|---|---|---|---|---|
| 13 | 76109 | 0.931 | 0.931 | 1.036 | 1.498 |
| 14 | 223045 | 1.070 | 1.072 | 1.141 | 1.519 |

Per lattice node (subtree-1 against the cycle; median over seeds of each tree's figure):

| quantity | median ratio | p10 | p90 | share within 25% |
|---|---|---|---|---|
| grew | 1.009 | 0.628 | 1.816 | 0.504 |
| leaf_cur | 0.836 | 0.350 | 2.203 | 0.240 |
| wood | 0.721 | 0.313 | 1.787 | 0.254 |
| section | 0.429 | 0.124 | 2.118 | 0.123 |

Whole tree beside R2 (median over seeds, min to max). Law = the structure's own plus the law's cohorts (subtree-1). The index is over the tree's own leaf-bearing 0.5 m columns: a lone tree's, not a stand's.

| measure | cycle | law | R2 checked | cycle against R2 | law against R2 | nearer |
|---|---|---|---|---|---|---|
| new shoots, weighted | 2.178e+04 (1.478e+04 to 2.439e+04) | 2.194e+04 (1.501e+04 to 2.463e+04) | no checked source | - | - | - |
| new shoots, unweighted (cycle) | 2.695e+04 (1.791e+04 to 3.479e+04) | - | no checked source | - | - | - |
| current-year leaf, m2 | 182.7 (120.9 to 215.1) | 166.9 (113 to 188.4) | no checked source | - | - | - |
| standing leaf, m2 | 182.7 (120.9 to 215.1) | 166.9 (113 to 188.4) | no checked source | - | - | - |
| leaf area index | 3.962 (3.263 to 4.349) | 3.654 (3.206 to 4.046) | no checked source | - | - | - |

## norway-spruce, age 40, seeds 1001 to 1012 (12 trees)

Cohort sets: 0.25 m [12]; 0.5 m [12]; 1 m [10, 12]
Lattice check: 17499851 apex reads, largest difference 0.0e+00.

Shares of the cycle's units inside the cohorts (median over seeds, min to max):

| tree age | at 0.25 m | at 0.5 m | at 1.0 m |
|---|---|---|---|
| 10 | 0.809 (0.783 to 0.817) | 0.809 (0.783 to 0.817) | 0.989 (0.983 to 0.991) |
| 20 | 0.849 (0.847 to 0.851) | 0.849 (0.847 to 0.851) | 0.997 (0.997 to 0.997) |
| 30 | 0.835 (0.834 to 0.838) | 0.835 (0.834 to 0.838) | 0.998 (0.998 to 0.998) |
| 40 | 0.820 (0.818 to 0.823) | 0.820 (0.818 to 0.823) | 0.998 (0.998 to 0.998) |

Light bins holding at least 5% of bearing units (median over seeds): ['0.2-0.4', '0.4-0.6'].

Law over cycle, per tree: median over seeds (min to max); then each counted light bin's median; sorted by PROTOCOL.md's rule.

| quantity | law reading | per tree | light bins | sorted |
|---|---|---|---|---|
| grew | root-1 | 1.012 (1.009 to 1.017) | 1.02, 1.01 | agrees |
| grew | subtree-1 | 1.012 (1.009 to 1.017) | 1.02, 1.01 | agrees |
| grew | subtree-size | 1.012 (1.009 to 1.017) | 1.02, 1.01 | agrees |
| grew | root-size | 1.012 (1.009 to 1.017) | 1.02, 1.01 | agrees |
| grew | subtree-1-drawn | 1.006 (1.003 to 1.009) | 1.01, 1.01 | agrees |
| living | root-1 | 0.771 (0.762 to 0.781) | 0.89, 0.75 | differs |
| living | subtree-1 | 0.771 (0.762 to 0.781) | 0.89, 0.75 | differs |
| living | subtree-size | 0.771 (0.762 to 0.781) | 0.89, 0.75 | differs |
| living | root-size | 0.771 (0.762 to 0.781) | 0.89, 0.75 | differs |
| living | subtree-1-drawn | 0.764 (0.757 to 0.773) | 0.88, 0.74 | differs |
| standing | root-1 | 0.906 (0.900 to 0.911) | 0.98, 0.87 | agrees |
| standing | subtree-1 | 0.906 (0.900 to 0.911) | 0.98, 0.87 | agrees |
| standing | subtree-size | 0.906 (0.900 to 0.911) | 0.98, 0.87 | agrees |
| standing | root-size | 0.906 (0.900 to 0.911) | 0.98, 0.87 | agrees |
| standing | subtree-1-drawn | 0.899 (0.895 to 0.905) | 0.97, 0.86 | differs |
| leaf_cur | root-1 | 1.012 (1.009 to 1.017) | 1.02, 1.01 | agrees |
| leaf_cur | subtree-1 | 1.012 (1.009 to 1.017) | 1.02, 1.01 | agrees |
| leaf_cur | subtree-size | 1.012 (1.009 to 1.017) | 1.02, 1.01 | agrees |
| leaf_cur | root-size | 1.012 (1.009 to 1.017) | 1.02, 1.01 | agrees |
| leaf_cur | subtree-1-drawn | 1.006 (1.003 to 1.009) | 1.01, 1.01 | agrees |
| leaf_drawn | root-1 | 1.011 (1.009 to 1.014) | 1.01, 1.01 | agrees |
| leaf_drawn | subtree-1 | 1.011 (1.009 to 1.014) | 1.01, 1.01 | agrees |
| leaf_drawn | subtree-size | 1.011 (1.009 to 1.014) | 1.01, 1.01 | agrees |
| leaf_drawn | root-size | 1.011 (1.009 to 1.014) | 1.01, 1.01 | agrees |
| leaf_drawn | subtree-1-drawn | 1.005 (1.003 to 1.006) | 1.01, 1.01 | agrees |
| wood | root-1 | 1.011 (1.009 to 1.014) | 1.01, 1.01 | agrees |
| wood | subtree-1 | 1.011 (1.009 to 1.014) | 1.01, 1.01 | agrees |
| wood | subtree-size | 1.011 (1.009 to 1.014) | 1.01, 1.01 | agrees |
| wood | root-size | 1.011 (1.009 to 1.014) | 1.01, 1.01 | agrees |
| wood | subtree-1-drawn | 1.005 (1.003 to 1.006) | 1.01, 1.01 | agrees |
| section | root-1 | 0.948 (0.945 to 0.951) | 0.94, 0.95 | agrees |
| section | subtree-1 | 0.948 (0.945 to 0.951) | 0.94, 0.95 | agrees |
| section | subtree-size | 0.948 (0.945 to 0.951) | 0.94, 0.95 | agrees |
| section | root-size | 0.948 (0.945 to 0.951) | 0.94, 0.95 | agrees |
| section | subtree-1-drawn | 1.006 (1.005 to 1.007) | 1.01, 1.01 | agrees |
| join | root-1 | 0.948 (0.945 to 0.951) | 0.94, 0.95 | agrees |
| join | subtree-1 | 0.948 (0.945 to 0.951) | 0.94, 0.95 | agrees |
| join | subtree-size | 0.948 (0.945 to 0.951) | 0.94, 0.95 | agrees |
| join | root-size | 0.948 (0.945 to 0.951) | 0.94, 0.95 | agrees |
| join | subtree-1-drawn | 1.006 (1.005 to 1.007) | 1.01, 1.01 | agrees |

Roots the balance hazard alone sheds (law, subtree-1), per tree: median 0, most 0.

Size trace, per cohort age, summed over seeds: mean size of the last cycle's units weighted by one, by the survival's presence, by the pencil's presence, and by the settled scale (which holds the size):

| age | units | by one | by survival | by pencil | by settled scale |
|---|---|---|---|---|---|
| 12 | 941859 | 1.000 | 1.000 | 1.000 | 1.000 |

Per lattice node (subtree-1 against the cycle; median over seeds of each tree's figure):

| quantity | median ratio | p10 | p90 | share within 25% |
|---|---|---|---|---|
| grew | 1.014 | 0.882 | 1.191 | 0.906 |
| leaf_cur | 1.014 | 0.882 | 1.191 | 0.906 |
| wood | 1.012 | 0.913 | 1.120 | 0.952 |
| section | 0.943 | 0.817 | 1.034 | 0.948 |

Whole tree beside R2 (median over seeds, min to max). Law = the structure's own plus the law's cohorts (subtree-1). The index is over the tree's own leaf-bearing 0.5 m columns: a lone tree's, not a stand's.

| measure | cycle | law | R2 checked | cycle against R2 | law against R2 | nearer |
|---|---|---|---|---|---|---|
| new shoots, weighted | 6.465e+04 (6.055e+04 to 7.283e+04) | 6.531e+04 (6.115e+04 to 7.344e+04) | new shoots a tree a year, ~36 y, Bellan 2019 | 27.3 | 27.5 | cycle |
| new shoots, unweighted (cycle) | 9.472e+04 (8.83e+04 to 1.056e+05) | - | no checked source | - | - | - |
| current-year leaf, m2 | 82.02 (77.01 to 93.04) | 82.9 (77.81 to 93.86) | no checked source | - | - | - |
| standing leaf, m2 | 441.6 (411.4 to 489.4) | 444.6 (415.1 to 492.7) | no checked source | - | - | - |
| leaf area index | 16.65 (15.07 to 17.81) | 16.77 (15.18 to 17.98) | projected LAI, 28 y plantation, Bolstad and Gower 1990 | 1.59 | 1.6 | cycle |

## european-beech, age 40, seeds 1001 to 1012 (12 trees)

Cohort sets: 0.25 m [14]; 0.5 m [12, 14]; 1 m [12, 14]
Lattice check: 1022850 apex reads, largest difference 0.0e+00.

Shares of the cycle's units inside the cohorts (median over seeds, min to max):

| tree age | at 0.25 m | at 0.5 m | at 1.0 m |
|---|---|---|---|
| 10 | 0.753 (0.713 to 0.775) | 0.932 (0.902 to 0.946) | 0.932 (0.902 to 0.946) |
| 20 | 0.801 (0.765 to 0.835) | 0.932 (0.903 to 0.959) | 0.932 (0.903 to 0.959) |
| 30 | 0.767 (0.736 to 0.780) | 0.922 (0.911 to 0.932) | 0.922 (0.911 to 0.932) |
| 40 | 0.782 (0.754 to 0.789) | 0.937 (0.915 to 0.941) | 0.937 (0.915 to 0.941) |

Light bins holding at least 5% of bearing units (median over seeds): ['0.4-0.6', '0.6-0.8', '0.8-1.0'].

Law over cycle, per tree: median over seeds (min to max); then each counted light bin's median; sorted by PROTOCOL.md's rule.

| quantity | law reading | per tree | light bins | sorted |
|---|---|---|---|---|
| grew | root-1 | 0.911 (0.882 to 0.928) | 0.85, 0.91, 1.00 | agrees |
| grew | subtree-1 | 1.007 (0.993 to 1.040) | 1.02, 1.00, 1.04 | agrees |
| grew | subtree-size | 1.007 (0.993 to 1.040) | 1.02, 1.00, 1.04 | agrees |
| grew | root-size | 0.911 (0.882 to 0.928) | 0.85, 0.91, 1.00 | agrees |
| grew | subtree-1-drawn | 1.004 (0.993 to 1.041) | 0.99, 1.01, 1.01 | agrees |
| living | root-1 | 0.753 (0.677 to 0.852) | 0.68, 0.83, 0.78 | differs |
| living | subtree-1 | 0.792 (0.718 to 0.886) | 0.75, 0.87, 0.80 | differs |
| living | subtree-size | 0.792 (0.718 to 0.886) | 0.75, 0.87, 0.80 | differs |
| living | root-size | 0.753 (0.677 to 0.852) | 0.68, 0.83, 0.78 | differs |
| living | subtree-1-drawn | 0.785 (0.706 to 0.894) | 0.75, 0.88, 0.78 | differs |
| standing | root-1 | 0.745 (0.676 to 0.831) | 0.68, 0.83, 0.81 | differs |
| standing | subtree-1 | 0.854 (0.791 to 0.926) | 0.83, 0.93, 0.85 | differs |
| standing | subtree-size | 0.854 (0.791 to 0.926) | 0.83, 0.93, 0.85 | differs |
| standing | root-size | 0.745 (0.676 to 0.831) | 0.68, 0.83, 0.81 | differs |
| standing | subtree-1-drawn | 0.848 (0.777 to 0.932) | 0.82, 0.93, 0.83 | differs |
| leaf_cur | root-1 | 0.910 (0.874 to 0.927) | 0.85, 0.91, 1.00 | agrees |
| leaf_cur | subtree-1 | 1.012 (0.984 to 1.042) | 1.02, 1.01, 1.04 | agrees |
| leaf_cur | subtree-size | 1.012 (0.984 to 1.042) | 1.02, 1.01, 1.04 | agrees |
| leaf_cur | root-size | 0.910 (0.874 to 0.927) | 0.85, 0.91, 1.00 | agrees |
| leaf_cur | subtree-1-drawn | 1.008 (0.992 to 1.042) | 1.00, 1.01, 1.01 | agrees |
| leaf_drawn | root-1 | 0.909 (0.881 to 0.925) | 0.85, 0.91, 0.99 | agrees |
| leaf_drawn | subtree-1 | 1.008 (0.994 to 1.040) | 1.02, 1.01, 1.04 | agrees |
| leaf_drawn | subtree-size | 1.008 (0.994 to 1.040) | 1.02, 1.01, 1.04 | agrees |
| leaf_drawn | root-size | 0.909 (0.881 to 0.925) | 0.85, 0.91, 0.99 | agrees |
| leaf_drawn | subtree-1-drawn | 1.005 (0.993 to 1.040) | 0.99, 1.01, 1.01 | agrees |
| wood | root-1 | 0.711 (0.666 to 0.765) | 0.62, 0.75, 0.92 | differs |
| wood | subtree-1 | 0.956 (0.914 to 0.974) | 0.93, 0.95, 1.01 | agrees |
| wood | subtree-size | 0.956 (0.914 to 0.974) | 0.93, 0.95, 1.01 | agrees |
| wood | root-size | 0.711 (0.666 to 0.765) | 0.62, 0.75, 0.92 | differs |
| wood | subtree-1-drawn | 0.938 (0.919 to 0.963) | 0.91, 0.95, 0.99 | agrees |
| section | root-1 | 0.703 (0.672 to 0.754) | 0.62, 0.73, 0.87 | differs |
| section | subtree-1 | 0.930 (0.894 to 0.946) | 0.91, 0.92, 0.95 | agrees |
| section | subtree-size | 0.930 (0.894 to 0.946) | 0.91, 0.92, 0.95 | agrees |
| section | root-size | 0.703 (0.672 to 0.754) | 0.62, 0.73, 0.87 | differs |
| section | subtree-1-drawn | 0.959 (0.945 to 0.991) | 0.92, 0.97, 1.00 | agrees |
| join | root-1 | 0.702 (0.670 to 0.753) | 0.62, 0.73, 0.87 | differs |
| join | subtree-1 | 0.932 (0.894 to 0.950) | 0.91, 0.92, 0.96 | agrees |
| join | subtree-size | 0.932 (0.894 to 0.950) | 0.91, 0.92, 0.96 | agrees |
| join | root-size | 0.702 (0.670 to 0.753) | 0.62, 0.73, 0.87 | differs |
| join | subtree-1-drawn | 0.958 (0.944 to 0.991) | 0.92, 0.97, 1.00 | agrees |

Roots the balance hazard alone sheds (law, subtree-1), per tree: median 0, most 0.

Size trace, per cohort age, summed over seeds: mean size of the last cycle's units weighted by one, by the survival's presence, by the pencil's presence, and by the settled scale (which holds the size):

| age | units | by one | by survival | by pencil | by settled scale |
|---|---|---|---|---|---|
| 12 | 18013 | 1.000 | 1.000 | 1.000 | 1.000 |
| 14 | 87189 | 1.000 | 1.000 | 1.000 | 1.000 |

Per lattice node (subtree-1 against the cycle; median over seeds of each tree's figure):

| quantity | median ratio | p10 | p90 | share within 25% |
|---|---|---|---|---|
| grew | 0.998 | 0.625 | 1.755 | 0.511 |
| leaf_cur | 1.003 | 0.630 | 1.797 | 0.485 |
| wood | 0.959 | 0.553 | 1.746 | 0.485 |
| section | 0.895 | 0.541 | 1.611 | 0.477 |

Whole tree beside R2 (median over seeds, min to max). Law = the structure's own plus the law's cohorts (subtree-1). The index is over the tree's own leaf-bearing 0.5 m columns: a lone tree's, not a stand's.

| measure | cycle | law | R2 checked | cycle against R2 | law against R2 | nearer |
|---|---|---|---|---|---|---|
| new shoots, weighted | 4936 (2694 to 1.254e+04) | 5065 (2794 to 1.258e+04) | no checked source | - | - | - |
| new shoots, unweighted (cycle) | 8616 (4081 to 1.732e+04) | - | no checked source | - | - | - |
| current-year leaf, m2 | 31.64 (17.33 to 80.14) | 32.39 (17.97 to 80.37) | no checked source | - | - | - |
| standing leaf, m2 | 31.62 (17.36 to 80.09) | 32.39 (17.97 to 80.37) | no checked source | - | - | - |
| leaf area index | 1.118 (0.7927 to 1.8) | 1.145 (0.8083 to 1.796) | LAI 5.6 to 9.5, Leuschner 2006 (7.1, 7.4 Glatthorn 2017) | 0.2 | 0.205 | law |

