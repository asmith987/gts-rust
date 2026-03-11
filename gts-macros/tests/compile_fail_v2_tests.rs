//! Compile-fail tests for `#[derive(GtsSchema)]` macro validation.

#[test]
fn compile_fail_v2_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail_v2/*.rs");
}
