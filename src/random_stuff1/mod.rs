use std::collections::HashMap;

#[derive(Clone)]
struct FoobarWithClone {
    x: u8,
    y: u8,
}

impl FoobarWithClone {
    fn new() -> FoobarWithClone {
        FoobarWithClone {
            x: 0,
            y: 0,
        }
    }
}

struct FoobarNoClone {
    x: u8,
    y: u8,
    z: HashMap<String,String>,    
}

impl FoobarNoClone {
    fn new() -> FoobarNoClone {
        FoobarNoClone {
            x: 0,
            y: 0,
            z: HashMap::new(),
        }
    }
}

#[test]
fn struct_with_clone() {
    let reference_to_object: &FoobarWithClone = &FoobarWithClone::new();
    let clone_of_reference:  FoobarWithClone  = reference_to_object.clone();
    assert!( (reference_to_object as *const FoobarWithClone) != (&clone_of_reference as *const FoobarWithClone) );
    assert_eq!( reference_to_object.x, clone_of_reference.x );
    assert_eq!( reference_to_object.y, clone_of_reference.y );
}

#[test]
fn struct_without_clone() {
    let reference_to_object: &FoobarNoClone = &FoobarNoClone::new();
    let clone_of_reference:  &FoobarNoClone = reference_to_object.clone();
    assert_eq!( (reference_to_object as *const FoobarNoClone), (clone_of_reference as *const FoobarNoClone) );
}
