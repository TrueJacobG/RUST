use std::fmt;

pub enum Location {
    Berlin,
    Paris,
    Rome,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Location::Berlin => write!(f, "Berlin"),
            Location::Paris => write!(f, "Paris"),
            Location::Rome => write!(f, "Rome"),
        }
    }
}

pub struct Person {
    name: String,
    age: i32,
    location: Location,
}

impl Person {
    pub fn new(name: String, age: i32, location: Location) -> Person {
        Person {
            name: name,
            age: age,
            location: location,
        }
    }

    pub fn display(&self) {
        println!("{}", self.name);
        println!("{}", self.age);
        println!("{}", self.location.to_string());
    }
}
