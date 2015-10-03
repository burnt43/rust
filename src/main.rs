extern crate time;
use std::collections::HashMap;
use std::fmt;
use std::cmp::Ordering;
use time::{Duration, PreciseTime, SteadyTime, Timespec};

fn test_number_0 () {
    assert!(true);
    assert_eq!(10,5+5);
    assert!( cfg!(target_os = "linux") );
    println!("column!() = {}",column!());
    assert_eq!("s1s2s3",concat!("s1","s2","s3"));
    for path in env!("PATH").split(":") {
        println!("{}",path);
    }
    assert_eq!("src/main.rs",file!());
    assert_eq!("x: 10,y: 20",format!("x: {},y: {}",10,20));
    let s = std::fmt::format( format_args!("{}",5) );
    assert_eq!(s,"5");
    let x:u8 = include!("./some_file");
    assert_eq!(x,27);
    let x:&[u8] = include_bytes!("./some.bin");
    assert_eq!(65,x[0]);
    let x:&str = include_str!("./some.txt");
    for line in x.lines() {
        println!("{}",line);
    }
    println!("line!(): {}",line!());
    println!("module_path!(): {}",module_path!());
    assert_eq!(None,option_env!("FOOBAR"));
    assert_eq!("1 + 1",stringify!(1 + 1));

    /*fn unfinished() {
        unimplemented!();
    }*/

    fn some_computation () -> Result<u8, std::str::Utf8Error> {
        let _ = try!( std::str::from_utf8(&[255,255]) );
        Ok(40)
    }

    println!("{}",some_computation().unwrap_or(22));

    match Option::Some(25) {
        Some(n) if n >= 0 => println!(">= 0"),
        Some(n) if n <  0 => println!("<  0"),
        Some(_)           => unreachable!(),
        None              => println!("none"),
    }

    enum List<T> {
        Some(T,Box<List<T>>),
        None,
    }

    let mut x:List<u8> = List::Some(1, Box::new(List::Some(2, Box::new(List::None))) );

    loop {
        match x {
            List::Some(value,next) => { 
                print!("{}->",value);
                x = *next;
            },
            List::None => {
                print!("EOL");
                break
            },
        }
    }
    println!("");
    
    #[derive(PartialEq, Eq, Hash)]
    enum MessageField {
        Name,
        Job,
    }

    fn convert_field_name_to_enum ( s:&str ) -> Option<MessageField> {
        match &*s.to_lowercase() {
            "name" => Some(MessageField::Name),
            "job"  => Some(MessageField::Job),
            _      => None,
        }
    }

    let mut hash:HashMap<MessageField,&str> = HashMap::new();

    let x = "Name: james carson\nJob: engineer";
    for line in x.lines() {
        let v:Vec<&str> = line.splitn(2,": ").collect();
        let (message_field_name,data) = ( v[0], v[1] );
        match convert_field_name_to_enum( message_field_name ) {
            Some(message_field) => {
                hash.insert(message_field,data);
                ()
            },
            None => {},
        }
    }
    println!("Name = {}",hash.get(&MessageField::Name).unwrap());
    println!("Job  = {}",hash.get(&MessageField::Job).unwrap());

    let s:&str = "hello my name is jim";
    let mut iter = s.split(" ").map(|c:&str| c.len());
    assert_eq!(iter.next().unwrap(),5);
    assert_eq!(iter.next().unwrap(),2);
    assert_eq!(iter.last().unwrap(),3);
    let s:&str = "hello my name is jim";
    let mut iter = s.split(" ").map(|c:&str| c.len());
    assert_eq!(iter.nth(2).unwrap(),4);
    assert_eq!(iter.next().unwrap(),2);
    assert_eq!(iter.next().unwrap(),3);
    assert!(iter.next().is_none());
    let a = vec![1,2,3,4,5];
    let b = vec![6,7,8,9,10];
    for i in a.iter().chain(b.iter()) {
        println!("i: {}",i);
    }

    for (i,j) in a.iter().zip(b.iter()) {
        println!("i: {},j: {}",i,j);
    }

    for i in (0..10).filter(|c| c % 2 == 0) {
        println!("{}",i);
    }

    for i in (0..10).filter_map(|c| {
        if c % 2 == 0 {
            Some(c * 5)
        } else {
            None
        }
    }) {
        println!("{}",i);
    }

    for (index,el) in (1..10).enumerate() {
        println!("index: {}, el: {}",index,el);
    }

    let mut iter = (0..3).peekable();
    assert_eq!(iter.next().unwrap(),0);
    assert_eq!(*iter.peek().unwrap(),1);

    let s:&str = "THIS IS A STRING SLICE";
    let mut iter = s.split(" ").peekable();

    loop {
        match iter.next() {
            Some(x) => {
                match iter.peek() {
                    Some(_) => println!("Val: {}",x),
                    None => {
                        println!("Last Val: {}",x);
                        break
                    },
                }
            },
            None => break,
        }
    }
}

fn test_number_1 () {
    struct Parser {
        buffer: String,
    }

    enum MessageType {
        Attack,
        Defend,
    }

    struct Message {
        message_type: MessageType,
    }

    impl Message {
        fn create_from_str( s: &str ) -> Message {
            Message{ message_type: MessageType::Attack }
        }
    }

    impl Parser {
        fn new() -> Parser {
            Parser { buffer: String::new() }
        }
        fn push_to_buffer ( &mut self, s: &str ) -> Vec<Message> {
            let mut messages: Vec<Message> = Vec::new();
            let mut temp: String = self.buffer.clone();
            temp.push_str(s);
            let mut v: Vec<&str> = temp.split("\r\n\r\n").collect();
            if v.len() > 1 {
                let remainder: &str = v.pop().unwrap();
                for x in v {
                    println!("creating from {}",x);
                    messages.push( Message::create_from_str(x) );
                }
                self.buffer = remainder.to_string();
                println!("self.buffer: {}",self.buffer);
            }
            messages
        }
    }

    let mut parser = Parser::new();
    let some_data  = "Attack\r\nfoo\r\nbar\r\n\r\nDefend";
    let _          = parser.push_to_buffer( some_data );
    let _          = parser.push_to_buffer( "\nA\nB\nC\r\n\r\n" );

    let foo: Option<&str> = Some("James");
    match foo {
        Some("Jame") => println!("1"),
        _ => println!("2"),
    }
}

fn test_number_2 () {
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

fn test_number_3 () {
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

fn test_number_4 () {
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

    let s: &str;
    {
        let some_text: SomeText = SomeText::new("James","Carson");
        s = some_text.str_a;
    }
    println!("s: {}",s);
}

fn test_number_5 () {
    struct SomeBullShit {
        foo: String,
        bar: u8,
    }

    impl SomeBullShit {
        fn new(foo:&str, bar:u8) -> SomeBullShit {
            SomeBullShit {
                foo: foo.to_string(),
                bar: bar,
            }
        }
    }

    impl fmt::Debug for SomeBullShit {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"SomeBullShit( foo: {}, bar: {} )", self.foo, self.bar)
        }
    }

    impl fmt::Display for SomeBullShit {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"{}",self.foo)
        }
    }

    let b = SomeBullShit::new("Jim",45);
    println!("SomeBullShit Debug --- {:?}",b);
    println!("SomeBullShit Display --- {}",b);

    assert_eq!(Duration::weeks(1).cmp(&Duration::days(3)),Ordering::Greater);
    assert_eq!(Duration::hours(1).cmp(&Duration::minutes(70)),Ordering::Less);
    assert_eq!(Duration::seconds(1).cmp(&Duration::milliseconds(1000)),Ordering::Equal);
    assert_eq!(Duration::microseconds(1).cmp(&Duration::nanoseconds(1)),Ordering::Greater);

    let d = Duration::span(|| {
        let big_slice   = &[1u8; 10000];
        let mut _sum:u64 = 0;
        for value in big_slice.iter() {
            _sum += *value as u64;
        }
    });

    println!("Duration: {}µs",d.num_microseconds().unwrap());

    assert_eq!(Duration::hours(5).checked_add(&Duration::hours(20)).unwrap(),Duration::hours(25));
    assert_eq!(Duration::minutes(1).checked_sub(&Duration::seconds(30)).unwrap(),Duration::seconds(30));
    assert!(Duration::zero().is_zero());

    let start_time = PreciseTime::now();
    while start_time.to( PreciseTime::now() ) < Duration::milliseconds(1) {
        println!("hello");
    }
    let start_time = SteadyTime::now();
    while SteadyTime::now() - start_time < Duration::milliseconds(1) {
        println!("goodbye");
    }
    assert_eq!(Timespec::new(100,0) - Timespec::new(50,0),Duration::seconds(50));

    let ts:Timespec = time::now().to_timespec();
    println!("current epoch: {}",ts.sec);
    println!("current local time: {}",time::at(ts).strftime("%Y-%m-%d %H:%M:%S").unwrap());
    println!("current UTC time: {}",time::at_utc(ts).strftime("%Y-%m-%d %H:%M:%S").unwrap());
    println!("empty time: {:?}",time::empty_tm());
    println!("current epoch: {}",time::get_time().sec);
    println!("current UTC time: {}",time::now_utc().strftime("%Y-%m-%d %H:%M:%S").unwrap());
    println!("another strftime: {}",time::strftime("%Y-%m-%d %H:%M:%S",&time::now()).unwrap());

    let dates:Vec<&str>        = vec!["1981-07-24","09/23/1983","04-27-1987"];
    let date_formats:Vec<&str> = vec!["%Y-%m-%d","%m/%d/%Y","%m-%d-%Y"];
    for date in &dates {
        for format in &date_formats {
            match time::strptime(date,format) {
                Ok(time) => {
                    println!("date({}) as rfc3339: {}",date,time.rfc3339());
                    break;
                }
                Err(_) => continue,
            }
        }
    }
    println!("time since unspecified epoch: {}",time::precise_time_s());
    println!("time since unspecified epoch as i64: {}",time::precise_time_s() as i64);
    println!("unspecified epoch???: {}",(time::now() - Duration::seconds(time::precise_time_s() as i64)).rfc3339());
    println!("current ctime: {}",time::now().ctime());
}

fn main() {
    test_number_0();
    test_number_1();
    test_number_2();
    test_number_3();
    test_number_4();
    test_number_5();
}


