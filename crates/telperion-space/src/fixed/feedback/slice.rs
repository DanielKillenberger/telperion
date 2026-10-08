//! The feedback slice (fn-211 task 8; Astra's review of CONTRACT-A.md,
//! section 4): canonical leaf positions and areas, their integer
//! deposits, the shared CPU sweep, integer sampling at buds, and D16's
//! threshold and kept share. The CPU reference of the exchange the GPU's
//! slice test holds word for word, with the deposits' order, chunking and
//! a forced buffer retry varied. No tree runs it yet.
use super::lattice::{Grid, AREA};
use super::shed::{balance, Balance, Bud};
use super::wide::prod_shr;
use super::Fault;
use crate::fixed::mul;
use crate::light::Light;

/// A leaf: its Q32 tip, and the Q32 factors of its area.
#[derive(Debug, Clone, Copy)]
pub struct Leaf {
    pub at: [i64; 3],
    pub scale: i64,
    pub size: i64,
    pub area: i64,
}

impl Leaf {
    /// `scale size area`, Q32, rounded once; `Range` outside `[0, AREA)`.
    pub fn area(&self) -> Result<i64, Fault> {
        let a = prod_shr([self.scale, self.size, self.area, 1], 64)?;
        if !(0..AREA).contains(&a) {
            return Err(Fault::Range);
        }
        Ok(a)
    }
}

/// A bud that reads the light: its Q32 tip and presence (`base_scale
/// running`), and its balance's other inputs (`q` is taken from the light).
#[derive(Debug, Clone, Copy)]
pub struct Read {
    pub at: [i64; 3],
    pub presence: i64,
    pub bud: Bud,
}

/// The slice's state after one cycle's exchange.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slice {
    pub grid: Grid,
    pub lattice: Vec<u128>,
    pub light: Vec<i64>,
    /// Each read's light and balance.
    pub reads: Vec<(i64, Balance)>,
}

/// The box every leaf and read lies in.
pub fn grid(leaves: &[Leaf], reads: &[Read]) -> Result<Grid, Fault> {
    Grid::spanning(
        leaves
            .iter()
            .map(|l| l.at)
            .chain(reads.iter().map(|r| r.at)),
    )
}

/// The leaves deposited in `order`, `chunk` at a time, each chunk into its
/// own partial lattice and the partials added last first.
pub fn deposited(
    grid: &Grid,
    leaves: &[Leaf],
    order: &[usize],
    chunk: usize,
) -> Result<Vec<u128>, Fault> {
    let mut parts = Vec::new();
    for c in order.chunks(chunk.max(1)) {
        let mine: Vec<_> = c
            .iter()
            .map(|&i| Ok((leaves[i].at, leaves[i].area()?)))
            .collect::<Result<_, Fault>>()?;
        parts.push(grid.deposit_all(&mine)?);
    }
    let mut lattice = vec![0u128; grid.nodes()];
    for part in parts.iter().rev() {
        for (l, p) in lattice.iter_mut().zip(part) {
            *l = l.wrapping_add(*p);
        }
    }
    Ok(lattice)
}

/// Each read's light from `light` and its balance.
pub fn read(grid: &Grid, light: &[i64], reads: &[Read]) -> Result<Vec<(i64, Balance)>, Fault> {
    reads
        .iter()
        .map(|r| {
            let l = grid.sample(light, r.at);
            let bud = Bud {
                q: mul(r.presence, l),
                ..r.bud
            };
            Ok((l, balance(&bud)?))
        })
        .collect()
}

/// One cycle's exchange: deposits in `order` and `chunk`s, the sweep, the
/// reads.
pub fn run(
    light: &Light,
    leaves: &[Leaf],
    reads: &[Read],
    order: &[usize],
    chunk: usize,
) -> Result<Slice, Fault> {
    let grid = grid(leaves, reads)?;
    let lattice = deposited(&grid, leaves, order, chunk)?;
    let swept = grid.swept(light, &lattice);
    let reads = read(&grid, &swept, reads)?;
    Ok(Slice {
        grid,
        lattice,
        light: swept,
        reads,
    })
}

/// A synthetic lit crown for the slice: `n` leaves in an ellipsoid shell
/// about 6 m up, and `m` buds among them, from SplitMix64 at `seed`.
pub fn crown(n: usize, m: usize, seed: u64) -> (Vec<Leaf>, Vec<Read>) {
    let mut s = seed;
    let mut next = move || {
        s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = s;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    };
    let q = crate::fixed::ONE32;
    let mut point = |r: i64| -> [i64; 3] {
        let mut c = || ((next() >> 20) as i64 % (2 * r)) - r;
        [c(), c(), 6 * q + c() / 2]
    };
    let leaves = (0..n)
        .map(|_| Leaf {
            at: point(4 * q),
            scale: q / 2 + (q / 3),
            size: q - q / 7,
            area: q / 50,
        })
        .collect();
    let reads = (0..m)
        .map(|k| Read {
            at: point(5 * q),
            presence: q - q / 9,
            bud: Bud {
                q: 0,
                c: q / 3 + (k as i64 % 7) * (q / 20),
                memory: q / 5 - (k as i64 % 11) * (q / 25),
                hazard: 2 * q,
                tolerance: q / 3,
                draw: (k as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 11,
                window: q / 2 + (k as i64 % 5) * q,
                keep: (q - (k as i64 % 3) * (q / 10)) as u64,
            },
        })
        .collect();
    (leaves, reads)
}
