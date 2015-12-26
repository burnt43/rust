struct Cat {
    meow: u64,
}

struct Dog {
    woof: u64,
}

trait NumerizeAnimal {
    fn as_u64 (&self) -> u64;
}

impl NumerizeAnimal for Cat {
    fn as_u64 (&self) -> u64 {
        100
    }
}

impl NumerizeAnimal for Dog {
    fn as_u64 (&self) -> u64 {
        200
    }
}

#[test]
fn test1 () {
    //let v : Vec<NumerizeAnimal> = Vec::new(); does not work. no Sized trait
}
