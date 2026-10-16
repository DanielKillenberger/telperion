//! The one engine-neutral mesh call: every preset builds, the reported counts
//! describe the buffers a renderer uploads, and the bounds enclose both parts.
use super::specimens;
use telperion_core::{
    math::Vec3,
    mesh,
    presets::Preset,
    surface::{Bounds, CHUNK_VERTICES},
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
    let _turn = crate::one_tree::turn();
    for id in IDENTITIES {
        let family = Preset::from_id(id).expect("preset identity").parameters();
        // An engine preset's tree is the engine's, built as production builds
        // it; the scaffold grows the species-less tables alone (fn-198).
        let m = match family.space.species {
            Some(_) => mesh::build(&family).expect("the preset builds"),
            None => specimens::mesh(&family),
        };
        let mut ends = vec![0; m.wood.chunks.len()];
        let mut radius = f64::INFINITY;
        // Widest first on each side of the fine roots' seam: the structure
        // above the cut, then the fine roots (fn-212 D5), so the order
        // starts over at most once. The spans tile each chunk's indices.
        let mut sides = 1;
        assert_eq!(m.wood.run_table.len(), m.wood.runs);
        for (i, run) in m.wood.run_table.iter().enumerate() {
            let (end, span) = (&mut ends[run.chunk as usize], run.span);
            assert_eq!(span.first_index, *end, "{id}: run {i}");
            assert!(
                span.index_count > 0 && span.index_count % 3 == 0,
                "{id}: run {i}"
            );
            if span.largest_radius > radius {
                sides += 1;
            }
            assert!(
                span.largest_radius.is_finite() && span.largest_radius >= 0.0 && sides <= 2,
                "{id}: run {i}"
            );
            *end += span.index_count;
            radius = span.largest_radius;
        }
        assert_eq!(
            m.foliage.element.coords.len(),
            m.foliage.element.positions.len() * 2,
            "{id}"
        );
        assert_eq!(m.foliage_instances(), m.foliage.instances.len(), "{id}");
        assert!(m.wood_triangles() > 0 && m.foliage_instances() > 0, "{id}");
        for (chunk, &end) in m.wood.chunks.iter().zip(&ends) {
            let vertices = chunk.positions.len() / 3;
            assert!(
                vertices <= CHUNK_VERTICES,
                "{id}: a chunk of {vertices} vertices"
            );
            assert_eq!(end as usize, chunk.indices.len(), "{id}");
            assert_eq!(chunk.normals.len(), chunk.positions.len(), "{id}");
            assert_eq!(chunk.coords.len(), vertices * 2, "{id}");
            assert!(
                chunk.indices.iter().all(|&i| (i as usize) < vertices),
                "{id}: wood index past its chunk"
            );
            for p in chunk.positions.as_chunks::<3>().0 {
                assert!(
                    contains(m.bounds, Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)),
                    "{id}: wood vertex outside the mesh bounds"
                );
            }
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
    let mut family = Preset::from_id("date-palm")
        .expect("preset identity")
        .parameters();
    family.shell_depth = 2.0;
    assert_eq!(
        mesh::build(&family).err(),
        Some(Error::InvalidInput("shell depth"))
    );
}
