#[derive(Debug)]
struct Foo {
    x: u8,
}

impl Drop for Foo {
    fn drop (&mut self) {
        println!("{:?} has gone out of scope",self);
    }
}

pub fn execute () {
    let _foo = Foo { x: 12 };
    let _bar = Foo { x: 45 };
}
