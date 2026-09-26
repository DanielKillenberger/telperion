use super::rings::Scratch;
use super::*;
use std::{mem::size_of, thread};
const MAX_WORKERS: usize = 8;
const STACK: usize = 64 * 1024;
const WORKER_ENVELOPE: usize = STACK + 4096 + 64 * 1024;
const CONTROL: usize = 16 * 1024;
fn workers() -> usize {
    thread::available_parallelism().map_or(1, |n| n.get().min(MAX_WORKERS))
}
fn envelope(
    prep: usize,
    order: usize,
    scratch: usize,
    angular: usize,
    vertices: usize,
    indices: usize,
    runs: usize,
    workers: usize,
) -> bool {
    let output = vertices as u128 * 32 + indices as u128 * 4 + runs as u128 * 16;
    let serial = output + prep as u128 + order as u128 + scratch as u128 + angular as u128;
    let descriptors = runs as u128 * size_of::<SurfaceRun>() as u128;
    let a = vertices as u128 * 20
        + prep as u128
        + angular as u128
        + descriptors
        + workers as u128 * (scratch + WORKER_ENVELOPE) as u128
        + CONTROL as u128;
    let b = output
        + descriptors
        + workers as u128 * (scratch + 2 * WORKER_ENVELOPE) as u128
        + CONTROL as u128;
    a <= serial && b <= serial
}
pub(super) fn admitted(
    at: Swept,
    ordered: &Vec<(usize, f64)>,
    longest: usize,
    vertices: usize,
    indices: usize,
) -> Option<usize> {
    let Swept {
        paths,
        distance,
        angular,
        ..
    } = at;
    let count = workers();
    (count >= 2
        && vertices >= 250_000
        && paths.runs.len() >= count
        && envelope(
            paths.nodes.capacity() * size_of::<usize>()
                + paths.runs.capacity() * size_of::<paths::Run>()
                + paths.forks.capacity()
                + distance.len() * 8,
            ordered.capacity() * size_of::<(usize, f64)>(),
            // A worker's sweep scratch, and one run's normals in the mesh step.
            longest * (size_of::<Sample>() + size_of::<(Vec3, Vec3)>() + size_of::<Vec3>())
                + (longest * angular.len() + 2) * 3 * size_of::<f32>(),
            size_of_val(angular),
            vertices,
            indices,
            paths.runs.len(),
            count,
        ))
    .then_some(count)
}

/// Contiguous run groups of about equal vertices, one a worker: each
/// group's runs, its first vertex and its vertex count. Contiguous groups
/// keep the final buffers disjoint and preserve their order.
fn groups(
    runs: usize,
    vertices: impl Fn(usize) -> usize,
    total: usize,
    count: usize,
) -> impl Iterator<Item = (std::ops::Range<usize>, usize, usize)> {
    let (mut at, mut end) = (0, 0);
    (1..=count).map(move |k| {
        let (start, first) = (at, end);
        let target = if k == count {
            usize::MAX
        } else {
            total * k / count
        };
        while at < runs && end + vertices(at) <= target {
            end += vertices(at);
            at += 1;
        }
        (start..at, first, end - first)
    })
}

#[cfg(test)]
thread_local! {
 static SPAWN_FAILURE: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
 static SPAWN_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}
fn spawn<'scope, 'env, F, T>(
    scope: &'scope thread::Scope<'scope, 'env>,
    f: F,
) -> std::io::Result<thread::ScopedJoinHandle<'scope, T>>
where
    F: FnOnce() -> T + Send + 'scope,
    T: Send + 'scope,
{
    #[cfg(test)]
    SPAWN_CALLS.with(|calls| calls.set(calls.get() + 1));
    #[cfg(test)]
    if SPAWN_FAILURE.with(|failure| match failure.get() {
        Some(0) => true,
        Some(n) => {
            failure.set(Some(n - 1));
            false
        }
        None => false,
    }) {
        return Err(std::io::Error::other("injected surface spawn failure"));
    }
    thread::Builder::new()
        .stack_size(STACK)
        .spawn_scoped(scope, f)
}

fn failed() -> Error {
    Error::ResourceLimit("surface parallel retry")
}
fn join_all<T>(
    handles: &mut [Option<thread::ScopedJoinHandle<'_, Result<T>>>; MAX_WORKERS],
) -> Result<()> {
    let mut valid = true;
    for handle in handles {
        if let Some(handle) = handle.take() {
            valid &= matches!(handle.join(), Ok(Ok(_)));
        }
    }
    if valid {
        Ok(())
    } else {
        Err(failed())
    }
}

/// Phase 1, the ring step for a drawn wood: each worker sweeps its runs'
/// rings, ranked as `ordered`, into its part of the positions and coords.
pub(super) fn rings(
    at: Swept,
    ordered: &[(usize, f64)],
    out: &mut Rings,
    sweep: Sweep,
    longest: usize,
    count: usize,
) -> Result<()> {
    if count < 2 {
        return Err(failed());
    }
    let vertices = |i: usize| at.rings(ordered[i].0) * out.segments + 2;
    let total = (0..ordered.len()).map(vertices).sum::<usize>();
    if sweep.edges {
        let mut base = 0;
        for (i, &(path_id, _)) in ordered.iter().enumerate() {
            at.record(&mut out.edges, path_id, base);
            base += vertices(i);
        }
    }
    let mut positions = Unfilled::new(total * 3)?;
    let mut coords = Unfilled::new(total * 2)?;
    thread::scope(|scope| -> Result<()> {
        let mut handles = std::array::from_fn(|_| None);
        let (mut p, mut c) = (positions.parts(), coords.parts());
        let mut spawn_ok = true;
        for (k, (group, _, n)) in groups(ordered.len(), vertices, total, count).enumerate() {
            let (mut out_p, mut out_c) = (p.take(n * 3)?, c.take(n * 2)?);
            let group = &ordered[group];
            let result = spawn(scope, move || -> Result<()> {
                let mut scratch = Scratch::new(longest)?;
                for &(path_id, _) in group {
                    let (samples, frame) = scratch.sweep(at, path_id);
                    let shape = at.section(path_id);
                    emit_run(samples, frame, at, shape, true, |xyz, coord| {
                        out_p.extend(&xyz);
                        out_c.extend(&coord);
                    })?;
                }
                Ok(())
            });
            match result {
                Ok(handle) => handles[k] = Some(handle),
                Err(_) => {
                    spawn_ok = false;
                    break;
                }
            }
        }
        let joined = join_all(&mut handles);
        if !spawn_ok {
            return Err(failed());
        }
        joined
    })?;
    out.positions = positions.filled()?;
    out.coords = coords.filled()?;
    out.workers = Some(count);
    Ok(())
}

/// Phase 2, the mesh step: each worker writes its runs' indices and normals
/// into its part. A dropped triangle moves every later run's indices, so it
/// fails the phase and the serial step takes the rings over.
pub(super) fn faces(rings: &Rings, resweep: &Resweep, count: usize) -> Result<Faces> {
    let total = rings.positions.len() / 3;
    let runs = rings.runs.len();
    let vertices = |i: usize| rings.vertices(&rings.runs[i]);
    // The most vertices a run has: the size of a worker's normals scratch.
    let widest = (0..runs).map(vertices).max().unwrap_or(0);
    let mut normals = Unfilled::new(total * 3)?;
    let mut indices = Unfilled::new((total - runs * 2) * 6)?;
    thread::scope(|scope| -> Result<()> {
        let mut handles = std::array::from_fn(|_| None);
        let (mut n, mut i) = (normals.parts(), indices.parts());
        let mut spawn_ok = true;
        for (k, (group, first, nv)) in groups(runs, vertices, total, count).enumerate() {
            let ni = (nv - group.len() * 2) * 6;
            let (mut out_n, mut out_i) = (n.take(nv * 3)?, i.take(ni)?);
            let result = spawn(scope, move || -> Result<()> {
                let mut scratch = reserved(widest * 3)?;
                let mut base = first;
                for run in group {
                    scratch.clear();
                    scratch.resize(vertices(run) * 3, 0.0);
                    let keep = |t: [u32; 3]| out_i.extend(&t);
                    if shade(rings, run, base, resweep, &mut scratch, keep)? != 0 {
                        return Err(failed());
                    }
                    out_n.extend(&scratch);
                    base += vertices(run);
                }
                Ok(())
            });
            match result {
                Ok(handle) => handles[k] = Some(handle),
                Err(_) => {
                    spawn_ok = false;
                    break;
                }
            }
        }
        let joined = join_all(&mut handles);
        if !spawn_ok {
            return Err(failed());
        }
        joined
    })?;
    Ok(Faces {
        normals: normals.filled()?,
        indices: indices.filled()?,
        run_table: None,
        bounds: Some(bounds(&rings.positions)),
        dropped: 0,
    })
}

mod unfilled;
use unfilled::Unfilled;
#[cfg(test)]
mod tests;
