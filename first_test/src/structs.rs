struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct otherColor(u8, u8, u8);

struct Person {
    name: String,
    age: u16,
}

impl Person {
    fn new(n: String, a: u16) -> Person {
        Person { name: n, age: a }
    }

    fn print_info(&self) {
        println!("Name: {} Age: {}", self.name, self.age);
    }

    fn set_age(&mut self, a: u16) {
        self.age = a;
    }

    fn to_tuple(self) -> (String, u16) {
        (self.name, self.age)
    }
}

pub fn run() {
    let mut color = Color {
        red: 100,
        green: 100,
        blue: 0,
    };

    color.red = 3;

    println!("Color: {} {} {}", color.red, color.green, color.blue);

    let color2 = otherColor(0, 1, 2);
    println!("Color2: {} {} {}", color2.0, color2.1, color2.2);

    let mut p = Person::new(String::from("John"), 20);
    p.print_info();
    p.set_age(30);
    p.print_info();
    println!("{:?}", p.to_tuple());
}
