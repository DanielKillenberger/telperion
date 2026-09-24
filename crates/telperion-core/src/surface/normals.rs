//! Vertex normals for one run, from the float32 corners the renderer draws.
//! A triangle whose corners span no area in float32 is dropped from the run
//! and counted; a vertex it leaves with no triangle, or whose triangles
//! cancel, takes the direction its ring faces. A run with nothing to drop is
//! shaded exactly as before: the same triangles, in the same order, summed
//! in the same precision.
use super::prepared::Run;
use crate::{math::Vec3, Error, Result};

fn corner(positions: &[f32], index: u32) -> Result<Vec3> {
    let offset = index as usize * 3;
    let p = positions
        .get(offset..offset + 3)
        .ok_or(Error::InvalidInput("surface index"))?;
    Ok(Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64))
}

/// Adds each triangle of `run` with area, in draw order, its float64 cross
/// product of its float32 corners into its three vertices, and hands it to
/// `keep`. `positions` and `normals` are the run's own, from its base.
/// Returns the triangles dropped.
pub(super) fn accumulate(
    positions: &[f32],
    run: &Run,
    segments: u32,
    normals: &mut [f32],
    mut keep: impl FnMut([u32; 3]),
) -> Result<usize> {
    let mut dropped = 0;
    run.visit_triangles(segments, |t| {
        let [a, b, c] = [
            corner(positions, t[0] - run.base)?,
            corner(positions, t[1] - run.base)?,
            corner(positions, t[2] - run.base)?,
        ];
        let normal = (c - b).cross(a - b);
        if !normal.is_finite() {
            return Err(Error::InvalidInput("surface float32 position overflow"));
        }
        if normal.length_squared() == 0.0 {
            dropped += 1;
            return Ok(());
        }
        for &index in &t {
            let at = (index - run.base) as usize * 3;
            let n = &mut normals[at..at + 3];
            for (k, v) in [normal.x, normal.y, normal.z].into_iter().enumerate() {
                n[k] = (n[k] as f64 + v) as f32;
            }
        }
        keep(t);
        Ok(())
    })?;
    Ok(dropped)
}

/// Unit normals for the run's vertices; one with nothing left to sum faces
/// the way its ring does.
pub(super) fn normalize(
    normals: &mut [f32],
    mut facing: impl FnMut(usize) -> Result<Vec3>,
) -> Result<()> {
    for (j, n) in normals.as_chunks_mut::<3>().0.iter_mut().enumerate() {
        let mut v = Vec3::new(n[0] as f64, n[1] as f64, n[2] as f64);
        if !v.is_finite() {
            return Err(Error::InvalidInput("surface normal overflow"));
        }
        if v.length_squared() == 0.0 {
            v = facing(j)?;
        }
        let v = v.normalized();
        n.copy_from_slice(&[v.x as f32, v.y as f32, v.z as f32]);
    }
    Ok(())
}
