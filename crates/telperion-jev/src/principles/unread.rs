//! Class 3: output nothing reads. A shader input the change declared that the
//! shader never reads, with the Rust lines that fill its buffer; and a public
//! output field the change added, holding data (a vector, a slice or a
//! buffer), that production code reads at most once, with its reads and its
//! mentions in the shaders and the browser source. Code counts; Jev judges
//! whether one of them is a use.

use std::collections::BTreeMap;

use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::Visit;

use super::candidates::{Candidate, Class, Context};
use super::modtree::{is_test, RustFile};

const CONSUMER_EXTENSIONS: &[&str] = &["wgsl", "ts", "tsx"];

pub fn find(ctx: &Context, files: &[RustFile]) -> Vec<Candidate> {
    let mut out = shader_inputs(ctx);
    out.extend(fields(ctx, files));
    out
}

/// A vertex input a shader the change touched declares and never reads:
/// the buffer behind it is uploaded for nothing.
fn shader_inputs(ctx: &Context) -> Vec<Candidate> {
    let mut out = Vec::new();
    for (path, added) in &ctx.change.added {
        let Some(text) = ctx.after.get(path).filter(|_| path.ends_with(".wgsl")) else { continue };
        let code: Vec<(usize, &str)> = text
            .lines()
            .enumerate()
            .map(|(i, l)| (i + 1, l.split("//").next().unwrap_or("")))
            .collect();
        for (n, line) in &code {
            let Some(rest) = line.trim_start().strip_prefix("@location(") else { continue };
            let Some(name) = rest.split_once(')').and_then(|(_, r)| r.split(':').next()).map(str::trim) else { continue };
            if name.is_empty() || !added.contains(n) {
                continue;
            }
            let used = code.iter().any(|(m, l)| m != n && l.split(|c: char| !c.is_alphanumeric() && c != '_').any(|w| w == name));
            if used {
                continue;
            }
            let mut evidence = Vec::new();
            evidence.extend(ctx.evidence("after", path, (n.saturating_sub(5), n + 1)));
            for (p, l) in uploads(ctx, name).into_iter().take(2) {
                evidence.extend(ctx.evidence("after", &p, (l.saturating_sub(1), l + 1)));
            }
            out.push(Candidate {
                id: String::new(),
                class: Class::Unread,
                location: format!("{path}:{n}"),
                shape: format!("shader input `{name}` is declared and read nowhere in `{path}`"),
                weight: 2.5,
                evidence,
            });
        }
    }
    out
}

/// Rust lines of the change's head that name `name` or its plural.
fn uploads(ctx: &Context, name: &str) -> Vec<(String, usize)> {
    let plural = format!("{name}s");
    let mut out = Vec::new();
    for (path, text) in ctx.after.files.iter().filter(|(p, _)| p.ends_with(".rs") && ctx.change.added.contains_key(*p)) {
        for (i, line) in text.lines().enumerate() {
            if line.split(|c: char| !c.is_alphanumeric() && c != '_').any(|w| w == name || w == plural) {
                out.push((path.clone(), i + 1));
            }
        }
    }
    out
}

fn fields(ctx: &Context, files: &[RustFile]) -> Vec<Candidate> {
    let mut fields = Vec::new();
    for file in files.iter().filter(|f| ctx.change.added.contains_key(&f.path)) {
        let mut v = Fields { file, out: &mut fields };
        v.visit_file(&file.ast);
    }
    let fields: Vec<_> = fields
        .into_iter()
        .filter(|(path, line, ..)| ctx.change.added[path].contains(line))
        .collect();
    if fields.is_empty() {
        return Vec::new();
    }
    let mut reads: BTreeMap<String, Vec<(String, usize)>> = BTreeMap::new();
    for file in files {
        let mut v = Reads { file, wanted: &fields, out: &mut reads };
        v.visit_file(&file.ast);
    }
    let mut out = Vec::new();
    for (path, line, name, owner) in &fields {
        let rust = reads.get(name).cloned().unwrap_or_default();
        if rust.len() > 1 {
            continue;
        }
        let mut evidence = Vec::new();
        evidence.extend(ctx.evidence("after", path, (line.saturating_sub(1), line + 1)));
        for (p, l) in rust.iter().take(2) {
            evidence.extend(ctx.evidence("after", p, (l.saturating_sub(1), l + 1)));
        }
        for (p, l) in consumers(ctx, name).into_iter().take(2) {
            evidence.extend(ctx.evidence("after", &p, (l.saturating_sub(1), l + 1)));
        }
        out.push(Candidate {
            id: String::new(),
            class: Class::Unread,
            location: format!("{path}:{line}"),
            shape: format!("new output `{owner}.{name}` is read {} time(s) in production Rust", rust.len()),
            weight: 2.0 - rust.len() as f64 * 0.5,
            evidence,
        });
    }
    out
}

/// Lines in the shaders and the browser source naming `name` or its stem.
fn consumers(ctx: &Context, name: &str) -> Vec<(String, usize)> {
    let stem = name.trim_end_matches('s');
    let mut out = Vec::new();
    for (path, text) in &ctx.after.files {
        let ext = path.rsplit('.').next().unwrap_or("");
        if !CONSUMER_EXTENSIONS.contains(&ext) || path.contains(".test.") {
            continue;
        }
        for (i, line) in text.lines().enumerate() {
            if line.split(|c: char| !c.is_alphanumeric() && c != '_').any(|w| w == name || w == stem) {
                out.push((path.clone(), i + 1));
            }
        }
    }
    out
}

struct Fields<'a> {
    file: &'a RustFile,
    out: &'a mut Vec<(String, usize, String, String)>,
}

impl<'ast> Visit<'ast> for Fields<'_> {
    fn visit_item_mod(&mut self, m: &'ast syn::ItemMod) {
        if !is_test(&m.attrs) {
            syn::visit::visit_item_mod(self, m);
        }
    }
    fn visit_item_struct(&mut self, s: &'ast syn::ItemStruct) {
        if is_test(&s.attrs) || !matches!(s.vis, syn::Visibility::Public(_)) {
            return;
        }
        for f in &s.fields {
            let Some(ident) = &f.ident else { continue };
            let ty = f.ty.to_token_stream().to_string();
            let data = ty.contains("Vec") || ty.starts_with('[') || ty.contains("Box < [");
            if data && matches!(f.vis, syn::Visibility::Public(_)) {
                let line = ident.span().start().line;
                self.out.push((self.file.path.clone(), line, ident.to_string(), s.ident.to_string()));
            }
        }
    }
}

struct Reads<'a> {
    file: &'a RustFile,
    wanted: &'a [(String, usize, String, String)],
    out: &'a mut BTreeMap<String, Vec<(String, usize)>>,
}

impl<'ast> Visit<'ast> for Reads<'_> {
    fn visit_item_mod(&mut self, m: &'ast syn::ItemMod) {
        if !is_test(&m.attrs) {
            syn::visit::visit_item_mod(self, m);
        }
    }
    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        if !is_test(&f.attrs) {
            syn::visit::visit_item_fn(self, f);
        }
    }
    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        if !is_test(&f.attrs) {
            syn::visit::visit_impl_item_fn(self, f);
        }
    }
    fn visit_expr_assign(&mut self, a: &'ast syn::ExprAssign) {
        // The target of an assignment is a write, not a read.
        if !matches!(&*a.left, syn::Expr::Field(_)) {
            self.visit_expr(&a.left);
        }
        self.visit_expr(&a.right);
    }
    fn visit_expr_method_call(&mut self, m: &'ast syn::ExprMethodCall) {
        // Filling a buffer is a write: `self.coords.push(..)` reads nothing.
        let fills = ["push", "extend", "extend_from_slice", "resize", "reserve", "clear", "insert"];
        match &*m.receiver {
            syn::Expr::Field(f) if fills.contains(&m.method.to_string().as_str()) => {
                self.visit_expr(&f.base);
            }
            other => self.visit_expr(other),
        }
        for arg in &m.args {
            self.visit_expr(arg);
        }
    }
    fn visit_expr_field(&mut self, e: &'ast syn::ExprField) {
        if let syn::Member::Named(name) = &e.member {
            let name = name.to_string();
            if self.wanted.iter().any(|w| w.2 == name) {
                let at = (self.file.path.clone(), e.span().start().line);
                self.out.entry(name).or_default().push(at);
            }
        }
        syn::visit::visit_expr_field(self, e);
    }
}
