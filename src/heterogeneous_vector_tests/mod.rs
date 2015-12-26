macro_rules! numerize_animal_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec : Vec<u64> = Vec::new();
            $( temp_vec.push($x.as_u64()); )*
            temp_vec
        }
    };
}

struct Cat {
    meow: u64,
}

struct Dog {
    woof: u64,
}

impl Cat {
    fn new (x: u64) -> Cat {
        Cat { meow: x }
    }
}

impl Dog {
    fn new (x: u64) -> Dog {
        Dog { woof: x }
    }
}

trait NumerizeAnimal {
    fn as_u64 (&self) -> u64;
}

impl NumerizeAnimal for Cat {
    fn as_u64 (&self) -> u64 {
        self.meow
    }
}

impl NumerizeAnimal for Dog {
    fn as_u64 (&self) -> u64 {
        self.woof
    }
}

#[test]
fn test1 () {
    let v : Vec<u64> = numerize_animal_vec![Cat::new(1),Dog::new(2)];
    assert_eq!( *(v.last().unwrap()), 2 );
}
