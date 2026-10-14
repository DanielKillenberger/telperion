//! Red and green fixtures for `tint_rules`, each pair run through Tint
//! itself (Dawn's `tint`, fn-237 R14) to agree with it, and the rules held
//! on every shipped module.
use crate::tint_rules::findings;

#[test]
fn a_nan_constant_is_refused_and_a_finite_one_kept() {
    let nan = "const NAN_BITS: u32 = 0x7fc00000u;\nconst NAN: f32 = bitcast<f32>(NAN_BITS);";
    assert_eq!(findings(nan).len(), 1);
    let inf = "const INF: f32 = bitcast<f32>(0x7f800000u);";
    assert_eq!(findings(inf).len(), 1);
    // The spike's shed module, refused by Chrome: a constant expression.
    let shed =
        "const NAN_BITS: u32 = 0x7fc00000u;\nfn f() { var memory = bitcast<f32>(NAN_BITS); }";
    assert_eq!(findings(shed).len(), 1);
    assert!(findings("const ONE: f32 = bitcast<f32>(0x3f800000u);").is_empty());
}

#[test]
fn a_barrier_in_a_loop_bound_by_storage_is_refused() {
    let bad = "@group(0) @binding(1) var<storage, read_write> grid: array<u32>;
        fn nz() -> u32 { return grid[2]; }
        @compute @workgroup_size(64)
        fn sweep(@builtin(local_invocation_index) lid: u32) {
            let n = nz();
            for (var step = 0u; step < n; step++) { workgroupBarrier(); }
        }";
    assert_eq!(findings(bad).len(), 1, "{:?}", findings(bad));
    let good = "@group(0) @binding(1) var<storage, read_write> grid: array<u32>;
        var<workgroup> shape: u32;
        @compute @workgroup_size(64)
        fn sweep(@builtin(local_invocation_index) lid: u32) {
            if lid == 0u { shape = grid[2]; }
            let n = workgroupUniformLoad(&shape);
            for (var step = 0u; step < n; step++) { workgroupBarrier(); }
        }";
    assert!(findings(good).is_empty(), "{:?}", findings(good));
}

#[test]
fn a_barrier_after_a_non_uniform_return_is_refused() {
    let bad = "@compute @workgroup_size(64)
        fn f(@builtin(local_invocation_index) lid: u32) {
            if lid > 3u { return; }
            workgroupBarrier();
        }";
    assert_eq!(findings(bad).len(), 1);
    let good = "@compute @workgroup_size(64)
        fn f(@builtin(workgroup_id) wid: vec3<u32>) {
            if wid.x > 3u { return; }
            workgroupBarrier();
        }";
    assert!(findings(good).is_empty(), "{:?}", findings(good));
}

/// Every shipped module (fn-237 R14).
#[test]
fn every_shipped_shader_keeps_tints_rules() {
    let mut all = Vec::new();
    for (name, source) in crate::shader_tests::shipped() {
        all.extend(
            findings(&source)
                .into_iter()
                .map(|f| format!("{name}: {f}")),
        );
    }
    assert!(all.is_empty(), "Tint would refuse:\n{}", all.join("\n"));
}

/// A function returning a uniform buffer's field is uniform; one
/// returning a storage read is not (`select`'s `group_count`).
#[test]
fn a_return_of_a_uniform_field_is_uniform() {
    let head = "struct U { n: u32 }
        @group(0) @binding(0) var<uniform> u: U;
        @group(0) @binding(1) var<storage, read_write> grid: array<u32>;";
    let entry = "@compute @workgroup_size(64) fn main(@builtin(workgroup_id) id: vec3<u32>) {
        if id.x >= count() { return; }
        workgroupBarrier();
    }";
    let good = format!("{head}\nfn count() -> u32 {{ return u.n; }}\n{entry}");
    assert!(findings(&good).is_empty(), "{:?}", findings(&good));
    let bad = format!("{head}\nfn count() -> u32 {{ return grid[0]; }}\n{entry}");
    assert_eq!(findings(&bad).len(), 1, "{:?}", findings(&bad));
}

/// A non-uniform `continue` makes the rest of its loop non-uniform, not
/// what follows the loop (`positions`' `admit`).
#[test]
fn a_jump_ends_with_its_loop() {
    let head = "@group(0) @binding(1) var<storage, read_write> grid: array<u32>;";
    let good = format!("{head}
        @compute @workgroup_size(64) fn main(@builtin(local_invocation_index) lane: u32) {{
            for (var v = lane; v < 100u; v += 64u) {{ if grid[v] == 0u {{ continue; }} grid[v] = 1u; }}
            workgroupBarrier();
        }}");
    assert!(findings(&good).is_empty(), "{:?}", findings(&good));
    let bad = format!("{head}
        @compute @workgroup_size(64) fn main(@builtin(local_invocation_index) lane: u32) {{
            for (var v = 0u; v < 4u; v += 1u) {{ if grid[v] == lane {{ continue; }} workgroupBarrier(); }}
        }}");
    assert_eq!(findings(&bad).len(), 1, "{:?}", findings(&bad));
}
