//! The pipeline boundary, as the renderer meets it: a build through the
//! executor interface compiles, and a stage called around it does not.
#[test]
fn a_stage_outside_the_executor_interface_does_not_compile() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/boundary/interface.rs");
    cases.compile_fail("tests/boundary/stage.rs");
}
