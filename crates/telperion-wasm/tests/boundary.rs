//! The pipeline boundary, as the binding meets it: a build through the one
//! pipeline compiles, and a stage called around it does not, nor the growth
//! path's scaffold grower.
#[test]
fn a_stage_outside_the_pipeline_does_not_compile() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/boundary/pipeline.rs");
    cases.compile_fail("tests/boundary/stage.rs");
    cases.compile_fail("tests/boundary/specimen.rs");
}
