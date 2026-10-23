//! The film's flat overlay (fn-272 R1b), drawn into the rendered still's
//! pixels after the renderer is done with them: the corner map of species
//! space with its labelled dots, the faint trail of the path so far and
//! the current point; the age; and the closing line over the last hold.
use super::map::Map;
use font8x8::{UnicodeFonts, BASIC_FONTS};

type Rgb = [u8; 3];
const INK: Rgb = [236, 236, 230];
const SHADE: Rgb = [0, 0, 0];
const NOW: Rgb = [255, 214, 120];

/// A picture to draw over: tightly packed RGBA, top row first.
pub struct Canvas<'a> {
    pub rgba: &'a mut [u8],
    pub width: u32,
    pub height: u32,
}

/// The map laid into its corner of a frame of one size.
pub struct Overlay {
    /// The panel's left, top, width and height, pixels.
    panel: [f64; 4],
    /// Map units to pixels: the scale and the map point at the panel's
    /// centre.
    scale: f64,
    centre: [f64; 2],
    /// A letter's size in pixels: 8 times this.
    letter: u32,
    dots: Vec<(String, [f64; 2])>,
    /// Each dot's label, placed so none overlaps another (`place`).
    pub labels: Vec<Label>,
    axes: [String; 2],
}

/// A dot's name where it is written: its box's left, top, width and
/// height in pixels, and the dot it names. A label moved off its dot is
/// joined to it by a leader line.
#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub text: String,
    pub frame: [f64; 4],
    pub dot: [f64; 2],
}

impl Label {
    /// Whether the label stands away from its dot's own place, just to
    /// its right at the dot's height.
    pub fn moved(&self, letter: f64) -> bool {
        (self.frame[1] + 4.0 * letter - self.dot[1]).abs() > 0.5
            || self.frame[0] - self.dot[0] > 5.5 * letter
    }
}

/// Whether two boxes share a pixel row and column.
pub fn overlap(a: [f64; 4], b: [f64; 4]) -> bool {
    a[0] < b[0] + b[2] && b[0] < a[0] + a[2] && a[1] < b[1] + b[3] && b[1] < a[1] + a[3]
}

impl Overlay {
    /// The map in the lower right of a `width` by `height` frame; each dot
    /// named by `names` (an id and its name).
    pub fn new(map: &Map, names: &[(&str, &str)], width: u32, height: u32) -> Overlay {
        let letter = (height / 720).max(1);
        let unit = f64::from(height) / 720.0;
        let (w, h) = (340.0 * unit, 240.0 * unit);
        let panel = [
            f64::from(width) - w - 24.0 * unit,
            f64::from(height) - h - 24.0 * unit,
            w,
            h,
        ];
        let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
        for (_, p) in &map.dots {
            for k in 0..2 {
                (lo[k], hi[k]) = (lo[k].min(p[k]), hi[k].max(p[k]));
            }
        }
        // One scale on both axes, so distances read the same either way;
        // room left for the labels and the axis names.
        let scale = ((w - 150.0 * unit) / (hi[0] - lo[0]).max(1e-9))
            .min((h - 70.0 * unit) / (hi[1] - lo[1]).max(1e-9));
        let centre = [(lo[0] + hi[0]) / 2.0, (lo[1] + hi[1]) / 2.0];
        let name = |id: &str| {
            names
                .iter()
                .find(|(i, _)| *i == id)
                .map_or(id.to_string(), |(_, n)| n.to_string())
        };
        let dots = map.dots.iter().map(|(id, p)| (name(id), *p)).collect();
        let axes = [
            format!("x: {}", map.labels[0].join(", ")),
            format!("y: {}", map.labels[1].join(", ")),
        ];
        let mut overlay = Overlay {
            panel,
            scale,
            centre,
            letter,
            dots,
            labels: Vec::new(),
            axes,
        };
        overlay.labels = overlay.place();
        overlay
    }

    /// Every dot's label to its right, top dot first; one that would
    /// overlap a label already placed, a dot or the axis names, or leave
    /// the panel, steps a line down, then up, then further, set off to the
    /// right so its leader line shows (the owner, 2026-10-19: the beech,
    /// Laurelin and Telperion stand almost on one point). The dots stay
    /// where the projection puts them.
    fn place(&self) -> Vec<Label> {
        let s = f64::from(self.letter);
        let [px, py, pw, ph] = self.panel();
        let mut order: Vec<(&String, [f64; 2])> =
            self.dots.iter().map(|(n, p)| (n, self.pixel(*p))).collect();
        order.sort_by(|a, b| a.1[1].total_cmp(&b.1[1]).then(a.1[0].total_cmp(&b.1[0])));
        // The axis names' two rows, and every dot, are kept clear.
        let mut blocked: Vec<[f64; 4]> = self.axis_rows().to_vec();
        blocked.extend(
            order
                .iter()
                .map(|(_, d)| [d[0] - 3.0 * s, d[1] - 3.0 * s, 6.0 * s, 6.0 * s]),
        );
        let inside = |f: &[f64; 4]| {
            f[0] >= px && f[1] >= py && f[0] + f[2] <= px + pw && f[1] + f[3] <= py + ph
        };
        let mut placed: Vec<Label> = Vec::new();
        for (text, dot) in order {
            let (w, h) = (8.0 * s * text.chars().count() as f64, 8.0 * s);
            let step = h + 3.0 * s;
            let at = |k: i32| {
                let dy = f64::from((k + 1) / 2 * if k % 2 == 1 { 1 } else { -1 }) * step;
                let dx = if k == 0 { 5.0 } else { 18.0 } * s;
                [dot[0] + dx, dot[1] - 4.0 * s + dy, w, h]
            };
            let clear = |f: &[f64; 4]| {
                inside(f)
                    && blocked
                        .iter()
                        .chain(placed.iter().map(|l| &l.frame))
                        .all(|b| !overlap(*f, *b))
            };
            let frame = (0..64).map(at).find(|f| clear(f)).unwrap_or_else(|| at(0));
            placed.push(Label {
                text: text.clone(),
                frame,
                dot,
            });
        }
        placed
    }

    /// The panel's left, top, width and height, pixels.
    pub fn panel(&self) -> [f64; 4] {
        self.panel
    }

    /// The boxes the two axis names are written in.
    pub fn axis_rows(&self) -> [[f64; 4]; 2] {
        let s = f64::from(self.letter);
        let [x, y, w, h] = self.panel;
        [30.0, 16.0].map(|up| [x + 8.0 * s, y + h - up * s, w - 16.0 * s, 9.0 * s])
    }

    /// A map point in pixels, y down; the panel's centre a little left, so
    /// the labels to the right of the dots fit.
    fn pixel(&self, p: [f64; 2]) -> [f64; 2] {
        let [x, y, w, h] = self.panel;
        let mid = [
            x + w / 2.0 - 40.0 * f64::from(self.letter),
            y + h / 2.0 + 4.0 * f64::from(self.letter),
        ];
        [
            mid[0] + (p[0] - self.centre[0]) * self.scale,
            mid[1] - (p[1] - self.centre[1]) * self.scale,
        ]
    }

    /// The overlay over `c`: the trail through `trail` (the path's map
    /// points so far, the last one now), the age, and `line` faded in by
    /// `fade` (0 to 1).
    pub fn draw(&self, c: &mut Canvas, trail: &[[f64; 2]], age: f64, line: Option<(&str, f64)>) {
        let s = f64::from(self.letter);
        let [x, y, w, h] = self.panel;
        fill(c, [x, y, w, h], SHADE, 0.38);
        for pair in trail.windows(2) {
            segment(c, self.pixel(pair[0]), self.pixel(pair[1]), INK, 0.4);
        }
        for label in &self.labels {
            if label.moved(s) {
                let [x, y, _, h] = label.frame;
                segment(c, label.dot, [x - s, y + h / 2.0], INK, 0.55);
            }
            disc(c, label.dot, 2.5 * s, INK, 0.9);
            write(
                c,
                [label.frame[0], label.frame[1]],
                &label.text,
                self.letter,
                INK,
                0.85,
            );
        }
        if let Some(now) = trail.last() {
            disc(c, self.pixel(*now), 4.5 * s, NOW, 1.0);
        }
        for (row, axis) in self.axis_rows().iter().zip(&self.axes) {
            write(c, [row[0], row[1]], axis, self.letter, INK, 0.6);
        }
        write(
            c,
            [24.0 * s, 24.0 * s],
            &format!("age {}", age.round()),
            3 * self.letter,
            INK,
            0.95,
        );
        if let Some((text, fade)) = line {
            let big = 3 * self.letter;
            let wide = f64::from(8 * big) * text.chars().count() as f64;
            let at = [
                (f64::from(c.width) - wide) / 2.0,
                f64::from(c.height) * 0.12,
            ];
            write(c, at, text, big, INK, fade.clamp(0.0, 1.0));
        }
    }
}

fn blend(c: &mut Canvas, x: i64, y: i64, rgb: Rgb, alpha: f64) {
    if x < 0 || y < 0 || x >= i64::from(c.width) || y >= i64::from(c.height) {
        return;
    }
    let at = (y as usize * c.width as usize + x as usize) * 4;
    for k in 0..3 {
        let old = f64::from(c.rgba[at + k]);
        c.rgba[at + k] = (old + (f64::from(rgb[k]) - old) * alpha).round() as u8;
    }
}

fn fill(c: &mut Canvas, [x, y, w, h]: [f64; 4], rgb: Rgb, alpha: f64) {
    for py in y as i64..(y + h) as i64 {
        for px in x as i64..(x + w) as i64 {
            blend(c, px, py, rgb, alpha);
        }
    }
}

/// A disc with a soft pixel edge.
fn disc(c: &mut Canvas, [cx, cy]: [f64; 2], r: f64, rgb: Rgb, alpha: f64) {
    for py in (cy - r - 1.0) as i64..=(cy + r + 1.0) as i64 {
        for px in (cx - r - 1.0) as i64..=(cx + r + 1.0) as i64 {
            let d = (px as f64 + 0.5 - cx).hypot(py as f64 + 0.5 - cy);
            let cover = (r + 0.5 - d).clamp(0.0, 1.0);
            if cover > 0.0 {
                blend(c, px, py, rgb, alpha * cover);
            }
        }
    }
}

/// A line a pixel wide, stepped a pixel at a time.
fn segment(c: &mut Canvas, a: [f64; 2], b: [f64; 2], rgb: Rgb, alpha: f64) {
    let steps = (b[0] - a[0]).abs().max((b[1] - a[1]).abs()).ceil().max(1.0) as i64;
    for k in 0..=steps {
        let t = k as f64 / steps as f64;
        blend(
            c,
            (a[0] + (b[0] - a[0]) * t) as i64,
            (a[1] + (b[1] - a[1]) * t) as i64,
            rgb,
            alpha,
        );
    }
}

/// `text` from its top left `at`, each letter 8 by 8 pixels times `size`,
/// over a shadow a letter's pixel down and right.
fn write(c: &mut Canvas, at: [f64; 2], text: &str, size: u32, rgb: Rgb, alpha: f64) {
    for (shadow, colour, a) in [(1, SHADE, alpha * 0.6), (0, rgb, alpha)] {
        for (i, ch) in text.chars().enumerate() {
            let Some(glyph) = BASIC_FONTS.get(ch) else {
                continue;
            };
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..8 {
                    if bits & (1 << col) == 0 {
                        continue;
                    }
                    let x0 = at[0] as i64 + ((i * 8 + col) as i64 + shadow) * i64::from(size);
                    let y0 = at[1] as i64 + (row as i64 + shadow) * i64::from(size);
                    for dy in 0..i64::from(size) {
                        for dx in 0..i64::from(size) {
                            blend(c, x0 + dx, y0 + dy, colour, a);
                        }
                    }
                }
            }
        }
    }
}
