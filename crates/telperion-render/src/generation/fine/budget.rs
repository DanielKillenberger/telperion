//! The browser's device budget (fn-212 task 10, host decision 3). WebGPU
//! says nothing of a device's free memory, so a browser build is checked
//! before it starts: the device bytes the page's owned buffers hold now,
//! plus the peak a build of the tree's expected size reaches, against a
//! stated budget. A build over it is refused by name, never started.
use telperion_core::presets::Family;

/// The page's budget, in bytes of owned buffers (`buffer::Ledger`):
/// measured on the desk's RTX 3080 (10 GB) in Chromium, 2026-10-08. A
/// spruce at 40 built with a spruce at 80 drawn (8.04 GiB by this count)
/// built; a spruce at 80 with one at 40 drawn (8.97 GiB) ran out of
/// memory (TASK10.md, part 4).
pub(super) const BROWSER: u64 = 17 << 29;

/// A build's peak owned bytes, from a fresh generator and renderer, by
/// the predictor's expected kept phytomers: the spruce's, the costliest
/// measured (`route_tests::budget_probe`; the beech lies under it).
const PEAKS: [(f64, f64); 5] = [
    (30_251.0, 1_761.0),
    (199_595.0, 2_188.0),
    (1_149_893.0, 3_498.0),
    (2_783_975.0, 4_866.0),
    (4_653_767.0, 5_479.0),
];

/// The peak a build of `kept` expected phytomers reaches, in bytes: the
/// measured peaks joined by straight lines, the last one's slope beyond.
pub(super) fn peak(kept: f64) -> u64 {
    let at = PEAKS
        .partition_point(|&(k, _)| k < kept)
        .clamp(1, PEAKS.len() - 1);
    let ((k0, p0), (k1, p1)) = (PEAKS[at - 1], PEAKS[at]);
    let mebibytes = (p0 + (kept - k0) * (p1 - p0) / (k1 - k0)).max(PEAKS[0].1);
    (mebibytes * f64::from(1 << 20)) as u64
}

/// Whether `family` built now fits `budget` beside the `held` bytes.
pub(super) fn check(family: &Family, held: u64, budget: u64) -> crate::Result<()> {
    let Some(species) = family.space.species.as_ref() else {
        return Ok(());
    };
    let expected = telperion_space::expected_kept(species, family.age as u32);
    let Ok(Some(kept)) = expected else {
        return Ok(());
    };
    if held.saturating_add(peak(kept)) > budget {
        return Err(telperion_core::Error::ResourceLimit(
            "the browser's device budget: the tree's expected peak and the page's \
             buffers exceed 8.5 GiB; draw a smaller tree first or grow it younger",
        )
        .into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIB: f64 = (1u64 << 30) as f64;

    fn spruce(age: f64) -> Family {
        let mut family = telperion_core::presets::by_identity("norway-spruce").unwrap();
        family.age = age;
        family
    }

    #[test]
    fn the_budget_refuses_what_ran_out_of_memory_and_lets_what_built() {
        // Held bytes after each tree drawn (`budget_probe`), MiB.
        let held = |mib: u64| mib << 20;
        let fits = |age, h| check(&spruce(age), held(h), BROWSER).is_ok();
        assert!(fits(80.0, 0), "a spruce at 80 in a fresh page");
        assert!(fits(40.0, 4_540), "40 with an 80 drawn");
        assert!(fits(80.0, 1_757), "80 with a 12 drawn");
        assert!(!fits(80.0, 3_494), "80 with a 40 drawn ran out of memory");
        assert!(!fits(80.0, 4_540), "80 with an 80 drawn ran out of memory");
        assert!((peak(4_653_767.0) as f64 / GIB - 5.35).abs() < 0.01);
    }
}
