use super::curve::Curve;
use super::rings::{written, Coords, Scratch};
use super::*;
use std::{mem::size_of, ops::Range, thread};
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
    let table = size_of::<WoodRun>() as u128;
    let output = vertices as u128 * 20 + indices as u128 * 2 + runs as u128 * table;
    let serial = output + prep as u128 + order as u128 + scratch as u128 + angular as u128;
    let descriptors = runs as u128 * table;
    let a = vertices as u128 * 16
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
    layout: &Layout,
) -> Option<usize> {
    let extent = layout.extent(at.segments());
    let vertices = extent.positions / 3;
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
            // A worker's sweep scratch, and a split run's points and normals
            // gathered whole in the mesh step.
            longest * (size_of::<Sample>() + size_of::<(Vec3, Vec3)>() + size_of::<Vec3>())
                + (longest * angular.len() + 2) * 6 * size_of::<f32>(),
            size_of_val(angular),
            vertices,
            extent.indices,
            layout.pieces.len(),
            count,
        ))
    .then_some(count)
}

/// Contiguous groups of whole chunks of about equal vertices, one a
/// worker, each cut where a chunk starts with a run's first piece so no
/// split run is shared: each group's pieces. Contiguous groups keep the
/// chunks in order.
fn groups(layout: &Layout, count: usize) -> impl Iterator<Item = Range<usize>> + '_ {
    let pieces = &layout.pieces;
    let total = layout.chunks.iter().sum::<usize>();
    // The pieces from `at` to the next chunk a run starts, and their vertices.
    let unit = move |at: usize| {
        let opens = |p: &Piece| p.base == 0 && p.lead;
        let next = pieces[at + 1..].iter().position(opens);
        let end = next.map_or(pieces.len(), |n| at + 1 + n);
        let chunks = pieces[at].chunk..pieces[end - 1].chunk + 1;
        (end, layout.chunks[chunks].iter().sum::<usize>())
    };
    let (mut at, mut done) = (0, 0);
    (1..=count).map(move |k| {
        let start = at;
        let target = if k == count {
            usize::MAX
        } else {
            total * k / count
        };
        while at < pieces.len() {
            let (end, vertices) = unit(at);
            if done + vertices > target {
                break;
            }
            (at, done) = (end, done + vertices);
        }
        start..at
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
/// Every worker's answer in order, each joined whatever another returned.
fn join_all<T>(
    handles: &mut [Option<thread::ScopedJoinHandle<'_, Result<T>>>; MAX_WORKERS],
) -> Result<Vec<T>> {
    let mut out = Vec::new();
    let mut valid = true;
    for handle in handles {
        if let Some(handle) = handle.take() {
            match handle.join() {
                Ok(Ok(value)) if valid => out.push(value),
                _ => valid = false,
            }
        }
    }
    if valid {
        Ok(out)
    } else {
        Err(failed())
    }
}

/// Each group of `layout` on a worker of its own, their answers in order.
fn on_workers<'a, T: Send>(
    layout: &'a Layout,
    count: usize,
    work: impl Fn(&'a [Piece]) -> Result<T> + Sync,
) -> Result<Vec<T>> {
    thread::scope(|scope| -> Result<Vec<T>> {
        let mut handles = std::array::from_fn(|_| None);
        let mut spawn_ok = true;
        for (k, group) in groups(layout, count).enumerate() {
            let pieces = &layout.pieces[group];
            let work = &work;
            match spawn(scope, move || work(pieces)) {
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
    })
}

/// Phase 1, the ring step for a drawn wood: each worker sweeps its chunks'
/// runs, ranked as `ordered`, or reads them from `curve`, into chunks of
/// its own.
pub(super) fn rings(
    at: Swept,
    curve: Option<&Curve>,
    ordered: &[(usize, f64)],
    out: &mut Rings,
    longest: usize,
    count: usize,
) -> Result<()> {
    if count < 2 {
        return Err(failed());
    }
    let (layout, along) = (&out.layout, out.along);
    let parts = on_workers(layout, count, |pieces| {
        let mut scratch = Scratch::new(longest)?;
        written(
            at,
            curve,
            ordered,
            layout,
            pieces,
            Coords::Bark(along),
            &mut scratch,
        )
    })?;
    let mut chunks = reserved(layout.chunks.len())?;
    chunks.extend(parts.into_iter().flatten());
    out.chunks = chunks;
    out.workers = Some(count);
    Ok(())
}

/// Phase 2, the mesh step: each worker writes its chunks' indices and
/// normals. A dropped triangle moves its piece's span, so it fails the
/// phase and the serial step takes the rings over.
pub(super) fn faces(rings: &Rings, resweep: &Resweep, count: usize) -> Result<Faces> {
    let parts = on_workers(&rings.layout, count, |pieces| {
        let mut spans = reserved(pieces.len())?;
        let (chunks, dropped) = shaded(rings, resweep, pieces, &mut spans)?;
        if dropped != 0 {
            return Err(failed());
        }
        Ok(chunks)
    })?;
    let mut chunks = reserved(rings.layout.chunks.len())?;
    chunks.extend(parts.into_iter().flatten());
    Ok(Faces {
        chunks,
        run_table: None,
        bounds: Some(rings.bounds()),
        dropped: 0,
    })
}

#[cfg(test)]
mod tests;
