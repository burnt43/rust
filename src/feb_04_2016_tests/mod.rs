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

    let _car = Car { wheels: [&front_left, &front_right, &rear_left, &rear_right] };

    let mut big_wheel  = Wheel { size: 20, name: "Enkei".to_string() };
    let mut _car = Car { wheels: [&big_wheel, &big_wheel, &big_wheel, &big_wheel] };
    // cannot be done?
    // let foo: &mut Wheel = &mut big_wheel;
    // let _tmp: &mut Wheel = _car.wheels[0].as_mut();
    // _tmp.size = 21;
}
