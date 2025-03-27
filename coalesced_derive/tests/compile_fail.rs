#[test]
fn test_ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compile_fail/**/*.rs");
}
