use libc::size_t;

#[link(name="snappy")]
extern {
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}

#[link(name="ncurses")]
extern {
    fn has_mouse () -> bool;
}

pub fn execute () {
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("x: {}",x);

    let y = unsafe { has_mouse() };
    println!("y: {}",y);
}
