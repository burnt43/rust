// {"age":23}

use std::collections::{HashMap};

enum Strinteger {
    A(u8),
    B(String),
}

trait JamesEncodable {
    fn from_james_encoding(&str) -> Result<Self,()>;
    fn to_james_encoding(&self) -> String;
    fn from_strinteger_result(input: &Strinteger) -> Result<Self,()>;
}

type Object = HashMap<String,Strinteger>;

impl JamesEncodable for u8 {
    fn from_james_encoding(s: &str) -> Result<u8,()> {
        Ok(16u8)
    }
    fn to_james_encoding(&self) -> String {
        let mut result = String::new();
        result.push_str(stringify!(*self));
        result
    }
    fn from_strinteger_result(input: &Strinteger) -> Result<Self,()> {
        if let &Strinteger::A(ref a) = input { Ok(*a) } else { Err(()) }
    }
}

#[test]
fn still_trying_wacky_stuff() {
    assert!( u8::from_strinteger_result( &Strinteger::A(1u8) ).is_ok() );
    assert!( u8::from_strinteger_result( &Strinteger::B(String::new()) ).is_err() );
}

fn simple_try_stuff() {
    fn foo(n: u8) -> Result<u8,()> {
        if n == 10 { Ok(n) } else { Err(()) }
    }
    fn bar() -> Result<u8,()> {
        let x: u8 = try!(foo(10));
        Ok(x)
    }
    assert!(bar().is_ok());
}
