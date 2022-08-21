#![allow(warnings)]

mod color;
mod graphic;
mod tetris;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use {
    graphic::Sdl,
    tetris::Tetris,
};


fn main() {
    let mut tetris = Tetris::new();
    let mut sdl = Sdl::init();

    'runtime: loop {
        // handling events
        for event in sdl.listen() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'runtime;
                }
                _ => tetris.react_to(event),
            };
        }
    }
}
