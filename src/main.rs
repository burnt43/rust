//-----------------------------------------------------------------------------------------------
//
// Testing Generics
//
//-----------------------------------------------------------------------------------------------
struct Person<T> {
    id: T,
}
fn only_even_add(a: i8, b: i8) -> Result<i8,String> {
    if a % 2 == 0 && b % 2 == 0 {
        Result::Ok(a+b)
    } else {
        Result::Err("I can only add 2 even numbers!".to_string())
    }
}

fn a_generic_function<T>(foo: T) {
    match foo {
        _ => println!("test"),
    }
}

fn generics_test() {
    let x: Option<i8> = Some(2);
    match x {
        Option::Some(i) => println!("{}",i),
        Option::None    => println!("nothing"),
    }
    match only_even_add(1,2) {
        Result::Ok(i)  => println!("result: {}",i),
        Result::Err(e) => println!("operation failed: '{}'",e),
    }
    a_generic_function("jcarson");
    a_generic_function(6);
    a_generic_function(Person{id: 6});
    a_generic_function(Person{id: "james"});
}

//-----------------------------------------------------------------------------------------------
//
// Main Function
//
//-----------------------------------------------------------------------------------------------

fn main () {
    generics_test();
}
