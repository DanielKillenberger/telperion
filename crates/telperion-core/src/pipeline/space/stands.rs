//! A coarse tree's fine roots (fn-214) as the pipeline holds them: each
//! live root on the node its bearer draws where it stands, its wood from its
//! bud toward twice its expected wood's centre, in the pipeline's y up. A
//! root whose bearer draws no node there stands for wood thinner than any
//! drawn and is left out, as the conversion leaves that wood out.
use super::up;
use crate::tree::Stand;
use telperion_space::{Coarse, Origin};

pub(super) fn convert(coarse: &Coarse, drawn: &[Vec<u32>]) -> Vec<Stand> {
    coarse
        .stands
        .iter()
        .filter_map(|s| {
            let Origin::Lateral { parent, node, .. } = coarse.structure.axes[s.axis].origin else {
                unreachable!("a fine root is a lateral bud")
            };
            let node = *drawn[parent].get(node)?;
            Some(Stand {
                node,
                base: up(s.base),
                tip: up(s.base + s.heading * (2.0 * s.centre)),
                radius: s.radius,
                reach: s.centre,
                phytomers: s.phytomers,
            })
        })
        .collect()
}
