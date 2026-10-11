// fn-256 task 2 (DESIGN.md section 5): every dispatch sized from the
// device. A point of the cycle runs one invocation per program (`vm.rs`);
// each program reads the device's counts (`census`), computes one word
// in integers and stores it: a dispatch's uniform word, its indirect
// workgroups, or a count. The host knows only the cycle and the bounds.

struct Cfg {
    first: u32,
    n: u32,
    pad0: u32,
    pad1: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> code: array<u32>;
@group(0) @binding(2) var<storage, read_write> census: array<u32>;
@group(0) @binding(3) var<storage, read_write> cfgs: array<u32>;
@group(0) @binding(4) var<storage, read_write> args: array<u32>;

const STACK: u32 = 16u;
// The longest program.
const STEPS: u32 = 256u;

@compute @workgroup_size(64)
fn vm_main(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= cfg.n {
        return;
    }
    var pc = code[cfg.first + id.x];
    var stack: array<u32, STACK>;
    var top = 0u;
    // cap: `STEPS` operations.
    for (var step = 0u; step < STEPS; step++) {
        let op = code[pc] & 0xffu;
        let k = code[pc] >> 8u;
        pc += 1u;
        if op == 0u {
            return;
        }
        if op == 1u || op == 2u || op == 3u {
            var v = k;
            if op == 2u {
                v = code[pc];
                pc += 1u;
            } else if op == 3u {
                v = census[k];
            }
            if top < STACK {
                stack[top] = v;
            }
            top += 1u;
            continue;
        }
        if op >= 20u {
            top -= 1u;
            let v = stack[min(top, STACK - 1u)];
            if op == 20u {
                census[k] = v;
            } else if op == 21u {
                cfgs[k] = v;
            } else {
                args[k] = v;
            }
            continue;
        }
        if op == 11u {
            top -= 2u;
            let c = stack[top - 1u];
            stack[top - 1u] = select(stack[top + 1u], stack[top], c != 0u);
            continue;
        }
        top -= 1u;
        let b = stack[top];
        let a = stack[top - 1u];
        var r = 0u;
        switch op {
            case 4u: { r = a + b; }
            case 5u: { r = select(0u, a - b, a > b); }
            case 6u: { r = a * b; }
            case 7u: { r = select(0u, (a + b - 1u) / b, b != 0u); }
            case 8u: { r = min(a, b); }
            case 9u: { r = max(a, b); }
            case 10u: { r = u32(a > b); }
            case 12u: { r = select(0u, a / b, b != 0u); }
            default: { r = 0u; }
        }
        stack[top - 1u] = r;
    }
}
