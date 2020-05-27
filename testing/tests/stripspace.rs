use askama::Template;

#[derive(askama::Template)]
#[template(path = "stripspace.html")]
struct StripSpace;

#[test]
fn test_strip_space() {
    let template = StripSpace;
    assert_eq!(
        template.render().unwrap(),
        "[\n1\n  23\n    47\n  8 I  M \n]"
    );
}

#[derive(askama::Template)]
#[template(
    source = "1 {% stripspace %} 2 {% nop %} 3 {% space %} 4 5 {% tab %} 6 7 {% newline %} 8 9 {% endstripspace %} 10",
    ext = "txt"
)]
struct InsertSpace;

#[test]
fn test_insert_space() {
    let template = InsertSpace;
    assert_eq!(template.render().unwrap(), "1 23 4 5\t6 7\n8 9 10");
}
