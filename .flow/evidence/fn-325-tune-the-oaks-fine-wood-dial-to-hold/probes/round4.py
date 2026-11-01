import random
ev = '.flow/evidence/fn-325-tune-the-oaks-fine-wood-dial-to-hold/'
with open(ev + 'ROUNDS.md', 'a') as f:
    f.write('''
## Round 3's result

On seeds 1001 and 1002: s 2, leaf 8, pipe 2.5, b 1.3, h 2, s2 1, x 0.1 costs 287 ms, worst 0.038: height 1.03, spread 0.96, trunk 1.00, limbs 0.99, shed share 0.99, leaf area 0.99. The shedding delay closed the shed share. A second factor on twigs (s2 above 1) loses shed share and leaf area quickly. The fastest fits, 93 to 97 ms, have worst 0.32.

## Round 4

A random search around that candidate, for the cheapest with worst at or under 0.05, seeded at 325, 1,000 candidates on seeds 1001, 1002 and 1003. Uniform in log space: s 1.5 to 5, s2 1 to 2, leaf 3 to 24, pipe 1.2 to 5, b 1 to 3, h 1 to 6, x 0.05 to 1. Scope first age 9. The generator is probes/round4.py.
''')
r = random.Random(325)
def log(lo, hi):
    import math
    return round(math.exp(r.uniform(math.log(lo), math.log(hi))), 3)
with open(ev + 'raw/round4.txt', 'w') as f:
    for _ in range(1000):
        f.write(f"9 {log(1.5,5)} {log(3,24)} {log(1.2,5)} {log(1,3)} {log(1,6)} {log(1,2)} {log(.05,1)}\n")
p = 'crates/telperion-core/src/pipeline/field/coarse_tests.rs'
t = open(p).read()
t += '''
/// Candidates from `SEARCH_FILE`, one a line: first s leaf pipe bud hazard s2 x.
#[test]
#[ignore = "fn-325's search"]
fn coarse_list() {
    let age = env_list::<u32>("SEARCH_AGE")[0];
    let seeds: Vec<u32> = env_list("SEARCH_SEEDS");
    let repeats = env_list::<usize>("SEARCH_REPEATS")[0];
    let base = by_identity("oregon-white-oak").expect("the oak");
    let species = base.space.species.clone().expect("a species");
    let r = medians(&base, &species, age, &seeds, repeats).expect("the reference");
    println!("REFERENCE age {age} {r:?}");
    let list = std::fs::read_to_string(std::env::var("SEARCH_FILE").expect("a file")).expect("the list");
    for row in list.lines().filter(|l| !l.trim().is_empty()) {
        let v: Vec<f64> = row.split_whitespace().map(|x| x.parse().expect("a value")).collect();
        let d = Dial { first: v[0] as usize, s: v[1], leaf: v[2], pipe: v[3], bud: v[4], hazard: v[5], s2: v[6], shed: v[7] };
        match medians(&base, &scaled(&species, d), age, &seeds, repeats) {
            Some(m) => line("CAND", d, &m, &r, age),
            None => println!("FAILED {d:?}"),
        }
    }
}
'''
open(p, 'w').write(t)
