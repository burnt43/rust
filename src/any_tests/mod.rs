use std::fmt::Debug;
use std::any::Any;

fn strlen_or_debug<T: Any + Debug>(value: &T) -> String {
    let value_any = value as &Any;

    match value_any.downcast_ref::<String>() {
        Some(string) => {
            format!("Length: {}", string.len())
        }
        None => {
            format!("Debug: {:?}",value)
        }
    }
}

fn modify_input(something: &mut Any) {
    if let Some(string) = something.downcast_mut::<String>() {
        string.push_str("hello");
    }
    if let Some(array) = something.downcast_mut::<[u8;3]>() {
        array[0] = 128;
    }
}

#[test]
fn using_any_1 () {
    let foobar:String = "foobar".to_string();
    assert!( (&foobar as &Any).is::<String>() );

    let foobar:&str = "foobar";
    assert!( (&foobar as &Any).is::<&str>() );
}

#[test]
fn using_any_2 () {
    assert_eq!( strlen_or_debug( &("James".to_string()) ), "Length: 5" );
    assert_eq!( strlen_or_debug(&34), "Debug: 34" );
}

#[test]
fn using_any_3 () {
    let mut foobar:String = "jello".to_string();
    modify_input(&mut foobar);
    assert_eq!( foobar, "jellohello" );

    let mut foobar:[u8;3] = [1,2,3];
    modify_input(&mut foobar);
    assert_eq!( foobar, [128,2,3] );
}
