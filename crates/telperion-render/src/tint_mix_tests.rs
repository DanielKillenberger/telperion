//! Each operator mix Tint refuses and a twin it accepts, every pair run
//! through Dawn's `tint` (v20261009.190412) first (fn-237 R14).
use crate::tint_mix::findings;

fn holds(refused: &str, accepted: &str) {
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
        holds(refused, accepted);
    }
}

/// Template lists as WGSL discovers them: a shift inside one does not
/// hide what follows, its arguments are checked, and a name is no
/// template unless a `>` closes it.
#[test]
fn template_lists_are_found_as_wgsl_finds_them() {
    holds(
        "alias A = array<u32, (1u << 1u)>;
         fn f(a: u32, b: u32, c: u32) -> u32 { return a | b << c; }",
        "alias A = array<u32, (1u << 1u)>;
         fn f(a: u32, b: u32, c: u32) -> u32 { return a | (b << c); }",
    );
    holds(
        "alias A = array<u32, (1u | 2u + 3u)>;",
        "alias A = array<u32, (1u | (2u + 3u))>;",
    );
    holds(
        "fn f(texture_x: u32, a: u32, b: bool) -> bool { return texture_x < a < b; }",
        "fn f(texture_x: u32, a: u32) -> bool { return texture_x < a; }",
    );
}

/// An operator after a keyword that opens an expression is unary.
#[test]
fn an_operator_after_a_keyword_is_unary() {
    holds(
        "fn f(a: u32, b: u32, c: u32) -> u32 { return a - b << c; }",
        "fn f(a: i32, b: u32) -> i32 { return -a << b; }",
    );
    holds(
        "fn f(a: i32, b: i32) -> i32 { if -a + b | a > 0 { return a; } return b; }",
        "fn f(a: i32, b: i32) -> i32 { return -a | b; }",
    );
}

/// Comments, block comments nested, hide nothing and add nothing.
#[test]
fn comments_are_not_read() {
    holds(
        "fn f(a: u32, b: u32, c: u32) -> u32 { let x = a | /* /* ; */ */ b << c; return x; }",
        "fn f(a: u32, b: u32) -> u32 { let x = a /* x | y */ + b; return x; }",
    );
    holds(
        "fn f(a: u32, b: u32, c: u32) -> u32 { let x = a | // ;\n b << c; return x; }",
        "fn f(a: u32, b: u32) -> u32 { let x = a // | y\n + b; return x; }",
    );
}
