use pulldown_cmark::{Parser, html};

#[must_use]
pub fn load_readme() -> String {
    const README: &str = include_str!("../readme.md");
    README.to_string()
}

#[must_use]
pub fn markdown_to_html(md: &str) -> String {
    let parser = Parser::new(md);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

// Unit tests
#[test]
fn converts_multiple_markdown_features() {
    let input = "# Title\n\n**Bold** text";
    let html = markdown_to_html(input);

    assert!(html.contains("<h1>Title</h1>"));
    assert!(html.contains("<strong>Bold</strong>"));
}
