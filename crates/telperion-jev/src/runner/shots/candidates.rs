//! The candidates a found photograph's shot is chosen from (host,
//! 2026-09-26). Code owns every number here: the cameras, the tree boxes
//! and crown-base lines drawn over the photograph, and the light presets.
//! A look only names one of each, or none.
use serde_json::{json, Value};

/// The hero pose's direction; a found photograph's bearing is unknown and
/// a crown reads the same from any side at these fills.
pub const AZIMUTH: f64 = 115.0;
/// A photograph's usual lens.
pub const FOV: f64 = 40.0;
/// Breast height, where a bark close-up is aimed.
pub const BREAST_HEIGHT_M: f64 = 1.3;

/// Six cameras for a view: a whole or a bare tree framed full height at
/// three elevations of the eye below the aim and two fills; bark aimed at
/// breast height from three distances through two lenses.
pub fn cameras(view: &str, height_m: f64) -> Vec<Value> {
    let mut out = Vec::new();
    if view == "bark" {
        let aim = (BREAST_HEIGHT_M / height_m.max(BREAST_HEIGHT_M)).clamp(0.0, 1.0);
        for distance in [1.2, 2.0, 3.5] {
            for fov in [35.0, 55.0] {
                out.push(json!({"azimuth": AZIMUTH, "elevation": 0.0, "fill": 1.0,
                    "fov": fov, "targetHeight": aim, "distance": distance}));
            }
        }
        return out;
    }
    for elevation in [-15.0, -8.0, 0.0] {
        for fill in [0.8, 0.93] {
            out.push(json!({"azimuth": AZIMUTH, "elevation": elevation, "fill": fill,
                "fov": FOV, "targetHeight": 0.5}));
        }
    }
    out
}

/// Box scales around the render's tree box, (width, height): the render's
/// own, clearly narrower and wider, shorter and taller, and wider and
/// taller together, so the photograph's own shape can win.
const SCALES: [(f64, f64); 6] = [
    (1.0, 1.0),
    (0.7, 1.0),
    (1.4, 1.0),
    (1.0, 0.8),
    (1.0, 1.2),
    (1.3, 1.15),
];

/// The labelled candidate boxes, `[x, y, w, h]` as fractions of the
/// photograph, each scaled about the render box's bottom centre (the tree
/// stands where the render's does) and clipped to the frame.
pub fn boxes(render: [f64; 4]) -> Vec<(String, [f64; 4])> {
    let [x, y, w, h] = render;
    let (centre, bottom) = (x + w / 2.0, y + h);
    SCALES
        .iter()
        .enumerate()
        .map(|(i, (sw, sh))| {
            let (bw, bh) = (w * sw, h * sh);
            let left = (centre - bw / 2.0).max(0.0);
            let right = (centre + bw / 2.0).min(1.0);
            let top = (bottom - bh).max(0.0);
            let label = ((b'A' + i as u8) as char).to_string();
            (label, [round(left), round(top), round(right - left), round(bottom - top)])
        })
        .collect()
}

/// Crown-base heights over the render box, as fractions of its height.
const BASES: [f64; 5] = [0.05, 0.15, 0.25, 0.35, 0.5];

/// The labelled crown-base lines, each the fraction of the photograph's
/// height down from its top where the line is drawn.
pub fn lines(render: [f64; 4]) -> Vec<(String, f64)> {
    let [_, y, _, h] = render;
    BASES
        .iter()
        .enumerate()
        .map(|(i, f)| ((i + 1).to_string(), round(y + h - f * h)))
        .collect()
}

/// A chosen line's crown base: its height above the chosen box's bottom as
/// a fraction of the box's height.
pub fn crown_base(tree: [f64; 4], line: f64) -> f64 {
    let [_, y, _, h] = tree;
    round(((y + h - line) / h).clamp(0.0, 1.0))
}

/// The light presets a photograph is matched to: overcast, and a low sun
/// on the camera's left, right or behind the tree.
pub fn lights(azimuth: f64) -> Vec<(&'static str, &'static str, Value)> {
    let sun = |offset: f64| {
        json!({"overcast": 0.2, "sunAzimuth": (azimuth + offset).rem_euclid(360.0), "sunElevation": 35.0})
    };
    vec![
        ("overcast", "a flat, overcast sky: soft light, no cast shadow", overcast()),
        ("sun-left", "sunlight from the camera's left: the left side of the crown is lit", sun(90.0)),
        ("sun-right", "sunlight from the camera's right: the right side of the crown is lit", sun(-90.0)),
        ("sun-behind", "the sun behind the tree: a backlit, dark crown against a bright sky", sun(180.0)),
    ]
}

/// The overcast preset, the light the candidate cameras are drawn under
/// and the one a photograph whose light matched none is drawn under.
pub fn overcast() -> Value {
    json!({"overcast": 0.9, "sunAzimuth": 135.0, "sunElevation": 55.0})
}

fn round(v: f64) -> f64 {
    (v * 10_000.0).round() / 10_000.0
}

/// A photograph's pixel size, from a PNG's header or a JPEG's frame marker.
pub fn image_size(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.starts_with(b"\x89PNG") && bytes.len() >= 24 {
        let be = |at: usize| u32::from_be_bytes(bytes[at..at + 4].try_into().unwrap());
        return Some((be(16), be(20)));
    }
    if !bytes.starts_with(&[0xFF, 0xD8]) {
        return None;
    }
    let mut at = 2;
    while at + 9 < bytes.len() {
        if bytes[at] != 0xFF {
            return None;
        }
        let marker = bytes[at + 1];
        let length = u16::from_be_bytes([bytes[at + 2], bytes[at + 3]]) as usize;
        let frame = (0xC0..=0xCF).contains(&marker) && ![0xC4, 0xC8, 0xCC].contains(&marker);
        if frame {
            let be = |i: usize| u16::from_be_bytes([bytes[i], bytes[i + 1]]) as u32;
            return Some((be(at + 7), be(at + 5)));
        }
        at += 2 + length;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_cameras_frame_each_view() {
        assert_eq!(cameras("leaf-on", 30.0).len(), 6);
        let bark = cameras("bark", 26.0);
        assert_eq!(bark.len(), 6);
        assert!((bark[0]["targetHeight"].as_f64().unwrap() - 0.05).abs() < 1e-9);
    }

    /// The boxes stand on the render box's bottom centre and include
    /// narrower, wider, shorter and taller ones; a line's crown base is
    /// read against the box chosen.
    #[test]
    fn the_boxes_stand_where_the_render_does_and_the_lines_read_against_the_chosen_one() {
        let render = [0.3, 0.1, 0.4, 0.8];
        let boxes = boxes(render);
        assert_eq!(boxes.len(), 6);
        assert_eq!(boxes[0], ("A".to_string(), render));
        for (_, b) in &boxes {
            assert!((b[1] + b[3] - 0.9).abs() < 1e-4, "{b:?}");
            assert!((b[0] + b[2] / 2.0 - 0.5).abs() < 1e-4 || b[0] == 0.0, "{b:?}");
        }
        assert!(boxes[1].1[2] < 0.4 && boxes[2].1[2] > 0.4);
        assert!(boxes[3].1[3] < 0.8 && boxes[4].1[3] > 0.8);
        let lines = lines(render);
        assert_eq!(lines[1], ("2".to_string(), 0.78));
        assert_eq!(crown_base(render, 0.78), 0.15);
        assert_eq!(crown_base(boxes[3].1, 0.78), round(0.12 / 0.64));
    }

    #[test]
    fn the_sun_presets_turn_with_the_camera() {
        let lights = lights(300.0);
        assert_eq!(lights[0].0, "overcast");
        assert_eq!(lights[1].2["sunAzimuth"], 30.0);
        assert_eq!(lights[3].2["sunAzimuth"], 120.0);
    }

    #[test]
    fn a_photographs_size_is_read_from_its_header() {
        let mut png = b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR".to_vec();
        png.extend([0, 0, 2, 0, 0, 0, 1, 0]);
        assert_eq!(image_size(&png), Some((512, 256)));
        let jpeg = [
            0xFF, 0xD8, 0xFF, 0xE0, 0, 4, 0, 0, 0xFF, 0xC0, 0, 11, 8, 0x03, 0x84, 0x02, 0x58, 3, 0,
        ];
        assert_eq!(image_size(&jpeg), Some((600, 900)));
    }
}
