//! A direct build that its twig layer would carry past the node budget gives
//! up fine detail evenly instead: it keeps the most twig detail that finishes
//! inside the budget, every axis to its tips. The level is found by whole
//! generations and then by bisection inside the one that does not fit. Each
//! level regrows the tree from its rows, which draw the same scaffold every
//! time, so the level depends on the budget and the tree and never on the
//! order the build grew in. Nothing is cloned: a specimen's clone would carry
//! the whole retained-growth state into the slim field module.
use super::super::local::detail::{Detail, STEPS};
use super::*;

impl Specimen {
    /// The scaffold, grown out at the most detail the budget holds. Where even
    /// no twig detail fits, the tree is the least-detailed one, stopped where
    /// the count ran out, and stays capped.
    pub(super) fn within_budget(
        params: &SkeletonParams,
        radii: RadiusParams,
        crowned: bool,
    ) -> Result<Self> {
        let top = u16::try_from(params.twigs.resolved()?.generations)
            .map_err(|_| Error::InvalidInput("twig generations"))?
            * STEPS;
        // `top` itself is the full build, which already ran past the budget.
        let (mut low, mut high) = (0, top);
        let mut kept = None;
        // Whole generations first, from the scaffold out: a tree's twig layer
        // is rarely as deep as its row allows, and a level that fits is cheap
        // to grow where one that does not costs the whole budget.
        while low + STEPS < high {
            let grown = Self::grown_at(params, radii, crowned, Detail(low + STEPS))?;
            if grown.tree.diagnostics.node_capped {
                high = low + STEPS;
                break;
            }
            low += STEPS;
            kept = Some(grown);
        }
        while high - low > 1 {
            let mid = low + (high - low) / 2;
            let grown = Self::grown_at(params, radii, crowned, Detail(mid))?;
            if grown.tree.diagnostics.node_capped {
                high = mid;
            } else {
                low = mid;
                kept = Some(grown);
            }
        }
        let mut s = match kept {
            Some(s) => s,
            None => Self::grown_at(params, radii, crowned, Detail(low))?,
        };
        // Grown at `low`, the tree is whole unless even that level ran out.
        s.tree.diagnostics.twig_detail = Some(low);
        Ok(s)
    }
    fn grown_at(
        params: &SkeletonParams,
        radii: RadiusParams,
        crowned: bool,
        detail: Detail,
    ) -> Result<Self> {
        let mut s = Self::new(params, radii)?;
        s.local.crowned = crowned;
        s.step(usize::MAX, 0)?;
        s.local.detail = Some(detail);
        s.step(0, usize::MAX)?;
        Ok(s)
    }
}
