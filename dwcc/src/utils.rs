use std::fs::File;
use std::io::Read;

pub fn get_file_contents(filename: &str) -> String {
    let mut file = File::open(filename).expect("Could not read file.");
    let mut contents = String::new();

    file.read_to_string(&mut contents);

    contents
}
