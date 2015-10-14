use libc::size_t;

#[link(name="snappy")]
extern {
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}

pub fn execute () {
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("x: {}",x);
}
