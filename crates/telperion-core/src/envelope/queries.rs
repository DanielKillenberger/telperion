//! Who asks the crown for its radius, and what for. With the `query-count`
//! feature every `Envelope::radius_at` counts against the purpose of the
//! innermost scope open on its thread, so each query has one caller. Without
//! it a scope is an empty value and the count an empty call, so a timing run
//! carries no counter at all.

/// What a crown radius query is asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purpose {
    /// Anything no scope names.
    Other,
    /// The scaffold's straight-line room for a first-order axis.
    ScaffoldRoom,
    /// The scaffold's edge containment and its attractor scatter.
    ScaffoldContainment,
    /// The twig layer's check at the end of each stride.
    TwigStride,
    /// The twig layer's search for where a refused stride meets the outline.
    TwigBisection,
    /// The admission of a terminal or leaf-bearing twig.
    TerminalAdmission,
    /// A dropping curtain's search for the shell's lower surface.
    CurtainBand,
    /// Shedding's depth test of every node past the crossover.
    Shedding,
}

impl Purpose {
    pub const ALL: [Purpose; 8] = [
        Purpose::Other,
        Purpose::ScaffoldRoom,
        Purpose::ScaffoldContainment,
        Purpose::TwigStride,
        Purpose::TwigBisection,
        Purpose::TerminalAdmission,
        Purpose::CurtainBand,
        Purpose::Shedding,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Purpose::Other => "other",
            Purpose::ScaffoldRoom => "scaffold_room",
            Purpose::ScaffoldContainment => "scaffold_containment",
            Purpose::TwigStride => "twig_stride",
            Purpose::TwigBisection => "twig_bisection",
            Purpose::TerminalAdmission => "terminal_admission",
            Purpose::CurtainBand => "curtain_band",
            Purpose::Shedding => "shedding",
        }
    }
}

/// What one thread asked since the counts were last taken.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    /// Radius queries, indexed as `Purpose::ALL`.
    pub queries: [u64; 8],
    /// Axes the twig layer planned stride by stride.
    pub planned_axes: u64,
}

/// An open purpose; dropping it restores the one it replaced.
#[must_use]
pub struct Scope {
    #[cfg(feature = "query-count")]
    previous: Purpose,
}

#[cfg(any(test, feature = "query-count"))]
#[derive(Default)]
struct Tally {
    current: Option<Purpose>,
    counts: Counts,
}

#[cfg(any(test, feature = "query-count"))]
impl Tally {
    fn open(&mut self, purpose: Purpose) -> Purpose {
        self.current.replace(purpose).unwrap_or(Purpose::Other)
    }
    fn close(&mut self, previous: Purpose) {
        self.current = Some(previous);
    }
    fn radius(&mut self) {
        let at = self.current.unwrap_or(Purpose::Other) as usize;
        self.counts.queries[at] += 1;
    }
}

#[cfg(feature = "query-count")]
thread_local! {
    static TALLY: std::cell::RefCell<Tally> = std::cell::RefCell::default();
}

/// Opens `purpose` until the returned scope drops.
#[inline(always)]
pub fn during(purpose: Purpose) -> Scope {
    #[cfg(feature = "query-count")]
    return Scope {
        previous: TALLY.with_borrow_mut(|t| t.open(purpose)),
    };
    #[cfg(not(feature = "query-count"))]
    {
        let _ = purpose;
        Scope {}
    }
}

#[cfg(feature = "query-count")]
impl Drop for Scope {
    fn drop(&mut self) {
        TALLY.with_borrow_mut(|t| t.close(self.previous));
    }
}

#[inline(always)]
pub(crate) fn radius() {
    #[cfg(feature = "query-count")]
    TALLY.with_borrow_mut(Tally::radius);
}

#[inline(always)]
pub(crate) fn planned_axis() {
    #[cfg(feature = "query-count")]
    TALLY.with_borrow_mut(|t| t.counts.planned_axes += 1);
}

/// This thread's counts since the last take, reset to zero; none when the
/// crate was built without `query-count`.
pub fn take() -> Option<Counts> {
    #[cfg(feature = "query-count")]
    return Some(TALLY.with_borrow_mut(|t| std::mem::take(&mut t.counts)));
    #[cfg(not(feature = "query-count"))]
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A query counts once, against the innermost open purpose, and closing a
    /// scope hands the count back to the one around it.
    #[test]
    fn a_query_counts_against_the_innermost_purpose() {
        let mut t = Tally::default();
        t.radius();
        let outer = t.open(Purpose::TwigStride);
        t.radius();
        let inner = t.open(Purpose::CurtainBand);
        t.radius();
        t.radius();
        t.close(inner);
        t.radius();
        t.close(outer);
        t.radius();
        let at = |p: Purpose| t.counts.queries[p as usize];
        assert_eq!(at(Purpose::Other), 2);
        assert_eq!(at(Purpose::TwigStride), 2);
        assert_eq!(at(Purpose::CurtainBand), 2);
        assert_eq!(t.counts.queries.iter().sum::<u64>(), 6);
    }

    #[test]
    fn every_purpose_has_its_own_slot_and_name() {
        for (i, p) in Purpose::ALL.into_iter().enumerate() {
            assert_eq!(p as usize, i);
            assert!(Purpose::ALL[..i].iter().all(|q| q.name() != p.name()));
        }
    }
}
