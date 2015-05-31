// Testing Generics ------------------------------------------------------------------------
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
    struct Person<T> {
        id: T,
    }
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


fn multi_threads () {
    struct Thing {
        x: u16,
    }
    let foo = Arc::new(Mutex::new(Thing {x:0}));
    for _ in 0..10 {
        let foo = foo.clone();
        let thread = thread::spawn(move || {
            let mut foo = foo.lock().unwrap();
            while foo.x < 10 {
                println!("count: {}",foo.x);
                foo.x+=1;
            }
        });
        thread.join();
    }
}

fn multi_threads_read_only () {
    struct Thing {
        x: u16,
    }
    let foo = Arc::new(Thing {x:0});
    for _ in 0..10 {
        let foo = foo.clone();
        let thread = thread::spawn(move || {
            for _ in 0..10 {
                println!("x: {}",foo.x);
            }
        });
        thread.join();
    }
}

fn thread_test () {
    spawn_one_boring_thread();
    shared_and_mutable();
    multi_threads();
    multi_threads_read_only();
}
// patterns  ------------------------------------------------------------------------------

fn basic_pattern (x: u8) {
    match x {
        0 => println!("You gave me 0"),
        1 => println!("You gave me 1"),
        _ => println!("Not 0 or 1"),
    }
}

fn basic_pattern2 (x: u8) {
    match x {
        0 | 1 => println!("You gave me 0 or 1"),
        _     => println!("You have me something else"),
    }
}

fn basic_pattern3 (x: u8) {
    match x {
        0...10  => println!("1-10"),
        10...20 => println!("10-20"),
        _       => println!("20-255"),
    }
}

fn basic_pattern4 (x: u8) {
    match x {
        x @ 0...10 => println!("x: {}",x),
        _          => println!("foobar"),
    }
}

fn basic_pattern5 () {
    struct Person {
        name: Option<String>,
    }
    let x = Person{name: Some("James".to_string())};
    match x {
        Person{name: Some(a)} => println!("{:?}",a),
        _ => println!("BAR"),
    }
}

fn basic_pattern6 () {
    enum OptionalInt {
        Value(i32),
        Missing,
    }
    let x = OptionalInt::Value(128);
    match x {
        OptionalInt::Value(..) => println!("Value"),
        OptionalInt::Missing   => println!("Missing"),
    }
}

fn basic_pattern7 () {
    enum OptionalInt {
        Value(i32),
        Missing,
    }
    let x = OptionalInt::Value(45);
    match x {
        OptionalInt::Value(i) if i > 20 => println!("{} > 20!",i),
        OptionalInt::Value(..)          => println!("other"),
        OptionalInt::Missing            => println!("missing"),
    }
}

fn pattern_test () {
    basic_pattern(0);
    basic_pattern(1);
    basic_pattern(56);
    basic_pattern2(0);
    basic_pattern2(1);
    basic_pattern2(35);
    basic_pattern3(5);
    basic_pattern3(45);
    basic_pattern3(17);
    basic_pattern4(3);
    basic_pattern4(128);
    basic_pattern5();
    basic_pattern6();
    basic_pattern7();
}
// Trait Objects  -----------------------------------------------------------------------------------
fn trait_objects_test() {
    trait Foo {
        fn method(&self) -> String;
    }
    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}",*self) }
    }
    impl Foo for String {
        fn method(&self) -> String { format!("string: {}",*self) }
    }
    fn do_something<T: Foo> (x: T) {
        x.method();
    }
    fn do_something2(x: &Foo) {
        x.method();
    }
    let x = 5u8;
    let y = "Hello".to_string();
    do_something(x);
    do_something(y);
    do_something2(&x as &Foo);
    do_something2(&x);
}
// closures  -----------------------------------------------------------------------------------
fn closures_test() {
    //
    let plus_one = |x: u8| x + 1;
    let plus_two = |x: u8| {
        let mut result:u8 = x;
        result+=1;
        result+=1;
        result
    };
    assert_eq!(2,plus_one(1));
    assert_eq!(4,plus_two(2));
    //
    let mut num = 5;
    let clos = |x: u8| x + num;
    assert_eq!(8,clos(3));
    //
    let mut foo = 6;
    {
        let clos2 = |x: u8| x + foo;
    }
    let y = &mut foo;
    //
    let mut baz = 5;
    {
        let mut add_num = |x: u8| baz+=x;
        add_num(2);
    }
    assert_eq!(baz,7);
    //
    let mut shiz = 10;
    {
        let mut add_num = move |x: u8| shiz+=x;
        add_num(7);
    }
    assert_eq!(shiz,10);
    //
    fn call_with_two<F>(some_closure: F) -> i32 where F : Fn(i32) -> i32 {
        some_closure(2)
    }
    assert_eq!(64,call_with_two(|x| x * 32));
    //
    fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }
    assert_eq!(5,call_with_one(&|x| x + 4));
    //
    fn factory() -> Box<Fn(u8) -> u8> {
        let num = 5;
        Box::new(move |x| x + num)
    }
    let f = factory();
    assert_eq!(9,f(4));
}

fn universal_function_call() {
    trait Foo {
        fn f(&self);
    }
    trait Bar {
        fn f(&self);
    }
    struct Baz;
    impl Foo for Baz {
        fn f(&self) {
            println!("FOO");
        }
    }
    impl Bar for Baz {
        fn f(&self) {
            println!("BAR");
        }
    }
    let thing = Baz;
    Foo::f(&thing);
    Bar::f(&thing);
}

static N: u8 = 8;
const MAX_THINGS: u8 = 68;

fn static_and_const() {
    println!("N: {}",N);
    println!("MAX_THINGS: {}",MAX_THINGS);
}

fn aliases() {
    type Name     = String;
    let x: Name   = "Jim".to_string();
    let y: String = "Gym".to_string();
    println!("My name is {}",x);
    if x == y {
        println!("foobar");
    }
}

fn casting() {
    let x: u8 = 8;
    let y = x as u16;
    println!("x: {}",x);
    println!("y: {}",y);
}

fn associated_types() {
    trait Graph {
        type N;
        type E;
        fn has_edge(&self,&Self::N,&Self::N) -> bool;
        fn edges(&self, &Self::N) -> Vec<Self::E>;
    }
    struct Node;
    struct Edge;
    struct MyGraph;
    impl Graph for MyGraph {
        type N = Node;
        type E = Edge;

        fn has_edge(&self,n1: &Node,n2: &Node) -> bool {
            true
        }
        fn edges(&self,n: &Node) -> Vec<Edge> {
            Vec::new()
        }
    }
    let graph = MyGraph;
    let obj = Box::new(graph) as Box<Graph<N=Node,E=Edge>>;
}

// main  -----------------------------------------------------------------------------------
fn main () {
    generics_test();
    traits_test();
    if_let_test();
    while_let_test();
    thread_test();
    pattern_test();
    trait_objects_test();
    closures_test();
    universal_function_call();
    static_and_const();
    aliases();
    casting();
    associated_types();
}
