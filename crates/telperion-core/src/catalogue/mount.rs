//! Where each group sits in a family, and the family's rows in catalogue
//! order: every group's rows in declaration order, groups in the order below.
//! What the build reads and what the rows document are held in two tables of
//! the same shape, so a build that never documents a row never carries its
//! prose.
use super::{Blend, Bounds, Check, Checked, Info, Row, Scalar};
use crate::{
    bias::{BiasParams, SupernaturalParams},
    branching::{GrowthOverrides, HabitParams, SkeletonParams},
    envelope::Envelope,
    foliage::{CanopyParams, ElementParams},
    growth::GrowthTraits,
    material::MaterialParams,
    radius::RadiusParams,
    surface::SurfaceParams,
    twigs::{TwigAnatomy, TwigParams},
    Family,
};

trait Group: Sync {
    fn len(&self) -> usize;
    fn path(&self, index: usize) -> &'static str;
    fn rule(&self, index: usize) -> (Bounds, Option<Check>, Blend);
    fn get<'f>(&self, index: usize, f: &'f Family) -> &'f dyn Scalar;
    fn set<'f>(&self, index: usize, f: &'f mut Family) -> &'f mut dyn Scalar;
}

struct Mounted<S: 'static> {
    rows: &'static [Row<S>],
    checks: &'static [Checked<S>],
    of: fn(&Family) -> &S,
    of_mut: fn(&mut Family) -> &mut S,
}
impl<S> Group for Mounted<S> {
    fn len(&self) -> usize {
        self.rows.len()
    }
    fn path(&self, index: usize) -> &'static str {
        self.rows[index].path
    }
    fn rule(&self, index: usize) -> (Bounds, Option<Check>, Blend) {
        let checked = &self.checks[index];
        (checked.bounds, checked.check, self.rows[index].blend)
    }
    fn get<'f>(&self, index: usize, f: &'f Family) -> &'f dyn Scalar {
        (self.rows[index].get)((self.of)(f))
    }
    fn set<'f>(&self, index: usize, f: &'f mut Family) -> &'f mut dyn Scalar {
        (self.rows[index].set)((self.of_mut)(f))
    }
}

macro_rules! mount {
    ($group:ty: $($at:ident).*) => {
        &Mounted::<$group> {
            rows: <$group>::ROWS,
            checks: <$group>::CHECKS,
            of: {
                fn of(f: &Family) -> &$group {
                    &(*f)$(.$at)*
                }
                of
            },
            of_mut: {
                fn of_mut(f: &mut Family) -> &mut $group {
                    &mut (*f)$(.$at)*
                }
                of_mut
            },
        }
    };
}

/// The groups, in catalogue order. `DOCS` holds their `INFO` in the same order.
static GROUPS: [&dyn Group; 15] = [
    mount!(Family:),
    mount!(GrowthTraits: growth),
    mount!(SkeletonParams: skeleton),
    mount!(HabitParams: skeleton.habit),
    mount!(Envelope: skeleton.envelope),
    mount!(BiasParams: skeleton.bias),
    mount!(SupernaturalParams: skeleton.bias.supernatural),
    mount!(TwigAnatomy: skeleton.twigs.twig),
    mount!(TwigParams: skeleton.twigs),
    mount!(GrowthOverrides: skeleton.growth),
    mount!(RadiusParams: radii),
    mount!(SurfaceParams: surface),
    mount!(CanopyParams: canopy),
    mount!(ElementParams: element),
    mount!(MaterialParams: material),
];

static DOCS: [&[Info]; 15] = [
    Family::INFO,
    GrowthTraits::INFO,
    SkeletonParams::INFO,
    HabitParams::INFO,
    Envelope::INFO,
    BiasParams::INFO,
    SupernaturalParams::INFO,
    TwigAnatomy::INFO,
    TwigParams::INFO,
    GrowthOverrides::INFO,
    RadiusParams::INFO,
    SurfaceParams::INFO,
    CanopyParams::INFO,
    ElementParams::INFO,
    MaterialParams::INFO,
];

/// One row of a family.
#[derive(Clone, Copy)]
pub struct Entry {
    group: usize,
    index: usize,
}
impl Entry {
    /// The row's JSON pointer on the wire.
    pub fn path(self) -> &'static str {
        GROUPS[self.group].path(self.index)
    }
    /// How a walk between two families moves the row.
    pub fn blend(self) -> Blend {
        GROUPS[self.group].rule(self.index).2
    }
    /// Everything the row's declaration states.
    pub fn info(self) -> &'static Info {
        &DOCS[self.group][self.index]
    }
    pub fn get(self, f: &Family) -> &dyn Scalar {
        GROUPS[self.group].get(self.index, f)
    }
    pub fn set(self, f: &mut Family) -> &mut dyn Scalar {
        GROUPS[self.group].set(self.index, f)
    }
}

/// Every row of a family, in catalogue order.
pub fn entries() -> impl Iterator<Item = Entry> {
    GROUPS
        .iter()
        .enumerate()
        .flat_map(|(group, g)| (0..g.len()).map(move |index| Entry { group, index }))
}

/// The row at a wire path.
pub fn entry(path: &str) -> Option<Entry> {
    entries().find(|e| e.path() == path)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two tables describe the same rows: the build's rule for each row
    /// is the one its declaration states.
    #[test]
    fn every_row_reads_as_it_is_declared() {
        for (group, docs) in GROUPS.iter().zip(DOCS) {
            assert_eq!(group.len(), docs.len());
            for (index, info) in docs.iter().enumerate() {
                assert_eq!(group.rule(index), (info.bounds, info.check, info.blend));
            }
        }
    }
}
