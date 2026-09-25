//! Path resolution over the production Rust: each file's `use` aliases, the
//! crates' `pub use` re-exports, and `crate`, `self` and `super`. A path
//! resolves to its defining item's absolute path, crate ident first.

use std::collections::{BTreeMap, BTreeSet};

use super::modtree::RustFile;

/// One `use` leaf: the name it binds (none for a glob) and its target.
#[derive(Debug, Clone)]
pub struct UseLeaf {
    pub alias: Option<String>,
    pub target: Vec<String>,
    pub public: bool,
    pub line: usize,
}

/// Every `use` leaf in `items`, inline modules included.
pub fn uses(items: &[syn::Item], module: &[String], crates: &BTreeSet<String>) -> Vec<UseLeaf> {
    let mut out = Vec::new();
    for item in items {
        match item {
            syn::Item::Use(u) => {
                let public = !matches!(u.vis, syn::Visibility::Inherited);
                let line = u.use_token.span.start().line;
                flatten(&u.tree, Vec::new(), public, line, &mut out);
            }
            syn::Item::Mod(m) if !super::modtree::is_test(&m.attrs) => {
                if let Some((_, inner)) = &m.content {
                    let mut sub = module.to_vec();
                    sub.push(m.ident.to_string());
                    out.extend(uses(inner, &sub, crates));
                }
            }
            _ => {}
        }
    }
    for leaf in &mut out {
        leaf.target = absolute(&leaf.target, module, &BTreeMap::new(), crates);
    }
    out
}

fn flatten(tree: &syn::UseTree, prefix: Vec<String>, public: bool, line: usize, out: &mut Vec<UseLeaf>) {
    let mut path = prefix;
    match tree {
        syn::UseTree::Path(p) => {
            path.push(p.ident.to_string());
            flatten(&p.tree, path, public, line, out);
        }
        syn::UseTree::Name(n) => {
            let name = n.ident.to_string();
            if name == "self" {
                let alias = path.last().cloned();
                out.push(UseLeaf { alias, target: path, public, line });
            } else {
                path.push(name.clone());
                out.push(UseLeaf { alias: Some(name), target: path, public, line });
            }
        }
        syn::UseTree::Rename(r) => {
            let name = r.ident.to_string();
            if name != "self" {
                path.push(name);
            }
            out.push(UseLeaf { alias: Some(r.rename.to_string()), target: path, public, line });
        }
        syn::UseTree::Glob(_) => out.push(UseLeaf { alias: None, target: path, public, line }),
        syn::UseTree::Group(g) => {
            for item in &g.items {
                flatten(item, path.clone(), public, line, out);
            }
        }
    }
}

/// `path` as written in `module`, made absolute through `aliases`.
pub fn absolute(
    path: &[String],
    module: &[String],
    aliases: &BTreeMap<String, Vec<String>>,
    crates: &BTreeSet<String>,
) -> Vec<String> {
    let Some(first) = path.first() else { return Vec::new() };
    let mut out: Vec<String> = match first.as_str() {
        "crate" => vec![module[0].clone()],
        "self" => module.to_vec(),
        "super" => {
            let mut base = module.to_vec();
            let mut rest = path;
            while rest.first().map(String::as_str) == Some("super") {
                base.pop();
                rest = &rest[1..];
            }
            base.extend(rest.iter().cloned());
            return base;
        }
        "::" => Vec::new(),
        name if aliases.contains_key(name) => aliases[name].clone(),
        name if crates.contains(name) => vec![name.to_string()],
        _ => {
            let mut base = module.to_vec();
            base.push(first.clone());
            base
        }
    };
    out.extend(path[1..].iter().cloned());
    out
}

/// Re-exports across the snapshot, and the items each module defines, so a
/// glob re-export resolves only to a name its target module really has.
#[derive(Debug, Default)]
pub struct Exports {
    named: BTreeMap<String, Vec<String>>,
    globs: BTreeMap<String, Vec<Vec<String>>>,
    defined: BTreeSet<String>,
    /// Every module's `use` leaves, private ones included: a child's
    /// `use super::*` sees its parent's imports.
    imports: BTreeMap<String, Vec<UseLeaf>>,
}

impl Exports {
    pub fn of(files: &[RustFile], crates: &BTreeSet<String>) -> Self {
        let mut exports = Self::default();
        for file in files {
            define(&file.ast.items, &file.module, &mut exports.defined);
            let leaves = uses(&file.ast.items, &file.module, crates);
            exports.imports.insert(file.module.join("::"), leaves.clone());
            for leaf in leaves {
                if !leaf.public {
                    continue;
                }
                let module = file.module.join("::");
                match leaf.alias {
                    Some(alias) => {
                        exports.named.insert(format!("{module}::{alias}"), leaf.target);
                    }
                    None => exports.globs.entry(module).or_default().push(leaf.target),
                }
            }
        }
        exports
    }

    /// The aliases a file sees: its own `use` leaves, then those its globs
    /// bring from modules of the snapshot, nearest first. Globs of modules
    /// the snapshot does not hold are returned for the caller to judge.
    pub fn aliases(&self, module: &[String], own: &[UseLeaf]) -> (BTreeMap<String, Vec<String>>, Vec<UseLeaf>) {
        let mut aliases = BTreeMap::new();
        let mut foreign = Vec::new();
        let mut queue: Vec<(Vec<UseLeaf>, usize)> = vec![(own.to_vec(), 0)];
        let mut seen = BTreeSet::from([module.join("::")]);
        while let Some((leaves, depth)) = queue.pop() {
            for leaf in leaves {
                let target = self.canonical(&leaf.target);
                match &leaf.alias {
                    Some(alias) => {
                        aliases.entry(alias.clone()).or_insert(target);
                    }
                    None => match self.imports.get(&target.join("::")) {
                        Some(inner) if depth < 4 && seen.insert(target.join("::")) => {
                            queue.insert(0, (inner.clone(), depth + 1));
                        }
                        Some(_) => {}
                        None => foreign.push(UseLeaf { target, ..leaf }),
                    },
                }
            }
        }
        (aliases, foreign)
    }

    /// Follows re-exports from the longest matching prefix, each at most
    /// once, so a module re-exporting its own namesake cannot loop.
    pub fn canonical(&self, path: &[String]) -> Vec<String> {
        let mut current = path.to_vec();
        let mut used = BTreeSet::new();
        while let Some((key, next)) = self.step(&current, &used) {
            used.insert(key);
            current = next;
        }
        current
    }

    fn step(&self, path: &[String], used: &BTreeSet<String>) -> Option<(String, Vec<String>)> {
        if self.defined.contains(&path.join("::")) {
            return None;
        }
        for split in (1..=path.len()).rev() {
            let key = path[..split].join("::");
            if used.contains(&key) {
                continue;
            }
            if let Some(target) = self.named.get(&key) {
                let mut next = target.clone();
                next.extend(path[split..].iter().cloned());
                return Some((key, next));
            }
        }
        for split in (1..path.len()).rev() {
            let key = format!("{}::*", path[..split].join("::"));
            if used.contains(&key) {
                continue;
            }
            for base in self.globs.get(&path[..split].join("::")).into_iter().flatten() {
                let mut next = base.clone();
                next.push(path[split].clone());
                if self.defined.contains(&next.join("::")) || self.named.contains_key(&next.join("::")) {
                    next.extend(path[split + 1..].iter().cloned());
                    return Some((key, next));
                }
            }
        }
        None
    }
}

/// Records every item `items` defines, impl functions as `Type::name`.
fn define(items: &[syn::Item], module: &[String], out: &mut BTreeSet<String>) {
    let at = |name: String| format!("{}::{name}", module.join("::"));
    for item in items {
        match item {
            syn::Item::Fn(f) => {
                out.insert(at(f.sig.ident.to_string()));
            }
            syn::Item::Struct(s) => {
                out.insert(at(s.ident.to_string()));
            }
            syn::Item::Enum(e) => {
                out.insert(at(e.ident.to_string()));
            }
            syn::Item::Const(c) => {
                out.insert(at(c.ident.to_string()));
            }
            syn::Item::Type(t) => {
                out.insert(at(t.ident.to_string()));
            }
            syn::Item::Mod(m) => {
                out.insert(at(m.ident.to_string()));
                if let Some((_, inner)) = &m.content {
                    let mut sub = module.to_vec();
                    sub.push(m.ident.to_string());
                    define(inner, &sub, out);
                }
            }
            syn::Item::Impl(i) => {
                let syn::Type::Path(p) = &*i.self_ty else { continue };
                let Some(ty) = p.path.segments.last() else { continue };
                for inner in &i.items {
                    if let syn::ImplItem::Fn(f) = inner {
                        out.insert(at(format!("{}::{}", ty.ident, f.sig.ident)));
                    }
                }
            }
            _ => {}
        }
    }
}
