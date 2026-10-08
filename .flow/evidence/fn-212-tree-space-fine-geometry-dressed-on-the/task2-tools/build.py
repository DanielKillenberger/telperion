#!/usr/bin/env python3
"""Build the timing probe at each state (and the digest probe at the tip),
copied into raw/task2/bin. The probes are copied into the render crate's
examples only while building and removed afterwards."""
import os, shutil, subprocess

W = "/home/daniel/Projects/telperion/.worktrees/readers"
R = f"{W}/.flow/evidence/fn-212-tree-space-fine-geometry-dressed-on-the/raw/task2"
EX = f"{W}/crates/telperion-render/examples"
env = dict(os.environ, CARGO_BUILD_JOBS="8")
os.chdir(W)


def digest_source():
    s = open(f"{EX}/generation_digest.rs").read()
    for a, b in [
        ('let mut f = Preset::from_id(id).ok_or("unknown preset")?.parameters();',
         'let mut f = Preset::from_id(id).ok_or("unknown preset")?.parameters();\n'
         '            if let Ok(a) = std::env::var("DIGEST_AGE") { f.age = a.parse()?; }\n'
         '            let gpu_only = std::env::var("DIGEST_AGE").is_ok();\n'
         '            let t0 = std::time::Instant::now();'),
        ('"pipeline": full(&f)?,', '"pipeline": if gpu_only { Value::Null } else { full(&f)? },'),
        ('"fieldOnly": field_only(&f)?,', '"fieldOnly": Value::Null,'),
        ('"mesh": digest(&mesh::build(&f)?),',
         '"mesh": if gpu_only { Value::Null } else { json!(digest(&mesh::build(&f)?)) },'),
        ('"gpuResident": prepared(&resident, &resident.prepare(&f, Delivery::Resident)?)?,',
         '"gpuResident": prepared(&resident, &resident.prepare(&f, Delivery::Resident)?)?,\n'
         '                "secs": t0.elapsed().as_secs_f64(),'),
    ]:
        assert a in s, a
        s = s.replace(a, b)
    return s


states = [("s0", "7f0cade5"), ("s1", "1c3ccb93"), ("s2", "f83e03f6"), ("s3", "471fb1d9"), ("s4", "884a0bb7")]
try:
    for name, commit in states:
        subprocess.run(["git", "checkout", "-q", commit], check=True)
        shutil.copy(f"{R}/tools/f212_time.rs", f"{EX}/f212_time.rs")
        examples = ["--example", "f212_time"]
        if name == "s4":
            open(f"{EX}/f212_digest.rs", "w").write(digest_source())
            examples += ["--example", "f212_digest"]
        subprocess.run(["cargo", "build", "--profile", "ci", "-p", "telperion-render", *examples],
                       check=True, capture_output=True, env=env)
        shutil.copy(f"{W}/target/ci/examples/f212_time", f"{R}/bin/time-{name}")
        if name == "s4":
            shutil.copy(f"{W}/target/ci/examples/f212_digest", f"{R}/bin/digest-s4")
        for f in ("f212_time.rs", "f212_digest.rs"):
            if os.path.exists(f"{EX}/{f}"):
                os.remove(f"{EX}/{f}")
        print(name, "built", flush=True)
finally:
    for f in ("f212_time.rs", "f212_digest.rs"):
        if os.path.exists(f"{EX}/{f}"):
            os.remove(f"{EX}/{f}")
    subprocess.run(["git", "checkout", "-q", "fn-212-tree-space-fine-geometry-dressed-on-the"], check=True)
