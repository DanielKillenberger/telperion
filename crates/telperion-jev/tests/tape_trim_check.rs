//! `tape_trim --check` fails when it lists a page that keeps more than the
//! run quoted, so a script chaining it stops there (fn-157 FRICTION.md).
use std::process::Command;

#[test]
fn the_check_exits_nonzero_when_it_lists_a_finding() {
    let tape = std::env::temp_dir().join(format!("trim-check-{}", std::process::id()));
    std::fs::create_dir_all(tape.join("firecrawl")).unwrap();
    let key = "d".repeat(64);
    let entry = serde_json::json!({"key": key,
        "request": {"op": "scrape", "url": "https://closed.test/page"},
        "response": {"ok": {"content_type": "text/html",
            "markdown": "A page nobody quoted, kept whole in the recording."}}});
    std::fs::write(
        tape.join(format!("firecrawl/{}.json", &key[..32])),
        entry.to_string(),
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_tape_trim"))
        .arg(&tape)
        .arg("--check")
        .output()
        .unwrap();
    let printed = String::from_utf8_lossy(&out.stdout);
    assert!(printed.contains("closed.test"), "{printed}");
    assert!(!out.status.success(), "{printed}");
}
