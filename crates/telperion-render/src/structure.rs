//! What separates bark from brickwork, as six numbers.
//!
//! fn-32's first round measured a still's tone and its band period and called
//! that structure. Both survive here, because both still carry information,
//! but neither can tell a wall of plates from a tree: a brick wall and an oak
//! can share a dark fraction and a band period exactly. The owner's round-one
//! verdict - "looks like armor plating in some cases" - is about what those
//! two numbers do not see, so this module adds the four that do.
//!
//! Every measurement is taken on the same normalisation fn-32 established:
//! the centre 400x400 crop, turned grey by Rec.709 luminance in linear light,
//! auto-levelled so a photograph's exposure cannot decide the answer, and
//! thresholded at 45%. A still and a photograph therefore arrive at the
//! statistics on equal terms.
use std::path::Path;

use crate::{RenderError, Result};

mod shape;

/// The crop every measurement is taken on, in pixels.
pub const CROP: usize = 400;
/// Where light stops and dark begins, on the auto-levelled grey.
const THRESHOLD: f64 = 0.45;

/// One still's structure. Read together they say whether a picture is a
/// network grown by a tree or a pattern laid by a mason.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Structure {
    /// Entropy of the Sobel gradient direction over 18 bins, weighted by
    /// magnitude and normalised to 0..1. Brickwork puts every edge on two
    /// angles and measures low; bark spreads its edges and measures high.
    pub orientation_entropy: f64,
    /// Coefficient of variation of the light regions' areas. Plates cut to
    /// one size measure near zero, which is precisely what armour is.
    pub area_variation: f64,
    /// Mean ratio of boundary length to end-to-end chord over dark-region
    /// boundary segments of a fixed length. A straight course measures 1;
    /// a furrow that wanders measures above it.
    pub furrow_curvature: f64,
    /// Mean number of dark arms meeting at a skeleton branch point. Four-way
    /// crossings are a lattice; three-way junctions are how bark branches.
    pub junction_arms: f64,
    /// The share of the crop below the threshold, unchanged from round one.
    pub dark_fraction: f64,
    /// 400 divided by the light-to-dark transitions per scanline, in pixels
    /// of the crop. Sound between two stills; indicative against a photograph
    /// whose physical scale is unrecorded.
    pub furrow_period: f64,
}

/// What one unit of difference is worth, per component: the spread the seven
/// catalogued references and the fn-32 wood stills actually occupy on that
/// axis, measured before any weight was chosen, so no component's units
/// decide the distance by themselves.
const SCALES: Structure = Structure {
    orientation_entropy: 0.05,
    area_variation: 0.70,
    furrow_curvature: 0.45,
    junction_arms: 0.50,
    dark_fraction: 0.25,
    furrow_period: 1.00,
};

/// What each component is worth, and why.
///
/// - **Area variation, 1.4.** The owner's word was "armor plating", and
///   plates cut to one size are what armour is. It is the only component
///   that sees that directly, and it is where the round-one stills are
///   furthest from the references: the oak trunk measures 0.000 against
///   1.34, 1.92 and 2.61 for its three.
/// - **Furrow curvature, 1.2.** Task one caught cuts running in straight
///   courses by eye and no number in that round would have. This is that
///   defect as a number.
/// - **Dark fraction, 0.8.** How much of the crop the furrows hold. Real,
///   and partly dependent on how close the photograph was taken.
/// - **Junction arms, 0.6.** How richly the network converges. A perfect
///   lattice measures 4.0 against a jittered network's 3.0 in the guard
///   test, but on a thresholded photograph it reads as how many furrows
///   meet, so it supports the verdict rather than steering it.
/// - **Orientation entropy, 0.4.** Every photograph and every still measures
///   between 0.90 and 1.00: it would catch a lattice but cannot distinguish
///   one bark from another.
/// - **Furrow period, 0.2.** The oak still's 61 px crop period is about a
///   6.5 cm plate at the trunk pose's known 0.42 m of wood across the crop,
///   which is inside the 3-8 cm a white oak actually wears. The references'
///   8-18 px is how close those photographs were taken, and their physical
///   scale is unrecorded. The period therefore guards against a wild change
///   of scale and is not allowed to drive one.
const WEIGHTS: Structure = Structure {
    orientation_entropy: 0.4,
    area_variation: 1.4,
    furrow_curvature: 1.2,
    junction_arms: 0.6,
    dark_fraction: 0.8,
    furrow_period: 0.2,
};

impl Structure {
    fn components(&self) -> [f64; 6] {
        [
            self.orientation_entropy,
            self.area_variation,
            self.furrow_curvature,
            self.junction_arms,
            self.dark_fraction,
            self.furrow_period,
        ]
    }

    /// The normalised weighted distance to another structure, zero when the
    /// two are the same picture. The period enters as a log ratio: a still
    /// twice a reference's scale is as far from it as one half its scale.
    pub fn distance(&self, other: &Self) -> f64 {
        let (mine, theirs) = (self.components(), other.components());
        let (scales, weights) = (SCALES.components(), WEIGHTS.components());
        let mut sum = 0.0;
        for i in 0..6 {
            let difference = if i == 5 {
                (mine[i].max(1e-6) / theirs[i].max(1e-6)).log2()
            } else {
                mine[i] - theirs[i]
            };
            sum += weights[i] * (difference / scales[i]).powi(2);
        }
        (sum / weights.iter().sum::<f64>()).sqrt()
    }
}

/// The middle of a set of references, which is what a still with more than
/// one catalogued reference is steered towards: no single photograph of a
/// species is the species. The period is averaged geometrically, because it
/// is compared as a ratio.
pub fn centroid(of: &[Structure]) -> Structure {
    let count = of.len().max(1) as f64;
    let mean = |pick: fn(&Structure) -> f64| of.iter().map(pick).sum::<f64>() / count;
    Structure {
        orientation_entropy: mean(|s| s.orientation_entropy),
        area_variation: mean(|s| s.area_variation),
        furrow_curvature: mean(|s| s.furrow_curvature),
        junction_arms: mean(|s| s.junction_arms),
        dark_fraction: mean(|s| s.dark_fraction),
        furrow_period: mean(|s| s.furrow_period.max(1e-6).ln()).exp(),
    }
}

/// The crop's mean colour, in code values, so a climb steered by a greyscale
/// statistic cannot pay for structure with the colour fn-29 was judged on.
/// A plain arithmetic mean of the same centre crop: it is a drift guard, not
/// a photometric measurement, and the stills' recorded means stay the ones
/// round one took with ImageMagick.
pub fn crop_mean(rgba: &[u8], width: usize, height: usize) -> Result<[f64; 3]> {
    if width < CROP || height < CROP {
        return Err(RenderError::Output {
            path: String::new(),
            message: format!("{width}x{height} is smaller than the {CROP}x{CROP} crop"),
        });
    }
    let (x0, y0) = ((width - CROP) / 2, (height - CROP) / 2);
    let mut sums = [0u64; 3];
    for y in 0..CROP {
        for x in 0..CROP {
            let at = ((y0 + y) * width + x0 + x) * 4;
            for (sum, channel) in sums.iter_mut().zip(0..3) {
                *sum += u64::from(rgba[at + channel]);
            }
        }
    }
    Ok(sums.map(|sum| sum as f64 / (CROP * CROP) as f64))
}

/// Measure an RGBA frame as it stands, without writing it out first: the
/// hill climb measures thousands of frames and none of them is a file.
pub fn measure_frame(rgba: &[u8], width: usize, height: usize) -> Result<Structure> {
    measure_pixels(rgba, width, height, 4)
}

/// Measure a PNG on disk - a still, or a reference photograph converted to
/// PNG without being modified in any other way.
pub fn measure_png(path: &Path) -> Result<Structure> {
    let fail = |message: String| RenderError::Output {
        path: path.display().to_string(),
        message,
    };
    let file = std::fs::File::open(path).map_err(|e| fail(e.to_string()))?;
    let mut decoder = png::Decoder::new(std::io::BufReader::new(file));
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().map_err(|e| fail(e.to_string()))?;
    let size = reader
        .output_buffer_size()
        .ok_or_else(|| fail("a PNG of a stated size".into()))?;
    let mut pixels = vec![0; size];
    let info = reader
        .next_frame(&mut pixels)
        .map_err(|e| fail(e.to_string()))?;
    let stride = match info.color_type {
        png::ColorType::Grayscale => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
        other => return Err(fail(format!("unsupported colour type {other:?}"))),
    };
    measure_pixels(&pixels, info.width as usize, info.height as usize, stride)
}

fn measure_pixels(pixels: &[u8], width: usize, height: usize, stride: usize) -> Result<Structure> {
    if width < CROP || height < CROP {
        return Err(RenderError::Output {
            path: String::new(),
            message: format!("{width}x{height} is smaller than the {CROP}x{CROP} crop"),
        });
    }
    let grey = levelled_crop(pixels, width, height, stride);
    let dark: Vec<bool> = grey.iter().map(|&v| v < THRESHOLD).collect();
    let light: Vec<bool> = dark.iter().map(|&d| !d).collect();
    let transitions = (0..CROP)
        .map(|y| {
            (1..CROP)
                .filter(|&x| dark[y * CROP + x] && !dark[y * CROP + x - 1])
                .count()
        })
        .sum::<usize>() as f64
        / CROP as f64;
    Ok(Structure {
        orientation_entropy: orientation_entropy(&grey),
        area_variation: shape::area_variation(&light),
        furrow_curvature: shape::curvature(&light),
        junction_arms: shape::junction_arms(&dark),
        dark_fraction: dark.iter().filter(|&&d| d).count() as f64 / (CROP * CROP) as f64,
        furrow_period: if transitions > 0.0 {
            CROP as f64 / transitions
        } else {
            CROP as f64
        },
    })
}

/// The centre crop, turned grey the way ImageMagick's `-colorspace Gray`
/// does - Rec.709 luminance in linear light, encoded back to sRGB - and then
/// stretched so its darkest pixel is 0 and its brightest 1. The stretch is
/// what lets a bright photograph and a dim render be compared at all.
fn levelled_crop(pixels: &[u8], width: usize, height: usize, stride: usize) -> Vec<f64> {
    let (x0, y0) = ((width - CROP) / 2, (height - CROP) / 2);
    let mut grey = Vec::with_capacity(CROP * CROP);
    for y in 0..CROP {
        for x in 0..CROP {
            let at = ((y0 + y) * width + x0 + x) * stride;
            let channel = |i: usize| linear(f64::from(pixels[at + i]) / 255.0);
            let luminance = if stride <= 2 {
                linear(f64::from(pixels[at]) / 255.0)
            } else {
                0.2126 * channel(0) + 0.7152 * channel(1) + 0.0722 * channel(2)
            };
            grey.push(encode(luminance));
        }
    }
    let low = grey.iter().copied().fold(f64::INFINITY, f64::min);
    let high = grey.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let span = (high - low).max(1e-6);
    for value in &mut grey {
        *value = (*value - low) / span;
    }
    grey
}

fn linear(value: f64) -> f64 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

fn encode(value: f64) -> f64 {
    if value <= 0.0031308 {
        12.92 * value
    } else {
        1.055 * value.powf(1.0 / 2.4) - 0.055
    }
}

/// How many directions the edges point in. A Sobel gradient per pixel, its
/// direction folded into half a turn because an edge has no side, binned and
/// weighted by how strong it is; the entropy of that distribution over the
/// number of bins is the answer. Two sharp peaks - a wall of courses and
/// perpends - leave most bins empty and measure low.
fn orientation_entropy(grey: &[f64]) -> f64 {
    const BINS: usize = 18;
    let mut histogram = [0.0; BINS];
    let at = |x: usize, y: usize| grey[y * CROP + x];
    for y in 1..CROP - 1 {
        for x in 1..CROP - 1 {
            let gx = at(x + 1, y - 1) + 2.0 * at(x + 1, y) + at(x + 1, y + 1)
                - at(x - 1, y - 1)
                - 2.0 * at(x - 1, y)
                - at(x - 1, y + 1);
            let gy = at(x - 1, y + 1) + 2.0 * at(x, y + 1) + at(x + 1, y + 1)
                - at(x - 1, y - 1)
                - 2.0 * at(x, y - 1)
                - at(x + 1, y - 1);
            let magnitude = gx.hypot(gy);
            if magnitude < 1e-3 {
                continue;
            }
            let angle = gy.atan2(gx).rem_euclid(std::f64::consts::PI);
            let bin = ((angle / std::f64::consts::PI) * BINS as f64) as usize;
            histogram[bin.min(BINS - 1)] += magnitude;
        }
    }
    let total: f64 = histogram.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let entropy: f64 = histogram
        .iter()
        .filter(|&&weight| weight > 0.0)
        .map(|&weight| {
            let p = weight / total;
            -p * p.ln()
        })
        .sum();
    entropy / (BINS as f64).ln()
}
