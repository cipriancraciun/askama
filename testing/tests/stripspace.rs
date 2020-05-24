use askama::Template;

#[derive(askama::Template)]
#[template(path = "stripspace.html")]
struct StripSpace;

#[test]
fn test_strip_space() {
    let template = StripSpace;
    assert_eq!(template.render().unwrap(), "[\n1\n  23\n    47\n  8\n]");
}
