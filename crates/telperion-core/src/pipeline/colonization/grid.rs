use crate::math::Vec3;

/// Stable bins over the fixed cloud, at most 33 cells per axis.
pub(super) struct AttractorGrid {
    min: Vec3,
    size: f64,
    dimensions: [usize; 3],
    offsets: Vec<usize>,
    items: Vec<usize>,
}
impl AttractorGrid {
    pub fn new(points: &[Vec3], reach: f64) -> Self {
        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for p in points {
            min = Vec3::new(min.x.min(p.x), min.y.min(p.y), min.z.min(p.z));
            max = Vec3::new(max.x.max(p.x), max.y.max(p.y), max.z.max(p.z));
        }
        let span = max - min;
        let extent = span.x.max(span.y).max(span.z);
        let size = if reach > 0.0 && extent.is_finite() {
            reach.max(extent / 32.0)
        } else {
            f64::INFINITY
        };
        let dimensions = [span.x, span.y, span.z].map(|s| {
            if size.is_finite() && s.is_finite() {
                (s / size).floor() as usize + 1
            } else {
                1
            }
        });
        let cells = dimensions.iter().product::<usize>();
        let mut grid = Self {
            min,
            size,
            dimensions,
            offsets: vec![0; cells + 1],
            items: vec![0; points.len()],
        };
        for p in points {
            let cell = grid.index(grid.cell(*p));
            grid.offsets[cell + 1] += 1;
        }
        for c in 0..cells {
            grid.offsets[c + 1] += grid.offsets[c];
        }
        let mut fill = grid.offsets.clone();
        for (a, p) in points.iter().enumerate() {
            let c = grid.index(grid.cell(*p));
            grid.items[fill[c]] = a;
            fill[c] += 1;
        }
        grid
    }
    fn cell(&self, p: Vec3) -> [usize; 3] {
        let delta = p - self.min;
        let mut out = [0; 3];
        for (axis, value) in [delta.x, delta.y, delta.z].iter().enumerate() {
            out[axis] =
                ((*value / self.size).floor().max(0.0) as usize).min(self.dimensions[axis] - 1);
        }
        out
    }
    fn index(&self, [x, y, z]: [usize; 3]) -> usize {
        (x * self.dimensions[1] + y) * self.dimensions[2] + z
    }
    pub fn visit(&self, p: Vec3, mut visitor: impl FnMut(usize)) {
        let [x, y, z] = self.cell(p);
        for cx in x.saturating_sub(1)..=(x + 1).min(self.dimensions[0] - 1) {
            for cy in y.saturating_sub(1)..=(y + 1).min(self.dimensions[1] - 1) {
                for cz in z.saturating_sub(1)..=(z + 1).min(self.dimensions[2] - 1) {
                    let c = self.index([cx, cy, cz]);
                    for &a in &self.items[self.offsets[c]..self.offsets[c + 1]] {
                        visitor(a);
                    }
                }
            }
        }
    }
}
