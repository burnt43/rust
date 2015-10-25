struct Foobar {
    x: u8,
    y: u8,
    z: u32,
    a: u32,
    b: u64,
}

impl Foobar {
    fn new () -> Foobar {
        Foobar {
            x: -1,
            y: -1,
            z: -1,
            a: -1,
            b: -1,
        }
    }
}

pub fn execute () {
    let f = Foobar::new();
    println!("{:p}",&f.x);
    println!("{:p}",&f.y);
    println!("{:p}",&f.z);
    println!("{:p}",&f.a);
    println!("{:p}",&f.b);
}
