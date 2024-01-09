mod args;
mod parsing;

use std::env;
use std::collections::HashMap;

fn main() {
    let keys: HashMap<char, String> = parsing::get_keys(args::get_file_string(args::get_file_path(env::args()))); 
    let states: HashMap<String, Vec<String>> = parsing::get_states(args::get_file_string(args::get_file_path(env::args())));
}
