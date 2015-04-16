fn main() {
    let foo = 27;
    let (x,y) = (1,2);
    let mut bar : u8 = 0;
    println!("foo: {}",foo);
    println!("(x,y): ({},{})",x,y);
    println!("bar: {}",bar);
    bar = bar + 255;
    println!("bar: {}",bar);
}
