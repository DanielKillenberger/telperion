//! What a tree reports as it grows (fn-211): each stage as it ends, and
//! within it each pass, so a profile is the time between reports and
//! needs no hand patch.

/// The stages a tree passes through, reported as each ends: per cycle its
/// growth, its rough layout, its full lay every few cycles and the sweep
/// of its light (where light shades), then once its sizes, shedding and
/// girth, and its final lay. Each pass within them is reported as it
/// ends as well (`Stage::Pass`), before its stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Grown,
    Sketched,
    /// The full lay of the living tree, every `RELAY_EVERY` cycles.
    Relaid,
    Lit,
    Settled,
    Laid,
    Pass(Pass),
}

/// The passes within the stages, each reported as it ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pass {
    /// Once, before growth: the sleeping buds' windows.
    Windows,
    /// Each cycle's growth: the carbon balance, the vigour allotted, the
    /// apexes' axes picked, their units grown, the shares merged, the
    /// sleeping buds woken.
    Balance,
    Allot,
    Homes,
    Units,
    Merge,
    Wake,
    /// Each cycle's rough layout.
    Sketch,
    /// The full lay: the standing tree settled, laid, its sag's levers,
    /// laid under sag, and its places written back.
    RelaySettle,
    RelayPlace,
    RelayLevers,
    RelaySag,
    RelayWrite,
    /// The light: the lattice swept, and read at the buds.
    ShadeField,
    ShadeRead,
    /// Once settled: the living apexes marked, presences assigned, the
    /// tree as grown sized for its shed pipes, shedding, the pipes
    /// kept, scale, girth and the leaf term's mean.
    Mark,
    Assign,
    Attachments,
    Shed,
    Disused,
    Scale,
    Thicken,
    LeafMean,
    /// The final lay, its sag's levers, and the lay under sag.
    Place,
    Levers,
    SagPlace,
}
