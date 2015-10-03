use std::fmt;

struct SomeBullShit {
    foo: String,
    bar: u8,
}

impl SomeBullShit {
    fn new(foo:&str, bar:u8) -> SomeBullShit {
        SomeBullShit {
            foo: foo.to_string(),
            bar: bar,
        }
    }
}

impl fmt::Debug for SomeBullShit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"SomeBullShit( foo: {}, bar: {} )", self.foo, self.bar)
    }
}

impl fmt::Display for SomeBullShit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.foo)
    }
}

pub fn execute () {
    let b = SomeBullShit::new("Jim",45);
    println!("SomeBullShit Debug --- {:?}",b);
    println!("SomeBullShit Display --- {}",b);
}
