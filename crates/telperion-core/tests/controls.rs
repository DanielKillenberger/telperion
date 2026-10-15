//! Every control the harness shows changes the tree of each shipped preset,
//! or its row says where it lies dormant (fn-284 R1). A shown row is one a
//! stage of the build reads (`catalogue::browser`'s reach); each is moved to
//! the far end of its tuning window on each preset and the whole build is
//! compared, with the rows the renderer reads off the family besides.
//!
//! On demand, not the gate: a full build per row and preset is four to nine
//! minutes a preset. Run it as
//!
//! ```sh
//! telperion-heavy 10 --threads 4 -- cargo test --profile ci -p telperion-core \
//!     --test controls -- --ignored --nocapture
//! ```
//!
//! `TELPERION_PRESETS=date-palm,ordinary` narrows it to those presets.
use std::fmt::Write;
use telperion_core::catalogue::{self, Dial, Entry, Kind};
use telperion_core::field::IndexSnapshot;
use telperion_core::{pipeline, presets, Family};

/// FNV-1a over the `Debug` text of a value.
fn digest(value: &dyn std::fmt::Debug) -> u64 {
    struct Fnv(u64);
    impl Write for Fnv {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            for b in s.bytes() {
                self.0 = (self.0 ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
            }
            Ok(())
        }
    }
    let mut fnv = Fnv(0xcbf2_9ce4_8422_2325);
    let _ = write!(fnv, "{value:?}");
    fnv.0
}

type Digests = Vec<(&'static str, u64)>;

/// Every artifact a build returns, and the rows the renderer reads off the
/// family: its drawn material, its reflection and its flower stalks.
fn build(family: &Family) -> Result<Digests, String> {
    let request = pipeline::Request {
        wood: true,
        curve: true,
        leaves: true,
        structure: true,
        field: Some(None),
        ..pipeline::Request::default()
    };
    let built = pipeline::build(family, request).map_err(|e| format!("{e:?}"))?;
    let out = &built.outputs;
    let leaves = out.leaves.as_ref().map(|l| (&l.instances, l.placed, l.retained));
    let structure = out.structure.as_ref().map(|s| (&s.nodes, &s.topology));
    let field = out.field.as_ref().map(|f| f.snapshot().map(|s| field_digest(&s)));
    let bloom = family.applied().bloom;
    let stalk = (bloom.rachis, bloom.droop, bloom.stamen_width, bloom.pedicel);
    Ok(vec![
        ("skeleton", digest(&built.skeleton.tree)),
        ("wood", digest(&out.wood)),
        ("curve", digest(&out.curve)),
        ("element", digest(&out.element)),
        ("leaves", digest(&leaves)),
        ("organ", digest(&out.organ)),
        ("structure", digest(&structure)),
        ("field", digest(&field.map(|f| f.ok()))),
        ("material", digest(&family.drawn_material())),
        ("reflection", digest(&family.supernatural.reflection())),
        ("stalk", digest(&stalk)),
    ])
}

fn field_digest(f: &telperion_core::field::FieldSnapshot) -> u64 {
    let index = |i: &IndexSnapshot| digest(&(&i.bounds, &i.topology, i.node_count));
    digest(&f.wood)
        ^ index(&f.wood_index)
        ^ index(&f.leaves)
        ^ digest(&(&f.plan, &f.plan_stations, &f.plan_sides))
        ^ index(&f.plan_index)
}

/// The rows the harness shows: those a stage of the build reads, as the
/// browser table derives its reach, but the age, which the age dial owns.
fn shown() -> Vec<Entry> {
    let read = |e: &Entry| !e.info().reads.is_empty() && e.path() != "/age";
    catalogue::entries().filter(read).collect()
}

/// The admitted value at the far end of the row's tuning window (its
/// bounds where it has none) from its value: none where it cannot move.
fn target(e: Entry, family: &Family) -> Option<f64> {
    let info = e.info();
    let (kind, now) = (e.get(family).kind(), e.get(family).number());
    if kind == Kind::Switch {
        return Some(if now == Some(1.0) { 0.0 } else { 1.0 });
    }
    let (b, tuning) = match info.dial {
        Dial::Tuned(t) => (info.bounds, Some(t)),
        _ => (info.bounds, None),
    };
    let [lo, hi] = tuning.and_then(|t| t.window).unwrap_or([b.low, b.high]);
    let (lo, hi) = (lo.max(b.low), hi.min(b.high));
    let from = now.or(tuning.and_then(|t| t.unset));
    let mut to = far(from, lo, hi);
    if b.low_open && to <= b.low {
        to = b.low + 0.5 * (from.unwrap_or(b.low + 1.0) - b.low).max(1e-3);
    }
    if kind.whole() {
        to = to.round();
    }
    (Some(to) != now).then_some(to)
}

/// The end of `[lo, hi]` farther from `from`, or a step off it where the
/// window is open.
fn far(from: Option<f64>, lo: f64, hi: f64) -> f64 {
    match (from, lo.is_finite(), hi.is_finite()) {
        (None, true, true) => 0.5 * (lo + hi),
        (None, true, false) => lo + 1.0,
        (None, _, _) => 1.0,
        (Some(c), true, true) if (hi - c).abs() >= (c - lo).abs() => hi,
        (Some(_), true, true) => lo,
        (Some(c), true, false) if c > lo => lo.max(c * 0.5),
        (Some(c), true, false) => (c * 2.0).max(lo + 1.0),
        (Some(c), false, _) => c * 2.0 + 1.0,
    }
}

/// What moving a row did to one preset's build.
enum Verdict {
    /// The artifacts that changed.
    Live(Vec<&'static str>),
    /// The build refused the value, by name: the harness shows the refusal.
    Refused(String),
    Same,
    Unmoved,
}

fn verdict(e: Entry, base: &Family, before: &Digests) -> Verdict {
    let mut family = base.clone();
    let Some(to) = target(e, &family) else {
        return Verdict::Unmoved;
    };
    e.set(&mut family).put(to);
    match build(&family) {
        Err(error) => Verdict::Refused(error),
        Ok(after) => {
            let moved: Vec<_> = (before.iter().zip(&after))
                .filter(|(a, b)| a.1 != b.1)
                .map(|(a, _)| a.0)
                .collect();
            if moved.is_empty() {
                Verdict::Same
            } else {
                Verdict::Live(moved)
            }
        }
    }
}

/// Each shown row's verdict on one preset, the rows shared across threads.
fn probe(base: &Family, rows: &[Entry]) -> Vec<(Entry, Verdict)> {
    let before = build(base).expect("the preset builds");
    let threads = threads();
    std::thread::scope(|s| {
        let handles: Vec<_> = (rows.chunks(rows.len().div_ceil(threads)))
            .map(|chunk| {
                let before = &before;
                let each = move |&e: &Entry| (e, verdict(e, base, before));
                s.spawn(move || chunk.iter().map(each).collect::<Vec<_>>())
            })
            .collect();
        (handles.into_iter())
            .flat_map(|h| h.join().expect("a probe thread"))
            .collect()
    })
}

/// The builds run at once: `RAYON_NUM_THREADS`, which `telperion-heavy
/// --threads` sets, so the probe keeps to the memory its cap allows.
fn threads() -> usize {
    let set = std::env::var("RAYON_NUM_THREADS").ok().and_then(|n| n.parse().ok());
    set.unwrap_or_else(|| std::thread::available_parallelism().map_or(1, usize::from))
}

/// The shipped presets the probe builds, or those `TELPERION_PRESETS` names.
fn ids() -> Vec<String> {
    let all = presets::CATALOGUE.iter().map(|entry| entry.1.to_owned());
    match std::env::var("TELPERION_PRESETS") {
        Ok(list) => list.split(',').map(str::to_owned).collect(),
        Err(_) => all.collect(),
    }
}

#[test]
#[ignore = "on demand: a full build per shown row and preset (see the module doc)"]
fn every_shown_control_changes_the_tree_or_says_it_is_dormant() {
    let rows = shown();
    let mut silent = Vec::new();
    for id in ids() {
        let base = presets::by_identity(&id).expect("a shipped preset");
        let (mut live, mut dormant, mut refused) = (0, 0, 0);
        for (e, v) in probe(&base, &rows) {
            let applies = e.info().applies;
            match v {
                Verdict::Live(moved) => {
                    live += 1;
                    println!("{id}\t{}\tlive\t{}", e.path(), moved.join(","));
                }
                Verdict::Refused(why) => {
                    refused += 1;
                    println!("{id}\t{}\trefused\t{why}", e.path());
                }
                Verdict::Same | Verdict::Unmoved if !applies.is_empty() => {
                    dormant += 1;
                    println!("{id}\t{}\tdormant\t{applies}", e.path());
                }
                Verdict::Same | Verdict::Unmoved => {
                    println!("{id}\t{}\tSILENT", e.path());
                    silent.push(format!("{id} {}", e.path()));
                }
            }
        }
        println!("{id}: {} shown, {live} live, {dormant} dormant, {refused} refused", rows.len());
    }
    assert!(silent.is_empty(), "shown, unread and not dormant: {silent:#?}");
}
