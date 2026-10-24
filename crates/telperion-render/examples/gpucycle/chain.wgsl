// The dependency probe: dispatch k of a chain reads its workgroup count
// from `from` (as an indirect argument) and writes dispatch k+1's into
// `to`, so every dispatch waits on the one before; `noop` does nothing.
//
// P.a[0].x: k.

@group(0) @binding(1) var<storage, read_write> to: array<u32>;
@group(0) @binding(2) var<storage, read_write> state: array<u32>;

@compute @workgroup_size(64)
fn step(@builtin(local_invocation_index) lid: u32) {
    if lid == 0u {
        let k = P.a[0].x + 1u;
        state[0] = state[0] + 1u;
        to[3u * k] = 1u;
        to[3u * k + 1u] = 1u;
        to[3u * k + 2u] = 1u;
    }
}

@compute @workgroup_size(64)
fn noop() {}
