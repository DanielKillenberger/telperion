//! fn-198 task 1 probe: the engine's growth and the GPU path's CPU
//! preparation in wasm32, memory pages read at each step.
#[path = "/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-render/examples/space/tree.rs"]
mod tree;
use telperion_core::{params, pipeline::executor, presets::Preset};
use telperion_space::{grow, Light, Request};

#[link(wasm_import_module = "env")]
extern "C" {
    fn now() -> f64;
}
static mut OUT: [f64; 16] = [0.0; 16];

fn pages() -> f64 {
    core::arch::wasm32::memory_size(0) as f64 * 65536.0
}

#[no_mangle]
pub extern "C" fn out_ptr() -> *const f64 {
    core::ptr::addr_of!(OUT) as *const f64
}

fn put(i: usize, v: f64) {
    unsafe { (*core::ptr::addr_of_mut!(OUT))[i] = v }
}

/// 0 palm, 1 beech, 2 oak, 3 spruce. Returns 0 on success, 1 on error.
#[no_mangle]
pub extern "C" fn run(which: u32, age: u32, prep: u32) -> u32 {
    let (species, trunk, preset, rows, light): (_, &[usize], _, &str, Light) = match which {
        0 => (telperion_space::palm(), &tree::PALM_TRUNK, "date-palm", "{}", Light::NEUTRAL),
        1 => (telperion_space::beech(), &tree::BEECH_TRUNK, "european-beech",
              r#"{"canopy": {"shortShootSpacing": 0, "shootRadius": 0.018, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.006}}}}"#, Light::NEUTRAL),
        2 => (telperion_space::oak(), &tree::OAK_TRUNK, "oregon-white-oak",
              r#"{"canopy": {"shortShootSpacing": 0, "shootRadius": 0.06, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.008}}}}"#,
              Light { extinction: 0.5, sky: 0.5 }),
        _ => (telperion_space::spruce(), &tree::SPRUCE_TRUNK, "norway-spruce",
              r#"{"canopy": {"shortShootSpacing": 0, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.0032}}}}"#, Light::NEUTRAL),
    };
    put(0, pages());
    let t0 = unsafe { now() };
    let request = Request { age, seed: 1, budget: 20_000_000, light };
    let Ok(structure) = grow(&species, request) else { return 1 };
    put(1, unsafe { now() } - t0);
    put(2, pages());
    put(3, structure.phytomer_count() as f64);
    if prep == 0 {
        return 0;
    }
    let t1 = unsafe { now() };
    let pipeline_tree = tree::convert(&structure, trunk);
    drop(structure);
    put(4, pipeline_tree.nodes.len() as f64);
    put(5, pages());
    let family = Preset::from_id(preset).unwrap().parameters();
    let rows: serde_json::Value = serde_json::from_str(rows).unwrap();
    let mut family = params::overlay(&family, &rows).unwrap();
    family.skeleton.seed = 1;
    let Ok(x) = executor::expand(pipeline_tree, &family) else { return 2 };
    put(6, pages());
    if x.supports_stations() && x.round_section() {
        if x.seats() {
            let Ok(shared) = x.compact_with_contacts() else { return 3 };
            put(7, pages());
            let st = x.compact_stations(&shared);
            if st.is_err() { return 4; }
            put(8, pages());
            let Ok(curve) = x.curve() else { return 5 };
            put(9, pages());
            drop((curve, st, shared));
        } else {
            let Ok(compact) = x.compact() else { return 3 };
            put(7, pages());
            let st = x.stations();
            if st.is_err() { return 4; }
            put(8, pages());
            let Ok(curve) = x.curve() else { return 5 };
            put(9, pages());
            drop((curve, st, compact));
        }
        put(10, 1.0);
    } else {
        let Ok(mesh) = x.mesh() else { return 6 };
        put(9, pages());
        drop(mesh);
        put(10, 0.0);
    }
    put(11, unsafe { now() } - t1);
    0
}

/// grow_fine on the palm, to see whether it runs on wasm32.
#[no_mangle]
pub extern "C" fn fine() -> u32 {
    let species = telperion_space::spruce();
    let request = Request { age: 10, seed: 1, budget: 20_000_000, light: Light::NEUTRAL };
    let expected = telperion_space::Expected::new(&species, 10, 1);
    match telperion_space::grow_fine(&species, request, &expected, telperion_space::Plan::default()) {
        Ok(f) => f.structure.phytomer_count() as u32,
        Err(_) => 0,
    }
}

/// std's clock alone, to name what traps.
#[no_mangle]
pub extern "C" fn instant() -> u32 {
    std::time::Instant::now().elapsed().as_nanos() as u32
}
