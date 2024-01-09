use std::collections::HashMap;

pub fn get_keys(keys_vec: Vec<&str>) -> HashMap<char, String>{
    keys_vec.iter()
            .filter_map(|&str| {
                let splitted: Vec<&str> = str.split("=").into_iter().map(|str| str.trim()).collect();
                Some((
                    splitted.get(0)?.chars().next()?,
                    splitted.get(1)?.to_string(),
                ))
            })
            .collect()
}

pub fn get_states (states_vec: Vec<&str>) -> HashMap<String, Vec<String>>{
    states_vec.iter()
            .filter_map(|&str| {
                let splitted: Vec<&str> = str.split("=").into_iter().map(|str| str.trim()).collect();
                Some((
                    splitted.get(1)?.to_string(),
                    splitted.get(0)?.split(",").into_iter().map(|str| str.trim().to_string()).collect(),
                ))
            })
            .collect()
}