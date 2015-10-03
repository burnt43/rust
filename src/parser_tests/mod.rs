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

pub fn execute () {
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
