//! The browser's device budget (fn-212 task 10, host decision 3). WebGPU
//! says nothing of a device's free memory, so a browser build is checked
//! before it starts: the device bytes the page's owned buffers hold now,
//! plus the peak a build of the tree's expected size reaches, against a
//! stated budget. A build over it is refused by name, never started.
use telperion_core::presets::Family;

/// The page's budget, in bytes of owned buffers (`buffer::Ledger`), 6 GiB
/// (fn-235 host decision 3, 2026-10-09): the reference 10 GiB card (the
/// desk's RTX 3080), less 3 GiB other applications may hold (the
/// supported condition), less 1 GiB for the page's textures, pipelines
/// and the driver's own allocations, none of which the ledger counts.
/// One shipped tree at 80 fits in a fresh page: the spruce's measured
/// peak is 5.35 GiB (`PEAKS`). The 8.5 GiB before it was what the card
/// held with nothing else on it (TASK10.md, part 4).
pub(super) const BROWSER: u64 = GIB * (10 - 3 - 1);
const GIB: u64 = 1 << 30;

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
            "the browser's device budget: the tree's expected peak and the tree \
             drawn now exceed the 6 GiB a page may hold; draw a smaller tree first \
             or grow it younger",
        )
        .into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;


    fn spruce(age: f64) -> Family {
        let mut family = telperion_core::presets::by_identity("norway-spruce").unwrap();
        family.age = age;
        family
    }

    #[test]
    fn the_budget_is_six_gib_and_a_tree_at_80_fits_a_fresh_page() {
        assert_eq!(BROWSER, 6 << 30);
        // Held bytes after each tree drawn (`budget_probe`), MiB.
        let held = |mib: u64| mib << 20;
        let fits = |age, h| check(&spruce(age), held(h), BROWSER).is_ok();
        assert!(fits(80.0, 0), "a spruce at 80 in a fresh page");
        assert!(fits(40.0, 0), "a spruce at 40 in a fresh page");
        // R5's beech at 40 and 80 likewise (the oak builds on the CPU).
        for age in [40.0, 80.0] {
            let mut beech = telperion_core::presets::by_identity("european-beech").unwrap();
            beech.age = age;
            assert!(check(&beech, 0, BROWSER).is_ok(), "a beech at {age}");
        }
        assert!(!fits(80.0, 1_757), "80 with a 12 drawn");
        assert!(!fits(40.0, 4_540), "40 with an 80 drawn");
        assert!((peak(4_653_767.0) as f64 / GIB as f64 - 5.35).abs() < 0.01);
        let refused = check(&spruce(80.0), held(1_757), BROWSER).unwrap_err();
        assert!(refused.to_string().contains("device budget"), "{refused}");
    }
}
