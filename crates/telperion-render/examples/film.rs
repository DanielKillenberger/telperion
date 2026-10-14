//! The release film (fn-272 R2): the declared path rendered frame by frame
//! through the one pipeline and the shipped renderer, a growth grown once
//! on the device and settled at each frame's age (fn-270), a walk built at
//! each frame's share as `headless --walk` builds one, the corner map drawn
//! flat over each still, and the stills assembled into an mp4 by ffmpeg.
//! Each segment's cost is printed as it ends; each frame's tree in
//! `frames.tsv` beside the stills.
//!
//!   cargo run --profile ci -p telperion-render --example film -- <path.json> <out dir>
//!   cargo run --profile ci -p telperion-render --example film -- <path.json> --oracle
//!
//! `--oracle` judges every walk frame by fn-251's oracle on the CPU and
//! draws nothing.
#[path = "film/camera.rs"]
mod camera;
#[path = "film/cost.rs"]
mod cost;
#[path = "film/map.rs"]
mod map;
#[path = "film/oracle.rs"]
mod oracle;
#[path = "film/overlay.rs"]
mod overlay;
#[path = "film/path.rs"]
mod path;

use cost::{Cost, Peaks};
use overlay::{Canvas, Overlay};
use path::{ease, Frame, Kind, Path, Sample};
use std::io::Write as _;
use telperion_core::{
    blend, mesh,
    presets::{by_identity, CATALOGUE},
    surface::Bounds,
    Family,
};
use telperion_render::{generation::Generator, render, write_png, Gpu, Level, Renderer, SceneRow, STILL_FORMAT};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [file, out] = &args[..] else {
        return Err("usage: film <path.json> <out dir> | film <path.json> --oracle".into());
    };
    let path = Path::read(std::path::Path::new(file))?;
    let frames = path.frames();
    if out == "--oracle" {
        let missed = oracle::check(&path, &frames)?;
        println!("{missed} walk frames miss the oracle");
        return Ok(());
    }
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let shipped: Vec<(&str, Family)> = CATALOGUE.iter().map(|e| Ok((e.1, by_identity(e.1)?))).collect::<Result<_, telperion_core::Error>>().map_err(|e| e.to_string())?;
    let map = map::Map::of(&shipped)?;
    println!("map: {:.1} % of the variance; x {:?}, y {:?}", 100.0 * map.share, map.labels[0], map.labels[1]);
    for (id, p) in &map.dots {
        println!("map: {id} at {:.3}, {:.3}", p[0], p[1]);
    }
    let names: Vec<(&str, &str)> = CATALOGUE.iter().map(|e| (e.1, e.2)).collect();
    let overlay = Overlay::new(&map, &names, path.size.0, path.size.1);
    let shown = shown(&path, &frames, &map)?;
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut film = Film { path: &path, frames: &frames, shown: &shown, overlay, out, renderer: Renderer::new(gpu, STILL_FORMAT), log: None };
    film.renderer.set_scene(SceneRow { sun_azimuth: path.sun.0, sun_elevation: path.sun.1, ..SceneRow::default() });
    let gen = Generator::new(&film.renderer).map_err(|e| e.to_string())?;
    let log = std::fs::File::create(format!("{out}/frames.tsv")).map_err(|e| e.to_string())?;
    film.log = Some(std::io::BufWriter::new(log));
    film.line("frame\tsegment\tat\tleaves\theight\twidth\tbuild_ms")?;
    let peaks = Peaks::start();
    for (k, segment) in path.segments.iter().enumerate() {
        peaks.reset();
        let cost = match &segment.kind {
            Kind::Grow { species, from, to, sample } => film.grow(&gen, k, species, (*from, *to), *sample)?,
            Kind::Walk { from, to, age } => film.walk(k, (from, to), *age)?,
        };
        let (rss, device) = peaks.take();
        println!(
            "cost: segment {k}: {} frames, {} trees, build {:.1} s, draw {:.1} s, peak resident {rss} MB, device {device} MB",
            cost.frames,
            cost.trees,
            cost.build_ms / 1e3,
            cost.draw_ms / 1e3
        );
    }
    drop(film.log.take());
    assemble(out, path.fps)
}

/// What the overlay shows at each frame: the map point, the age and the
/// closing line with its fade.
struct Shown {
    point: [f64; 2],
    age: f64,
    line: Option<(String, f64)>,
}

fn shown(path: &Path, frames: &[Frame], map: &map::Map) -> Result<Vec<Shown>, String> {
    let mut out: Vec<Shown> = Vec::with_capacity(frames.len());
    let mut held = 0usize;
    for f in frames {
        let segment = &path.segments[f.segment];
        held = if f.held { held + 1 } else { 0 };
        let line = segment.line.as_ref().filter(|_| f.held).map(|l| (l.clone(), held as f64 / (0.3 * f64::from(path.fps))));
        let (point, age) = match &segment.kind {
            Kind::Grow { species, from, to, sample } => (map.dot(species)?, age_at(*from, *to, f.along, *sample)),
            Kind::Walk { from, to, age } => {
                let (a, b) = (ends(from, *age, path.seed)?, ends(to, *age, path.seed)?);
                (map.project(&map::walked(&a, &b, ease(f.along))?)?, *age)
            }
        };
        out.push(Shown { point, age, line });
    }
    Ok(out)
}

/// A growth's age `along` its segment, as the frame samples it.
fn age_at(from: f64, to: f64, along: f64, sample: Sample) -> f64 {
    let age = (from + (to - from) * along).max(1.0);
    match sample {
        Sample::Continuous => age,
        Sample::Whole => age.round(),
    }
}

/// A shipped species at `age` and the path's seed.
fn ends(id: &str, age: f64, seed: u32) -> Result<Family, String> {
    let mut f = by_identity(id).map_err(|e| format!("{id}: {e}"))?;
    (f.age, f.skeleton.seed) = (age, seed);
    Ok(f)
}

struct Film<'a> {
    path: &'a Path,
    frames: &'a [Frame],
    shown: &'a [Shown],
    overlay: Overlay,
    out: &'a str,
    renderer: Renderer,
    log: Option<std::io::BufWriter<std::fs::File>>,
}

impl Film<'_> {
    fn line(&mut self, text: &str) -> Result<(), String> {
        let log = self.log.as_mut().ok_or("no frame log")?;
        writeln!(log, "{text}").map_err(|e| e.to_string())
    }

    /// The frames of segment `k`, by their index in the film.
    fn of(&self, k: usize) -> Vec<(usize, Frame)> {
        self.frames.iter().copied().enumerate().filter(|(_, f)| f.segment == k).collect()
    }

    /// The tree just put up, logged against every frame in `indices`.
    fn log(&mut self, (leaves, b): (usize, Bounds), at: f64, indices: &[usize], ms: f64) -> Result<(), String> {
        let size = b.max - b.min;
        for &i in indices {
            let row = format!("{i}\t{}\t{at:.4}\t{leaves}\t{:.3}\t{:.3}\t{ms:.0}", self.frames[i].segment, size.y, size.x.max(size.z));
            self.line(&row)?;
        }
        Ok(())
    }

    /// Frame `i` drawn from its pose, the overlay laid over it and written.
    fn draw(&mut self, i: usize) -> Result<f64, String> {
        let started = std::time::Instant::now();
        let (w, h) = self.path.size;
        let pose = camera::pose(&self.path.camera, i as f64 / f64::from(self.path.fps));
        let mut still = render(&mut self.renderer, &pose, w, h).map_err(|e| e.to_string())?;
        let trail: Vec<[f64; 2]> = self.shown[..=i].iter().map(|s| s.point).collect();
        let now = &self.shown[i];
        let mut canvas = Canvas { rgba: &mut still.rgba, width: w, height: h };
        self.overlay.draw(&mut canvas, &trail, now.age, now.line.as_ref().map(|(l, f)| (l.as_str(), *f)));
        write_png(std::path::Path::new(&format!("{}/frame-{i:05}.png", self.out)), &still).map_err(|e| e.to_string())?;
        Ok(started.elapsed().as_secs_f64() * 1e3)
    }

    /// A growth: one tree grown once, settled at each frame's age in rising
    /// order (a shrink's frames are drawn last to first), each age settled
    /// once and drawn for every frame that shows it.
    fn grow(&mut self, gen: &Generator, k: usize, species: &str, (from, to): (f64, f64), sample: Sample) -> Result<Cost, String> {
        let family = ends(species, from.max(to), self.path.seed)?;
        let mut cost = Cost::default();
        let started = std::time::Instant::now();
        let mut tree = pollster::block_on(gen.begin_growth(&family)).map_err(|e| e.to_string())?;
        cost.build_ms += started.elapsed().as_secs_f64() * 1e3;
        let mut order: Vec<(usize, f64)> = self.of(k).into_iter().map(|(i, f)| (i, age_at(from, to, f.along, sample))).collect();
        order.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
        let mut k0 = 0;
        while k0 < order.len() {
            let at = order[k0].1;
            let same: Vec<usize> = order[k0..].iter().take_while(|(_, a)| *a == at).map(|(i, _)| *i).collect();
            let started = std::time::Instant::now();
            pollster::block_on(gen.grow_to(&mut tree, at)).map_err(|e| format!("{species} to {at}: {e}"))?;
            let settled = pollster::block_on(gen.settle(&tree, at)).map_err(|e| format!("{species} at {at}: {e}"))?;
            let ms = started.elapsed().as_secs_f64() * 1e3;
            (cost.build_ms, cost.trees) = (cost.build_ms + ms, cost.trees + 1);
            let shape = (settled.count(), settled.bounds());
            self.renderer.submit_prepared(settled).map_err(|e| e.to_string())?;
            self.renderer.set_family(&family);
            self.log(shape, at, &same, ms)?;
            for &i in &same {
                cost.draw_ms += self.draw(i)?;
            }
            (k0, cost.frames) = (k0 + same.len(), cost.frames + same.len());
        }
        Ok(cost)
    }

    /// A walk, as `headless --walk` draws one: each frame's family built
    /// at its eased share by the direct build (`mesh::build`) and put up;
    /// a held frame draws the last tree again.
    fn walk(&mut self, k: usize, (from, to): (&str, &str), age: f64) -> Result<Cost, String> {
        let (a, b) = (ends(from, age, self.path.seed)?, ends(to, age, self.path.seed)?);
        let mut cost = Cost::default();
        let mut last = None;
        for (i, f) in self.of(k) {
            let t = ease(f.along);
            if last != Some(t) {
                let started = std::time::Instant::now();
                let family = blend::families(&a, &b, t).map_err(|e| format!("{from} -> {to} at {t}: {e}"))?;
                let tree = mesh::build(&family).map_err(|e| format!("{from} -> {to} at {t}: {e}"))?;
                let ms = started.elapsed().as_secs_f64() * 1e3;
                (cost.build_ms, cost.trees) = (cost.build_ms + ms, cost.trees + 1);
                self.renderer.submit_at(&tree, Level::Chosen).map_err(|e| e.to_string())?;
                self.renderer.set_family(&family);
                self.log((tree.foliage_instances(), tree.bounds), t, &[i], ms)?;
                last = Some(t);
            }
            cost.draw_ms += self.draw(i)?;
            cost.frames += 1;
        }
        Ok(cost)
    }
}

/// The stills as an H.264 mp4 at the path's rate.
fn assemble(out: &str, fps: u32) -> Result<(), String> {
    let status = std::process::Command::new("ffmpeg")
        .args(["-y", "-loglevel", "error", "-framerate", &fps.to_string(), "-i", &format!("{out}/frame-%05d.png")])
        .args(["-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "18", &format!("{out}/film.mp4")])
        .status()
        .map_err(|e| format!("ffmpeg: {e}"))?;
    status.success().then_some(()).ok_or(format!("ffmpeg: {status}"))?;
    println!("{out}/film.mp4");
    Ok(())
}
