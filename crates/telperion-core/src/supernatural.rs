//! The supernatural group (fn-217, DESIGN.md sections 6 and 7): every
//! legendary trait of a family, off by default and on in a legendary preset.
//! While `enabled` is off the group is read as `NONE`: the tree is its base
//! species in form, size, leaf and flower. On, its own rows act (the scale
//! and the leaf's back and margin reflectance), and each override stands in
//! for the natural row it mirrors. An override's path is the natural row's
//! with `/supernatural` in front; the flowering share of age `k`, a `/space`
//! row, is overridden at `/supernatural/flowering/age<k>`. An unset
//! override leaves its natural row as it is.
use crate::catalogue::{self, input, linear, value, Blend, Bounds, Dial, Entry, Site};
use crate::{Family, Result};
use std::borrow::Cow;

mod flowering;
mod overrides;
pub use flowering::SupernaturalFlowering;
pub use overrides::{SupernaturalBloom, SupernaturalMaterial};

/// No legendary row is tuned until a legendary species is onboarded.
const LEGENDARY: Dial = Dial::Excluded(
    "A legendary row, dormant on every natural species; the Two Trees' specs open it to \
     tuning (fn-218, fn-219).",
);

crate::catalogue::rows! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct Supernatural in "/supernatural" {
        /// Whether the legendary traits act. Off, the group reads as none:
        /// the scale is one, the reflectances zero and every override
        /// leaves its natural row as it is.
        pub enabled: bool = "enabled" "switch" Bounds::closed(0.0, 1.0)
            => [Grow, Plan, Expand, Draw] {
            wire: 278,
            note: "A walk interpolates what each side applies, so a family that switches the \
                group on ramps up from its natural rows.",
            blend: Blend::Coupled,
            dial: Dial::Excluded("Boolean, not a numeric scalar."),
        },
        /// How many times its natural size the tree is drawn: every position
        /// and length of the grown wood, after the engine grew it at its
        /// natural size, so its topology is the natural tree's. Leaves and
        /// flowers keep their own size; the leaves keep their spacing along
        /// the wood. One is the natural tree.
        pub scale: f64 = "scale" "times the natural size" Bounds::closed(0.1, 100.0) => [Grow] {
            wire: 279,
            check: value(Site::Supernatural, 0, "scale"),
            applies: "`enabled` off",
            note: "Checked while dormant too. Applied once, to the converted tree, before \
                the whole-tree folds; the device scales the wood it draws by the same value.",
            blend: Blend::Coupled,
            dial: LEGENDARY,
        },
        /// What a leaf's underside mirrors at normal incidence, as a metal
        /// does: the sun's glint and the sky's sheen off the back of the
        /// blade, tinted by the back tint, taken from the face's diffuse.
        /// Zero mirrors nothing and leaves the back as the natural leaf's.
        pub back_reflectance: f64 = "backReflectance" "reflectance"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 280,
            check: input(Site::Supernatural, 1, "back reflectance"),
            applies: "`enabled` off",
            blend: Blend::Coupled,
            dial: LEGENDARY,
        },
        /// The colour the underside mirrors in, a linear reflectance per
        /// channel: a metal's reflection is its colour. Silver is near white.
        pub back_red: f64 = "backRed" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 281,
            check: input(Site::Supernatural, 2, "back tint red"),
            applies: "`enabled` off, or `backReflectance` zero",
            blend: Blend::Coupled,
            dial: LEGENDARY,
        },
        /// The green channel of `back_red`.
        pub back_green: f64 = "backGreen" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 282,
            check: input(Site::Supernatural, 3, "back tint green"),
            applies: "`enabled` off, or `backReflectance` zero",
            blend: Blend::Coupled,
            dial: LEGENDARY,
        },
        /// The blue channel of `back_red`.
        pub back_blue: f64 = "backBlue" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 283,
            check: input(Site::Supernatural, 4, "back tint blue"),
            applies: "`enabled` off, or `backReflectance` zero",
            blend: Blend::Coupled,
            dial: LEGENDARY,
        },
        /// What a leaf's margin band mirrors at normal incidence, on either
        /// face, tinted by the margin's colour: a metallic rim, taken from
        /// the band's diffuse. Zero mirrors nothing.
        pub margin_reflectance: f64 = "marginReflectance" "reflectance"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 284,
            check: input(Site::Supernatural, 5, "margin reflectance"),
            applies: "`enabled` off, or `/material/marginWidth` zero",
            blend: Blend::Coupled,
            dial: LEGENDARY,
        },
        /// How rough the back and the margin are as mirrors: low is a sharp
        /// glint, high a broad sheen.
        pub roughness: f64 = "roughness" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 285,
            check: input(Site::Supernatural, 6, "reflection roughness"),
            applies: "`enabled` off, or both reflectances zero",
            blend: Blend::Coupled,
            dial: LEGENDARY,
        },
        pub material: SupernaturalMaterial,
        pub bloom: SupernaturalBloom,
        pub flowering: SupernaturalFlowering,
    }
}

impl Default for Supernatural {
    fn default() -> Self {
        Self::NONE
    }
}

/// The group's own reflection rows as the renderer reads them, each at its
/// neutral value while the group is off.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reflection {
    pub back: f64,
    pub back_tint: [f64; 3],
    pub margin: f64,
    pub roughness: f64,
}

impl Reflection {
    /// Nothing mirrored: the natural leaf.
    pub const NONE: Self = Self {
        back: 0.0,
        back_tint: [1.0; 3],
        margin: 0.0,
        roughness: 0.3,
    };
}

impl Supernatural {
    /// The group off: every row at its neutral value.
    pub const NONE: Self = Self {
        enabled: false,
        scale: 1.0,
        back_reflectance: 0.0,
        back_red: Reflection::NONE.back_tint[0],
        back_green: Reflection::NONE.back_tint[1],
        back_blue: Reflection::NONE.back_tint[2],
        margin_reflectance: 0.0,
        roughness: Reflection::NONE.roughness,
        material: SupernaturalMaterial::NONE,
        bloom: SupernaturalBloom::NONE,
        flowering: SupernaturalFlowering::NONE,
    };

    /// What the group applies: itself while enabled, none while off.
    pub fn applied(&self) -> Self {
        if self.enabled {
            *self
        } else {
            Self::NONE
        }
    }

    /// The scale the wood is drawn at: one while the group is off.
    pub fn scale(&self) -> f64 {
        self.applied().scale
    }

    /// The reflection rows the leaves are drawn with.
    pub fn reflection(&self) -> Reflection {
        let a = self.applied();
        Reflection {
            back: a.back_reflectance,
            back_tint: [a.back_red, a.back_green, a.back_blue],
            margin: a.margin_reflectance,
            roughness: a.roughness,
        }
    }

    /// Refuses the first of the group's rows off its bounds, whether or not
    /// the group is on.
    pub fn validate(&self) -> Result<()> {
        catalogue::check(Self::CHECKS, self, Site::Supernatural)?;
        catalogue::check(
            SupernaturalMaterial::CHECKS,
            &self.material,
            Site::Supernatural,
        )?;
        catalogue::check(SupernaturalBloom::CHECKS, &self.bloom, Site::Supernatural)?;
        catalogue::check(
            SupernaturalFlowering::CHECKS,
            &self.flowering,
            Site::Supernatural,
        )
    }
}

/// Each override row of a family and the natural row it stands in for:
/// the `/material` and `/bloom` overrides by their path.
fn mirrored() -> impl Iterator<Item = (Entry, Entry)> {
    catalogue::entries().filter_map(|e| {
        let natural = e.path().strip_prefix("/supernatural")?;
        let groups = ["/material/", "/bloom/"];
        groups
            .iter()
            .any(|g| natural.starts_with(g))
            .then(|| catalogue::entry(natural).map(|n| (e, n)))
            .flatten()
    })
}

impl Family {
    /// The family as every stage and the renderer read it: while the
    /// supernatural group is on, each override it sets laid over its
    /// natural row; the family itself while it is off or sets none.
    pub fn applied(&self) -> Cow<'_, Family> {
        let g = &self.supernatural;
        if !g.enabled || !g.overrides() {
            return Cow::Borrowed(self);
        }
        let mut f = self.clone();
        for (over, natural) in mirrored() {
            if let Some(v) = over.get(self).number() {
                natural.set(&mut f).put(v);
            }
        }
        if let Some(species) = f.space.species.as_mut() {
            for (k, share) in g.flowering.shares() {
                if let (Some(v), Some(state)) = (share, species.states.get_mut(k)) {
                    state.flowering = v;
                }
            }
        }
        Cow::Owned(f)
    }

    /// The material the renderer draws the applied family with: the bark's
    /// metre rows (`shootRadius`, `matureRadius`, `matureSpan`) at the
    /// group's scale, as S1 scales every other length of the wood, once.
    pub fn drawn_material(&self) -> crate::material::MaterialParams {
        let mut m = self.applied().material;
        let s = self.supernatural.scale();
        m.shoot_radius *= s;
        m.mature_radius *= s;
        m.mature_span *= s;
        m
    }
}

impl Supernatural {
    /// Whether any override is set.
    fn overrides(&self) -> bool {
        fn set<S>(rows: &[catalogue::Checked<S>], s: &S) -> bool {
            rows.iter().any(|r| (r.get)(s).number().is_some())
        }
        set(SupernaturalMaterial::CHECKS, &self.material)
            || set(SupernaturalBloom::CHECKS, &self.bloom)
            || self.flowering.shares().any(|(_, v)| v.is_some())
    }
}

/// The group at `t` between `a`'s and `b`'s: each row walks between what
/// each side applies, so a walk from a family with the group off ramps the
/// legendary terms up from neutral and each override up from the natural
/// row it stands in for. An override neither side applies stays unset.
pub(crate) fn blend(a: &Family, b: &Family, t: f64) -> Supernatural {
    let (from, to) = (a.supernatural.applied(), b.supernatural.applied());
    let mut g = Supernatural {
        enabled: a.supernatural.enabled || b.supernatural.enabled,
        material: SupernaturalMaterial::NONE,
        bloom: SupernaturalBloom::NONE,
        flowering: SupernaturalFlowering::NONE,
        ..from
    };
    let rows = [
        (from.scale, to.scale),
        (from.back_reflectance, to.back_reflectance),
        (from.back_red, to.back_red),
        (from.back_green, to.back_green),
        (from.back_blue, to.back_blue),
        (from.margin_reflectance, to.margin_reflectance),
        (from.roughness, to.roughness),
    ];
    let [s, br, bg, bb, bbl, m, r] = rows.map(|(x, y)| linear(x, y, t));
    (
        g.scale,
        g.back_reflectance,
        g.back_red,
        g.back_green,
        g.back_blue,
    ) = (s, br, bg, bb, bbl);
    (g.margin_reflectance, g.roughness) = (m, r);
    let over = |x: &Family, at: Entry, natural: Option<f64>| {
        let applied = x.supernatural.enabled.then(|| at.get(x).number()).flatten();
        (applied.is_some(), applied.or(natural))
    };
    let mut f = Family {
        supernatural: g,
        ..a.clone()
    };
    for (at, natural) in mirrored() {
        let (sa, va) = over(a, at, natural.get(a).number());
        let (sb, vb) = over(b, at, natural.get(b).number());
        if let (true, Some(x), Some(y)) = (sa || sb, va, vb) {
            at.set(&mut f).put(linear(x, y, t));
        }
    }
    let share = |x: &Family, k: usize| {
        let natural = x
            .space
            .species
            .as_ref()
            .and_then(|s| s.states.get(k))
            .map(|s| s.flowering);
        let applied = x
            .supernatural
            .enabled
            .then(|| x.supernatural.flowering.get(k))
            .flatten();
        (applied.is_some(), applied.or(natural))
    };
    for k in 0..SupernaturalFlowering::AGES {
        let ((sa, va), (sb, vb)) = (share(a, k), share(b, k));
        if let (true, Some(x), Some(y)) = (sa || sb, va, vb) {
            f.supernatural.flowering.set(k, linear(x, y, t));
        }
    }
    f.supernatural
}

#[cfg(all(test, feature = "geometry"))]
mod scale_tests;
#[cfg(test)]
mod tests;
