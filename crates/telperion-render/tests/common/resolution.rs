//! What every cross-resolution contract shares: the 2x2 box agreement between
//! a full-size still and its half-size draw, a wood mask read off the clay
//! room, and the receipt each test writes on a green run so the margin is
//! known before the next shading change (fn-71). A plain run leaves that
//! receipt in the target temp directory; `TELPERION_RECORD_EVIDENCE=1` is how a
//! change to bark resolution refreshes the committed margins on purpose.
use std::path::PathBuf;

use telperion_render::Still;

pub fn masked_agreement(high: &Still, low: &Still, mask: &[(usize, usize)]) -> (f64, f64) {
    let (hw, lw) = (high.width as usize, low.width as usize);
    let mut errors = Vec::new();
    for &(x, y) in mask {
        for c in 0..3 {
            let sum: u32 = (0..2)
                .flat_map(|dy| (0..2).map(move |dx| (dx, dy)))
                .map(|(dx, dy)| u32::from(high.rgba[((y * 2 + dy) * hw + x * 2 + dx) * 4 + c]))
                .sum();
            errors.push((f64::from(sum) / 4.0 - f64::from(low.rgba[(y * lw + x) * 4 + c])).abs());
        }
    }
    let mean = errors.iter().sum::<f64>() / errors.len() as f64;
    errors.sort_by(f64::total_cmp);
    (mean, errors[errors.len() * 95 / 100])
}

/// Pixels the clay room paints as wood, eroded clear of every silhouette: the
/// clay is warm and the room behind it is cool, so red over blue is wood.
#[allow(dead_code)]
pub fn wood_mask(clay: &Still) -> Vec<(usize, usize)> {
    let (w, h) = (clay.width as usize, clay.height as usize);
    let warm = |x: usize, y: usize| {
        let at = (y * w + x) * 4;
        clay.rgba[at] > clay.rgba[at + 2]
    };
    (4..h - 4)
        .flat_map(|y| (4..w - 4).map(move |x| (x, y)))
        .filter(|&(x, y)| (y - 4..=y + 4).all(|sy| (x - 4..=x + 4).all(|sx| warm(sx, sy))))
        .collect()
}

/// Where a resolution test's receipt lands: one JSON file per test, each row a
/// fixture with its measured mean and p95 in code values and the bound it was
/// held to. A run that was not asked for the evidence keeps its receipt in the
/// target temp directory, so it decides nothing about what the tracked evidence
/// says; only `TELPERION_RECORD_EVIDENCE=1` names the spec's evidence.
fn receipt_path(name: &str, to_evidence: bool) -> PathBuf {
    let directory = if to_evidence {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.flow/evidence/fn71/resolution")
    } else {
        PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("resolution")
    };
    directory.join(format!("{name}.json"))
}

/// Writes the test's measured margins. A receipt that cannot be written is a
/// failure: the margin the next change needs would be unknown.
pub fn record(name: &str, rows: &[(String, f64, f64, f64)]) {
    let asked_for = std::env::var("TELPERION_RECORD_EVIDENCE").as_deref() == Ok("1");
    let path = receipt_path(name, asked_for);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let body = rows
        .iter()
        .map(|(fixture, mean, p95, bound)| {
            format!(
                "  {{\"fixture\": \"{fixture}\", \"mean\": {mean:.4}, \"p95\": {p95:.2}, \"bound\": {bound}}}"
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");
    std::fs::write(&path, format!("[\n{body}\n]\n"))
        .unwrap_or_else(|error| panic!("receipt {}: {error}", path.display()));
    assert!(
        path.is_file(),
        "receipt missing after write: {}",
        path.display()
    );
}

/// Both settings, without a device: the run nobody asked for the evidence from
/// writes beside the test binary, and only the asked-for run names the tracked
/// file, in the same place and under the same name as before.
#[test]
fn only_an_asked_for_run_names_the_tracked_evidence() {
    let asked = receipt_path("smooth_bark", true);
    assert!(
        asked.ends_with(".flow/evidence/fn71/resolution/smooth_bark.json"),
        "asked-for receipt moved: {}",
        asked.display()
    );

    let plain = receipt_path("smooth_bark", false);
    assert_eq!(
        plain,
        PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("resolution/smooth_bark.json"),
        "a plain run must keep its receipt in the target temp directory"
    );
    assert!(
        !plain.components().any(|part| part.as_os_str() == "evidence"),
        "a plain run reached the tracked evidence: {}",
        plain.display()
    );
}
