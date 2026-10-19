use super::*;
use crate::tree::Node;

/// Runs fill a chunk in order until the next does not fit; a run longer
/// than a chunk splits into pieces that repeat three rings, every ring pair
/// drawn once over its pieces, and no chunk passes its limit.
#[test]
fn runs_fill_chunks_in_order_and_a_long_run_splits() {
    let (segments, limit) = (4, 64);
    // 15 rings and caps is 62 vertices; 3 is 14; 38 is 154 and splits, its
    // last piece repeating the first ring and bottom cap.
    let layout = Layout::new([15, 3, 38, 2].into_iter(), segments, true, limit).unwrap();
    let pieces: Vec<_> = layout
        .pieces
        .iter()
        .map(|p| (p.run, p.chunk, p.base, p.first, p.rings, p.lead, p.tail))
        .collect();
    assert_eq!(
        pieces,
        [
            (0, 0, 0, 0, 15, true, true),
            (1, 1, 0, 0, 3, true, true),
            (2, 2, 0, 0, 15, true, false),
            (2, 3, 0, 12, 16, false, false),
            (2, 4, 0, 25, 13, false, true),
            (3, 5, 0, 0, 2, true, true),
        ]
    );
    assert_eq!(layout.chunks, [62, 14, 61, 64, 58, 10]);
    assert!(layout.chunks.iter().all(|&n| n <= limit));
    let split = &layout.pieces[2..5];
    let pairs: Vec<_> = split.iter().flat_map(|p| p.pairs()).collect();
    assert_eq!(pairs, (0..37).collect::<Vec<_>>());
    let extent = layout.extent(segments);
    assert_eq!(extent.chunks, 6);
    assert_eq!(extent.indices, (15 + 3 + 38 + 2) * segments * 6);
}

/// A trunk of `nodes` nodes with a short side branch.
fn trunk(nodes: usize) -> Tree {
    let bend = |i: usize| Vec3::new((i as f64 * 0.003).sin(), i as f64 * 0.01, 0.0);
    let mut out: Vec<Node> = (0..nodes)
        .map(|i| Node {
            position: bend(i),
            parent: i.checked_sub(1).map(|p| p as u32),
            radius: 0.5 - 0.4 * i as f64 / nodes as f64,
            start_radius: 0.5,
            ..Node::root()
        })
        .collect();
    for k in 0..6 {
        let parent = if k == 0 { 100 } else { (nodes + k - 1) as u32 };
        out.push(Node {
            position: bend(100) + Vec3::new(0.1 * (k + 1) as f64, 0.0, 0.05 * k as f64),
            parent: Some(parent),
            radius: 0.05,
            start_radius: 0.05,
            ..Node::root()
        });
    }
    Tree {
        nodes: out,
        ..Tree::default()
    }
}

/// A corner as its bits: position, encoded normal and coords.
type Corner = ([u32; 3], [i16; 2], [u16; 2]);

fn corner(positions: &[f32], normal: [i16; 2], coords: &[u16], i: usize) -> Corner {
    let (p, t) = (&positions[i * 3..], &coords[i * 2..]);
    ([p[0], p[1], p[2]].map(f32::to_bits), normal, [t[0], t[1]])
}

/// A trunk of 140,000 vertices splits over three chunks and changes no
/// value: every vertex its pieces hold is the vertex the trunk has swept
/// whole, every triangle the whole trunk draws is drawn once from the same
/// corners, in its draw order over the chunks laid end to end, caps last
/// in the last chunk, and every seat on the rings
/// lands where it lands on rings swept apart. The chunks hold no more than
/// their limit, each reserved at its exact size.
#[test]
fn a_run_longer_than_a_chunk_splits_with_no_value_changed() {
    let tree = trunk(7_000);
    let (height, params) = (70.0, SurfaceParams::default());
    let sweep = Sweep {
        drawn: true,
        edges: true,
        distances: false,
    };
    let rings =
        |limit| super::super::rings::rings_mode(&tree, height, &params, sweep, None, false, limit);
    let (kept, chunked) = (
        rings(CHUNK_VERTICES).unwrap(),
        rings(CHUNK_VERTICES).unwrap(),
    );
    let mesh = chunked.into_mesh(faces(&kept, &tree, height, &params).unwrap());
    // The trunk swept whole, as one run in one buffer, shaded as the flat
    // wood shaded it.
    let whole = rings(usize::MAX).unwrap();
    let trunk = whole.layout.pieces[0];
    assert_eq!((trunk.run, trunk.base), (0, 0));
    let flat = &whole.chunks[0];
    let segments = whole.segments;
    let vertices = trunk.rings * segments + 2;
    assert!(vertices > 2 * CHUNK_VERTICES);
    let mut normals = vec![0.0; vertices * 3];
    let mut drawn = Vec::new();
    let resweep = Resweep::new(&tree, height, &params);
    let dropped = shade(
        &flat.positions,
        trunk.rings,
        segments,
        0,
        &resweep,
        &mut normals,
        |t| drawn.push(t),
    );
    assert_eq!(dropped, Ok(0));
    let at = |i: u32| {
        let n = &normals[i as usize * 3..][..3];
        let n = encode::normal(Vec3::new(n[0].into(), n[1].into(), n[2].into()));
        corner(&flat.positions, n, &flat.coords, i as usize)
    };
    let expected: Vec<[Corner; 3]> = drawn.iter().map(|t| t.map(at)).collect();

    assert_eq!(mesh.chunks.len(), 3);
    for c in &mesh.chunks {
        assert!(c.positions.len() / 3 <= CHUNK_VERTICES);
        let exact = [
            (c.positions.len(), c.positions.capacity()),
            (c.normals.len(), c.normals.capacity()),
            (c.coords.len(), c.coords.capacity()),
            (c.indices.len(), c.indices.capacity()),
        ];
        assert!(exact.iter().all(|(n, cap)| n == cap), "{exact:?}");
    }
    let pieces = &mesh.run_table[..3];
    assert!(pieces
        .iter()
        .all(|p| p.span.largest_radius == pieces[0].span.largest_radius));
    // The pieces' spans laid end to end, chunk after chunk, draw the whole
    // trunk's triangles in its order: every strip, then the caps in turn.
    let mut all = Vec::new();
    for WoodRun { chunk, span } in pieces {
        let c = &mesh.chunks[*chunk as usize];
        let at = |i: u16| {
            let n = &c.normals[i as usize * 2..];
            corner(&c.positions, [n[0], n[1]], &c.coords, i as usize)
        };
        let range =
            span.first_index as usize / 3..(span.first_index + span.index_count) as usize / 3;
        all.extend(
            c.indices.as_chunks::<3>().0[range]
                .iter()
                .map(|t| t.map(at)),
        );
    }
    assert!(all == expected, "the split trunk's draw order");
    let read = AttachmentSurface::on_wood(&kept).unwrap();
    let swept = AttachmentSurface::new(&tree, height, &params).unwrap();
    for (node, n) in tree.nodes.iter().enumerate().skip(1) {
        let parent = tree.nodes[n.parent.unwrap() as usize].position;
        let origin = (parent + n.position) * 0.5;
        let radial = (n.position - parent)
            .cross(Vec3::new(0.3, 0.1, 0.9))
            .normalized();
        let seat = |s: &AttachmentSurface| s.point(node, origin, radial, n.radius);
        assert_eq!(seat(&read), seat(&swept), "{node}");
    }
}

/// The address past an exactly full chunk is the next chunk's first: a
/// contact cache sized by it holds every ring pair of the last chunk.
#[test]
fn the_end_of_a_full_odd_chunk_is_the_next_chunks_start() {
    assert_eq!(address(1, CHUNK_VERTICES, 16), 2 * CHUNK_VERTICES);
    assert_eq!(address(1, 7, 16), CHUNK_VERTICES + 7);
}
