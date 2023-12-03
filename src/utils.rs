use std::fs::read_to_string;

pub fn input_as_str_vec(file: &str) -> Vec<String> {
    read_to_string("./input/".to_owned() + file).unwrap().lines().map(String::from).collect()
}