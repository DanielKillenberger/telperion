## Goal

A beech carries most of its leaves on short spur shoots along its older limbs and deep in the crown. Telperion grew only long shoots, so the few big limbs the owner asked for (fn-45) left the inside of the crown bare and each twig read as a fern's pinna. This spec adds short shoots as leaf-cluster placements on existing wood below a radius, sized and spaced by rows, costing foliage instances and no skeleton nodes; neutral grows none. Judged on the beech's pairs.

## Decisions

- user (2026-09-15, on the beech winter pair): "Too many small branches. It does seem like there's less density on the reference actually. Fewer larger branches compared to ours which has many more thinner ones directly attached to the trunk"
- host read, fn-45 round 6b (2026-09-15): "The leaf mass is a flat-topped umbrella on the top third; the lower two thirds of the crown are bare grey limbs ... The foliage reads as feathery fronds."
- worker, fn-45 round 6c (2026-09-15): "The leaf mass stops at about a quarter of the height; the photograph's reaches about 2 m ... twig wood only grows at limb tips and on thin wood ... A twig is a fixed 25 cm with a leaf every 2 cm, which is a fern's pinna; a beech's shoots are nearer 10 cm ... A shoot that carries a cluster of leaves without a node per leaf would be generator work, not a row."
- Short shoots are placements (own module `foliage/short_shoots.rs`), not skeleton; the node ceiling is untouched. No epicormic sprouts, no seasonal development, no leaf-mass lighting.

## Acceptance Criteria

- **R1:** five canopy rows (spacing, radius band, length, leaves per cluster, spread), neutral spacing 0, refused by name, wire, blend, metadata, harness; presets byte-identical at neutral — met.
- **R2:** clusters on eligible wood above the crown base, retained by the canopy's rules, no node added — met (node count identical on and off, asserted).
- **R3:** beech states short shoots; pairs rendered beside the previous round; visual QA; owner judges in fn-34 — met (round 12, `round12-fn50` rows in `.flow/evidence/fn34/rounds.tsv`; 48/48; beech 151,319 to 200,476 nodes, 4.25 to 5.60 million leaves inside the fidelity band).
- **R4:** tests for neutral identity, rails, eligible wood only, no node, determinism, cluster count, instance budget, blend walk — met.

## Result

Beech: a spur every 2.5 cm on wood under 0.35 of the trunk's radius, 5 cm long, five leaves fanned level; plus, by host direction on round 11's conifer read, twig angle 50, golden-angle divergence, a leaf every 5 cm, leaf lean 0.1. Host read of B-WHOLE: fronds and spiky sprays gone; one rounded dense broadleaf mass leafed through its inside; very dark without fn-52's canopy lighting; still a uniform mass without billowy lobes, leaves no lower than ~3.5 m at the sides against the photograph's 2 m. Specimen reads and wasm `specimen_read` carry no short shoots (only the growth view's mesh). The beech's own verdict moved to fn-62; the owner accepted the silver birch at fn-34 round 25 (2026-09-16).

## Open

None here; the beech's verdict is fn-62's.
