//! The palm's trunk organs: the retained leaf bases that clothe the trunk on
//! the crown's own spiral, and the wood they are hung on. fn-110.
use telperion_core::{
    bias::BiasParams,
    branching::{self, LeafBase},
    foliage,
    math::Vec3,
    mesh,
    presets::{Family, Preset},
    tree::{NodeKind, Tree},
};

fn family(id: &str, seed: u32) -> Family {
    let mut f = Preset::from_id(id)
        .unwrap_or_else(|| panic!("{id} is a preset"))
        .parameters();
    f.skeleton.seed = seed;
    f
}

/// Every retained base the tree carries as wood, newest first: the node its
/// run leaves the axis at, and the node that stands out of the bark.
fn hung(tree: &Tree) -> Vec<(usize, usize)> {
    let branch = |i: usize| tree.nodes[i].kind == NodeKind::Branch;
    (1..tree.nodes.len())
        .filter(|&i| branch(i) && !branch(tree.nodes[i].parent.unwrap() as usize))
        .map(|attach| {
            let tip = (attach + 1..tree.nodes.len())
                .find(|&i| tree.nodes[i].parent == Some(attach as u32))
                .expect("a base that leaves the axis stands out of the bark");
            (attach, tip)
        })
        .collect()
}

/// The apex of every stem, and the axis it stands on.
fn apices(tree: &Tree) -> Vec<(Vec3, Vec3)> {
    foliage::rosettes(tree)
        .into_iter()
        .map(|r| (r.at, r.axis))
        .collect()
}

/// R3: one retained base for every row the table states, on every stem, and
/// every one of them wood the crown's own placement never sees.
#[test]
fn the_palm_hangs_a_retained_base_for_every_row_its_table_states() {
    let f = family("date-palm", 1);
    assert!(f.canopy.leaf_bases > 0 && f.canopy.leaf_base_length > 0.);
    let tree = mesh::grow(&f).expect("the skeleton grows");
    let bases = hung(&tree);
    assert_eq!(
        bases.len(),
        f.skeleton.habit.stems as usize * f.canopy.leaf_bases as usize,
        "a clothed trunk is stems x bases and nothing besides"
    );
    for &(attach, tip) in &bases {
        for i in [attach, tip] {
            assert!(!tree.nodes[i].stem, "node {i} moved a stem apex");
            assert_eq!(tree.nodes[i].kind, NodeKind::Branch, "node {i} is not wood");
            assert!(tree.nodes[i].radius > 0., "node {i} carries no girth");
            assert!(tree.nodes[i].start_radius >= tree.nodes[i].radius);
        }
        let borne = tree.nodes[attach].parent.expect("a base is borne on wood") as usize;
        assert!(
            tree.nodes[borne].stem,
            "base {attach} is borne on node {borne}, which is not stem wood"
        );
    }
}

/// R3: the bases are the rosette's own history. Base `k`, counting down from
/// the crown, stands at `(rosette_fronds + k) * rosette_divergence` in the
/// frame the crown's own fronds are turned in, which is the crown's spiral
/// carried on down the trunk. Stated on a stem that stands straight up, where
/// the frame a base leaves on is the frame the apex stands in.
#[test]
fn the_bases_carry_the_crowns_own_spiral_down_the_trunk() {
    let mut f = family("date-palm", 1);
    f.skeleton.habit.crookedness = 0.;
    f.skeleton.habit.attractor_weight = 0.;
    f.skeleton.bias = BiasParams::NONE;
    let tree = mesh::grow(&f).expect("the skeleton grows");
    let table: Vec<LeafBase> = branching::leaf_bases(&tree, &f.canopy);
    assert_eq!(
        table.len(),
        f.skeleton.habit.stems as usize * f.canopy.leaf_bases as usize
    );
    let (_, axis) = apices(&tree)[0];
    let (normal, binormal) = foliage::frame(axis);
    for (k, base) in table.iter().enumerate() {
        assert!(
            base.radial.dot(axis).abs() < 1e-9,
            "base {k} does not leave the axis square to it"
        );
        let stood = base
            .radial
            .dot(binormal)
            .atan2(base.radial.dot(normal))
            .to_degrees()
            .rem_euclid(360.);
        let turn = f64::from(f.canopy.rosette_fronds + k as u32) * f.canopy.rosette_divergence;
        let expected = turn.rem_euclid(360.);
        let apart = (stood - expected).abs();
        assert!(
            apart.min(360. - apart) < 1e-6,
            "base {k} stands at {stood} degrees, the spiral asks for {expected}"
        );
    }
}

/// R3: the lattice is a spiral on the trunk the palm actually grows too. The
/// trunk wanders, and the crown's frame is carried onto the axis each base
/// leaves on rather than squared onto it, so no two bases in a row stand on
/// one radial and the lattice never bands.
#[test]
fn no_two_bases_in_a_row_stand_on_one_radial() {
    let f = family("date-palm", 1);
    let tree = mesh::grow(&f).expect("the skeleton grows");
    let table = branching::leaf_bases(&tree, &f.canopy);
    for k in 1..table.len() {
        let apart = table[k].radial.dot(table[k - 1].radial);
        assert!(
            apart < 0.9,
            "base {k} stands on the radial of the base above it"
        );
    }
}

/// R3: no base stands above the oldest living frond. The crown spreads its
/// insertions over its own depth and the bases begin below them, so the two
/// sequences meet without a gap or an overlap.
#[test]
fn no_base_stands_above_the_oldest_living_frond() {
    let f = family("date-palm", 1);
    let tree = mesh::grow(&f).expect("the skeleton grows");
    let (at, axis) = apices(&tree)[0];
    for (k, base) in branching::leaf_bases(&tree, &f.canopy).iter().enumerate() {
        let below = (at - base.from).dot(axis);
        assert!(
            below >= f.canopy.rosette_depth - 1e-9,
            "base {k} stands {below} below the apex, inside the crown's own {}",
            f.canopy.rosette_depth
        );
    }
}

/// R3: weathering wears the foot of the trunk away. Every base below another
/// is worn further back, in its reach out of the bark and in its girth alike.
#[test]
fn weathering_wears_the_lowest_bases_back() {
    let mut f = family("date-palm", 1);
    f.canopy.leaf_base_weathering = 0.6;
    let tree = mesh::grow(&f).expect("the skeleton grows");
    let table = branching::leaf_bases(&tree, &f.canopy);
    let bases = hung(&tree);
    assert_eq!(table.len(), bases.len());
    let reach = |k: usize| {
        let (attach, tip) = bases[k];
        tree.nodes[tip]
            .position
            .distance(tree.nodes[attach].position)
            - table[k].girth
    };
    let wear = |k: usize| tree.nodes[bases[k].1].radius / tree.nodes[bases[k].1].start_radius;
    for k in 1..table.len() {
        assert!(table[k].age > table[k - 1].age, "base {k} is not older");
        assert!(
            reach(k) < reach(k - 1) + 1e-9,
            "base {k} reaches {} out of the bark, further than {} above it",
            reach(k),
            reach(k - 1)
        );
        assert!(
            wear(k) < wear(k - 1) + 1e-12,
            "base {k} is not worn further"
        );
    }
    assert!(
        reach(table.len() - 1) < reach(0) * 0.5,
        "the lowest base is not worn back against the newest"
    );
}

/// R2: the bases absent, the tree is the tree the generator grew before them -
/// every node, every radius, byte for byte.
#[test]
fn the_bases_absent_leave_the_skeleton_where_it_was() {
    let mut f = family("date-palm", 1);
    f.canopy.leaf_bases = 0;
    let grown = mesh::grow(&f).expect("the skeleton grows");
    let mut bare = branching::generate(&f.skeleton, f.radii)
        .expect("the skeleton grows")
        .tree;
    branching::clear_apical_twigs(&mut bare).expect("the apical twigs clear");
    assert_eq!(grown, bare, "a base was hung where no row asked for one");
    assert!(hung(&grown).is_empty());
}

/// R2's positive control: the shipped presets keep every byte because their
/// tables leave the organs absent, not because the switch does nothing. The
/// palm's own wood moves the moment the row rises off zero.
#[test]
fn raising_the_bases_moves_the_palms_own_wood() {
    let mut bare = family("date-palm", 1);
    bare.canopy.leaf_bases = 0;
    let without = mesh::build(&bare).expect("the palm builds");
    let with = mesh::build(&family("date-palm", 1)).expect("the palm builds");
    assert_ne!(
        without.wood.positions, with.wood.positions,
        "a trunk clothed in bases has to be a different piece of wood"
    );
    assert!(with.wood_vertices() > without.wood_vertices());
    assert_eq!(
        without.foliage.instances.leaves, with.foliage.instances.leaves,
        "a base is wood: it places no leaf"
    );
}

/// The bases are hung after the radius solve, so no base enters the pipe
/// model and no trunk thickens under the wood it carries.
#[test]
fn a_base_never_thickens_the_trunk_it_hangs_on() {
    let f = family("date-palm", 1);
    let mut bare = f.clone();
    bare.canopy.leaf_bases = 0;
    let (with, without) = (
        mesh::grow(&f).expect("the skeleton grows"),
        mesh::grow(&bare).expect("the skeleton grows"),
    );
    assert_eq!(with.crossover, without.crossover);
    for (i, node) in without.nodes.iter().enumerate() {
        assert_eq!(node, &with.nodes[i], "the bases moved stem node {i}");
    }
}
