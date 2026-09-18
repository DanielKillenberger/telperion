//! The one engine-neutral mesh call: every preset builds, the reported counts
//! describe the buffers a renderer uploads, and the bounds enclose both parts.
mod specimens;
use telperion_core::{
    math::Vec3,
    mesh::{self, Detail},
    presets::Preset,
    surface::Bounds,
    Error,
};

const IDENTITIES: [&str; 7] = [
    "ordinary",
    "oregon-white-oak",
    "norway-spruce",
    "european-beech",
    "silver-birch",
    "telperion",
    "laurelin",
];

fn contains(b: Bounds, p: Vec3) -> bool {
    p.x >= b.min.x
        && p.y >= b.min.y
        && p.z >= b.min.z
        && p.x <= b.max.x
        && p.y <= b.max.y
        && p.z <= b.max.z
}

#[test]
fn every_preset_builds_a_mesh_whose_counts_and_bounds_describe_its_buffers() {
    for id in IDENTITIES {
        let family = Preset::from_id(id).expect("preset identity").parameters();
        let m = specimens::mesh(&family);
        let mut end = 0;
        let mut radius = f64::INFINITY;
        assert_eq!(m.wood.run_table.len(), m.wood.runs);
        for (i, run) in m.wood.run_table.iter().enumerate() {
            assert_eq!(run.first_index, end, "{id}: run {i}");
            assert!(
                run.index_count > 0 && run.index_count % 3 == 0,
                "{id}: run {i}"
            );
            assert!(
                run.largest_radius.is_finite()
                    && run.largest_radius >= 0.0
                    && run.largest_radius <= radius,
                "{id}: run {i}"
            );
            end += run.index_count;
            radius = run.largest_radius;
        }
        assert_eq!(end as usize, m.wood.indices.len());
        assert_eq!(m.wood_vertices() * 3, m.wood.positions.len(), "{id}");
        assert_eq!(m.wood.normals.len(), m.wood.positions.len(), "{id}");
        assert_eq!(m.wood.coords.len(), m.wood_vertices() * 2, "{id}");
        assert_eq!(
            m.foliage.element.coords.len(),
            m.foliage.element.positions.len() * 2,
            "{id}"
        );
        assert_eq!(m.wood_triangles() * 3, m.wood.indices.len(), "{id}");
        assert_eq!(
            m.foliage_instances(),
            m.foliage.instances.matrices.len(),
            "{id}"
        );
        assert!(m.wood_triangles() > 0 && m.foliage_instances() > 0, "{id}");
        assert!(
            m.wood
                .indices
                .iter()
                .all(|&i| (i as usize) < m.wood_vertices()),
            "{id}: wood index past the vertex buffer"
        );
        for p in m.wood.positions.as_chunks::<3>().0 {
            assert!(
                contains(m.bounds, Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)),
                "{id}: wood vertex outside the mesh bounds"
            );
        }
        let leaves = m
            .foliage
            .instances
            .bounds(&m.foliage.element)
            .unwrap()
            .expect("foliage bounds");
        assert!(
            contains(m.bounds, leaves.min) && contains(m.bounds, leaves.max),
            "{id}: foliage outside the mesh bounds"
        );
    }
}

#[test]
fn a_family_the_generator_rejects_surfaces_its_own_message() {
    let mut family = Preset::from_id("ordinary")
        .expect("preset identity")
        .parameters();
    family.shell_depth = 2.0;
    assert_eq!(
        mesh::build(&family, Detail::Full).err(),
        Some(Error::InvalidInput("shell depth"))
    );
}
