#[derive(Debug)]
pub enum PhoneEnum {
    Home,
    Work,
    Cell,
    Unknown,
}

impl PhoneEnum {
    pub fn from_string(input_type: &str) -> Self {
        match input_type.to_lowercase().as_str() {
            "home" => PhoneEnum::Home,
            "work" => PhoneEnum::Work,
            "cell" => PhoneEnum::Cell,
            _ => {
                println!("unknown phone type");
                PhoneEnum::Unknown
            }
        }
    }
}
