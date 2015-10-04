pub fn execute () {
    let number: Option<u8> = Some(4);
    if let Some(x) = number {
        assert_eq!(x,4);
    }
}
