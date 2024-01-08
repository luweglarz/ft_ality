mod args;

use std::collections;
use std::collections::HashMap;
use std::env;

use crate::args::ality_args;

//Should be in a functional way
fn get_keys(keys_vec: Vec<&str>) -> collections::HashMap<char, String>{
    let mut keys_map = collections::HashMap::<char, String>::new();
    for str in keys_vec{
        let splitted: Vec<&str> = str.split("=").into_iter().map(|str| str.trim()).collect();
        let split_str: (char, &str) = if splitted.len() == 2 && splitted[0].len() == 1 {
            (splitted[0].chars().next().unwrap(), splitted[1])
        }else {
            eprintln!("Error reading file: wrong format",);
            std::process::exit(1);
        };
        keys_map.insert(split_str.0, split_str.1.to_string());
    }
    return keys_map;
}

fn get_states (states_vec: Vec<&str>) -> collections::HashMap<String, Vec<String>>{
    let mut states_map = collections::HashMap::<String, Vec<String>>::new();
    for str in states_vec{
        let splitted: Vec<&str> = str.split("=").into_iter().map(|str| str.trim()).collect();
        let split_str: (String, Vec<String>) = (splitted[1].to_string(), splitted[0].split(",").into_iter().map(|str| str.trim().to_string()).collect());
        states_map.insert(split_str.0, split_str.1);
    }
    return states_map;
}

fn main() {
    let file_str: String = ality_args::get_file_string(ality_args::get_file_path(env::args()));
    let lines: Vec<&str> = file_str.lines().collect();
    let keys_vec: Vec<&str> = match lines.iter().position(|&line| line.contains('-')) {
        Some(index) => lines[..=index - 1].to_vec(),
        None => lines.to_vec(),
    };
    let keys: HashMap<char, String> = get_keys(keys_vec); 
    let states_vec: Vec<&str> = match lines.iter().position(|&line| line.contains('-')) {
        Some(index) => lines[1 + index..].to_vec(),
        None => lines.to_vec(),
    };
    let states: HashMap<String, Vec<String>> = get_states(states_vec);
}
