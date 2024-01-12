mod args;
mod parsing;

use std::env;
use std::collections::HashMap;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use tailcall::tailcall;

struct FiniteAutomata {
    keys: HashMap<Keycode, String>,
    states: HashMap<String, Vec<String>>,
}

fn get_combo(combo_move: &str) -> Vec<String> {
    vec![]
}

fn combo_core(automata: &FiniteAutomata ,key: Keycode) -> Vec<String> {
    match automata.keys.get(&key) {
        Some(combo_move) => { get_combo(&combo_move)},
        None => {vec![]}
    }
}

#[tailcall] #[allow(unreachable_code)]
fn event_loop(event_pump: &mut sdl2::EventPump, automata: &FiniteAutomata, actual_combos: Vec<String>) {
    if let Some(event) = event_pump.poll_iter().next() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
            Event::KeyDown { keycode: Some(key), .. } => {
                event_loop(event_pump, &automata, combo_core(&automata, key))
            },
            _ => {}
        }
    }
    event_loop(event_pump, automata, vec![])
}

fn init_sdl(automata: FiniteAutomata) {
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
    event_loop(&mut event_pump, &automata, vec![])
}

fn main() {
    let automata = FiniteAutomata {
        states: parsing::get_states(args::get_file_string(args::get_file_path(env::args()))),
        keys: parsing::get_keys(args::get_file_string(args::get_file_path(env::args())))
    };

    init_sdl(automata);
    
}
