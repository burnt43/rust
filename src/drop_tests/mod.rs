#[derive(Debug)]
struct Foo<'a> {
    x:    u8,
    list: &'a mut Vec<u8>,
}

impl<'a> Drop for Foo<'a> {
    fn drop (&mut self) {
        self.list.push(self.x);
        println!("{:?} has gone out of scope",self);
    }
}

pub fn execute () {
    let mut numbers: Vec<u8> = Vec::new();

    {
        let _foo = Foo {
            x:    12,
            list: &mut numbers,
        };
    }

    {
        let _bar = Foo {
            x:    45,
            list: &mut numbers,
        };
    }

    assert_eq!(numbers,vec![12,45]);
}
