use std::fs;

fn main() {
    let filename = "src/text.txt";
    let file = fs::read_to_string(filename).expect("Failed to read");

    println!("Text: {}", file);
}
