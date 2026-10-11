// fn-256 task 2 (DESIGN.md sections 5 and 7a): every dispatch's words
// from the device's counts. A point of the cycle runs one workgroup over
// its programs (`vm.rs`); each program reads the device's counts
// (`census`), computes one word in integers and stores it: a dispatch's
// uniform word, a count, or a flag. The host sizes each dispatch from
// its bounds; a program per dispatch raises a flag where the true size
// passes the workgroups given (`given`).

struct Cfg {
    first: u32,
    n: u32,
    // The flags (`census.rs`): once one is raised, every uniform word a
    // point sets is none, so no kernel reads what a count past its
    // capacity or bound left, and the host begins the build again.
    flags_at: u32,
    flags_n: u32,
    // The point's programs that flag, run before the rest.
    early: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> code: array<u32>;
@group(0) @binding(2) var<storage, read_write> census: array<u32>;
@group(0) @binding(3) var<storage, read_write> cfgs: array<u32>;
@group(0) @binding(4) var<storage, read_write> given: array<u32>;

const STACK: u32 = 16u;
// The longest program.
const STEPS: u32 = 256u;
const LANES: u32 = 64u;

fn halted() -> bool {
    var any = 0u;
    for (var f = 0u; f < cfg.flags_n; f++) {
        any |= census[cfg.flags_at + f];
    }
    return any != 0u;
}

// Program `j` of the point run; a uniform word it sets is none where the
// point is `stopped`.
fn run(j: u32, stopped: bool) {
    var pc = code[cfg.first + j];
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
        if op == 1u || op == 2u || op == 3u || op == 13u {
            var v = k;
            if op == 2u {
                v = code[pc];
                pc += 1u;
            } else if op == 3u {
                v = census[k];
            } else if op == 13u {
                v = given[k];
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
                cfgs[k] = select(v, 0u, stopped);
            } else if v != 0u {
                census[k] = v;
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

// The point's flags first, then, past a barrier, the rest under them.
@compute @workgroup_size(64)
fn vm_main(@builtin(local_invocation_index) lane: u32) {
    for (var j = lane; j < cfg.early; j += LANES) {
        run(j, false);
    }
    storageBarrier();
    let stopped = halted();
    for (var j = cfg.early + lane; j < cfg.n; j += LANES) {
        run(j, stopped);
    }
}
