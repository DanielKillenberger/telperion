//! Evidence for the bark-distance gate (fn-351): what
//! `tests/bark_distance.rs` renders for the oak on seed 7 at 2x and 4x, and
//! how much the trunk strip shimmers as the camera slides along its ray.
//!
//! `bark_distance_frames <out-dir>` writes the stills, the enlarged strip
//! crops and the frame contact sheets, and prints one JSON line a distance.
//! The masked agreement is the test's own (`tests/common/resolution.rs`):
//! the full-size still box-reduced 2x2 onto the half-size grid, the absolute
//! difference per channel over the eroded clay wood mask.
use std::path::{Path, PathBuf};

use telperion_core::{math::Vec3, mesh, presets::Preset};
use telperion_render::{render, Camera, Gpu, Renderer, Still, View, STILL_FORMAT};

const RAY: Vec3 = Vec3::new(1.7307636095778745, 0.2967023330704928, -1.7307636095778745);
const TARGET: Vec3 = Vec3::new(0.0, 2.0, 0.0);
/// The crop about the strip: the test's rows 200..300 with a margin.
const CROP_X: std::ops::Range<usize> = 340..460;
const CROP_Y: std::ops::Range<usize> = 192..308;

/// An RGB picture on the low grid.
struct Image {
    width: usize,
    height: usize,
    rgb: Vec<u8>,
}

impl Image {
    fn from_still(still: &Still) -> Self {
        let rgb = still.rgba.chunks(4).flat_map(|p| [p[0], p[1], p[2]]).collect();
        let (width, height) = (still.width as usize, still.height as usize);
        Self { width, height, rgb }
    }

    /// The full-size still reduced the way `masked_agreement` reads it, as
    /// floats so no rounding enters the comparison.
    fn reduced(high: &Still) -> Vec<f64> {
        let (hw, w, h) = (high.width as usize, high.width as usize / 2, high.height as usize / 2);
        let mut out = vec![0.0; w * h * 3];
        for y in 0..h {
            for x in 0..w {
                for c in 0..3 {
                    let at = |dx: usize, dy: usize| {
                        f64::from(high.rgba[((y * 2 + dy) * hw + x * 2 + dx) * 4 + c])
                    };
                    out[(y * w + x) * 3 + c] = (at(0, 0) + at(1, 0) + at(0, 1) + at(1, 1)) / 4.0;
                }
            }
        }
        out
    }

    fn get(&self, x: usize, y: usize) -> [u8; 3] {
        let at = (y * self.width + x) * 3;
        [self.rgb[at], self.rgb[at + 1], self.rgb[at + 2]]
    }

    fn write(&self, path: &Path) {
        let file = std::fs::File::create(path).unwrap();
        let mut encoder =
            png::Encoder::new(std::io::BufWriter::new(file), self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.write_header().unwrap().write_image_data(&self.rgb).unwrap();
    }
}

/// The test's wood mask, eroded by four pixels, as a per-pixel flag.
fn wood(clay: &Still) -> Vec<bool> {
    let (w, h) = (clay.width as usize, clay.height as usize);
    let warm = |x: usize, y: usize| clay.rgba[(y * w + x) * 4] > clay.rgba[(y * w + x) * 4 + 2];
    let mut mask = vec![false; w * h];
    for y in 4..h - 4 {
        for x in 4..w - 4 {
            mask[y * w + x] = (y - 4..=y + 4).all(|sy| (x - 4..=x + 4).all(|sx| warm(sx, sy)));
        }
    }
    mask
}

/// The test's strip: rows 200..300 and the columns 100 / distance about the
/// centre, as the test computes them for an integer distance.
fn strip(mask: &[bool], width: usize, distance: f64) -> Vec<(usize, usize)> {
    let half = (100.0 / distance) as usize;
    (200..300)
        .flat_map(|y| (400 - half..400 + half).map(move |x| (x, y)))
        .filter(|&(x, y)| mask[y * width + x])
        .collect()
}

/// The trunk's mean width in pixels over rows 200..300: the run of warm clay
/// through the column nearest the centre that is warm on that row.
fn trunk_width(clay: &Still) -> f64 {
    let w = clay.width as usize;
    let warm = |x: usize, y: usize| clay.rgba[(y * w + x) * 4] > clay.rgba[(y * w + x) * 4 + 2];
    let widths = (200..300).filter_map(|y| {
        let seed = (0..60).flat_map(|d| [400 - d, 400 + d]).find(|&x| warm(x, y))?;
        let left = (0..seed).rev().take_while(|&x| warm(x, y)).count();
        let right = (seed..w).take_while(|&x| warm(x, y)).count();
        Some((left + right) as f64)
    });
    let widths: Vec<f64> = widths.collect();
    widths.iter().sum::<f64>() / widths.len().max(1) as f64
}

/// Mean and p95 of a pooled set of absolute differences, in code values.
fn summary(mut errors: Vec<f64>) -> (f64, f64) {
    let mean = errors.iter().sum::<f64>() / errors.len() as f64;
    errors.sort_by(f64::total_cmp);
    (mean, errors[errors.len() * 95 / 100])
}

fn camera(distance: f64) -> Camera {
    Camera {
        target: TARGET,
        position: TARGET + RAY * distance,
        field_of_view: 38.0,
        near: 0.01,
        far: 1000.0,
    }
}

/// One view drawn at the low and the full size, the full one reduced.
struct Pair {
    clay: Still,
    low: Still,
    reduced: Vec<f64>,
}

fn draw(renderer: &mut Renderer, distance: f64) -> Pair {
    let camera = camera(distance);
    renderer.set_view(View::Clay);
    let clay = render(renderer, &camera, 800, 500).unwrap();
    renderer.set_view(View::Bare);
    let high = render(renderer, &camera, 1600, 1000).unwrap();
    let low = render(renderer, &camera, 800, 500).unwrap();
    Pair { clay, low, reduced: Image::reduced(&high) }
}

/// The crop enlarged by `scale`, nearest neighbour; pixels off `mask` dimmed.
fn crop(pixel: impl Fn(usize, usize) -> [u8; 3], mask: &[bool], scale: usize) -> Image {
    let (w, h) = (CROP_X.len() * scale, CROP_Y.len() * scale);
    let mut rgb = Vec::with_capacity(w * h * 3);
    for y in 0..h {
        for x in 0..w {
            let (sx, sy) = (CROP_X.start + x / scale, CROP_Y.start + y / scale);
            let on = mask[sy * 800 + sx];
            rgb.extend(pixel(sx, sy).map(|v| if on { v } else { v / 3 }));
        }
    }
    Image { width: w, height: h, rgb }
}

fn side_by_side(parts: &[Image], gap: usize) -> Image {
    let height = parts.iter().map(|p| p.height).max().unwrap();
    let width = parts.iter().map(|p| p.width).sum::<usize>() + gap * (parts.len() - 1);
    let mut rgb = vec![255u8; width * height * 3];
    let mut left = 0;
    for part in parts {
        for y in 0..part.height {
            let row = &part.rgb[y * part.width * 3..(y + 1) * part.width * 3];
            rgb[(y * width + left) * 3..(y * width + left + part.width) * 3].copy_from_slice(row);
        }
        left += part.width + gap;
    }
    Image { width, height, rgb }
}

fn stacked(rows: Vec<Image>, gap: usize) -> Image {
    let width = rows.iter().map(|r| r.width).max().unwrap();
    let mut rgb = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        if i > 0 {
            rgb.extend(std::iter::repeat_n(255u8, width * gap * 3));
        }
        for y in 0..row.height {
            rgb.extend_from_slice(&row.rgb[y * row.width * 3..(y + 1) * row.width * 3]);
            rgb.extend(std::iter::repeat_n(255u8, (width - row.width) * 3));
        }
    }
    let height = rgb.len() / (width * 3);
    Image { width, height, rgb }
}

/// The still pair at one test distance: agreement, crops and stills.
fn still_pair(renderer: &mut Renderer, distance: f64, out: &Path) -> String {
    let pair = draw(renderer, distance);
    let mask = wood(&pair.clay);
    let pixels = strip(&mask, 800, distance);
    let low = Image::from_still(&pair.low);
    let errors = pixels.iter().flat_map(|&(x, y)| {
        (0..3).map(move |c| (y * 800 + x) * 3 + c)
    });
    let errors = errors.map(|i| (pair.reduced[i] - f64::from(low.rgb[i])).abs()).collect();
    let (mean, p95) = summary(errors);
    let mut on = vec![false; 800 * 500];
    pixels.iter().for_each(|&(x, y)| on[y * 800 + x] = true);
    let at = |i: usize| pair.reduced[i].round() as u8;
    let high = crop(|x, y| std::array::from_fn(|c| at((y * 800 + x) * 3 + c)), &on, 4);
    let diff = crop(
        |x, y| {
            let i = (y * 800 + x) * 3;
            let d = (0..3).map(|c| (pair.reduced[i + c] - f64::from(low.rgb[i + c])).abs());
            let d = d.fold(0.0, f64::max);
            [(d * 255.0 / 16.0).min(255.0) as u8; 3]
        },
        &on,
        4,
    );
    let tag = format!("{distance}x");
    side_by_side(&[high, crop(|x, y| low.get(x, y), &on, 4), diff], 8)
        .write(&out.join(format!("strip-{tag}.png")));
    low.write(&out.join(format!("bare-low-{tag}.png")));
    Image::from_still(&pair.clay).write(&out.join(format!("clay-{tag}.png")));
    format!(
        "\"trunk_px\": {:.1}, \"mask_px\": {}, \"mean\": {mean:.4}, \"p95\": {p95:.2}",
        trunk_width(&pair.clay),
        pixels.len()
    )
}

/// Twelve frames slid along the ray over +-2.5% of `distance`: consecutive
/// frame differences on the shared strip mask, low and reduced full size.
fn shimmer(renderer: &mut Renderer, distance: f64, out: &Path) -> String {
    let frames: Vec<Pair> = (0..12)
        .map(|i| draw(renderer, distance * (0.975 + 0.05 * f64::from(i) / 11.0)))
        .collect();
    let masks: Vec<Vec<bool>> = frames.iter().map(|f| wood(&f.clay)).collect();
    let (mut low, mut reference) = (Vec::new(), Vec::new());
    for i in 0..11 {
        let shared = strip(&masks[i], 800, distance)
            .into_iter()
            .filter(|&(x, y)| masks[i + 1][y * 800 + x]);
        for (x, y) in shared {
            for c in 0..3 {
                let k = (y * 800 + x) * 4 + c;
                let a = f64::from(frames[i].low.rgba[k]) - f64::from(frames[i + 1].low.rgba[k]);
                low.push(a.abs());
                let k = (y * 800 + x) * 3 + c;
                reference.push((frames[i].reduced[k] - frames[i + 1].reduced[k]).abs());
            }
        }
    }
    let crops = frames.iter().zip(&masks).map(|(f, m)| {
        let image = Image::from_still(&f.low);
        crop(|x, y| image.get(x, y), m, 2)
    });
    let crops: Vec<Image> = crops.collect();
    let rows = crops.chunks(6).map(|row| side_by_side(row, 4));
    stacked(rows.collect(), 4).write(&out.join(format!("frames-{distance}x.png")));
    let (lm, lp) = summary(low);
    let (rm, rp) = summary(reference);
    format!(
        "\"shimmer_low_mean\": {lm:.4}, \"shimmer_low_p95\": {lp:.2}, \
         \"shimmer_ref_mean\": {rm:.4}, \"shimmer_ref_p95\": {rp:.2}"
    )
}

fn main() {
    let out = PathBuf::from(std::env::args().nth(1).expect("usage: bark_distance_frames <out>"));
    std::fs::create_dir_all(&out).unwrap();
    let gpu = pollster::block_on(Gpu::request(None)).expect("a device");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 7;
    let tree = mesh::build(&family).unwrap();
    renderer.submit(&tree).unwrap();
    renderer.set_material(family.material);
    // The stills in the test's own order, first after the submit: the
    // renderer keeps state from frame to frame (the sun's map, the wood's
    // surfacing), and a different history moved the mask by 2-4% here.
    let stills = [2.0, 4.0].map(|distance| still_pair(&mut renderer, distance, &out));
    for (distance, still) in [2.0, 4.0].into_iter().zip(stills) {
        let motion = shimmer(&mut renderer, distance, &out);
        println!("{{\"distance\": {distance}, {still}, {motion}}}");
    }
    // The noise floor: the same low view drawn twice.
    renderer.set_view(View::Bare);
    let a = render(&mut renderer, &camera(4.0), 800, 500).unwrap();
    let b = render(&mut renderer, &camera(4.0), 800, 500).unwrap();
    println!("{{\"repeat_identical\": {}}}", a.rgba == b.rgba);
}
