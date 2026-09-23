use super::*;
use std::{mem::size_of, thread};
const MAX_WORKERS: usize = 8;
const STACK: usize = 64 * 1024;
const WORKER_ENVELOPE: usize = STACK + 4096 + 64 * 1024;
const CONTROL: usize = 16 * 1024;
#[derive(Clone, Copy)]
struct Run {
    path: u32,
    base: u32,
    rings: u32,
    first_index: u32,
    index_count: u32,
    radius: f64,
}
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
    let descriptors = runs as u128 * size_of::<Run>() as u128;
    let a = vertices as u128 * 20
        + prep as u128
        + angular as u128
        + descriptors
        + workers as u128 * (scratch + WORKER_ENVELOPE) as u128
        + CONTROL as u128;
    let b = output + descriptors + (2 * workers * WORKER_ENVELOPE + CONTROL) as u128;
    a <= serial && b <= serial
}
pub(super) fn admitted(
    paths: &paths::Paths,
    distance: &Vec<f64>,
    ordered: &Vec<(usize, f64)>,
    angular: &Vec<angular::Angular>,
    longest: usize,
    vertices: usize,
    indices: usize,
) -> Option<usize> {
    let count = workers();
    (count >= 2
        && vertices >= 250_000
        && paths.runs.len() >= count
        && envelope(
            paths.nodes.capacity() * size_of::<usize>()
                + paths.runs.capacity() * size_of::<paths::Run>()
                + paths.forks.capacity()
                + distance.capacity() * 8,
            ordered.capacity() * size_of::<(usize, f64)>(),
            longest * (size_of::<Sample>() + size_of::<(Vec3, Vec3)>() + size_of::<Vec3>()),
            angular.capacity() * size_of::<angular::Angular>(),
            vertices,
            indices,
            paths.runs.len(),
            count,
        ))
    .then_some(count)
}

// Contiguous run groups keep final buffers disjoint and preserve their order.
fn partitions(
    runs: &[Run],
    vertices: usize,
    segments: usize,
    count: usize,
) -> [usize; MAX_WORKERS + 1] {
    let mut out = [runs.len(); MAX_WORKERS + 1];
    out[0] = 0;
    let mut at = 0;
    for (k, boundary) in out.iter_mut().enumerate().take(count).skip(1) {
        let target = vertices * k / count;
        while at < runs.len()
            && (runs[at].base as usize + runs[at].rings as usize * segments + 2) <= target
        {
            at += 1;
        }
        *boundary = at;
    }
    out
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

#[cfg(test)]
pub(super) fn build(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    paths: paths::Paths,
    distance: Vec<f64>,
    ordered: Vec<(usize, f64)>,
    angular: Vec<angular::Angular>,
    longest: usize,
    vertices: usize,
    indices_len: usize,
    count: usize,
) -> Result<SurfaceMesh> {
    let sizes = (longest, vertices, indices_len, count);
    build_with(
        tree, height, params, paths, distance, ordered, angular, sizes, None,
    )
}

/// The parallel build; each run's vertices come from a shared sweep's rings
/// where one ran, and from the run's own sweep where none did.
#[allow(clippy::too_many_arguments)]
pub(super) fn build_with(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    paths: paths::Paths,
    distance: Vec<f64>,
    ordered: Vec<(usize, f64)>,
    angular: Vec<angular::Angular>,
    (longest, vertices, indices_len, count): (usize, usize, usize, usize),
    swept: Option<Swept>,
) -> Result<SurfaceMesh> {
    if count < 2 {
        return Err(failed());
    }
    let segments = angular.len();
    let mut runs = reserved(ordered.len())?;
    let (mut base, mut first_index) = (0usize, 0usize);
    for (path_id, radius) in ordered {
        let path = &paths.runs[path_id];
        let rings = path.end - path.start + usize::from(path.trunk && params.flare_depth > 0.0);
        let index_count = rings * segments * 6;
        runs.push(Run {
            path: u32::try_from(path_id).map_err(|_| failed())?,
            base: base as u32,
            rings: rings as u32,
            first_index: u32::try_from(first_index).map_err(|_| failed())?,
            index_count: u32::try_from(index_count).map_err(|_| failed())?,
            radius,
        });
        base += rings * segments + 2;
        first_index += index_count;
    }
    if base != vertices || first_index != indices_len || first_index > u32::MAX as usize {
        return Err(failed());
    }
    let boundaries = partitions(&runs, vertices, segments, count);
    let mut positions = filled(vertices * 3, 0.0)?;
    let mut coords = filled(vertices * 2, 0.0)?;
    thread::scope(|scope| -> Result<()> {
        let mut handles = std::array::from_fn(|_| None);
        let mut p = positions.as_mut_slice();
        let mut c = coords.as_mut_slice();
        let mut spawn_ok = true;
        for k in 0..count {
            let group = &runs[boundaries[k]..boundaries[k + 1]];
            let n = group
                .iter()
                .map(|r| r.rings as usize * segments + 2)
                .sum::<usize>();
            let (out_p, rest) = p.split_at_mut(n * 3);
            p = rest;
            let (out_c, rest) = c.split_at_mut(n * 2);
            c = rest;
            let paths = &paths;
            let distance = &distance;
            let angular = &angular;
            let result = spawn(scope, move || -> Result<()> {
                let mut samples = reserved(longest)?;
                let mut frame = reserved(longest)?;
                let mut scratch = reserved(longest)?;
                let mut offset = 0;
                for run in group {
                    sample_path(
                        tree,
                        height,
                        params,
                        paths,
                        &paths.runs[run.path as usize],
                        distance,
                        &mut samples,
                    );
                    let emit = |xyz: [f32; 3], coord: [f32; 2]| {
                        out_p[offset * 3..offset * 3 + 3].copy_from_slice(&xyz);
                        out_c[offset * 2..offset * 2 + 2].copy_from_slice(&coord);
                        offset += 1;
                    };
                    if let Some(sweep) = swept {
                        let rings = sweep.run_rings(run.path as usize, samples.len());
                        emit_swept(rings, &samples, angular, emit)?;
                        continue;
                    }
                    frames(&samples, &mut scratch, &mut frame);
                    emit_run(&samples, &frame, angular, params, height, emit)?;
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
    drop((paths, distance, angular));
    let mut normals = filled(vertices * 3, 0.0)?;
    let mut indices = filled(indices_len, 0)?;
    thread::scope(|scope| -> Result<()> {
        let mut handles = std::array::from_fn(|_| None);
        let mut p = positions.as_slice();
        let mut n = normals.as_mut_slice();
        let mut i = indices.as_mut_slice();
        let mut spawn_ok = true;
        for k in 0..count {
            let group = &runs[boundaries[k]..boundaries[k + 1]];
            let nv = group
                .iter()
                .map(|r| r.rings as usize * segments + 2)
                .sum::<usize>();
            let ni = group.iter().map(|r| r.index_count as usize).sum::<usize>();
            let (out_p, rest) = p.split_at(nv * 3);
            p = rest;
            let (out_n, rest) = n.split_at_mut(nv * 3);
            n = rest;
            let (out_i, rest) = i.split_at_mut(ni);
            i = rest;
            let result = spawn(scope, move || -> Result<()> {
                let (mut vp, mut ip) = (0, 0);
                for run in group {
                    let vertex_count = (run.rings as usize * segments + 2) * 3;
                    let pos = &out_p[vp..vp + vertex_count];
                    let normal = &mut out_n[vp..vp + vertex_count];
                    let index = &mut out_i[ip..ip + run.index_count as usize];
                    let desc = prepared::Run {
                        base: run.base,
                        first_index: run.first_index,
                        ring_start: 0,
                        rings: run.rings,
                        index_count: run.index_count,
                    };
                    let mut at = 0;
                    desc.visit_triangles(segments as u32, |triangle| {
                        index[at..at + 3].copy_from_slice(&triangle);
                        at += 3;
                        Ok(())
                    })?;
                    if normals::accumulate(pos, index, normal, run.base)? != 0
                        || normal.chunks_exact(3).any(|v| v.iter().all(|&x| x == 0.0))
                    {
                        return Err(failed());
                    }
                    normals::normalize(normal, |_| unreachable!())?;
                    vp += vertex_count;
                    ip += run.index_count as usize;
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
    let mut run_table = reserved(runs.len())?;
    for run in &runs {
        run_table.push(SurfaceRun {
            first_index: run.first_index,
            index_count: run.index_count,
            largest_radius: run.radius,
        });
    }
    finish(
        SurfaceMesh {
            positions,
            coords,
            normals,
            indices,
            bounds: None,
            runs: runs.len(),
            run_table,
            dropped: 0,
        },
        segments,
    )
}

#[cfg(test)]
mod tests;
