mod args;
mod parsing;

use std::env;
use std::collections::HashMap;

fn main() {
    let file_str: String = args::get_file_string(args::get_file_path(env::args()));
    let lines: Vec<&str> = file_str.lines().collect();
    let keys_vec: Vec<&str> = match lines.iter().position(|&line| line.contains('-')) {
        Some(index) => lines[..=index - 1].to_vec(),
        None => lines.to_vec(),
    };
    let keys: HashMap<char, String> = parsing::get_keys(keys_vec); 
    let states_vec: Vec<&str> = match lines.iter().position(|&line| line.contains('-')) {
        Some(index) => lines[1 + index..].to_vec(),
        None => lines.to_vec(),
    };
    let states: HashMap<String, Vec<String>> = parsing::get_states(states_vec);
}
