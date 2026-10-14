//! WGSL's operator mixes without parentheses, which naga accepts and Tint
//! refuses ("mixing '|' and '<<' requires parenthesis"; fn-237 R14, the
//! cycle hierarchy build's `begin_main` in Chrome). The rules are the WGSL
//! grammar's, W3C WGSL section "Operator Precedence and Associativity"
//! (https://www.w3.org/TR/WGSL/#operator-precedence-associativity): a
//! bitwise expression's operands are unary expressions under one operator,
//! so `|`, `&` and `^` take no other binary operator beside them; a shift's
//! operands are unary, so a shift takes no shift or arithmetic beside it
//! (a comparison of shifts is allowed); comparisons do not chain; `&&` and
//! `||` do not mix. Template lists are found as WGSL finds them (section
//! "Template Lists"), and their arguments are checked as expressions of
//! their own. Checked against Tint itself, one fixture a mix.
use std::collections::HashSet;

const OPERATORS: [&str; 21] = [
    "<<=", ">>=", "->", "<<", ">>", "<=", ">=", "==", "!=", "&&", "||", "+=", "-=", "*=", "/=",
    "%=", "&=", "|=", "^=", "++", "--",
];

/// Keywords an expression follows: an operator after one is unary.
const KEYWORDS: [&str; 12] = [
    "return",
    "if",
    "else",
    "while",
    "switch",
    "case",
    "for",
    "let",
    "var",
    "const",
    "override",
    "const_assert",
];

/// A template list's `<` and `>`, told apart from comparisons.
const OPEN: &str = "<template";
const CLOSE: &str = ">template";

/// Each operator mix Tint refuses in `source`, one line each.
pub(crate) fn findings(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut levels: Vec<Vec<String>> = vec![Vec::new()];
    let mut operand = false;
    let end = |levels: &mut Vec<Vec<String>>, out: &mut Vec<String>| {
        out.extend(mixed(&std::mem::take(levels.last_mut().expect("a level"))));
    };
    for t in tokens(&uncommented(source)) {
        let t = t.as_str();
        let assigns = t.ends_with('=') && t.len() > 1 && !matches!(t, "==" | "!=" | "<=" | ">=");
        match t {
            "(" | "[" | OPEN => {
                levels.push(Vec::new());
                operand = false;
            }
            ")" | "]" | CLOSE => {
                out.extend(mixed(&levels.pop().unwrap_or_default()));
                if levels.is_empty() {
                    levels.push(Vec::new());
                }
                operand = true;
            }
            "," | ";" | "{" | "}" | ":" | "=" | "->" | "@" => {
                end(&mut levels, &mut out);
                operand = false;
            }
            _ if assigns => {
                end(&mut levels, &mut out);
                operand = false;
            }
            "++" | "--" => operand = true,
            _ if kind(t).is_some() => {
                if operand {
                    levels.last_mut().expect("a level").push(t.to_string());
                }
                operand = false;
            }
            _ if KEYWORDS.contains(&t) => operand = false,
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
    let mut pairs: Vec<(&str, &str)> = (ops.windows(2))
        .filter(|w| bitwise(w))
        .map(|w| (w[0], w[1]))
        .collect();
    let logical: Vec<&str> = ops.iter().copied().filter(is(Kind::Logical)).collect();
    pairs.extend(
        (logical.windows(2))
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

/// `source` with every comment blanked: line comments, and block comments,
/// which nest.
fn uncommented(source: &str) -> Vec<char> {
    let cs: Vec<char> = source.chars().collect();
    let mut out = Vec::with_capacity(cs.len());
    let (mut i, mut depth, mut line) = (0, 0, false);
    while i < cs.len() {
        let pair = (cs[i], cs.get(i + 1).copied().unwrap_or(' '));
        if line {
            line = cs[i] != '\n';
        } else if pair == ('/', '*') {
            depth += 1;
            i += 1;
        } else if depth > 0 && pair == ('*', '/') {
            depth -= 1;
            i += 1;
        } else if depth == 0 && pair == ('/', '/') {
            line = true;
        } else if depth == 0 {
            out.push(cs[i]);
            i += 1;
            continue;
        }
        out.push(' ');
        i += 1;
    }
    out
}

/// Where a number starting at `i` ends: its suffix and exponent's sign in.
fn number_end(cs: &[char], mut i: usize) -> usize {
    let hex = cs.get(i + 1).is_some_and(|x| *x == 'x' || *x == 'X');
    let start = i;
    while i < cs.len() {
        let x = cs[i];
        let exponent = if hex { ['p', 'P'] } else { ['e', 'E'] };
        let sign = (x == '+' || x == '-') && i > start && exponent.contains(&cs[i - 1]);
        if !(x.is_alphanumeric() || x == '.' || x == '_' || sign) {
            break;
        }
        i += 1;
    }
    i
}

fn ident_end(cs: &[char], mut i: usize) -> usize {
    while i < cs.len() && (cs[i].is_alphanumeric() || cs[i] == '_') {
        i += 1;
    }
    i
}

/// The positions of every template list's `<` and `>`, by WGSL's template
/// list discovery (https://www.w3.org/TR/WGSL/#template-list-discovery):
/// an identifier's `<` opens a candidate, a `>` at its nesting depth
/// closes it, and an assignment, `;`, `{`, `:`, a closing bracket below
/// it, `&&` or `||` abandons it.
fn templates(cs: &[char]) -> HashSet<usize> {
    let mut found = HashSet::new();
    let mut pending: Vec<(usize, usize)> = Vec::new();
    let (mut i, mut depth) = (0, 0usize);
    let next = |i: usize| cs.get(i + 1).copied().unwrap_or(' ');
    while i < cs.len() {
        let c = cs[i];
        if c.is_ascii_digit() {
            i = number_end(cs, i);
            continue;
        }
        if c.is_alphabetic() || c == '_' {
            let mut j = ident_end(cs, i);
            while j < cs.len() && cs[j].is_whitespace() {
                j += 1;
            }
            if cs.get(j) == Some(&'<') {
                if matches!(next(j), '<' | '=') {
                    i = j + 2;
                    continue;
                }
                pending.push((j, depth));
                i = j + 1;
                continue;
            }
            i = j;
            continue;
        }
        let abandon = |pending: &mut Vec<(usize, usize)>, depth: usize| {
            while pending.last().is_some_and(|&(_, d)| d >= depth) {
                pending.pop();
            }
        };
        match c {
            '>' if pending.last().is_some_and(|&(_, d)| d == depth) => {
                found.insert(pending.pop().expect("a candidate").0);
                found.insert(i);
            }
            '>' | '!' if next(i) == '=' => i += 1,
            '(' | '[' => depth += 1,
            ')' | ']' => {
                abandon(&mut pending, depth);
                depth = depth.saturating_sub(1);
            }
            '=' if next(i) == '=' => i += 1,
            '=' | ';' | '{' | ':' => {
                depth = 0;
                pending.clear();
            }
            '&' | '|' if next(i) == c => {
                abandon(&mut pending, depth);
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    found
}

/// The tokens an expression is read from: operators by longest match,
/// numbers whole, and a template list's `<` and `>` as `OPEN` and `CLOSE`.
fn tokens(cs: &[char]) -> Vec<String> {
    let marks = templates(cs);
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        let start = i;
        if c.is_whitespace() {
            i += 1;
            continue;
        } else if marks.contains(&i) {
            out.push(if c == '<' { OPEN } else { CLOSE }.to_string());
            i += 1;
            continue;
        } else if c.is_ascii_digit() {
            i = number_end(cs, i);
        } else if c.is_alphabetic() || c == '_' {
            i = ident_end(cs, i);
        } else {
            let rest: String = cs[i..(i + 3).min(cs.len())].iter().collect();
            let op = (OPERATORS.iter())
                .find(|o| rest.starts_with(*o))
                .map_or(1, |o| o.len());
            i += op;
        }
        out.push(cs[start..i].iter().collect());
    }
    out
}
