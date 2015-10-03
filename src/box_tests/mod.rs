
pub fn execute () {
    match Option::Some(25) {
        Some(n) if n >= 0 => println!(">= 0"),
        Some(n) if n <  0 => println!("<  0"),
        Some(_)           => unreachable!(),
        None              => println!("none"),
    }

    enum List<T> {
        Some(T,Box<List<T>>),
        None,
    }

    let mut x:List<u8> = List::Some(1, Box::new(List::Some(2, Box::new(List::None))) );

    loop {
        match x {
            List::Some(value,next) => { 
                print!("{}->",value);
                x = *next;
            },
            List::None => {
                print!("EOL");
                break
            },
        }
    }
    println!("");
}
