//! Guard 1, the production boundary: production code outside the pipeline
//! never calls a build stage. Aliases and re-exports are resolved, test code
//! is never read, a registered exception covers its named callers and stages,
//! and a push is judged by the callers it adds net of those it removes.

use std::collections::{BTreeMap, BTreeSet};

use syn::visit::Visit;

use super::modtree::{is_test, production, ParseError, RustFile};
use super::policy::{Boundary, Exception};
use super::resolve::{absolute, uses, Exports};
use super::source::Snapshot;

/// One production reference to a build stage.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Caller {
    pub file: String,
    pub line: usize,
    /// The enclosing function, e.g. `telperion_wasm::generate::build`.
    pub symbol: String,
    /// The stage's canonical path, e.g. `telperion_core::branching::generate`.
    pub stage: String,
}

/// An import the guard cannot follow to a definite item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unresolved {
    pub file: String,
    pub line: usize,
    pub what: String,
}

#[derive(Debug, Default)]
pub struct Scan {
    /// References no pipeline module, stage module or exception accounts for.
    pub violations: Vec<Caller>,
    /// References an exception covers, with its id.
    pub excepted: Vec<(Caller, String)>,
    pub unresolved: Vec<Unresolved>,
    pub parse_errors: Vec<ParseError>,
}

impl Scan {
    pub fn passes(&self) -> bool {
        self.violations.is_empty() && self.unresolved.is_empty() && self.parse_errors.is_empty()
    }
}

/// Every stage reference in the snapshot's production code, classified.
pub fn scan(snapshot: &Snapshot, rules: &Boundary, exceptions: &[Exception]) -> Scan {
    let (files, parse_errors) = production(snapshot);
    let crates: BTreeSet<String> = files.iter().map(|f| f.module[0].clone()).collect();
    let exports = Exports::of(&files, &crates);
    let split = |p: &String| p.split("::").map(str::to_string).collect::<Vec<_>>();
    let stages: BTreeSet<Vec<String>> = rules
        .stages
        .iter()
        .map(|s| exports.canonical(&split(s)))
        .collect();
    let bearing: BTreeSet<Vec<String>> = stages.iter().map(|s| s[..s.len() - 1].to_vec()).collect();
    let exceptions: Vec<Exception> = exceptions
        .iter()
        .cloned()
        .map(|mut e| {
            e.stages = e.stages.iter().map(|s| exports.canonical(&split(s)).join("::")).collect();
            e
        })
        .collect();
    let mut out = Scan { parse_errors, ..Scan::default() };
    for file in &files {
        let mut refs = References::new(file, &crates, &exports, &stages, &bearing);
        refs.visit_file(&file.ast);
        out.unresolved.extend(refs.unresolved);
        for caller in refs.found {
            if rules.allows(&caller) {
                continue;
            }
            match exceptions.iter().find(|e| e.covers(&caller.symbol, &caller.stage)) {
                Some(e) => out.excepted.push((caller, e.id.clone())),
                None => out.violations.push(caller),
            }
        }
    }
    out.violations.sort();
    out.violations.dedup();
    out
}

/// Violations `head` adds over `base`. A caller moved within its crate is
/// matched against the one removed, so a move is not an addition.
pub fn added(base: &Scan, head: &Scan) -> Vec<Caller> {
    let key = |c: &Caller| (c.symbol.split("::").next().unwrap_or("").to_string(), c.stage.clone());
    let mut before: BTreeMap<_, usize> = BTreeMap::new();
    let known: BTreeSet<_> = base.violations.iter().map(|c| (c.symbol.clone(), c.stage.clone())).collect();
    for c in &base.violations {
        *before.entry(key(c)).or_default() += 1;
    }
    let mut after: BTreeMap<_, Vec<&Caller>> = BTreeMap::new();
    for c in &head.violations {
        after.entry(key(c)).or_default().push(c);
    }
    let mut out = Vec::new();
    for (k, callers) in after {
        let excess = callers.len().saturating_sub(before.get(&k).copied().unwrap_or(0));
        let fresh = callers
            .into_iter()
            .filter(|c| !known.contains(&(c.symbol.clone(), c.stage.clone())));
        out.extend(fresh.take(excess).cloned());
    }
    out
}

/// False when `file` sits in the module that defines `target`: a stage
/// module's own glob of itself is internal.
fn outside_home(file: &RustFile, target: &[String]) -> bool {
    file.module.iter().take(2).ne(target.iter().take(2))
}

struct References<'a> {
    file: &'a RustFile,
    crates: &'a BTreeSet<String>,
    exports: &'a Exports,
    stages: &'a BTreeSet<Vec<String>>,
    bearing: &'a BTreeSet<Vec<String>>,
    aliases: BTreeMap<String, Vec<String>>,
    module: Vec<String>,
    symbol: Vec<String>,
    found: Vec<Caller>,
    unresolved: Vec<Unresolved>,
}

impl<'a> References<'a> {
    fn new(
        file: &'a RustFile,
        crates: &'a BTreeSet<String>,
        exports: &'a Exports,
        stages: &'a BTreeSet<Vec<String>>,
        bearing: &'a BTreeSet<Vec<String>>,
    ) -> Self {
        let own = uses(&file.ast.items, &file.module, crates);
        let (aliases, _) = exports.aliases(&file.module, &own);
        let mut unresolved = Vec::new();
        for glob in own.iter().filter(|l| l.alias.is_none()) {
            let target = exports.canonical(&glob.target);
            if bearing.contains(&target) && outside_home(file, &target) {
                unresolved.push(Unresolved {
                    file: file.path.clone(),
                    line: glob.line,
                    what: format!("glob import of {}", target.join("::")),
                });
            }
        }
        Self {
            file,
            crates,
            exports,
            stages,
            bearing,
            aliases,
            module: file.module.clone(),
            symbol: Vec::new(),
            found: Vec::new(),
            unresolved,
        }
    }

    fn resolve(&self, path: &syn::Path) -> Vec<String> {
        let segments: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
        self.exports
            .canonical(&absolute(&segments, &self.module, &self.aliases, self.crates))
    }

    fn enclosing(&self) -> String {
        let mut s = self.module.clone();
        s.extend(self.symbol.iter().cloned());
        s.join("::")
    }

    fn function(&mut self, name: String, attrs: &[syn::Attribute], body: impl FnOnce(&mut Self)) {
        if is_test(attrs) {
            return;
        }
        self.symbol.push(name);
        body(self);
        self.symbol.pop();
    }
}

impl<'ast> Visit<'ast> for References<'_> {
    fn visit_item_mod(&mut self, m: &'ast syn::ItemMod) {
        if is_test(&m.attrs) || m.content.is_none() {
            return;
        }
        self.module.push(m.ident.to_string());
        syn::visit::visit_item_mod(self, m);
        self.module.pop();
    }

    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        self.function(f.sig.ident.to_string(), &f.attrs, |v| syn::visit::visit_item_fn(v, f));
    }

    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        if is_test(&i.attrs) {
            return;
        }
        let name = match &*i.self_ty {
            syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()),
            _ => None,
        };
        self.symbol.push(name.unwrap_or_else(|| "impl".into()));
        syn::visit::visit_item_impl(self, i);
        self.symbol.pop();
    }

    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        self.function(f.sig.ident.to_string(), &f.attrs, |v| syn::visit::visit_impl_item_fn(v, f));
    }

    fn visit_item_type(&mut self, t: &'ast syn::ItemType) {
        if let syn::Type::Path(p) = &*t.ty {
            let target = self.resolve(&p.path);
            if self.bearing.contains(&target) || self.stages.iter().any(|s| s.starts_with(&target)) {
                self.unresolved.push(Unresolved {
                    file: self.file.path.clone(),
                    line: t.ident.span().start().line,
                    what: format!("type alias {} of {}", t.ident, target.join("::")),
                });
            }
        }
    }

    fn visit_expr_path(&mut self, e: &'ast syn::ExprPath) {
        let target = self.resolve(&e.path);
        if self.stages.contains(&target) {
            let line = e.path.segments.last().map_or(0, |s| s.ident.span().start().line);
            self.found.push(Caller {
                file: self.file.path.clone(),
                line,
                symbol: self.enclosing(),
                stage: target.join("::"),
            });
        }
    }
}
