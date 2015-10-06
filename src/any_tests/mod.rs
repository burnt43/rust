use std::fmt::Debug;
use std::any::Any;

fn log<T: Any + Debug>(value: &T) {
    let value_any = value as &Any;

    if value_any.is::<String>() {
        println!("i am a string");
    }
    match value_any.downcast_ref::<String>() {
        Some(string) => {
            println!("String ({}): {}", string.len(), string);
        }
        None => {
            println!("{:?}",value);
        }
    }
}

fn i_take_any_type(something: &mut Any) {
    if let Some(string) = something.downcast_mut::<String>() {
        string.push_str("hello");
    }
    if let Some(array) = something.downcast_mut::<[u8;3]>() {
        array[0] = 128;
    }
}

pub fn execute () {
    let some_string = "Hello".to_string();
    log(&some_string);

    let some_integer = 19u8;
    log(&some_integer);

    let mut some_slice:[u8;3] = [1,2,3];
    i_take_any_type(&mut some_slice);
    assert_eq!([128,2,3],some_slice);

    let mut some_string = "foo: ".to_string();
    i_take_any_type(&mut some_string);
    assert_eq!("foo: hello",some_string);
}
