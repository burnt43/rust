pub fn execute () {
    let s:&str = "hello my name is jim";
    let mut iter = s.split(" ").map(|c:&str| c.len());
    assert_eq!(iter.next().unwrap(),5);
    assert_eq!(iter.next().unwrap(),2);
    assert_eq!(iter.last().unwrap(),3);
    let s:&str = "hello my name is jim";
    let mut iter = s.split(" ").map(|c:&str| c.len());
    assert_eq!(iter.nth(2).unwrap(),4);
    assert_eq!(iter.next().unwrap(),2);
    assert_eq!(iter.next().unwrap(),3);
    assert!(iter.next().is_none());
    let a = vec![1,2,3,4,5];
    let b = vec![6,7,8,9,10];
    for i in a.iter().chain(b.iter()) {
        println!("i: {}",i);
    }

    for (i,j) in a.iter().zip(b.iter()) {
        println!("i: {},j: {}",i,j);
    }

    for i in (0..10).filter(|c| c % 2 == 0) {
        println!("{}",i);
    }

    for i in (0..10).filter_map(|c| {
        if c % 2 == 0 {
            Some(c * 5)
        } else {
            None
        }
    }) {
        println!("{}",i);
    }

    for (index,el) in (1..10).enumerate() {
        println!("index: {}, el: {}",index,el);
    }

    let mut iter = (0..3).peekable();
    assert_eq!(iter.next().unwrap(),0);
    assert_eq!(*iter.peek().unwrap(),1);

    let s:&str = "THIS IS A STRING SLICE";
    let mut iter = s.split(" ").peekable();

    loop {
        match iter.next() {
            Some(x) => {
                match iter.peek() {
                    Some(_) => println!("Val: {}",x),
                    None => {
                        println!("Last Val: {}",x);
                        break
                    },
                }
            },
            None => break,
        }
    }
}
