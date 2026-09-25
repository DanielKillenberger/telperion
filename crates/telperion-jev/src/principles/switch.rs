//! Class 1: a branch the change added whose arms run different builders, or
//! whose condition reads a setting and one arm drops the work the rest does
//! (a suppression, or a stop). Dormancy and count steps take this shape too;
//! the confirming questions tell them apart, never this extractor. An arm
//! that returns an error is validation and is left out.

use std::collections::BTreeSet;

use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::Visit;

use super::candidates::{Candidate, Class, Context};
use super::index::Function;
use super::modtree::{is_test, RustFile};

pub fn find(ctx: &Context, files: &[RustFile], base_fns: &[Function]) -> Vec<Candidate> {
    let mut out = Vec::new();
    for file in files {
        if !ctx.change.added.contains_key(&file.path) {
            continue;
        }
        let mut v = Branches { ctx, file, found: Vec::new(), function: None, args: BTreeSet::new() };
        v.visit_file(&file.ast);
        for (line_span, shape, weight, function) in v.found {
            let mut evidence = Vec::new();
            evidence.extend(ctx.evidence("after", &file.path, line_span));
            let before = function
                .as_ref()
                .and_then(|name| base_fns.iter().find(|f| f.path == file.path && f.symbol.ends_with(name)));
            match before {
                Some(f) => evidence.extend(ctx.evidence("before", &f.path, f.lines)),
                None => evidence.push(super::candidates::Evidence {
                    id: String::new(),
                    side: "before".into(),
                    path: file.path.clone(),
                    lines: (0, 0),
                    text: "(the enclosing function is new in this change)".into(),
                    added: false,
                }),
            }
            out.push(Candidate {
                id: String::new(),
                class: Class::Switch,
                location: format!("{}:{}", file.path, line_span.0),
                shape,
                weight,
                evidence,
            });
        }
    }
    out
}

struct Branches<'a> {
    ctx: &'a Context<'a>,
    file: &'a RustFile,
    found: Vec<((usize, usize), String, f64, Option<String>)>,
    function: Option<String>,
    /// The enclosing function's argument names: a condition on one of them
    /// reads a setting the caller handed in, not the builder's own state.
    args: BTreeSet<String>,
}

fn arg_names(sig: &syn::Signature) -> BTreeSet<String> {
    sig.inputs
        .iter()
        .filter_map(|a| match a {
            syn::FnArg::Typed(t) => match &*t.pat {
                syn::Pat::Ident(i) => Some(i.ident.to_string()),
                _ => None,
            },
            syn::FnArg::Receiver(_) => None,
        })
        .collect()
}

impl Branches<'_> {
    fn consider(&mut self, span: proc_macro2::Span, cond: &syn::Expr, arms: Vec<Vec<String>>, exits: Vec<bool>) {
        let lines = (span.start().line, span.end().line);
        let added = self.ctx.change.added.get(&self.file.path);
        if !added.is_some_and(|set| set.contains(&lines.0)) {
            return;
        }
        let sets: Vec<BTreeSet<&String>> = arms.iter().map(|a| a.iter().collect()).collect();
        let distinct = sets.len() >= 2
            && sets.iter().all(|s| !s.is_empty())
            && sets.windows(2).any(|w| overlap(&w[0], &w[1]) < 0.5);
        let drops = exits.iter().any(|&e| e) && reads_setting(cond);
        let (shape, weight) = match (distinct, drops) {
            (true, _) => ("arms run different builders on a setting", 3.0),
            (false, true) => ("an arm drops the work the other arm or the rest of the function does", 2.0),
            _ => return,
        };
        let tokens = super::index::fold(cond.to_token_stream());
        let weight = if tokens.iter().any(|t| self.args.contains(t)) { weight + 0.5 } else { weight };
        let condition = cond.to_token_stream().to_string();
        let shape = format!("{shape}: `{}`", condition.chars().take(120).collect::<String>());
        self.found.push((lines, shape, weight, self.function.clone()));
    }
}

impl<'ast> Visit<'ast> for Branches<'_> {
    fn visit_item_mod(&mut self, m: &'ast syn::ItemMod) {
        if !is_test(&m.attrs) {
            syn::visit::visit_item_mod(self, m);
        }
    }
    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        if !is_test(&f.attrs) {
            self.function = Some(f.sig.ident.to_string());
            self.args = arg_names(&f.sig);
            syn::visit::visit_item_fn(self, f);
        }
    }
    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        if !is_test(&f.attrs) {
            self.function = Some(f.sig.ident.to_string());
            self.args = arg_names(&f.sig);
            syn::visit::visit_impl_item_fn(self, f);
        }
    }
    fn visit_expr_if(&mut self, e: &'ast syn::ExprIf) {
        let mut arms = vec![calls(&e.then_branch.to_token_stream())];
        let mut leaves = vec![exits(&e.then_branch.to_token_stream())];
        if let Some((_, other)) = &e.else_branch {
            arms.push(calls(&other.to_token_stream()));
            leaves.push(exits(&other.to_token_stream()));
        }
        self.consider(e.span(), &e.cond, arms, leaves);
        syn::visit::visit_expr_if(self, e);
    }
    fn visit_expr_match(&mut self, e: &'ast syn::ExprMatch) {
        let arms = e.arms.iter().map(|a| calls(&a.body.to_token_stream())).collect();
        let exits = e.arms.iter().map(|a| exits(&a.body.to_token_stream())).collect();
        self.consider(e.span(), &e.expr, arms, exits);
        syn::visit::visit_expr_match(self, e);
    }
}

fn overlap(a: &BTreeSet<&String>, b: &BTreeSet<&String>) -> f64 {
    let union = a.union(b).count();
    if union == 0 { 1.0 } else { a.intersection(b).count() as f64 / union as f64 }
}

/// A condition reads a setting when it reads a field or passes a value to a
/// call; a bare local test such as `x.is_empty()` does not.
fn reads_setting(cond: &syn::Expr) -> bool {
    struct Reads(bool);
    impl<'ast> Visit<'ast> for Reads {
        fn visit_expr_field(&mut self, _: &'ast syn::ExprField) {
            self.0 = true;
        }
        fn visit_expr_call(&mut self, c: &'ast syn::ExprCall) {
            if !c.args.is_empty() {
                self.0 = true;
            }
            syn::visit::visit_expr_call(self, c);
        }
    }
    let mut r = Reads(false);
    r.visit_expr(cond);
    r.0
}

const TRIVIAL: &[&str] = &[
    "Ok", "Err", "Some", "None", "clone", "into", "push", "len", "iter", "map", "collect", "unwrap",
    "expect", "to_string", "as_ref", "from", "new", "default", "extend", "insert", "get", "min", "max",
];

/// Callees a block names, as written with their paths (`rosette::count`),
/// trivial constructors and adapters left out.
fn calls(stream: &proc_macro2::TokenStream) -> Vec<String> {
    let tokens = super::index::fold(stream.clone());
    let mut out = Vec::new();
    for i in 1..tokens.len() {
        let name = &tokens[i - 1];
        if tokens[i] != "(" || !name.chars().next().is_some_and(char::is_alphabetic) {
            continue;
        }
        let mut path = vec![name.clone()];
        let mut j = i - 1;
        while j >= 3 && tokens[j - 1] == ":" && tokens[j - 2] == ":" {
            path.insert(0, tokens[j - 3].clone());
            j -= 3;
        }
        if path.len() > 1 || !TRIVIAL.contains(&name.as_str()) {
            out.push(path.join("::"));
        }
    }
    out
}

/// A block that leaves early or clears what was built, where the way out is
/// not an error: an error return is validation, never a switch.
fn exits(stream: &proc_macro2::TokenStream) -> bool {
    let tokens = super::index::fold(stream.clone());
    let words = ["return", "continue", "break", "clear", "retain", "truncate"];
    tokens.iter().any(|t| words.contains(&t.as_str())) && !tokens.iter().any(|t| t == "Err")
}
