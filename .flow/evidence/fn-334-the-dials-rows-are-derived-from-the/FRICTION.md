# fn-334 friction

## 9 Nov 2026, Opus build worker: the crate's formatter check is red on files nobody touched

Doing: formatting the new `pipeline/derived_tests.rs` before its commit.
Hindered by: `cargo fmt -p telperion-core -- --check` reports drift in `examples/early.rs`, `examples/premises.rs` and `examples/trunk.rs` on the base (`70cdc500a`), so formatting the crate would have put three unrelated files in this child's diff. Formatted the one file with `rustfmt --edition 2021` instead.
Cost: about 1 minute.
Would remove it: one `cargo fmt` commit on the examples, or a fmt check in the gate so drift cannot land.
