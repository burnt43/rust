pub fn execute () {
    let foo = 5u8;
    let p_foo = &foo as *const u8;

    println!("{:p}",p_foo);
    unsafe {
        println!("{}",*p_foo);
    }

    let raw_pointer: *mut u8;
    {
        let mut foo = 20u8;
        raw_pointer = &mut foo;
    }
    println!("{:p}",raw_pointer);
    unsafe {
        *raw_pointer = 54;
        println!("{}",*raw_pointer);
    }
}
