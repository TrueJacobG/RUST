pub fn run() {
    println!("{}", greeting("grats", "julia"));

    // closure, lambda
    let add_two = |n1: i32, n2: i32| n1 + n2;
    println!("{}", add_two(1, 2));
}

fn greeting(greet: &str, name: &str) -> String {
    let mut greetings: String = String::from("");
    greetings.push_str(greet);
    greetings.push_str(" ");
    greetings.push_str(name);

    greetings
}
