
macro_rules! shlec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        }
    };
}
    
// assert_eq!( 1, i_want_some_trait( &(&[Foobar::new()] as &[Foobar]) ) );
macro_rules! attributes {
    ( $( $attr:expr ),* ) => {
        &[$( $attr ),*] as &[Foobar]
    };
}

macro_rules! o_O {
    (
        $(
            $x:expr; [ $( $y:expr ),* ]
        );*
    ) => {
        &[ $($( $x + $y ),*),* ]
    }
}

struct Foobar {
    x: u8,
    y: u8,
}

impl Foobar {
    fn new () -> Foobar {
        Foobar {
            x: 0,
            y: 0,
        }
    }
}

trait SomeTrait {
    fn some_function(&self) -> u8;
}

impl SomeTrait for Foobar {
    fn some_function(&self) -> u8 {
        self.x + self.y
    }   
}

impl<'a> SomeTrait for &'a[Foobar] {
    fn some_function(&self) -> u8 {
        let mut result: u8 = 0;
        for foo in *self {
            result = result + foo.some_function();
        }
        result
    }
}

fn i_want_some_trait<T: SomeTrait> (foo: T) -> u8 {
    foo.some_function()
}

macro_rules! goofy_addition {
    ( $x:expr,$y:expr ) => { $x + $y } 
}

macro_rules! int_flags {
    ( $x:expr;$($y:expr),* ) => {
        for i in &[$($y),*] {
            if $x & i > 0 {
                println!("\033[0;32m1\033[0;39m");
            } else {
                println!("\033[0;31m0\033[0;39m");
            }
        }
    }
}

macro_rules! str_to_u64vec {
    ( $x:expr ) => {
            $x.chars().map(|c| c as u64).collect::<Vec<u64>>()
    }
}

#[test]
fn foobar () {
    let v: Vec<u8> = shlec![1,2,3,4];
    assert_eq!( v[0], 1 );
    assert_eq!( 30, i_want_some_trait( Foobar{x: 10, y: 20} ) );
    assert_eq!( 4,  i_want_some_trait( &[Foobar{x: 1, y:3}] as &[Foobar] ) );
    assert_eq!( 10, i_want_some_trait( attributes![Foobar{x:1,y:2},Foobar{x:3,y:4}] ) );
    assert_eq!( 50, i_want_some_trait( attributes![Foobar{x:25,y:25}] ) );

    let a: &[u32] = o_O!(10; [1,2]; 20; [3,4]);
    assert_eq!( a, [11,12,23,24] );
    assert_eq!( 10, goofy_addition!(4,6) );

    const CAN_READ : u64 = 4;
    const CAN_WRITE : u64 = 2;
    const CAN_EXEC: u64 = 1;
    int_flags!(5;CAN_READ,CAN_WRITE,CAN_EXEC);
    
    assert_eq!( vec![65u64,66u64], str_to_u64vec!("AB") );
}
