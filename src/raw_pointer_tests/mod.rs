use std::ptr;
use std::mem;

struct SomeData {
    x: u8,
    y: u8,
    z: u8,
}

fn some_function () {
    let _x: &mut[u8] = &mut[1,2,3,4,5];
    let _y: &[u8] = &[5,6,7,8,9];

    let p_x: *mut u8 = _x.as_mut_ptr();
    let p_y: *const u8 = _y.as_ptr();
    let v: Vec<u8> = unsafe { Vec::from_raw_parts(p_x,40,40) };
    println!("{:p}",p_x);
    unsafe { println!("{:p}",p_x.offset(1)); }
    println!("{:p}",p_y);
    println!("{:?}",v);

    let _z: *const u8 = ptr::null();
    assert!(_z.is_null());
}

fn stupid_reverse ( list: &mut [u8] ) {
    let pointer:   *mut u8 = list.as_mut_ptr();
    let mut begin: isize   = 0;
    let mut end:   isize   = (list.len() - 1) as isize;

    while begin < end {
        unsafe { ptr::swap( pointer.offset(begin), pointer.offset(end) ); }
        begin += 1;
        end -= 1;
    }
}

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

    some_function();

    let foo: &mut [u8] = &mut [1,2,3,4,5];
    println!("{:?}",foo);
    stupid_reverse(foo);
    println!("{:?}",foo);

}
