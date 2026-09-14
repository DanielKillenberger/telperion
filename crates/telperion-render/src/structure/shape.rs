//! The three shape statistics, on the thresholded crop: how evenly the light
//! regions are sized, how far the dark boundaries wander, and how many arms
//! meet where the dark network branches. Nothing here knows what a tree is;
//! they are the same measurements on a photograph of a wall.
use super::CROP;

/// How far apart the light regions' areas are, as a coefficient of variation.
/// Regions touching the crop's edge are left out: a plate the crop cut in half
/// has an area the crop chose. Regions under 32 pixels are noise at this
/// threshold and are left out too.
pub fn area_variation(light: &[bool]) -> f64 {
    let areas: Vec<f64> = regions(light, true)
        .into_iter()
        .filter(|region| !region.on_edge && region.pixels.len() >= 32)
        .map(|region| region.pixels.len() as f64)
        .collect();
    if areas.len() < 3 {
        return 0.0;
    }
    let mean = areas.iter().sum::<f64>() / areas.len() as f64;
    let variance = areas.iter().map(|a| (a - mean).powi(2)).sum::<f64>() / areas.len() as f64;
    variance.sqrt() / mean.max(1e-6)
}

/// How far the boundary between a plate and its furrow wanders over a fixed
/// run of it. Each light region's outline is walked once - that outline is
/// the furrow wall, seen from the plate's side, and unlike the dark network's
/// own outer contour it is a closed curve per plate rather than the frame of
/// the crop. Every window of `SEGMENT` steps contributes the ratio of the
/// distance walked to the distance made good. A course laid by a mason
/// returns 1; a furrow that bends returns more.
pub fn curvature(light: &[bool]) -> f64 {
    const SEGMENT: usize = 24;
    let mut ratios = Vec::new();
    for region in regions(light, true) {
        if region.on_edge || region.pixels.len() < 64 {
            continue;
        }
        let outline = outline(&region, light);
        if outline.len() <= SEGMENT {
            continue;
        }
        for start in (0..outline.len()).step_by(SEGMENT / 2) {
            let mut walked = 0.0;
            for step in 0..SEGMENT {
                let a = outline[(start + step) % outline.len()];
                let b = outline[(start + step + 1) % outline.len()];
                walked += distance(a, b);
            }
            let chord = distance(outline[start], outline[(start + SEGMENT) % outline.len()]);
            // A boundary that returns to where it started in one segment has
            // turned as far as this measurement can see; it is not infinitely
            // curved, so the ratio is capped rather than divided by nothing.
            ratios.push((walked / chord.max(1.0)).min(6.0));
        }
    }
    if ratios.is_empty() {
        return 1.0;
    }
    ratios.iter().sum::<f64>() / ratios.len() as f64
}

/// How many arms meet where the dark network branches. The dark regions are
/// thinned to one-pixel lines, the stubs that thinning leaves on a ragged
/// edge are pruned, and every place where three or more lines meet is counted
/// by its arms. A lattice crosses four ways; bark, which branches and merges
/// the way a river network does, meets three.
///
/// Thinning splits a true four-way crossing into two three-way junctions a
/// pixel apart - that is a property of the algorithm, not of the picture - so
/// junctions within a pixel of each other are one node and are counted once,
/// by the number of lines leaving the node as a whole.
pub fn junction_arms(dark: &[bool]) -> f64 {
    let skeleton = prune(thin(dark), 6);
    let branch: Vec<bool> = (0..CROP * CROP)
        .map(|at| {
            let (x, y) = (at % CROP, at / CROP);
            (2..CROP - 2).contains(&x)
                && (2..CROP - 2).contains(&y)
                && skeleton[at]
                && crossings(&skeleton, x, y) >= 3
        })
        .collect();
    let node: Vec<bool> = (0..CROP * CROP)
        .map(|at| {
            let (x, y) = (at % CROP, at / CROP);
            skeleton[at]
                && (1..CROP - 1).contains(&x)
                && (1..CROP - 1).contains(&y)
                && RING.iter().chain(&[(0, 0)]).any(|(dx, dy)| {
                    branch[(y as isize + dy) as usize * CROP + (x as isize + dx) as usize]
                })
        })
        .collect();
    let mut arms = Vec::new();
    for cluster in regions(&node, true) {
        let inside: std::collections::HashSet<(usize, usize)> =
            cluster.pixels.iter().copied().collect();
        // The lines leaving this node: skeleton pixels beside it that are not
        // part of it, grouped so that one line touching the node twice counts
        // once.
        let mut leaving: Vec<(usize, usize)> = Vec::new();
        for &(x, y) in &cluster.pixels {
            for (dx, dy) in RING {
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if nx < 1 || ny < 1 || nx >= CROP as isize - 1 || ny >= CROP as isize - 1 {
                    continue;
                }
                let next = (nx as usize, ny as usize);
                if skeleton[next.1 * CROP + next.0] && !inside.contains(&next) {
                    leaving.push(next);
                }
            }
        }
        leaving.sort_unstable();
        leaving.dedup();
        let mut mask = vec![false; CROP * CROP];
        for &(x, y) in &leaving {
            mask[y * CROP + x] = true;
        }
        let count = leaving
            .iter()
            .filter(|&&(x, y)| {
                // One representative per connected group of departures.
                mask[y * CROP + x] && flood(&mut mask, x, y)
            })
            .count();
        if count >= 3 {
            arms.push(count as f64);
        }
    }
    if arms.is_empty() {
        return 0.0;
    }
    arms.iter().sum::<f64>() / arms.len() as f64
}

/// Clear the group this pixel belongs to and say that it was there: the
/// caller counts one arm per group rather than one per pixel.
fn flood(mask: &mut [bool], x: usize, y: usize) -> bool {
    let mut stack = vec![(x, y)];
    mask[y * CROP + x] = false;
    while let Some((x, y)) = stack.pop() {
        for (dx, dy) in RING {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx < 0 || ny < 0 || nx >= CROP as isize || ny >= CROP as isize {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if mask[ny * CROP + nx] {
                mask[ny * CROP + nx] = false;
                stack.push((nx, ny));
            }
        }
    }
    true
}

/// Thinning leaves a stub wherever the dark region had a bump on its edge,
/// and every stub is a three-way junction that no furrow made. Shortening
/// each line's free end by a fixed number of pixels removes them without
/// touching a junction, because a junction has no free end.
fn prune(mut skeleton: Vec<bool>, steps: usize) -> Vec<bool> {
    for _ in 0..steps {
        let ends: Vec<usize> = (0..CROP * CROP)
            .filter(|&at| {
                let (x, y) = (at % CROP, at / CROP);
                (1..CROP - 1).contains(&x)
                    && (1..CROP - 1).contains(&y)
                    && skeleton[at]
                    && crossings(&skeleton, x, y) <= 1
            })
            .collect();
        for at in ends {
            skeleton[at] = false;
        }
    }
    skeleton
}

struct Region {
    pixels: Vec<(usize, usize)>,
    on_edge: bool,
}

/// Connected regions of the given value. Light regions are taken eight-
/// connected and dark ones four-connected, the usual pairing: taking both
/// eight-connected would let a plate and its neighbour touch through the
/// corner of the furrow that separates them.
fn regions(mask: &[bool], eight: bool) -> Vec<Region> {
    let mut seen = vec![false; CROP * CROP];
    let mut found = Vec::new();
    for start in 0..CROP * CROP {
        if seen[start] || !mask[start] {
            continue;
        }
        let mut pixels = Vec::new();
        let mut on_edge = false;
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(at) = stack.pop() {
            let (x, y) = (at % CROP, at / CROP);
            on_edge |= x == 0 || y == 0 || x == CROP - 1 || y == CROP - 1;
            pixels.push((x, y));
            for (dx, dy) in NEIGHBOURS.iter().take(if eight { 8 } else { 4 }) {
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if nx < 0 || ny < 0 || nx >= CROP as isize || ny >= CROP as isize {
                    continue;
                }
                let next = ny as usize * CROP + nx as usize;
                if mask[next] && !seen[next] {
                    seen[next] = true;
                    stack.push(next);
                }
            }
        }
        found.push(Region { pixels, on_edge });
    }
    found
}

/// The four edge neighbours first, so a four-connected walk is the same list
/// cut short. The eight-connected order below is the ring, clockwise from due
/// east, which is what the outline walk and the crossing number both need.
const NEIGHBOURS: [(isize, isize); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
    (1, -1),
];
const RING: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

/// The region's outline, traced once clockwise by the Moore neighbourhood
/// from its topmost-leftmost pixel and stopped when that pixel is entered
/// again from the same direction.
fn outline(region: &Region, mask: &[bool]) -> Vec<(usize, usize)> {
    let start = *region
        .pixels
        .iter()
        .min_by_key(|&&(x, y)| (y, x))
        .expect("a region has pixels");
    let filled = |x: isize, y: isize| {
        x >= 0
            && y >= 0
            && x < CROP as isize
            && y < CROP as isize
            && mask[y as usize * CROP + x as usize]
    };
    let mut path = vec![start];
    let mut at = start;
    let mut entry = 4usize;
    loop {
        let mut stepped = false;
        for turn in 1..=8 {
            let direction = (entry + turn) % 8;
            let (dx, dy) = RING[direction];
            let (nx, ny) = (at.0 as isize + dx, at.1 as isize + dy);
            if filled(nx, ny) {
                at = (nx as usize, ny as usize);
                entry = (direction + 4) % 8;
                stepped = true;
                break;
            }
        }
        if !stepped || (at == start && path.len() > 1) || path.len() > 8 * CROP * CROP {
            break;
        }
        path.push(at);
    }
    path
}

fn distance(a: (usize, usize), b: (usize, usize)) -> f64 {
    let dx = a.0 as f64 - b.0 as f64;
    let dy = a.1 as f64 - b.1 as f64;
    dx.hypot(dy)
}

/// The number of times the eight-neighbour ring changes from empty to filled:
/// one for a line's end, two for a point along it, three or more for a
/// junction, and that count is the number of arms leaving it.
fn crossings(mask: &[bool], x: usize, y: usize) -> usize {
    let ring: Vec<bool> = RING
        .iter()
        .map(|(dx, dy)| {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            mask[ny as usize * CROP + nx as usize]
        })
        .collect();
    (0..8).filter(|&i| !ring[i] && ring[(i + 1) % 8]).count()
}

/// Zhang-Suen thinning: the standard two-subiteration reduction of a region
/// to the one-pixel line down its middle, which is what makes a junction
/// countable at all. It preserves connectivity and stops when a pass removes
/// nothing.
fn thin(mask: &[bool]) -> Vec<bool> {
    let mut image = mask.to_vec();
    loop {
        let mut removed = false;
        for pass in 0..2 {
            let mut doomed = Vec::new();
            for y in 1..CROP - 1 {
                for x in 1..CROP - 1 {
                    if !image[y * CROP + x] {
                        continue;
                    }
                    // The ring clockwise from due north, as Zhang and Suen
                    // number it: p2 north, p3 north-east, and round.
                    let p: Vec<bool> = [
                        (0, -1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                        (0, 1),
                        (-1, 1),
                        (-1, 0),
                        (-1, -1),
                    ]
                    .iter()
                    .map(|(dx, dy)| {
                        image[(y as isize + dy) as usize * CROP + (x as isize + dx) as usize]
                    })
                    .collect();
                    let filled = p.iter().filter(|&&v| v).count();
                    let transitions = (0..8).filter(|&i| !p[i] && p[(i + 1) % 8]).count();
                    if !(2..=6).contains(&filled) || transitions != 1 {
                        continue;
                    }
                    let (a, b) = if pass == 0 {
                        ((p[0], p[2], p[4]), (p[2], p[4], p[6]))
                    } else {
                        ((p[0], p[2], p[6]), (p[0], p[4], p[6]))
                    };
                    if (a.0 && a.1 && a.2) || (b.0 && b.1 && b.2) {
                        continue;
                    }
                    doomed.push(y * CROP + x);
                }
            }
            for at in &doomed {
                image[*at] = false;
            }
            removed |= !doomed.is_empty();
        }
        if !removed {
            return image;
        }
    }
}
