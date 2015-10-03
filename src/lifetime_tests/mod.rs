struct SomeText<'a> {
    str_a: &'a str,
    str_b: &'a str,
}

impl<'a> SomeText<'a> {
    fn new (a: &'a str, b: &'a str) -> SomeText<'a> {
        SomeText {
            str_a: a,
            str_b: b 
        }
    }
}

pub fn execute () {
    let s: &str;
    {
        let some_text: SomeText = SomeText::new("James","Carson");
        s = some_text.str_a;
    }
    println!("s: {}",s);
}
