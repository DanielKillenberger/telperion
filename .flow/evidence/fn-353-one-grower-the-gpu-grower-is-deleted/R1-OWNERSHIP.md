# What shared ownership of the species touches (11 Nov 2026)

Task 1 stopped again under answer 2 of `R1-ANSWERS.md`: the borrow-to-`Arc` change is more than one chain of types. Read in code at `5c15b3b1c`; nothing was changed or compiled.

## The chain the browser handle needs owned

`pipeline::space::growth::GrowthState<'p>` borrows `Params`, which owns the `Species`. Under it, `telperion_space::Paused<'a>` holds `Grower<'a>`, which holds `species: &'a Species` and `rules: Rules<'a>` (`grow.rs:241-244`), and `Rules` holds `species: &'a Species` too (`grow/shoot.rs:20-21`).

## Why it is not one chain (read in code)

1. **The production grow goes through the same type.** `grow_cycle` runs `Paused::with(species: &Species, …)` (`grow/cycle.rs:69-73`), and every `mesh::build` reaches it. If `Paused` owns an `Arc<Species>`, either every production grow clones its `Species` into an `Arc`, or `grow_cycle`, `grow_census` and every caller up through `telperion-core` take an `Arc<Species>`. The first is a cost on every tree, small but unmeasured. The second is an API change across both crates.
2. **`Rules` is also built from a borrowed species outside the grower.** The fine roots build it in `grow/alone.rs:10-33`, called from `fine/surface.rs:75` (production) and `fine/tables.rs:9`. If `Rules` owns an `Arc`, every fine root's rules clone the species or the fine-root path takes an `Arc` as well. The other way is to take the species out of `Rules` and pass it to `Shoot` beside it, which changes `Shoot::new` and every place that reads `self.rules.species` or `self.species()` (`shoot.rs:158`, `stop.rs`, `wake.rs`, `advance.rs`).
3. **The lifetime runs through the parallel shares.** `Shoot<'g, 'a>` holds `&'g Rules<'a>`, and `grow_shares<'g, 'a: 'g>` takes a `dyn Fn(Vec<Home<'g>>) -> Shoot<'g, 'a> + Sync` (`grow/parallel.rs:224-230`).
4. **The code copies the species reference out of the grower and then mutates the grower.** About 15 sites do `let species = self.species;` and then go on to `&mut self` work or to disjoint field borrows: `balance.rs:49,102`, `draw.rs:120,144,213`, `sketch.rs:178`, `passes.rs:29,106`, `settle.rs:57`, `grow.rs:303`, `retire.rs:134,153`, `paused.rs:254,328`. Once the field is an `Arc`, each needs its own clone or a restructured borrow. That is mechanical one site at a time, but it is not a lifetime swapped in one place.

## Options I see (not chosen)

- **A.** `Paused` and `Grower` own `Arc<Species>`, and `grow_cycle` clones its borrowed species once per tree. `Rules` loses its species field, and `Shoot` borrows the species from the grower for the share's lifetime. This touches about 20 files in `telperion-space/src/grow/`, plus `alone.rs`, `fine/surface.rs` and the census and capture callers. The per-tree clone would need measuring.
- **B.** Make `Grower`, `Rules` and `Paused` generic over the species holder (`S: Borrow<Species>`, a reference for the one-call grow and an `Arc` for the paused handle). No clone and no change to the production API, but a type parameter replaces the lifetime through the same files.
- **C.** Make the binding's handle hold no paused grower. It keeps the family and the years drawn, and regrows on each request. This is option 2 of `R1-EQUALITY.md` in another form, and the host already declined it on cost.
