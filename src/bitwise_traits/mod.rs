use std::ops::{BitAnd};

struct Foo {
    x: u8,
    y: u8,
}

struct Bar {
    x: u8,
    s: String,
}

trait Ghost {
    fn to_u8(&self) -> u8;
}

impl Ghost for Foo {
    fn to_u8(&self) -> u8 {
        self.x + self.y
    }
}

impl Ghost for Bar {
    fn to_u8(&self) -> u8 {
        self.x + (self.s.len() as u8)
    }
}

impl BitAnd<Foo> for Foo {
    type Output = u8;
    fn bitand(self, rhs: Foo) -> u8 {
        self.to_u8() & rhs.to_u8()
    }
}

impl BitAnd<Bar> for Foo {
    type Output = u8;
    fn bitand(self, rhs: Bar) -> u8 {
        self.to_u8() & rhs.to_u8()
    }
}

#[test]
fn foo () {
    assert_eq!( Foo{x: 3, y: 4} & Foo{x: 2,y: 1}, 3 );
    assert_eq!( Foo{x: 3, y: 4} & Bar{x: 10,s: "persons".to_string()}, 1);
}
