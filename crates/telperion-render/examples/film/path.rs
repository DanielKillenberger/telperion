//! The film's declared path (fn-272 R1): a value file of segments, each a
//! growth of one species from one age to another (played backwards where
//! the second is the smaller) or a walk from one species to another at one
//! age, with the camera's keys; and the frames it lays out. Nothing here
//! touches a device.
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Grow { species: String, from: f64, to: f64 },
    Walk { from: String, to: String, age: f64 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub kind: Kind,
    pub seconds: f64,
    /// Seconds held still at the end.
    pub hold: f64,
    /// A line shown over the hold.
    pub line: Option<String>,
}

/// A camera key: at a time of the film, how far back the eye stands and
/// the height it looks at, in metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Key {
    pub at: f64,
    pub distance: f64,
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    /// Degrees about the vertical at the first frame, and turned a second.
    pub azimuth: f64,
    pub orbit: f64,
    /// Degrees above the horizontal the eye looks down from.
    pub elevation: f64,
    pub fov: f64,
    pub keys: Vec<Key>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub fps: u32,
    pub size: (u32, u32),
    pub seed: u32,
    /// The sun's azimuth and elevation, degrees.
    pub sun: (f64, f64),
    /// The share of each year of a growth over which the next whole year
    /// fades in, eased (0 to 1): 0 a hard step at the year, 1 a fade
    /// across it (owner, 2026-10-19: the treatment is the owner's look).
    pub fade: f64,
    pub camera: Camera,
    pub segments: Vec<Segment>,
}

/// One frame: its segment and how far along the segment's motion it is
/// (0 to 1; 1 through the hold), and whether it is held.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub segment: usize,
    pub along: f64,
    pub held: bool,
}

fn field<'v>(v: &'v Value, at: &str, name: &str) -> Result<&'v Value, String> {
    v.get(name).ok_or(format!("{at}: no \"{name}\""))
}

fn number(v: &Value, at: &str, name: &str) -> Result<f64, String> {
    let x = field(v, at, name)?.as_f64().filter(|x| x.is_finite());
    x.ok_or(format!("{at}: \"{name}\" wants a number"))
}

fn positive(v: &Value, at: &str, name: &str) -> Result<f64, String> {
    let x = number(v, at, name)?;
    (x > 0.0)
        .then_some(x)
        .ok_or(format!("{at}: \"{name}\" wants a number above zero"))
}

/// A tree's age: whole years, one or more, as the pipeline builds it.
fn years(v: &Value, at: &str, name: &str) -> Result<f64, String> {
    let x = positive(v, at, name)?;
    (x >= 1.0 && x.fract() == 0.0)
        .then_some(x)
        .ok_or(format!("{at}: \"{name}\" wants whole years, one or more"))
}

fn text(v: &Value, at: &str, name: &str) -> Result<String, String> {
    let s = field(v, at, name)?.as_str();
    s.map(str::to_string)
        .ok_or(format!("{at}: \"{name}\" wants a string"))
}

fn pair(v: &Value, at: &str, name: &str) -> Result<(f64, f64), String> {
    match field(v, at, name)?.as_array().map(Vec::as_slice) {
        Some([a, b]) => match (a.as_f64(), b.as_f64()) {
            (Some(a), Some(b)) => Ok((a, b)),
            _ => Err(format!("{at}: \"{name}\" wants two numbers")),
        },
        _ => Err(format!("{at}: \"{name}\" wants two numbers")),
    }
}

impl Path {
    pub fn read(file: &std::path::Path) -> Result<Path, String> {
        let raw = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
        let v: Value =
            serde_json::from_str(&raw).map_err(|e| format!("{}: {e}", file.display()))?;
        Path::parse(&v)
    }

    pub fn parse(v: &Value) -> Result<Path, String> {
        let (w, h) = pair(v, "path", "size")?;
        let whole = |x: f64| (x >= 1.0 && x.fract() == 0.0).then_some(x as u32);
        let size = (whole(w), whole(h));
        let (Some(w), Some(h)) = size else {
            return Err("path: \"size\" wants two whole pixels".into());
        };
        let fps = whole(positive(v, "path", "fps")?).ok_or("path: \"fps\" wants a whole number")?;
        let seed = number(v, "path", "seed")?;
        let seed = (seed >= 0.0 && seed.fract() == 0.0 && seed <= f64::from(u32::MAX))
            .then_some(seed as u32);
        let c = field(v, "path", "camera")?;
        let mut keys = Vec::new();
        for (k, key) in field(c, "camera", "keys")?
            .as_array()
            .ok_or("camera: \"keys\" wants a list")?
            .iter()
            .enumerate()
        {
            let at = format!("camera key {k}");
            keys.push(Key {
                at: number(key, &at, "at")?,
                distance: positive(key, &at, "distance")?,
                height: number(key, &at, "height")?,
            });
        }
        if keys.is_empty() || keys.windows(2).any(|w| w[1].at <= w[0].at) {
            return Err("camera: \"keys\" wants one key or more, in rising time".into());
        }
        let camera = Camera {
            azimuth: number(c, "camera", "azimuth")?,
            orbit: number(c, "camera", "orbit")?,
            elevation: number(c, "camera", "elevation")?,
            fov: positive(c, "camera", "fov")?,
            keys,
        };
        let list = field(v, "path", "segments")?
            .as_array()
            .ok_or("path: \"segments\" wants a list")?;
        let segments = list
            .iter()
            .enumerate()
            .map(|(k, s)| segment(s, &format!("segment {k}")))
            .collect::<Result<Vec<_>, _>>()?;
        if segments.is_empty() {
            return Err("path: \"segments\" wants one segment or more".into());
        }
        let seed = seed.ok_or("path: \"seed\" wants a whole number")?;
        let fade = Some(number(v, "path", "fade")?)
            .filter(|f| (0.0..=1.0).contains(f))
            .ok_or("path: \"fade\" wants a share from 0 to 1")?;
        Ok(Path {
            fps,
            size: (w, h),
            seed,
            sun: pair(v, "path", "sun")?,
            fade,
            camera,
            segments,
        })
    }

    /// Every frame in order. A segment's motion takes its seconds and
    /// begins where the last one ended, so the first frame of each but the
    /// first is a step past its start; the hold follows the motion.
    pub fn frames(&self) -> Vec<Frame> {
        let fps = f64::from(self.fps);
        let mut out = Vec::new();
        for (k, s) in self.segments.iter().enumerate() {
            let n = ((s.seconds * fps).round() as usize).max(1);
            for i in 0..n {
                let along = if k == 0 {
                    i as f64 / (n.max(2) - 1) as f64
                } else {
                    (i + 1) as f64 / n as f64
                };
                out.push(Frame {
                    segment: k,
                    along,
                    held: false,
                });
            }
            for _ in 0..(s.hold * fps).round() as usize {
                out.push(Frame {
                    segment: k,
                    along: 1.0,
                    held: true,
                });
            }
        }
        out
    }
}

fn segment(v: &Value, at: &str) -> Result<Segment, String> {
    let kind = match (v.get("grow"), v.get("walk")) {
        (Some(_), None) => {
            let (from, to) = (years(v, at, "from")?, years(v, at, "to")?);
            Kind::Grow {
                species: text(v, at, "grow")?,
                from,
                to,
            }
        }
        (None, Some(_)) => Kind::Walk {
            from: text(v, at, "walk")?,
            to: text(v, at, "to")?,
            age: years(v, at, "age")?,
        },
        _ => return Err(format!("{at}: one of \"grow\" or \"walk\"")),
    };
    let hold = match v.get("hold") {
        Some(_) => Some(number(v, at, "hold")?)
            .filter(|h| *h >= 0.0)
            .ok_or(format!("{at}: \"hold\" is never negative"))?,
        None => 0.0,
    };
    let line = match v.get("line") {
        Some(_) => Some(text(v, at, "line")?),
        None => None,
    };
    Ok(Segment {
        kind,
        seconds: positive(v, at, "seconds")?,
        hold,
        line,
    })
}

/// Smoothstep: a walk that leaves at rest and arrives at rest, as
/// `headless --walk` eases its own.
pub fn ease(at: f64) -> f64 {
    let at = at.clamp(0.0, 1.0);
    at * at * (3.0 - 2.0 * at)
}

/// The later year's share of a frame `into` its year (0 to 1) when it
/// fades in over the last `fade` of the year, eased: none before, a hard
/// step at the whole year where `fade` is 0, the whole year where it is 1.
pub fn faded(into: f64, fade: f64) -> f64 {
    if fade <= 0.0 {
        return 0.0;
    }
    ease((into - (1.0 - fade)) / fade)
}
