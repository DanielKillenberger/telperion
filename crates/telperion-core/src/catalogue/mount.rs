//! Where each group sits in a family, and the family's rows in catalogue
//! order: every group's rows in declaration order, groups in the order below.
use super::{Info, Row, Scalar};
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
    fn row(&self, index: usize) -> (&'static str, &'static Info);
    fn get<'f>(&self, index: usize, f: &'f Family) -> &'f dyn Scalar;
    fn set<'f>(&self, index: usize, f: &'f mut Family) -> &'f mut dyn Scalar;
}

struct Mounted<S: 'static> {
    rows: &'static [Row<S>],
    of: fn(&Family) -> &S,
    of_mut: fn(&mut Family) -> &mut S,
}
impl<S> Group for Mounted<S> {
    fn len(&self) -> usize {
        self.rows.len()
    }
    fn row(&self, index: usize) -> (&'static str, &'static Info) {
        let row = &self.rows[index];
        (row.path, &row.info)
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

/// One row of a family.
#[derive(Clone, Copy)]
pub struct Entry {
    group: &'static dyn Group,
    index: usize,
}
impl Entry {
    /// The row's JSON pointer on the wire.
    pub fn path(self) -> &'static str {
        self.group.row(self.index).0
    }
    pub fn info(self) -> &'static Info {
        self.group.row(self.index).1
    }
    pub fn get(self, f: &Family) -> &dyn Scalar {
        self.group.get(self.index, f)
    }
    pub fn set(self, f: &mut Family) -> &mut dyn Scalar {
        self.group.set(self.index, f)
    }
}

/// Every row of a family, in catalogue order.
pub fn entries() -> impl Iterator<Item = Entry> {
    GROUPS.iter().flat_map(|group| {
        (0..group.len()).map(move |index| Entry {
            group: *group,
            index,
        })
    })
}

/// The row at a wire path.
pub fn entry(path: &str) -> Option<Entry> {
    entries().find(|e| e.path() == path)
}
