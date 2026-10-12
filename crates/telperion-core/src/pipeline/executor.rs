//! The interface a second executor builds through: the GPU executor in
//! telperion-render. It hands out prepared artifacts
//! and the CPU steps that make them, never a stage to chain; each step is
//! synchronous, so the executor keeps its own schedule and overlap around
//! them. `docs/pipeline.md` has the rationale.
pub use super::input::LeafInput;
// The whole-tree folds an executor holding part of a tree's nodes folds
// its part of and the pipeline combines (fn-212 task 4).
pub use super::input::{beyond, bins, crown_point, envelope, reach, Folds, BINS};
pub use super::space::ConvertRow;
use super::{grown_envelope, GrowInput, Inputs, Request};
use crate::{
    envelope::Envelope,
    math::Vec3,
    mesh::{self, TreeMesh},
    pipeline::foliage::{
        self,
        prepared::{self, PreparedStations},
        BloomParams, CanopyParams, Element, ElementParams, Reference, TwigPlacement,
    },
    pipeline::surface::{
        self,
        compact::{CompactSurface, CompactWithContacts},
        prepared::{PreparedSurface, PreparedWithContacts},
        Curve, SurfaceMesh,
    },
    presets::Family,
    tree::Tree,
    Error, Result,
};

/// The skeleton stage's artifact and the inputs every later stage reads.
pub struct Grown {
    tree: Tree,
    inputs: Inputs,
}

/// Stage 2: the family read once into its stage inputs, and its skeleton.
pub fn grow(family: &Family) -> Result<Grown> {
    let tree = super::skeleton(GrowInput::of(family))?.tree;
    let inputs = Inputs::of(family, grown_envelope(&tree));
    Ok(Grown { tree, inputs })
}

/// What another executor grows and draws a family's tree by (fn-256
/// task 3): the engine's species, the request, the scale the tree is
/// drawn at, and each PA's conversion row (`ConvertRow`). The executor
/// grows the tree through the one cycle and draws it as the conversion
/// draws it; nothing of it comes back to the CPU.
pub struct Growth {
    pub species: telperion_space::Species,
    pub request: telperion_space::Request,
    pub scale: f64,
    pub rows: Vec<ConvertRow>,
}

/// The family read for another executor (`Growth`). A family with no
/// engine species is refused by name.
pub fn growth(family: &Family) -> Result<Growth> {
    let (species, request, scale, rows) = super::space::growth(GrowInput::of(family))?;
    Ok(Growth {
        species,
        request,
        scale,
        rows,
    })
}

/// The whole-tree summaries an executor holding the whole tree folds
/// (DESIGN.md 12.3 of fn-256): its folds, its crown nodes no other passes
/// (read under its own bins), its nodes' leaf box, the root's radius and
/// the stem radius a canopy measures its slender wood against
/// (`Tree::stem_radius`).
pub struct Summaries {
    pub folds: Folds,
    pub crown: Vec<Vec3>,
    pub corners: (Vec3, Vec3),
    pub root: f64,
    pub stem: f64,
}

/// The terms of a station's reach from its wood (`Reach::terms`) under a
/// tree `height` tall, which is all of the envelope they read: what an
/// executor folds its nodes' leaf box by.
pub fn reach_terms(family: &Family, height: f64) -> Result<[f64; 4]> {
    let envelope = Envelope {
        height,
        ..Envelope::default()
    };
    Ok(Inputs::of(family, envelope).plan.reach?.terms())
}

/// Whether an executor drawing the tree itself places `family`'s leaves:
/// on its twig stations, or in a rosette at each stem apex (fn-212 task
/// 10, fn-256), with a twig layer and no limb clumping.
pub fn stations(family: &Family) -> bool {
    let inputs = Inputs::of(family, Envelope::default());
    foliage::plan::supports(inputs.leaves.canopy, inputs.plan.twig.ok())
}

/// The canopy rows `family` hangs retained leaf bases on its stems by,
/// checked as `clothe_leaf_bases` checks them; none where it hangs none
/// (fn-266, DESIGN.md 12.8 of fn-256).
pub fn bases(family: &Family) -> Result<Option<CanopyParams>> {
    if !super::branching::hangs_leaf_bases(&family.canopy) {
        return Ok(None);
    }
    foliage::validate_canopy(&family.canopy)?;
    Ok(Some(family.canopy))
}

/// Stage 3's descriptors for a tree another executor holds whole and
/// draws itself, from its summaries: the rows its leaves are placed by,
/// its element and twig placement, the box they are quantised against
/// (its nodes' leaf box and the root), and its bearing bounds. No node
/// of the tree is held here.
pub fn resident(family: &Family, s: Summaries) -> Result<Expansion> {
    let envelope = envelope(s.folds, s.crown.iter().copied());
    let inputs = Inputs::of(family, envelope);
    let plan = &inputs.plan;
    let element = foliage::build_element(plan.element)?;
    element.validate()?;
    let twig = plan.twig.clone()?;
    check_shell(&inputs)?;
    // The root's own box, as `Reach::wood_box` takes the root's node.
    let out = plan.reach.clone()?.out(s.root);
    let (a, b) = s.corners;
    let lo = Vec3::new(a.x.min(-out), a.y.min(-out), a.z.min(-out));
    let hi = Vec3::new(b.x.max(out), b.y.max(out), b.z.max(out));
    let shoot = inputs.leaves.canopy.shoot_radius;
    let input = GrowInput::of(family);
    let species = input.space.species.as_ref();
    let states = species.map(|s| &s.states[..]).unwrap_or_default();
    Ok(Expansion {
        bearing: [super::space::STRUCTURAL * s.root, s.stem * shoot],
        retains: states.iter().any(|s| crate::tree::retention(s.leaf_years).0 != u32::MAX),
        tree: Tree::default(),
        inputs,
        element,
        twig,
        reference: Reference::spanning(lo, hi),
    })
}

/// The shell depth, refused unless a share.
fn check_shell(inputs: &Inputs) -> Result<()> {
    let shell = inputs.leaves.shell_depth;
    if !shell.is_finite() || !(0.0..=1.0).contains(&shell) {
        return Err(Error::InvalidInput("shell depth"));
    }
    Ok(())
}

/// A solved tree the caller already holds, read with this family's inputs
/// and the envelope it grew to: the same expansion `grow` leads to, for a
/// tree the executor was handed (its tests' hand-built trees). Growing a
/// tree stays inside the pipeline: nothing outside it converts the engine's
/// structure (fn-198).
pub fn expand(tree: Tree, family: &Family) -> Result<Expansion> {
    let inputs = Inputs::of(family, grown_envelope(&tree));
    Grown { tree, inputs }.expansion()
}

/// The leaf element built from its rows, as the plan stage builds it.
pub fn element(params: ElementParams) -> Result<Element> {
    foliage::build_element(params)
}

impl Grown {
    /// Stage 3's descriptors, in the order their errors answer: the leaf
    /// bases hung on the stems, the element and its own check, the twig
    /// placement, the shell depth and the box leaves are quantised against.
    pub fn expansion(self) -> Result<Expansion> {
        let mut tree = self.tree;
        super::clothe(&mut tree, &self.inputs)?;
        let plan = &self.inputs.plan;
        let element = foliage::build_element(plan.element)?;
        element.validate()?;
        let twig = plan.twig.clone()?;
        check_shell(&self.inputs)?;
        let reference = Reference::grown(&tree, plan.reach.clone()?);
        let root = tree.nodes.first().map_or(0.0, |n| n.radius);
        let slender = tree.stem_radius(|i| tree.pipe(i)[1]) * self.inputs.leaves.canopy.shoot_radius;
        Ok(Expansion {
            bearing: [super::space::STRUCTURAL * root, slender],
            retains: !tree.leafing.is_empty(),
            tree,
            inputs: self.inputs,
            element,
            twig,
            reference,
        })
    }
}

/// The wood's section (`Expansion::section`).
pub struct Section {
    pub segments: u32,
    pub lobes: u32,
    pub depth: f32,
    pub angular: Vec<[f32; 4]>,
}

/// A grown tree prepared for expansion by another executor.
pub struct Expansion {
    /// The bounds its stations decide kinds and leaf-bearing by
    /// (`bearing_bounds`).
    bearing: [f64; 2],
    /// Whether some wood sheds its leaves by age (fn-248).
    retains: bool,
    tree: Tree,
    inputs: Inputs,
    element: Element,
    twig: TwigPlacement,
    reference: Reference,
}

impl Expansion {
    pub fn tree(&self) -> &Tree {
        &self.tree
    }
    pub fn element(&self) -> &Element {
        &self.element
    }
    pub fn into_element(self) -> Element {
        self.element
    }
    pub fn twig(&self) -> TwigPlacement {
        self.twig
    }
    pub fn reference(&self) -> Reference {
        self.reference
    }
    /// The rows leaves are placed and culled by.
    pub fn leaves(&self) -> LeafInput {
        self.inputs.leaves
    }
    fn envelope(&self) -> Envelope {
        self.inputs.leaves.envelope
    }
    fn canopy(&self) -> CanopyParams {
        self.inputs.leaves.canopy
    }

    /// The terms of a station's reach from its wood, for an executor
    /// folding its own nodes' leaf box: `[scale, beyond, rachis,
    /// authored]` (`Reach::terms`).
    pub fn wood_reach(&self) -> Result<[f64; 4]> {
        Ok(self.inputs.plan.reach.clone()?.terms())
    }
    /// The bounds an executor drawing its own nodes decides their kinds and
    /// leaf-bearing by (fn-212 task 6): wood under the first is fine wood,
    /// a twig where nothing grows from it; wood within the second bears
    /// leaves (`plan::bearing_runs`).
    pub fn bearing_bounds(&self) -> [f64; 2] {
        self.bearing
    }
    /// Whether some of the tree's wood sheds its leaves by age (fn-248):
    /// the conversion records each node's retention where it does.
    pub fn retains(&self) -> bool {
        self.retains
    }
    /// The wood's section, which an executor drawing its own rings lays
    /// them by: its segments, lobes and lobe depth, and each segment's
    /// angle, cosine and sine (`CompactSurface`'s).
    pub fn section(&self) -> Result<Section> {
        let p = &self.inputs.surface.params;
        let segments = surface::segments(p) as u32;
        Ok(Section {
            segments,
            lobes: p.lobes,
            depth: p.lobe_depth as f32,
            angular: surface::compact::angular_words(segments, p)?,
        })
    }
    /// The corners of the leaf box of the nodes `keep` keeps, and the
    /// root's position: combined by their corners with an executor's own,
    /// exactly the box of the whole (`reference`).
    pub fn wood_box(&self, keep: impl Fn(usize) -> bool) -> Result<(Vec3, Vec3)> {
        Ok(self.inputs.plan.reach.clone()?.wood_box(&self.tree, keep))
    }

    /// Whether the family's leaves stand on regular twig stations.
    pub fn supports_stations(&self) -> bool {
        prepared::supports_stations(self.canopy(), Some(self.twig))
    }
    /// Whether the wood's section is round: no lobes, or lobes of no depth.
    pub fn round_section(&self) -> bool {
        let s = &self.inputs.surface.params;
        s.lobes == 0 || s.lobe_depth == 0.0
    }
    /// Whether leaves of some size are seated on the wood.
    pub fn seats(&self) -> bool {
        let c = self.canopy();
        c.surface_contact > 0.0 && c.size != 0.0
    }

    /// The wood's compact rings.
    pub fn compact(&self) -> Result<CompactSurface> {
        let s = &self.inputs.surface;
        surface::compact::prepare(&self.tree, s.height, &s.params)
    }
    /// The wood's compact rings with the node contacts seated leaves read.
    pub fn compact_with_contacts(&self) -> Result<CompactWithContacts<'_>> {
        let s = &self.inputs.surface;
        surface::compact::prepare_with_contacts(&self.tree, s.height, &s.params)
    }
    /// The wood as curves and its compact rings with contacts, read from
    /// the one sweep (fn-212 R2).
    pub fn curve_with_contacts(&self) -> Result<(Curve, CompactWithContacts<'_>)> {
        let s = &self.inputs.surface;
        let traced = surface::traced(&self.tree, s.height, &s.params)?;
        let compact = surface::compact::with_contacts(&traced, &self.tree, s.height, &s.params)?;
        Ok((traced.curve, compact))
    }
    /// Stations tied to the compact rings' emitted vertices.
    pub fn compact_stations(
        &self,
        shared: &CompactWithContacts<'_>,
    ) -> Result<Option<PreparedStations<()>>> {
        let (envelope, canopy) = (self.envelope(), self.canopy());
        prepared::prepare_compact_stations(shared, envelope, canopy, Some(self.twig))
    }
    /// Stations with contact rings of their own; `None` where the family has
    /// no regular stations.
    pub fn stations(&self) -> Result<Option<PreparedStations>> {
        let (envelope, canopy) = (self.envelope(), self.canopy());
        let params = &self.inputs.surface.params;
        prepared::prepare_stations(&self.tree, envelope, canopy, Some(self.twig), params)
    }
    /// The canonical float32 wood with its node contacts; `None` where the
    /// CPU builder must sweep it.
    pub fn prepared_with_contacts(&self) -> Result<Option<PreparedWithContacts<'_>>> {
        let s = &self.inputs.surface;
        surface::prepared::prepare_with_contacts(&self.tree, s.height, &s.params)
    }
    /// Stations borrowing the canonical wood's contacts.
    pub fn shared_stations<'a>(
        &self,
        shared: &'a PreparedWithContacts<'_>,
    ) -> Result<Option<PreparedStations<&'a [f32]>>> {
        let (envelope, canopy) = (self.envelope(), self.canopy());
        prepared::prepare_shared_stations(shared, envelope, canopy, Some(self.twig))
    }
    /// The canonical float32 wood; `None` where the CPU builder must sweep it.
    pub fn prepared_wood(&self) -> Result<Option<PreparedSurface>> {
        let s = &self.inputs.surface;
        surface::prepared::prepare(&self.tree, s.height, &s.params)
    }
    /// The wood as curves (fn-208): every run's ring points and frames, the
    /// run table and the clusters a view culls by, which the wood is surfaced
    /// from.
    pub fn curve(&self) -> Result<Curve> {
        let s = &self.inputs.surface;
        surface::curve(&self.tree, s.height, &s.params)
    }
    /// The curve's rows with no run in it, for an executor that sweeps its
    /// own runs (fn-256): the lobes, their depth, the twist and the height.
    pub fn curve_rows(&self) -> Result<Curve> {
        let s = &self.inputs.surface;
        surface::curve_rows(s.height, &s.params)
    }
    /// The wood the CPU builder sweeps.
    pub fn wood(&self) -> Result<SurfaceMesh> {
        let s = &self.inputs.surface;
        surface::build(&self.tree, s.height, &s.params)
    }
    /// The flowers at the window's middle (fn-216), a CPU step over the
    /// converted tree: their element and florets, keyed by each node's
    /// lineage; none where the tree flowers nowhere.
    pub fn organ(&self) -> Result<Option<mesh::Foliage>> {
        if self.tree.blooms.is_empty() {
            return Ok(None);
        }
        let bloom = &self.inputs.bloom;
        let element = foliage::bloom_element(bloom)?;
        let instances = foliage::place_blooms(&self.tree, bloom, None)?;
        Ok(Some(mesh::Foliage { element, instances }))
    }
    /// The organ element an executor drawing its own florets places
    /// (fn-229, fn-256).
    pub fn organ_element(&self) -> Result<Element> {
        foliage::bloom_element(&self.inputs.bloom)
    }
    /// The flowers' rows, the time of year a request draws them at, and
    /// the farthest a floret stands from its node (fn-229).
    pub fn bloom(&self) -> (BloomParams, f64, f64) {
        let p = self.inputs.bloom;
        (p, foliage::bloom_now(&p, None), foliage::bloom_reach(&p))
    }
    /// The pipeline's own CPU build of this tree: the reference a GPU
    /// expansion falls back to.
    pub fn mesh(&self) -> Result<TreeMesh> {
        mesh::assembled(super::outputs(&self.tree, &self.inputs, Request::mesh())?)
    }
}

#[cfg(test)]
mod tests;
