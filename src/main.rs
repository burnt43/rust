struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    //comments
    let foo = 27;
    let (x,y) = (1,2);
    let mut bar : u8 = 0;
    println!("foo: {}",foo);
    println!("(x,y): ({},{})",x,y);
    println!("bar: {}",bar);
    bar = bar + 255;
    println!("bar: {}",bar);
    print_thing(45);
    print_sum(24,20);
    println!("sum2: {}",ret_sum(1,3));
    let a = [0;10];
    println!("a length: {}",a.len());
    println!("a[5]: {}",a[5]);
    let b = &a[2..4];
    println!("b length: {}",b.len());
    let tuple: (u8,&str) = (0,"apple");
    let index: u8 = tuple.0;
    let value: &str = tuple.1;
    let (first,second) = tuple;
    println!("index: {}",index);
    println!("value: {}",value);
    println!("first: {}",first);
    println!("second: {}",second);
    let z: fn(u8,u8) -> u8 = ret_sum;
    foobar1(5);
    foobar1(3);
    let foo1: u8 = if 4 == 4 {8} else {10};
    println!("foo1: {}",foo1);
    for x in 0..10 {
        println!("x: {}",x);
    }
    let mut done: bool = false;
    let mut foo2: u8 = 0;
    while !done {
        println!("foo2: {}",foo2);
        foo2 = foo2 + 1;
        done = foo2 == 10;
    }
    let mut foo3: u8 = 0;
    loop {
        println!("foo3: {}",foo3);
        foo3 = foo3 + 5;
        if foo3 > 50 {break;}
    }
    for x in 0..10 {
        if x%2 == 1 {continue;}
        println!("x: {}",x);
    }
    let foo4 = Box::new(5);
    let foo5 = add_one(foo4);
    println!("foo4: {}",foo5);
    let mut foo6: i32 = 10;
    add_one_v2(&mut foo6);
    println!("foo6: {}",foo6);
    //---------------------------------------------------
    let foo7 = &5;
    let f = Foo { x: foo7 };
    println!("f.x: {}",f.x);
    // I think this SHOULD WORK, but it doesn't...
    /*
    {
        let foo8;
        let foo9 = &5;
        let f1 = Foo { x: foo9 };
        foo8 = &f1.x;
    }
    */
}

fn add_one_v2(x: &mut i32) {
    *x+=1;
}

fn add_one(mut x: Box<i32>) -> Box<i32> {
    *x += 1;
    x
}

fn print_thing(x: u8) {
    println!("x: {}",x);
}

fn foobar1(x: u8) {
    if x == 5 {
        println!("You gave me 5!!");
    } else {
        println!("You didn't give me 5 :(");
    }
}

/// Adds 2 Numbers
///
/// #Examples
///
/// ```
/// let (x,y) = (1,2)
///
/// assert_eq!(3,print_sum(1,2));
/// ```
fn print_sum(x: u8,y: u8) {
    println!("sum: {}",x+y);
}

fn ret_sum(x: u8, y: u8) -> u8 {
    x + y
}
