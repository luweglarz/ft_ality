mod args;
mod parsing;

use std::env;
use std::collections::HashMap;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;

struct FiniteAutomata {
    keys: HashMap<Keycode, String>,
    states: HashMap<String, Vec<String>>,
    // current_state: ,
    // transitions: ,
}

fn combo_core(file_str: &str ,key: Keycode){
    let automata = FiniteAutomata {
        states: parsing::get_states(args::get_file_string(args::get_file_path(env::args()))),
        keys: parsing::get_keys(args::get_file_string(args::get_file_path(env::args())))
    };
    println!("key: {}", key);
    //if (automata.keys.contains_key(key))
}

fn event_loop(event_pump: &mut sdl2::EventPump, file_str: String) {
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(key), .. } => combo_core(&file_str, key),
                _ => {}
            }
        }
    }
}

fn init_sdl(file_str: String) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Ft_ality", 800, 800)
        .build()
        .expect("Could not initialize the video subsystem");

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("./image.png").unwrap();
    canvas.copy(&texture,None,None).unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    event_loop(&mut event_pump, file_str)
}

fn main() {
    // let _automata = FiniteAutomata {
    //     _states: parsing::get_states(args::get_file_string(args::get_file_path(env::args()))),
    //     _keys: parsing::get_keys(args::get_file_string(args::get_file_path(env::args())))
    // };

    init_sdl(args::get_file_string(args::get_file_path(env::args())));
    
}
