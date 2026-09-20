//! Vertex normals for one run, from the float32 corners the renderer draws.
//! A triangle whose corners span no area in float32 is dropped from the run
//! and counted; a vertex it leaves with no triangle, or whose triangles
//! cancel, takes the direction its ring faces. A run with nothing to drop is
//! shaded exactly as before: the same triangles, in the same order, summed
//! in the same precision.
use crate::{math::Vec3, Error, Result};

/// Shades the run whose triangles start at index `first` and whose vertices
/// start at vertex `base`, the last run in the buffers. `facing` gives a
/// run vertex's own direction, counted from `base`. Returns the triangles
/// dropped.
pub(super) fn shade(
    positions: &[f32],
    indices: &mut Vec<u32>,
    normals: &mut [f32],
    (first, base): (usize, usize),
    facing: impl Fn(usize) -> Vec3,
) -> Result<usize> {
    let count = indices.len();
    let dropped = accumulate(positions, &mut indices[first..], normals, 0)?;
    indices.truncate(count - dropped * 3);
    normalize(&mut normals[base * 3..], facing)?;
    Ok(dropped)
}

fn corner(positions: &[f32], index: u32) -> Result<Vec3> {
    let offset = index as usize * 3;
    let p = positions
        .get(offset..offset + 3)
        .ok_or(Error::InvalidInput("surface index"))?;
    Ok(Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64))
}

/// Adds each triangle's float64 cross product of its float32 corners into its
/// three vertices, in index order, and closes the gaps the dropped ones leave.
pub(super) fn accumulate(
    positions: &[f32],
    indices: &mut [u32],
    normals: &mut [f32],
    base: u32,
) -> Result<usize> {
    let mut kept = 0;
    for read in (0..indices.len()).step_by(3) {
        let t = [indices[read], indices[read + 1], indices[read + 2]];
        let [a, b, c] = [
            corner(positions, t[0] - base)?,
            corner(positions, t[1] - base)?,
            corner(positions, t[2] - base)?,
        ];
        let normal = (c - b).cross(a - b);
        if !normal.is_finite() {
            return Err(Error::InvalidInput("surface float32 position overflow"));
        }
        if normal.length_squared() == 0.0 {
            continue;
        }
        for &index in &t {
            let n = &mut normals[(index - base) as usize * 3..(index - base) as usize * 3 + 3];
            for (k, v) in [normal.x, normal.y, normal.z].into_iter().enumerate() {
                n[k] = (n[k] as f64 + v) as f32;
            }
        }
        indices.copy_within(read..read + 3, kept);
        kept += 3;
    }
    let dropped = (indices.len() - kept) / 3;
    Ok(dropped)
}

/// Unit normals for the run's vertices; one with nothing left to sum faces
/// the way its ring does.
pub(super) fn normalize(normals: &mut [f32], facing: impl Fn(usize) -> Vec3) -> Result<()> {
    for (j, n) in normals.as_chunks_mut::<3>().0.iter_mut().enumerate() {
        let mut v = Vec3::new(n[0] as f64, n[1] as f64, n[2] as f64);
        if !v.is_finite() {
            return Err(Error::InvalidInput("surface normal overflow"));
        }
        if v.length_squared() == 0.0 {
            v = facing(j);
        }
        let v = v.normalized();
        n.copy_from_slice(&[v.x as f32, v.y as f32, v.z as f32]);
    }
    Ok(())
}
