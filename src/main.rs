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
    println!("------------------------------------------------------------");
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
// Testing Traits
//
//-----------------------------------------------------------------------------------------------
struct Cat {
    name: &'static str,
    age:  i8,
}
struct Dog {
    name: &'static str,
    age:  i8
}
trait CanSpeak {
    fn speak(&self);
}
impl CanSpeak for Cat {
    fn speak(&self) {
        println!("meow! my name is {} and i'm {} years old",self.name,self.age);
    }
}
impl CanSpeak for Dog {
    fn speak(&self) {
        println!("woof! my name is {} and i'm {} years old",self.name,self.age);
    }
}

fn make_animal_speak<T: CanSpeak>(animal: T) {
    print!("making animal speak: ");
    animal.speak();
}

fn traits_test() {
    println!("------------------------------------------------------------");
    let cat = Cat{name: "Vixen", age: 13};
    let dog = Dog{name: "Desmond", age: 3};
    cat.speak();
    dog.speak();
    make_animal_speak(cat);
    make_animal_speak(dog);
}

//-----------------------------------------------------------------------------------------------
//
// Main Function
//
//-----------------------------------------------------------------------------------------------

fn main () {
    generics_test();
    traits_test();
}
