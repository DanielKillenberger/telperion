//! The WGSL's copies of the CPU's registries: every lineage key
//! (`lineage.wgsl` of `telperion_space::lineage_keys::ALL`) and every probe
//! selector (`probe.wgsl` of `selectors::ALL`), by name and value, and no
//! other.
use super::selectors;
use telperion_space::lineage_keys;

/// Each `const <prefix>NAME ... = <value>;` line of `source`, its value
/// read by `value`.
fn consts(source: &str, prefix: &str, value: fn(&str) -> u64) -> Vec<(String, u64)> {
    source
        .lines()
        .filter_map(|l| l.strip_prefix("const ")?.strip_prefix(prefix))
        .map(|l| {
            let (name, rest) = l.split_once([':', ' ']).expect("a named constant");
            let (_, v) = rest.split_once('=').expect("a value");
            (name.to_string(), value(v.trim().trim_end_matches(';')))
        })
        .collect()
}

fn hex(word: &str) -> u64 {
    let digits = word.trim().trim_start_matches("0x").trim_end_matches('u');
    u64::from_str_radix(digits, 16).expect("a hex word")
}

/// `vec2<u32>(low, high)` as the u64 it holds.
fn pair(v: &str) -> u64 {
    let inner = v
        .strip_prefix("vec2<u32>(")
        .and_then(|v| v.strip_suffix(')'));
    let (lo, hi) = inner.and_then(|v| v.split_once(',')).expect("a pair");
    hex(lo) | hex(hi) << 32
}

fn decimal(v: &str) -> u64 {
    v.trim_end_matches('u').parse().expect("a decimal word")
}

#[test]
fn the_wgsl_keys_are_the_cpus() {
    let wgsl = consts(include_str!("keys.wgsl"), "KEY_", pair);
    let cpu: Vec<(String, u64)> = lineage_keys::ALL
        .iter()
        .map(|&(name, value, _)| (name.to_string(), value))
        .collect();
    assert_eq!(wgsl, cpu);
}

#[test]
fn the_wgsl_selectors_are_the_cpus() {
    let wgsl = consts(include_str!("probe.wgsl"), "P_", decimal);
    let cpu: Vec<(String, u64)> = selectors::ALL
        .iter()
        .map(|&(s, name, _)| (name.trim_start_matches("P_").to_string(), s as u64))
        .collect();
    assert_eq!(wgsl, cpu);
}

#[test]
fn every_selector_is_listed_once_in_order() {
    for (i, (s, _, _)) in selectors::ALL.iter().enumerate() {
        assert_eq!(*s as usize, i, "{s:?}");
    }
}
