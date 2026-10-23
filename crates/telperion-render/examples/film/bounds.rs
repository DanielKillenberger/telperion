//! Every frame's crowns, from the whole film's real trees (fn-310, host
//! ruling on the film's side): a frame's placement of the person reads the
//! crowns a minute of frames round it, in segments a `--frames` run does
//! not draw, so the film builds every tree once for its bounds first. The
//! pass is kept in a file the next run reads instead of building again; a
//! drawn tree that differs from its line refuses the file as stale.
use super::path::{ease, Frame, Kind, Path};
use super::placing;
use telperion_core::{blend, math::Vec3, mesh, presets::by_identity, surface::Bounds, Family};
use telperion_render::generation::Generator;

/// A frame's crowns: its earlier year, its later year (the earlier again
/// where none fades in) and the later's share.
pub type Crowns = (Bounds, Bounds, f64);

/// A shipped species at `age` and the path's seed.
pub fn ends(id: &str, age: f64, seed: u32) -> Result<Family, String> {
    let mut f = by_identity(id).map_err(|e| format!("{id}: {e}"))?;
    (f.age, f.skeleton.seed) = (age, seed);
    Ok(f)
}

/// Every frame's crowns: read from `cache` where it holds the film's, else
/// built from every tree of the film and written there.
pub fn of_film(
    gen: &Generator,
    path: &Path,
    frames: &[Frame],
    cache: Option<&std::path::Path>,
) -> Result<Vec<Crowns>, String> {
    if let Some(file) = cache.filter(|f| f.exists()) {
        println!("bounds: read {}", file.display());
        return read(file, path, frames.len());
    }
    let mut crowns = Vec::with_capacity(frames.len());
    let indexed: Vec<(usize, Frame)> = frames.iter().copied().enumerate().collect();
    for (k, segment) in path.segments.iter().enumerate() {
        let of: Vec<(usize, Frame)> = indexed
            .iter()
            .copied()
            .filter(|f| f.1.segment == k)
            .collect();
        let started = std::time::Instant::now();
        match &segment.kind {
            Kind::Grow { species, from, to } => {
                crowns.extend(grown(gen, path, &of, species, (*from, *to))?)
            }
            Kind::Walk { from, to, age } => crowns.extend(walked(path, &of, (from, to), *age)?),
        }
        let seconds = started.elapsed().as_secs_f64();
        println!("bounds: segment {k}, {} frames, {seconds:.1} s", of.len());
    }
    if let Some(file) = cache {
        write(file, path, &crowns)?;
    }
    Ok(crowns)
}

/// A growth's frames: every whole year it shows grown and settled once.
fn grown(
    gen: &Generator,
    path: &Path,
    of: &[(usize, Frame)],
    species: &str,
    (from, to): (f64, f64),
) -> Result<Vec<Crowns>, String> {
    let family = ends(species, from.max(to), path.seed)?;
    let years = placing::years_of(of, (from, to), path.fade);
    let mut tree = pollster::block_on(gen.begin_growth(&family)).map_err(|e| e.to_string())?;
    let mut settled: Vec<(f64, Bounds)> = Vec::new();
    for at in placing::built(&years) {
        pollster::block_on(gen.grow_to(&mut tree, at)).map_err(|e| format!("{species}: {e}"))?;
        let prepared = pollster::block_on(gen.settle(&tree, at)).map_err(|e| e.to_string())?;
        settled.push((at, prepared.bounds()));
    }
    let of_year = |y: f64| settled.iter().find(|s| s.0 == y).map(|s| s.1);
    years
        .iter()
        .map(|&(i, lo, hi, share)| {
            let earlier = of_year(lo).ok_or(format!("frame {i}: year {lo} not grown"))?;
            let later = if share > 0.0 {
                of_year(hi)
            } else {
                Some(earlier)
            };
            Ok((
                earlier,
                later.ok_or(format!("frame {i}: year {hi}"))?,
                share,
            ))
        })
        .collect()
}

/// A walk's frames: each frame's family built at its eased share, a held
/// frame's tree reused.
fn walked(
    path: &Path,
    of: &[(usize, Frame)],
    (from, to): (&str, &str),
    age: f64,
) -> Result<Vec<Crowns>, String> {
    let (a, b) = (ends(from, age, path.seed)?, ends(to, age, path.seed)?);
    let mut last: Option<(f64, Bounds)> = None;
    let mut crowns = Vec::with_capacity(of.len());
    for (_, f) in of {
        let t = ease(f.along);
        let bounds = match last {
            Some((at, bounds)) if at == t => bounds,
            _ => {
                let family =
                    blend::families(&a, &b, t).map_err(|e| format!("{from} -> {to}: {e}"))?;
                mesh::build(&family)
                    .map_err(|e| format!("{from} -> {to}: {e}"))?
                    .bounds
            }
        };
        last = Some((t, bounds));
        crowns.push((bounds, bounds, 0.0));
    }
    Ok(crowns)
}

/// The file's first two lines: the path it was built from, as a 64-bit
/// FNV-1a digest of every field the path declares, and the columns. A
/// path whose years, fade, seed or segments changed reads as another file.
fn header(path: &Path) -> String {
    let digest = format!("{path:?}")
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325u64, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
        });
    format!("path {digest:016x}\nframe\tshare\tearlier (min xyz, max xyz)\tlater\n")
}

/// The crowns kept in `file`, one line a frame; an error where it was
/// built from another path or holds another number of frames.
pub fn read(file: &std::path::Path, path: &Path, frames: usize) -> Result<Vec<Crowns>, String> {
    let text = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
    let expected = header(path);
    if !text.starts_with(&expected) {
        let found: Vec<&str> = text.lines().take(1).collect();
        return Err(format!(
            "{}: built from another path ({found:?}, not {:?}); delete it to build again",
            file.display(),
            expected.lines().next().unwrap_or_default()
        ));
    }
    let crowns = text
        .lines()
        .skip(2)
        .map(|line| {
            let v: Vec<f64> = line
                .split('\t')
                .map(|c| c.parse().map_err(|e| format!("{e}: {line}")))
                .collect::<Result<_, String>>()?;
            if v.len() != 14 {
                return Err(format!("{}: {line}", file.display()));
            }
            let corner = |k: usize| Vec3::new(v[k], v[k + 1], v[k + 2]);
            let bounds = |k: usize| Bounds {
                min: corner(k),
                max: corner(k + 3),
            };
            Ok((bounds(2), bounds(8), v[1]))
        })
        .collect::<Result<Vec<_>, String>>()?;
    match crowns.len() == frames {
        true => Ok(crowns),
        false => Err(format!(
            "{}: {} frames, the film has {frames}",
            file.display(),
            crowns.len()
        )),
    }
}

/// The crowns written to `file` whole: to a sibling first, then moved in.
pub fn write(file: &std::path::Path, path: &Path, crowns: &[Crowns]) -> Result<(), String> {
    let corners = |b: &Bounds| {
        [b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z]
            .map(|v| format!("{v:?}"))
            .join("\t")
    };
    let mut text = header(path);
    for (i, (earlier, later, share)) in crowns.iter().enumerate() {
        text += &format!("{i}\t{share:?}\t{}\t{}\n", corners(earlier), corners(later));
    }
    let temporary = file.with_extension("tmp");
    std::fs::write(&temporary, text).map_err(|e| e.to_string())?;
    std::fs::rename(&temporary, file).map_err(|e| e.to_string())
}
