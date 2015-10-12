use std::fmt;

struct Foo {
    x: u8,
    y: u8,
}

impl fmt::Binary for Foo {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:b}{:b}",self.x,self.y)
    }
}

pub fn execute () {
    assert_eq!("Hi",              format!("{}","Hi") );
    assert_eq!("3 2 1",           format!("{2} {1} {0}",1,2,3) );
    assert_eq!("C B A B C A C B", format!("{2} {1} {} {} {} {0} {2} {1}","A","B","C") );
    assert_eq!("name: Jim",       format!("name: {name}",name="Jim") );
    assert_eq!("xxx 1 zzz",       format!("{foo} {} {bar}",1,foo="xxx",bar="zzz") );
    assert_eq!("3.14", format!("{:.*}",2,3.14159) );
    assert_eq!("1010", format!("{:b}",10) );
    assert_eq!("a", format!("{:x}",10) );
    assert_eq!("A", format!("{:X}",10) );
    println!("{}",format!("{:p}",&10));
    assert_eq!("31",format!("{:o}",25) );
    assert_eq!("2.7e2",format!("{:e}",270.0));
    assert_eq!("5.555E3",format!("{:E}",5555.0));

    let foo = Foo {
        x: 10,
        y: 8,
    };

    assert_eq!("10101000", format!("{:b}",foo) );
}
