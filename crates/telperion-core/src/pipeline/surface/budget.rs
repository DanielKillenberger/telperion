//! The drawn wood's memory, checked against what is left before any of it
//! is allocated (fn-291 R2f, host decision 2026-10-18). The need is exact,
//! from the layout. The headroom covers fragmentation: no wood buffer is
//! larger than a full chunk's positions, so live bytes are a sound count.
use super::*;
use rings::{Coords, RingChunk};

/// A full chunk's positions, the largest buffer the wood reserves.
const HEADROOM: u64 = CHUNK_VERTICES as u64 * 3 * 4;

/// Refuses, by name, a wood whose layout needs more bytes than are left.
/// Where no source of the bytes left is installed, nothing is checked.
pub(super) fn check(layout: &Layout, segments: usize, coords: Coords) -> Result<()> {
    fits(need(layout, segments, coords), crate::memory::left())
}

/// `needed` against `left`, where a host said what is left.
fn fits(needed: u64, left: Option<u64>) -> Result<()> {
    match left {
        Some(available) if needed > available => Err(Error::WoodMemory { needed, available }),
        _ => Ok(()),
    }
}

/// The bytes the drawn wood reserves from its layout on, plus the headroom:
/// each chunk's positions, coords or distances, normals and indices; the
/// chunk lists the ring step, the mesh step and the wood each hold; the run
/// table and spans, twice where the mesh step drops triangles; and the mesh
/// step's float scratch (`scratch`).
pub(super) fn need(layout: &Layout, segments: usize, coords: Coords) -> u64 {
    let vertices: u64 = layout.chunks.iter().map(|&v| v as u64).sum();
    let rings: u64 = layout.chunks.iter().map(|&v| (v / segments) as u64).sum();
    let pieces = &layout.pieces;
    let indices: u64 = pieces
        .iter()
        .map(|p| p.indices(segments, true) as u64)
        .sum();
    let along = match coords {
        Coords::None => 0,
        Coords::Bark(_) => vertices * 2 * 2,
        Coords::Distances => rings * 4,
    };
    let chunk = size_of::<RingChunk>() + size_of::<FaceChunk>() + size_of::<SurfaceChunk>();
    let lists = layout.chunks.len() as u64 * chunk as u64;
    let tables = pieces.len() as u64 * (2 * size_of::<WoodRun>() as u64 + 8);
    vertices * 3 * 4
        + along
        + vertices * 2 * 2
        + indices * 2
        + lists
        + tables
        + scratch(layout, segments)
        + HEADROOM
}

/// The mesh step's float scratch at its largest: the normals of its longest
/// whole run, held while a split run gathers its points and float normals
/// whole beside its pieces' index starts (`build::split`).
fn scratch(layout: &Layout, segments: usize) -> u64 {
    let pieces = &layout.pieces;
    let whole = pieces
        .iter()
        .filter(|p| p.lead && p.tail)
        .map(|p| p.vertices(segments, true) as u64)
        .max()
        .unwrap_or(0);
    let split = pieces
        .chunk_by(|a, b| a.run == b.run)
        .filter(|run| run.len() > 1)
        .map(|run| {
            let tail = run[run.len() - 1];
            let vertices = ((tail.first + tail.rings) * segments + 2) as u64;
            vertices * 3 * 4 * 2 + run.len() as u64 * 8
        })
        .max()
        .unwrap_or(0);
    whole * 3 * 4 + split
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_wood_over_what_is_left_is_refused_with_both_numbers() {
        let refused = fits(5_000, Some(4_999));
        assert_eq!(
            refused,
            Err(Error::WoodMemory {
                needed: 5_000,
                available: 4_999
            })
        );
        let said = refused.unwrap_err().to_string();
        assert!(
            said.contains("5000 bytes") && said.contains("4999 are left"),
            "{said}"
        );
        assert!(
            said.contains("device entry") && said.contains("radialSegments"),
            "{said}"
        );
    }

    #[test]
    fn a_wood_within_what_is_left_or_with_no_limit_passes() {
        assert_eq!(fits(4_999, Some(4_999)), Ok(()));
        assert_eq!(fits(u64::MAX, None), Ok(()));
    }

    #[test]
    fn a_split_run_s_whole_points_and_normals_count_as_scratch() {
        // 38 rings of 4 sides split over three chunks of at most 64 (the
        // layout `chunks::tests` pins); the longest whole run is 62 vertices.
        let layout = Layout::new([15usize, 3, 38, 2].into_iter(), 4, true, 64).unwrap();
        let split = (38 * 4 + 2) * 3 * 4 * 2 + 3 * 8;
        assert_eq!(scratch(&layout, 4), 62 * 3 * 4 + split);
    }

    #[test]
    fn the_need_counts_every_chunk_buffer_and_the_headroom() {
        let segments = 8;
        let layout =
            Layout::new([10usize, 20].into_iter(), segments, true, CHUNK_VERTICES).unwrap();
        let vertices: u64 = layout.chunks.iter().map(|&v| v as u64).sum();
        let bare = need(&layout, segments, Coords::None);
        // Positions and normals at least, and never less than the headroom.
        assert!(bare >= vertices * (12 + 4) + HEADROOM);
        // Bark coords add exactly two unorm16 a vertex.
        assert_eq!(
            need(&layout, segments, Coords::Bark(1.0)) - bare,
            vertices * 4
        );
    }
}
