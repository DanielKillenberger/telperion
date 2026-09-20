//! What one specimen costs to keep, counted from the tree it grew on.
//!
//! The counts are the builders' own. The wood's come from `surface::extent`,
//! the same ring arithmetic `surface::build` reserves by; the crown's from the
//! count `foliage::place` reserves by. There is no second implementation to
//! drift from either, and no byte figure is written down here: every term is a
//! count times the size of the type that holds it, so a stored leaf that
//! changes shape moves the answer with it.
//!
//! What is counted is what the specimen keeps: the skeleton's nodes, the
//! wood's four buffers and the crown's leaves, each as its capacity rather
//! than its length, because the cull and the limb clumping retain in place and
//! keep the block they were handed. What is not counted is what the build
//! passes through on the way - the surface builder's scratch, the buffer a
//! digest serialises into, a metrics pass's adjacency - which is why a caller
//! sizing a run leaves headroom above what this returns.
use crate::{
    foliage::{Instances, Leaf, TwigPlacement},
    presets::Family,
    surface::{SurfaceMesh, WoodExtent},
    tree::{Node, Tree},
    Result,
};

/// The buffers one finished specimen keeps resident, in elements.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Footprint {
    /// Nodes the skeleton holds.
    pub nodes: usize,
    /// Elements the wood's four buffers hold.
    pub wood: WoodExtent,
    /// Leaves the crown holds.
    pub leaves: usize,
}

impl Footprint {
    /// Bytes the specimen keeps: each count times the size of the type that
    /// holds it, and nothing else.
    pub fn bytes(&self) -> usize {
        self.bytes_with_leaf(size_of::<Leaf>())
    }

    /// The same sum with the stored leaf at a stated size. The only caller in
    /// production is `bytes`, handing it `size_of::<Leaf>()`; it exists so a
    /// test can change what a leaf costs and watch the answer move, which a
    /// test cannot do by changing the type itself.
    pub fn bytes_with_leaf(&self, leaf: usize) -> usize {
        self.nodes * size_of::<Node>() + self.wood.bytes() + self.leaves * leaf
    }
}

/// What this family will keep resident once the tree it has grown is swept and
/// clothed. It builds no mesh, no placement buffer and no matrix; its own
/// scratch is the paths pass and the runs, both O(nodes).
pub fn predict(tree: &Tree, family: &Family) -> Result<Footprint> {
    let envelope = family.skeleton.envelope;
    Ok(Footprint {
        nodes: tree.nodes.capacity(),
        wood: crate::surface::extent(tree, envelope.height, &family.surface)?,
        leaves: crate::foliage::leaf_count(
            tree,
            envelope,
            family.skeleton.seed,
            family.canopy,
            Some(TwigPlacement::of(family)?),
        )?,
    })
}

/// The same reading taken off the live objects: what the specimen in hand
/// actually holds, so a prediction can be held to it.
///
/// `normals` is `positions`' equal by construction - the sweep reserves one
/// block for each and resizes the second to the first - so the wood's two
/// vertex buffers are read as one count.
pub fn measure(tree: &Tree, wood: &SurfaceMesh, crown: &Instances) -> Footprint {
    Footprint {
        nodes: tree.nodes.capacity(),
        wood: WoodExtent {
            positions: wood.positions.capacity(),
            coords: wood.coords.capacity(),
            indices: wood.indices.capacity(),
        },
        leaves: crown.leaves.capacity(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::foliage::WORDS;

    fn zero() -> Footprint {
        Footprint::default()
    }

    /// Every term of the answer is the size of the type that carries it. A
    /// leaf that grows a word grows the prediction by exactly that word, and
    /// so does a node and so does a vertex: the constant this replaced could
    /// not move at all, which is how it drifted three times unnoticed.
    #[test]
    fn each_term_is_the_size_of_the_type_that_holds_it() {
        let one_more = |f: fn(&mut Footprint)| {
            let mut more = zero();
            f(&mut more);
            more.bytes()
        };
        assert_eq!(one_more(|f| f.nodes = 1), size_of::<Node>());
        assert_eq!(one_more(|f| f.leaves = 1), size_of::<Leaf>());
        // One more vertex is three position floats, three normal floats and
        // two coordinate floats; one more triangle is three indices.
        assert_eq!(
            one_more(|f| {
                f.wood.positions = 3;
                f.wood.coords = 2;
            }),
            8 * size_of::<f32>()
        );
        assert_eq!(one_more(|f| f.wood.indices = 3), 3 * size_of::<u32>());
        // And the stored leaf's own size is its words, not a number beside it.
        assert_eq!(size_of::<Leaf>(), WORDS * size_of::<u32>());
    }

    /// The stored leaf grows a word and the prediction grows with it, by the
    /// leaves it holds times that word and by nothing else. A test cannot
    /// resize the type, so it resizes what the sum is told a leaf costs, and
    /// `bytes` is that same sum told `size_of::<Leaf>()`. This is the
    /// regression the constant could not have: 350 MB stayed 350 MB while the
    /// real figure went 941, then 470, then 88.
    #[test]
    fn a_leaf_that_grows_a_word_grows_the_prediction_by_that_word() {
        let mut f = zero();
        f.nodes = 7;
        f.leaves = 1_000;
        f.wood.positions = 30;
        f.wood.coords = 20;
        f.wood.indices = 9;

        let word = size_of::<u32>();
        assert_eq!(
            f.bytes(),
            f.bytes_with_leaf(WORDS * word),
            "bytes is the sum at the leaf's own size"
        );
        assert_eq!(
            f.bytes_with_leaf((WORDS + 1) * word) - f.bytes(),
            f.leaves * word,
            "a word on every leaf moved the answer by something else"
        );
        assert_eq!(
            f.bytes() - f.bytes_with_leaf((WORDS - 1) * word),
            f.leaves * word,
            "shrinking the leaf did not shrink the answer by the same"
        );
        // Nothing but the crown moves: a specimen with no leaves is deaf to it.
        let mut bare = f;
        bare.leaves = 0;
        assert_eq!(bare.bytes_with_leaf((WORDS + 9) * word), bare.bytes());
    }

    /// A tree with no nodes predicts nothing rather than refusing: an empty
    /// specimen is a legitimate answer, and it costs nothing to keep.
    #[test]
    fn a_tree_with_no_nodes_predicts_nothing() {
        assert_eq!(zero().bytes(), 0);
        let empty = predict(&Tree::default(), &Family::default()).unwrap();
        assert_eq!(empty, zero());
        assert_eq!(empty.bytes(), 0);
    }
}
