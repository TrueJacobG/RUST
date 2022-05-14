// variables are immutable by default

pub fn run() {
    let name = "John";
    let mut age = 18;

    age = 19;

    println!("My name is {} and I am {}", name, age);

    const ID: i16 = 040;
    println!("ID: {}", ID);

    println!("i32 max: {}", std::i32::MAX);

    let face: char = '\u{1F600}';
    println!("Emoji? {}", face);

    // str = immutable, fixed-length
    // String = growable, ds

    // clear console
    //std::process::Command::new("clear").status().unwrap();

    let s1 = "Hello";
    let mut s2: String = String::from("Hello");

    println!("str: {} String: {}", s1, s2);
    println!("Length: {} {}", s1.len(), s2.len());

    s2.push('!');
    s2.push_str(" another hello :D");
    println!("String after add: {}", s2);

    for word in s2.split_whitespace() {
        println!("{} ", word);
    }
}
