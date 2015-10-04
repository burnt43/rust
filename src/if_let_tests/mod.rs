pub fn execute () {

    let number: Option<u8> = Some(4);
    if let Some(x) = number {
        assert_eq!(x,4);
    }

    let mut vec = vec![1,3,2,4,3,5,4,6,5,7];
    while let Some(x) = vec.pop() {
        print!("{} ",x);
    }
    println!("");
}
