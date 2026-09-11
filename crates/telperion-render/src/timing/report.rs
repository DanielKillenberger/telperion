//! What a session reports, and the rule that an invalid one reports no number.
//!
//! Split out of the protocol beside it so neither file outgrows the project's
//! line rule. Every name here is the one the browser module and the headless
//! example already call; the split moved code, not contracts.
use super::{judge, percentile, Verdict, CONDITIONING, MEASURED, WARMUP};

/// Who did the measuring, in the adapter's own words for itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hardware {
    pub adapter: String,
    pub driver: String,
    pub backend: &'static str,
}

impl From<&wgpu::AdapterInfo> for Hardware {
    fn from(info: &wgpu::AdapterInfo) -> Self {
        Self {
            adapter: info.name.clone(),
            driver: format!("{} {}", info.driver, info.driver_info)
                .trim()
                .to_owned(),
            backend: info.backend.to_str(),
        }
    }
}

/// How many leaves one level drew, over the measured frames. The bucket of
/// leaves no level drew has no tolerance of its own, so its deviation is
/// absent rather than zero: nothing was approximated, the leaf was not shown.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LevelCount {
    pub deviation_m: Option<f64>,
    pub instances_p50: u32,
}

/// One measured session, in the terms a reader of the evidence needs. Every
/// GPU percentile exists only on a valid verdict, so nothing in an invalid
/// record can be mistaken for a number that passed. The wall clock is the one
/// number that is not the GPU's, and it is judged on its own series.
#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub hardware: Hardware,
    pub conditioning: usize,
    pub warmup: usize,
    pub measured: usize,
    /// Samples actually kept, which is `measured` unless the session stopped.
    pub samples: usize,
    verdict: Verdict,
    p50_ms: Option<f64>,
    p95_ms: Option<f64>,
    /// The selection compute pass, median and tail.
    selection: Option<[f64; 2]>,
    /// The sun's depth pass, median and tail.
    shadow: Option<[f64; 2]>,
    /// The three passes the tree costs together, per frame and then ranked:
    /// the pair of numbers the frame budget is spent on.
    total: Option<[f64; 2]>,
    levels: Vec<LevelCount>,
    /// Frame to frame on the host: median, tail, worst.
    wall: Option<[f64; 3]>,
    /// How many frame-to-frame waits those three are taken over. A wall
    /// window is a length of time, so the frames it came to are the display's
    /// answer rather than a constant, and the record says which.
    wall_frames: usize,
}

impl Report {
    /// The report of a session that ran, judged by its own samples. The
    /// samples are the vegetation pass; everything else is added onto it.
    pub fn measured(hardware: Hardware, samples: &[f64]) -> Self {
        let verdict = judge(samples);
        let percentiles = verdict.is_valid().then(|| ranked(samples, [0.5, 0.95]));
        Self {
            samples: samples.len(),
            p50_ms: percentiles.map(|pair| pair[0]),
            p95_ms: percentiles.map(|pair| pair[1]),
            verdict,
            ..Self::blank(hardware)
        }
    }

    /// The report of a session that could not run at all.
    pub fn unavailable(hardware: Hardware, reason: impl Into<String>) -> Self {
        Self {
            verdict: Verdict::Unavailable(reason.into()),
            ..Self::blank(hardware)
        }
    }

    /// Adds what the other two passes of a frame cost - the selection pass and
    /// the sun's depth pass - and what all three came to together. Each pass
    /// stands on its own account: a pair no pass wrote reads as a zero rather
    /// than a duration and is refused, and a refused pass takes only itself and
    /// the total with it. The frame's own verdict judges the frame.
    pub fn with_passes(mut self, vegetation: &[f64], selection: &[f64], shadow: &[f64]) -> Self {
        if !self.verdict.is_valid() {
            return self;
        }
        let measured = |samples: &[f64]| {
            (durations(samples) && samples.len() == vegetation.len())
                .then(|| ranked(samples, [0.5, 0.95]))
        };
        self.selection = measured(selection);
        self.shadow = measured(shadow);
        if self.selection.is_none() || self.shadow.is_none() {
            return self;
        }
        let total: Vec<f64> = (0..vegetation.len())
            .map(|frame| vegetation[frame] + selection[frame] + shadow[frame])
            .collect();
        self.total = Some(ranked(&total, [0.5, 0.95]));
        self
    }

    /// Adds what each level drew, as the median over the measured frames. One
    /// entry per level, coarsest first, and last the bucket of leaves no level
    /// drew. A frame whose readback disagrees with the ladder adds nothing:
    /// a partial count is not a count.
    pub fn with_levels(mut self, deviations: &[f64], frames: &[Vec<u32>]) -> Self {
        let wanted = deviations.len() + 1;
        if !self.verdict.is_valid()
            || frames.is_empty()
            || frames.iter().any(|frame| frame.len() != wanted)
        {
            return self;
        }
        self.levels = (0..wanted)
            .map(|level| {
                let counts: Vec<f64> = frames.iter().map(|f| f64::from(f[level])).collect();
                LevelCount {
                    deviation_m: deviations.get(level).copied(),
                    instances_p50: ranked(&counts, [0.5])[0].round() as u32,
                }
            })
            .collect();
        self
    }

    /// Adds the frame-to-frame wall clock of an orbit session: how long the
    /// host waited between one measured frame and the next, and over how many
    /// frames.
    ///
    /// This is the host's own clock and not the GPU's, so it stands on its own
    /// account rather than on the verdict beside it: a session that could not
    /// be timed at all still kept a cadence, and on a page that cadence is the
    /// frame rate a viewer saw. A series that is not durations is still
    /// refused, here as everywhere.
    pub fn with_wall(mut self, samples: &[f64]) -> Self {
        if !durations(samples) {
            return self;
        }
        let [median, tail] = ranked(samples, [0.5, 0.95]);
        let worst = samples.iter().copied().fold(f64::MIN, f64::max);
        self.wall = Some([median, tail, worst]);
        self.wall_frames = samples.len();
        self
    }

    fn blank(hardware: Hardware) -> Self {
        Self {
            hardware,
            conditioning: CONDITIONING,
            warmup: WARMUP,
            measured: MEASURED,
            samples: 0,
            verdict: Verdict::Valid,
            p50_ms: None,
            p95_ms: None,
            selection: None,
            shadow: None,
            total: None,
            levels: Vec::new(),
            wall: None,
            wall_frames: 0,
        }
    }

    pub fn verdict(&self) -> &Verdict {
        &self.verdict
    }

    /// The median measured vegetation pass, milliseconds, on a valid session
    /// only.
    pub fn p50_ms(&self) -> Option<f64> {
        self.p50_ms
    }

    /// The p95 measured vegetation pass, milliseconds, on a valid session only.
    pub fn p95_ms(&self) -> Option<f64> {
        self.p95_ms
    }

    /// The median selection pass, milliseconds.
    pub fn selection_p50_ms(&self) -> Option<f64> {
        self.selection.map(|pair| pair[0])
    }

    /// The median shadow pass, milliseconds. The number the sun's own budget is
    /// judged on.
    pub fn shadow_p50_ms(&self) -> Option<f64> {
        self.shadow.map(|pair| pair[0])
    }

    /// The p95 shadow pass, milliseconds.
    pub fn shadow_p95_ms(&self) -> Option<f64> {
        self.shadow.map(|pair| pair[1])
    }

    /// The median of the three passes together, which is what the hero budget
    /// is judged on.
    pub fn total_p50_ms(&self) -> Option<f64> {
        self.total.map(|pair| pair[0])
    }

    /// The median frame-to-frame wall time of an orbit session.
    pub fn wall_p50_ms(&self) -> Option<f64> {
        self.wall.map(|three| three[0])
    }

    /// The worst frame-to-frame wall time of an orbit session.
    pub fn wall_max_ms(&self) -> Option<f64> {
        self.wall.map(|three| three[2])
    }

    /// What each level drew, median over the measured frames, the unseen
    /// bucket last. Empty unless the session both ran and read the counters.
    pub fn levels(&self) -> &[LevelCount] {
        &self.levels
    }

    /// The record as it is committed to the evidence. Absent percentiles are
    /// absent keys, never zeroes. The fields fn-22's records carry keep their
    /// names, their order and their formatting; everything this spec adds
    /// follows them.
    pub fn to_json(&self) -> String {
        let mut fields = vec![
            format!("\"adapter\": {}", quote(&self.hardware.adapter)),
            format!("\"driver\": {}", quote(&self.hardware.driver)),
            format!("\"backend\": {}", quote(self.hardware.backend)),
            format!("\"conditioning\": {}", self.conditioning),
            format!("\"warmup\": {}", self.warmup),
            format!("\"measured\": {}", self.measured),
            format!("\"samples\": {}", self.samples),
            format!("\"verdict\": {}", quote(self.verdict.name())),
        ];
        if let Some(reason) = self.verdict.reason() {
            fields.push(format!("\"reason\": {}", quote(reason)));
        }
        if let (Some(median), Some(tail)) = (self.p50_ms, self.p95_ms) {
            fields.push(format!("\"p50_ms\": {median:.4}"));
            fields.push(format!("\"p95_ms\": {tail:.4}"));
        }
        if let Some([median, tail]) = self.selection {
            fields.push(format!("\"selection_p50_ms\": {median:.4}"));
            fields.push(format!("\"selection_p95_ms\": {tail:.4}"));
        }
        if let Some([median, tail]) = self.shadow {
            fields.push(format!("\"shadow_p50_ms\": {median:.4}"));
            fields.push(format!("\"shadow_p95_ms\": {tail:.4}"));
        }
        if let Some([median, tail]) = self.total {
            fields.push(format!("\"total_p50_ms\": {median:.4}"));
            fields.push(format!("\"total_p95_ms\": {tail:.4}"));
        }
        if !self.levels.is_empty() {
            fields.push(format!("\"levels\": [\n    {}\n  ]", self.levels_json()));
        }
        if let Some([median, tail, worst]) = self.wall {
            fields.push(format!("\"wall_frames\": {}", self.wall_frames));
            fields.push(format!("\"wall_p50_ms\": {median:.4}"));
            fields.push(format!("\"wall_p95_ms\": {tail:.4}"));
            fields.push(format!("\"wall_max_ms\": {worst:.4}"));
        }
        format!("{{\n  {}\n}}\n", fields.join(",\n  "))
    }

    fn levels_json(&self) -> String {
        self.levels
            .iter()
            .map(|level| {
                let deviation = level
                    .deviation_m
                    .map_or("null".to_owned(), |metres| format!("{metres:.6}"));
                format!(
                    "{{ \"deviation_m\": {deviation}, \"instances_p50\": {} }}",
                    level.instances_p50
                )
            })
            .collect::<Vec<_>>()
            .join(",\n    ")
    }
}

/// The named percentiles of a series, each by nearest rank.
fn ranked<const N: usize>(samples: &[f64], fractions: [f64; N]) -> [f64; N] {
    let mut sorted = samples.to_vec();
    sorted.sort_by(f64::total_cmp);
    fractions.map(|fraction| percentile(&sorted, fraction))
}

/// Whether every sample in a series is a length of time. A pass that never ran
/// resolves to a pair of zeroes and arrives here as one.
fn durations(samples: &[f64]) -> bool {
    !samples.is_empty() && samples.iter().all(|s| s.is_finite() && *s > 0.0)
}

/// A JSON string literal. Adapter and driver names are the platform's words,
/// so they are escaped rather than trusted.
fn quote(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if control < ' ' => out.push_str(&format!("\\u{:04x}", control as u32)),
            ordinary => out.push(ordinary),
        }
    }
    out.push('"');
    out
}
