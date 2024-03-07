use std::fs;

pub fn read_file(location: &str) -> String {
    let path = format!("text/{}", location);
    fs::read_to_string(path).expect("Error reading file")
}
