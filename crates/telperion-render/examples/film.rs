//! The release film (fn-272 R2): the declared path rendered frame by frame
//! through the one pipeline and the shipped renderer, a growth grown once
//! on the device and settled at whole ages, the next year faded in over
//! the path's `fade` share of each (fn-270; host, 2026-10-14; owner,
//! 2026-10-19), a walk built at each frame's share by the direct build, as
//! `headless --walk` builds one (the device refuses some walk frames:
//! FRICTION.md, 2026-10-19),
//! the corner map drawn flat over each still, and the stills assembled
//! into an mp4 by ffmpeg.
//! Each segment's cost is printed as it ends; each frame's tree in
//! `frames.tsv` beside the stills.
//!
//!   cargo run --profile ci -p telperion-render --example film -- <path.json> <out dir>
//!   cargo run --profile ci -p telperion-render --example film -- <path.json> <out dir> --frames 105,255,700-719
//!   cargo run --profile ci -p telperion-render --example film -- <path.json> --oracle
//!
//! `--frames` draws only the frames named, through the same path, builds
//! only the segments they fall in and assembles no video (fn-310 task 6).
//! `--oracle` judges every walk frame by fn-251's oracle on the CPU and
//! draws nothing.
#[path = "film/bounds.rs"]
mod bounds;
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
#[path = "film/placing.rs"]
mod placing;
#[path = "film/standing.rs"]
mod standing;

use cost::{Cost, Peaks};
use overlay::{Canvas, Overlay};
use path::{ease, selection, Frame, Kind, Path};
use std::io::Write as _;
use telperion_core::{
    blend, mesh,
    presets::{by_identity, CATALOGUE},
    surface::Bounds,
    Family,
};
use telperion_render::{
    generation::{Generator, Prepared},
    render, write_png, Gpu, Level, Placement, Renderer, SceneRow, Still, STILL_FORMAT,
};

/// A settled year to be drawn: its age, the tree, its leaves and bounds,
/// and its build time.
type Settled = (f64, Prepared, (usize, Bounds), f64);

/// A growth's frames, and the earlier year's picture of each frame that
/// waits for its later year.
struct Year<'a> {
    frames: &'a [placing::Year],
    earlier: std::collections::HashMap<usize, Vec<u8>>,
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: film <path.json> <out dir> [--frames <list>] [--bounds <file>] \
                 | film <path.json> --oracle";
    let (file, out, flags) = match &args[..] {
        [file, out, flags @ ..] if flags.len() % 2 == 0 => (file, out, flags),
        _ => return Err(usage.into()),
    };
    let flag = |name: &str| flags.chunks(2).find(|p| p[0] == name).map(|p| &p[1]);
    if flags
        .chunks(2)
        .any(|p| p[0] != "--frames" && p[0] != "--bounds")
    {
        return Err(usage.into());
    }
    let (chosen, cache) = (flag("--frames"), flag("--bounds"));
    let path = Path::read(std::path::Path::new(file))?;
    let frames = path.frames();
    let chosen = match chosen {
        Some(list) => Some(selection(list, frames.len())?),
        None => None,
    };
    if out == "--oracle" {
        let missed = oracle::check(&path, &frames)?;
        println!("{missed} walk frames miss the oracle");
        return Ok(());
    }
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let shipped: Vec<(&str, Family)> = CATALOGUE
        .iter()
        .map(|e| Ok((e.1, by_identity(e.1)?)))
        .collect::<Result<_, telperion_core::Error>>()
        .map_err(|e| e.to_string())?;
    let map = map::Map::of(&shipped)?;
    println!(
        "map: {:.1} % of the variance; x {:?}, y {:?}",
        100.0 * map.share,
        map.labels[0],
        map.labels[1]
    );
    for (id, p) in &map.dots {
        println!("map: {id} at {:.3}, {:.3}", p[0], p[1]);
    }
    let names: Vec<(&str, &str)> = CATALOGUE.iter().map(|e| (e.1, e.2)).collect();
    let overlay = Overlay::new(&map, &names, path.size.0, path.size.1);
    let shown = shown(&path, &frames, &map)?;
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut film = Film {
        path: &path,
        frames: &frames,
        shown: &shown,
        overlay,
        out,
        renderer: Renderer::new(gpu, STILL_FORMAT),
        log: None,
        chosen: chosen.as_ref(),
        crowns: &[],
        placements: &[],
    };
    film.renderer.set_scene(SceneRow {
        sun_azimuth: path.sun.0,
        sun_elevation: path.sun.1,
        ..SceneRow::default()
    });
    let gen = Generator::new(&film.renderer).map_err(|e| e.to_string())?;
    let crowns = bounds::of_film(&gen, &path, &frames, cache.map(std::path::Path::new))?;
    let standing = standing::of_film(&path, &crowns)?;
    film.crowns = &crowns;
    film.placements = &standing.placements;
    let log = std::fs::File::create(format!("{out}/frames.tsv")).map_err(|e| e.to_string())?;
    film.log = Some(std::io::BufWriter::new(log));
    film.line("frame\tsegment\tat\tleaves\theight\twidth\tbuild_ms")?;
    let peaks = Peaks::start();
    for (k, segment) in path.segments.iter().enumerate() {
        if film.of(k).is_empty() {
            continue;
        }
        peaks.reset();
        let cost = match &segment.kind {
            Kind::Grow { species, from, to } => film.grow(&gen, k, species, (*from, *to))?,
            Kind::Walk { from, to, age } => film.walk(k, (from, to), *age)?,
        };
        let (rss, device) = peaks.take();
        println!(
            "cost: segment {k}: {} frames, {} trees, build {:.1} s, draw {:.1} s (render {:.2} s, {} renders, {} sun maps; write {:.2} s), peak resident {rss} MB, device {device} MB",
            cost.frames,
            cost.trees,
            cost.build_ms / 1e3,
            cost.draw_ms / 1e3,
            cost.render_ms / 1e3,
            cost.renders,
            cost.sun_maps,
            (cost.draw_ms - cost.render_ms) / 1e3
        );
    }
    drop(film.log.take());
    match chosen {
        Some(_) => Ok(()),
        None => assemble(out, path.fps, frames.len()),
    }
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
        let line = segment
            .line
            .as_ref()
            .filter(|_| f.held)
            .map(|l| (l.clone(), held as f64 / (0.3 * f64::from(path.fps))));
        let (point, age) = match &segment.kind {
            Kind::Grow { species, from, to } => {
                (map.dot(species)?, placing::age_at(*from, *to, f.along))
            }
            Kind::Walk { from, to, age } => (map.along(from, to, ease(f.along))?, *age),
        };
        out.push(Shown { point, age, line });
    }
    Ok(out)
}

/// `a` faded into `b` by `share` (0 to 1), pixel by pixel.
fn crossfade(a: &[u8], b: &[u8], share: f64) -> Vec<u8> {
    a.iter()
        .zip(b)
        .map(|(&x, &y)| (f64::from(x) + (f64::from(y) - f64::from(x)) * share).round() as u8)
        .collect()
}

struct Film<'a> {
    path: &'a Path,
    frames: &'a [Frame],
    shown: &'a [Shown],
    overlay: Overlay,
    out: &'a str,
    renderer: Renderer,
    log: Option<std::io::BufWriter<std::fs::File>>,
    /// The frames to draw, where `--frames` names them; all where it does not.
    chosen: Option<&'a std::collections::BTreeSet<usize>>,
    /// Every frame's crowns from the bounds pass, which each drawn tree must
    /// match, and where the person stands in it.
    crowns: &'a [bounds::Crowns],
    placements: &'a [Placement],
}

impl Film<'_> {
    fn line(&mut self, text: &str) -> Result<(), String> {
        let log = self.log.as_mut().ok_or("no frame log")?;
        writeln!(log, "{text}").map_err(|e| e.to_string())
    }

    /// The frames of segment `k` to draw, by their index in the film.
    fn of(&self, k: usize) -> Vec<(usize, Frame)> {
        self.frames
            .iter()
            .copied()
            .enumerate()
            .filter(|(i, f)| f.segment == k && self.chosen.is_none_or(|c| c.contains(i)))
            .collect()
    }

    /// The tree just put up, logged against every frame in `indices`.
    fn log(
        &mut self,
        (leaves, b): (usize, Bounds),
        at: f64,
        indices: &[usize],
        ms: f64,
    ) -> Result<(), String> {
        let size = b.max - b.min;
        for &i in indices {
            let row = format!(
                "{i}\t{}\t{at:.4}\t{leaves}\t{:.3}\t{:.3}\t{ms:.0}",
                self.frames[i].segment,
                size.y,
                size.x.max(size.z)
            );
            self.line(&row)?;
        }
        Ok(())
    }

    /// Frame `i`'s tree, `which` of its crowns, refused where the bounds
    /// pass built another: a stale bounds file.
    fn matches(&self, i: usize, which: usize, bounds: Bounds) -> Result<(), String> {
        let (earlier, later, _) = self.crowns[i];
        match [earlier, later][which] == bounds {
            true => Ok(()),
            false => Err(format!(
                "frame {i}: the bounds pass built another tree; stale file?"
            )),
        }
    }

    /// Frame `i` drawn from its pose with the tree put up now, no overlay,
    /// its render and read back counted in `cost`.
    fn still(&mut self, i: usize, cost: &mut Cost) -> Result<Vec<u8>, String> {
        let (w, h) = self.path.size;
        let pose = camera::pose(&self.path.camera, i as f64 / f64::from(self.path.fps));
        let started = std::time::Instant::now();
        let rgba = render(&mut self.renderer, &pose, w, h)
            .map_err(|e| e.to_string())?
            .rgba;
        cost.render_ms += started.elapsed().as_secs_f64() * 1e3;
        cost.renders += 1;
        cost.sun_maps += usize::from(!self.renderer.shadow_cached());
        Ok(rgba)
    }

    /// Frame `i`'s picture with the overlay laid over it, written.
    fn finish(&self, i: usize, mut rgba: Vec<u8>) -> Result<(), String> {
        let (w, h) = self.path.size;
        let trail: Vec<[f64; 2]> = self.shown[..=i].iter().map(|s| s.point).collect();
        let now = &self.shown[i];
        let mut canvas = Canvas {
            rgba: &mut rgba,
            width: w,
            height: h,
        };
        self.overlay.draw(
            &mut canvas,
            &trail,
            now.age,
            now.line.as_ref().map(|(l, f)| (l.as_str(), *f)),
        );
        let still = Still {
            width: w,
            height: h,
            rgba,
            stats: Default::default(),
        };
        write_png(
            std::path::Path::new(&format!("{}/frame-{i:05}.png", self.out)),
            &still,
        )
        .map_err(|e| e.to_string())
    }

    /// Frame `i` drawn and written.
    fn draw(&mut self, i: usize, cost: &mut Cost) -> Result<(), String> {
        let started = std::time::Instant::now();
        let rgba = self.still(i, cost)?;
        self.finish(i, rgba)?;
        cost.draw_ms += started.elapsed().as_secs_f64() * 1e3;
        Ok(())
    }

    /// A growth: one tree grown once and settled at whole ages only, where
    /// it stands at full leaf (host, 2026-10-14). A frame between two whole
    /// ages shows the earlier year, the later faded over it by `faded`
    /// where the path's fade has begun; ages rise, so a shrink is drawn
    /// last frame first.
    fn grow(
        &mut self,
        gen: &Generator,
        k: usize,
        species: &str,
        (from, to): (f64, f64),
    ) -> Result<Cost, String> {
        let family = bounds::ends(species, from.max(to), self.path.seed)?;
        let mut cost = Cost::default();
        let started = std::time::Instant::now();
        let mut tree = pollster::block_on(gen.begin_growth(&family)).map_err(|e| e.to_string())?;
        cost.build_ms += started.elapsed().as_secs_f64() * 1e3;
        let frames = placing::years_of(&self.of(k), (from, to), self.path.fade);
        let mut year = Year {
            frames: &frames,
            earlier: std::collections::HashMap::new(),
        };
        for at in placing::built(&frames) {
            let started = std::time::Instant::now();
            pollster::block_on(gen.grow_to(&mut tree, at))
                .map_err(|e| format!("{species} to {at}: {e}"))?;
            let settled = pollster::block_on(gen.settle(&tree, at))
                .map_err(|e| format!("{species} at {at}: {e}"))?;
            let ms = started.elapsed().as_secs_f64() * 1e3;
            (cost.build_ms, cost.trees) = (cost.build_ms + ms, cost.trees + 1);
            let shape = (settled.count(), settled.bounds());
            self.show(&family, (at, settled, shape, ms), &mut year, &mut cost)?;
        }
        cost.frames = frames.len();
        Ok(cost)
    }

    /// One settled year put up and every frame that shows it drawn, the
    /// person where the frame stands it; a frame that waits for its later
    /// year keeps this one's picture.
    fn show(
        &mut self,
        family: &Family,
        (at, settled, shape, ms): Settled,
        year: &mut Year,
        cost: &mut Cost,
    ) -> Result<(), String> {
        self.renderer
            .submit_prepared(settled)
            .map_err(|e| e.to_string())?;
        self.renderer.set_family(family);
        let showing = placing::showing(year.frames, at);
        let indices: Vec<usize> = showing.iter().map(|(i, _)| *i).collect();
        self.log(shape, at, &indices, ms)?;
        for (i, share) in showing {
            let started = std::time::Instant::now();
            let frame = year.frames.iter().find(|f| f.0 == i).ok_or("no frame")?;
            self.matches(i, usize::from(frame.1 != at), shape.1)?;
            self.renderer.place_figure(self.placements[i]);
            let rgba = self.still(i, cost)?;
            match (share == 0.0, year.earlier.remove(&i)) {
                (true, _) => self.finish(i, rgba)?,
                (false, None) => drop(year.earlier.insert(i, rgba)),
                (false, Some(before)) => self.finish(i, crossfade(&before, &rgba, share))?,
            }
            cost.draw_ms += started.elapsed().as_secs_f64() * 1e3;
        }
        Ok(())
    }

    /// A walk, as `headless --walk` draws one: each frame's family built at
    /// its eased share by the direct build (`mesh::build`) and put up; a
    /// held frame draws the last tree again. Not on the device: it refuses
    /// beech to spruce at 40, t = 0.216 (FRICTION.md, 2026-10-19).
    fn walk(&mut self, k: usize, (from, to): (&str, &str), age: f64) -> Result<Cost, String> {
        let (a, b) = (
            bounds::ends(from, age, self.path.seed)?,
            bounds::ends(to, age, self.path.seed)?,
        );
        let mut cost = Cost::default();
        // The tree put up last, its share, shape and build time: a held
        // frame draws it again and is logged against it.
        let mut last: Option<(f64, (usize, Bounds), f64)> = None;
        for (i, f) in self.of(k) {
            let t = ease(f.along);
            if last.map(|l| l.0) != Some(t) {
                let started = std::time::Instant::now();
                let family = blend::families(&a, &b, t)
                    .map_err(|e| format!("{from} -> {to} at {t}: {e}"))?;
                let tree =
                    mesh::build(&family).map_err(|e| format!("{from} -> {to} at {t}: {e}"))?;
                let ms = started.elapsed().as_secs_f64() * 1e3;
                (cost.build_ms, cost.trees) = (cost.build_ms + ms, cost.trees + 1);
                let shape = (tree.foliage_instances(), tree.bounds);
                self.renderer
                    .submit_at(&tree, Level::Chosen)
                    .map_err(|e| e.to_string())?;
                self.renderer.set_family(&family);
                last = Some((t, shape, ms));
            }
            if let Some((t, shape, ms)) = last {
                self.log(shape, t, &[i], ms)?;
                self.matches(i, 0, shape.1)?;
                self.renderer.place_figure(self.placements[i]);
            }
            self.draw(i, &mut cost)?;
            cost.frames += 1;
        }
        Ok(cost)
    }
}

/// The path's `frames` stills as an H.264 mp4 at its rate; stills a longer
/// cut left in the directory past them are not read.
fn assemble(out: &str, fps: u32, frames: usize) -> Result<(), String> {
    let status = std::process::Command::new("ffmpeg")
        .args([
            "-y",
            "-loglevel",
            "error",
            "-framerate",
            &fps.to_string(),
            "-i",
            &format!("{out}/frame-%05d.png"),
        ])
        .args([
            "-frames:v",
            &frames.to_string(),
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            "-crf",
            "18",
            &format!("{out}/film.mp4"),
        ])
        .status()
        .map_err(|e| format!("ffmpeg: {e}"))?;
    status
        .success()
        .then_some(())
        .ok_or(format!("ffmpeg: {status}"))?;
    println!("{out}/film.mp4");
    Ok(())
}
