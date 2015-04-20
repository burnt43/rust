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
}

fn print_thing(x: u8) {
    println!("x: {}",x);
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
