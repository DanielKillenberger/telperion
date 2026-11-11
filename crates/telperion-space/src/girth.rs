//! Girth by the pipe model (as Pałubicki et al. 2009 use it): a section is
//! the sum of the sections it carries, by area. Each phytomer adds the
//! section of its PA's primary radius by its scale.
use crate::species::{Form, Species};
use crate::structure::{Axis, Join, Origin, Structure};
use crate::upward::Tree;

/// The radius a phytomer was established with: its PA's primary, by its
/// scale.
pub(crate) fn established(form: &Form, scale: f64) -> f64 {
    f64::from(form.primary as f32 * scale as f32)
}

/// A radius's section, by area.
fn section(radius: f64) -> f32 {
    let r = radius as f32;
    r * r
}

/// A section's radius, by area.
fn radius(section: f32) -> f64 {
    f64::from(section.max(0.0).sqrt())
}

/// Sets every phytomer's radius from the sections it carries, on up to
/// `threads` cores. Each relay gets its parent's girth where it joins
/// (`Axis::joined`).
pub(crate) fn thicken(structure: &mut Structure, species: &Species, threads: usize) {
    let attached: Vec<_> = (0..structure.axes.len())
        .map(|i| self::attached(&structure.axes, species, i))
        .collect();
    let axes = &mut structure.axes;
    let links: Vec<Origin> = axes.iter().map(|a| a.origin).collect();
    let parents: Vec<Option<usize>> = links.iter().map(|l| l.parent()).collect();
    let tree = Tree::new(&parents);
    // Section carried into each axis's phytomers by its laterals and
    // relays, and into its tip by its continuation, the last child first,
    // as a backward pass meets them. Each child hands on its base radius;
    // each relay's join is read where it stands.
    let given_out = tree.upward(
        axes,
        threads,
        &|i, axis, out: &[(f64, Vec<(usize, Join)>)], at_node: &mut Vec<f32>| {
            let form = species.states[axis.pa].form;
            at_node.clear();
            at_node.resize(axis.phytomers.len(), 0.0);
            let (mut at_tip, mut at_base) = (0.0f32, 0.0f32);
            // Each relay's node, share and parts, for its join.
            let mut relays: Vec<(usize, usize, f32, f32, f32)> = Vec::new();
            for &c in tree.children(i) {
                let section = section(out[c].0);
                match (links[c], attached[c]) {
                    (Origin::Seed, _) => {}
                    (Origin::Lateral { node, .. }, _) => match at_node.get_mut(node) {
                        Some(at) => *at += section,
                        None => at_tip += section,
                    },
                    // A relay's wood enters where it joins (fn-244).
                    (Origin::Relay { .. }, Some((k, t))) => {
                        let (at, below) = joining(section, t);
                        at_node[k] += at;
                        match k.checked_sub(1) {
                            Some(j) => at_node[j] += below,
                            None => at_base += below,
                        }
                        relays.push((c, k, t, at, below));
                    }
                    (Origin::Relay { .. } | Origin::Continuation { .. }, _) => at_tip += section,
                }
            }
            let mut joins = Vec::with_capacity(relays.len());
            let mut section = at_tip;
            for (k, phytomer) in axis.phytomers.iter_mut().enumerate().rev() {
                let own = established(&form, phytomer.scale);
                section += self::section(own) + at_node[k];
                phytomer.radius = radius(section);
                for &(c, _, t, at, below) in relays.iter().filter(|r| r.1 == k) {
                    let (below, above) = (radius(section + below), radius(section - at));
                    joins.push((
                        c,
                        Join {
                            node: k,
                            t,
                            below,
                            above,
                        },
                    ));
                }
            }
            (radius(section + at_base), joins)
        },
    );
    for (c, join) in given_out.iter().flat_map(|o| &o.1) {
        structure.axes[*c].joined = Some(Box::new(*join));
    }
}

/// A relay's section as its parent takes it at the relay's attachment
/// (`geometry::attach`): `t` of it at the node whose span it stands in,
/// the rest at the node before (or the base), so the parent's samples
/// move by degree as it moves along (fn-199 R4); the parts.
pub(crate) fn joining(x: f32, t: f32) -> (f32, f32) {
    let at = x * t;
    (at, x - at)
}

/// Axis `i`'s attachment as its parent's node and share
/// (`geometry::attach`, `Join`): none for all but a relay on a parent with
/// nodes.
pub(crate) fn attached(axes: &[Axis], species: &Species, i: usize) -> Option<(usize, f32)> {
    let x = crate::geometry::attach(axes, species, i)?;
    let last = axes[axes[i].origin.parent()?]
        .phytomers
        .len()
        .checked_sub(1)?;
    Some(crate::geometry::split(x, last))
}

#[cfg(test)]
mod tests;
