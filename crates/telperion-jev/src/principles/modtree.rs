//! The production Rust of a snapshot: every file a crate root reaches through
//! `mod` items, with its module path. Test modules (`#[cfg(test)]`, alone or
//! inside `all(..)`) and the `tests`, `examples` and `benches` trees are never
//! reached, so nothing here is test code.

use super::source::Snapshot;

pub struct RustFile {
    pub path: String,
    /// Crate ident first, e.g. `telperion_core::foliage::plan`.
    pub module: Vec<String>,
    pub ast: syn::File,
}

/// A file a crate reaches that does not parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub path: String,
    pub message: String,
}

/// Every production file of every crate under `crates/`.
pub fn production(snapshot: &Snapshot) -> (Vec<RustFile>, Vec<ParseError>) {
    let mut out = Vec::new();
    let mut errors = Vec::new();
    for (manifest, text) in &snapshot.files {
        let Some(dir) = manifest
            .strip_prefix("crates/")
            .and_then(|rest| rest.strip_suffix("/Cargo.toml"))
        else {
            continue;
        };
        let Some(name) = package_name(text) else { continue };
        let ident = name.replace('-', "_");
        let src = format!("crates/{dir}/src");
        let mut roots = vec![(format!("{src}/lib.rs"), ident.clone())];
        roots.push((format!("{src}/main.rs"), ident.clone()));
        let bin = format!("{src}/bin/");
        for path in snapshot.files.keys() {
            if let Some(file) = path.strip_prefix(&bin) {
                if let Some(stem) = file.strip_suffix(".rs").filter(|s| !s.contains('/')) {
                    roots.push((path.clone(), stem.replace('-', "_")));
                }
            }
        }
        for (root, krate) in roots {
            if snapshot.get(&root).is_some() {
                let dir = parent(&root);
                walk(snapshot, &root, vec![krate], &dir, &mut out, &mut errors);
            }
        }
    }
    (out, errors)
}

fn package_name(manifest: &str) -> Option<String> {
    let package = manifest.split("[package]").nth(1)?;
    let line = package
        .lines()
        .find(|line| line.trim_start().starts_with("name"))?;
    Some(line.split('"').nth(1)?.to_string())
}

fn parent(path: &str) -> String {
    path.rsplit_once('/').map(|(dir, _)| dir.to_string()).unwrap_or_default()
}

fn walk(
    snapshot: &Snapshot,
    path: &str,
    module: Vec<String>,
    child_dir: &str,
    out: &mut Vec<RustFile>,
    errors: &mut Vec<ParseError>,
) {
    let Some(text) = snapshot.get(path) else { return };
    let ast = match syn::parse_file(text) {
        Ok(ast) => ast,
        Err(err) => {
            let line = err.span().start().line;
            errors.push(ParseError {
                path: path.to_string(),
                message: format!("line {line}: {err}"),
            });
            return;
        }
    };
    let file_dir = parent(path);
    children(snapshot, &ast.items, &module, child_dir, &file_dir, out, errors);
    out.push(RustFile {
        path: path.to_string(),
        module,
        ast,
    });
}

fn children(
    snapshot: &Snapshot,
    items: &[syn::Item],
    module: &[String],
    child_dir: &str,
    file_dir: &str,
    out: &mut Vec<RustFile>,
    errors: &mut Vec<ParseError>,
) {
    for item in items {
        let syn::Item::Mod(m) = item else { continue };
        if is_test(&m.attrs) {
            continue;
        }
        let name = m.ident.to_string();
        let mut sub = module.to_vec();
        sub.push(name.clone());
        let dir = format!("{child_dir}/{name}");
        if let Some((_, items)) = &m.content {
            children(snapshot, items, &sub, &dir, file_dir, out, errors);
            continue;
        }
        let explicit = m.attrs.iter().find_map(path_attr);
        let candidates = match explicit {
            Some(p) => vec![format!("{file_dir}/{p}")],
            None => vec![format!("{dir}.rs"), format!("{dir}/mod.rs")],
        };
        if let Some(found) = candidates.into_iter().find(|c| snapshot.get(c).is_some()) {
            let next = if found.ends_with("/mod.rs") { parent(&found) } else { dir };
            walk(snapshot, &found, sub, &next, out, errors);
        }
    }
}

fn path_attr(attr: &syn::Attribute) -> Option<String> {
    if !attr.path().is_ident("path") {
        return None;
    }
    let syn::Meta::NameValue(nv) = &attr.meta else { return None };
    match &nv.value {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(s),
            ..
        }) => Some(s.value()),
        _ => None,
    }
}

/// True for `#[test]` and for a `cfg` that holds only when testing.
pub fn is_test(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if attr.path().is_ident("test") {
            return true;
        }
        if !attr.path().is_ident("cfg") {
            return false;
        }
        attr.parse_args::<syn::Meta>()
            .map(|meta| needs_test(&meta))
            .unwrap_or(false)
    })
}

fn needs_test(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(p) => p.is_ident("test"),
        syn::Meta::List(list) => {
            let nested = list
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .unwrap_or_default();
            if list.path.is_ident("all") {
                nested.iter().any(needs_test)
            } else if list.path.is_ident("any") {
                !nested.is_empty() && nested.iter().all(needs_test)
            } else {
                false
            }
        }
        syn::Meta::NameValue(_) => false,
    }
}
