//! Where each group sits in a family, and the family's rows in catalogue
//! order: every group's rows in declaration order, groups in the order below.
//! What the build reads and what the rows document are held in two tables of
//! the same shape, so a build that never documents a row never carries its
//! prose.
use super::{Blend, Bounds, Check, Checked, Info, Kind, Row, Scalar};
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
    fn wire(&self, index: usize) -> u16;
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
    fn wire(&self, index: usize) -> u16 {
        self.rows[index].wire
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

/// The groups, in catalogue order: what the build reads of each, what each
/// documents in the same order, and the compile-time lookup of a path.
macro_rules! groups {
    ($($group:ty: $($at:ident).*;)*) => {
        const GROUP_COUNT: usize = [$(stringify!($group)),*].len();
        static GROUPS: [&dyn Group; GROUP_COUNT] = [$(mount!($group: $($at).*)),*];
        static DOCS: [&[Info]; GROUP_COUNT] = [$(<$group>::INFO),*];

        /// The row at a wire path with its bounds and kind, in a const context:
        /// a preset value file is checked against the catalogue as it compiles.
        pub const fn locate(path: &str) -> Option<(Entry, Bounds, Kind)> {
            let mut group = 0;
            $(
                if let Some(index) = find(<$group>::ROWS, path) {
                    let bounds = <$group>::CHECKS[index].bounds;
                    return Some((Entry { group, index }, bounds, <$group>::ROWS[index].kind));
                }
                group += 1;
            )*
            let _ = group;
            None
        }
    };
}
groups! {
    Family:;
    GrowthTraits: growth;
    SkeletonParams: skeleton;
    HabitParams: skeleton.habit;
    Envelope: skeleton.envelope;
    BiasParams: skeleton.bias;
    SupernaturalParams: skeleton.bias.supernatural;
    TwigAnatomy: skeleton.twigs.twig;
    TwigParams: skeleton.twigs;
    GrowthOverrides: skeleton.growth;
    RadiusParams: radii;
    SurfaceParams: surface;
    CanopyParams: canopy;
    ElementParams: element;
    MaterialParams: material;
}

const fn find<S>(rows: &[Row<S>], path: &str) -> Option<usize> {
    let mut i = 0;
    while i < rows.len() {
        if same(rows[i].path.as_bytes(), path.as_bytes()) {
            return Some(i);
        }
        i += 1;
    }
    None
}

const fn same(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

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
    /// The row's rank on the wire before the catalogue.
    pub fn wire(self) -> u16 {
        GROUPS[self.group].wire(self.index)
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
                assert_eq!(group.wire(index), info.wire);
            }
        }
    }
}
