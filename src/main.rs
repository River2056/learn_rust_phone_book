use std::io::stdin;

#[path ="./enums/phone_enum.rs"]
mod enums;
use enums::*;

#[derive(Debug)]
struct Contact {
    name: String,
    phone_num: String,
    phone_type: PhoneEnum
}

impl Contact {
    fn new(name: &str, phone_num: &str, phone_type: &str) -> Self {
        let phone_t: PhoneEnum = PhoneEnum::from_string(phone_type);
        Self {
            name: name.to_string(),
            phone_num: phone_num.to_string(),
            phone_type: phone_t,
        }
    }
}

fn read_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut v: String = String::new();
    stdin()
        .read_line(&mut v)
        .expect("error while reading input");
    return v.trim().to_string();
}

fn main() {

    let mut contact_list: Vec<Contact> = vec![
        Contact::new("Jane & John Doe", "555-123-4567", "home"),
        Contact::new("Joseph Bloggs", "555-123-2222", "cell"),
        Contact::new("Rowena Stevenson", "555-123-3333", "work"),
        Contact::new("Marcus Mills", "555-123-4444", "work"),
        Contact::new("Orson Richards", "555-123-5555", "work"),
    ];

    loop {
        let name: String = read_user_input("Enter name: ");
        let phone: String = read_user_input("Enter phone number: ");
        let phone_type: String = read_user_input("Enter phone type: ");
        if name.is_empty() && phone.is_empty() && phone_type.is_empty() {
            println!("printing contact list");
            break;
        } else {
            println!("adding new contacts to list");
            contact_list.push(Contact::new(&name, &phone, &phone_type));
        }
    }
    println!("{:#?}", contact_list);
}
