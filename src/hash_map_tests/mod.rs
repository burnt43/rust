use std::collections::HashMap;

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

pub fn execute () {
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
}

