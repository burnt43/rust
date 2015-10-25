use std::mem;

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
    println!("{}",mem::size_of::<Foobar>());
    println!("{:p}",&f.x);
    println!("{:p}",&f.y);
    println!("{:p}",&f.z);
    println!("{:p}",&f.a);
    println!("{:p}",&f.b);
    let raw_ptr: *const u8 = &f.x;
    unsafe {
        assert_eq!( *raw_ptr.offset(0), f.x );
        assert_eq!( *raw_ptr.offset(1), f.y );
        assert_eq!( *(raw_ptr.offset(4) as *const u32), f.z );
        assert_eq!( *(raw_ptr.offset(8) as *const u32), f.a );
        assert_eq!( *(raw_ptr.offset(16) as *const u64), f.b );
        for i in 0..mem::size_of::<Foobar>() {
            println!("{:2}: {:X}",i,*raw_ptr.offset(i as isize));
        }
    }
}
