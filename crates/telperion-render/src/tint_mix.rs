//! WGSL's operator mixes without parentheses, which naga accepts and Tint
//! refuses ("mixing '|' and '<<' requires parenthesis"; fn-237 R14, the
//! cycle hierarchy build's `begin_main` in Chrome). The rules are the WGSL
//! grammar's, W3C WGSL section "Operator Precedence and Associativity"
//! (https://www.w3.org/TR/WGSL/#operator-precedence-associativity): a
//! bitwise expression's operands are unary expressions under one operator,
//! so `|`, `&` and `^` take no other binary operator beside them; a shift's
//! operands are unary, so a shift takes no shift or arithmetic beside it
//! (a comparison of shifts is allowed); comparisons do not chain; `&&` and
//! `||` do not mix. Checked against Tint itself, one fixture a mix.
const OPERATORS: [&str; 21] = [
    "<<=", ">>=", "->", "<<", ">>", "<=", ">=", "==", "!=", "&&", "||", "+=", "-=", "*=", "/=",
    "%=", "&=", "|=", "^=", "++", "--",
];

/// Each operator mix Tint refuses in `source`, one line each.
pub(crate) fn findings(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut levels: Vec<Vec<String>> = vec![Vec::new()];
    let mut operand = false;
    for t in tokens(&crate::tint_rules::strip(source)) {
        let t = t.as_str();
        match t {
            "(" | "[" => {
                levels.push(Vec::new());
                operand = false;
            }
            ")" | "]" => {
                out.extend(mixed(&levels.pop().unwrap_or_default()));
                if levels.is_empty() {
                    levels.push(Vec::new());
                }
                operand = true;
            }
            "," | ";" | "{" | "}" | ":" | "=" | "->" | "@" => {
                let level = levels.last_mut().expect("a level");
                out.extend(mixed(&std::mem::take(level)));
                operand = false;
            }
            _ if t.ends_with('=') && t.len() > 1 && !matches!(t, "==" | "!=" | "<=" | ">=") => {
                let level = levels.last_mut().expect("a level");
                out.extend(mixed(&std::mem::take(level)));
                operand = false;
            }
            "++" | "--" => operand = true,
            _ if kind(t).is_some() => {
                if operand {
                    levels.last_mut().expect("a level").push(t.to_string());
                }
                operand = false;
            }
            _ => operand = t.starts_with(|c: char| c.is_alphanumeric() || c == '_' || c == '.'),
        }
    }
    for level in &levels {
        out.extend(mixed(level));
    }
    out
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Bitwise,
    Shift,
    Arithmetic,
    Relational,
    Logical,
}

fn kind(op: &str) -> Option<Kind> {
    Some(match op {
        "|" | "&" | "^" => Kind::Bitwise,
        "<<" | ">>" => Kind::Shift,
        "+" | "-" | "*" | "/" | "%" => Kind::Arithmetic,
        "<" | ">" | "<=" | ">=" | "==" | "!=" => Kind::Relational,
        "&&" | "||" => Kind::Logical,
        _ => return None,
    })
}

/// The first refused mix among one level's binary operators.
fn mixed(ops: &[String]) -> Option<String> {
    let is = |k: Kind| move |o: &&str| kind(o) == Some(k);
    let ops: Vec<&str> = ops.iter().map(String::as_str).collect();
    // A bitwise operator beside any other; `&&` beside `||`.
    let bitwise =
        |w: &[&str]| w[0] != w[1] && (is(Kind::Bitwise)(&w[0]) || is(Kind::Bitwise)(&w[1]));
    let mut pairs: Vec<(&str, &str)> = ops
        .windows(2)
        .filter(|w| bitwise(w))
        .map(|w| (w[0], w[1]))
        .collect();
    let logical: Vec<&str> = ops.iter().copied().filter(is(Kind::Logical)).collect();
    pairs.extend(
        logical
            .windows(2)
            .filter(|w| w[0] != w[1])
            .map(|w| (w[0], w[1])),
    );
    for run in ops.split(is(Kind::Logical)) {
        // Comparisons do not chain.
        let rel: Vec<&str> = run.iter().copied().filter(is(Kind::Relational)).collect();
        pairs.extend(rel.windows(2).map(|w| (w[0], w[1])));
        // A shift's operands are unary: no other operator in its operand.
        for operand in run.split(is(Kind::Relational)) {
            if let Some(at) = operand.iter().position(is(Kind::Shift)) {
                let other = (0..operand.len()).find(|&k| k != at);
                let pair = |k: usize| {
                    if k < at {
                        (operand[k], operand[at])
                    } else {
                        (operand[at], operand[k])
                    }
                };
                pairs.extend(other.map(pair));
            }
        }
    }
    let first = pairs.first().copied();
    first.map(|(a, b)| format!("mixing '{a}' and '{b}' requires parenthesis"))
}

/// The tokens an expression is read from: operators by longest match,
/// numbers with their exponent's sign, and template lists (`vec3<f32>`,
/// `var<storage, read>`) dropped, since their `<` and `>` are not
/// comparisons.
fn tokens(s: &str) -> Vec<String> {
    let cs: Vec<char> = s.chars().collect();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() {
            let start = i;
            let hex = cs.get(i + 1).is_some_and(|x| *x == 'x' || *x == 'X');
            while i < cs.len() {
                let x = cs[i];
                let sign = (x == '+' || x == '-')
                    && matches!(cs[i - 1], 'e' | 'E' | 'p' | 'P')
                    && (!hex || matches!(cs[i - 1], 'p' | 'P'));
                if !(x.is_alphanumeric() || x == '.' || x == '_' || sign) {
                    break;
                }
                i += 1;
            }
            out.push(cs[start..i].iter().collect());
        } else if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < cs.len() && (cs[i].is_alphanumeric() || cs[i] == '_') {
                i += 1;
            }
            let name: String = cs[start..i].iter().collect();
            let mut j = i;
            while j < cs.len() && cs[j].is_whitespace() {
                j += 1;
            }
            if templated(&name) && cs.get(j) == Some(&'<') {
                let mut depth = 0;
                while j < cs.len() {
                    depth += match cs[j] {
                        '<' => 1,
                        '>' => -1,
                        _ => 0,
                    };
                    j += 1;
                    if depth == 0 {
                        break;
                    }
                }
                i = j;
            }
            out.push(name);
        } else {
            let rest: String = cs[i..(i + 3).min(cs.len())].iter().collect();
            let op = OPERATORS
                .iter()
                .find(|o| rest.starts_with(*o))
                .map_or(c.to_string(), |o| o.to_string());
            i += op.chars().count();
            out.push(op);
        }
    }
    out
}

/// Names that take a template list.
fn templated(name: &str) -> bool {
    let mat = name.len() == 6 && name.starts_with("mat") && name.as_bytes()[4] == b'x';
    mat || name.starts_with("texture_")
        || matches!(
            name,
            "vec2"
                | "vec3"
                | "vec4"
                | "array"
                | "ptr"
                | "atomic"
                | "bitcast"
                | "var"
                | "binding_array"
        )
}

#[cfg(test)]
mod tests {
    use super::findings;

    /// Each mix Tint refuses (Dawn's `tint`, v20261009.190412) and its
    /// parenthesised or allowed twin, which it accepts.
    #[test]
    fn each_refused_mix_is_found_and_its_twin_is_not() {
        for (refused, accepted) in [
            (
                // The cycle hierarchy build's state word, which Chrome refused.
                "g = select(3u, 2u, rank % cfg.cap == 0u) | (rank / cfg.cap) << 2u;",
                "g = select(3u, 2u, rank % cfg.cap == 0u) | ((rank / cfg.cap) << 2u);",
            ),
            ("g = a & b | c;", "g = a | b | c;"),
            ("g = a + b << c;", "g = (a + b) << c;"),
            ("g = a << b << c;", "g = a << (b << c);"),
            ("g = a % b | c;", "g = w[a & 1u] + 1u;"),
            ("let x = a & b == c;", "let x = (a & b) == c;"),
            ("let x = a < b == p;", "let x = a << b == c;"),
            ("let x = p && q || p;", "let x = a < b && c < a;"),
            ("let x = a < 1e-4 < c;", "let x = s < 1e-4 || r <= 0.0;"),
            (
                "g = v.x >> 2u & 1u;",
                "g = u32(f32(a) * 1e-3 + 2.0) | vec3<u32>(b).x;",
            ),
        ] {
            assert_eq!(
                findings(refused).len(),
                1,
                "{refused}: {:?}",
                findings(refused)
            );
            assert!(
                findings(accepted).is_empty(),
                "{accepted}: {:?}",
                findings(accepted)
            );
        }
    }
}
