use telperion_render::Still;

pub fn masked_agreement(high: &Still, low: &Still, mask: &[(usize, usize)]) -> (f64, f64) {
    let mut errors = Vec::new();
    // Fixed, material-independent mask inside the trunk silhouette at this
    // pinned pose. Includes the shaded side and the foreshortened right side;
    // excludes the silhouette's geometry samples, sky, ground and frame edge.
    // 116160 trunk pixels: x=280..520, y=8..492 in the half-size frame.
    for &(x, y) in mask {
        for c in 0..3 {
            let sum: u32 = (0..2)
                .flat_map(|dy| (0..2).map(move |dx| (dx, dy)))
                .map(|(dx, dy)| u32::from(high.rgba[((y * 2 + dy) * 1600 + x * 2 + dx) * 4 + c]))
                .sum();
            errors.push((f64::from(sum) / 4.0 - f64::from(low.rgba[(y * 800 + x) * 4 + c])).abs());
        }
    }
    let mean = errors.iter().sum::<f64>() / errors.len() as f64;
    errors.sort_by(f64::total_cmp);
    (mean, errors[errors.len() * 95 / 100])
}
