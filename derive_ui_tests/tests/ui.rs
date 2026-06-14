#[test]
fn fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
}

#[test]
pub fn pass() {
    macrotest::expand("tests/ui/pass/*.rs");
}
