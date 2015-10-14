use libc::{c_int,c_char,size_t};

#[link(name="snappy")]
extern {
    fn snappy_compress(input:             *const u8,
                       input_length:      size_t,
                       compressed:        *mut u8,
                       compressed_length: *mut size_t) -> c_int;

    fn snappy_uncompress(compressed:          *const u8,
                         compressed_length:   size_t,
                         uncompressed:        *mut u8,
                         uncompressed_length: *mut size_t) -> c_int;

    fn snappy_max_compressed_length(source_length: size_t) -> size_t;

    fn snappy_uncompressed_length(compressed:        *const u8,
                                  compressed_length: size_t,
                                  result:            *mut size_t) -> c_int;

    fn snappy_validate_compressed_buffer(compressed:        *const u8,
                                         compressed_length: size_t) -> c_int;
}

#[link(name="ncurses")]
extern {
    fn has_mouse () -> bool;
}

fn validate_compressed_buffer(src: &[u8]) -> Result<bool,&str> {
    unsafe {
        match snappy_validate_compressed_buffer( src.as_ptr(), src.len() as size_t ) {
            0 => Ok(true),
            1 => Err("invalid input"),
            2 => Err("buffer too small"),
            _ => unreachable!(),
        }
    }
}

pub fn execute () {
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("x: {}",x);

    let y = unsafe { has_mouse() };
    println!("y: {}",y);

    match validate_compressed_buffer(&[65;100]) {
        Ok(boolean) => println!("{}",boolean),
        Err(e)      => println!("{}",e),
    }
}
