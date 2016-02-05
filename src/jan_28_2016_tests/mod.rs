#[derive(Debug, PartialEq)]
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
    assert_eq!(_foo, Color::Red);
}

struct Foo<'a> {
    id: u8,
    bars: Vec<&'a u8>,
}

impl<'a> Default for Foo<'a> {
    fn default() -> Foo<'a> {
        Foo {
            id: 0,
            bars: Vec::new(),
        }
    }
}

impl<'a> Foo<'a> {
    fn new() -> Foo<'a> {
        Self::default()
    }
    fn add_bar(&mut self, bar: &'a u8) {
        self.bars.push(bar);
    }
}

#[test]
fn messing_around_with_lifetimes() {
    let      _a:  u8 = 1;
    let mut foo: Foo = Foo::default();
    foo.add_bar(&_a);
}

struct Person {
    name: String,
}

impl Default for Person {
    fn default() -> Person {
        Person { name: "Rick".to_string() }
    }
}

struct Judger;
impl Judger {
    fn bad_judge_person(person: Person) -> u8 {
        0
    }
    fn good_judge_person(person: &Person) -> u8 {
        0
    }
}

#[test]
fn borrowing() {
    let person: Person = Person::default();
    let _ = Judger::bad_judge_person(person);
    //assert_eq!(&person.name, "Rick"); person no longer owns the data
    let person: Person = Person::default();
    let _ = Judger::good_judge_person(&person);
    assert_eq!(&person.name, "Rick"); //this is ok because i sent a reference to the Judger

    let mut x: Person = Person::default();
    let     y: Person = x;
    x = Person{ name: "Joe".to_string() };
    assert_eq!(&x.name,"Joe");
    assert_eq!(&y.name,"Rick");
}
