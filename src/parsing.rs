use std::collections::HashMap;
use sdl2::keyboard::Keycode;

fn get_keys_vec(file_str: String) -> Vec<String>{
    let lines: Vec<&str> = file_str.lines().collect();
    match lines.iter().position(|&line| line.contains('-')) {
        Some(index) => lines[..=index - 1].iter().map(|&s| s.to_string()).collect(),
        None => {eprintln!("'-' is missing"); std::process::exit(1);},
    }
}

fn char_to_key(key: char) -> Option<Keycode>{
    match key {
        'a' => Some(Keycode::A),
        'b' => Some(Keycode::B),
        'c' => Some(Keycode::C),
        'a' => Some(Keycode::D),
        'e' => Some(Keycode::E),
        'f' => Some(Keycode::F),
        'g' => Some(Keycode::G),
        'h' => Some(Keycode::H),
        'i' => Some(Keycode::I),
        'j' => Some(Keycode::J),
        'k' => Some(Keycode::K),
        'l' => Some(Keycode::L),
        'm' => Some(Keycode::M),
        'n' => Some(Keycode::N),
        'o' => Some(Keycode::O),
        'p' => Some(Keycode::P),
        'q' => Some(Keycode::Q),
        'r' => Some(Keycode::R),
        's' => Some(Keycode::S),
        't' => Some(Keycode::T),
        'u' => Some(Keycode::U),
        'v' => Some(Keycode::V),
        'w' => Some(Keycode::W),
        'x' => Some(Keycode::X),
        'y' => Some(Keycode::Y),
        'z' => Some(Keycode::Z),
        _ => None,
    }
}

pub fn get_keys(file_str: String) -> HashMap<Keycode, String>{
    get_keys_vec(file_str).iter()
            .filter_map(|str| {
                if !str.contains('='){eprintln!("Wrong key mapping format"); std::process::exit(1);}
                let splitted: Vec<&str> = str.split("=").into_iter().map(|str| str.trim()).collect();

                let key: Option<Keycode> = char_to_key(splitted.get(0)?.chars().next()?);
                let state: String = splitted.get(1)?.to_string();
                match key {
                    Some(key) => Some((key, state)),
                    None => {
                        eprintln!("Wrong key mapping format {:?}", key);
                        std::process::exit(1);
                    }
                }
            })
            .collect()
}

fn get_states_vec(file_str: String) -> Vec<String>{
    let lines: Vec<&str> = file_str.lines().collect();
    match lines.iter().position(|&line| line.contains('-')) {
        Some(index) => lines[1 + index..].iter().map(|&s| s.to_string()).collect(),
        None => {eprintln!("'-' is missing"); std::process::exit(1);},
    }
}

pub fn get_states(file_str: String) -> HashMap<String, Vec<String>>{
    get_states_vec(file_str).iter()
            .filter_map(|str| {
                if !str.contains('='){eprintln!("Wrong move mapping format"); std::process::exit(1);}
                let splitted: Vec<&str> = str.split("=").into_iter().map(|str| str.trim()).collect();
                Some((
                    splitted.get(1)?.to_string(),
                    splitted.get(0)?.split(",").into_iter().map(|str| str.trim().to_string()).collect(),
                ))
            })
            .collect()
}