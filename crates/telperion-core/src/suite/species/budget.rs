//! What the whole test binary may spend on specimens at once.
//!
//! Three numbers, not two. `MemAvailable` is read once, at the first call: the
//! machine keeps a **reserve** of the larger of two gibibytes and a quarter of
//! it, what is left is the **process ceiling** this binary holds itself to,
//! and admission may charge only two thirds of that ceiling. The third between
//! them is headroom for the two things a prediction does not size - the
//! fifteen percent it is allowed to underestimate the retained buffers by, and
//! the transient allocations that sit on top of them while a seed is checked -
//! and it is inside the ceiling rather than outside it.
//!
//! One budget serves the whole binary. Each heavy test calls `in_flight`
//! independently, so a budget divided per test would let every one of them
//! spend the same memory at once. A seed is charged its own prediction once it
//! is grown, and the charge is released when it is dropped, whether its checks
//! returned or panicked.

use std::{
    fs,
    sync::{Condvar, Mutex, MutexGuard, OnceLock},
};

/// The machine keeps at least this much whatever else is going on.
const RESERVE_FLOOR: u64 = 2 * (1 << 30);

/// The one budget the binary shares, from this host's own free memory.
pub fn budget() -> &'static Budget {
    static SHARED: OnceLock<Budget> = OnceLock::new();
    SHARED.get_or_init(Budget::of_host)
}

pub struct Budget {
    /// What this process may reach, in bytes.
    ceiling: u64,
    /// What admission may charge at once: two thirds of the ceiling.
    charge_limit: u64,
    /// The most seeds checked at once whatever the arithmetic allows.
    cores: usize,
    state: Mutex<State>,
    room: Condvar,
}

#[derive(Default)]
struct State {
    charged: u64,
    peak: u64,
    /// Seats taken by threads growing or checking a seed.
    seated: usize,
    /// Admission is first come, first served: a specimen too large to fit
    /// beside what is already charged holds the queue rather than starving
    /// behind a stream of smaller ones.
    next: u64,
    serving: u64,
    /// Every specimen admitted although it asked for more than the whole
    /// charge limit, named with what it asked for.
    outside: Vec<String>,
}

impl Budget {
    /// The three numbers this host's free memory gives, read once.
    pub fn of_host() -> Self {
        Self::with(ceiling_from(available_bytes().unwrap_or(0)), cores())
    }

    /// A budget on a stated ceiling, for a test that drives it rather than
    /// the host that has to live within it.
    pub fn with(ceiling: u64, cores: usize) -> Self {
        Self {
            ceiling,
            charge_limit: ceiling / 3 * 2,
            cores: cores.max(1),
            state: Mutex::new(State::default()),
            room: Condvar::new(),
        }
    }

    pub fn ceiling(&self) -> u64 {
        self.ceiling
    }

    pub fn charge_limit(&self) -> u64 {
        self.charge_limit
    }

    pub fn cores(&self) -> usize {
        self.cores
    }

    /// The most charges of this size that stand together: never none, never
    /// more than the machine has cores to check them on.
    pub fn admission(&self, predicted: u64) -> usize {
        let fitting = match predicted {
            0 => self.cores,
            size => (self.charge_limit / size) as usize,
        };
        fitting.clamp(1, self.cores)
    }

    /// The highest the charges ever stood together.
    pub fn peak(&self) -> u64 {
        self.locked().peak
    }

    /// What the charges stand at now.
    pub fn charged(&self) -> u64 {
        self.locked().charged
    }

    /// Every specimen that ran outside the bound, in the order they were let
    /// through.
    pub fn outside(&self) -> Vec<String> {
        self.locked().outside.clone()
    }

    /// A seat at the work: no more threads grow or check a seed at once than
    /// the machine has cores, however many tests are asking.
    pub fn seat(&self) -> Seat<'_> {
        let mut state = self.locked();
        while state.seated >= self.cores {
            state = self.wait(state);
        }
        state.seated += 1;
        Seat { budget: self }
    }

    /// Charges `bytes` against the limit, waiting for room. A specimen larger
    /// than the whole charge limit is admitted alone rather than refused -
    /// one seed always runs - and says so by name when it is, so an OOM reads
    /// as a stated condition rather than a killed process.
    pub fn charge(&self, name: &str, bytes: u64) -> Charge<'_> {
        let mut state = self.locked();
        let ticket = state.next;
        state.next += 1;
        loop {
            let room =
                state.charged == 0 || state.charged.saturating_add(bytes) <= self.charge_limit;
            if state.serving == ticket && room {
                break;
            }
            state = self.wait(state);
        }
        state.serving += 1;
        if bytes > self.charge_limit {
            let report = format!(
                "{name} predicts {bytes} bytes, over the whole charge limit of {} bytes: \
                 it runs alone and outside the bound",
                self.charge_limit
            );
            eprintln!("{report}");
            state.outside.push(report);
        }
        state.charged += bytes;
        state.peak = state.peak.max(state.charged);
        drop(state);
        self.room.notify_all();
        Charge {
            budget: self,
            bytes,
        }
    }

    /// The binary's own high-water mark against the ceiling it holds itself
    /// to. A run that had to admit a specimen larger than the whole charge
    /// limit is exempt, and only that: it named the specimen when it did it.
    pub fn hold_to_the_ceiling(&self) {
        let outside = self.outside();
        if !outside.is_empty() {
            eprintln!(
                "the ceiling is not asserted on this run: {}",
                outside.join("; ")
            );
            return;
        }
        let Some(peak) = peak_resident() else {
            eprintln!("the ceiling is not asserted: this host reports no VmHWM");
            return;
        };
        println!(
            "peak resident {peak} bytes, process ceiling {} bytes, \
             charge limit {} bytes, peak charged {} bytes, seats {}",
            self.ceiling,
            self.charge_limit,
            self.peak(),
            self.cores,
        );
        assert!(
            peak < self.ceiling,
            "peak resident {peak} bytes stands over the process ceiling {} bytes \
             (charge limit {} bytes, peak charged {} bytes)",
            self.ceiling,
            self.charge_limit,
            self.peak(),
        );
    }

    fn locked(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|held| held.into_inner())
    }

    fn wait<'a>(&'a self, state: MutexGuard<'a, State>) -> MutexGuard<'a, State> {
        self.room
            .wait(state)
            .unwrap_or_else(|held| held.into_inner())
    }
}

/// One admitted specimen's charge, released when the specimen is dropped -
/// on the way out of a panic as readily as on the way out of a return.
pub struct Charge<'a> {
    budget: &'a Budget,
    bytes: u64,
}

impl Drop for Charge<'_> {
    fn drop(&mut self) {
        self.budget.locked().charged -= self.bytes;
        self.budget.room.notify_all();
    }
}

/// One thread's seat at the work.
pub struct Seat<'a> {
    budget: &'a Budget,
}

impl Drop for Seat<'_> {
    fn drop(&mut self) {
        self.budget.locked().seated -= 1;
        self.budget.room.notify_all();
    }
}

/// What a machine with this much free memory leaves this process: it keeps the
/// larger of two gibibytes and a quarter of what it has, and the rest is the
/// ceiling. A machine with less than the reserve leaves nothing, which admits
/// one specimen at a time and says so of every one of them.
fn ceiling_from(available: u64) -> u64 {
    available.saturating_sub(RESERVE_FLOOR.max(available / 4))
}

/// Memory this machine can hand out without swapping, in bytes.
fn available_bytes() -> Option<u64> {
    field("/proc/meminfo", "MemAvailable:")
}

/// The highest resident set this process has held, in bytes.
pub fn peak_resident() -> Option<u64> {
    field("/proc/self/status", "VmHWM:")
}

/// One `name: <value> kB` row of a proc file, in bytes.
fn field(path: &str, name: &str) -> Option<u64> {
    let text = fs::read_to_string(path).ok()?;
    let row = text.lines().find(|line| line.starts_with(name))?;
    let kb: u64 = row.split_whitespace().nth(1)?.parse().ok()?;
    Some(kb * 1024)
}

fn cores() -> usize {
    std::thread::available_parallelism().map_or(1, |n| n.get())
}

/// The child run the saturation test spawns: set, it drives the budget to its
/// whole charge limit in a process of its own and holds its own high-water
/// mark to the ceiling.
pub const SATURATION: &str = "TELPERION_BUDGET_SATURATION";

/// A ceiling a saturation run can actually reach, in bytes, and the specimens
/// it is filled with: eight at a time, each a share of the charge limit.
const TRIAL_CEILING: u64 = 384 * (1 << 20);
const TRIAL_SPECIMENS: usize = 8;

/// Fills the charge limit with real allocations and reports what the process
/// reached. Each admitted specimen retains a fifteenth more than it charged,
/// the most a prediction is allowed to underestimate by, and allocates its
/// transient beside it - the buffer a digest serialises into, the scratch a
/// metrics pass takes - with every specimen's transient live at once, because
/// that is what the harness does.
///
/// The transient is a share of the charge rather than the whole of it. The
/// three runs in AFTER.md measured the resident set at 1.23, 1.27 and 1.26
/// times what was charged, so the overhead above the retained buffers is a
/// tenth of a charge, not a charge; modelling it as a whole charge and then
/// serialising the threads to survive it would be a test passing on a
/// restriction the harness does not have.
pub fn saturate() {
    let budget = Budget::with(TRIAL_CEILING, TRIAL_SPECIMENS);
    let share = budget.charge_limit() / TRIAL_SPECIMENS as u64;
    assert_eq!(budget.admission(share), TRIAL_SPECIMENS);
    let ready = std::sync::Barrier::new(TRIAL_SPECIMENS);
    let overlapped = std::sync::Barrier::new(TRIAL_SPECIMENS);
    std::thread::scope(|scope| {
        for k in 0..TRIAL_SPECIMENS {
            let (budget, ready, overlapped) = (&budget, &ready, &overlapped);
            scope.spawn(move || {
                let _charge = budget.charge(&format!("trial {k}"), share);
                let mut kept = touched(share + share * 15 / 100);
                // Every specimen stands charged at once: the sum of charges is
                // the whole charge limit, which is what the ceiling has to
                // survive.
                ready.wait();
                let passing = touched(share / 10);
                // And every transient is live at once beside them, with
                // nothing serialising the threads.
                overlapped.wait();
                kept[0] = kept[0].wrapping_add(passing[0]);
                std::hint::black_box((&kept, &passing));
            });
        }
    });
    assert_eq!(
        budget.peak(),
        budget.charge_limit(),
        "the trial did not saturate the budget"
    );
    assert!(budget.outside().is_empty());
    budget.hold_to_the_ceiling();
    println!(
        "saturated {} bytes of charges under a {} byte ceiling; peak resident {:?} bytes",
        budget.peak(),
        budget.ceiling(),
        peak_resident(),
    );
}

/// Bytes that are really there: written to, so the pages are resident.
fn touched(bytes: u64) -> Vec<u8> {
    let mut block = vec![0_u8; bytes as usize];
    for page in block.chunks_mut(4096) {
        page[0] = 1;
    }
    block
}

/// The machine's reserve, the process ceiling and the charge limit are three
/// numbers, and each is a stated share of the one before it.
#[test]
fn the_reserve_the_ceiling_and_the_charge_limit_are_three_numbers() {
    let gib = 1_u64 << 30;
    // Under four times the floor the machine keeps the floor; above it, the
    // quarter is the larger and the machine keeps that.
    for (available, ceiling) in [
        (0, 0),
        (gib, 0),
        (2 * gib, 0),
        (4 * gib, 2 * gib),
        (8 * gib, 6 * gib),
        (64 * gib, 48 * gib),
    ] {
        assert_eq!(
            ceiling_from(available),
            ceiling,
            "at {available} bytes free"
        );
        let budget = Budget::with(ceiling, 4);
        assert_eq!(budget.ceiling(), ceiling);
        assert_eq!(budget.charge_limit(), ceiling / 3 * 2);
        assert!(
            budget.charge_limit() < budget.ceiling() || ceiling == 0,
            "the charge limit is the smaller of the two"
        );
    }
}

/// A specimen larger than the whole charge limit is admitted rather than
/// refused - one seed always runs - it is named where it happens, and that
/// one state, and only it, waives the ceiling the rest of the run is held to.
#[test]
fn a_specimen_over_the_whole_limit_runs_alone_and_says_so() {
    let budget = Budget::with(1 << 20, 4);
    assert_eq!(budget.admission(budget.charge_limit() * 4), 1);
    {
        let _charge = budget.charge("a tree too large", budget.charge_limit() * 4);
        assert_eq!(budget.charged(), budget.charge_limit() * 4);
    }
    let outside = budget.outside();
    assert_eq!(outside.len(), 1);
    assert!(
        outside[0].contains("a tree too large") && outside[0].contains("charge limit"),
        "the report names neither the specimen nor the limit: {}",
        outside[0]
    );
    // Its peak stands over the ceiling, and the run says why rather than
    // failing an assertion nobody could have met.
    assert!(budget.peak() > budget.ceiling());
    budget.hold_to_the_ceiling();
}
