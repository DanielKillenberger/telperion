//! Space as the resource (Pałubicki et al. 2009, space colonization), with
//! ontogeny: every marker carries the crown scale s at which it joins the
//! crown, so the crown grows from a seedling's to the mature envelope and
//! the lower crown falls behind as the tree rises.
//!
//! Each grid cell keeps its slots as [live | dead | waiting]; waiting
//! markers are sorted by the scale at which they join. A marker that dies is
//! swapped to the end of the live run; a marker that joins is swapped to its
//! start. Wood is remembered on a voxel grid so a joining marker inside wood
//! dies at once.
use telperion_core::math::Vec3;

pub struct Space {
    ids: Vec<u32>,
    xyz: Vec<[f64; 3]>,
    pts: Vec<Vec3>,
    join: Vec<f32>,
    start: Vec<u32>,
    live_len: Vec<u32>,
    wait: Vec<u32>,
    lo: Vec3,
    cell: f64,
    n: [usize; 3],
    wood: std::collections::HashSet<(i32, i32, i32)>,
    wood_cell: f64,
    pub live: usize,
    pub placed: usize,
    pub visits: u64,
    threads: usize,
    best: Vec<f64>,
    owner: Vec<u32>,
    touched: Vec<u32>,
}

fn rng(s: &mut u64) -> f64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    (*s >> 11) as f64 / (1u64 << 53) as f64
}

/// Runs `f` over `n` items in `threads` contiguous chunks; results in order.
pub fn chunks<T: Send>(n: usize, threads: usize, f: impl Fn(std::ops::Range<usize>) -> T + Sync) -> Vec<T> {
    let t = threads.max(1).min(n.max(1));
    if t == 1 {
        return vec![f(0..n)];
    }
    let size = n.div_ceil(t);
    std::thread::scope(|s| {
        let hs: Vec<_> = (0..t)
            .map(|k| {
                let f = &f;
                s.spawn(move || f((k * size).min(n)..((k + 1) * size).min(n)))
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap()).collect()
    })
}

impl Space {
    /// `count` uniform draws over the box; `place(draw, index)` gives the
    /// marker's position and the crown scale at which it joins, `None` when
    /// the draw is not in the crown.
    #[allow(clippy::too_many_arguments)]
    pub fn fill(
        lo: Vec3,
        hi: Vec3,
        count: usize,
        cell: f64,
        wood_cell: f64,
        seed: u64,
        threads: usize,
        place: &(dyn Fn(Vec3, u64) -> Option<(Vec3, f64)> + Sync),
    ) -> Self {
        let size = hi - lo;
        let n = [
            (size.x / cell).ceil() as usize + 1,
            (size.y / cell).ceil() as usize + 1,
            (size.z / cell).ceil() as usize + 1,
        ];
        let mut st = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        let draws: Vec<Vec3> = (0..count)
            .map(|_| lo + Vec3::new(size.x * rng(&mut st), size.y * rng(&mut st), size.z * rng(&mut st)))
            .collect();
        let placed: Vec<Option<(Vec3, f64)>> =
            chunks(count, threads, |r| r.clone().map(|m| place(draws[m], m as u64)).collect::<Vec<_>>()).into_iter().flatten().collect();
        let mut pts = vec![];
        let mut js = vec![];
        for (p, j) in placed.into_iter().flatten() {
            pts.push(p);
            js.push(j as f32);
        }
        let mut s = Self {
            ids: vec![],
            xyz: vec![],
            pts,
            join: js,
            start: vec![],
            live_len: vec![],
            wait: vec![],
            lo,
            cell,
            n,
            wood: Default::default(),
            wood_cell,
            live: 0,
            placed: 0,
            visits: 0,
            threads,
            best: vec![],
            owner: vec![],
            touched: vec![],
        };
        let cells = n[0] * n[1] * n[2];
        let of: Vec<usize> = s.pts.iter().map(|&p| s.cell_of(p).unwrap()).collect();
        let mut per: Vec<Vec<u32>> = vec![vec![]; cells];
        for (m, &c) in of.iter().enumerate() {
            per[c].push(m as u32);
        }
        s.start = vec![0; cells + 1];
        for c in 0..cells {
            s.start[c + 1] = s.start[c] + per[c].len() as u32;
        }
        s.live_len = vec![0; cells];
        s.wait = s.start[..cells].to_vec();
        for (c, ms) in per.iter_mut().enumerate() {
            ms.sort_by(|&a, &b| s.join[a as usize].total_cmp(&s.join[b as usize]).then(a.cmp(&b)));
            for &m in ms.iter() {
                s.ids.push(m);
                let p = s.pts[m as usize];
                s.xyz.push([p.x, p.y, p.z]);
            }
            debug_assert_eq!(s.ids.len(), s.start[c + 1] as usize);
        }
        s.placed = s.pts.len();
        s.best = vec![f64::INFINITY; s.pts.len()];
        s.owner = vec![u32::MAX; s.pts.len()];
        s
    }

    fn coords(&self, p: Vec3) -> [isize; 3] {
        let c = |v: f64, o: f64| ((v - o) / self.cell).floor() as isize;
        [c(p.x, self.lo.x), c(p.y, self.lo.y), c(p.z, self.lo.z)]
    }

    fn cell_of(&self, p: Vec3) -> Option<usize> {
        let [i, j, k] = self.coords(p);
        let ok = |v: isize, n: usize| v >= 0 && (v as usize) < n;
        (ok(i, self.n[0]) && ok(j, self.n[1]) && ok(k, self.n[2])).then(|| (j as usize * self.n[2] + k as usize) * self.n[0] + i as usize)
    }

    fn wood_key(&self, p: Vec3) -> (i32, i32, i32) {
        let c = |v: f64| (v / self.wood_cell).floor() as i32;
        (c(p.x), c(p.y), c(p.z))
    }

    /// Markers whose crown scale is reached join the live set, unless wood
    /// already stands in their voxel.
    pub fn grow_to(&mut self, scale: f64) {
        for c in 0..self.live_len.len() {
            let end = self.start[c + 1];
            while self.wait[c] < end {
                let w = self.wait[c] as usize;
                let m = self.ids[w] as usize;
                if f64::from(self.join[m]) > scale {
                    break;
                }
                self.wait[c] += 1;
                let [x, y, z] = self.xyz[w];
                if self.wood.contains(&self.wood_key(Vec3::new(x, y, z))) {
                    continue;
                }
                let at = (self.start[c] + self.live_len[c]) as usize;
                self.ids.swap(at, w);
                self.xyz.swap(at, w);
                self.live_len[c] += 1;
                self.live += 1;
            }
        }
    }

    /// The cells within `r` of `p`, in a fixed order.
    fn cells_near(&self, p: Vec3, r: f64, mut f: impl FnMut(usize)) {
        let range = |v: f64, o: f64, n: usize| {
            let a = ((v - r - o) / self.cell).floor().max(0.0) as usize;
            let b = ((v + r - o) / self.cell).floor();
            (a, if b < 0.0 { None } else { Some((b as usize).min(n - 1)) })
        };
        let (i0, i1) = range(p.x, self.lo.x, self.n[0]);
        let (j0, j1) = range(p.y, self.lo.y, self.n[1]);
        let (k0, k1) = range(p.z, self.lo.z, self.n[2]);
        let (Some(i1), Some(j1), Some(k1)) = (i1, j1, k1) else { return };
        for j in j0..=j1 {
            for k in k0..=k1 {
                for i in i0..=i1 {
                    f((j * self.n[2] + k) * self.n[0] + i);
                }
            }
        }
    }

    /// Removes the live markers within `rho` of new wood and remembers it.
    pub fn occupy(&mut self, p: Vec3, rho: f64) {
        self.wood.insert(self.wood_key(p));
        let mut cells = [0usize; 64];
        let mut nc = 0;
        self.cells_near(p, rho, |c| {
            if nc < 64 {
                cells[nc] = c;
                nc += 1;
            }
        });
        for &c in &cells[..nc] {
            let s0 = self.start[c] as usize;
            let mut x = 0usize;
            while x < self.live_len[c] as usize {
                let [qx, qy, qz] = self.xyz[s0 + x];
                if (Vec3::new(qx, qy, qz) - p).length() <= rho {
                    let last = s0 + self.live_len[c] as usize - 1;
                    self.ids.swap(s0 + x, last);
                    self.xyz.swap(s0 + x, last);
                    self.live_len[c] -= 1;
                    self.live -= 1;
                } else {
                    x += 1;
                }
            }
        }
    }

    fn scan(&self, p: Vec3, r: f64, mut f: impl FnMut(u32, Vec3)) -> u64 {
        let mut v = 0;
        self.cells_near(p, r, |c| {
            let s0 = self.start[c] as usize;
            for x in s0..s0 + self.live_len[c] as usize {
                v += 1;
                let [qx, qy, qz] = self.xyz[x];
                let q = Vec3::new(qx, qy, qz);
                if (q - p).length_squared() <= r * r {
                    f(self.ids[x], q);
                }
            }
        });
        v
    }

    /// Whether any live marker stands in the cone of `dir` within `r`.
    pub fn sees(&self, p: Vec3, dir: Vec3, r: f64, cos: f64) -> bool {
        let mut any = false;
        self.scan(p, r, |_, q| {
            let d = q - p;
            if !any && d.dot(dir) >= cos * d.length() {
                any = true;
            }
        });
        any
    }

    /// Associates every marker with the closest bud whose cone holds it and
    /// returns each bud's (markers won, normalised direction to them, sees
    /// any). Scanned in parallel, merged in bud order.
    pub fn perceive(&mut self, buds: &[(Vec3, Vec3)], r: f64, cos: f64) -> Vec<(u32, Vec3, bool)> {
        let found = chunks(buds.len(), self.threads.min(buds.len() / 256).max(1), |range| {
            let (mut hits, mut ends, mut sees, mut v) = (vec![], vec![], vec![], 0u64);
            for &(p, dir) in &buds[range] {
                let mut any = false;
                v += self.scan(p, r, |m, q| {
                    let d = q - p;
                    let len = d.length();
                    if d.dot(dir) >= cos * len {
                        any = true;
                        hits.push((m, len));
                    }
                });
                ends.push(hits.len());
                sees.push(any);
            }
            (hits, ends, sees, v)
        });
        let mut seen = Vec::with_capacity(buds.len());
        let mut b = 0usize;
        for (hits, ends, sees, v) in found {
            self.visits += v;
            let mut from = 0;
            for (e, s) in ends.into_iter().zip(sees) {
                for &(m, len) in &hits[from..e] {
                    if len < self.best[m as usize] {
                        if self.owner[m as usize] == u32::MAX {
                            self.touched.push(m);
                        }
                        self.best[m as usize] = len;
                        self.owner[m as usize] = b as u32;
                    }
                }
                from = e;
                seen.push(s);
                b += 1;
            }
        }
        self.touched.sort_unstable();
        let mut out = vec![(0u32, Vec3::ZERO); buds.len()];
        for &m in &self.touched {
            let b = self.owner[m as usize] as usize;
            let d = (self.pts[m as usize] - buds[b].0).normalized();
            out[b] = (out[b].0 + 1, out[b].1 + d);
            self.best[m as usize] = f64::INFINITY;
            self.owner[m as usize] = u32::MAX;
        }
        self.touched.clear();
        out.iter().zip(seen).map(|(o, s)| (o.0, if o.0 > 0 { o.1.normalized() } else { o.1 }, s)).collect()
    }
}
