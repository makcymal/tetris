mod geom;
mod map;
mod random;
mod score;
mod tetrimino;

use sdl2::keyboard::Keycode;
use {
	map::Map,
	tetrimino::{
		Generator,
		Tetrimino,
	},
	sdl2::{
		event::Event,
	},
};


pub struct Tetris {
	level: u8,
	score: usize,
	lines: u8,
	generator: Generator,
	tetrimino: Option<Tetrimino>,
	map: Map,
}

impl Tetris {
	// makes all setup for the game
	pub fn new() -> Tetris {
		let generator = Generator::new();
		let map = Map::new(generator.map_bg());

		Tetris {
			level: 1,
			score: 0,
			lines: 0,
			generator,
			tetrimino: None,
			map,
		}
	}

	// handling the given event related to the game mechanics
	pub fn react_to(&mut self, event: Event) {
		match event {
			Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
				self.rotate(true)
			},
			_ => (),
		}
	}

	pub fn rotate(&mut self, clockwise: bool) {

	}
}
