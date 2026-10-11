//! Two of Tint's rules that naga does not hold (the owner's Chrome probe,
//! 2026-10-11; DESIGN.md section 7b), checked on the cycle module's WGSL
//! until a Tint check runs in the shader tests: no NaN or Inf `f32`
//! constant, and every barrier under uniform control. The uniformity
//! check is conservative: a value is uniform only when built from
//! literals, constants, overrides, uniform buffers, `workgroup_id`,
//! `num_workgroups` and what `workgroupUniformLoad` returned; every
//! storage read, read-only ones too, is taken as non-uniform.
use std::collections::{HashMap, HashSet};

/// What a source breaks, one line a finding.
pub(super) fn findings(source: &str) -> Vec<String> {
    let toks = tokens(&strip(source));
    let mut out = nan_constants(&toks);
    let globals = uniform_globals(&toks);
    let fns = functions(&toks);
    let barred = barred(&fns);
    let returning = uniform_returning(&fns, &globals);
    for f in &fns {
        let mut walk = Walk {
            name: &f.name,
            toks: &f.body,
            uniform: globals.clone(),
            returning: &returning,
            barred: &barred,
            out: &mut out,
            stopped: false,
        };
        walk.uniform.extend(f.uniform_params.iter().cloned());
        walk.block(0, f.body.len(), true);
    }
    out
}

const BARRIERS: [&str; 4] = ["workgroupBarrier", "storageBarrier", "textureBarrier", "workgroupUniformLoad"];
const PURE: [&str; 12] = ["min", "max", "select", "u32", "i32", "f32", "vec2", "vec3", "vec4", "clamp", "array", "bitcast"];

fn strip(s: &str) -> String {
    s.lines().map(|l| l.split("//").next().unwrap_or("")).collect::<Vec<_>>().join("\n")
}

fn tokens(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
        } else if c.is_alphanumeric() || c == '_' {
            let start = i;
            while i < cs.len() && (cs[i].is_alphanumeric() || cs[i] == '_' || cs[i] == '.' && cs[start].is_ascii_digit()) {
                i += 1;
            }
            out.push(cs[start..i].iter().collect());
        } else {
            out.push(c.to_string());
            i += 1;
        }
    }
    out
}

fn is_ident(t: &str) -> bool {
    t.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_')
}

/// Every `bitcast<f32>` of a literal or a constant whose bits are a NaN
/// or an infinity: a constant expression, which Tint evaluates and
/// refuses wherever it stands.
fn nan_constants(toks: &[String]) -> Vec<String> {
    let mut values: HashMap<&str, u64> = HashMap::new();
    for k in 0..toks.len() {
        if toks[k] == "const" && k + 1 < toks.len() {
            let end = (k..toks.len()).find(|&j| toks[j] == ";").unwrap_or(toks.len());
            if let Some(v) = toks[k..end].iter().find_map(|t| literal(t)) {
                values.entry(toks[k + 1].as_str()).or_insert(v);
            }
        }
    }
    let mut out = Vec::new();
    for k in 0..toks.len().saturating_sub(5) {
        let cast = toks[k] == "bitcast" && toks[k + 1] == "<" && toks[k + 2] == "f32" && toks[k + 4] == "(";
        if !cast || toks.get(k + 6).is_none_or(|t| t != ")") {
            continue;
        }
        let arg = toks[k + 5].as_str();
        let bits = literal(arg).or_else(|| values.get(arg).copied());
        if bits.is_some_and(|v| (v as u32 >> 23) & 0xff == 0xff) {
            out.push(format!("bitcast<f32>({arg}): a NaN or infinite f32 constant"));
        }
    }
    out
}

fn literal(t: &str) -> Option<u64> {
    let t = t.trim_end_matches(['u', 'i']);
    match t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        Some(h) => u64::from_str_radix(h, 16).ok(),
        None => t.parse().ok(),
    }
}

/// Names uniform at module scope: constants, overrides and uniform buffers.
fn uniform_globals(toks: &[String]) -> HashSet<String> {
    let mut out = HashSet::new();
    let mut depth = 0i32;
    for k in 0..toks.len() {
        match toks[k].as_str() {
            "{" => depth += 1,
            "}" => depth -= 1,
            "const" | "override" if depth == 0 && k + 1 < toks.len() => {
                out.insert(toks[k + 1].clone());
            }
            "var" if depth == 0 && toks.get(k + 1).is_some_and(|t| t == "<") && toks.get(k + 2).is_some_and(|t| t == "uniform") => {
                if let Some(j) = (k..toks.len()).find(|&j| toks[j] == ">") {
                    out.insert(toks[j + 1].clone());
                }
            }
            _ => {}
        }
    }
    out
}

struct Function {
    name: String,
    uniform_params: Vec<String>,
    body: Vec<String>,
}

/// Every function: its name, the parameters that are uniform builtins,
/// and its body's tokens between its braces.
fn functions(toks: &[String]) -> Vec<Function> {
    let mut out = Vec::new();
    let mut k = 0;
    while k < toks.len() {
        if toks[k] != "fn" {
            k += 1;
            continue;
        }
        let name = toks[k + 1].clone();
        let open = (k..toks.len()).find(|&j| toks[j] == "(").expect("parameters");
        let close = matching(toks, open, "(", ")");
        let mut uniform_params = Vec::new();
        for j in open..close {
            if toks[j] == "builtin" && matches!(toks[j + 2].as_str(), "workgroup_id" | "num_workgroups") {
                uniform_params.push(toks[j + 4].clone());
            }
        }
        let start = (close..toks.len()).find(|&j| toks[j] == "{").expect("a body");
        let end = matching(toks, start, "{", "}");
        out.push(Function { name, uniform_params, body: toks[start + 1..end].to_vec() });
        k = end + 1;
    }
    out
}

fn matching(toks: &[String], open: usize, a: &str, b: &str) -> usize {
    let mut depth = 0;
    for (j, t) in toks.iter().enumerate().skip(open) {
        if t == a {
            depth += 1;
        } else if t == b {
            depth -= 1;
            if depth == 0 {
                return j;
            }
        }
    }
    toks.len()
}

/// Functions that hold a barrier, themselves or through a call.
fn barred(fns: &[Function]) -> HashSet<String> {
    let mut out: HashSet<String> = BARRIERS.iter().map(|s| s.to_string()).collect();
    loop {
        let before = out.len();
        for f in fns {
            if f.body.iter().any(|t| out.contains(t)) {
                out.insert(f.name.clone());
            }
        }
        if out.len() == before {
            return out;
        }
    }
}

/// Functions whose every return is `workgroupUniformLoad`'s, a uniform
/// global's or a literal's (their parameters taken as non-uniform).
fn uniform_returning(fns: &[Function], globals: &HashSet<String>) -> HashSet<String> {
    let mut out = HashSet::new();
    for f in fns {
        let returns: Vec<usize> = (0..f.body.len()).filter(|&j| f.body[j] == "return").collect();
        let uniform = !returns.is_empty()
            && returns.iter().all(|&j| {
                let end = (j..f.body.len()).find(|&e| f.body[e] == ";").unwrap_or(f.body.len());
                let e = &f.body[j + 1..end];
                e.first().is_some_and(|t| t == "workgroupUniformLoad")
                    || e.iter().filter(|t| is_ident(t)).all(|t| globals.contains(t) || PURE.contains(&t.as_str()))
            });
        if uniform {
            out.insert(f.name.clone());
        }
    }
    out
}

struct Walk<'a> {
    name: &'a str,
    toks: &'a [String],
    uniform: HashSet<String>,
    returning: &'a HashSet<String>,
    barred: &'a HashSet<String>,
    out: &'a mut Vec<String>,
    /// A return, break or continue under non-uniform control was passed.
    stopped: bool,
}

impl Walk<'_> {
    /// Whether the expression `toks[a..b]` is uniform.
    fn uniform(&self, a: usize, b: usize) -> bool {
        let t = &self.toks[a..b];
        let mut j = 0;
        while j < t.len() {
            let x = t[j].as_str();
            if x == "workgroupUniformLoad" {
                j = matching(t, j + 1, "(", ")") + 1;
                continue;
            }
            if self.returning.contains(x) {
                j = matching(t, j + 1, "(", ")") + 1;
                continue;
            }
            let field = j > 0 && t[j - 1] == ".";
            let call = t.get(j + 1).is_some_and(|n| n == "(" || n == "<");
            let fine = !is_ident(x)
                || field
                || self.uniform.contains(x)
                || call && PURE.contains(&x)
                || matches!(x, "true" | "false" | "u" | "f" | "i");
            if !fine {
                return false;
            }
            j += 1;
        }
        true
    }

    /// The statements of `toks[a..b]`, under control `control`.
    fn block(&mut self, a: usize, b: usize, control: bool) {
        let mut k = a;
        while k < b {
            k = self.statement(k, b, control);
        }
    }

    /// One statement from `k`; returns the index after it.
    fn statement(&mut self, k: usize, b: usize, control: bool) -> usize {
        let t = self.toks[k].as_str();
        match t {
            "if" | "while" => {
                let open = (k..b).find(|&j| self.toks[j] == "{").expect("a block");
                let c = control && self.uniform(k + 1, open);
                let close = matching(self.toks, open, "{", "}");
                self.block(open + 1, close, c);
                let mut next = close + 1;
                if self.toks.get(next).is_some_and(|x| x == "else") {
                    if self.toks.get(next + 1).is_some_and(|x| x == "if") {
                        return self.statement(next + 1, b, c);
                    }
                    let close = matching(self.toks, next + 1, "{", "}");
                    self.block(next + 2, close, c);
                    next = close + 1;
                }
                next
            }
            "for" => self.for_loop(k, control),
            "loop" | "{" => {
                let open = (k..b).find(|&j| self.toks[j] == "{").expect("a block");
                let close = matching(self.toks, open, "{", "}");
                self.block(open + 1, close, control);
                close + 1
            }
            _ => self.simple(k, b, control),
        }
    }

    fn for_loop(&mut self, k: usize, control: bool) -> usize {
        let open = k + 1;
        let close = matching(self.toks, open, "(", ")");
        let semis: Vec<usize> = (open..close).filter(|&j| self.toks[j] == ";").collect();
        let (s1, s2) = (semis[0], semis[1]);
        self.simple(open + 1, s1 + 1, control);
        let c = control && self.uniform(s1 + 1, s2) && self.uniform(s2 + 1, close);
        if !c {
            for j in open + 1..s1 {
                self.uniform.remove(&self.toks[j]);
            }
        }
        let body = matching(self.toks, close + 1, "{", "}");
        self.block(close + 2, body, c);
        body + 1
    }

    /// A declaration, assignment, call or jump, up to its `;`.
    fn simple(&mut self, k: usize, b: usize, control: bool) -> usize {
        let end = (k..b).find(|&j| self.toks[j] == ";").unwrap_or(b);
        let s = &self.toks[k..end];
        if s.iter().any(|t| self.barred.contains(t)) && !(control && !self.stopped) {
            let what = s.iter().find(|t| self.barred.contains(*t)).expect("a barrier");
            self.out.push(format!("fn {}: {what} under non-uniform control", self.name));
        }
        if matches!(s.first().map(String::as_str), Some("return" | "break" | "continue")) && !control {
            self.stopped = true;
        }
        let decl = matches!(s.first().map(String::as_str), Some("let" | "var"));
        let eq = s.iter().position(|t| t == "=");
        if let Some(e) = eq {
            let name = if decl { s.get(1) } else { s.first() };
            if let Some(name) = name.filter(|n| is_ident(n)) {
                let assigned = !decl || s.get(2).is_some_and(|t| t == ":" || t == "=");
                let u = control && self.uniform(k + e + 1, end);
                let compound = e > 0 && matches!(s[e - 1].as_str(), "+" | "-" | "*" | "/" | "|" | "&" | "^");
                if assigned && u && (decl || !compound || self.uniform.contains(name.as_str())) {
                    self.uniform.insert(name.clone());
                } else {
                    self.uniform.remove(name.as_str());
                }
            }
        } else if s.len() >= 2 && (s[s.len() - 1] == "+" && s[s.len() - 2] == "+" || s[s.len() - 1] == "-" && s[s.len() - 2] == "-") && !control {
            self.uniform.remove(&s[0]);
        }
        end + 1
    }
}

#[cfg(test)]
mod tests {
    use super::findings;

    #[test]
    fn a_nan_constant_is_refused_and_a_finite_one_kept() {
        let nan = "const NAN_BITS: u32 = 0x7fc00000u;\nconst NAN: f32 = bitcast<f32>(NAN_BITS);";
        assert_eq!(findings(nan).len(), 1);
        let inf = "const INF: f32 = bitcast<f32>(0x7f800000u);";
        assert_eq!(findings(inf).len(), 1);
        // The spike's shed module, refused by Chrome: a constant expression.
        let shed = "const NAN_BITS: u32 = 0x7fc00000u;\nfn f() { var memory = bitcast<f32>(NAN_BITS); }";
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

    /// Every composed module of the cycle executor, both arithmetics.
    #[test]
    fn the_cycle_modules_shaders_keep_tints_rules() {
        let mut all = Vec::new();
        for (name, source) in super::super::sources() {
            all.extend(findings(&source).into_iter().map(|f| format!("{name}: {f}")));
        }
        assert!(all.is_empty(), "Tint would refuse:\n{}", all.join("\n"));
    }
}
