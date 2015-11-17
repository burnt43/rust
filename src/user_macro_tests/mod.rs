
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
}
