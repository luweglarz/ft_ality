mod args;

use std::collections;
use std::collections::HashMap;
use std::env;

use crate::args::ality_args;

struct State {
    key : char,
    name : String,
    transition: Vec<State>
}

//Should be in a functional way
fn get_keys(keys_vec: Vec<&str>) -> collections::HashMap<char, String>{
    let mut key_map = collections::HashMap::<char, String>::new();
    for str in keys_vec{
        let splitted: Vec<&str> = str.split("=").into_iter().map(|str| str.trim()).collect();
        let split_str: (char, &str) = if splitted.len() == 2 && splitted[0].len() == 1 {
            (splitted[0].chars().next().unwrap(), splitted[1])
        }else {
            eprintln!("Error reading file: wrong format",);
            std::process::exit(1);
        };
        key_map.insert(split_str.0, split_str.1.to_string());
    }
    return key_map;
}

fn main() {
    let file_path : String = ality_args::get_file_path(env::args());
    let file_str: String = ality_args::get_file_string(file_path);
    let lines: Vec<&str> = file_str.lines().collect();
    let keys_vec: Vec<&str> = match lines.iter().position(|&line| line.contains('-')) {
        Some(index) => lines[..=index - 1].to_vec(),
        None => lines.to_vec(),
    };
    let keys: HashMap<char, String> = get_keys(keys_vec);
}
