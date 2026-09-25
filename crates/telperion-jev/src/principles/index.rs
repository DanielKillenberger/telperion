//! Every production function with its span, its tokens and its doc text, so
//! the extractors can compare bodies and quote the lines they cite.

use std::collections::BTreeSet;

use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::Visit;

use super::modtree::{is_test, RustFile};

#[derive(Debug, Clone)]
pub struct Function {
    pub path: String,
    pub symbol: String,
    pub lines: (usize, usize),
    /// Identifiers and punctuation with literals folded, docs included.
    pub tokens: Vec<String>,
    pub shingles: BTreeSet<u64>,
}

pub fn functions(files: &[RustFile]) -> Vec<Function> {
    let mut out = Vec::new();
    for file in files {
        let mut v = Collect { file, module: file.module.clone(), owner: Vec::new(), out: &mut out };
        v.visit_file(&file.ast);
    }
    out
}

struct Collect<'a> {
    file: &'a RustFile,
    module: Vec<String>,
    owner: Vec<String>,
    out: &'a mut Vec<Function>,
}

impl Collect<'_> {
    fn push(&mut self, name: String, span: proc_macro2::Span, attrs: &[syn::Attribute], body: TokenStream) {
        let mut symbol = self.module.clone();
        symbol.extend(self.owner.iter().cloned());
        symbol.push(name);
        let mut tokens = doc_words(attrs);
        tokens.extend(fold(body));
        self.out.push(Function {
            path: self.file.path.clone(),
            symbol: symbol.join("::"),
            lines: (span.start().line, span.end().line),
            shingles: shingles(&tokens),
            tokens,
        });
    }
}

impl<'ast> Visit<'ast> for Collect<'_> {
    fn visit_item_mod(&mut self, m: &'ast syn::ItemMod) {
        if is_test(&m.attrs) || m.content.is_none() {
            return;
        }
        self.module.push(m.ident.to_string());
        syn::visit::visit_item_mod(self, m);
        self.module.pop();
    }

    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        if !is_test(&f.attrs) {
            self.push(f.sig.ident.to_string(), f.span(), &f.attrs, f.block.to_token_stream());
        }
    }

    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        if is_test(&i.attrs) {
            return;
        }
        let name = match &*i.self_ty {
            syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()),
            _ => None,
        };
        self.owner.push(name.unwrap_or_else(|| "impl".into()));
        syn::visit::visit_item_impl(self, i);
        self.owner.pop();
    }

    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        if !is_test(&f.attrs) {
            self.push(f.sig.ident.to_string(), f.span(), &f.attrs, f.block.to_token_stream());
        }
    }
}

fn doc_words(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut out = Vec::new();
    for attr in attrs.iter().filter(|a| a.path().is_ident("doc")) {
        if let syn::Meta::NameValue(nv) = &attr.meta {
            if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &nv.value {
                out.extend(
                    s.value()
                        .split(|c: char| !c.is_alphanumeric())
                        .filter(|w| w.len() > 2)
                        .map(str::to_lowercase),
                );
            }
        }
    }
    out
}

/// Flattens a token stream; every literal becomes `L`.
pub fn fold(stream: TokenStream) -> Vec<String> {
    let mut out = Vec::new();
    for tree in stream {
        match tree {
            TokenTree::Group(g) => {
                out.push(open(g.delimiter()).into());
                out.extend(fold(g.stream()));
            }
            TokenTree::Ident(i) => out.push(i.to_string()),
            TokenTree::Punct(p) => out.push(p.as_char().to_string()),
            TokenTree::Literal(_) => out.push("L".into()),
        }
    }
    out
}

fn open(d: proc_macro2::Delimiter) -> &'static str {
    match d {
        proc_macro2::Delimiter::Brace => "{",
        proc_macro2::Delimiter::Bracket => "[",
        proc_macro2::Delimiter::Parenthesis => "(",
        proc_macro2::Delimiter::None => "",
    }
}

/// Hashed 5-shingles of a token sequence.
pub fn shingles(tokens: &[String]) -> BTreeSet<u64> {
    tokens
        .windows(5)
        .map(|w| {
            let mut h = 1469598103934665603_u64;
            for s in w {
                for byte in s.bytes() {
                    h = (h ^ byte as u64).wrapping_mul(1099511628211);
                }
                h = (h ^ 0xff).wrapping_mul(1099511628211);
            }
            h
        })
        .collect()
}

/// Jaccard similarity of two shingle sets.
pub fn similarity(x: &BTreeSet<u64>, y: &BTreeSet<u64>) -> f64 {
    if x.is_empty() || y.is_empty() {
        return 0.0;
    }
    let (small, large) = if x.len() < y.len() { (x, y) } else { (y, x) };
    if (large.len() as f64) > 2.5 * small.len() as f64 {
        return 0.0;
    }
    let shared = small.iter().filter(|h| large.contains(h)).count();
    shared as f64 / (x.len() + y.len() - shared) as f64
}
