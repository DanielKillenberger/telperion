//! R5's still strips (fn-192): representative walks of the walk tests'
//! tree at a fixed seed, nine frames each, seen from the side (x across, z
//! up) at one scale per strip. Each internode is drawn as wide as its scale.
//!
//!   cargo run --release -p telperion-space --example strips -- <out dir>
//!   for f in <out dir>/*.svg; do rsvg-convert -b white -w 2400 "$f" -o "${f%.svg}.png"; done
#[path = "../tests/walk/mod.rs"]
mod walk;
use std::fmt::Write;
use telperion_space::Structure;

const FRAMES: u32 = 9;
const COLOURS: [&str; 3] = ["#4a3222", "#2f6b2a", "#8a9a2b"];
/// Each walk, whether it is drawn from its high end, and its seed: the
/// trunk's survival is drawn at the seed whose trunk dies in the range.
const WALKS: [(&str, bool, u64); 8] = [
    ("states[0].readiness", true, 1),
    ("states[0].zones[1].lateral[1]", false, 1),
    ("states[1].abortion", false, 1),
    ("states[1].straightening", false, 1),
    ("states[0].rhythm", true, 1),
    ("states[0].zones[0].nodes.mean", false, 1),
    ("states[2].viability", false, 1),
    ("states[0].viability", false, 2),
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = args.first().expect("usage: strips <out dir>");
    std::fs::create_dir_all(out).unwrap();
    let settings = walk::settings();
    for (name, downward, seed) in WALKS {
        let setting = settings.iter().find(|s| s.name == name).unwrap();
        let frames: Vec<(f64, Structure)> = (0..FRAMES)
            .map(|k| {
                let k = if downward { FRAMES - 1 - k } else { k };
                let x = setting.step(k * walk::STEPS / (FRAMES - 1));
                let value = setting.value(x);
                (value, walk::tree(&setting.at(value), seed).unwrap())
            })
            .collect();
        let file = name.replace(['[', ']'], "").replace('.', "-");
        std::fs::write(format!("{out}/{file}.svg"), strip(name, seed, &frames)).unwrap();
        println!("{out}/{file}.svg");
    }
}

/// The frames side by side, on one scale, each labelled with its value.
fn strip(name: &str, seed: u64, frames: &[(f64, Structure)]) -> String {
    let (mut low, mut high, mut top) = (0.0f64, 0.0f64, 1.0f64);
    for (_, tree) in frames {
        for p in tree.axes.iter().flat_map(|a| &a.phytomers) {
            low = low.min(p.tip.x);
            high = high.max(p.tip.x);
            top = top.max(p.tip.z);
        }
    }
    let width = (high - low) * 1.1 + 1.0;
    let mut body = String::new();
    for (k, (value, tree)) in frames.iter().enumerate() {
        let shift = k as f64 * width - low + 0.5;
        draw(&mut body, tree, shift, top);
        let label = format!("{value:.3}");
        let x = shift + low;
        writeln!(
            body,
            r#"<text x="{x:.2}" y="{:.2}" font-size="{:.2}">{label}</text>"#,
            top + 1.5,
            top / 25.0
        )
        .unwrap();
    }
    let (w, h) = (width * frames.len() as f64, top + 2.5);
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 {:.2} {w:.2} {h:.2}" stroke-linecap="round"><rect x="0" y="-1" width="{w:.2}" height="{h:.2}" fill="white"/><text x="0.3" y="0" font-size="{:.2}">{name}, seed {seed}</text>{body}</svg>"#,
        -1.0,
        top / 20.0,
    )
}

/// One tree, flipped so z is up and the ground at `top`.
fn draw(svg: &mut String, tree: &Structure, shift: f64, top: f64) {
    for axis in &tree.axes {
        let mut from = axis.base;
        for p in &axis.phytomers {
            let width = 0.16 * p.scale / (1.0 + axis.pa as f64);
            if width > 1e-4 {
                let colour = COLOURS[axis.pa.min(COLOURS.len() - 1)];
                writeln!(
                    svg,
                    r#"<line x1="{:.3}" y1="{:.3}" x2="{:.3}" y2="{:.3}" stroke="{colour}" stroke-width="{width:.4}"/>"#,
                    from.x + shift,
                    top - from.z,
                    p.tip.x + shift,
                    top - p.tip.z
                )
                .unwrap();
            }
            from = p.tip;
        }
    }
}
