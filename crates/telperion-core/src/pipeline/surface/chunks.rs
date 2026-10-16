//! The wood's chunks: the runs in table order, each chunk filled until the
//! next run does not fit, so every index fits sixteen bits and no buffer
//! needs one long free run of memory. A run longer than a chunk is split at
//! ring boundaries into pieces that repeat `OVERLAP` rings, so every ring
//! pair a piece draws, and the pair either side of it a contact query reads,
//! lies in one chunk.
use super::*;
use std::ops::Range;

/// Rings two neighbouring pieces of a split run both hold.
const OVERLAP: usize = 3;
/// The address shift of rings swept into one chunk: an address is the
/// vertex.
pub(super) const ONE: u32 = usize::BITS - 1;

/// A run's place in the chunks, or one piece of a run too long for one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Piece {
    /// The run's place in the sweep's order.
    pub(super) run: usize,
    pub(super) chunk: usize,
    /// Its first vertex in the chunk.
    pub(super) base: usize,
    /// The first ring of the run it holds, and how many it holds.
    pub(super) first: usize,
    pub(super) rings: usize,
    /// Whether it holds the run's first ring and bottom cap, and its last
    /// ring and top cap.
    pub(super) lead: bool,
    pub(super) tail: bool,
}

impl Piece {
    /// The caps it holds where the sweep draws them.
    pub(super) fn caps(&self, drawn: bool) -> usize {
        if drawn {
            usize::from(self.lead) + usize::from(self.tail)
        } else {
            0
        }
    }

    pub(super) fn vertices(&self, segments: usize, drawn: bool) -> usize {
        self.rings * segments + self.caps(drawn)
    }

    /// The ring pairs it draws and answers contact queries for, by the lower
    /// ring's place in the run: every pair once over a run's pieces.
    pub(super) fn pairs(&self) -> Range<usize> {
        let last = self.first + self.rings - 1;
        let from = self.first + usize::from(!self.lead);
        let to = if self.tail { last } else { last - 1 };
        from..to
    }

    /// The indices it draws: six a segment of each pair, and three a segment
    /// of each cap.
    pub(super) fn indices(&self, segments: usize, drawn: bool) -> usize {
        (self.pairs().len() * 2 + self.caps(drawn)) * segments * 3
    }

    /// Where run vertex `v` sits in its chunk: a ring vertex this piece holds,
    /// or one of the run's caps, `rings` rings past its first.
    pub(super) fn local(&self, v: usize, rings: usize, segments: usize) -> usize {
        let ring = rings * segments;
        if v < ring {
            return self.base + v - self.first * segments;
        }
        let top = v - ring;
        self.base + self.rings * segments + top * usize::from(self.lead)
    }
}

/// The chunks a sweep fills and the pieces that fill them, in order.
#[derive(Debug, Default)]
pub(super) struct Layout {
    pub(super) pieces: Vec<Piece>,
    /// Each chunk's vertices.
    pub(super) chunks: Vec<usize>,
    /// Bits a chunk's local vertex takes in a ring address.
    pub(super) shift: u32,
}

impl Layout {
    /// Lays out runs of `rings` rings each, in order, in chunks of at most
    /// `limit` vertices; `usize::MAX` is one chunk.
    pub(super) fn new(
        rings: impl ExactSizeIterator<Item = usize>,
        segments: usize,
        drawn: bool,
        limit: usize,
    ) -> Result<Self> {
        let caps = 2 * usize::from(drawn);
        // A piece holds more rings than it repeats, so a split moves on.
        assert!(
            limit / segments > OVERLAP + 1,
            "a chunk of {limit} holds too few rings"
        );
        let mut out = Self {
            pieces: reserved(rings.len())?,
            chunks: Vec::new(),
            shift: if limit == usize::MAX {
                ONE
            } else {
                limit.next_power_of_two().ilog2()
            },
        };
        let too_many = || Error::ResourceLimit("surface vertices");
        for (run, count) in rings.enumerate() {
            let whole = count
                .checked_mul(segments)
                .and_then(|n| n.checked_add(caps))
                .ok_or_else(too_many)?;
            let mut piece = Piece {
                run,
                chunk: 0,
                base: 0,
                first: 0,
                rings: count,
                lead: true,
                tail: true,
            };
            if whole <= limit {
                out.place(piece, whole, limit)?;
                continue;
            }
            out.open()?;
            loop {
                let rest = count - piece.first;
                let lead = usize::from(drawn && piece.lead);
                if rest * segments + lead + usize::from(drawn) <= limit {
                    piece.rings = rest;
                    out.place(piece, piece.vertices(segments, drawn), limit)?;
                    break;
                }
                piece.rings = (limit - lead) / segments;
                piece.tail = false;
                out.place(piece, piece.vertices(segments, drawn), limit)?;
                out.open()?;
                piece.first += piece.rings - OVERLAP;
                (piece.lead, piece.tail) = (false, true);
            }
        }
        Ok(out)
    }

    /// Opens a fresh chunk unless the last one is still empty.
    fn open(&mut self) -> Result<()> {
        if self.chunks.last().is_none_or(|&n| n > 0) {
            self.chunks
                .try_reserve(1)
                .map_err(|_| Error::ResourceLimit("surface allocation"))?;
            self.chunks.push(0);
        }
        Ok(())
    }

    /// Places a piece of `vertices` at the end of the last chunk, or in a
    /// fresh one where it does not fit.
    fn place(&mut self, mut piece: Piece, vertices: usize, limit: usize) -> Result<()> {
        if self.chunks.last().is_none_or(|&n| n > limit - vertices) {
            self.open()?;
        }
        piece.chunk = self.chunks.len() - 1;
        let fill = &mut self.chunks[piece.chunk];
        piece.base = *fill;
        *fill += vertices;
        self.pieces.push(piece);
        Ok(())
    }

    /// The pieces of each run, in order.
    pub(super) fn runs(&self) -> impl Iterator<Item = &[Piece]> {
        self.pieces.chunk_by(|a, b| a.run == b.run)
    }

    /// Each chunk's indices, over the pieces it holds.
    pub(super) fn indices(&self, segments: usize, drawn: bool) -> Result<Vec<usize>> {
        let mut out = filled(self.chunks.len(), 0)?;
        for piece in &self.pieces {
            out[piece.chunk] += piece.indices(segments, drawn);
        }
        Ok(out)
    }

    /// The elements every chunk together holds.
    pub(super) fn extent(&self, segments: usize) -> WoodExtent {
        let vertices: usize = self.chunks.iter().sum();
        let pieces = self.pieces.iter();
        WoodExtent {
            positions: vertices * 3,
            coords: vertices * 2,
            indices: pieces.map(|p| p.indices(segments, true)).sum(),
            chunks: self.chunks.len(),
        }
    }
}

/// Records each node's contact rings over a run's pieces: its pair's lower
/// and upper ring, then the first and last ring of the piece that answers
/// for the pair, as ring addresses. `offset` is the rings before the run's
/// first node, a buried foot.
pub(super) fn record(
    edges: &mut [Option<[usize; 4]>],
    nodes: &[usize],
    pieces: &[Piece],
    segments: usize,
    offset: usize,
    shift: u32,
) {
    let mut pieces = pieces.iter().peekable();
    for (i, &node) in nodes.iter().enumerate().skip(1) {
        let pair = i - 1 + offset;
        while pieces.peek().is_some_and(|p| !p.pairs().contains(&pair)) {
            pieces.next();
        }
        let Some(p) = pieces.peek() else { return };
        let at = |ring: usize| address(p.chunk, p.base + (ring - p.first) * segments, shift);
        edges[node] = Some([
            at(pair),
            at(pair + 1),
            at(p.first),
            at(p.first + p.rings - 1),
        ]);
    }
}

/// The ring address of vertex `local` of chunk `chunk`.
#[inline]
pub(super) fn address(chunk: usize, local: usize, shift: u32) -> usize {
    chunk << shift | local
}

/// A ring address split into its chunk and its vertex in the chunk.
#[inline]
pub(super) fn split(address: usize, shift: u32) -> (usize, usize) {
    (address >> shift, address & ((1 << shift) - 1))
}

#[cfg(test)]
mod tests;
