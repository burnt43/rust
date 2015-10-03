use std::str;
use std::fmt;

fn some_computation () -> Result<u8, str::Utf8Error> {
    let _ = try!( str::from_utf8(&[255,255]) );
    Ok(40)
}

pub fn execute () {
    assert!(true);
    assert_eq!(10,5+5);
    assert!( cfg!(target_os = "linux") );
    println!("column!() = {}",column!());
    assert_eq!("s1s2s3",concat!("s1","s2","s3"));
    for path in env!("PATH").split(":") {
        println!("{}",path);
    }
    assert_eq!("src/macro_tests/mod.rs",file!());
    assert_eq!("x: 10,y: 20",format!("x: {},y: {}",10,20));
    let s = fmt::format( format_args!("{}",5) );
    assert_eq!(s,"5");
    let x:u8 = include!("./some_file");
    assert_eq!(x,27);
    let x:&[u8] = include_bytes!("./some.bin");
    assert_eq!(65,x[0]);
    let x:&str = include_str!("./some.txt");
    for line in x.lines() {
        println!("{}",line);
    }
    println!("line!(): {}",line!());
    println!("module_path!(): {}",module_path!());
    assert_eq!(None,option_env!("FOOBAR"));
    assert_eq!("1 + 1",stringify!(1 + 1));

    /*fn unfinished() {
        unimplemented!();
    }*/

    println!("{}",some_computation().unwrap_or(22));
}
