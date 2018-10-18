use std::fs::File;
use std::io::prelude::*;
pub fn load_into_string(filename: &str) -> Option<Box<String>> {
    let mut contents = Box::new(String::new());
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut *contents).unwrap();
    Some(contents)
}
