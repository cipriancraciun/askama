use askama::Template;

#[derive(askama::Template)]
#[template(path = "stripspace.html")]
struct StripSpace;

#[test]
fn test_strip_space() {
    let template = StripSpace;
    assert_eq!(
        template.render().unwrap(),
        "[\n1 a2 b3 c4 d7 g8 h I i  M1 m1  M2 m2 \n]"
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
