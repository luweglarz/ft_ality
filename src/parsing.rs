use std::collections::HashMap;

fn get_keys_vec(file_str: String) -> Vec<String>{
    let lines: Vec<&str> = file_str.lines().collect();
    match lines.iter().position(|&line| line.contains('-')) {
        Some(index) => lines[..=index - 1].iter().map(|&s| s.to_string()).collect(),
        None => lines.iter().map(|&s| s.to_string()).collect(),
    }
}

pub fn get_keys(file_str: String) -> HashMap<char, String>{
    get_keys_vec(file_str).iter()
            .filter_map(|str| {
                let splitted: Vec<&str> = str.split("=").into_iter().map(|str| str.trim()).collect();
                Some((
                    splitted.get(0)?.chars().next()?,
                    splitted.get(1)?.to_string(),
                ))
            })
            .collect()
}

fn get_states_vec(file_str: String) -> Vec<String>{
    let lines: Vec<&str> = file_str.lines().collect();
    match lines.iter().position(|&line| line.contains('-')) {
        Some(index) => lines[1 + index..].iter().map(|&s| s.to_string()).collect(),
        None => lines.iter().map(|&s| s.to_string()).collect(),
    }
}

pub fn get_states(file_str: String) -> HashMap<String, Vec<String>>{
    get_states_vec(file_str).iter()
            .filter_map(|str| {
                let splitted: Vec<&str> = str.split("=").into_iter().map(|str| str.trim()).collect();
                Some((
                    splitted.get(1)?.to_string(),
                    splitted.get(0)?.split(",").into_iter().map(|str| str.trim().to_string()).collect(),
                ))
            })
            .collect()
}