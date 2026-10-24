use telperion_space::{grow, Light, Request};
#[link(wasm_import_module = "env")]
extern "C" {
    fn now() -> f64;
}
/// 0 palm, 1 beech, 2 oak (lit), 3 spruce: milliseconds, or -1.
#[no_mangle]
pub extern "C" fn run(which: u32, age: u32) -> f64 {
    let (s, light) = match which {
        0 => (telperion_space::palm(), Light::default()),
        1 => (telperion_space::beech(), Light::default()),
        2 => (telperion_space::oak(), Light { extinction: 0.5, sky: 0.5 }),
        _ => (telperion_space::spruce(), Light::default()),
    };
    let t = unsafe { now() };
    let r = Request { age, seed: 1, budget: 40_000_000, light };
    match grow(&s, r) {
        Ok(tree) => {
            let ms = unsafe { now() } - t;
            std::hint::black_box(tree.phytomer_count());
            ms
        }
        Err(_) => -1.0,
    }
}

#[no_mangle]
pub extern "C" fn micro(which: u32, n: u32) -> f64 {
    let mut x = 0x9e37_79b9_7f4a_7c15u64;
    let mut acc = 0i64;
    let t = unsafe { now() };
    for _ in 0..n {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        acc = acc.wrapping_add(match which {
            0 => telperion_space::fixed::log2(std::hint::black_box(x >> 11)),
            _ => telperion_space::fixed::exp2_neg(std::hint::black_box((x >> 30) as i64)) as i64,
        });
    }
    std::hint::black_box(acc);
    (unsafe { now() } - t) * 1e6 / n as f64
}

/// The lineage digest of the tree `run` grows (seed 1), its low and high halves by `half`.
#[no_mangle]
pub extern "C" fn digest(which: u32, age: u32, half: u32) -> u32 {
    let (s, light) = match which {
        0 => (telperion_space::palm(), Light::default()),
        1 => (telperion_space::beech(), Light::default()),
        2 => (telperion_space::oak(), Light { extinction: 0.5, sky: 0.5 }),
        _ => (telperion_space::spruce(), Light::default()),
    };
    let tree = grow(&s, Request { age, seed: 1, budget: 40_000_000, light }).unwrap();
    let d = telperion_space::digest(&tree);
    (if half == 0 { d } else { d >> 32 }) as u32
}

static mut SUMS: [f64; 64] = [0.0; 64];
static mut NAMES: [u8; 4096] = [0; 4096];
/// `grow_staged`'s time between reports summed by report, into SUMS, names (Debug, newline-separated) into NAMES.
#[no_mangle]
pub extern "C" fn staged(which: u32, age: u32) -> u32 {
    let (s, light) = match which {
        0 => (telperion_space::palm(), Light::default()),
        1 => (telperion_space::beech(), Light::default()),
        2 => (telperion_space::oak(), Light { extinction: 0.5, sky: 0.5 }),
        _ => (telperion_space::spruce(), Light::default()),
    };
    let mut names: Vec<String> = Vec::new();
    let mut sums: Vec<f64> = Vec::new();
    let mut last = unsafe { now() };
    let tree = telperion_space::grow_staged(&s, Request { age, seed: 1, budget: 40_000_000, light }, &mut |st| {
        let t = unsafe { now() };
        let key = match st { telperion_space::Stage::Pass(p) => format!("{p:?}"), o => format!("[{o:?}]") };
        match names.iter().position(|n| *n == key) { Some(i) => sums[i] += t - last, None => { names.push(key); sums.push(t - last); } }
        last = t;
    }).unwrap();
    names.push("~order".into());
    sums.push(unsafe { now() } - last);
    std::hint::black_box(tree.phytomer_count());
    let joined = names.join("\n");
    unsafe {
        let n = &mut *core::ptr::addr_of_mut!(NAMES);
        n[..joined.len()].copy_from_slice(joined.as_bytes());
        let out = &mut *core::ptr::addr_of_mut!(SUMS);
        out[..sums.len()].copy_from_slice(&sums);
    }
    joined.len() as u32
}
#[no_mangle]
pub extern "C" fn sums_ptr() -> *const f64 { core::ptr::addr_of!(SUMS) as *const f64 }
#[no_mangle]
pub extern "C" fn names_ptr() -> *const u8 { core::ptr::addr_of!(NAMES) as *const u8 }

static mut FP: [f64; 16] = [0.0; 16];
/// grow_fine's total, predictor, expansion phases, and lineage_order after it, into FP.
#[no_mangle]
pub extern "C" fn finep(which: u32, age: u32) -> *const f64 {
    let s = if which == 1 { telperion_space::beech() } else { telperion_space::spruce() };
    let t = unsafe { now() };
    let f = telperion_space::grow_fine(&s, Request { age, seed: 1, budget: 40_000_000, light: Light::default() }, telperion_space::Plan::default()).unwrap();
    let total = unsafe { now() } - t;
    let t2 = unsafe { now() };
    let tree = telperion_space::lineage_order(f.structure);
    let order = unsafe { now() } - t2;
    std::hint::black_box(tree.phytomer_count());
    unsafe {
        let o = &mut *core::ptr::addr_of_mut!(FP);
        o[0] = total; o[1] = f.predict * 1e3; o[2] = order;
        for (i, p) in f.phases.iter().enumerate().take(8) { o[3 + i] = p * 1e3; }
        o.as_ptr()
    }
}

#[no_mangle]
pub extern "C" fn whole(which: u32, age: u32) -> f64 {
    let (s, light) = match which {
        0 => (telperion_space::palm(), Light::default()),
        1 => (telperion_space::beech(), Light::default()),
        2 => (telperion_space::oak(), Light { extinction: 0.5, sky: 0.5 }),
        _ => (telperion_space::spruce(), Light::default()),
    };
    let t = unsafe { now() };
    let tree = telperion_space::grow_whole(&s, Request { age, seed: 1, budget: 40_000_000, light }).unwrap();
    let ms = unsafe { now() } - t;
    std::hint::black_box(tree.phytomer_count());
    ms
}
