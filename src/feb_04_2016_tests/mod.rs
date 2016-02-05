struct Wheel {
    size: u8,
    name: String,
}

struct Car<'a> {
    wheels: [&'a Wheel;4],
}

#[test]
fn foo () {
    let front_left  = Wheel { size: 18, name: "OZ Racing".to_string() };
    let front_right = Wheel { size: 18, name: "OZ Racing".to_string() };
    let rear_left   = Wheel { size: 18, name: "OZ Racing".to_string() };
    let rear_right  = Wheel { size: 18, name: "OZ Racing".to_string() };

    let big_wheel  = Wheel { size: 20, name: "Enkei".to_string() };

    let _car = Car { wheels: [&front_left, &front_right, &rear_left, &rear_right] };
    let _car = Car { wheels: [&big_wheel, &big_wheel, &big_wheel, &big_wheel] };
}
