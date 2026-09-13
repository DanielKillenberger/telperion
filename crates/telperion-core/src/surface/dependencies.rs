use super::paths::paths;
use crate::Result;

/// A sweep's frame reads its path and attachment, and its leader choices read
/// siblings. Select both endpoint sweeps when a stamped dependency changes.
pub(crate) fn affected(
    tree: &crate::tree::Tree,
    changed: &std::collections::BTreeSet<crate::tree::NodeIdentity>,
) -> Result<std::collections::BTreeSet<crate::tree::NodeIdentity>> {
    let mut touched = vec![false; tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate() {
        if changed.contains(&n.identity) {
            touched[i] = true;
            if let Some(p) = n.parent {
                let mut stand = p as usize;
                while let Some(parent) = tree.nodes[stand].parent {
                    if tree.nodes[stand]
                        .position
                        .distance(tree.nodes[parent as usize].position)
                        > 1e-9
                    {
                        break;
                    }
                    stand = parent as usize;
                }
                touched[stand] = true;
            }
        }
    }
    let paths = paths(&tree.nodes)?;
    let mut out = std::collections::BTreeSet::new();
    for path in paths.runs {
        let nodes = &paths.nodes[path.start..path.end];
        if nodes.iter().any(|&i| touched[i]) {
            out.extend(nodes.iter().skip(1).map(|&i| tree.nodes[i].identity));
        }
    }
    Ok(out)
}
