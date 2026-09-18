//! What every cross-resolution contract shares: the 2x2 box agreement between
//! a full-size still and its half-size draw, a wood mask read off the clay
//! room, and the receipt each test writes on a green run so the margin is
//! known before the next shading change (fn-71).
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

/// Where a resolution test's receipt lands: the spec's evidence, one JSON
/// file per test, each row a fixture with its measured mean and p95 in code
/// values and the bound it was held to.
fn receipt_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../.flow/evidence/fn71/resolution")
        .join(format!("{name}.json"))
}

/// Writes the test's measured margins. A receipt that cannot be written is a
/// failure: the margin the next change needs would be unknown.
pub fn record(name: &str, rows: &[(String, f64, f64, f64)]) {
    let path = receipt_path(name);
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
