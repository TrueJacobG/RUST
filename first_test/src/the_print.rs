pub fn run() {
    println!("Print func from the_print!");
    println!("Number: {}", 2);
    println!(
        "{0} with {1} looks like that. {0} {0}",
        "Formating", "Index"
    );
    println!(
        "Or {like} that {number} {number}",
        like = "Like",
        number = 0
    );
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    println!("Print multiple values: {:?}", (1, 2, 3, 4));
}
