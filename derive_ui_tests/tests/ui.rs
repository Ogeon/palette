#[test]
fn fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
}

#[test]
fn pass() {
    let t = trybuild::TestCases::new();
    for path in glob::glob("tests/ui/pass/*.rs")
        .unwrap()
        .filter_map(Result::ok)
        .filter(|path| {
            !path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .ends_with(".expanded")
        })
    {
        t.pass(path);
    }
}

#[test]
pub fn pass_expand() {
    macrotest::expand("tests/ui/pass/*.rs");
}
