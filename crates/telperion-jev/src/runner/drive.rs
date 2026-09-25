//! Running the stages: all of them, up to one, or one alone, and the status
//! of each without running any.
use super::record::{Records, State};
use super::{Done, Run, Stage, Stop};

/// Which stages a run takes.
pub enum Scope {
    /// Every stage, to the owner's look.
    All,
    /// Every stage up to and including the named one.
    Until(String),
    /// The named stage alone; what earlier stages write for it must exist.
    Only(String),
}

/// Runs the stages `scope` takes, in order, until one stops. `Ok(None)` is
/// a run that reached the end of its scope: for `All`, the tree accepted.
pub fn run(run: &Run, stages: &[&dyn Stage], scope: &Scope) -> Result<Option<Stop>, String> {
    std::fs::create_dir_all(run.out()).map_err(|e| e.to_string())?;
    let mut records = Records::load(&run.out())?;
    let taken = match scope {
        Scope::All => stages,
        Scope::Until(name) => &stages[..=at(stages, name)?],
        Scope::Only(name) => {
            let at = at(stages, name)?;
            written(run, &stages[..at], stages[at])?;
            &stages[at..=at]
        }
    };
    // A file an earlier stage of this run reads and a later one writes (the
    // resolutions Profile settles claims into, which Sources reads) is
    // settled by the run: the earlier stage's record takes the new bytes,
    // so a second run reruns nothing. An edit between runs still reruns it.
    let mut settled: Vec<&dyn Stage> = Vec::new();
    for stage in taken {
        let (ran, stop) = step(run, &mut records, *stage)?;
        if ran {
            for earlier in &settled {
                records.set(earlier.name(), &earlier.inputs(run)?)?;
            }
        }
        settled.push(*stage);
        if let Some(stop) = stop {
            records.log(stage.name(), &format!("stopped: {stop}"))?;
            return Ok(Some(stop));
        }
    }
    Ok(None)
}

/// Each stage's state, read from its record and the files on disk; a stage
/// whose files cannot be named yet is missing, with the reason.
pub fn status(run: &Run, stages: &[&dyn Stage]) -> Result<Vec<(&'static str, State)>, String> {
    let records = Records::load(&run.out())?;
    let state = |s: &dyn Stage| records.state(s.name(), &s.inputs(run)?, &s.outputs(run)?);
    Ok(stages
        .iter()
        .map(|s| (s.name(), state(*s).unwrap_or_else(State::Missing)))
        .collect())
}

/// Runs `stage` unless it is current, records it, and reads its stop; the
/// flag says whether it ran.
fn step(
    run: &Run,
    records: &mut Records,
    stage: &dyn Stage,
) -> Result<(bool, Option<Stop>), String> {
    let name = stage.name();
    let state = records.state(name, &stage.inputs(run)?, &stage.outputs(run)?)?;
    let done = match state {
        State::Current => Done::Current,
        _ => {
            let done = stage.run(run).map_err(|e| {
                let _ = records.log(name, &format!("failed: {e}"));
                format!("{name}: {e}")
            })?;
            // Inputs are read again: a stage may write a file it also reads.
            records.set(name, &stage.inputs(run)?)?;
            done
        }
    };
    let word = match &done {
        Done::Current => "current".to_string(),
        Done::Ran(word) => format!("ran: {word}"),
    };
    records.log(name, &word)?;
    println!("{name}: {word}");
    Ok((matches!(done, Done::Ran(_)), stage.stop(run)?))
}

fn at(stages: &[&dyn Stage], name: &str) -> Result<usize, String> {
    stages.iter().position(|s| s.name() == name).ok_or_else(|| {
        let names: Vec<&str> = stages.iter().map(|s| s.name()).collect();
        format!("no stage {name}; the stages are {}", names.join(", "))
    })
}

/// Refuses to run `stage` alone while a file an earlier stage writes for
/// it is missing, naming the file and the stage that writes it.
fn written(run: &Run, earlier: &[&dyn Stage], stage: &dyn Stage) -> Result<(), String> {
    let inputs = stage.inputs(run)?;
    for writer in earlier {
        for path in writer.outputs(run)? {
            if inputs.contains(&path) && !path.exists() {
                return Err(format!(
                    "{} needs {}, which {} writes: run `species {} --until {}` first",
                    stage.name(),
                    path.display(),
                    writer.name(),
                    run.species,
                    writer.name()
                ));
            }
        }
    }
    Ok(())
}
