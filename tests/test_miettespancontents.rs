use miette::{MietteSpanContents, SourceOffset, SourceSpan, SpanContents};

#[test]
fn test_spancontents() {
    let data = "hello_world";
    let line = 1;
    let col = 2;
    let line_count = 1;
    let offset = SourceOffset::from_location(data, line, col);
    let span_len = 3;
    let span = SourceSpan::new(offset, span_len);
    let contents = MietteSpanContents::new(data.as_bytes(), span, line, col, line_count);
    assert_eq!(contents.data(), data.as_bytes());
    assert_eq!(*contents.span(), span);
    assert_eq!(contents.line(), line);
    assert_eq!(contents.column(), col);
    assert_eq!(contents.line_count(), line_count);
    assert_eq!(contents.name(), None);
    assert_eq!(contents.language(), None);

    let language_name = "Rust";
    let contents1 = contents.with_language(language_name);
    assert_eq!(contents1.language(), Some(language_name));

    let file_name = "hello_world.rs";
    let contents2 = contents1.with_name(file_name);
    assert_eq!(contents2.name(), Some(file_name));
}
