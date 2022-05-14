use std::fs;

pub fn open_file_and_to_upper_letters(filename: &str, num: i32) {
    let upper_case = fs::read_to_string(filename)
        .expect("Failed to read")
        .to_uppercase();

    let mut result = "".to_string();

    for i in 0..num {
        result.push_str(upper_case.as_str());
        result.push_str("\n");
    }

    fs::write(filename, &result).expect("Failed to write");

    println!("Text: \n {}", result);
}
