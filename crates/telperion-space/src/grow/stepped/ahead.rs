//! The fine roots grown ahead under R3 and R4 (fn-226 task 3): each root
//! alone, from the cycle it was handed over to the tree's age, reading
//! only the history the structure above recorded (`outline.rs`). Each
//! cycle its buds read their light from the last cycle's outline at their
//! own tips, its balance takes its root's shedding, kept share and memory
//! from the record, its split starts from the root's recorded incoming
//! vigour (S4), and its sizes are normalised by the outline's means: no
//! root waits on another, and nothing it grows flows back up.
use super::build;
use super::outline::{read, size, Growth, History};
use super::phases::links;
use super::{Executor, Item, ITEMS};
use crate::allocation::split;
use crate::error::{Error, Result};
use crate::fine::Plan;
use crate::grow::Request;
use crate::species::Species;
use crate::structure::Structure;

/// A tree grown under R3 and R4 (fn-226 task 3), beside today's model
/// for the head-to-head: the tree, its fine roots, the phytomers grown,
/// per cycle and PA the growing buds' presence and presence times size
/// (host decision 23's drift), and the seconds the closed form's rows,
/// the structure above, the roots grown ahead and the settle took.
pub struct Outlined {
    pub structure: Structure,
    pub roots: usize,
    pub grown: u64,
    pub growth: Vec<Vec<[f64; 2]>>,
    pub seconds: [f64; 4],
}

/// Grows `species` under R3 and R4 (`outline.rs`, `ahead.rs`): the
/// structure above the cut first, each fine root an expectation in it,
/// then every root ahead from the history it recorded. Where no bud
/// reads light the two models are one, and this is `grow`'s build.
pub fn grow_outline(species: &Species, request: Request) -> Result<Outlined> {
    let clock = crate::clock::seconds;
    let started = clock();
    let Some(mut x) = Executor::outlined(species, request)? else {
        let fine = build(species, request, Plan::default())?;
        return Ok(Outlined {
            roots: fine.roots.len(),
            grown: 0,
            structure: crate::fine::lineage_order(fine.structure),
            growth: Vec::new(),
            seconds: [0.0, 0.0, 0.0, clock() - started],
        });
    };
    let mut above = x.above();
    let rows = clock() - started;
    for cycle in 1..=request.age {
        x.above_cycle(cycle, &mut above)?;
    }
    let grown_above = clock() - started - rows;
    let mut growth = std::mem::take(&mut above.growth);
    x.ahead(&above.history, &mut growth)?;
    let ahead = clock() - started - rows - grown_above;
    let roots = x.items.len();
    let grown = x.coarse.rules.grown + x.items.iter().map(|i| i.g.rules.grown).sum::<u64>();
    let fine = x.settle()?;
    let settle = clock() - started - rows - grown_above - ahead;
    Ok(Outlined {
        structure: crate::fine::lineage_order(fine.structure),
        roots,
        grown,
        growth,
        seconds: [rows, grown_above, ahead, settle],
    })
}

impl Executor<'_> {
    /// Every item grown ahead to the tree's age on every core, its growth
    /// added to `growth`.
    pub(super) fn ahead(&mut self, history: &History, growth: &mut Growth) -> Result<()> {
        let age = self.request.age;
        let (balances, allots) = (self.coarse.balances(), self.coarse.allots());
        let items: Vec<(usize, Item)> = std::mem::take(&mut self.items)
            .into_iter()
            .enumerate()
            .collect();
        let threads = self.coarse.threads;
        let ran = crate::cores::pinned(items, ITEMS, threads, &|_, (k, mut item)| {
            let (ledger, g) = (&history.ledgers[k], &mut item.g);
            let mut grew: Growth = Vec::new();
            let mut next = item.next;
            for c in item.next..=age {
                if g.idle(c) {
                    break;
                }
                let n = c as usize;
                read(g, history.outline(c - 1), 1);
                if balances {
                    let rows = links(g);
                    let totals = g.totals(&rows, &[]);
                    let root = &mut g.sketch.as_mut().expect("sketching").pencils[1];
                    root.memory = ledger.memory[n];
                    g.shed_by(c, &rows, &totals, Some((ledger.shed[n], ledger.kept[n])));
                }
                if allots {
                    let rows = links(g);
                    let inputs = g.inputs(&rows);
                    let base = Some(ledger.vigour[n]);
                    let got = split(&rows.links, &inputs.lit, &inputs.lambda, base).1;
                    let o = history.outline(c - 1);
                    size(g, (&got, &inputs, &rows.pa), o, (c, &mut grew));
                }
                let advanced: Vec<usize> = g.live.iter().map(|a| a.axis).collect();
                let live = std::mem::take(&mut g.live);
                g.grow_live(live, c, &mut |_| {})?;
                g.wake(c)?;
                if c < age {
                    g.sketch(c, &advanced)?;
                }
                next = c + 1;
            }
            item.next = next;
            Ok::<_, Error>((item, grew))
        });
        for r in ran {
            let (item, grew) = r?;
            add(growth, &grew);
            self.items.push(item);
        }
        let grown =
            self.coarse.rules.grown + self.items.iter().map(|i| i.g.rules.grown).sum::<u64>();
        if grown > u64::from(self.request.budget) {
            return Err(Error::Budget {
                limit: self.request.budget,
            });
        }
        Ok(())
    }
}

/// `more` added into `growth`, cycle by cycle and PA by PA.
fn add(growth: &mut Growth, more: &Growth) {
    if growth.len() < more.len() {
        growth.resize(more.len(), Vec::new());
    }
    for (row, more) in growth.iter_mut().zip(more) {
        if row.len() < more.len() {
            row.resize(more.len(), [0.0; 2]);
        }
        for (cell, m) in row.iter_mut().zip(more) {
            cell[0] += m[0];
            cell[1] += m[1];
        }
    }
}
