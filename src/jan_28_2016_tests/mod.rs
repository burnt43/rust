enum Color {
    Red,
    Green,
    Blue,
}

impl Default for Color {
    fn default() -> Color {
        Color::Red
    }
}

#[test]
fn default_values() {
    let _foo: u8 = Default::default();
    assert_eq!(_foo,0);
    let _foo: String = Default::default();
    assert!(_foo.is_empty());
    let _foo: Color = Color::default();
}
