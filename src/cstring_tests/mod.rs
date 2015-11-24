use libc;
use libc::c_char;
use std::str::Utf8Error;
use std::ptr;
use std::ffi::CString;

fn get_a_string () -> Result<String,Utf8Error> {
    // allocate a string
    let buffer: Vec<u8> = vec![1;8];
    let heap: *mut c_char = CString::new(buffer).unwrap().into_raw();
    // now pretend i'm sending this to some c function and it writes to it
    unsafe {
        ptr::write( heap.offset(0), 65 );
        ptr::write( heap.offset(1), 66 );
        ptr::write( heap.offset(2), 67 );
        ptr::write( heap.offset(3), 68 );
        ptr::write( heap.offset(4), 69 );
        ptr::write( heap.offset(5), 0 );
        ptr::write( heap.offset(6), 0 );
        ptr::write( heap.offset(7), 0 );
    }
    assert_eq!( unsafe{ *(heap.offset(0)) }, 65 );
    assert_eq!( unsafe{ *(heap.offset(1)) }, 66 );
    assert_eq!( unsafe{ *(heap.offset(2)) }, 67 );
    assert_eq!( unsafe{ *(heap.offset(3)) }, 68 );
    assert_eq!( unsafe{ *(heap.offset(4)) }, 69 );
    assert_eq!( unsafe{ *(heap.offset(5)) }, 0 );
    assert_eq!( unsafe{ *(heap.offset(6)) }, 0 );
    assert_eq!( unsafe{ *(heap.offset(7)) }, 0 );
    
    let result: CString = unsafe { CString::from_raw(heap) };
    match result.to_str() {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(e),
    }
    
}

#[test]
fn test_it () {
    let raw: *mut c_char = CString::new("ABC").unwrap().as_ptr() as *mut c_char;
    assert_eq!( unsafe{ ptr::read::<c_char>( raw ) }, 65 );
    assert_eq!( unsafe{ ptr::read::<c_char>( raw.offset(1) ) }, 66 );
    assert_eq!( unsafe{ ptr::read::<c_char>( raw.offset(2) ) }, 67 );
    assert_eq!( unsafe{ ptr::read::<c_char>( raw.offset(3) ) }, 0 );
    let foo: CString = unsafe { CString::from_raw( raw ) };
    match get_a_string() {
        Ok(s) => assert_eq!(s,"ABCDE"),
        Err(_) => assert!(false),
    }
}
