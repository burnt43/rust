use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Color {
    Blue,
    Green,
}

struct Person {
    first_name: String,
    last_name:  String,
}

struct PersonBuilder {
    first_name: String,
    last_name:  String,
}

impl PersonBuilder {
    fn new () -> PersonBuilder {
        PersonBuilder {
            first_name: "James".to_string(),
            last_name:  "Carson".to_string(),
        }
    }
    fn first_name(&mut self, s: &str) -> &mut PersonBuilder {
        self.first_name = s.to_string();
        self
    }
    fn last_name(&mut self, s: &str) -> &mut PersonBuilder {
        self.last_name = s.to_string();
        self
    }
    fn finalize (&self) -> Person {
        Person {
            first_name: self.first_name.clone(),
            last_name:  self.last_name.clone(),
        }
    }
}

fn take_ownership ( person: Person ) {
    println!("first_name: {}, last_name: {}", person.first_name, person.last_name);
}

fn borrows ( person: &Person ) {
    println!("first_name: {}, last_name: {}", person.first_name, person.last_name);
}

pub fn execute () {
    // #1 Testing Ownership
    // Won't compile since the function takes over person
    let person: Person = Person { first_name: "Joe".to_string(), last_name: "Blow".to_string() };
    take_ownership( person );
    // take_ownership( person );
    
    // #2 Testing Borrowing
    // Will work since it takes a reference there for only borrows person
    let person: Person = Person { first_name: "Joe".to_string(), last_name: "Blow".to_string() };
    borrows( &person );
    borrows( &person );

    // #3 Taking Ownership in Iterators
    let strings: Vec<String> = vec!["A".to_string(),"B".to_string(),"C".to_string()];
    println!("{:?}",strings);
    for string in strings {
        println!("string: {}",string);
    }
    // println!("{:?}",strings);

    // #4 Borrowing in Iterators
    let strings: Vec<String> = vec!["A".to_string(),"B".to_string(),"C".to_string()];
    println!("{:?}",strings);
    for string in &strings {
        println!("string: {}",string);
    }
    println!("{:?}",strings);

    let _: Person = PersonBuilder::new().first_name("John").last_name("Goolie").finalize();

    let mut hash: HashMap<u8,u8> = HashMap::new();
    hash.insert(1,10);
    hash.insert(2,20);

    let bash: HashMap<u8,u8> = hash.clone();
    println!("{:?}",bash);

    let mut foo: HashMap<Color,u8> = HashMap::new();
    foo.insert(Color::Blue,0);
    foo.insert(Color::Green,1);

    let bar: HashMap<Color,u8> = foo.clone();
    println!("{:?}",bar);
}
