//! The browser's device budget (fn-212 task 10, host decision 3). WebGPU
//! says nothing of a device's free memory, so a browser build is checked
//! before it starts: the device bytes the page's owned buffers hold now,
//! plus the peak a build of the tree's expected size reaches, against a
//! stated budget. The budget counts the tree drawn now, so one rule
//! (fn-235 host decision, third): a build that fits beside the drawn tree
//! builds; one that fits only once the drawn tree is let go lets it go
//! first, the screen empty while it builds; one that does not fit even
//! alone is refused by name, never started, and the drawn tree is kept.
use telperion_core::presets::Family;

/// The page's budget, in bytes of owned buffers (`buffer::Ledger`), 6 GiB
/// (fn-235 host decision 3, 2026-10-09): the reference 10 GiB card (the
/// desk's RTX 3080), less 3 GiB other applications may hold (the
/// supported condition), less 1 GiB for the page's textures, pipelines
/// and the driver's own allocations, none of which the ledger counts.
/// One shipped tree at 80 fits in a fresh page: the spruce's measured
/// peak is 5.35 GiB (`PEAKS`). The 8.5 GiB before it was what the card
/// held with nothing else on it (TASK10.md, part 4).
pub(crate) const BROWSER: u64 = GIB * (10 - 3 - 1);
const GIB: u64 = 1 << 30;

/// The native budget's default, in bytes of owned buffers (fn-256 task 5,
/// COUNCIL round 4 step 1): every build is held to it as it grows, so a
/// card other processes fill refuses a build by name rather than hanging
/// (NVRM Xid 109, five times in task 4). Conservative: the reference card
/// holds 10 GiB, a desktop 2 to 3 of it.
pub(crate) const NATIVE: u64 = 4 * GIB;

/// The share of the device memory other processes leave a native build may
/// take: the rest is the driver's and the pipelines', which the ledger
/// does not count.
#[cfg(not(target_arch = "wasm32"))]
const FREE_SHARE: f64 = 0.85;

/// The native budget, with `held` bytes charged now: the stated one
/// (`TELPERION_DEVICE_BUDGET_MIB`, else `NATIVE`), and no more than the
/// held bytes and the share of what other processes leave of the card
/// (`Gpu::free_device_memory`), so a card they fill refuses a build by
/// name before it is made.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn native(gpu: &crate::Gpu, held: u64) -> u64 {
    let stated = std::env::var("TELPERION_DEVICE_BUDGET_MIB").ok();
    let stated = stated.and_then(|m| m.trim().parse::<u64>().ok()).map_or(NATIVE, |m| m << 20);
    let free = gpu.free_device_memory().map_or(u64::MAX, |f| held + (f as f64 * FREE_SHARE) as u64);
    stated.min(free)
}

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

/// What a build may do under the budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admitted {
    /// It fits beside the tree drawn now.
    Beside,
    /// It fits once the drawn tree is let go, and only then.
    MakeRoom,
}

/// Whether `family` built now fits `budget`: beside the `held` bytes, or
/// only once the `drawn` tree's share of them is let go; refused by name
/// where it fits in neither.
pub(super) fn admit(
    family: &Family,
    held: u64,
    drawn: u64,
    budget: u64,
) -> crate::Result<Admitted> {
    let Some(species) = family.space.species.as_ref() else {
        return Ok(Admitted::Beside);
    };
    let expected = telperion_space::expected_kept(species, family.age as u32);
    // No expectation (a species that reads light has none): the build is
    // held to the budget as it grows, by the bytes it holds, and makes
    // room first wherever a tree is drawn, so it never grows beside one it
    // cannot be shown to fit with (fn-256 task 4 review).
    let Ok(Some(kept)) = expected else {
        return match (held.saturating_sub(drawn) < budget, drawn > 0) {
            (false, _) => Err(full().into()),
            (true, true) => Ok(Admitted::MakeRoom),
            (true, false) => Ok(Admitted::Beside),
        };
    };
    let peak = peak(kept);
    if held.saturating_add(peak) <= budget {
        return Ok(Admitted::Beside);
    }
    if held.saturating_sub(drawn).saturating_add(peak) <= budget {
        return Ok(Admitted::MakeRoom);
    }
    Err(telperion_core::Error::ResourceLimit(
        "the browser's device budget: the tree's expected peak exceeds the 6 GiB \
         a page may hold even with the drawn tree let go; grow it younger",
    )
    .into())
}

/// The refusal where the page already holds its whole budget.
fn full() -> telperion_core::Error {
    telperion_core::Error::ResourceLimit(
        "the browser's device budget: the page already holds the 6 GiB it may, with the drawn tree let go",
    )
}

/// Whether `family` built now fits `budget` beside the `held` bytes, the
/// check a build makes as it starts, after any room was made.
pub(super) fn check(family: &Family, held: u64, budget: u64) -> crate::Result<()> {
    // Nothing is drawn to let go: it fits beside `held` or is refused.
    admit(family, held, 0, budget).map(|_| ())
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
        // Beside a drawn 12 (1,757 MiB held, the tree's share of it 1,700),
        // an 80 fits only once the 12 is let go.
        let admitted = |age, h: u64, d: u64| admit(&spruce(age), held(h), held(d), BROWSER);
        assert_eq!(admitted(80.0, 1_757, 1_700).unwrap(), Admitted::MakeRoom);
        assert_eq!(admitted(12.0, 1_757, 1_700).unwrap(), Admitted::Beside);
        // Past the budget even alone: refused, the drawn tree kept.
        let alone = admitted(80.0, 1_757, 0).unwrap_err().to_string();
        assert!(alone.contains("even with the drawn tree let go"), "{alone}");
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

    /// fn-256 task 4 review: a species with no expectation (the ordinary
    /// reads light) never grows beside a drawn tree, and is refused where
    /// the page holds its whole budget even with that tree let go; its
    /// build is held to the budget as it grows (`route_tests`).
    #[test]
    fn a_tree_with_no_expectation_makes_room_first() {
        let ordinary = telperion_core::presets::by_identity("ordinary").unwrap();
        let species = ordinary.space.species.as_ref().unwrap();
        assert!(telperion_space::expected_kept(species, 40).unwrap().is_none());
        let admitted = |held: u64, drawn: u64| admit(&ordinary, held << 20, drawn << 20, BROWSER);
        assert_eq!(admitted(0, 0).unwrap(), Admitted::Beside);
        assert_eq!(admitted(1_757, 1_700).unwrap(), Admitted::MakeRoom);
        let full = admitted(6 << 10, 0).unwrap_err().to_string();
        assert!(full.contains("device budget"), "{full}");
    }
}
