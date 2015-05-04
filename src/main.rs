// Testing Generics ------------------------------------------------------------------------
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

// Testing Traits ---------------------------------------------------------------------------------
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
    fn id_self(&self) {
        println!("I am a thing!");
    }
}
trait CanPurr {
    fn purr(&self);
}
trait Killer {
    fn kill(&self);
}
trait Assassin : Killer {
    fn assassinate(&self);
}
impl CanSpeak for Cat {
    fn speak(&self) {
        println!("meow! my name is {} and i'm {} years old",self.name,self.age);
    }
}
impl Killer for Cat {
    fn kill(&self) {
        println!("death by scratches!");
    }
}
impl Assassin for Cat {
    fn assassinate(&self) {
        println!("i'm an assassin! meow!");
    }
}
impl CanPurr for Cat {
    fn purr(&self) {
        println!("prrrrrrrrrrrrrrrrrrr");
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

fn multiple_traits<A,B>(a: A, b: B) where A: CanSpeak, B: CanSpeak + CanPurr {
    a.speak();
    b.speak();
    b.purr();
}

fn traits_test() {
    println!("------------------------------------------------------------");
    let cat = Cat{name: "Vixen", age: 13};
    let dog = Dog{name: "Desmond", age: 3};
    cat.speak();
    cat.id_self();
    dog.speak();
    make_animal_speak(cat);
    make_animal_speak(dog);
    let x = Cat{
        name: "Steve",
        age:  30
    };
    let y = Dog{
        name: "Roger",
        age:  25
    };
    multiple_traits(y,x);
}

// if let --------------------------------------------------------------------------------------
fn foobar0 (a: Option<u8>) {
    if let Option::Some(x) = a {
        println!("{}",x);
    } else {
        println!("not Option::Some()");
    }
}
fn if_let_test () {
    let foo: Option<u8> = Some(32);
    let bar: Option<u8> = None;
    foobar0(foo);
    foobar0(bar);
}

// while let -----------------------------------------------------------------------------------
fn some_computation (a: u8) -> Result<u8,i8> {
    if a > 10 {
        Result::Err(-1)
    } else {
        Result::Ok(a*3)
    }
}
fn while_let_test () {
    let mut x = 0;
    while let Result::Ok(y) = some_computation(x) {
        println!("{}",y);
        x = x+1;
    }
    println!("done!");
}

// threads ---------------------------------------------------------------------------------

use std::thread;
use std::sync::{Arc,Mutex};

fn spawn_one_boring_thread () {
    let t = thread::spawn(|| {
        println!("i'm in a thread");
    });
    t.join();
}

fn shared_and_mutable () {
    let data = Arc::new(Mutex::new(vec![10u8,20,30]));
    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            println!("1. data[{}] = {}",i,data[i]);
            data[i] += 1;
            println!("2. data[{}] = {}",i,data[i]);
        });
    }
    thread::sleep_ms(50);
}
//34462
struct Thing {
    x: u16,
}

fn multi_threads () {
    let mut foo = Thing { x: 0 };
    for _ in 0..10 {
        let thread = thread::spawn(move || {
            while foo.x < 1000 {
                println!("count: {}",foo.x);
                foo.x+=1;
            }
        });
        thread.join();
    }
}

fn thread_test () {
    spawn_one_boring_thread();
    shared_and_mutable();
    multi_threads();
}

// main  -----------------------------------------------------------------------------------
fn main () {
    generics_test();
    traits_test();
    if_let_test();
    while_let_test();
    thread_test();
}
