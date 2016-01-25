enum Foobar {
    Alpha(String),
    Beta(u8),
    Gamma(char),
}

trait Better {
    fn better_func();
}

impl Better for String {
    fn better_func() {
        println!("String");
    }
}

impl Foobar {
    fn get<T>(&self) -> Result<T,()> where T: Better {
        Err(())
    }
}

#[test]
fn sanity_check() {
    let _x: Foobar = Foobar::Alpha(String::new());
    let _result    = _x.get::<String>();
}
