// Thanks to /u/diwic
use std::str::FromStr;

enum Foobar {
    Alpha(String),
    Beta(u8),
}

trait Getter {
    fn get_that_stuff(&Foobar) -> Result<Self,()>;
}

impl Getter for u8 {
    fn get_that_stuff(f: &Foobar) -> Result<u8,()> {
        if let &Foobar::Beta(ref z) = f { Ok(*z) } else { Err(()) }
    }
}

impl Foobar {
    fn get<T>(&self) -> Result<T,()> where T: Getter {
        T::get_that_stuff(self)
    }
}

#[test]
fn sanity_check() {
    let _whatever: Foobar = Foobar::Beta(10u8);
    assert!(_whatever.get::<u8>().is_ok());
    let _other: Foobar = Foobar::Alpha(String::new());
    assert!(_other.get::<u8>().is_err());
}

#[test]
fn other_test() {
    let _result = u8::from_str("8");
    let _result = "8".parse::<u8>();
}
