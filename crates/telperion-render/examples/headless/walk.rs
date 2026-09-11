//! What the command line says, and the plan it describes. The arguments the
//! headless target takes, and the schedule a sequence follows frame by frame:
//! how far the far family has come, and how far round the camera has drifted.
//! Nothing here touches a device, so a plan can be read without one.
use std::path::PathBuf;

use telperion_render::{SceneRow, View};

/// The rate a frame sequence is written for, and the rate the encoder is asked
/// for. A frame is a point on the walk, not a moment of a simulation.
pub const FPS: u32 = 24;

pub const USAGE: &str = "usage: headless --preset <id> --seed <n> --out <png> [--size WxH] \
                         [--view whole|bare|leaf] [--level <n>] [--timing <json>] [--orbit] \
                         [--scene <json>] [--to <preset>] [--frames <n>] \
                         [--walk <seconds>] [--hold <seconds>] [--sweep <degrees>]";

#[derive(Debug)]
pub struct Arguments {
    pub preset: String,
    pub seed: u32,
    pub out: PathBuf,
    pub size: (u32, u32),
    pub view: View,
    /// The level every leaf the frame shows is held at, judged against the
    /// element once there is an element to judge it against.
    pub level: Option<u32>,
    pub timing: Option<PathBuf>,
    /// The sun, sky and ground the stills are drawn under. Stated as the
    /// row's own JSON, read against the default: a flag that names two fields
    /// states two and takes the default for the rest. No flag is the default
    /// sky, so a run that says nothing about the sun still has one.
    pub scene: SceneRow,
    /// Whether the timing session turns the camera once around the hero pose
    /// instead of holding it still. The still beside it is always the hero
    /// pose: the orbit is what is measured, not what is judged.
    pub orbit: bool,
    /// The far end of the walk. With it the run is a transition and `--out`
    /// names the sequence rather than one still.
    pub to: Option<String>,
    pub schedule: Schedule,
}

/// A walk stated in time: seconds of blend, seconds of stillness at each end,
/// and the degrees of azimuth the camera drifts across the whole sequence.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Walk {
    pub seconds: f64,
    pub hold: f64,
    pub sweep: f64,
}

/// How a sequence is laid out over its frames: a plain count, which is what
/// `--frames` has always meant, or a walk in seconds with holds at both ends.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Schedule {
    Frames(u32),
    Walk(Walk),
}

/// Where one frame stands: how far the far family has come, and how many
/// degrees the camera has turned about the subject by then.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Step {
    pub blend: f64,
    pub azimuth: f64,
}

/// Smoothstep. A walk that leaves at rest and arrives at rest reads as one
/// tree becoming another; a straight ramp reads as a machine being driven.
pub fn ease(at: f64) -> f64 {
    let at = at.clamp(0.0, 1.0);
    at * at * (3.0 - 2.0 * at)
}

impl Walk {
    /// The frames of stillness at each end, and the frames of blend between
    /// them. A walk has two ends however short it was asked to be.
    fn held(&self) -> u32 {
        (self.hold * f64::from(FPS)).round() as u32
    }

    fn walked(&self) -> u32 {
        ((self.seconds * f64::from(FPS)).round() as u32).max(2)
    }
}

impl Schedule {
    pub fn frames(&self) -> u32 {
        match self {
            Self::Frames(count) => *count,
            Self::Walk(walk) => walk.walked() + walk.held() * 2,
        }
    }

    /// Seconds the whole sequence runs for at [`FPS`].
    pub fn seconds(&self) -> f64 {
        f64::from(self.frames()) / f64::from(FPS)
    }

    pub fn at(&self, frame: u32) -> Step {
        let last = f64::from(self.frames().saturating_sub(1).max(1));
        match self {
            // What `--frames` always did: the blend spread evenly over the
            // sequence, and a camera that does not move.
            Self::Frames(_) => Step {
                blend: f64::from(frame) / last,
                azimuth: 0.0,
            },
            Self::Walk(walk) => {
                let (held, walked) = (walk.held(), walk.walked());
                let along = if frame < held {
                    0.0
                } else if frame >= held + walked {
                    1.0
                } else {
                    f64::from(frame - held) / f64::from(walked - 1)
                };
                Step {
                    // The camera keeps drifting through the holds: it is the
                    // shot that never stops, not the tree.
                    blend: ease(along),
                    azimuth: walk.sweep * f64::from(frame) / last,
                }
            }
        }
    }
}

/// One number off the command line, refused by the name of the flag that
/// wanted it and by what that flag will take.
fn number(flag: &str, raw: &str, wants: &str, holds: impl Fn(f64) -> bool) -> Result<f64, String> {
    let value = raw.parse::<f64>().ok().filter(|value| value.is_finite());
    match value {
        Some(value) if holds(value) => Ok(value),
        _ => Err(format!("{flag} wants {wants}, not \"{raw}\"")),
    }
}

pub fn parse(arguments: impl Iterator<Item = String>) -> Result<Arguments, String> {
    let (mut preset, mut seed, mut out, mut size) = (None, None, None, (1024u32, 1024u32));
    let (mut view, mut level, mut timing) = (View::default(), None, None);
    let mut scene = SceneRow::default();
    let (mut orbit, mut to, mut frames) = (false, None, None);
    let (mut walk, mut hold, mut sweep) = (None, None, None);
    let mut args = arguments;
    while let Some(flag) = args.next() {
        let mut value = || args.next().ok_or(format!("{flag} needs a value\n{USAGE}"));
        match flag.as_str() {
            "--preset" => preset = Some(value()?),
            "--to" => to = Some(value()?),
            "--frames" => {
                let raw = value()?;
                frames = Some(
                    raw.parse::<u32>()
                        .map_err(|_| format!("--frames wants a whole number, not \"{raw}\""))?,
                );
            }
            "--walk" => {
                let raw = value()?;
                walk = Some(number("--walk", &raw, "seconds above zero", |v| v > 0.0)?);
            }
            "--hold" => {
                let raw = value()?;
                hold = Some(number("--hold", &raw, "seconds, never negative", |v| {
                    v >= 0.0
                })?);
            }
            "--sweep" => {
                let raw = value()?;
                sweep = Some(number(
                    "--sweep",
                    &raw,
                    "degrees within -360 to 360",
                    |v| v.abs() <= 360.0,
                )?);
            }
            "--seed" => {
                let raw = value()?;
                seed = Some(
                    raw.parse::<u32>()
                        .map_err(|_| format!("--seed wants a whole number, not \"{raw}\""))?,
                );
            }
            "--out" => out = Some(PathBuf::from(value()?)),
            "--timing" => timing = Some(PathBuf::from(value()?)),
            "--scene" => {
                let raw = value()?;
                scene = SceneRow::parse(&raw).map_err(|error| error.to_string())?;
            }
            "--orbit" => orbit = true,
            "--size" => {
                let raw = value()?;
                size = parse_size(&raw)?;
            }
            "--view" => {
                let raw = value()?;
                view = View::from_id(&raw).ok_or_else(|| {
                    format!("unknown view \"{raw}\"; one of {}", View::NAMES.join(", "))
                })?;
            }
            "--level" => {
                let raw = value()?;
                level = Some(
                    raw.parse::<u32>()
                        .map_err(|_| format!("--level wants a level number, not \"{raw}\""))?,
                );
            }
            other => return Err(format!("unknown argument \"{other}\"\n{USAGE}")),
        }
    }
    if to.is_none() && frames.is_some() {
        return Err(format!(
            "--frames is the length of a walk; it needs --to\n{USAGE}"
        ));
    }
    if to.is_none() && walk.is_some() {
        return Err(format!(
            "--walk is the length of a walk; it needs --to\n{USAGE}"
        ));
    }
    if walk.is_some() && frames.is_some() {
        return Err("--walk states a length in seconds and --frames in frames; pick one".into());
    }
    for (flag, given) in [("--hold", hold.is_some()), ("--sweep", sweep.is_some())] {
        if given && walk.is_none() {
            return Err(format!("{flag} shapes a walk; it needs --walk"));
        }
    }
    if to.is_some() && timing.is_some() {
        return Err("--timing measures one still; a transition is many".into());
    }
    let count = frames.unwrap_or(240);
    let schedule = match walk {
        Some(seconds) => Schedule::Walk(Walk {
            seconds,
            hold: hold.unwrap_or(0.0),
            sweep: sweep.unwrap_or(0.0),
        }),
        None => Schedule::Frames(count),
    };
    if to.is_some() && walk.is_none() && count < 2 {
        return Err(format!("--frames {count}: a walk has two ends"));
    }
    Ok(Arguments {
        preset: preset.ok_or(format!("--preset is required\n{USAGE}"))?,
        seed: seed.ok_or(format!("--seed is required\n{USAGE}"))?,
        out: out.ok_or(format!("--out is required\n{USAGE}"))?,
        size,
        view,
        level,
        timing,
        scene,
        orbit,
        to,
        schedule,
    })
}

fn parse_size(raw: &str) -> Result<(u32, u32), String> {
    let bad = || format!("--size wants WxH in pixels, not \"{raw}\"");
    let (width, height) = raw.split_once(['x', 'X']).ok_or_else(bad)?;
    let (width, height) = (
        width.parse::<u32>().map_err(|_| bad())?,
        height.parse::<u32>().map_err(|_| bad())?,
    );
    if width == 0 || height == 0 {
        return Err(bad());
    }
    Ok((width, height))
}
