use std::ptr;
use std::mem;

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

    let stuff: &mut [u8] = &mut [3,4,5,6,7];
    let fluff: &[u8] = &[8,9,10];
    let pointer = stuff.as_mut_ptr();
    unsafe {
        //mem::forget(stuff);
        ptr::write(pointer.offset(1),100);
        println!("{}",*pointer.offset(1));
        let v:Vec<u8> = Vec::from_raw_parts(pointer,20,20);
        println!("{:?}",v);
    }
    println!("{:?}",stuff);
}
