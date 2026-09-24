//! Shaped runs: a run the tree keeps a section for is swept on the rings of
//! its cell rather than on round ones. A tree that shapes no run never reaches
//! past the first check here, so its sweep is the one it always was.
use super::{
    paths::{Paths, Run},
    Sample,
};
use crate::{
    math::Vec3,
    tree::{Section, Tree},
};

/// The section a run is drawn as, if the tree shapes it.
pub(super) fn of<'a>(tree: &'a Tree, paths: &Paths, run: &Run) -> Option<&'a Section> {
    if tree.sections.is_empty() {
        return None;
    }
    tree.section(paths.nodes[run.end - 1])
}

/// Stand a sampled run's rings on its cell: each centre on the radial at the
/// ring's reach, each radius the ring's extent. Distances along the wood are
/// the run's own, so the bark is laid on a base as on any wood.
pub(super) fn reshape(section: &Section, samples: &mut [Sample]) {
    let count = samples.len();
    for (k, sample) in samples.iter_mut().enumerate() {
        let ring = section.ring(k, count);
        sample.p = section.centre(ring);
        sample.r = section.extent(ring);
    }
}

/// Square a shaped run's frames to its cell: across the stem and along it, so
/// a ring runs round the radial and the outer cap faces out along it.
pub(super) fn square(section: &Section, frame: &mut [(Vec3, Vec3)]) {
    frame.fill((section.across, section.axis));
}
