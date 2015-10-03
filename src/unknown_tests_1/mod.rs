#[derive(Debug)]
struct Person {
    foo: Option<u8>,
    bar: Option<u8>,
    baz: Option<String>,
}

impl Person {
    fn to_u8_option( s: &str ) -> Option<u8> {
        match s.parse::<u8>() {
            Ok(value) => Option::Some(value),
            Err(_)    => Option::None,
        }
    }
    fn create_from_str(s: &str) -> Person {
        let v: Vec<&str> = s.split(",").collect();
        Person {
            foo: Person::to_u8_option( v[0] ),
            bar: Person::to_u8_option( v[1] ),
            baz: Option::Some("James".to_string()),
        }
    }
}

struct PeopleMaker {
    data: String,
}

impl PeopleMaker {
    fn new(s: &str) -> PeopleMaker {
        PeopleMaker{ data: s.to_string() }
    }
    fn make_people( &self ) -> Vec<Person> {
        let mut people: Vec<Person> = Vec::new();
        for comma_sep in self.data.split(";") {
            people.push( Person::create_from_str(comma_sep) );
        }
        return people;
    }
}

pub fn execute () {
    let people_maker: PeopleMaker = PeopleMaker::new("1,2;6,8;2,3;9,1;5,7;1,5");
    let people:       Vec<Person> = people_maker.make_people();

    for person in people {
        match (person.foo, person.bar, person.baz.clone()) {
            (Some(1), Some(_bar), Some(baz)) => {
                if &baz == "James" {
                    println!("{:?}",person);
                }
            },
            (_,_,_) => {},
        }
    }
}
