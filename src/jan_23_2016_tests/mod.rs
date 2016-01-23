struct Cat {
    id: u8,
}

impl Cat {
    fn new() -> Cat {
        Cat {
            id: 1
        }
    }
}

struct Dog {
    id: u8,
}

impl Dog {
    fn new() -> Dog {
        Dog {
            id: 1
        }
    }
}

trait Speak {
    fn get_id(&self) -> u32;
}

impl Speak for Cat {
    fn get_id(&self) -> u32 {
        (self.id as u32) << 8
    }
}

impl Speak for Dog {
    fn get_id(&self) -> u32 {
        (self.id as u32) << 16
    }    
}

struct Foo {
    some_field: u8,
    some_speak: Box<Speak>,
}

impl Foo {
    fn new() -> Foo {
        Foo {
            some_field: 0,
            some_speak: Box::new(Cat::new())
        }
    }
    fn switch_to_cat(&mut self) {
        self.some_speak = Box::new(Cat::new());
    }
    fn switch_to_dog(&mut self) {
        self.some_speak = Box::new(Dog::new());
    }
}

#[test]
fn test1() {
    let mut f = Foo::new();
    assert_eq!(f.some_speak.get_id(),256);
    f.switch_to_dog();
    assert_eq!(f.some_speak.get_id(),65536);
    f.switch_to_cat();
    assert_eq!(f.some_speak.get_id(),256);
}

#[test]
fn test2() {
    {
        let _foo: Foo;
        {
            let _box: Box<Speak> = Box::new(Dog::new());
            _foo = Foo {
                some_field: 1,
                some_speak: _box,
            }
        }
        assert_eq!(_foo.some_speak.get_id(),65536);
    }
}

#[test]
fn test3() {
    let raw_pointer: *const u8;
    {
        let boxed_pointer: Box<u8>;
        boxed_pointer = Box::new(10u8);
        raw_pointer = Box::into_raw(boxed_pointer);
    }
    unsafe {
        assert_eq!(*raw_pointer,10u8);
    }
}
