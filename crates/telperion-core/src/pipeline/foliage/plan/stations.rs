//! How many stations a run carries and where each stands: the one count
//! the plan, the prediction and the builder all read.
use super::micrometres;
use crate::{
    envelope::Envelope,
    pipeline::foliage::{CanopyParams, TwigPlacement},
    Error, Result,
};

/// Metres between leaves on a run no twig layer marks: the row's share of the
/// tree's own height, never finer than a tenth of a millimetre.
pub(crate) fn spacing(envelope: Envelope, p: CanopyParams) -> Result<f64> {
    let spacing = p.spacing * envelope.height;
    if !spacing.is_finite() || spacing <= 0.0 {
        return Err(Error::InvalidInput("foliage spacing"));
    }
    Ok(spacing)
}

/// The most stations a run may hold before the vector that carries them would
/// outgrow what an index can address.
pub(crate) fn addressable() -> f64 {
    (isize::MAX as usize / std::mem::size_of::<f64>()) as f64
}

/// How many leaves a run of this grown length (`walk`'s units) carries: one
/// set per internode under a twig layer, height-relative spacing plus a
/// terminal clump without one. The builder places exactly this many and the
/// plan and the prediction count them, from here.
pub(crate) fn station_count(
    length: f64,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
) -> Result<usize> {
    if length == 0. {
        return Ok(0);
    }
    if !length.is_finite() {
        return Err(Error::ResourceLimit("shoot length overflow"));
    }
    if let Some(t) = twig {
        let per = t.stations_per_internode as f64;
        let internodes = (length / micrometres(t.internode_length)).ceil().max(1.);
        if !internodes.is_finite()
            || internodes * per > p.max_instances as f64
            || internodes * per >= addressable()
        {
            return Err(Error::ResourceLimit("foliage instance budget"));
        }
        return Ok(internodes as usize * t.stations_per_internode as usize);
    }
    let count = (length / spacing(envelope, p)?).ceil();
    if !count.is_finite()
        || count + p.clump as f64 > p.max_instances as f64
        || count + p.clump as f64 >= addressable()
    {
        return Err(Error::ResourceLimit("foliage instance budget"));
    }
    Ok(count as usize + p.clump as usize)
}

/// Where station `ordinal` stands along a run under the twig layer `t`, in
/// whole micrometres of grown wood (`walk`): at the start of its internode,
/// `stations_per_internode` stations to an internode.
pub(crate) fn station_at(ordinal: usize, t: TwigPlacement) -> f64 {
    (ordinal / t.stations_per_internode as usize) as f64 * micrometres(t.internode_length)
}
