mod with_file;

use with_file::open_file_and_to_upper_letters;

fn main() {
    let filename = "src/text.txt";
    open_file_and_to_upper_letters(filename, 3);
}
