//! Scratch-only native sampler for stage 2 (never shipped): a SIGPROF timer
//! records the interrupted instruction and the frame-pointer chain while the
//! skeleton grows, and the raw stacks are printed with the executable's load
//! base for `analyze-native.py` to symbolize. Build with frame pointers:
//! RUSTFLAGS="-C force-frame-pointers=yes" CARGO_PROFILE_RELEASE_DEBUG=line-tables-only
use std::sync::atomic::{AtomicUsize, Ordering};
use telperion_core::{pipeline, presets::Preset};

const DEPTH: usize = 48;
const CAPACITY: usize = 400_000;
static mut STACKS: [[u64; DEPTH]; CAPACITY] = [[0; DEPTH]; CAPACITY];
static NEXT: AtomicUsize = AtomicUsize::new(0);
static mut STACK_TOP: u64 = 0;

#[repr(C)]
struct Timeval(i64, i64);
#[repr(C)]
struct Itimerval(Timeval, Timeval);
#[repr(C)]
struct SigAction {
    handler: usize,
    mask: [u64; 16],
    flags: i32,
    restorer: usize,
}
extern "C" {
    fn setitimer(which: i32, new: *const Itimerval, old: *mut Itimerval) -> i32;
    fn sigaction(sig: i32, act: *const SigAction, old: *mut SigAction) -> i32;
}

extern "C" fn on_prof(_: i32, _: *mut u8, context: *mut u8) {
    let slot = NEXT.fetch_add(1, Ordering::Relaxed);
    if slot >= CAPACITY {
        return;
    }
    unsafe {
        let gregs = context.add(40) as *const u64;
        let (rip, mut rbp) = (*gregs.add(16), *gregs.add(10));
        let rsp = *gregs.add(15);
        let stack = &mut (*std::ptr::addr_of_mut!(STACKS))[slot];
        stack[0] = rip;
        // A leaf that has not pushed a frame yet (a shrink-wrapped fast path)
        // still has its return address on top of the stack; the analysis
        // keeps this word only where it is a call site in the executable.
        stack[1] = *(rsp as *const u64);
        let mut n = 2;
        while n < DEPTH && rbp >= rsp && rbp < STACK_TOP && rbp % 8 == 0 {
            let ret = *((rbp + 8) as *const u64);
            if ret == 0 {
                break;
            }
            stack[n] = ret - 1;
            n += 1;
            let next = *(rbp as *const u64);
            if next <= rbp {
                break;
            }
            rbp = next;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let preset = args.get(1).ok_or("preset required")?;
    let seed: u32 = args.get(2).ok_or("seed required")?.parse()?;
    let builds: usize = args.get(3).map_or(Ok(40), |s| s.parse())?;
    let mut f = Preset::from_id(preset).ok_or("unknown preset")?.parameters();
    f.skeleton.seed = seed;
    pipeline::skeleton(&f)?; // warm
    let marker = 0u64;
    unsafe { STACK_TOP = std::ptr::addr_of!(marker) as u64 + (1 << 20) };
    let act = SigAction { handler: on_prof as usize, mask: [0; 16], flags: 4 | 0x1000_0000, restorer: 0 };
    let every = Itimerval(Timeval(0, 100), Timeval(0, 100));
    let start = std::time::Instant::now();
    unsafe {
        sigaction(27, &act, std::ptr::null_mut());
        setitimer(2, &every, std::ptr::null_mut());
    }
    for _ in 0..builds {
        std::hint::black_box(pipeline::skeleton(&f)?);
    }
    unsafe { setitimer(2, &Itimerval(Timeval(0, 0), Timeval(0, 0)), std::ptr::null_mut()) };
    let ms = start.elapsed().as_secs_f64() * 1000.0 / builds as f64;
    let maps = std::fs::read_to_string("/proc/self/maps")?;
    let exe = std::fs::read_link("/proc/self/exe")?;
    let base = maps
        .lines()
        .find(|l| l.ends_with(exe.to_str().unwrap()) && l.split_whitespace().nth(2) == Some("00000000"))
        .and_then(|l| u64::from_str_radix(l.split('-').next()?, 16).ok())
        .ok_or("load base")?;
    println!("{{\"preset\":\"{preset}\",\"seed\":{seed},\"builds\":{builds},\"ms_per_build\":{ms:.3},\"base\":{base},\"interval_us\":100}}");
    let taken = NEXT.load(Ordering::Relaxed).min(CAPACITY);
    let stacks: &[[u64; DEPTH]] = unsafe { &*std::ptr::addr_of!(STACKS) };
    for stack in &stacks[..taken] {
        let frames: Vec<String> = stack.iter().enumerate().take_while(|&(k, &a)| k == 1 || a != 0).map(|(_, a)| format!("{:x}", a.wrapping_sub(base).wrapping_sub(0))).collect();
        println!("{}", frames.join(" "));
    }
    Ok(())
}
